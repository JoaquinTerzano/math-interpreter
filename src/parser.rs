use crate::structs::{InterpreterError, Node, Token};

fn token_is_operator(token: &Token) -> bool {
    match token {
        Token::PlusOp | Token::MinusOp | Token::MultiplyOp | Token::DivideOp | Token::PowOp => true,
        _ => false,
    }
}

// Parsea un número
fn parse_number(tokens: &[Token]) -> Option<Node> {
    if tokens.len() == 1 {
        match &tokens[0] {
            Token::Number(n) => {
                let node = Node::Number(n.to_string());
                return Some(node);
            }
            _ => {}
        }
    } else if tokens.len() == 0 {
        return Some(Node::Number("0".to_string()));
    }
    None
}

// Parsea un ángulo
fn parse_degree(tokens: &[Token]) -> Option<Node> {
    if tokens.len() == 1 {
        match &tokens[0] {
            Token::Degree(d) => {
                let node = Node::Degree(d.to_string());
                return Some(node);
            }
            _ => {}
        }
    }
    None
}

// Parsea la operación potencia entre dos expresiones
fn parse_pow(tokens: &[Token]) -> Option<Node> {
    let mut opened_parenthesis = 0;

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::LeftParenthesis => {
                opened_parenthesis += 1;
            }
            Token::RightParenthesis => {
                opened_parenthesis -= 1;
            }
            Token::PowOp => {
                if opened_parenthesis == 0 {
                    if let Some(left_side) = parse_expression(&tokens[0..i]) {
                        if let Some(right_side) = parse_expression(&tokens[i + 1..]) {
                            return Some(Node::Pow(Box::new(left_side), Box::new(right_side)));
                        }
                    }
                }
            }

            _ => {}
        }
    }
    return None;
}

// Parsea la operación suma entre dos expresiones
fn parse_add(tokens: &[Token]) -> Option<Node> {
    let mut opened_parenthesis = 0;

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::LeftParenthesis => {
                opened_parenthesis += 1;
            }
            Token::RightParenthesis => {
                opened_parenthesis -= 1;
            }
            Token::PlusOp => {
                if opened_parenthesis == 0 {
                    if let Some(left_side) = parse_expression(&tokens[0..i]) {
                        if let Some(right_side) = parse_expression(&tokens[i + 1..]) {
                            return Some(Node::Add(Box::new(left_side), Box::new(right_side)));
                        }
                    }
                }
            }

            _ => {}
        }
    }
    return None;
}

// Parsea la operación resta entre dos expresiones.
fn parse_substract(tokens: &[Token]) -> Option<Node> {
    let mut opened_parenthesis = 0;

    for (i, token) in tokens.iter().enumerate().rev() {
        match token {
            Token::LeftParenthesis => {
                opened_parenthesis += 1;
            }
            Token::RightParenthesis => {
                opened_parenthesis -= 1;
            }
            Token::MinusOp => {
                //println!("Left {:?} - Right {:?}", &tokens[i], &tokens[i + 1]);
                if opened_parenthesis == 0
                //&& tokens[0..i].len() >= 1
                {
                    if i > 0
                        && !token_is_operator(&tokens[i - 1])
                        && !token_is_operator(&tokens[i + 1])
                    {
                        if let Some(left_side) = parse_expression(&tokens[0..i]) {
                            if let Some(right_side) = parse_expression(&tokens[i + 1..]) {
                                return Some(Node::Subtract(
                                    Box::new(left_side),
                                    Box::new(right_side),
                                ));
                            }
                        }
                    }
                }
            }

            _ => {}
        }
    }
    return None;
}

// Parsea la operación multiplicación entre dos expresiones
fn parse_multiply(tokens: &[Token]) -> Option<Node> {
    let mut opened_parenthesis = 0;

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::LeftParenthesis => {
                opened_parenthesis += 1;
            }
            Token::RightParenthesis => {
                opened_parenthesis -= 1;
            }
            Token::MultiplyOp => {
                if opened_parenthesis == 0 {
                    if let Some(left_side) = parse_expression(&tokens[0..i]) {
                        if let Some(right_side) = parse_expression(&tokens[i + 1..]) {
                            return Some(Node::Multiply(Box::new(left_side), Box::new(right_side)));
                        }
                    }
                }
            }

            _ => {}
        }
    }
    return None;
}

// Parsea la operación division entre dos expresiones
fn parse_divide(tokens: &[Token]) -> Option<Node> {
    let mut opened_parenthesis = 0;

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::LeftParenthesis => {
                opened_parenthesis += 1;
            }
            Token::RightParenthesis => {
                opened_parenthesis -= 1;
            }
            Token::DivideOp => {
                if opened_parenthesis == 0 {
                    if let Some(left_side) = parse_expression(&tokens[0..i]) {
                        if let Some(right_side) = parse_expression(&tokens[i + 1..]) {
                            return Some(Node::Divide(Box::new(left_side), Box::new(right_side)));
                        }
                    }
                }
            }

            _ => {}
        }
    }
    return None;
}

fn parse_between_parenthesis_expression(tokens: &[Token]) -> Option<Node> {
    let first = tokens.first();
    let last = tokens.last();

    if first.is_some() && last.is_some() {
        let first = first.unwrap();
        let last = last.unwrap();

        if *first == Token::LeftParenthesis && *last == Token::RightParenthesis {
            let inside_expression = parse_expression(&tokens[1..tokens.len() - 1]);
            if let Some(n) = inside_expression {
                return Some(Node::Parenthesis(Box::new(n)));
            }
        }
    }
    None
}

// Es una expresion normal, solamente contempla la posibilidad de que sea un numero negativo
/* fn parse_left_side_expression(tokens: &[Token], parent: &Option<Node>) -> Option<Node> {
    if tokens.len() == 0 {
        return Some(Node::Number("0".to_string()));
    }
    if tokens.len() > 1 && tokens[0] == Token::MinusOp {
        if let Some(n) = parse_expression(&tokens[1..], parent) {
            let mut node = Some(Node::Negative(Box::new(n)));
            return node;
        }
    }

    None
} */

fn parse_negative(tokens: &[Token]) -> Option<Node> {
    //if parent.t == NodeType::Parenthesis {
    if tokens.len() > 1
        && tokens[0] == Token::MinusOp
        && (tokens[1] == Token::LeftParenthesis || tokens.len() == 2)
    {
        if let Some(expression) = parse_expression(&tokens[1..]) {
            return Some(Node::Negative(Box::new(expression)));
        }
    }
    //}
    None
}

fn parse_constant(tokens: &[Token]) -> Option<Node> {
    if tokens.len() == 1 {
        match &tokens[0] {
            Token::Constant(c) => {
                let node = Node::Constant(c.clone());
                return Some(node);
            }
            _ => {}
        }
    }
    None
}

// Parsea una expresión, puede ser una operación o un número no negativo
fn parse_expression(tokens: &[Token]) -> Option<Node> {
    let mut node = parse_between_parenthesis_expression(tokens);
    if node.is_none() {
        node = parse_negative(tokens);
    }
    if node.is_none() {
        node = parse_add(tokens);
    }
    if node.is_none() {
        node = parse_substract(tokens);
    }

    if node.is_none() {
        node = parse_multiply(tokens);
    }
    if node.is_none() {
        node = parse_pow(tokens);
    }
    if node.is_none() {
        node = parse_divide(tokens);
    }
    if node.is_none() {
        node = parse_constant(tokens);
    }
    if node.is_none() {
        node = parse_number(tokens)
    }
    if node.is_none() {
        node = parse_degree(tokens)
    }

    node
}

/*

( 1 - 1 ) * 5

Grammar!

S -> Expression

Expression -> Add | Substract |
              Multiply | Divide | BetweenParenthesisExpression | Number | LeftSideExpression |
              Degree | Constant

BetweenParenthesisExpression ->  LeftParenthesis LeftSideExpression RightParenthesis


Add -> LeftSideExpression PlusOp Expression
Substract -> LeftSideExpression MinusOp Expression
Multiply -> LeftSideExpression MultiplyOp Expression
Divide -> LeftSideExpression DivideOp Expression
Degree -> Expression DegreeOp


*/

// Build AST (Abstract-Syntax-Tree)
pub fn parse(tokens: Vec<Token>) -> Result<Node, InterpreterError> {
    let root = parse_expression(&tokens);

    let serialized = serde_json::to_string_pretty(&root).unwrap();
    //println!("{}", serialized);

    if root.is_none() {
        return Err(InterpreterError::SyntaxError);
    }

    Ok(root.unwrap())
}
