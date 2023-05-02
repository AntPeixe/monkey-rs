#[derive(Debug, PartialEq)]
pub enum LimiterToken {
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    Identifier(String),
    Literal(String),
    Limiter(LimiterToken),
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
    EQ,
    NotEq,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Token {
    fn len(&self) -> usize {
     match self {
            Token::Illegal | Token::Eof => 0,
            Token::Identifier(s) | Token::Literal(s) => s.len(),
            Token::Limiter(_)
            | Token::Assign
            | Token::Plus
            | Token::Minus
            | Token::Bang
            | Token::Asterisk
            | Token::Slash
            | Token::LT
            | Token::GT => 1,
            Token::Function | Token::If | Token::EQ | Token::NotEq => 2,
            Token::Let => 3,
            Token::True | Token::Else => 4,
            Token::False => 5,
            Token::Return => 6,
        }
    }
}

fn is_letter(ch: char) -> bool {
    // allowing `_` for identifiers
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_numeric()
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

fn look_up_identifier(ident: String) -> Token {
    return match ident.as_str() {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Identifier(ident),
    };
}

pub struct Lexer {
    input: String,
    read_position: usize,
    ch: Option<char>,
}

impl From<String> for Lexer {
    fn from(string: String) -> Self {
        let mut l = Lexer {
            input: string,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_white_spaces();
        let token: Token = match self.ch {
            None => Token::Eof,
            Some(x) => match x {
                ',' => Token::Limiter(LimiterToken::Comma),
                ';' => Token::Limiter(LimiterToken::Semicolon),
                '(' => Token::Limiter(LimiterToken::LParen),
                ')' => Token::Limiter(LimiterToken::RParen),
                '{' => Token::Limiter(LimiterToken::LBrace),
                '}' => Token::Limiter(LimiterToken::RBrace),
                '+' => Token::Plus,
                '-' => Token::Minus,
                '=' => {
                    if let Some('=') = self.peek_char_head() {
                        Token::EQ
                    } else {
                        Token::Assign
                    }
                }
                '!' => {
                    if let Some('=') = self.peek_char_head() {
                        Token::NotEq
                    } else {
                        Token::Bang
                    }
                }
                '*' => Token::Asterisk,
                '/' => Token::Slash,
                '<' => Token::LT,
                '>' => Token::GT,
                _ => {
                    if is_letter(x) {
                        look_up_identifier(self.read_identifier())
                    } else if is_digit(x) {
                        Token::Literal(self.read_number())
                    } else {
                        Token::Illegal
                    }
                }
            },
        };
        if token == Token::Eof { return None; }
        self.increment_read_position(&token);
        self.read_char();
        Some(token)
    }
}

impl Lexer {
    fn read_char(&mut self) {
        // FIXME: it's probably bad to create the iterator everytime
        // However having `input` as the iterator means that when reading `take_while` requires a
        // clone everytime.
        self.ch = self.input.chars().nth(self.read_position);
    }

    fn peek_char_head(&self) -> Option<char> {
        return self.input.chars().nth(self.read_position + 1);
    }

    fn increment_read_position(&mut self, token: &Token) {
        self.read_position += token.len();
    }

    fn read_identifier(&self) -> String {
        return self
            .input
            .chars()
            .skip(self.read_position)
            .take_while(|ch| is_letter(*ch))
            .collect::<String>();
    }

    fn read_number(&self) -> String {
        return self
            .input
            .chars()
            .skip(self.read_position)
            .take_while(|ch| is_digit(*ch))
            .collect::<String>();
    }

    fn skip_white_spaces(&mut self) {
        let spaces = self
            .input
            .chars()
            .skip(self.read_position)
            .take_while(|c| is_whitespace(*c))
            .count();

        // spaces don't create a token so we much increment and re-read the next char
        if spaces > 0 {
            self.read_position += spaces;
            self.read_char();
        }
    }
}

#[test]
fn lexer_test() {
    let program = "let five = 5;
    let ten = 10;
    let add = fn(x, y) {
        x + y;
    };
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    if (5 < 10) {
        return true;
    } else {
        return false;
    }

    10 == 10;
    10 != 9;
    ";
    let input = String::from(program);
    let lex = Lexer::from(input);

    let tests: [Token; 73] = [
        Token::Let,
        Token::Identifier(String::from("five")),
        Token::Assign,
        Token::Literal(String::from("5")),
        Token::Limiter(LimiterToken::Semicolon),
        Token::Let,
        Token::Identifier(String::from("ten")),
        Token::Assign,
        Token::Literal(String::from("10")),
        Token::Limiter(LimiterToken::Semicolon),
        Token::Let,
        Token::Identifier(String::from("add")),
        Token::Assign,
        Token::Function,
        Token::Limiter(LimiterToken::LParen),
        Token::Identifier(String::from("x")),
        Token::Limiter(LimiterToken::Comma),
        Token::Identifier(String::from("y")),
        Token::Limiter(LimiterToken::RParen),
        Token::Limiter(LimiterToken::LBrace),
        Token::Identifier(String::from("x")),
        Token::Plus,
        Token::Identifier(String::from("y")),
        Token::Limiter(LimiterToken::Semicolon),
        Token::Limiter(LimiterToken::RBrace),
        Token::Limiter(LimiterToken::Semicolon),
        Token::Let,
        Token::Identifier(String::from("result")),
        Token::Assign,
        Token::Identifier(String::from("add")),
        Token::Limiter(LimiterToken::LParen),
        Token::Identifier(String::from("five")),
        Token::Limiter(LimiterToken::Comma),
        Token::Identifier(String::from("ten")),
        Token::Limiter(LimiterToken::RParen),
        Token::Limiter(LimiterToken::Semicolon),
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Literal(String::from("5")),
        Token::Limiter(LimiterToken::Semicolon),
        Token::Literal(String::from("5")),
        Token::LT,
        Token::Literal(String::from("10")),
        Token::GT,
        Token::Literal(String::from("5")),
        Token::Limiter(LimiterToken::Semicolon),
        Token::If,
        Token::Limiter(LimiterToken::LParen),
        Token::Literal(String::from("5")),
        Token::LT,
        Token::Literal(String::from("10")),
        Token::Limiter(LimiterToken::RParen),
        Token::Limiter(LimiterToken::LBrace),
        Token::Return,
        Token::True,
        Token::Limiter(LimiterToken::Semicolon),
        Token::Limiter(LimiterToken::RBrace),
        Token::Else,
        Token::Limiter(LimiterToken::LBrace),
        Token::Return,
        Token::False,
        Token::Limiter(LimiterToken::Semicolon),
        Token::Limiter(LimiterToken::RBrace),
        Token::Literal(String::from("10")),
        Token::EQ,
        Token::Literal(String::from("10")),
        Token::Limiter(LimiterToken::Semicolon),
        Token::Literal(String::from("10")),
        Token::NotEq,
        Token::Literal(String::from("9")),
        Token::Limiter(LimiterToken::Semicolon),
    ];

    lex.into_iter()
        .zip(tests.into_iter())
        .map(|(token, test_token)| {
            assert_eq!(token, test_token);
        })
        .for_each(drop);
}
