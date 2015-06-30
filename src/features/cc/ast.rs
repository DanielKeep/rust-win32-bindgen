/*!
Defines the AST for conditional compilation expressions.
*/
use WinVersion;
use features::{Architectures, Partitions, is_important_define};
use super::eval::Value;

/**
A single node in a CC expression AST.
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Node {
    /// An integer literal.
    IntLit(u32),

    /// An identifier.
    Ident(String),

    /// A `defined(_)` or `defined _` expression.
    Defined(Box<Node>),

    /// A `! _` expression.
    Not(Box<Node>),

    /// A `_ && _` expression.
    And(Box<Node>, Box<Node>),

    /// A `_ || _` expression.
    Or(Box<Node>, Box<Node>),

    /// A `_ == _` expression.
    Eq(Box<Node>, Box<Node>),

    /// A `_ != _` expression.
    Ne(Box<Node>, Box<Node>),

    /// A `_ < _` expression.
    Lt(Box<Node>, Box<Node>),

    /// A `_ <= _` expression.
    Le(Box<Node>, Box<Node>),

    /// A `_ > _` expression.
    Gt(Box<Node>, Box<Node>),

    /// A `_ >= _` expression.
    Ge(Box<Node>, Box<Node>),

    /// A `_ >> _` expression.
    Rs(Box<Node>, Box<Node>),

    /// A `OSVER( _ )` expression.
    OsVer(Box<Node>),

    /// A `SPVER( _ )` expression.
    SpVer(Box<Node>),

    /// A `WINAPI_FAMILY_PARTITION( _ )` expression.
    Part(Box<Node>),

    /// A `_( _ )` expression.
    Invoke(Box<Node>, Box<Node>),

    /// An ignored expression.
    Ignore,
}

impl Node {
    /**
    Evaluates an AST node.
    */
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

            /*
            We want this because in at least one case, this being not-defined causes an architecture test to be ignored.
            */
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

    /**
    This effectively just ensures that the given value is a partition value, and immediately converts it into a full feature set.
    */
    fn eval_partition(value: Value) -> Result<Value, String> {
        use super::eval::Value::*;
        match value {
            Part(parts) => Ok(Feat(parts.into())),
            v => Err(format!("cannot eval as a partition: {:?}", v))
        }
    }

    /**
    Simplifies this node into an identifier, or fails if it can't.
    */
    fn simplify_to_ident(&self) -> Result<String, String> {
        use self::Node::*;
        match *self {
            Ident(ref s) => Ok(s.clone()),
            ref node => Err(format!("cannot simplify to identifier: {:?}", node))
        }
    }
}
