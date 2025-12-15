use crate::errors::lexer_errors::LexerError;
use crate::errors::lexer_errors::LexerErrorType::{MoreDotInANumberError, UnknownTokenError};
use crate::lexer::tokens::{Token};
use crate::lexer::tokens::TokenKind::{CONST, DIVIDE, EOF, FLOAT, FN, IDENTIFIER, MINUS, NUMB, PLUS, STR, TIMES, VAR};

pub struct Tokenizer {
    current_token:char,
    token_idx:usize,
    token_count:usize,
    source_text:Vec<char>,
    final_tokens:Vec<Token>,

}


impl Tokenizer {
    pub fn new(text:String) -> Self {
        Self{
            token_idx:0,
            token_count:text.len(),
            current_token:'0',
            source_text: text.chars().collect(),
            final_tokens:Vec::new(),

        }
    }
    pub fn tokenize(&mut self)->Result<&Vec<Token>,LexerError>{
        self.current_token=self.source_text[0];
        while self.current_token!='\0' {
            match self.current_token {
                ' ' | '\n'|'\t'=>{
                    self.advance();
                    continue},
                '+' => self.final_tokens.push(
                    Token {
                    token_kind: PLUS,
                    token_value: self.current_token.to_string(),
                    }
                ),
                '-'=> self.final_tokens.push(
                    Token{
                        token_kind:MINUS,
                        token_value:self.current_token.to_string()
                    }
                ),
                '*'=> self.final_tokens.push(
                    Token{
                        token_kind:TIMES,
                        token_value:self.current_token.to_string()
                    }
                ),
                '/'=> self.final_tokens.push(
                    Token{
                        token_kind:DIVIDE,
                        token_value:self.current_token.to_string()
                    }
                ),
                _ => {
                    if self.current_token.is_alphabetic(){
                        let token = self.create_text_token();
                        self.final_tokens.push(token);
                        continue;
                    }
                    else if self.current_token.is_numeric() {
                        let token = self.create_number_token()?;
                        self.final_tokens.push(token);
                        continue
                    }
                    else {
                        return Err(LexerError {
                            error_type: UnknownTokenError,
                            wrong_token: self.current_token.to_string(),
                        })
                    }

                },
            }
            self.advance();

        }
        self.final_tokens.push(Token{token_kind:EOF,token_value:"EOF".to_string()});
        Ok(&self.final_tokens)
    }
    fn advance(&mut self){
        self.token_idx+=1;
        if self.token_idx>=self.token_count{
            self.current_token='\0';
        }else {
            self.current_token = self.source_text[self.token_idx];
        }
    }
    fn create_number_token(&mut self)->Result<Token,LexerError> {
        let mut number_buffer:String=String::new();
        let mut dot_count:usize=0;
        while self.current_token.is_numeric() || self.current_token=='.' {
                if self.current_token=='.'{
                    if dot_count <1{
                        dot_count+=1;
                        number_buffer.push('.')
                    }else {
                        return Err(LexerError{wrong_token:number_buffer,error_type:MoreDotInANumberError});
                    }
                }else {
                    number_buffer.push(self.current_token);
                }
            self.advance();
        }
        Ok(Token{
            token_kind:if dot_count<1 {NUMB} else { FLOAT },
            token_value:number_buffer
        })
    }
    fn create_text_token(&mut self) -> Token{
        let mut text_buffer:String = String::new();
        while self.current_token.is_alphabetic() {
            text_buffer.push(self.current_token);
            self.advance()
        }
        match text_buffer.as_str() {
            "var" =>
                Token{
                    token_kind:VAR,
                    token_value:text_buffer
                },
            "fn" =>
                Token{
                    token_kind:FN,
                    token_value:text_buffer
                },
            "str" =>
                Token{
                    token_kind:STR,
                    token_value:text_buffer
                },
            "const" =>
                Token{
                    token_kind:CONST,
                    token_value:text_buffer
                },
            _ =>
                Token{
                    token_kind:IDENTIFIER,
                    token_value:text_buffer
                }
        }
    }
}