use crate::backend::{
    ast::nodes::{
        ArrayNode, BinaryOpNode, BoolNode, CallType, FloatNode, FunctionCallNode, NumberNode,
        ProgramNode, StringNode, VariableAccessNode, VariableAssignNode, VariableDefineNode,
    },
    buildin_macros::get_macro::MacroManager,
    compiler::{
        comptime_variable_checker::{
            comptime_context::{CompileContext, ComptimeVariable},
            comptime_value_for_check::ComptimeValueType::{
                self, Array, Bool, Number, StringValue, Void,
            },
        },
        instructions::Instructions::{
            self, Add, Div, Halt, LoadVar, Mul, PushBool, PushNumber, PushString, Sub,
        },
        optimization::optimze::optimize,
    },
    errors::compiler::compiler_errors::CompileError::{
        self, CannotInferType, TypeMismatch, VariableRecreation,
    },
    lexer::tokens::TokenKind::{self, TRUE},
};
use CompileError::ConstantWithoutValue;
use std::fmt::{self, Debug, Formatter};

pub trait Compilable: Debug {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError>;
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result;
}
pub fn indent_fn(n: usize) -> String {
    "  ".repeat(n)
}

pub struct Compiler {
    pub context: CompileContext,
    pub out: Vec<Instructions>,
    pub macros: MacroManager,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            context: CompileContext::new(),
            out: Vec::new(),
            macros: MacroManager::new(),
        }
    }
    pub fn optimize(&mut self) {
        let code = self.out.clone();
        self.out = optimize(code);
    }
}

impl Compilable for NumberNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        compiler.out.push(PushNumber(self.number as f32));
        Ok(Number)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Number({})", indent_fn(indent), self.number)
    }
}

impl Compilable for FloatNode {
    fn compile(&self, out: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        out.out.push(PushNumber(self.number));
        Ok(Number)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Float({})", indent_fn(indent), self.number)
    }
}

impl Compilable for BinaryOpNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        let right = self.left.compile(compiler)?;
        let left = self.right.compile(compiler)?;
        match self.op_tok {
            TokenKind::PLUS => match (&left, &right) {
                (Number, Number) => {
                    compiler.out.push(Add);
                    Ok(Number)
                }
                (StringValue, StringValue) => {
                    compiler.out.push(Add);
                    Ok(StringValue)
                }
                _ => Err(CompileError::InvalidBinaryOp {
                    op: "+",
                    left,
                    right,
                }),
            },
            TokenKind::MINUS => {
                if let Number = right {
                    compiler.out.push(Sub);
                    Ok(Number)
                } else {
                    Err(CompileError::InvalidBinaryOp {
                        op: "-",
                        left,
                        right,
                    })
                }
            }
            TokenKind::TIMES => {
                if let Number = right {
                    compiler.out.push(Mul);
                    Ok(Number)
                } else {
                    Err(CompileError::InvalidBinaryOp {
                        op: "*",
                        left,
                        right,
                    })
                }
            }
            TokenKind::DIVIDE => {
                if let Number = right {
                    compiler.out.push(Div);
                    Ok(Number)
                } else {
                    Err(CompileError::InvalidBinaryOp {
                        op: "/",
                        left,
                        right,
                    })
                }
            }
            TokenKind::GREATER => {
                if let Number = right {
                    compiler.out.push(Instructions::GreaterThan);
                    Ok(Bool)
                } else {
                    Err(CompileError::InvalidBinaryOp {
                        op: ">",
                        left,
                        right,
                    })
                }
            }
            TokenKind::LESS => {
                if let Number = right {
                    compiler.out.push(Instructions::LessThan);
                    Ok(Bool)
                } else {
                    Err(CompileError::InvalidBinaryOp {
                        op: "<",
                        left,
                        right,
                    })
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}BinaryOp({:?})", indent_fn(indent), self.op_tok)?;
        self.left.fmt_with_indent(f, indent + 2)?;
        self.right.fmt_with_indent(f, indent + 2)?;
        Ok(())
    }
}
impl Compilable for ProgramNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        for program_node in &self.program_nodes {
            program_node.compile(compiler)?;
        }
        compiler.out.push(Halt);
        Ok(Void)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Program", indent_fn(indent))?;
        for node in &self.program_nodes {
            node.fmt_with_indent(f, indent + 1)?;
        }
        Ok(())
    }
}

impl Compilable for VariableAccessNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        let var = compiler.context.variables.get(&self.variable_name).ok_or(
            CompileError::UndefinedVariable {
                name: self.variable_name.clone(),
            },
        )?;
        compiler.out.push(LoadVar(self.variable_name.clone()));
        Ok(var.value_type.clone())
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Var({})", indent_fn(indent), self.variable_name)
    }
}

impl Compilable for StringNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        compiler.out.push(PushString(self.value.clone()));
        Ok(StringValue)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}String({})", indent_fn(indent), self.value)
    }
}

impl Compilable for VariableDefineNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        if compiler
            .context
            .variables
            .contains_key(&self.var_name.clone())
        {
            return Err(VariableRecreation {
                name: self.var_name.clone(),
            });
        }
        if self.is_const && self.value.is_none() {
            return Err(ConstantWithoutValue {
                name: self.var_name.clone(),
            });
        }
        /*
        Type
        */
        let inferred_type = if let Some(value) = &self.value {
            Some(value.compile(compiler)?)
        } else {
            None
        };
        let declared_type = if let Some(t) = &self.value_type {
            Some(CompileContext::get_type(t)?)
        } else {
            None
        };

        let final_type = match (declared_type, inferred_type) {
            (Some(d), Some(i)) if d == i => d,
            (Some(d), Some(i)) => {
                return Err(TypeMismatch {
                    expected: d,
                    found: i,
                });
            }
            (Some(d), None) => {
                match d {
                    StringValue => compiler.out.push(PushString("".to_string())),
                    Number => compiler.out.push(PushNumber(0f32)),
                    Bool => compiler.out.push(PushBool(false)),
                    Array(t) => {
                        todo!()
                    }
                    Void => {
                        unreachable!()
                    }
                }
                d
            }
            (None, Some(i)) => i,
            (None, None) => {
                return Err(CannotInferType {
                    name: self.var_name.clone(),
                });
            }
        };

        compiler.context.variables.insert(
            self.var_name.clone(),
            ComptimeVariable {
                value_type: final_type,
                is_const: self.is_const,
            },
        );
        compiler
            .out
            .push(Instructions::SaveVar(self.var_name.clone()));
        Ok(Void)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        write!(f, "{}var:{:?}=", indent_fn(indent), self.value_type)?;
        if let Some(value) = &self.value {
            value.fmt_with_indent(f, 0)?;
        } else {
            write!(f, "None")?;
        }
        Ok(())
    }
}

impl Compilable for BoolNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        compiler
            .out
            .push(PushBool(if self.value == TRUE { true } else { false }));
        Ok(Bool)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}String({:?})", indent_fn(indent), self.value)
    }
}

impl Compilable for VariableAssignNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        let (is_const, expected_type) = {
            let var = compiler.context.variables.get(&self.name).ok_or(
                CompileError::UndefinedVariable {
                    name: self.name.clone(),
                },
            )?;
            (var.is_const, var.value_type.clone())
        };
        if is_const {
            return Err(CompileError::ConstReassignment {
                name: self.name.clone(),
            });
        }

        let value_type = self.value.compile(compiler)?;

        if value_type != expected_type {
            return Err(TypeMismatch {
                expected: expected_type,
                found: value_type,
            });
        }

        compiler.out.push(Instructions::SaveVar(self.name.clone()));
        Ok(value_type)
    }
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}{}=", indent_fn(indent), self.name)?;
        self.value.fmt(f)?;
        Ok(())
    }
}
/*
 * Array node
 */
impl Compilable for ArrayNode {
    fn compile(&self, _compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        todo!()
    }

    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        writeln!(f, "{}Array [", " ".repeat(indent))?;
        for element in &self.elements {
            element.fmt_with_indent(f, indent + 2)?;
        }
        writeln!(f, "{}]", " ".repeat(indent))
    }
}
impl Compilable for FunctionCallNode {
    fn compile(&self, compiler: &mut Compiler) -> Result<ComptimeValueType, CompileError> {
        match self.call_type {
            CallType::Macro => {
                let mac = compiler.macros.macros.remove(&self.name).ok_or(
                    CompileError::UnknownMacro {
                        name: self.name.clone(),
                    },
                )?;
                let result = mac.compile(compiler, &self.args);
                compiler.macros.macros.insert(self.name.clone(), mac);
                result
            }
            CallType::Fn => {
                unreachable!()
            }
        }
    }
    fn fmt_with_indent(&self, _f: &mut Formatter<'_>, _indent: usize) -> fmt::Result {
        writeln!(_f, "{}{}(...)", indent_fn(_indent), self.name)
    }
}
