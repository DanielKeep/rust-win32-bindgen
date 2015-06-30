/*!
Contains the conditional compilation expression parser.
*/
use std::fmt::Debug;
use regex::Regex;
use super::ast::Node;
use util::ResultOptionExt;

/**
Shorthand for matching a sequence of tokens.

You might wonder why this is used so little for being so much code.  Answer: a lot of it got removed because it was hateful.  :P
*/
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

/**
Guards against attempting to parse an empty list of tokens.

This is a problem because `Ok(None)` will continue to try `ro_or_else` branches.

The *correct* solution to this, would, of course, to have been to define a proper monad with a distinct "out of input, give up" short-circuit variant... but I'm too lazy.

Just ensure that `parse_guard!(toks)` is at the start of every `parse_*` function and you'll be fine.
*/
macro_rules! parse_guard {
    ($toks:expr) => {
        if $toks.len() == 0 { return Ok(None); }
    };
}

lazy_static! {
    static ref RE_INT_DEC_LITERAL: Regex = Regex::new(r#"^([0-9]+)$"#).unwrap();
    static ref RE_INT_HEX_LITERAL: Regex = Regex::new(r#"^0[Xx]([0-9A-Fa-f]+)$"#).unwrap();
    static ref RE_IDENT_LITERAL: Regex = Regex::new(r#"^([A-Za-z_][A-Za-z0-9_]*)$"#).unwrap();
}

/// Shorthand for the parse result type.
pub type ParseResult<'a, S> = Result<Option<(Node, &'a [S])>, String>;

// http://www.nongnu.org/hcb/#conditional-expression

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
