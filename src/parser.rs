use std::borrow::Borrow;

use crate::{
    data_frame::Value,
    statement_adt::ParsedStatement,
    tokenizer::{self, Token},
};

pub fn parse_to_adt(statement: &str) -> Result<ParsedStatement, &str> {
    let parts: Vec<&str> = statement.trim().split_whitespace().collect();

    let mut tokenizer = tokenizer::Tokenizer::new(statement.to_string());

    let mut look_ahead = tokenizer.get_next_token();
    let mut value = String::new();

    
    match look_ahead.clone() {
        Some(token) if token.token_type == tokenizer::TokenType::NUMBER => {
            return get_number_parsedstatement(&mut tokenizer, &mut look_ahead);
        }
        Some(token) if token.token_type == tokenizer::TokenType::STRING => {
            return get_string_parsedstatement(&mut tokenizer, &mut look_ahead);
        }
        Some(token) if token.token_type == tokenizer::TokenType::KEYWORD => {
            return get_keyword_parsedstatement(&mut tokenizer, &mut look_ahead);
        }
        Some(_) => {}
        None => return Err("Unexpected end of input"),
    }

    match parts.as_slice() {
        ["show", "tables"] => Ok(ParsedStatement::ShowTables),
        ["show", "table", table_name] => Ok(ParsedStatement::ShowTable(table_name.to_string())),
        _ => Err("Unrecognized statement"),
    }
}

fn get_string_parsedstatement(tokenizer: &mut tokenizer::Tokenizer, look_ahead: &mut Option<Token>) -> Result<ParsedStatement, &'static str> {
    let token = eat_token(tokenizer::TokenType::STRING, tokenizer, look_ahead);

    match token {
        Ok(token) => {
            let string_literal = token.value;
            Ok(ParsedStatement::StringLiteral(string_literal))
        }
        Err(e) => Err(e),
    }
}

fn get_number_parsedstatement(
    tokenizer: &mut tokenizer::Tokenizer,
    look_ahead: &mut Option<Token>,
) -> Result<ParsedStatement, &'static str> {
    let token = eat_token(tokenizer::TokenType::NUMBER, tokenizer, look_ahead);

    match token {
        Ok(token) => Ok(ParsedStatement::NumberLiteral(Value::IntegerValue(
            token.value.parse::<i64>().unwrap(),
        ))),
        Err(e) => Err(e),
    }
}

fn get_keyword_parsedstatement(
    tokenizer: &mut tokenizer::Tokenizer,
    look_ahead: &mut Option<Token>,
) -> Result<ParsedStatement, &'static str> {
    let token = eat_token(tokenizer::TokenType::KEYWORD, tokenizer, look_ahead);

    match token {
        Ok(token) => match token.value.as_str() {
            "show" => {
                let next_token = eat_token(tokenizer::TokenType::KEYWORD, tokenizer, look_ahead);
                match next_token {
                    Ok(next_token) => match next_token.value.as_str() {
                        "tables" => Ok(ParsedStatement::ShowTables),
                        "table" => {
                            let table_name = eat_token(tokenizer::TokenType::STRING, tokenizer, look_ahead);
                            match table_name {
                                Ok(table_name) => Ok(ParsedStatement::ShowTable(table_name.value)),
                                Err(e) => Err(e),
                            }
                        }
                        _ => Err("Unrecognized statement"),
                    },
                    Err(e) => Err(e),
                }
            }
            _ => Err("Unrecognized statement"),
        },
        Err(e) => Err(e),
    }
}

fn eat_token(
    token_type: tokenizer::TokenType,
    tokenizer: &mut tokenizer::Tokenizer,
    look_ahead: &mut Option<Token>,
) -> Result<tokenizer::Token, &'static str> {
    match look_ahead.take() {
        Some(t) if t.token_type == token_type => {
            *look_ahead = tokenizer.get_next_token();
            Ok(t)
        }
        Some(_) => {
            *look_ahead = tokenizer.get_next_token();
            Err("Unexpected token")
        }
        None => {
            Err("Unexpected end of input")
        }
    }
}
