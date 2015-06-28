use regex::Regex;
use generated::winver::WinVersion;
use util::ResultOptionExt;
use super::{Features, Partitions, WinVersions, Architectures, is_important_define};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Node {
    IntLit(u32),
    Ident(String),
    Defined(Box<Node>),
    Not(Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Eq(Box<Node>, Box<Node>),
    Ne(Box<Node>, Box<Node>),
    Lt(Box<Node>, Box<Node>),
    Ge(Box<Node>, Box<Node>),
    OsVer(Box<Node>),
    SpVer(Box<Node>),
    Part(Box<Node>),
    Invoke(Box<Node>, Box<Node>),
    Ignore,
}

impl Node {
    pub fn eval(&self) -> Result<Value, String> {
        use self::Node::*;
        match *self {
            IntLit(v) => Ok(Value::Int(v)),
            Ident(ref s) => Node::eval_ident(s),
            Defined(ref n) => Node::eval_defined(&*try!(n.simplify_to_ident())),
            Not(ref n) => try!(n.eval()).complement(),
            And(ref l, ref r) => Ok(try!(try!(l.eval()).and(try!(r.eval())))),
            Or(ref l, ref r) => Ok(try!(try!(l.eval()).or(try!(r.eval())))),
            Eq(ref l, ref r) => Ok(try!(try!(l.eval()).eq(try!(r.eval())))),
            Ne(ref l, ref r) => Ok(try!(try!(l.eval()).ne(try!(r.eval())))),
            Lt(ref l, ref r) => Ok(try!(try!(l.eval()).lt(try!(r.eval())))),
            Ge(ref l, ref r) => Ok(try!(try!(l.eval()).ge(try!(r.eval())))),
            OsVer(ref n) => try!(n.eval()).os_ver(),
            SpVer(ref n) => try!(n.eval()).sp_ver(),
            Part(ref n) => Node::eval_partition(&*try!(n.simplify_to_ident())),
            Invoke(ref n, ref a) => {
                try!(try!(n.eval()).ignore());
                try!(try!(a.eval()).ignore());
                Ok(Value::Ignore)
            },
            Ignore => Ok(Value::Ignore),
        }
    }

    fn eval_ident(ident: &str) -> Result<Value, String> {
        match ident {
            "NTDDI_VERSION" => return Ok(Value::FullVersion),
            "_WIN32_WINNT" => return Ok(Value::ShortVersion),
            _ => ()
        }

        if ident.starts_with("NTDDI_") {
            return match WinVersion::from_name(&ident["NTDDI_".len()..]) {
                Some(wv) => Ok(Value::FullVersionValue(wv as u32)),
                None => Err(format!("unknown NTDDI symbol {:?}", ident))
            };
        }

        if ident.starts_with("_WIN32_WINNT_") {
            return match WinVersion::from_name(&ident["_WIN32_WINNT_".len()..]) {
                Some(wv) => Ok(Value::ShortVersionValue((wv as u32) >> 16)),
                None => Err(format!("unknown WIN32_WINNT symbol {:?}", ident))
            };
        }

        if is_important_define(ident) {
            return Err(format!("cannot eval important ident {:?}", ident))
        }

        Ok(Value::Ignore)
    }

    fn eval_defined(ident: &str) -> Result<Value, String> {
        match ident {
            "NTDDI_VERSION" => return Ok(Value::Ignore),
            _ => ()
        }

        if let Some(arch) = Architectures::from_define(ident) {
            return Ok(Value::Feat(arch.into()));
        }

        if is_important_define(ident) {
            return Err(format!("cannot eval important defined({})", ident));
        }

        Ok(Value::Ignore)
    }

    fn eval_partition(ident: &str) -> Result<Value, String> {
        if let Some(part) = Partitions::from_define(ident) {
            return Ok(Value::Feat(part.into()));
        }

        Err(format!("cannot eval partition {:?}", ident))
    }

    fn simplify_to_ident(&self) -> Result<String, String> {
        use self::Node::*;
        match *self {
            Ident(ref s) => Ok(s.clone()),
            ref node => Err(format!("cannot simplify to identifier: {:?}", node))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Int(u32),
    Feat(Features),
    FullVersionValue(u32),
    ShortVersionValue(u32),
    FullVersion,
    ShortVersion,
    OsVersion,
    SpVersion,
    Ignore,
}

impl Value {
    fn complement(self) -> Result<Value, String> {
        panic!("nyi");
    }

    fn and(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (Ignore, Feat(f)) | (Feat(f), Ignore) => Ok(Feat(f)),
            (Feat(l), Feat(r)) => Ok(Feat(l.and(r))),
            (l, r) => Err(format!("invalid op: {:?} && {:?}", l, r))
        }
    }

    fn or(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (Ignore, Feat(f)) | (Feat(f), Ignore) => Ok(Feat(f)),
            (Feat(l), Feat(r)) => Ok(Feat(l.or(r))),
            (l, r) => Err(format!("invalid op: {:?} || {:?}", l, r))
        }
    }

    fn eq(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (OsVersion, FullVersionValue(i)) => {
                let start = WinVersion::from_u32_round_up(i).unwrap();
                let end = WinVersion::from_u32_round_up((i & 0xFFFF_0000) + 0x1_0000);
                let wv = match end {
                    Some(end) => WinVersions::from(start..end),
                    None => WinVersions::from(start..)
                };
                Ok(Feat(wv.into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} == {:?}", l, r))
        }
    }

    fn ne(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (OsVersion, FullVersionValue(i)) => {
                let start = WinVersion::from_u32_round_up(i).unwrap();
                let end = WinVersion::from_u32_round_up((i & 0xFFFF_0000) + 0x1_0000);
                let wv = match end {
                    Some(end) => WinVersions::from((..start, end..)),
                    None => WinVersions::from(..start)
                };
                Ok(Feat(wv.into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} != {:?}", l, r))
        }
    }

    fn lt(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersion, FullVersionValue(i)) | (FullVersion, Int(i)) => {
                let end = WinVersion::from_u32_round_up(i).unwrap();
                Ok(Feat(WinVersions::from(..end).into()))
            },
            (ShortVersion, ShortVersionValue(i)) | (ShortVersion, Int(i)) => {
                let end = WinVersion::from_u32_round_up(i << 16).unwrap();
                Ok(Feat(WinVersions::from(..end).into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} < {:?}", l, r))
        }
    }

    fn ge(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersion, FullVersionValue(i)) | (FullVersion, Int(i)) => {
                let start = WinVersion::from_u32_round_up(i).unwrap();
                Ok(Feat(WinVersions::from(start..).into()))
            },
            (ShortVersion, ShortVersionValue(i)) | (ShortVersion, Int(i)) => {
                let start = WinVersion::from_u32_round_up(i << 16).unwrap();
                Ok(Feat(WinVersions::from(start..).into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} >= {:?}", l, r))
        }
    }

    fn os_ver(self) -> Result<Value, String> {
        use self::Value::*;
        match self {
            FullVersion => Ok(OsVersion),
            n => Err(format!("invalid op: OSVER({:?})", n))
        }
    }

    fn sp_ver(self) -> Result<Value, String> {
        use self::Value::*;
        match self {
            FullVersion => Ok(SpVersion),
            n => Err(format!("invalid op: SPVER({:?})", n))
        }
    }

    fn ignore(self) -> Result<Value, String> {
        use self::Value::*;
        match self {
            Ignore => Ok(Ignore),
            n => Err(format!("cannot ignore {:?}", n))
        }
    }
}

macro_rules! match_toks {
    (
        $toks:expr,
        $([$($match_toks:expr),*; ..$match_tail:ident] => $match_expr:expr,)*
        _ => $fallback_expr:expr
    ) => {
        {
            let toks = $toks;
            let mut outer_result = Ok(None);
            loop {
                $(
                    {
                        loop {
                            let mut cur = toks;
                            $(
                                if cur.len() == 0 { break; }
                                if cur[0].as_ref() != $match_toks { break; }
                                cur = &cur[1..];
                            )*
                            let $match_tail = cur;
                            outer_result = $match_expr;
                            break;
                        }
                        match outer_result {
                            Ok(Some(_)) => break,
                            Ok(None) => (),
                            Err(err) => return Err(err)
                        }
                    }
                )*
                outer_result = $fallback_expr;
                break;
            }
            outer_result
        }
    };
}

macro_rules! parse_binary {
    ($toks:expr, $parse_lhs:expr => $lhs:ident, $parse_op:expr, $parse_rhs:expr => $rhs:ident, $body:expr) => {
        $parse_lhs($toks)
            .ro_and_then(|(lhs, toks)| match_toks! {
                toks,
                [$parse_op; ..tail] => Ok(Some((lhs, tail))),
                _ => Ok(None)
            })
            .ro_and_then(|($lhs, toks)| $parse_rhs(toks)
                .ro_and_then(|($rhs, tail)| Ok(Some(($body, tail))))
            )
    };
}

pub type ParseResult<'a, S> = Result<Option<(Node, &'a [S])>, String>;

// http://www.nongnu.org/hcb/#conditional-expression

pub fn parse_conditional_expr<S: AsRef<str>>(toks: &[S]) -> ParseResult<S> {
    parse_logical_or_expr(toks)
}

fn parse_logical_or_expr<S: AsRef<str>>(toks: &[S]) -> ParseResult<S> {
    parse_logical_and_expr(toks)
        .ro_or_else(|| parse_binary!(toks,
            parse_logical_or_expr => lhs, "||", parse_logical_and_expr => rhs,
            Node::Or(Box::new(lhs), Box::new(rhs))))
}

fn parse_logical_and_expr<S: AsRef<str>>(toks: &[S]) -> ParseResult<S> {
    parse_equality_expr(toks)
        .ro_or_else(|| parse_binary!(toks,
            parse_logical_and_expr => lhs, "&&", parse_equality_expr => rhs,
            Node::And(Box::new(lhs), Box::new(rhs))))
}

fn parse_equality_expr<S: AsRef<str>>(toks: &[S]) -> ParseResult<S> {
    parse_relational_expr(toks)
        .ro_or_else(|| parse_binary!(toks,
            parse_equality_expr => lhs, "==", parse_relational_expr => rhs,
            Node::Eq(Box::new(lhs), Box::new(rhs))))
        .ro_or_else(|| parse_binary!(toks,
            parse_equality_expr => lhs, "!=", parse_relational_expr => rhs,
            Node::Ne(Box::new(lhs), Box::new(rhs))))
}

fn parse_relational_expr<S: AsRef<str>>(toks: &[S]) -> ParseResult<S> {
    parse_unary_expr(toks)
        .ro_or_else(|| parse_binary!(toks,
            parse_relational_expr => lhs, "<", parse_unary_expr => rhs,
            Node::Lt(Box::new(lhs), Box::new(rhs))))
        .ro_or_else(|| parse_binary!(toks,
            parse_relational_expr => lhs, ">=", parse_unary_expr => rhs,
            Node::Ge(Box::new(lhs), Box::new(rhs))))
}

fn parse_unary_expr<S: AsRef<str>>(toks: &[S]) -> ParseResult<S> {
    parse_primary_expr(toks)
        .ro_or_else(|| parse_munch(toks, "!")
            .ro_and_then(|toks| parse_primary_expr(toks))
            .ro_and_then(|(node, tail)| Ok(Some((Node::Not(Box::new(node)), tail))))
        )
        .ro_or_else(|| parse_munch(toks, "defined")
            .ro_and_then(|toks| parse_primary_expr(toks))
            .ro_and_then(|(node, tail)| Ok(Some((Node::Defined(Box::new(node)), tail))))
        )
        .ro_or_else(|| parse_munch(toks, "OSVER")
            .ro_and_then(|toks| parse_munch(toks, "("))
            .ro_and_then(|toks| parse_conditional_expr(toks))
            .ro_and_then(|(node, toks)| parse_munch(toks, ")")
                .ro_and_then(|tail| Ok(Some((Node::OsVer(Box::new(node)), tail))))
            )
        )
        .ro_or_else(|| parse_munch(toks, "SPVER")
            .ro_and_then(|toks| parse_munch(toks, "("))
            .ro_and_then(|toks| parse_conditional_expr(toks))
            .ro_and_then(|(node, toks)| parse_munch(toks, ")")
                .ro_and_then(|tail| Ok(Some((Node::SpVer(Box::new(node)), tail))))
            )
        )
        .ro_or_else(|| parse_munch(toks, "WINAPI_FAMILY_PARTITION")
            .ro_and_then(|toks| parse_munch(toks, "("))
            .ro_and_then(|toks| parse_conditional_expr(toks))
            .ro_and_then(|(node, toks)| parse_munch(toks, ")")
                .ro_and_then(|tail| Ok(Some((Node::Part(Box::new(node)), tail))))
            )
        )
        .ro_or_else(|| parse_munch(toks, "WINAPI_FAMILY_ONE_PARTITION")
            /*
            This basically does `((WINAPI_FAMILY & arg0) == arg1)`.  As far as I know, it's only used in *one* place: to determine if the family is *exactly* `WINAPI_PARTITION_APP`... but it doesn't appear to actually make any real difference in terms of our binding work, so we'll just ignore it.
            */
            .ro_and_then(|toks| parse_munch(toks, "("))
            .ro_and_then(|toks| parse_conditional_expr(toks))
            .ro_and_then(|(_, toks)| parse_munch(toks, ")")
                .ro_and_then(|tail| Ok(Some((Node::Ignore, tail))))
            )
        )
        .ro_or_else(|| parse_ident(toks)
            .ro_and_then(|(node, toks)| parse_munch(toks, ")")
                .ro_and_then(|toks| parse_conditional_expr(toks))
                .ro_and_then(|(arg, toks)| parse_munch(toks, ")")
                    .ro_and_then(|tail| Ok(Some((Node::Invoke(Box::new(node), Box::new(arg)), tail))))
                )
            )
        )
}

fn parse_primary_expr<S: AsRef<str>>(toks: &[S]) -> ParseResult<S> {
    parse_literal(toks)
        .ro_or_else(|| parse_munch(toks, "(")
            .ro_and_then(|toks| parse_conditional_expr(toks))
            .ro_and_then(|(node, toks)| parse_munch(toks, ")")
                .ro_and_then(|tail| Ok(Some((node, tail))))
            )
        )
}

lazy_static! {
    static ref RE_INT_HEX_LITERAL: Regex = Regex::new(r#"0[Xx]([0-9A-Fa-f]+)"#).unwrap();
    static ref RE_IDENT_LITERAL: Regex = Regex::new(r#"([A-Fa-f_][A-Fa-f0-9_]*)"#).unwrap();
}

fn parse_literal<S: AsRef<str>>(toks: &[S]) -> ParseResult<S> {
    if toks.len() == 0 { return Ok(None); }

    if let Some(cap) = RE_INT_HEX_LITERAL.captures(toks[0].as_ref()) {
        return Ok(Some((
            Node::IntLit(u32::from_str_radix(cap.at(1).unwrap(), 16).unwrap()),
            &toks[1..]
        )));
    }

    parse_ident(toks)
}

fn parse_ident<S: AsRef<str>>(toks: &[S]) -> ParseResult<S> {
    if toks.len() == 0 { return Ok(None); }

    if let Some(cap) = RE_IDENT_LITERAL.captures(toks[0].as_ref()) {
        return Ok(Some((
            Node::Ident(cap.at(1).unwrap().into()),
            &toks[1..]
        )));
    }

    Ok(None)
}

fn parse_peek<'a, S: AsRef<str>>(toks: &'a [S], tok: &str) -> Result<Option<&'a [S]>, String> {
    match_toks! {
        toks,
        [tok; .._tail] => Ok(Some(toks)),
        _ => Ok(None)
    }
}

fn parse_munch<'a, S: AsRef<str>>(toks: &'a [S], tok: &str) -> Result<Option<&'a [S]>, String> {
    match_toks! {
        toks,
        [tok; ..tail] => Ok(Some(tail)),
        _ => Ok(None)
    }
}
