use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::str::Chars;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind{
     // keywords 
     Fun,
     Var,
     Const,
     Return,
     Cmhr,
     Cmho,
     Print,
     If,
     Else,
     This,
     For,
     In,
     Range,
     Break,
     Continue,
     When,
     While,
     Loop,
     As,
     
     
     // types
     UInt8,
     UInt16,
     UInt32,
     UInt64,
     UInt128,
     Int,
     UInt,
     Int8,
     Int16,
     Int32,
     Int64,
     Int128,
     Vec,
     Bool,
     Void,
     Str,
     
     // literals
     IntLiteral(i128),
     NegIntLiteral(i128),
     UIntLiteral(u128),
     BoolLiteral(bool),
     StringLiteral(String),
     InnerType(String),
     VecLiteral(Vec<i128>),
     // ident
     Ident(String),
     
     // Operators
     Plus,
     Minus,
     Star,
     Slash,
     Percent,
     Equal,
     DoubleEqual,
     NotEqual,
     FatArrow,
     Less,
     LessEqual,
     Greater,
     Indentity,
     GreaterEqual,
     CompoundAdd,
     CompoundSub,
     CompoundMul,
     CompoundDiv,
     And,
     Or,
     Not,
     
     // Separators
     Comma,
     Semicolon,
     Colon,
     Arrow,
     Dot,
     LeftParen,
     RightParen,
     LeftBrace,
     RightBrace,
     LeftBracket,
     RightBracket,
    
     // end of file
     EOF,
 }

#[derive(Debug, Clone)] 
pub enum LexerErrorKind{
    InvalidChar(char),
    InvalidNumericSuffix,
    NegativeNumWithUSuffix,
    UnterminatedString,
}
#[derive(Debug, Clone)]
pub struct Span{
    pub line: usize,
    pub column: usize,
}
#[derive(Debug)]
pub struct LexerError{
    pub kind: LexerErrorKind,
    pub span: Span,
}
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            LexerErrorKind::InvalidChar(c) => {
                write!(f, "error: invalid character '{}'", c)
            }
            LexerErrorKind::InvalidNumericSuffix => {
                write!(f, "error: invalid numeric suffix")
            }
            LexerErrorKind::UnterminatedString => {
                write!(f, "error: unterminated string")
            }
            LexerErrorKind::NegativeNumWithUSuffix => {
                write!(f, "error: negative numeric value with 'u' suffix")
            }
        }
    }
}

#[derive(Debug)]
pub struct LexerErrors(pub Vec<LexerError>);

impl fmt::Display for LexerErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for err in &self.0 {
            writeln!(f, "{err}")?;
        }
        Ok(())
    }
}

impl std::error::Error for LexerErrors {}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
pub fn print_lexer_error(file_path: &str, err: &LexerError, lines: &Vec<String>) {
    if err.span.line == 0 || err.span.line > lines.len() {
        eprintln!("Lexer Error at invalid line: {:?}", err);
        return;
    }

    let line_text = &lines[err.span.line - 1];
    // Título do erro
    eprintln!("{}", err);
    eprintln!(" --> {}:{}:{}", file_path, err.span.line, err.span.column);

    // Linha do código
    eprintln!("{:>4} | {}", err.span.line, line_text);

    // Marcador "^" apontando para a coluna
    let col = if err.span.column > line_text.len() { line_text.len() } else { err.span.column - 1 };
    let mut marker = String::new();
    for _ in 0..col { marker.push(' '); }
    marker.push('^');
    eprintln!("     | {}", marker);
}

fn lex_vec_literal(chars: &mut Peekable<Chars>, line: usize, column: &mut usize) -> Token {
    let start = *column;
    
    chars.next(); // consome '['
    *column += 1;

    let mut values: Vec<i128> = Vec::new();
    let mut current_num = String::new();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                current_num.push(c);
                chars.next();
                *column += 1;
            }
            ',' => {
                if !current_num.is_empty() {
                    values.push(current_num.parse().unwrap());
                    current_num.clear();
                }
                chars.next();
                *column += 1;
            }
            ']' => {
                if !current_num.is_empty() {
                    values.push(current_num.parse().unwrap());
                }
                chars.next();
                *column += 1;
                break;
            }
            ' ' | '\t' => {
                chars.next();
                *column += 1;
            }
            _ => break,
        }
    }

    Token {
        kind: TokenKind::VecLiteral(values),
        span: Span{
        line,
        column: start
       }
    }
}


fn lex_num(chars: &mut Peekable<Chars>, line: usize, column: &mut usize) -> Result<Token, LexerError>{
    let start = *column;
    let mut is_negative = false;
    let mut value = String::new();
    if let Some(&'-') = chars.peek(){
        let next = chars.clone().nth(1); // peek no segundo caractere
        if let Some(c) = next {
            if c.is_ascii_digit() {
                is_negative = true;
                chars.next();           // consome o '-'
                *column += 1;
            }
        }
    }
    while let Some(&c) = chars.peek(){
        if c.is_ascii_digit(){
            value.push(c);
            chars.next();
            *column += 1;
        }else{
            break;
        }
    }
    if value.is_empty(){
        return Err(LexerError{
            kind: LexerErrorKind::InvalidChar('-'), // ou outro erro apropriado
            span: Span { line, column: start },
        })
    }
    let mut value_parsed: u128 = value.parse().map_err(|_| LexerError {
        kind: LexerErrorKind::InvalidChar('?'), // melhorar depois
        span: Span { line, column: start },
    })?;
    
    let kind = if chars.peek() == Some(&'u'){
        chars.next();
        if matches!(chars.peek(), Some(c) if c.is_ascii_alphanumeric() || *c == '_') {
     	   return Err(LexerError{
                kind: LexerErrorKind::InvalidNumericSuffix,
                span: Span{
                	line: line,
              	  column: start,
                }
            });
  	  }
        if is_negative {
            // erro: sufixo u com número negativo
            return Err(LexerError { kind: LexerErrorKind::NegativeNumWithUSuffix,  span: Span { line, column: start } });
        }
        *column += 1;
        TokenKind::UIntLiteral(value.parse().unwrap())
    }else{
        let signed_value = if is_negative { -(value_parsed as i128) } else { value_parsed as i128 };
        if is_negative{
      	  TokenKind::NegIntLiteral(signed_value)
        }else{
            TokenKind::IntLiteral(signed_value)
        }
    };
    Ok(Token{
        kind,
        span: Span { 
            line: line,
       	 column: start
        },
    })
    
}

fn lex_ident_or_keyword(chars: &mut Peekable<Chars>, line: usize, column: &mut usize) -> Token {
    let start = *column;
    let mut word = String::new();
    while let Some(&c) = chars.peek(){
        if c.is_ascii_alphanumeric() || c == '_'{
            word.push(c);
            chars.next();
            *column += 1;
        }else{
            break;
        }
    }
    let kind = match word.as_str(){
        "fun" => TokenKind::Fun,
        "const" => TokenKind::Const,
        "var" => TokenKind::Var,
        "cmho" => TokenKind::Cmho,
        "cmhr" => TokenKind::Cmhr,
        "return" => TokenKind::Return,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "for" => TokenKind::For,
        "while" => TokenKind::While,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "in" => TokenKind::In,
        "loop" => TokenKind::Loop,
        "when" => TokenKind::When,
        "as" => TokenKind::As,
        "print" => TokenKind::Print,
        "true" => TokenKind::BoolLiteral(true),
        "false" => TokenKind::BoolLiteral(false),
        "this" => TokenKind::This,
        "bool" => TokenKind::Bool,
        "int" => TokenKind::Int,
        "Vec" => TokenKind::Vec,
        "u8" => TokenKind::UInt8,
        "u16" => TokenKind::UInt16,
        "u32" => TokenKind::UInt32,
        "string" => TokenKind::Str,
        "u64" => TokenKind::UInt64,
        "u128" => TokenKind::UInt128,
        "i8" => TokenKind::Int8,
        "i16" => TokenKind::Int16,
        "i32" => TokenKind::Int32,
        "i64" => TokenKind::Int64,
        "uint" => TokenKind::UInt,
        "i128" => TokenKind::Int128,
        "void" => TokenKind::Void,
        _ => TokenKind::Ident(word),
    };
    Token { kind, span: Span{
        line, 
        column: start
        } 
    }
}
fn lex_str(chars: &mut Peekable<Chars>, line: usize, column: &mut usize) -> Result<Token, LexerError>{
    let start = *column;
    let mut string = String::new();
    let mut closed: bool = false;
    chars.next();
    *column += 1;
    while let Some(&c) = chars.peek(){
        if c == '"'{
            chars.next();
            *column += 1;
            closed = true;
            break;
        }
            string.push(c);
            chars.next();
            *column += 1;
        }
    if !closed{
        return Err(LexerError{
            kind: LexerErrorKind:: UnterminatedString,
            span: Span { 
                line: line,
          	  column: start,
            }
        });
    }
    Ok(Token{kind: TokenKind::StringLiteral(string), span: Span{
        line, 
        column: start
        }
    })
}
fn lex_operator(
    chars: &mut Peekable<Chars>,
    line: usize,
    column: &mut usize
) -> Result<Token, LexerError> {
    let start = *column;
    let c = match chars.next(){
        Some(c) => c,
        None => {
            return Err(LexerError{
            kind: LexerErrorKind::InvalidChar('\0'),
            span: Span{
                line,
                column: start
                }});
        }
    };
    *column += 1;

    let kind = match c {
        '+' => {
            if chars.peek() == Some(&'='){
                chars.next();
                *column += 1;
                TokenKind::CompoundAdd
            }else{
                TokenKind::Plus
            }
        }
        '-' => {
            if chars.peek() == Some(&'>') {
                chars.next();
                *column += 1;
                TokenKind::Arrow
            }else if chars.peek() == Some(&'='){
                chars.next();
                *column += 1;
                TokenKind::CompoundSub
            }else {
                TokenKind::Minus
            }
        }
        '*' => {
            if chars.peek() == Some(&'='){
                chars.next();
                *column += 1;
                TokenKind::CompoundMul
            }else{
                TokenKind::Star
            }
        }
        '/' => {
            if chars.peek() == Some(&'='){
                chars.next();
                *column += 1;
                TokenKind::CompoundDiv
            }else{
                TokenKind::Slash
          }
        }
        '%' => TokenKind::Percent,
        '=' => {
            if chars.peek() == Some(&'=') {
                chars.next();
                *column += 1;
                TokenKind::DoubleEqual
            }else if chars.peek() == Some(&'>'){
                chars.next();
                *column += 1;
                TokenKind::FatArrow	
            }else {
                TokenKind::Equal
            }
        }
        '.' => {
            if chars.peek() == Some(&'.'){
                chars.next();
                *column += 1;
                TokenKind::Range
            }else{
                TokenKind::Dot
            }
        }
        ':' => {
            if chars.peek() == Some(&'='){
                chars.next();
                *column += 1;
                if chars.peek() == Some(&':'){
                    chars.next();
                    *column += 1;
                    TokenKind::Indentity
                }else{
                    return Err(LexerError{
                        kind: LexerErrorKind::InvalidChar(c),
                   	 span: Span{
                        line: line,
                   	 column: start},
                    });
                }
            }else{
                TokenKind::Colon
            }
        }
        ';' => TokenKind::Semicolon,
        '|' => {
            if chars.peek() == Some(&'|'){
                chars.next();
                *column += 1;
                TokenKind::Or
            }else{
                return Err(LexerError{
                    kind: LexerErrorKind::InvalidChar(c),
                    span: Span{
                    line: line,
                    column: start,
                    }
                });
            }
        }
        '&' => {
            if chars.peek() == Some(&'&'){
                chars.next();
                *column += 1;
                TokenKind::And
            }else{
                return Err(LexerError{
                    kind: LexerErrorKind::InvalidChar(c),
                    span: Span{
                    line: line,
                    column: start
                    },
                });
            }
        }
        '!' => {
            if chars.peek() == Some(&'='){
                chars.next();
                *column += 1;
                TokenKind::NotEqual
            }else{
                TokenKind::Not
            }
        }
        ',' => TokenKind::Comma,
        '<' => {
            if chars.peek() == Some(&'='){
                chars.next();
                *column += 1;
                TokenKind::LessEqual
            }else{
                TokenKind::Less
            }
        }
        '>' => {
            if chars.peek() == Some(&'='){
                chars.next();
                *column += 1;
                TokenKind::GreaterEqual
            }else{
                TokenKind::Greater
            }
        }
        '{' => TokenKind::LeftBrace,
        '}' => TokenKind::RightBrace,
        '(' => TokenKind::LeftParen,
        ')' => TokenKind::RightParen,
        '[' => TokenKind::LeftBracket,
        ']' => TokenKind::RightBracket,
        _ => return Err(LexerError {
            kind: LexerErrorKind::InvalidChar(c),
            span: Span{line: line,
            column: start
            },
        }),
    };

    Ok(Token { kind, span: Span{line, column: start }})
}

pub fn lexer(file_path: &str) -> Result<Vec<Token>, LexerErrors> {
    let file = File::open(file_path).expect("error reading file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors: Vec<LexerError> = Vec::new();
    
    for (line_num, line) in lines.iter().enumerate() {
        let line_no = line_num + 1;
        let mut chars = line.chars().peekable();
        let mut column = 1;

        while let Some(c) = chars.peek().copied() {
            let result: Option<Result<Token, LexerError>> = match c {
                _ if c.is_whitespace() => {
                    chars.next();
                    column += 1;
                    None // Ignora whitespace
                }
                '[' => Some(Ok(lex_vec_literal(&mut chars, line_no, &mut column))),
                '0'..='9' => {
  				  Some(lex_num(&mut chars, line_no, &mut column))
				}

				'-' => {
 				   match chars.clone().nth(1) {
     				   Some('0'..='9') => Some(lex_num(&mut chars, line_no, &mut column)),
                        _ => Some(lex_operator(&mut chars, line_no, &mut column))
 				   }
				}
                'a'..='z' | 'A'..='Z' | '_' => Some(Ok(lex_ident_or_keyword(&mut chars, line_no, &mut column))),
                '"' => Some(lex_str(&mut chars, line_no, &mut column)),
                _ => Some(lex_operator(&mut chars, line_no, &mut column)),
            };

            if let Some(res) = result {
                match res {
                    Ok(tok) => tokens.push(tok),
                    Err(err) => {
                        errors.push(err);
                        // Estratégia de Sincronização:
                        // Consome o char problemático para não entrar em loop infinito
                        chars.next();
                        column += 1;
                    }
                }
            }
        }
    }

    if !errors.is_empty() {
        for err in &errors {
       	 print_lexer_error(file_path, err, &lines);
 	   }
        return Err(LexerErrors(errors));
    }

    tokens.push(Token {
        kind: TokenKind::EOF,
        span: Span{
            line: lines.len(), // Melhor usar o total de linhas real
            column: 1
        },
    });

    Ok(tokens)
}

