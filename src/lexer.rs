use core::panic;

use regex::Regex;

use crate::structs::{InterpreterError, MathConstants, Token};

const NUMBER_REGEX: &str = r"^\d+(\.\d*)?$";

const DEGREE_REGEX: &str = r"^\d+(\.\d*)?(o)$";

fn is_pi(source: &str) -> bool {
    (source == "pi") || (source == "π")
}
fn is_e(source: &str) -> bool {
    source == "e"
}

fn is_number(source: &str) -> bool {
    let re = Regex::new(NUMBER_REGEX).unwrap();
    re.is_match(source)
}
fn is_degree(source: &str) -> bool {
    let re = Regex::new(DEGREE_REGEX).unwrap();
    re.is_match(source)
}
fn is_pow(source: &str) -> bool {
    source == "^"
}
fn is_plus(source: &str) -> bool {
    source == "+"
}
fn is_minus(source: &str) -> bool {
    source == "-"
}
fn is_multiply(source: &str) -> bool {
    source == "*"
}
fn is_divide(source: &str) -> bool {
    source == "/"
}
fn is_left_parenthesis(source: &str) -> bool {
    source == "("
}
fn is_right_parenthesis(source: &str) -> bool {
    source == ")"
}

fn get_possible_tokens(source: &str) -> Vec<Token> {
    let mut possible_tokens: Vec<Token> = Vec::new();
    if is_number(source) {
        possible_tokens.push(Token::Number(source.to_string()));
    }
    if is_degree(source) {
        possible_tokens.push(Token::Degree(source.to_string()));
    }
    if is_pow(source) {
        possible_tokens.push(Token::PowOp);
    }
    if is_pi(source) {
        possible_tokens.push(Token::Constant(MathConstants::Pi));
    }
    if is_e(source) {
        possible_tokens.push(Token::Constant(MathConstants::E));
    }
    if is_plus(source) {
        possible_tokens.push(Token::PlusOp);
    }
    if is_minus(source) {
        possible_tokens.push(Token::MinusOp);
    }
    if is_multiply(source) {
        possible_tokens.push(Token::MultiplyOp);
    }
    if is_divide(source) {
        possible_tokens.push(Token::DivideOp);
    }
    if is_left_parenthesis(source) {
        possible_tokens.push(Token::LeftParenthesis);
    }
    if is_right_parenthesis(source) {
        possible_tokens.push(Token::RightParenthesis);
    }
    possible_tokens
}

pub fn lexer(source: &str) -> Result<Vec<Token>, InterpreterError> {
    //println!("lexer: {}", source);
    let source_lower = source.to_lowercase();

    let mut tokens: Vec<Token> = Vec::new();
    let mut previous_possible_tokens: Vec<Token> = Vec::new();

    let mut start = 0;
    let mut end = 1;

    while end <= source_lower.len() {
        let current_char = &source_lower[start..end];
        let possible_tokens: Vec<Token> = get_possible_tokens(current_char);
        //println!("{:?}_{}_({}-{})", possible_tokens, current_char, start, end);

        if possible_tokens.len() == 0 {
            if current_char == " " {
                start += 1;
                end += 1;
            } else {
                if previous_possible_tokens.len() >= 1 {
                    tokens.push(previous_possible_tokens[0].clone());
                    previous_possible_tokens = Vec::new();
                    start = end - 1;
                } else {
                    end += 1;
                }
            }
        } else {
            previous_possible_tokens = possible_tokens;
            end += 1;
        }

        if end > source_lower.len() {
            if previous_possible_tokens.len() >= 1 {
                tokens.push(previous_possible_tokens[0].clone());
            } else {
                return Err(InterpreterError::InvalidToken(current_char.to_string()));
            }
        }
    }

    Ok(tokens)
}
