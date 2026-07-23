use crate::backend::compiler::byte_code::Compilable;
use crate::backend::lexer::tokens::TokenKind;
use std::fmt;
use std::fmt::{Debug, Formatter};
/*
 * Unary expresion node
 */

#[derive(Clone)]
pub struct PrefixExpressionNode {
    pub prefix: TokenKind,
    pub value: Box<dyn Compilable>,
}

/*
Number Node
*/

#[derive(Clone)]
pub struct NumberNode {
    pub number: i64,
}

impl fmt::Debug for NumberNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
Float node
*/
#[derive(Clone)]
pub struct FloatNode {
    pub number: f32,
}

impl fmt::Debug for FloatNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

/*
 * String node
 */

#[derive(Clone)]
pub struct StringNode {
    pub value: String,
}
impl Debug for StringNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
/*
 * Bool node
 */
#[derive(Clone)]
pub struct BoolNode {
    pub value: TokenKind,
}
impl Debug for BoolNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

/*
 * Array node
 */

#[derive(Clone)]
pub struct ArrayNode {
    pub elements: Vec<Box<dyn Compilable>>,
}
impl Debug for ArrayNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}