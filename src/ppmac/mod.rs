#![allow(dead_code)]

#[derive(Clone, Debug)]
pub enum Node {
    Binary(BinOp, Box<Node>, Box<Node>),
    Call { subject: Box<Node>, args: Vec<Node> },
    Cast { value: Box<Node>, ty: Box<Node> },
    Conditional { cond: Box<Node>, then_expr: Box<Node>, else_expr: Box<Node> },
    Ident(String),
    Integer(u64, Signed, Size),
    String(String, CharWidth),
    Type(String, bool),
    Unary(UnaryOp, Box<Node>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BinOp {
    LogOr,
    LogAnd,
    BitOr,
    BitXor,
    BitAnd,
    Eq,
    Ne,
    Le,
    Lt,
    Gt,
    Ge,
    Shl,
    Shr,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CharWidth {
    Narrow,
    Wide,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Signed {
    Yes, No
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Size {
    Unknown,
    Long,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnaryOp {
    Neg,
    Com,
}

pub mod parse {
    use std::borrow::Cow;
    use regex::Regex;
    use super::{Node, BinOp, CharWidth, Signed, Size, UnaryOp};

    use self::Result::{Parsed, Mismatch};

    pub type Tok = String;
    pub type Toks<'a> = &'a [Tok];

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Result<'a, T> {
        Parsed(T, Toks<'a>),
        Mismatch(Cow<'static, str>, Toks<'a>),
    }

    impl<'a> Result<'a, ()> {
        pub fn and_then<F, U>(self, f: F) -> Result<'a, U>
        where F: FnOnce(Toks<'a>) -> Result<'a, U> {
            use self::Result::*;
            match self {
                Parsed((), toks) => f(toks),
                Mismatch(msg, toks) => Mismatch(msg, toks)
            }
        }
    }

    impl<'a, T> Result<'a, T> {
        pub fn and_then_with<F, U>(self, f: F) -> Result<'a, U>
        where F: FnOnce(T, Toks<'a>) -> Result<'a, U> {
            use self::Result::*;
            match self {
                Parsed(v, toks) => f(v, toks),
                Mismatch(msg, toks) => Mismatch(msg, toks)
            }
        }

        pub fn map<F, U>(self, f: F) -> Result<'a, U>
        where F: FnOnce(T) -> U {
            use self::Result::*;
            match self {
                Parsed(v, toks) => Parsed(f(v), toks),
                Mismatch(msg, toks) => Mismatch(msg, toks)
            }
        }

        pub fn or_else<F>(self, f: F) -> Result<'a, T>
        where F: FnOnce(Toks<'a>) -> Result<'a, T> {
            use self::Result::*;
            match self {
                Parsed(v, toks) => Parsed(v, toks),
                Mismatch(_, toks) => f(toks)
            }
        }
    }

    struct DynMove<T>(Option<T>);

    impl<T> DynMove<T> {
        fn take(&mut self) -> T {
            self.0.take().expect("cannot move out of DynMove more than once")
        }
    }

    fn guard_empty(toks: Toks) -> Result<()> {
        if toks.len() == 0 {
            Mismatch("unexpected end-of-input".into(), toks)
        } else {
            Parsed((), toks)
        }
    }

    macro_rules! check_empty {
        ($toks:expr) => {
            match guard_empty($toks) {
                Parsed((), _) => (),
                Mismatch(msg, toks) => return Mismatch(msg, toks)
            }
        }
    }

    fn into_dyn_move<T>(v: T, toks: Toks) -> Result<DynMove<T>> {
        Parsed(DynMove(Some(v)), toks)
    }

    macro_rules! munch {
        ($lit:expr) => {
            {
                fn munch(toks: Toks) -> Result<()> {
                    match toks.get(0) {
                        Some(&ref tok) if &tok[..] == $lit => Parsed((), &toks[1..]),
                        _ => Mismatch(concat!("expected ", stringify!($lit)).into(), toks)
                    }
                }
                munch
            }
        };
    }

    // expression => assignment;
    pub fn expression(toks: Toks) -> Result<Node> {
        assignment(toks)
    }

    // assignment => conditional; // Skip
    fn assignment(toks: Toks) -> Result<Node> {
        guard_empty(toks).and_then(conditional)
    }

    // conditional => logical_or ("?" expression ":" assignment)?;
    fn conditional(toks: Toks) -> Result<Node> {
        logical_or(toks)
            .and_then_with(into_dyn_move)
            .and_then_with(|mut cond, toks| munch!("?")(toks)
                .and_then(expression)
                .and_then_with(|then_expr, toks| munch!(":")(toks)
                    .and_then(assignment)
                    .and_then_with(|else_expr, toks|
                        Parsed(
                            Node::Conditional {
                                cond: Box::new(cond.take()),
                                then_expr: Box::new(then_expr),
                                else_expr: Box::new(else_expr),
                            },
                            toks
                        )
                    )
                )
                .or_else(|toks| Parsed(cond.take(), toks))
            )
    }

    macro_rules! binary_op {
        ($op:expr, $op_name:ident, $parse_rhs:expr, or_else_with: $fallback:expr) => {
            {
                fn binary_op(lhs: Node, toks: Toks) -> Result<Node> {
                    let mut lhs = DynMove(Some(lhs));
                    munch!($op)(toks)
                        .and_then($parse_rhs)
                        .and_then_with(|rhs, toks|
                            Parsed(
                                Node::Binary(BinOp::$op_name, Box::new(lhs.take()), Box::new(rhs)),
                                toks
                            )
                        )
                        .or_else(|toks| ($fallback)(lhs.take(), toks))
                }
                binary_op
            }
        };

        ($op:expr, $op_name:ident, $parse_rhs:expr) => {
            binary_op!($op, $op_name, $parse_rhs, or_else_with: |lhs, toks| Parsed(lhs, toks))
        };
    }

    // logical_or => logical_and ("||" logical_or)?;
    fn logical_or(toks: Toks) -> Result<Node> {
        logical_and(toks).and_then_with(binary_op!("||", LogOr, logical_or))
    }

    // logical_and => bitwise_or ("&&" logical_and)?;
    fn logical_and(toks: Toks) -> Result<Node> {
        bitwise_or(toks).and_then_with(binary_op!("&&", LogAnd, logical_and))
    }

    // bitwise_or => bitwise_xor ("|" bitwise_or)?;
    fn bitwise_or(toks: Toks) -> Result<Node> {
        bitwise_xor(toks).and_then_with(binary_op!("|", BitOr, bitwise_or))
    }

    // bitwise_xor => bitwise_and ("^" bitwise_xor)?;
    fn bitwise_xor(toks: Toks) -> Result<Node> {
        bitwise_and(toks).and_then_with(binary_op!("^", BitXor, bitwise_xor))
    }

    // bitwise_and => equality ("&" bitwise_and)?;
    fn bitwise_and(toks: Toks) -> Result<Node> {
        equality(toks).and_then_with(binary_op!("&", BitAnd, bitwise_and))
    }

    // equality => relational (("==" | "!=") equality)?;
    fn equality(toks: Toks) -> Result<Node> {
        relational(toks).and_then_with(
            binary_op!("==", Eq, equality,
                or_else_with: binary_op!("!=", Ne, equality)
            )
        )
    }

    // relational => shift (("<=" | "<" | ">" | ">=") relational)?;
    fn relational(toks: Toks) -> Result<Node> {
        shift(toks).and_then_with(
            binary_op!("<=", Le, relational,
                or_else_with: binary_op!("<", Lt, relational,
                    or_else_with: binary_op!(">", Gt, relational,
                        or_else_with: binary_op!(">=", Ge, relational)
                    )
                )
            )
        )
    }

    // shift => additive (("<<" | ">>") shift)?;
    fn shift(toks: Toks) -> Result<Node> {
        additive(toks).and_then_with(
            binary_op!("<<", Shl, shift,
                or_else_with: binary_op!(">>", Shr, shift)
            )
        )
    }

    // additive => multiplicative (("+" | "-") additive)?;
    fn additive(toks: Toks) -> Result<Node> {
        multiplicative(toks).and_then_with(
            binary_op!("+", Add, additive,
                or_else_with: binary_op!("-", Sub, additive)
            )
        )
    }

    // multiplicative => pm (("*" | "/" | "%") multiplicative)?;
    fn multiplicative(toks: Toks) -> Result<Node> {
        pm(toks).and_then_with(
            binary_op!("*", Add, multiplicative,
                or_else_with: binary_op!("/", Sub, multiplicative,
                    or_else_with: binary_op!("%", Rem, multiplicative)
                )
            )
        )
    }

    // pm => cast; // Skip
    fn pm(toks: Toks) -> Result<Node> {
        cast(toks)
    }

    // cast => "(" type_id ")" cast | unary;
    fn cast(toks: Toks) -> Result<Node> {
        munch!("(")(toks)
            .and_then(type_id)
            .and_then_with(|ty, toks| munch!(")")(toks)
                .and_then(cast)
                .and_then_with(|expr, toks|
                    Parsed(
                        Node::Cast {
                            value: Box::new(expr),
                            ty: Box::new(ty),
                        },
                        toks
                    )
                )
            )
            .or_else(unary)
    }

    /*

    type_id => identifier "*"? // Simplified.  A lot.

    */
    fn type_id(toks: Toks) -> Result<Node> {
        parse_identifier(toks)
            .and_then_with(|ident, toks| {
                let mut ident = DynMove(Some(ident));
                munch!("*")(toks)
                    .map(|_| Node::Type(ident.take(), true))
                    .or_else(|toks| Parsed(Node::Type(ident.take(), false), toks))
            })
    }

    // unary => ("-" | "~") cast | postfix; // REWIND cast
    fn unary(toks: Toks) -> Result<Node> {
        munch!("-")(toks)
            .and_then(cast)
            .and_then_with(|expr, toks|
                Parsed(
                    Node::Unary(UnaryOp::Neg, Box::new(expr)),
                    toks
                )
            )
        .or_else(|toks| munch!("~")(toks)
            .and_then(cast)
            .and_then_with(|expr, toks|
                Parsed(
                    Node::Unary(UnaryOp::Com, Box::new(expr)),
                    toks
                )
            )
        )
        .or_else(postfix)
    }

    // postfix => primary ( "(" expression_list? ")" )*;
    fn postfix(toks: Toks) -> Result<Node> {
        fn try_call(lhs: Node, toks: Toks) -> Result<Node> {
            let mut lhs = DynMove(Some(lhs));
            munch!("(")(toks)
                .and_then(expression_list_opt)
                .and_then_with(|expr_list, toks| munch!(")")(toks)
                    .and_then(|toks|
                        Parsed(
                            Node::Call {
                                subject: Box::new(lhs.take()),
                                args: expr_list,
                            },
                            toks
                        )
                    )
                )
                .and_then_with(try_call)
                .or_else(|toks| Parsed(lhs.take(), toks))
        }

        primary(toks).and_then_with(try_call)
    }

    // expression_list => assignment ( "," expression_list ); // REWIND assignment
    fn expression_list_opt(toks: Toks) -> Result<Vec<Node>> {
        fn try_next(args: Vec<Node>, toks: Toks) -> Result<Vec<Node>> {
            let mut args = DynMove(Some(args));
            munch!(",")(toks)
                .and_then(assignment)
                .and_then_with(|arg, toks| {
                    let mut args = args.take();
                    args.push(arg);
                    Parsed(args, toks)
                })
                .and_then_with(try_next)
                .or_else(|toks| Parsed(args.take(), toks))
        }

        assignment(toks)
            .map(|node| vec![node])
            .and_then_with(try_next)
            .or_else(|toks| Parsed(vec![], toks))
    }

    // primary => literal | identifier | "(" expression ")"; // REWIND expression
    fn primary(toks: Toks) -> Result<Node> {
        literal(toks)
            .or_else(identifier)
            .or_else(|toks| munch!("(")(toks)
                .and_then(expression)
                .and_then_with(|expr, toks| munch!(")")(toks)
                    .and_then(|toks|
                        Parsed(expr, toks)
                    )
                )
            )
    }

    // literal => integer_literal | character_literal | floating_literal | string_literal;
    fn literal(toks: Toks) -> Result<Node> {
        integer_literal(toks)
            // .or_else(character_literal)
            // .or_else(floating_literal)
            .or_else(string_literal)
    }

    lazy_static! {
        static ref RE_IDENT_LITERAL: Regex = Regex::new(r#"^([A-Za-z_][A-Za-z0-9_]*)$"#).unwrap();
        static ref RE_INT_DEC_LITERAL: Regex = Regex::new(r#"^([0-9]+)([uU]?[lL]?|[lL]?[uU]?)$"#).unwrap();
        static ref RE_INT_HEX_LITERAL: Regex = Regex::new(r#"^0[Xx]([0-9A-Fa-f]+)([uU]?[lL]?|[lL]?[uU]?)$"#).unwrap();
        static ref RE_STR_LITERAL: Regex = Regex::new(r#"^(L?)"(.*)"$"#).unwrap();
    }

    fn identifier(toks: Toks) -> Result<Node> {
        parse_identifier(toks).map(|ident| Node::Ident(ident))
    }

    fn parse_identifier(toks: Toks) -> Result<String> {
        check_empty!(toks);

        if let Some(cap) = RE_IDENT_LITERAL.captures(&toks[0]) {
            return Parsed(
                cap.at(1).unwrap().into(),
                &toks[1..]
            );
        }

        Mismatch("expected identifier".into(), toks)
    }

    fn integer_literal(toks: Toks) -> Result<Node> {
        check_empty!(toks);

        fn parse_int<'a>(digits: &str, suffix: &str, radix: u32, toks: Toks<'a>) -> Result<'a, Node> {
            let sign = if suffix.contains("u") || suffix.contains("U") { Signed::No } else { Signed::Yes };
            let size = if suffix.contains("l") || suffix.contains("L") { Size::Long } else { Size::Unknown };
            return Parsed(
                Node::Integer(u64::from_str_radix(digits, radix).unwrap(), sign, size),
                toks
            );
        }

        if let Some(cap) = RE_INT_DEC_LITERAL.captures(&toks[0]) {
            let digits = cap.at(1).unwrap();
            let suffix = cap.at(2).unwrap();
            return parse_int(digits, suffix, 10, &toks[1..]);
        }

        if let Some(cap) = RE_INT_HEX_LITERAL.captures(&toks[0]) {
            let digits = cap.at(1).unwrap();
            let suffix = cap.at(2).unwrap();
            return parse_int(digits, suffix, 16, &toks[1..]);
        }

        Mismatch("expected integer literal".into(), toks)
    }

    fn string_literal(toks: Toks) -> Result<Node> {
        check_empty!(toks);

        if let Some(cap) = RE_STR_LITERAL.captures(&toks[0]) {
            let prefix = cap.at(1).unwrap();
            let content = cap.at(2).unwrap();
            let width = if prefix == "L" { CharWidth::Wide } else { CharWidth::Narrow };
            return Parsed(
                Node::String(content.into(), width),
                &toks[1..]
            )
        }

        Mismatch("expected string literal".into(), toks)
    }
}
