use crate::ast::value_node::ValueNode;
use crate::ast::value_node::ValueType::{FLOAT, NUMBER};

pub trait Node{
    fn visit_node(&self)->ValueNode;
}

pub struct ProgramNode{
    pub program_nodes:Vec<Box<dyn Node> >
}
impl Node for ProgramNode{
    fn visit_node(&self) -> ValueNode {
        todo!()
    }
}

pub struct NumberNode{
    pub number:i32
}
impl Node for NumberNode{
    fn visit_node(&self) -> ValueNode {
        ValueNode{
            value_type:NUMBER,
            int:Some(self.number),
            string:None,
            bool:None,
            float:None
        }
    }
}

pub struct FloatNode{
    number:f32
}
impl Node for FloatNode{
    fn visit_node(&self) -> ValueNode {
        ValueNode{
            value_type:FLOAT,
            int:None,
            string:None,
            bool:None,
            float:Some(self.number)
        }
    }
}