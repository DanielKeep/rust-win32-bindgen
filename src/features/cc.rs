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
    Le(Box<Node>, Box<Node>),
    Gt(Box<Node>, Box<Node>),
    Ge(Box<Node>, Box<Node>),
    Rs(Box<Node>, Box<Node>),
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
            Le(ref l, ref r) => Ok(try!(try!(l.eval()).le(try!(r.eval())))),
            Gt(ref l, ref r) => Ok(try!(try!(l.eval()).gt(try!(r.eval())))),
            Ge(ref l, ref r) => Ok(try!(try!(l.eval()).ge(try!(r.eval())))),
            Rs(ref l, ref r) => Ok(try!(try!(l.eval()).rs(try!(r.eval())))),
            OsVer(ref n) => try!(n.eval()).os_ver(),
            SpVer(ref n) => try!(n.eval()).sp_ver(),
            Part(ref n) => Node::eval_partition(try!(n.eval())),
            Invoke(ref n, ref a) => {
                try!(try!(n.eval()).ignore());
                try!(try!(a.eval()).ignore());
                Ok(Value::Ignore)
            },
            Ignore => Ok(Value::Ignore),
        }
    }

    /**
    Evaluates a given identifier.
    */
    fn eval_ident(ident: &str) -> Result<Value, String> {
        match ident {
            "NTDDI_VERSION" => return Ok(Value::FullVersion),
            "WINVER" | "_WIN32_WINNT" => return Ok(Value::ShortVersion),
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

        if ident.starts_with("WINAPI_PARTITION_") {
            return match Partitions::from_define(ident) {
                Some(parts) => Ok(Value::Part(parts)),
                None => Err(format!("unknown WINAPI_PARTITION symbol {:?}", ident))
            };
        }

        if is_important_define(ident) {
            return Err(format!("cannot eval important ident {:?}", ident))
        }

        Ok(Value::Ignore)
    }

    /**
    Evaluates `defined(ident)`.

    This should *also* be used to process the argument to `#ifdef` and `#ifndef` directives.
    */
    pub fn eval_defined(ident: &str) -> Result<Value, String> {
        match ident {
            "NTDDI_VERSION"
            | "_WIN32_WINNT"
            | "WINVER"
            => return Ok(Value::Ignore),

            "_CONTRACT_GEN"
            => return Ok(Value::Bool(false)),

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

    fn eval_partition(value: Value) -> Result<Value, String> {
        use self::Value::*;
        match value {
            Part(parts) => Ok(Feat(parts.into())),
            v => Err(format!("cannot eval as a partition: {:?}", v))
        }
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
    Bool(bool),
    Int(u32),
    Feat(Features),
    Part(Partitions),
    FullVersionValue(u32),
    ShortVersionValue(u32),
    FullVersion,
    ShortVersion,
    OsVersion,
    SpVersion,
    Ignore,
}

impl Value {
    pub fn to_features(self) -> Result<Features, String> {
        use self::Value::*;
        match self {
            Bool(_) => Ok(Features::default()),
            Feat(f) => Ok(f),
            Part(p) => Ok(p.into()),
            FullVersionValue(v) => {
                let wv = WinVersion::from_u32_round_up(v).expect("valid full version");
                Ok(WinVersions::from(wv).into())
            },
            ShortVersionValue(v) => {
                let wv = WinVersion::from_u32_round_up(v << 16).expect("valid full version");
                Ok(WinVersions::from(wv).into())
            },
            FullVersion
            | ShortVersion
            | Ignore => Ok(Features::default()),
            n => Err(format!("cannot convert to Features: {:?}", n))
        }
    }

    pub fn complement(self) -> Result<Value, String> {
        use self::Value::*;
        match self {
            Ignore => Ok(Ignore),
            Bool(b) => Ok(Bool(!b)),
            Feat(f) => Ok(Feat(f.complement())),
            Part(p) => Ok(Part(!p)),
            n => Err(format!("invalid op: ! {:?}", n))
        }
    }

    fn and(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Bool(b)) | (Bool(b), Ignore) => Ok(Bool(b)),
            (Ignore, Feat(f)) | (Feat(f), Ignore) => Ok(Feat(f)),
            (Bool(true), Feat(f)) | (Feat(f), Bool(true)) => Ok(Feat(f)),
            (Bool(false), Feat(_)) | (Feat(_), Bool(false)) => Ok(Ignore),
            (Feat(l), Feat(r)) => Ok(Feat(l.and(r))),
            (Part(l), Part(r)) => Ok(Part(l & r)),
            (l, r) => Err(format!("invalid op: {:?} && {:?}", l, r))
        }
    }

    fn or(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Bool(b)) | (Bool(b), Ignore) => Ok(Bool(b)),
            (Ignore, Feat(f)) | (Feat(f), Ignore) => Ok(Feat(f)),
            (Feat(l), Feat(r)) => Ok(Feat(l.or(r))),
            (Part(l), Part(r)) => Ok(Part(l | r)),
            (l, r) => Err(format!("invalid op: {:?} || {:?}", l, r))
        }
    }

    fn eq(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersion, FullVersionValue(i)) => {
                let start = WinVersion::from_u32_round_up(i).expect("valid version for fs == fvv");
                let end = WinVersion::from_u32_round_up((i & 0xFFFF_0000) + 1);
                let wv = match end {
                    Some(end) => WinVersions::from(start..end),
                    None => WinVersions::from(start..)
                };
                Ok(Feat(wv.into()))
            },
            (OsVersion, FullVersionValue(i)) => {
                let start = WinVersion::from_u32_round_up(i).expect("valid version for os == fvv");
                let end = WinVersion::from_u32_round_up((i & 0xFFFF_0000) + 0x1_0000);
                let wv = match end {
                    Some(end) => WinVersions::from(start..end),
                    None => WinVersions::from(start..)
                };
                Ok(Feat(wv.into()))
            },
            (ShortVersion, Int(v)) | (Int(v), ShortVersion) => {
                let start = WinVersion::from_u32_round_up(v << 16).expect("valid version for sv == int");
                let end = WinVersion::from_u32_round_up(((v << 16) + 0x1_0000));
                let wv = WinVersions::from(Some(start)..end);
                Ok(Feat(wv.into()))
            }
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} == {:?}", l, r))
        }
    }

    fn ne(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (OsVersion, FullVersionValue(i)) => {
                let start = WinVersion::from_u32_round_up(i).expect("valid full os version for !=");
                let end = WinVersion::from_u32_round_up((i & 0xFFFF_0000) + 0x1_0000);
                let wv = match end {
                    Some(end) => WinVersions::from((..start, end..)),
                    None => WinVersions::from(..start)
                };
                Ok(Feat(wv.into()))
            },
            (ShortVersion, ShortVersionValue(i)) => {
                let start = WinVersion::from_u32_round_up(i << 16).expect("valid short version for !=");
                let end = start.next_version();
                let wv = match end {
                    Some(end) => WinVersions::from((..start, end..)),
                    None => WinVersions::from(..start)
                };
                Ok(Feat(wv.into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} != {:?}", l, r))
        }
    }

    fn lt(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersion, FullVersionValue(i)) | (FullVersion, Int(i)) => {
                let end = WinVersion::from_u32_round_up(i).expect("valid full version for <");
                Ok(Feat(WinVersions::from(..end).into()))
            },
            (ShortVersion, ShortVersionValue(i)) | (ShortVersion, Int(i)) => {
                let end = WinVersion::from_u32_round_up(i << 16).expect("valid short version for <");
                Ok(Feat(WinVersions::from(..end).into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} < {:?}", l, r))
        }
    }

    fn le(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersion, FullVersionValue(i)) | (FullVersion, Int(i)) => {
                let end = WinVersion::from_u32_round_up(i + 1).expect("valid full version for <=");
                Ok(Feat(WinVersions::from(..end).into()))
            },
            (ShortVersion, ShortVersionValue(i)) | (ShortVersion, Int(i)) => {
                let end = WinVersion::from_u32_round_up((i << 16) + 1).expect("valid short version for <=");
                Ok(Feat(WinVersions::from(..end).into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} <= {:?}", l, r))
        }
    }

    fn gt(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersion, FullVersionValue(i)) | (FullVersion, Int(i)) => {
                let start = WinVersion::from_u32_round_up(i + 1).expect("valid full version for >");
                Ok(Feat(WinVersions::from(start..).into()))
            },
            (ShortVersion, ShortVersionValue(i)) | (ShortVersion, Int(i)) => {
                let start = WinVersion::from_u32_round_up((i << 16) + 1).expect("valid full version for >");
                Ok(Feat(WinVersions::from(start..).into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} > {:?}", l, r))
        }
    }

    fn ge(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersion, FullVersionValue(i)) | (FullVersion, Int(i)) => {
                let start = WinVersion::from_u32_round_up(i).expect("valid full version for >=");
                Ok(Feat(WinVersions::from(start..).into()))
            },
            (ShortVersion, ShortVersionValue(i)) | (ShortVersion, Int(i)) => {
                let start = WinVersion::from_u32_round_up(i << 16).expect("valid full version for >=");
                Ok(Feat(WinVersions::from(start..).into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} >= {:?}", l, r))
        }
    }

    fn rs(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersionValue(v), Int(16)) => Ok(ShortVersionValue(v >> 16)),
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} >> {:?}", l, r))
        }
    }

    fn os_ver(self) -> Result<Value, String> {
        use self::Value::*;
        match self {
            FullVersion => Ok(OsVersion),
            FullVersionValue(v) => Ok(FullVersionValue(v & 0xFFFF_0000)),
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

macro_rules! parse_guard {
    ($toks:expr) => {
        if $toks.len() == 0 { return Ok(None); }
    };
}

pub type ParseResult<'a, S> = Result<Option<(Node, &'a [S])>, String>;

// http://www.nongnu.org/hcb/#conditional-expression

use std::fmt::Debug;

pub fn parse_conditional_expr<S: AsRef<str> + Debug>(toks: &[S]) -> ParseResult<S> {
    debug!("parse_conditional_expr({:?})", toks);
    parse_guard!(toks);
    parse_logical_or_expr(toks)
}

fn parse_logical_or_expr<S: AsRef<str> + Debug>(toks: &[S]) -> ParseResult<S> {
    debug!("parse_logical_or_expr({:?})", toks);
    parse_guard!(toks);

    parse_logical_and_expr(toks).ro_and_then(|(lhs, toks)|
        (parse_munch(toks, "||")
            .ro_and_then(|toks| parse_logical_or_expr(toks))
            .ro_and_then(|(rhs, tail)| Ok(Some((Node::Or(Box::new(lhs.clone()), Box::new(rhs)), tail))))
        )
        .ro_or_else(|| Ok(Some((lhs, toks))))
    )
}

fn parse_logical_and_expr<S: AsRef<str> + Debug>(toks: &[S]) -> ParseResult<S> {
    debug!("parse_logical_and_expr({:?})", toks);
    parse_guard!(toks);

    parse_equality_expr(toks).ro_and_then(|(lhs, toks)|
        (parse_munch(toks, "&&")
            .ro_and_then(|toks| parse_logical_and_expr(toks))
            .ro_and_then(|(rhs, tail)| Ok(Some((Node::And(Box::new(lhs.clone()), Box::new(rhs)), tail))))
        )
        .ro_or_else(|| Ok(Some((lhs, toks))))
    )
}

fn parse_equality_expr<S: AsRef<str> + Debug>(toks: &[S]) -> ParseResult<S> {
    debug!("parse_equality_expr({:?})", toks);
    parse_guard!(toks);

    parse_relational_expr(toks).ro_and_then(|(lhs, toks)|
        (parse_munch(toks, "==")
            .ro_and_then(|toks| parse_equality_expr(toks))
            .ro_and_then(|(rhs, tail)| Ok(Some((Node::Eq(Box::new(lhs.clone()), Box::new(rhs)), tail))))
        )
        .ro_or_else(|| parse_munch(toks, "!=")
            .ro_and_then(|toks| parse_equality_expr(toks))
            .ro_and_then(|(rhs, tail)| Ok(Some((Node::Ne(Box::new(lhs.clone()), Box::new(rhs)), tail))))
        )
        .ro_or_else(|| Ok(Some((lhs, toks))))
    )
}

fn parse_relational_expr<S: AsRef<str> + Debug>(toks: &[S]) -> ParseResult<S> {
    debug!("parse_relational_expr({:?})", toks);
    parse_guard!(toks);

    parse_shift_expr(toks).ro_and_then(|(lhs, toks)|
        (parse_munch(toks, "<")
            .ro_and_then(|toks| parse_relational_expr(toks))
            .ro_and_then(|(rhs, tail)| Ok(Some((Node::Lt(Box::new(lhs.clone()), Box::new(rhs)), tail))))
        )
        .ro_or_else(|| parse_munch(toks, "<=")
            .ro_and_then(|toks| parse_relational_expr(toks))
            .ro_and_then(|(rhs, tail)| Ok(Some((Node::Le(Box::new(lhs.clone()), Box::new(rhs)), tail))))
        )
        .ro_or_else(|| parse_munch(toks, ">")
            .ro_and_then(|toks| parse_relational_expr(toks))
            .ro_and_then(|(rhs, tail)| Ok(Some((Node::Gt(Box::new(lhs.clone()), Box::new(rhs)), tail))))
        )
        .ro_or_else(|| parse_munch(toks, ">=")
            .ro_and_then(|toks| parse_relational_expr(toks))
            .ro_and_then(|(rhs, tail)| Ok(Some((Node::Ge(Box::new(lhs.clone()), Box::new(rhs)), tail))))
        )
        .ro_or_else(|| Ok(Some((lhs, toks))))
    )
}

fn parse_shift_expr<S: AsRef<str> + Debug>(toks: &[S]) -> ParseResult<S> {
    debug!("parse_shift_expr({:?})", toks);
    parse_guard!(toks);

    parse_unary_expr(toks).ro_and_then(|(lhs, toks)|
        (parse_munch(toks, ">>")
            .ro_and_then(|toks| parse_shift_expr(toks))
            .ro_and_then(|(rhs, tail)| Ok(Some((Node::Rs(Box::new(lhs.clone()), Box::new(rhs)), tail))))
        )
        .ro_or_else(|| Ok(Some((lhs, toks))))
    )
}

fn parse_unary_expr<S: AsRef<str> + Debug>(toks: &[S]) -> ParseResult<S> {
    debug!("parse_unary_expr({:?})", toks);
    parse_guard!(toks);

    parse_munch(toks, "!")
        .ro_and_then(|toks| parse_unary_expr(toks))
        .ro_and_then(|(node, tail)| Ok(Some((Node::Not(Box::new(node)), tail))))
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
        .ro_and_then(|(node, toks)| parse_munch(toks, "(")
            .ro_and_then(|toks| parse_conditional_expr(toks))
            .ro_and_then(|(arg, toks)| parse_munch(toks, ")")
                .ro_and_then(|tail| Ok(Some((Node::Invoke(Box::new(node), Box::new(arg)), tail))))
            )
        )
    )
    .ro_or_else(|| parse_primary_expr(toks))
}

fn parse_primary_expr<S: AsRef<str> + Debug>(toks: &[S]) -> ParseResult<S> {
    debug!("parse_primary_expr({:?})", toks);
    parse_guard!(toks);

    parse_munch(toks, "(")
        .ro_and_then(|toks| parse_conditional_expr(toks))
        .ro_and_then(|(node, toks)| parse_munch(toks, ")")
            .ro_and_then(|tail| Ok(Some((node, tail))))
        )
    .ro_or_else(|| parse_literal(toks))
}

lazy_static! {
    static ref RE_INT_DEC_LITERAL: Regex = Regex::new(r#"^([0-9]+)$"#).unwrap();
    static ref RE_INT_HEX_LITERAL: Regex = Regex::new(r#"^0[Xx]([0-9A-Fa-f]+)$"#).unwrap();
    static ref RE_IDENT_LITERAL: Regex = Regex::new(r#"^([A-Za-z_][A-Za-z0-9_]*)$"#).unwrap();
}

fn parse_literal<S: AsRef<str> + Debug>(toks: &[S]) -> ParseResult<S> {
    debug!("parse_literal({:?})", toks);
    parse_guard!(toks);

    if let Some(cap) = RE_INT_DEC_LITERAL.captures(toks[0].as_ref()) {
        return Ok(Some((
            Node::IntLit(u32::from_str_radix(cap.at(1).unwrap(), 10).ok().expect("valid int literal")),
            &toks[1..]
        )));
    }

    if let Some(cap) = RE_INT_HEX_LITERAL.captures(toks[0].as_ref()) {
        return Ok(Some((
            Node::IntLit(u32::from_str_radix(cap.at(1).unwrap(), 16).ok().expect("valid hex literal")),
            &toks[1..]
        )));
    }

    parse_ident(toks)
}

fn parse_ident<S: AsRef<str> + Debug>(toks: &[S]) -> ParseResult<S> {
    debug!("parse_ident({:?})", toks);
    parse_guard!(toks);

    if let Some(cap) = RE_IDENT_LITERAL.captures(toks[0].as_ref()) {
        return Ok(Some((
            Node::Ident(cap.at(1).unwrap().into()),
            &toks[1..]
        )));
    }

    Ok(None)
}

fn parse_munch<'a, S: AsRef<str> + Debug>(toks: &'a [S], tok: &str) -> Result<Option<&'a [S]>, String> {
    debug!("parse_munch({:?}, {:?})", toks, tok);
    parse_guard!(toks);
    match_toks! {
        toks,
        [tok; ..tail] => Ok(Some(tail)),
        _ => Ok(None)
    }
}
