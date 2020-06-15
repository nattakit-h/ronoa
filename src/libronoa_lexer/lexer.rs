use crate::token;

pub struct Lexer {
    result: Vec<token::Token>,
    source: String,
    current: usize,
    col: usize,
    row: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        return Lexer {
            result: Vec::default(),
            source: String::default(),
            current: 0,
            col: 1,
            row: 1,
        };
    }

    pub fn tokens_from(&mut self, source: String) -> &Vec<token::Token> {
        self.source = source;
        self.current = 0;
        self.col = 1;
        self.row = 1;
        self.process();
        return &self.result;
    }

    fn process(&mut self) {
        while !self.end() {
            match self.peek() {
                '\n' | '\r' => self.consume_newline(),
                '(' => self.consume_token("(", token::TokenKind::LeftParen),
                ')' => self.consume_token(")", token::TokenKind::RightParen),
                '{' => self.consume_token("{", token::TokenKind::LeftBrace),
                '}' => self.consume_token("}", token::TokenKind::RightBrace),
                '&' => self.consume_token("&", token::TokenKind::Ampersand),
                ':' => self.consume_token(":", token::TokenKind::Colon),
                ';' => self.consume_token(";", token::TokenKind::SemiColon),
                '.' => self.consume_token(".", token::TokenKind::Dot),
                ',' => self.consume_token(",", token::TokenKind::Comma),
                '=' => match self.peek_next() {
                    '=' => self.consume_token("==", token::TokenKind::EqualEqual),
                    _ => self.consume_token("=", token::TokenKind::Equal),
                },
                '!' => match self.peek_next() {
                    '=' => self.consume_token("!=", token::TokenKind::BangEqual),
                    _ => self.consume_token("!", token::TokenKind::Bang),
                },
                '+' => match self.peek_next() {
                    '=' => self.consume_token("+=", token::TokenKind::PlusEqual),
                    _ => self.consume_token("+", token::TokenKind::Plus),
                },

                '-' => match self.peek_next() {
                    '>' => self.consume_token("->", token::TokenKind::Arrow),
                    '=' => self.consume_token("-=", token::TokenKind::MinusEqual),
                    _ => self.consume_token("-", token::TokenKind::Minus),
                },
                '*' => match self.peek_next() {
                    '=' => self.consume_token("*=", token::TokenKind::StarEqual),
                    _ => self.consume_token("*", token::TokenKind::Star),
                },
                '/' => match self.peek_next() {
                    '/' => self.consume_line(),
                    '=' => self.consume_token("/=", token::TokenKind::SlashEqual),
                    _ => self.consume_token("/", token::TokenKind::Slash),
                },
                '>' => match self.peek_next() {
                    '=' => self.consume_token(">=", token::TokenKind::GreaterEqual),
                    _ => self.consume_token(">", token::TokenKind::Greater),
                },
                '<' => match self.peek_next() {
                    '=' => self.consume_token("<=", token::TokenKind::LesserEqual),
                    _ => self.consume_token("<", token::TokenKind::Lesser),
                },
                '"' => self.consume_string(),
                'c' => match self.peek_word().as_ref() {
                    "const" => self.consume_token("const", token::TokenKind::Const),
                    _ => {
                        self.consume_token(self.peek_word().as_ref(), token::TokenKind::Identifier);
                    }
                },
                'e' => match self.peek_word().as_ref() {
                    "else" => self.consume_token("if", token::TokenKind::Else),
                    _ => {
                        self.consume_token(self.peek_word().as_ref(), token::TokenKind::Identifier);
                    }
                },
                'f' => match self.peek_word().as_ref() {
                    "for" => self.consume_token("for", token::TokenKind::For),
                    "fun" => self.consume_token("fun", token::TokenKind::Fun),
                    _ => {
                        self.consume_token(self.peek_word().as_ref(), token::TokenKind::Identifier);
                    }
                },
                'i' => match self.peek_word().as_ref() {
                    "if" => self.consume_token("if", token::TokenKind::If),
                    _ => {
                        self.consume_token(self.peek_word().as_ref(), token::TokenKind::Identifier);
                    }
                },
                'r' => match self.peek_word().as_ref() {
                    "ref" => self.consume_token("ref", token::TokenKind::Ref),
                    "return" => self.consume_token("return", token::TokenKind::Return),
                    _ => {
                        self.consume_token(self.peek_word().as_ref(), token::TokenKind::Identifier);
                    }
                },
                'm' => match self.peek_word().as_ref() {
                    "mutate" => self.consume_token("muate", token::TokenKind::Mutate),
                    _ => {
                        self.consume_token(self.peek_word().as_ref(), token::TokenKind::Identifier);
                    }
                },
                'p' => match self.peek_word().as_ref() {
                    "public" => self.consume_token("public", token::TokenKind::Public),
                    _ => {
                        self.consume_token(self.peek_word().as_ref(), token::TokenKind::Identifier);
                    }
                },
                'l' => match self.peek_word().as_ref() {
                    "let" => self.consume_token("let", token::TokenKind::Let),
                    _ => {
                        self.consume_token(self.peek_word().as_ref(), token::TokenKind::Identifier);
                    }
                },
                'v' => match self.peek_word().as_ref() {
                    "var" => self.consume_token("var", token::TokenKind::Var),
                    _ => {
                        self.consume_token(self.peek_word().as_ref(), token::TokenKind::Identifier);
                    }
                },
                '0'..='9' => {
                    self.consume_number();
                }
                ' ' => self.advance(1),
                _ => self.consume_token(self.peek_word().as_ref(), token::TokenKind::Identifier),
            }
        }
    }

    fn end(&self) -> bool {
        return self.current == self.source.chars().count();
    }

    fn peek(&self) -> char {
        return self
            .source
            .chars()
            .nth(self.current)
            .expect("internal lexer error");
    }

    fn advance(&mut self, position: usize) {
        self.current += position;
        self.col += position;
    }

    fn peek_next(&self) -> char {
        return self
            .source
            .chars()
            .nth(self.current + 1)
            .expect("internal lexer error");
    }

    fn peek_at(&self, index: usize) -> char {
        return self
            .source
            .chars()
            .nth(index)
            .expect("internal lexer error");
    }

    fn peek_word(&self) -> String {
        let mut word: String = String::new();
        let mut index = self.current;
        while !self.end()
            && !(self.peek_at(index).is_whitespace())
            && (!self.peek_at(index).is_ascii_punctuation() || self.peek_at(index) == '_')
        {
            word.push(self.peek_at(index));
            index += 1;
        }
        return word;
    }

    fn is_newline(&self, position: usize) -> bool {
        match self.peek_at(position) {
            '\n' | '\r'  => return true,
            _ => return false,
        }
    }

    fn make_token(&mut self, kind: token::TokenKind, data: String) {
        self.result.push(token::Token {
            kind: kind,
            data: data,
            col: self.col,
            row: self.row,
        });
    }

    fn consume_newline(&mut self) {
        if (self.peek() == '\n' && self.peek() == '\r') ||
            (self.peek() == '\r' && self.peek() == '\n')
        {
            self.current += 2;
        } else {
            self.current += 1;
        }
        self.row += 1;
        self.col = 1;
        self.make_token(token::TokenKind::Newline, String::from(""));
    }

    fn consume_line(&mut self) {
        while !self.end() {
            if self.is_newline(self.current) {
                self.consume_newline();
                break;
            } else {
                self.advance(1);
            }
        }
    }

    fn consume_token(&mut self, word: &str, kind: token::TokenKind) {
        self.advance(String::from(word).chars().count());
        self.make_token(kind, String::from(word));
    }

    fn consume_string(&mut self) {
        self.advance(1);
        let mut result = String::new();
        while self.peek() != '"' {
            if self.is_newline(self.current) || self.end() {
                panic!("unterminated string");
            }
            result.push(self.peek());
            self.advance(1);
        }
        self.advance(1);
        self.make_token(token::TokenKind::String, result);
    }

    fn consume_number(&mut self) {
        let mut result = self.consume_digit();
        let mut result_kind = token::TokenKind::Int;
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            result_kind = token::TokenKind::Float;
            result += ".";
            self.advance(1);
            result += self.consume_digit().as_ref();
        }
        self.make_token(result_kind, result)
    }

    fn consume_digit(&mut self) -> String {
        let mut result = String::new();
        while self.peek().is_ascii_digit() || self.peek() == '_' {
            if !(self.peek() == '_') {
                result.push(self.peek());
            }
            self.advance(1);
        }
        return result;
    }
}
