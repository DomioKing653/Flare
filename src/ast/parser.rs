use crate::ast::nodes::CallType::{Fn, Macro};
use crate::ast::nodes::{BoolNode, FunctionCallNode, VariableAssignNode};
use crate::lexer::tokens::TokenKind::{COMMA, FALSE, SEMICOLON, TRUE};
use crate::statements::if_statement::IfStatement;
use crate::{
    ast::nodes::{
        BinaryOpNode, FloatNode, NumberNode, ProgramNode, StringNode, VariableAccessNode,
        VariableDefineNode,
    },
    compiler::byte_code::Compilable,
    errors::parser_errors::{ParserError, ParserError::UnexpectedToken},
    lexer::tokens::{
        Token,
        TokenKind::{
            self, CLOSINGBRACE, COLON, CONST, DIVIDE, ELSE, EOF, EQUAL, FLOAT, IDENTIFIER, IF,
            LEFTPAREN, MINUS, NUMB, OPENINGBRACE, PLUS, RIGHTPAREN, STRING, TIMES, VALUE, VAR,
        },
    },
};

pub struct Parser {
    tokens: Vec<Token>,
    token_idx: usize,
}

impl Parser {
    pub fn new(token_list: Vec<Token>) -> Self {
        Self {
            tokens: token_list,
            token_idx: 0,
        }
    }

    pub fn current_token(&self) -> &Token {
        &self.tokens[self.token_idx]
    }

    pub fn advance(&mut self) {
        self.token_idx += 1;
    }

    pub fn parse(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        let mut program: ProgramNode = ProgramNode::new();
        while self.current_token().token_kind != EOF {
            program.program_nodes.push(self.parse_stmt()?)
        }
        Ok(Box::new(program))
    }

    fn parse_stmt(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        match &self.current_token().token_kind {
            VAR | CONST => {
                let value = self.parse_var_decl_stmt();
                self.expect(SEMICOLON)?;
                value
            }
            IDENTIFIER if self.peek() == EQUAL => {
                let id = self.current_token().token_value.clone();
                self.advance();
                self.expect(EQUAL)?;
                let value = self.parse_expr()?;
                self.expect(SEMICOLON)?;
                Ok(Box::new(VariableAssignNode { name: id, value }))
            }
            IF => {
                self.advance();
                self.expect(LEFTPAREN)?;
                let condition = self.parse_expr()?;
                self.expect(RIGHTPAREN)?;
                self.expect(OPENINGBRACE)?;
                let mut body: Vec<Box<dyn Compilable>> = Vec::new();
                while self.current_token().token_kind != CLOSINGBRACE {
                    if self.current_token().token_kind == EOF {
                        return Err(ParserError::UnexpectedToken {
                            found: "EOF".into(),
                            expected: SEMICOLON,
                        });
                    }
                    body.push(self.parse_stmt()?);
                }
                self.expect(CLOSINGBRACE)?;
                if self.current_token().token_kind == ELSE {
                    self.advance();
                    self.expect(OPENINGBRACE)?;
                    let mut else_body: Vec<Box<dyn Compilable>> = Vec::new();
                    while self.current_token().token_kind != CLOSINGBRACE {
                        if self.current_token().token_kind == EOF {
                            return Err(ParserError::UnexpectedToken {
                                found: "EOF".into(),
                                expected: SEMICOLON,
                            });
                        }
                        else_body.push(self.parse_stmt()?);
                    }
                    self.expect(CLOSINGBRACE)?;
                    return Ok(Box::new(IfStatement {
                        condition,
                        then_branch: body,
                        else_branch: Some(else_body),
                    }));
                }
                return Ok(Box::new(IfStatement {
                    condition,
                    then_branch: body,
                    else_branch: None,
                }));
            }
            _ => self.parse_expr(),
        }
    }

    fn parse_var_decl_stmt(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        let is_const: bool;
        if self.current_token().token_kind == CONST {
            is_const = true;
        } else {
            is_const = false;
        }
        let id: String;
        self.advance();
        id = self.expect(IDENTIFIER)?.token_value;
        let mut value_type = None;

        if self.current_token().token_kind == COLON {
            self.advance();

            value_type = Some(self.expect(IDENTIFIER)?.token_value);
        }
        let value: Option<Box<dyn Compilable>>;
        if self.current_token().token_kind == EQUAL {
            self.advance();
            value = Some(self.parse_expr()?);
        } else {
            value = None
        }
        Ok(Box::new(VariableDefineNode {
            value_type,
            value,
            var_name: id,
            is_const,
        }))
    }

    fn parse_expr(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        let mut term = self.parse_term()?;
        while self.current_token().token_kind == PLUS || self.current_token().token_kind == MINUS {
            let operator = self.current_token().token_kind.clone();
            self.advance();
            term = Box::new(BinaryOpNode {
                left: term,
                right: self.parse_term()?,
                op_tok: operator,
            });
        }
        Ok(term)
    }

    fn parse_term(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        let mut factor = self.parser_factor()?;
        while self.current_token().token_kind == TIMES || self.current_token().token_kind == DIVIDE
        {
            let operator = self.current_token().token_kind.clone();
            self.advance();
            factor = Box::new(BinaryOpNode {
                left: factor,
                right: self.parser_factor()?,
                op_tok: operator,
            });
        }
        Ok(factor)
    }

    fn parser_factor(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        if self.current_token().token_kind == FLOAT {
            let value = match self.current_token().token_value.parse::<f32>() {
                Err(_) => unreachable!(),
                Ok(numb) => numb,
            };
            self.advance();
            Ok(Box::new(FloatNode { number: value }))
        } else if self.current_token().token_kind == TRUE
            || self.current_token().token_kind == FALSE
        {
            let value = self.current_token().token_kind.clone();
            self.advance();
            Ok(Box::new(BoolNode { value }))
        } else if self.current_token().token_kind == NUMB {
            let value = match self.current_token().token_value.parse::<i32>() {
                Ok(numb) => numb,
                Err(_) => unreachable!(),
            };
            self.advance();
            Ok(Box::new(NumberNode { number: value }))
        } else if self.current_token().token_kind == IDENTIFIER {
            let value = self.current_token().token_value.clone();
            self.advance();

            if self.current_token().token_kind == LEFTPAREN {
                self.advance();
                let mut args: Vec<Box<dyn Compilable>> = Vec::new();

                if self.current_token().token_kind != RIGHTPAREN {
                    loop {
                        args.push(self.parse_expr()?);

                        if self.current_token().token_kind == COMMA {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                self.expect(RIGHTPAREN)?;
                let is_macro = value.ends_with('!');
                let name = value.trim_end_matches('!').to_string();

                Ok(Box::new(FunctionCallNode {
                    args,
                    name,
                    call_type: if is_macro { Macro } else { Fn },
                }))
            } else {
                Ok(Box::new(VariableAccessNode {
                    variable_name: value,
                }))
            }
        } else if self.current_token().token_kind == LEFTPAREN {
            self.advance();
            let value = self.parse_expr()?;
            self.expect(RIGHTPAREN)?;
            self.advance();
            Ok(value)
        } else if self.current_token().token_kind == STRING {
            let value = StringNode {
                value: self.current_token().token_value.clone(),
            };
            self.advance();
            Ok(Box::new(value))
        } else {
            Err(UnexpectedToken {
                found: self.current_token().token_value.clone(),
                expected: VALUE,
            })
        }
    }

    fn expect(&mut self, token_kind: TokenKind) -> Result<Token, ParserError> {
        if self.current_token().token_kind == token_kind {
            let token = self.current_token().clone();
            self.advance();
            Ok(token)
        } else {
            Err(UnexpectedToken {
                expected: token_kind,
                found: self.current_token().token_value.clone(),
            })
        }
    }

    fn peek(&self) -> TokenKind {
        let idx = self.token_idx + 1;
        self.tokens[idx].token_kind.clone()
    }
}
