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

fn is_letter(ch: char) -> bool {
    // allowing `_` for identifiers
    return ch.is_ascii_alphabetic() || ch == '_';
}

fn is_digit(ch: char) -> bool {
    return ch.is_numeric();
}

fn is_whitespace(ch: char) -> bool {
    return ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r';
}

fn look_up_identifier(ident: String) -> Token {
    return match ident.as_str() {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        _ => Token::IDENTIFIER(ident),
    };
}

struct Lexer {
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
        return l;
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_white_spaces();
        let token: Token = match self.ch {
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
                    if is_letter(x) {
                        look_up_identifier(self.read_identifier())
                    } else if is_digit(x) {
                        Token::LITERAL(self.read_number().to_string())
                    } else {
                        Token::ILLEGAL
                    }
                }
            },
        };
        self.increment_read_position(&token);
        self.read_char();
        return Some(token);
    }
}

impl Lexer {
    fn read_char(&mut self) {
        // FIXME: it's probably bad to create the iterator everytime
        // However having `input` as the iterator means that when reading `take_while` requires a
        // clone everytime.
        self.ch = self.input.chars().nth(self.read_position);
    }

    fn increment_read_position(&mut self, token: &Token) {
        match token {
            Token::ILLEGAL | Token::EOF => (),
            Token::IDENTIFIER(s) | Token::LITERAL(s) => self.read_position += s.len(),
            Token::LIMITER(_) | Token::ASSIGN | Token::PLUS => self.read_position += 1,
            Token::FUNCTION => self.read_position += 2,
            Token::LET => self.read_position += 3,
        }
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
    ";
    let input = String::from(program);
    let lex = Lexer::from(input);

    let tests: [Token; 37] = [
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

    lex.into_iter()
        .zip(tests.iter())
        .map(|(token, test_token)| {
            assert_eq!(token, *test_token);
        })
        .for_each(drop);
}
