#[derive(Debug, PartialEq)]
enum LimiterToken {
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
}

#[derive(Debug, PartialEq)]
enum Token {
    ILLEGAL,
    EOF,
    IDENTIFIER(String),
    LITERAL(String),
    LIMITER(LimiterToken),
    ASSIGN,
    PLUS,
    FUNCTION,
    LET,
}

fn is_letter(ch: Option<char>) -> bool {
    match ch {
        None => false,
        Some(x) => { x.is_ascii_alphabetic() || x == '_' },
    }
}

fn is_digit(ch: Option<char>) -> bool {
    match ch {
        None => false,
        Some(x) => { x.is_numeric() },
    }
}

fn is_whitespace(ch: Option<char>) -> bool {
    match ch {
        None => false,
        Some(x) => { x == ' ' || x == '\t' || x == '\n' || x == '\r' },
    }
}

fn look_up_identifier(ident: &str) -> Token {
    match ident {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        _ => Token::IDENTIFIER(ident.to_string()),
    }
}

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

fn build_lexer(input: String) -> Lexer {
    let mut l = Lexer { input, position: 0, read_position: 0, ch: None };
    l.read_char();
    l
}

impl Lexer {
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            // FIXME: it's probably quite bad to convert the whole string every time
            self.ch = Some(char::from(self.input.as_bytes()[self.read_position]));
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> &str {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        &self.input[pos..self.position]
    }

    fn read_number(&mut self) -> &str {
        let pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        &self.input[pos..self.position]
    }

    fn skip_white_spaces(&mut self) {
        while is_whitespace(self.ch) { self.read_char(); }
    }

    fn next_token(&mut self) -> Token {
        self.skip_white_spaces();
        let t: Token = match self.ch {
            None => Token::EOF,
            Some(x) => match x {
                    ',' => Token::LIMITER(LimiterToken::COMMA),
                    ';' => Token::LIMITER(LimiterToken::SEMICOLON),
                    '(' => Token::LIMITER(LimiterToken::LPAREN),
                    ')' => Token::LIMITER(LimiterToken::RPAREN),
                    '{' => Token::LIMITER(LimiterToken::LBRACE),
                    '}' => Token::LIMITER(LimiterToken::RBRACE),
                    '+' => Token::PLUS,
                    '=' => Token::ASSIGN,
                    _ => {
                        // early return because we can't run `self.read_char` at the end, we
                        // already incremented during `read_identifier`
                        if is_letter(Some(x)) {
                            return look_up_identifier(self.read_identifier());
                        } else if is_digit(Some(x)) {
                            return Token::LITERAL(self.read_number().to_string());
                        } else {
                            Token::ILLEGAL
                        }
                    },
            }
        };
        self.read_char();
        t
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
    ";
    let input = String::from(program);
    let mut lex = build_lexer(input);
    
    let tests: [Token; 37]  = [
        Token::LET,
        Token::IDENTIFIER(String::from("five")),
        Token::ASSIGN,
        Token::LITERAL(String::from("5")),
        Token::LIMITER(LimiterToken::SEMICOLON),

        Token::LET,
        Token::IDENTIFIER(String::from("ten")),
        Token::ASSIGN,
        Token::LITERAL(String::from("10")),
        Token::LIMITER(LimiterToken::SEMICOLON),

        Token::LET,
        Token::IDENTIFIER(String::from("add")),
        Token::ASSIGN,
        Token::FUNCTION,
        Token::LIMITER(LimiterToken::LPAREN),
        Token::IDENTIFIER(String::from("x")),
        Token::LIMITER(LimiterToken::COMMA),
        Token::IDENTIFIER(String::from("y")),
        Token::LIMITER(LimiterToken::RPAREN),
        Token::LIMITER(LimiterToken::LBRACE),
        Token::IDENTIFIER(String::from("x")),
        Token::PLUS,
        Token::IDENTIFIER(String::from("y")),
        Token::LIMITER(LimiterToken::SEMICOLON),
        Token::LIMITER(LimiterToken::RBRACE),
        Token::LIMITER(LimiterToken::SEMICOLON),

        Token::LET,
        Token::IDENTIFIER(String::from("result")),
        Token::ASSIGN,
        Token::IDENTIFIER(String::from("add")),
        Token::LIMITER(LimiterToken::LPAREN),
        Token::IDENTIFIER(String::from("five")),
        Token::LIMITER(LimiterToken::COMMA),
        Token::IDENTIFIER(String::from("ten")),
        Token::LIMITER(LimiterToken::RPAREN),
        Token::LIMITER(LimiterToken::SEMICOLON),

        Token::EOF,

    ];

    for test_token in tests.iter() {
        let token: Token = lex.next_token();
        assert_eq!(token, *test_token);
    };
}
