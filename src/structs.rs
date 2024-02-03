use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MathConstants {
    Pi,
    E,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),
    Constant(MathConstants),
    Degree(String),
    PlusOp,
    MinusOp,
    MultiplyOp,
    DivideOp,
    LeftParenthesis,
    RightParenthesis,
    PowOp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]

pub enum Node {
    Parenthesis(Box<Node>),
    Negative(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Number(String),
    Constant(MathConstants),
    Degree(String),
}

#[derive(Debug, Clone)]
pub enum InterpreterError {
    MathError(String),
    SyntaxError,
    InvalidToken(String),
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
/* impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}
 */


// Funciones matemáticas
pub mod MathFunctions {
    pub fn to_radians(degree: f32) -> f32 {
        degree * std::f32::consts::PI / 180.0
    }
}