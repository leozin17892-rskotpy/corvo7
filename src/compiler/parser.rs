use crate::compiler::lexer::{Token, TokenKind};
use std::fmt;
use std::collections::HashMap;

pub struct Parser{
    tokens: Vec<Token>,
    pos: usize,
    variables: HashMap<String, VarDecl>
}
#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl(VarDecl),
    FuncDecl(FuncDecl),
    IfStatement(IfStatement),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    Return(Expr),
    Print(Vec<Expr>),
    CompoundAssign{
        target: String,
        op: BinOp,
        value: Expr,
    },
    Assignment(Assignment),
    Break,
    Continue
}
#[derive(Debug, Clone)]
pub struct WhileLoop{
    pub cond: Expr,
    pub body: Vec<Stmt>
}
#[derive(Debug, Clone)]
pub struct Assignment {
    pub target: String,
    pub value: Expr,
}
#[derive(Debug, Clone)]
pub enum ParserErrorKind{
    ExpectedSemiColonKind,
    ExpectedTokenNotFound,
    ExpectedIdentifierNotFound,
    InvalidExpression,
    InvalidVariableDecl,
    DuplicateVariableError,
    UndefinedVariable
}
#[derive(Debug, Clone)]
pub struct ParseResult{
    pub stmts: Vec<Stmt>,
    pub errors: Vec<ParserError>,
    pub success: bool
}
impl ParseResult{
    pub fn unwrap_or_exit(self) -> Vec<Stmt>{
        if !self.errors.is_empty(){
            for err in &self.errors{
                eprintln!("{}", err);
            }
            std::process::exit(1);
        }
        self.stmts
    }
}
#[derive(Debug, Clone)]
pub struct ParserError{
    kind: ParserErrorKind,
    column: usize,
    line: usize,
    error: String,
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parser Error at {}:{} - {} ({:?})",
            self.line, self.column, self.error, self.kind)
    }
}
#[derive(Debug, Clone)]
pub struct IfStatement{
    pub condition: Expr,
    pub then_branch: Vec<Stmt>,
    pub else_branch: Option<Vec<Stmt>>
}
#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub ty: Type,
    pub value: Expr,
    pub mutability: Mutability
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mutability {
    Const,
    Mutable,
}

#[derive(Debug, Clone)]
pub struct FuncDecl{
    pub name: String,
    pub params: Vec<(Type, String)>,
    pub return_type: Option<Type>,
    pub body: Vec<Stmt>, 
}
#[derive(Debug, Clone)]
pub struct ForLoop{
    pub var: String,
    pub start: Expr,
    pub step: Option<Expr>,
    pub end: Expr,
    pub body: Vec<Stmt>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    Int,
    UInt,
    Bool,
    Str,
    Void,
    Vec{
        inner: Box<Type>,
        size: usize,
    }
}
#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Percent,
    DoubleEqual,
    IndentityOp,
    NotEqual,
    LessEqual,
    Less,
    Greater,
    GreaterEqual,
    CompoundAdd,
    CompoundSub,
    CompoundMul,
    CompoundDiv,
}

#[derive(Debug, Clone)]
pub enum Expr {
    UIntLiteral(u64),
    IntLiteral(i64),
    BoolLiteral(bool),
    StringLiteral(String),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Int(i64),
    UInt(u64),
    Vec{
        values: Vec<Expr>, 
        size: usize
    },
    Ident(String),
    Identity {
    	expr: Box<Expr>,
  	  negated: bool,
	},
    Call{
        name: String,
        args: Vec<Expr>
    },
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>
    },
    Unknown
}

fn infer_type_from_expr(expr: &Expr) -> Type {
   	 match expr {
      	  Expr::UIntLiteral(v) => {
          	  if *v <= u8::MAX as u64 {
           	     Type::U8
       	     }else if *v <= u16::MAX as u64 {
           	     Type::U16
          	  } else if *v <= u32::MAX as u64 {
             	   Type::U32
          	  } else if *v <= u64::MAX {
                    Type::UInt
            	}else{
                    Type::U128
                }
        	}
        Expr::IntLiteral(v) => {
            if *v >= i8::MIN as i64 && *v <= i8::MAX as i64 {
                Type::I8
            }else if *v >= i16::MIN as i64 && *v <= i16::MAX as i64 {
                Type::I16
            }else if *v >= i32::MIN as i64 && *v <= i32::MAX as i64 {
                Type::I32
            }else if *v <= i64::MAX && *v >= i64::MIN{
                Type::Int
            }else{
                Type::I128 
            }
        }
        Expr::BoolLiteral(v) => {
            if *v == true || *v == false{
                Type::Bool
            }else{
                panic!("Bool type expected but found: {:?}", v)
            }
        }
        _ => panic!("Não consigo inferir tipo de {:?}", expr),
  	  }
}        
impl Parser{
    pub fn new(tokens: Vec<Token>) -> Self{
        Self { tokens, pos: 0, variables: HashMap::new()}
    }
    pub fn peek(&self) -> &Token{
        &self.tokens[self.pos]
    }
    fn add_var(&mut self, var: VarDecl) {
        self.variables.insert(var.name.clone(), var);
    }
    fn lookup_var(&self, name: &str) -> Option<&VarDecl> {
        self.variables.get(name)
    }
    fn advance(&mut self) -> &Token{
        let tok = &self.tokens[self.pos];
        self.pos += 1;
        tok
    }
    fn is_compound_op(&mut self) -> bool{
        matches!(self.peek().kind, 
            TokenKind::CompoundAdd | TokenKind::CompoundSub | TokenKind::CompoundMul | TokenKind::CompoundDiv
        )
    }
    fn parse_compound_assign(&mut self, target_name: String) -> Result<Stmt, ParserError> {
    	let op = match self.peek().kind {
     	   TokenKind::CompoundAdd => BinOp::CompoundAdd,
      	  TokenKind::CompoundSub => BinOp::CompoundSub,
       	 TokenKind::CompoundMul => BinOp::CompoundMul,
       	 TokenKind::CompoundDiv => BinOp::CompoundDiv,
     	   _ => unreachable!(),
  	  };
    	self.advance(); 
   	 let value = self.parse_expr()?;
   	 self.expect(TokenKind::Semicolon)?;

    	Ok(Stmt::CompoundAssign {
     	   target: target_name,
      	  op,
       	 value,
    	})
	}
    fn peek_next(&self) -> &Token {
    	&self.tokens[self.pos + 1]
	}

    fn parse_print(&mut self) -> Result<Stmt, ParserError>{
        self.expect(TokenKind::Print)?;
        self.expect(TokenKind::LeftParen)?;
        let mut internal = Vec::new();
        while self.peek().kind != TokenKind::RightParen && self.peek().kind != TokenKind::EOF{
            internal.push(self.parse_expr()?)
        }
        self.expect(TokenKind::RightParen)?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Print(internal))
    }
    fn parse_unary(&mut self) -> Result<Expr, ParserError> {
   	 if self.peek().kind == TokenKind::Indentity {
   	     return self.parse_identity();
  	  }

   	 if self.peek().kind == TokenKind::Not {
     	   self.advance();
     	   let right = self.parse_unary()?;
      	  return Ok(Expr::Identity {
       	     expr: Box::new(right),
       	     negated: true,
      	  });
  	  }

  	  self.parse_primary()
	}
    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
    // clone aqui evita problema de borrow
    match self.peek().kind.clone() {
        TokenKind::Indentity => {
  		  return self.parse_identity();
		}
        TokenKind::UIntLiteral(v) => {
            self.advance();
            Ok(Expr::UIntLiteral(v as u64))
        }
        TokenKind::IntLiteral(v) => {
            self.advance();
            Ok(Expr::IntLiteral(v as i64))
        }
        TokenKind::BoolLiteral(v) => {
            self.advance();
            Ok(Expr::BoolLiteral(v))
        }
        TokenKind::StringLiteral(ref v) => {
            let s = v.clone();
            self.advance();
            Ok(Expr::StringLiteral(s))
        }
        TokenKind::Ident(ref name) => {
            let n = name.clone();
            self.advance();
            if self.peek().kind == TokenKind::LeftParen {
                self.advance();
                let mut args = Vec::new();
                while self.peek().kind != TokenKind::RightParen {
                    args.push(self.parse_expr()?);
                    if self.peek().kind == TokenKind::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(TokenKind::RightParen)?;
                Ok(Expr::Call { name: n, args })
            } else {
                Ok(Expr::Ident(n))
            }
        }
        TokenKind::VecLiteral(ref values) => {
            let vals = values.clone(); // evita borrow
            self.advance();
            let expr_values = vals.into_iter().map(|v| Expr::IntLiteral(v as i64)).collect();
            Ok(Expr::Vec { values: expr_values, size: values.len() })
        }
        _ => Err(ParserError{
            kind: ParserErrorKind::InvalidExpression,
            error: format!("Invalid Expression: {:?} in {}:{}",
            self.peek().kind,
            self.peek().line,
            self.peek().column
      	  ),
            column: self.peek().column,
            line: self.peek().line
            })
  	  }
	}
    fn parse_term(&mut self) -> Result<Expr, ParserError>{
        let mut expr = self.parse_unary()?;

        while matches!(self.peek().kind, TokenKind::Star | TokenKind::Slash | TokenKind::Percent) {
            let op = match self.peek().kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                TokenKind::Percent => BinOp::Percent,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
     }   
    
    
    fn expect_ident(&mut self) -> Result<String, ParserError>{
        match &self.peek().kind{
            TokenKind::Ident(name) =>{
          	  let name = name.clone();
       	     self.advance();
                Ok(name)
            },
            _ => Err(ParserError{
                kind: ParserErrorKind::ExpectedIdentifierNotFound,
                error: format!("Expected identifier in {}:{} found {:?}", self.peek().line, self.peek().column, self.peek().kind), 
                column: self.peek().column, 
                line: self.peek().line
            }),
        }
    } 
    
    fn expect(&mut self, kind: TokenKind) -> Result<(), ParserError> {
        if self.peek().kind == kind {
            self.advance();
            Ok(())
        } else {
            Err(ParserError{
                kind: ParserErrorKind::ExpectedTokenNotFound,
                error: format!("Expected {:?}, found {:?} in {}:{}",
                kind,
                self.peek().kind,
                self.peek().line,
                self.peek().column
                ),
                line: self.peek().line,
                column: self.peek().column
                }
            )
        }
    } 
    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParserError> {
   	 let mut stmts = Vec::new();
  	  while self.peek().kind != TokenKind::RightBrace && self.peek().kind != TokenKind::EOF {
   	     stmts.push(self.parse_statement()?);
	    }
  	  self.expect(TokenKind::RightBrace)?;
      Ok(stmts)
	}

    fn parse_while(&mut self) -> Result<Stmt, ParserError>{
        self.expect(TokenKind::While)?;
        self.expect(TokenKind::LeftParen)?;
        
        let cond = self.parse_expr()?;
        self.expect(TokenKind::RightParen)?;
        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_block()?;
        
        Ok(Stmt::WhileLoop(WhileLoop{
            cond,
            body
        }))
    }
    fn parse_for(&mut self) -> Result<Stmt, ParserError> {
 	   self.expect(TokenKind::For)?;
  	  let var = self.expect_ident()?;
   	 self.expect(TokenKind::In)?;

    	let start = self.parse_expr()?; // Consome o 0
   	 self.expect(TokenKind::Range)?; // Consome o primeiro ..

   	 let next_val = self.parse_expr()?; // Consome o 2
    
   	 let mut step = None;
  	  let end;

  	  if let TokenKind::Range = self.peek().kind {
      	  self.advance(); 
     	   step = Some(next_val);
      	  end = self.parse_expr()?; 
   	 } else {
      	  end = next_val;
   	 }

   	 self.expect(TokenKind::LeftBrace)?;
    	let body = self.parse_block()?;

   	 Ok(Stmt::ForLoop(ForLoop { var, start, step, end, body }))
	}

    
    fn parse_type(&mut self) -> Result<Type, ParserError>{
        match &self.peek().kind{
            TokenKind::StringLiteral(_) => {
                self.advance();
                Ok(Type::Str)
            }
            TokenKind::UInt8 => {
                self.advance();
                Ok(Type::U8)
            },
            TokenKind::UInt16 => {
                self.advance();
                Ok(Type::U16)
            }
            TokenKind::UInt32 => {
                self.advance();
                Ok(Type::U32)
            }
            TokenKind::UInt64 => {
                self.advance();
                Ok(Type::U64)
            }
            TokenKind::UInt128 => {
                self.advance();
                Ok(Type::U128)
            }
            TokenKind::Int8 => {
                self.advance();
                Ok(Type::I8)
            }
            TokenKind::Int16 => {
                self.advance();
                Ok(Type::I16)
            }
            TokenKind::Int32 => {
                self.advance();
                Ok(Type::I32)
            }
            TokenKind::Int64 => {
                self.advance();
                Ok(Type::I64)
            }
            TokenKind::Int128 => {
                self.advance();
                Ok(Type::I128)
            }
            TokenKind::Int => {
                self.advance();
                Ok(Type::Int)
            }
            TokenKind::UInt => {
                self.advance();
                Ok(Type::UInt)
            }
            TokenKind::Bool => {
                self.advance();
                Ok(Type::Bool)
            }
            TokenKind::Void => {
                self.advance();
                Ok(Type::Void)
            }
            TokenKind::Vec => {
  			  self.advance();
                self.expect(TokenKind::Less)?;
   			 let inner_type = self.parse_type()?;
                self.expect(TokenKind::Comma)?;
                
  			  let size = match self.peek().kind {
      				  TokenKind::IntLiteral(n) => { self.advance(); n as usize }
      	 			 _ => panic!("Vec object needs Size.")
   			 };
    			self.expect(TokenKind::Greater)?;
    
   			 Ok(Type::Vec {
    			    inner: Box::new(inner_type),
     			   size,
  			  })
			}
            _ => {
                panic!(
            "Esperado tipo, encontrado {:?} em {}:{}",
          	  self.peek().kind,
          	  self.peek().line,
    	        self.peek().column
            )
        
            }   
       }
    }
    fn parse_identity(&mut self) -> Result<Expr, ParserError>{
   	 self.expect(TokenKind::Indentity)?;

  	  let negated = if self.peek().kind == TokenKind::Not{
 	       self.advance();
            true
  	  } else {
    	    false
 	   };

   	 let expr = self.parse_unary()?; 

   	 Ok(Expr::Identity {
    	    expr: Box::new(expr),
   	     negated,
 	   })
	}
    fn parse_equality(&mut self) -> Result<Expr, ParserError>{
        let mut expr = self.parse_comparison()?;
        while matches!(self.peek().kind, 
            TokenKind::NotEqual | TokenKind::DoubleEqual | TokenKind::Indentity 
        ){
            let op = match self.peek().kind {
                TokenKind::DoubleEqual => BinOp::DoubleEqual,
                TokenKind::NotEqual => BinOp::NotEqual,
                TokenKind::Indentity => BinOp::IndentityOp,
                _ => break
            };
            self.advance();
            let right = self.parse_comparison()?;
        	expr = Expr::BinaryOp { 
          	  left: Box::new(expr), 
          	  op, 
           	 right: Box::new(right)
        	};
        }
        Ok(expr)   
    }
    fn parse_comparison(&mut self) -> Result<Expr, ParserError>{
        let mut expr = self.parse_addition()?;
        while matches!(self.peek().kind, 
            TokenKind::GreaterEqual | 
            TokenKind::LessEqual | TokenKind::Greater | 
        	TokenKind::Less 
        ){
            let op = match self.peek().kind {
                TokenKind::GreaterEqual => BinOp::GreaterEqual,
                TokenKind::Greater => BinOp::Greater,
                TokenKind::LessEqual => BinOp::LessEqual,
                TokenKind::Less => BinOp::Less,
                
                _ => break
            };
            self.advance();
            let right = self.parse_addition()?;
        	expr = Expr::BinaryOp {
           	 left: Box::new(expr),
          	  op,
       	     right: Box::new(right),
      	  };
        }
   	 Ok(expr)
    }
    fn parse_addition(&mut self) -> Result<Expr, ParserError>{
        let mut expr = self.parse_term()?;
        while matches!(self.peek().kind, 
            TokenKind::Plus | 
        	TokenKind::Minus
        ){
            let op = match self.peek().kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => break
            };
            self.advance();
            let right = self.parse_term()?;
       	 expr = Expr::BinaryOp {
           	 left: Box::new(expr),
         	   op,
          	  right: Box::new(right),
      	  };
        }
        
   	 Ok(expr)
    }
    fn parse_expr(&mut self) -> Result<Expr, ParserError>{
  	  self.parse_equality()
	}

    fn parse_if(&mut self) -> Result<Stmt, ParserError>{
        
        self.expect(TokenKind::If)?;
        self.expect(TokenKind::LeftParen)?;
        let condition = self.parse_expr()?;
        
        self.expect(TokenKind::RightParen)?;
        self.expect(TokenKind::LeftBrace)?;
        let mut then_branch = Vec::new();
        while self.peek().kind != TokenKind::RightBrace && self.peek().kind != TokenKind::EOF{
            then_branch.push(self.parse_statement()?);
        }
        self.expect(TokenKind::RightBrace)?;
        let mut else_branch = None;
        
        if self.peek().kind == TokenKind::Else{
            self.advance();
            if self.peek().kind == TokenKind::If{
                else_branch = Some(vec![self.parse_if()?])
            }else{
            	self.expect(TokenKind::LeftBrace)?;
          	  let mut else_stmts = Vec::new();
         	   while self.peek().kind != TokenKind::RightBrace && self.peek().kind != TokenKind::EOF{
           	     else_stmts.push(self.parse_statement()?);
          	  }
            	self.expect(TokenKind::RightBrace)?;
           	 else_branch = Some(else_stmts);
            }
        }
        Ok(Stmt::IfStatement(IfStatement{
            condition,
            then_branch,
            else_branch
        }))
    }
    
    fn parse_statement(&mut self) -> Result<Stmt, ParserError>{
        match self.peek().kind {
     	   TokenKind::Fun => Ok(self.parse_funcdecl()?),
     	   TokenKind::Var | TokenKind::Const => Ok(self.parse_valdecl()?),
            TokenKind::If => self.parse_if(),
            TokenKind::For => self.parse_for(),
            TokenKind::While => self.parse_while(),
            TokenKind::Print => self.parse_print(),
            TokenKind::Return => {
                self.advance();
                let value = self.parse_expr()?;
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Return(value))
            }
            TokenKind::Break => {
                self.advance();
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Break)
            }
            TokenKind::Continue => {
                self.advance();
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Continue)
            }
            TokenKind::Ident(_) => {
                let var_name = if let TokenKind::Ident(n) = self.peek().kind.clone() { n } else { unreachable!() };
                self.advance();
                
                if self.is_compound_op(){
                    Ok(self.parse_compound_assign(var_name)?)
                }else if self.peek().kind == TokenKind::Equal {
   				 self.advance(); // consome '='
   				 let value = self.parse_expr()?;
    				self.expect(TokenKind::Semicolon)?;
                    if let Some(var) = self.lookup_var(&var_name) {
  					  if var.mutability == Mutability::Const {
      					  return Err(ParserError {
           					 kind: ParserErrorKind::InvalidVariableDecl,
           					 error: format!("Cannot assign to const variable '{}'", var_name),
          					  line: self.peek().line,
          					  column: self.peek().column,
       					 });
       				 }
    				}
                    if self.lookup_var(&var_name).is_none() {
  					  return Err(ParserError{
      					  kind: ParserErrorKind::UndefinedVariable,
     					   error: format!("assignment to undeclared variable `{}` at {}:{}", var_name, self.peek().line, self.peek().column),
      					  line: self.peek().line,
     					   column: self.peek().column,
   				 });
					}
   				 Ok(Stmt::Assignment(Assignment{
                        target: var_name,
                        value
                       }))
			 	}else{
                    Err(ParserError{
                        error: format!("Expected compound operator or '=' after identifier in {}:{}; found '{:?}'", self.peek().line, self.peek().column, self.peek().kind),
                        kind: ParserErrorKind::InvalidVariableDecl,
                        column: self.peek().column,
                        line: self.peek().line
                        })
                }
            }
       	 _ => Err(ParserError{
                    error: format!("Expected valid Expression in {}:{}; found '{:?}'", self.peek().line, self.peek().column, self.peek().kind),
                    kind: ParserErrorKind::InvalidExpression,
                    column: self.peek().column,
                    line: self.peek().line
                 })
  	  }
	}

	fn is_type_token(&self) -> bool {
  	  matches!(
      	  self.peek().kind,
   	     TokenKind::UInt8 | TokenKind::UInt16 | TokenKind::UInt32 | TokenKind::UInt64 | 
   	     TokenKind::Int8 | TokenKind::Int16 | TokenKind::Int32 | TokenKind::Bool | TokenKind::Void | TokenKind::Int | TokenKind::Str | TokenKind::Vec
   	 )
	}
    fn synchronize(&mut self) {
    // Avança até encontrar um ponto de sincronização
  	  while self.peek().kind != TokenKind::EOF {
      	  match self.peek().kind {
            // Pontos seguros para resincronizar
          	  TokenKind::Semicolon | TokenKind::RightBrace | 
           	 TokenKind::Fun | TokenKind::Var | TokenKind::If | 
          	  TokenKind::For | TokenKind::Return => break,
           	 _ => { self.advance(); }
        	}
    	}
    // Consome o token de sincronização se não for EOF
   	 if self.peek().kind != TokenKind::EOF {
      	  self.advance();
    	}
	}


    pub fn parse_funcdecl(&mut self) -> Result<Stmt, ParserError>{
        self.expect(TokenKind::Fun)?;
        let return_type = if self.is_type_token(){
            Some(self.parse_type()?)
        }else{
            Some(Type::Void)
        };    
        let name = self.expect_ident()?;
        self.expect(TokenKind::LeftParen)?;
        let mut params = Vec::new();
        if self.peek().kind != TokenKind::RightParen{
            loop {
                let p_type = self.parse_type()?;
                let p_name = self.expect_ident()?;
                params.push((p_type, p_name));
                if self.peek().kind == TokenKind::Comma{
                    self.advance();
                }else{
                    break;
                }
            }
        }
        self.expect(TokenKind::RightParen)?;
        
        self.expect(TokenKind::LeftBrace)?;
        let mut body = Vec::new();
        while self.peek().kind != TokenKind::RightBrace && self.peek().kind != TokenKind::EOF{
            body.push(self.parse_statement()?)
        }
        self.expect(TokenKind::RightBrace)?;
        Ok(Stmt::FuncDecl(FuncDecl{
            name,
            params,
            return_type,
            body
        }))
    }    
    
    pub fn parse_valdecl(&mut self) -> Result<Stmt, ParserError> {
    	let mutability = match self.peek().kind {
     	   TokenKind::Const => {
     	       self.advance();
     	       Mutability::Const
     	   }
       	 TokenKind::Var => {
          	  self.advance();
         	   Mutability::Mutable
       	 }
       	 _ => {
         	   return Err(ParserError {
            	    kind: ParserErrorKind::InvalidVariableDecl,
          	      error: format!(
           	         "Expected 'const' or 'var' at {}:{}, found {:?}",
            	        self.peek().line,
            	        self.peek().column,
             	       self.peek().kind
             	   ),
              	  column: self.peek().column,
             	   line: self.peek().line,
          	  });
      	  }
    };

    let ty = if self.is_type_token() {
        Some(self.parse_type()?)
    } else {
        None
    };

    let name = self.expect_ident()?;
    if let Some(varia) = self.lookup_var(&name){
        return Err(ParserError{
            kind: ParserErrorKind::DuplicateVariableError,
            error: format!("duplicate variable {} found in current scope; {}:{}", name, self.peek().line, self.peek().column),
            line: self.peek().line,
            column: self.peek().column
        }
        )
    }
    self.expect(TokenKind::Equal)?;
    let value = self.parse_expr()?;

    let ty = ty.unwrap_or_else(|| infer_type_from_expr(&value));

    self.expect(TokenKind::Semicolon)?;

    let valdecl = VarDecl {
        name,
        ty,
        value,
        mutability,
    };
    self.add_var(valdecl.clone());
    Ok(Stmt::VarDecl(valdecl))
	}
	pub fn parse(&mut self) -> ParseResult {
  	  let mut stmts_parse = Vec::new();
        let mut errors_parse = Vec::new();
 	   while self.peek().kind != TokenKind::EOF {
            match self.parse_statement(){
                Ok(stmt) => stmts_parse.push(stmt),
                Err(e) => {
                    errors_parse.push(e);
                    self.synchronize();
                   }
            }
 	   }
        let success = errors_parse.is_empty();
  	  ParseResult{
            stmts: stmts_parse,
            errors: errors_parse,
            success: success
        }
        
    }
}
