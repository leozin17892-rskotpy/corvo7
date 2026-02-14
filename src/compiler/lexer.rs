use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::str::Chars;


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
     IntLiteral(u128),
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
    UnterminatedString,
}
pub struct LexerError{
    pub kind: LexerErrorKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
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
        line,
        column: start,
    }
}


fn lex_num(chars: &mut Peekable<Chars>, line: usize, column: &mut usize) -> Result<Token, LexerError>{
    let start = *column;
    let mut value = String::new();
    while let Some(&c) = chars.peek(){
        if c.is_ascii_digit(){
            value.push(c);
            chars.next();
            *column += 1;
        }else{
            break;
        }
    }
    let kind = if chars.peek() == Some(&'u'){
        chars.next();
        if matches!(chars.peek(), Some(c) if c.is_ascii_alphanumeric() || *c == '_') {
     	   return Err(LexerError{
                kind: LexerErrorKind::InvalidNumericSuffix,
                line: line,
                column: start,
            });
  	  }
        *column += 1;
        TokenKind::UIntLiteral(value.parse().unwrap())
    }else{
        TokenKind::IntLiteral(value.parse().unwrap())
    };
    Ok(Token{
        kind,
        line: line,
        column: start,
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
        "when" => TokenKind::When,
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
    Token { kind, line, column: start }
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
            line: line,
            column: start,
        });
    }
    Ok(Token {kind: TokenKind::StringLiteral(string), line, column: start})
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
            line,
            column: start});
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
            if chars.peek() == Some(&'/'){
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
                   	 line: line,
                   	 column: start,
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
                    line: line,
                    column: start,
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
                    line: line,
                    column: start,
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
            line: line,
            column: start,
        }),
    };

    Ok(Token { kind, line, column: start })
}
pub fn lexer(file_path: &str) -> Vec<Token>{
    let file = File::open(file_path).expect("error reading file");
    let reader = BufReader::new(file);
    let mut tokens: Vec<Token> = Vec::new();
    let mut last_column = 1;
    let mut last_line = 1;
    for(line_num, line) in reader.lines().enumerate(){
        let line = line.expect("Fail reading line");
        let line_no = line_num + 1;
        let mut chars = line.chars().peekable();
        let mut column = 1;
        while let Some(c) = chars.peek().copied(){
            match c{
                _ if c.is_whitespace() => {
                    chars.next();
                    column += 1;
                }
                '[' => {
                    tokens.push(lex_vec_literal(&mut chars, line_no, &mut column))
                }
                '0'..='9' => {
                    match lex_num(&mut chars, line_no, &mut column){
                        Ok(tok) => tokens.push(tok),
                        Err(err) => {
                            eprintln!("Lexer Error at {}:{} -> {:?}", err.line, err.column, err.kind);
                            chars.next();
                            column += 1;
                        },
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(lex_ident_or_keyword(&mut chars, line_no, &mut column)),
                '"' =>{
                    match lex_str(&mut chars, line_no, &mut column){
                        Ok(tok) => tokens.push(tok),
                        Err(err) => {
                            eprintln!("Lexer Error at {}:{} -> {:?}", err.line, err.column, err.kind);
                        }
                    }
                },
                _ => {
                    match lex_operator(&mut chars, line_no, &mut column){
                        Ok(tok) => tokens.push(tok),
                        Err(err) => {
                            eprintln!("Lexer Error at {}:{} -> {:?}", err.line, err.column, err.kind);
                        }
                    }
                }
            }
        }
        last_column = column; 
        last_line = line_no;
    }
    tokens.push(Token{
        kind: TokenKind::EOF,
        line: last_line,
        column: last_column,
    });
    tokens
	
}    
