#![allow(dead_code)]
use regex::Regex;
use std::{fmt::Display, thread::current, vec};

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}
impl Token {
    fn new(token: String, token_type: TokenType) -> Token {
        Token {
            value: token,
            token_type,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<&Token> for Token {
    fn from(value: &Token) -> Self {
        Token {
            value: value.value.clone(),
            token_type: value.token_type.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    DEFAULT,
    KEYWORD,
    STRING,
    NUMBER,
}
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::DEFAULT => write!(f, "DEFAULT"),
            TokenType::KEYWORD => write!(f, "KEYWORD"),
            TokenType::STRING => write!(f, "STRING"),
            TokenType::NUMBER => write!(f, "NUMBER"),
        }
    }
}

pub struct Tokenizer {
    text: String,
    current_index: usize,
}

impl Tokenizer {
    pub fn new(text: String) -> Tokenizer {
        Tokenizer {
            text,
            current_index: 0,
        }
    }

    fn has_more_tokens(&self) -> bool {
        self.current_index < self.text.len()
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        if !self.has_more_tokens() {
            return None;
        }

        let current_char = self.text.chars().nth(self.current_index).unwrap();

        let mut token = self.get_word();
        println!("Tokenizer token: {}", token);
        println!(
            "Tokenizer text: {}",
            self.text[self.current_index..].to_string()
        );
        let mut token_type = TokenType::DEFAULT;
        println!("Tokenizer index: {}", self.current_index);
        if Regex::new(r#"^"[^"]*""#).unwrap().is_match(&token)
            || Regex::new(r#"^'[^']*'"#).unwrap().is_match(&token)
        {
            // token = self.get_string().unwrap();
            token_type = TokenType::STRING;
        } else if Regex::new(r#"^[A-Za-z0-9]+$"#).unwrap().is_match(&token) {
            // token = self.get_word();
            println!("I am inside the keyword tokenizer!");
            token_type = word_to_keyword(token.clone()).unwrap_or(TokenType::STRING);
        } else if Regex::new(r#"^\d+$"#).unwrap().is_match(&token) {
            // token = self.get_word();
            token_type = TokenType::NUMBER;
        }
        // println!("Tokenizer token: {}", token);
        // println!("Tokenizer token_type: {}", token_type);

        if !token.is_empty() {
            Some(Token::new(token, token_type))
        } else {
            None
        }
    }

    fn peek_word(&self) -> String {
        let mut substring = self.text[self.current_index..].trim_start();
        let mut word = String::new();
        let mut index = 0;
        let quotes = vec!['\"', '\''];

        let mut current_char = substring.chars().nth(index).unwrap();

        if quotes.contains(&current_char) {
            index += 1;
            word.push(current_char);
            while index < substring.len() {
                current_char = substring.chars().nth(index).unwrap();
                word.push(current_char);
                index += 1;
                if quotes.contains(&current_char) {
                    break;
                }
            }
        } else {
            while index < substring.len() {
                let current_char = substring.chars().nth(index).unwrap();
                if !current_char.is_whitespace() {
                    word.push(current_char);
                    index += 1;
                } else {
                    break;
                }
            }
        }
        word
    }

    fn get_word(&mut self) -> String {
        let quotes = vec!['\"', '\''];
        let substring = self.text[self.current_index..].trim_start();
        let mut word = String::new();

        let mut current_char = self.text.chars().nth(self.current_index).unwrap();

        if quotes.contains(&current_char) {
            word.push(current_char);
            self.current_index += 1;
            while self.has_more_tokens() {
                word.push(current_char);
                current_char = self.text.chars().nth(self.current_index).unwrap();
                self.current_index += 1;
                if quotes.contains(&current_char) {
                    break;
                }
            }
        } else {
            while self.has_more_tokens() {
                if !current_char.is_whitespace() {
                    word.push(current_char);
                    self.current_index += 1;
                    current_char = self.text.chars().nth(self.current_index).unwrap();
                } else {
                    break;
                }
            }
        }
        word
    }

    fn get_number_token(&mut self) -> Result<String, &str> {
        let mut number = String::new();
        while self.has_more_tokens() {
            let current_char = self.text.chars().nth(self.current_index).unwrap();
            if current_char.is_numeric() {
                number.push(current_char);
                self.current_index += 1;
            } else {
                if current_char.is_whitespace() {
                    break;
                }
                return Err("Invalid number");
            }
        }
        Ok(number)
    }

    fn get_string(&mut self) -> Result<String, &str> {
        let mut word = String::new();
        let opening_quote = self.text.chars().nth(self.current_index).unwrap_or(' ');
        let mut found_close_quote = false;

        if opening_quote != '\"' && opening_quote != '\'' {
            return Err("Invalid string");
        }
        self.current_index += 1;

        while self.has_more_tokens() {
            let current_char = self.text.chars().nth(self.current_index).unwrap();
            if current_char != opening_quote {
                word.push(current_char);
                self.current_index += 1;
            } else {
                found_close_quote = true;
                break;
            }
        }

        if found_close_quote {
            self.current_index += 1;
            Ok(word)
        } else {
            Err("Invalid string")
        }
    }
}

fn word_to_keyword(word: String) -> Option<TokenType> {
    let keywords = vec!["show", "tables", "table"];
    if keywords.contains(&word.as_str()) {
        Some(TokenType::KEYWORD)
    } else {
        None
    }
}
