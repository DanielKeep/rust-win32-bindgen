/*!
Contains all the code to deal with parsing and evaluating conditional compilation expressions.

Conditional expressions are *not* processed the same way they are in an actual C pre-processor.  Rather than just computing an integer value, we're concerned with working out what feature set a given expression represents.  To put it another way, for what set of configurations will the expression evaluate to logical true?

As a result, the AST, value representation, and evaluation semantics are weird.  For example, "Windows version" is a first-class concept.
*/
pub mod ast;
pub mod eval;
pub mod parse;

pub use self::ast::Node;
pub use self::eval::Value;
pub use self::parse::parse_conditional_expr;
