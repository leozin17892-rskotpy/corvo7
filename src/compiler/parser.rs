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
    WhenStatement(WhenStatement),
    ForLoop(ForLoop),
    Loop(Loop),
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
pub struct TypedExpr{
    pub expr: Box<Expr>,
    pub ty: Type,
}
#[derive(Debug, Clone)]
pub struct TypedStmt{
    pub kind: TypedStmtKind
}
#[derive(Debug, Clone)]
pub enum TypedStmtKind {
    VarDecl { name: String, ty: Type, value: TypedExpr, mutability: Mutability },
    Assignment { target: String, value: TypedExpr },
    If { condition: TypedExpr, then_branch: Vec<TypedStmt>, else_branch: Option<Vec<TypedStmt>> },
    Return(TypedExpr),
    Print(Vec<TypedExpr>),
    When { target: TypedExpr, arms: Vec<TypedWhenArm> },
    // ... outros stmts
}
#[derive(Debug, Clone)]
pub struct TypedWhenExpr {
    pub condition: TypedExpr,
    pub arms: Vec<TypedWhenArm>, // Cada um com (valor_case, valor_retorno)
    pub else_arm: Option<TypedExpr>,
}


#[derive(Debug, Clone)]
pub struct TypedWhenArm{
    pub value: TypedExpr,
    pub result: TypedExpr,
}

#[derive(Debug, Clone)]
pub enum ParserErrorKind{
    ExpectedSemiColonKind,
    ExpectedTokenNotFound,
    DuplicateElseStatement,
    ExpectedIdentifierNotFound,
    TypeNotFound,
    UnknownStmt,
    InvalidExpression,
    UnexpectedEOF,
    InvalidVariableDecl,
    DuplicateVariableError,
    UndefinedVariable,
    MissingElseArm
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
pub struct WhenStatement {
    pub condition: Expr,
    pub arms: Vec<(Expr, Vec<Stmt>)>,
    pub else_arm: Option<Vec<Stmt>>
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
pub struct Loop{
    pub body: Vec<Stmt>
}
#[derive(Debug, Clone)]
pub struct ForLoop{
    pub var: String,
    pub start: Expr,
    pub step: Option<Expr>,
    pub end: Expr,
    pub body: Vec<Stmt>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    },
    Unknown
}
impl Type {
    pub fn is_int(&self) -> bool {
        matches!(self, Type::Int | Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 | Type::UInt | Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128)
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
    U128Literal(u128),
    I128Literal(i128),
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
    WhenExpr(Box<WhenExpr>),
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>
    },
    Unknown
}
#[derive(Debug, Clone)]
pub struct WhenExpr {
    pub condition: Box<Expr>,
    pub arms: Vec<(Expr, Expr)>,      // cada arm é uma expressão, não um bloco
    pub else_arm: Option<Box<Expr>>
}

pub fn infer_type_from_expr(expr: &Expr) -> Type {
    match expr {
  	  Expr::UIntLiteral(_) => {
      	  Type::UInt
    	}
        Expr::IntLiteral(_) => {
    	    Type::Int
        }
        Expr::I128Literal(_) => {
            Type::I128
        }
        Expr::U128Literal(_) => {
            Type::U128
        }
        Expr::BoolLiteral(v) => {
            if *v == true || *v == false{
                Type::Bool
            }else{
                panic!("Bool type expected but found: {:?}", v)
            }
        }
        Expr::BinaryOp { op, .. } => {
            match op{
                BinOp::Add | BinOp::Sub | BinOp::Div | BinOp::Mul | BinOp::CompoundAdd | BinOp::CompoundDiv | BinOp::CompoundMul | BinOp::CompoundSub => Type::Int,
                BinOp::DoubleEqual | BinOp::Greater | BinOp::GreaterEqual | BinOp::IndentityOp | BinOp::Less | BinOp::LessEqual => Type::Bool,
                _ => panic!("Cant Infer BinOp {:?}", op)
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
    pub fn previous(&self) -> &Token{
        if self.pos == 0 {
      	  &self.tokens[0]
  	  } else {
       	 &self.tokens[self.pos - 1]
  	  }
    }
    fn add_var(&mut self, var: VarDecl) {
        self.variables.insert(var.name.clone(), var);
    }
    fn lookup_var(&self, name: &str) -> Option<&VarDecl> {
        self.variables.get(name)
    }
    fn error<T>(&self, kind: ParserErrorKind, message: String) -> Result<T, ParserError>{
        return Err(ParserError{
            kind,
            error: message,
            column: self.peek().span.column,
            line: self.peek().span.line
        })
    }
    fn error_span<T>(&self, message: String, token: Option<&Token>) -> Result<T, ParserError> {
   	 let (line, column) = if let Some(tok) = token {
       	 (tok.span.line, tok.span.column)
	    } else {
       	 (self.peek().span.line, self.peek().span.column)
	    };

 	   Err(ParserError {
     	   kind: ParserErrorKind::InvalidExpression,
     	   error: message,
     	   line,
     	   column,
   	 })
	}
    fn advance(&mut self) -> &Token{
        let tok = &self.tokens[self.pos];
        if self.pos != self.tokens.len() - 1{
            self.pos += 1;
        }
        tok
    }
    fn parse_assignment_or_compound(&mut self) -> Result<Stmt, ParserError> {
    let name = self.expect_ident()?;

    match self.peek().kind {
        TokenKind::Equal => {
            self.advance();
            let value = self.parse_expr()?;
            self.expect(TokenKind::Semicolon)?;
            Ok(Stmt::Assignment(Assignment {
                target: name,
                value,
            }))
        }

        TokenKind::CompoundAdd
        | TokenKind::CompoundSub
        | TokenKind::CompoundMul
        | TokenKind::CompoundDiv => {

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
                target: name,
                op,
                value,
            })
        }

        _ => {
            self.error(
                ParserErrorKind::InvalidExpression,
                format!("Unexpected token after identifier")
            )
    	}
      }
	}
    

    fn parse_print(&mut self) -> Result<Stmt, ParserError>{
        self.expect(TokenKind::Print)?;
        self.expect(TokenKind::LeftParen)?;
        let mut internal = Vec::new();
        while self.peek().kind != TokenKind::RightParen && self.peek().kind != TokenKind::EOF{
            if self.peek().kind == TokenKind::Comma{
                self.advance();
            }else{
           	 internal.push(self.parse_expr()?)
            }
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
        TokenKind::When => {
            return self.parse_when_expr();
        }
        TokenKind::UIntLiteral(v) => {
            self.advance();
            if v > u64::MAX as u128{
                return Ok(Expr::U128Literal(v as u128));
            }
            Ok(Expr::UIntLiteral(v as u64))
        }
        TokenKind::NegIntLiteral(v) => {
            self.advance();
            if v >= i64::MIN as i128 && v <= i64::MAX as i128{
                return Ok(Expr::IntLiteral(v as i64));
            }
            Ok(Expr::I128Literal(v))
        }
        TokenKind::IntLiteral(v) => {
            self.advance();
            if v >= i64::MIN as i128 && v <= i64::MAX as i128{
                return Ok(Expr::IntLiteral(v as i64));
            }
            Ok(Expr::I128Literal(v))
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
            self.peek().span.line,
            self.peek().span.column
      	  ),
            column: self.peek().span.column,
            line: self.peek().span.line
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
            _ => self.error_span(format!("Expected identifier in {}:{} found {:?}", self.peek().span.line, self.peek().span.column, self.peek().kind), Some(self.peek()))
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
                self.peek().span.line,
                self.peek().span.column
                ),
                line: self.peek().span.line,
                column: self.peek().span.column
                }
            )
        }
    } 
    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParserError> {
  	  let mut stmts = Vec::new();

    // ESSA É A CONDIÇÃO CORRETA E MAIS SEGURA
   	 while self.peek().kind != TokenKind::RightBrace && self.peek().kind != TokenKind::EOF {
       	 stmts.push(self.parse_statement()?);
  	  }

   	 // Se chegou aqui e não é RightBrace, é EOF → erro
  	  if self.peek().kind == TokenKind::EOF {
        	return Err(ParserError {
          	  kind: ParserErrorKind::UnexpectedEOF,
         	   error: "Unexpected end of file inside block".to_string(),
         	   line: self.peek().span.line,
           	 column: self.peek().span.column,
      	  });
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
    fn parse_loop(&mut self) -> Result<Stmt, ParserError>{
        self.expect(TokenKind::Loop)?;
        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::Loop(Loop{
            body
        }))
    }
    fn parse_for(&mut self) -> Result<Stmt, ParserError> {
    self.expect(TokenKind::For)?;
    let var = self.expect_ident()?;
    self.expect(TokenKind::In)?;

    let start = self.parse_expr()?;

    self.expect(TokenKind::Range)?;           // primeiro ".."

    // Aqui é o segredo: usamos parse_primary() para não consumir o segundo ".."
    let middle = self.parse_primary()?;

    let (step, end) = if self.peek().kind == TokenKind::Range {
        self.advance();                       // consome o segundo ".."
        let end = self.parse_expr()?;
        (Some(middle), end)
    } else {
        (None, middle)
    };

    self.expect(TokenKind::LeftBrace)?;
    let body = self.parse_block()?;

    Ok(Stmt::ForLoop(ForLoop {
        var,
        start,
        step,
        end,
        body,
    }))
}
    
    fn parse_type(&mut self) -> Result<Type, ParserError>{
        match &self.peek().kind{
            TokenKind::Str => {
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
                return Err(ParserError { kind: ParserErrorKind::TypeNotFound, column: self.peek().span.column, line: self.peek().span.line, error: format!("Expected Type, Found {:?}", self.peek().kind) });
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

    fn parse_when_expr(&mut self) -> Result<Expr, ParserError> {
   	 self.expect(TokenKind::When)?;
	    let condition = self.parse_expr()?;
    	self.expect(TokenKind::LeftBrace)?;
  	  let mut arms = Vec::new();
   	 let mut else_arm = None;

 	   while self.peek().kind != TokenKind::RightBrace && self.peek().kind != TokenKind::EOF {
      	  if self.peek().kind == TokenKind::Else {
        	    self.advance();
          	  self.expect(TokenKind::FatArrow)?;
           	 let expr = self.parse_expr()?;
           	 else_arm = Some(Box::new(expr));
          	  // consome a vírgula se tiver
           	 if self.peek().kind == TokenKind::Comma {
              	  self.advance();
          	  }
          	  break;
      	  } else {
          	  let value = self.parse_expr()?;
           	 self.expect(TokenKind::FatArrow)?;
           	 let expr = self.parse_expr()?;
           	 arms.push((value, expr));
            // vírgula entre arms
            	if self.peek().kind == TokenKind::Comma {
             	   self.advance();
           	 }
       	 }
    	}

  	  self.expect(TokenKind::RightBrace)?;

  	  if else_arm.is_none() {
      	  return self.error::<Expr>(
            	ParserErrorKind::MissingElseArm,
          	  "when expression requires an else arm".to_string()
     	   );
 	   }

    	Ok(Expr::WhenExpr(Box::new(WhenExpr {
  		  condition: Box::new(condition),
  		  arms,
   		 else_arm,
		})))
	}
    fn parse_when(&mut self) -> Result<Stmt, ParserError> {
        self.expect(TokenKind::When)?;
        let condition = self.parse_expr()?;
        self.expect(TokenKind::LeftBrace)?;
        let mut arms = Vec::new();
        let mut else_arm = None;
        while self.peek().kind != TokenKind::RightBrace && self.peek().kind != TokenKind::EOF {
            if self.peek().kind == TokenKind::Else {
                self.advance();
                self.expect(TokenKind::FatArrow)?;
                self.expect(TokenKind::LeftBrace)?;
                let block = self.parse_block()?;
                if else_arm.is_some() {
                    return self.error::<Stmt>(ParserErrorKind::DuplicateElseStatement, format!("Multiple else arms in when"));
                }
                else_arm = Some(block);
                break;
            } else {
                let value = self.parse_expr()?;
                self.expect(TokenKind::FatArrow)?;
                self.expect(TokenKind::LeftBrace)?;
                let block = self.parse_block()?;
                arms.push((value, block));
            }
        }
        self.expect(TokenKind::RightBrace)?;
        Ok(Stmt::WhenStatement(WhenStatement {
            condition,
            arms,
            else_arm,
        }))
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
    fn parse_params(&mut self) -> Result<Vec<(Type, String)>, ParserError> {
  	  let mut params = Vec::new();

    // Consome o '(' inicial (já deve estar posicionado antes de chamar, ou chame dentro de parse_func_decl)
    	self.expect(TokenKind::LeftParen)?;

    // Se for vazio: fun foo() { ... }
    	if self.peek().kind == TokenKind::RightParen {
      	  self.advance(); // consome ')'
       	 return Ok(params); // lista vazia
   	 }

   	 loop {
        // Parseia o tipo (obrigatório no teu lang?)
       	 let ty = self.parse_type()?;

        // Nome do parâmetro (identificador)
        	let name = self.expect_ident()?;

        // Adiciona na lista
       	 params.push((ty, name));

        // Checa se tem mais (vírgula) ou fecha
        	if self.peek().kind == TokenKind::Comma {
           	 self.advance(); // consome ','
            // Continua pro próximo param
       	 } else {
            // Se não for vírgula, espera ')'
         	   break;
      	  }
  	  }

    // Fecha os parênteses
  	  self.expect(TokenKind::RightParen)?;

  	  Ok(params)
	}
    fn parse_funcdecl(&mut self) -> Result<Stmt, ParserError>{
        self.expect(TokenKind::Fun)?;
        let return_type = self.parse_type().unwrap_or(Type::Void);
        let name = self.expect_ident()?;
        let params = self.parse_params()?;
        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::FuncDecl(FuncDecl{
            name,
            params,
            return_type: Some(return_type),
            body
        }))
    }
    fn parse_var_decl(&mut self) -> Result<Stmt, ParserError> {
  	  let mutability = if self.peek().kind == TokenKind::Const {
    	    self.advance();
      	  Mutability::Const
   	 } else {
      	  self.expect(TokenKind::Var)?;
      	  Mutability::Mutable
  	  };
        let mut ty = match self.parse_type(){
            Ok(v) => Some(v),
            Err(_) => None
        };
     	   
  	  let name = self.expect_ident()?; 

  	  self.expect(TokenKind::Equal)?;  // assume = obrigatório

  	  let value = self.parse_expr()?;
        if ty.is_none(){
            ty = Some(infer_type_from_expr(&value));
        }

  	  self.expect(TokenKind::Semicolon)?;

  	  let var_decl = VarDecl {
     	   name: name.clone(),
     	   ty: ty.unwrap(),
     	   value,
    	    mutability,
  	  };

    // Adiciona na scope local (pra checar duplicatas depois ou no semantic)
   	 if self.variables.contains_key(&name) {
     	   return self.error::<Stmt>(
          	  ParserErrorKind::DuplicateVariableError,
          	  format!("Variable '{}' was already declared", name),
     	   );
	    }
  	  self.add_var(var_decl.clone());

   	 Ok(Stmt::VarDecl(var_decl))
	}
    
    fn parse_statement(&mut self) -> Result<Stmt, ParserError>{
        let token = self.peek().clone();
        match token.kind {
     	   TokenKind::Fun => Ok(self.parse_funcdecl()?),
     	   TokenKind::Var | TokenKind::Const => Ok(self.parse_var_decl()?),
            TokenKind::If => self.parse_if(),
            TokenKind::For => self.parse_for(),
            TokenKind::When => self.parse_when(),
            TokenKind::Loop => self.parse_loop(),
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
                self.parse_assignment_or_compound()
 		   }
            _ => {
                self.advance();
                Err(ParserError{kind: ParserErrorKind::UnknownStmt, column: token.span.column, line: token.span.line, error: format!("Stmt {:?} was not found in {}:{}", token.kind, token.span.line, token.span.column)})
            },
        }    
    }
    fn synchronize(&mut self) {
        self.advance();
  	  while self.peek().kind != TokenKind::EOF {
  		  if self.previous().kind == TokenKind::Semicolon {
      		  return;
  		  }

   		 match self.peek().kind {
     		   TokenKind::Fun
   		     | TokenKind::Var
      		  | TokenKind::If
     		   | TokenKind::For
     		   | TokenKind::While
      		  | TokenKind::Return
                | TokenKind::RightBrace => return,
     		   _ => {}
		    }

   	 self.advance();
		}
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