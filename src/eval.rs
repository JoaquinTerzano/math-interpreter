use crate::structs::{InterpreterError, MathConstants, Node, MathFunctions};

pub fn eval(ast: Node) -> Result<f32, InterpreterError> {
    match ast {
        Node::Add(a, b) => {
            let a = eval(*a);
            let b = eval(*b);
            if a.is_ok() && b.is_ok() {
                return Ok(a.unwrap() + b.unwrap());
            }
            a
        }
        Node::Subtract(a, b) => {
            let a = eval(*a);
            let b = eval(*b);
            if a.is_ok() && b.is_ok() {
                return Ok(a.unwrap() - b.unwrap());
            }
            a
        }
        Node::Multiply(a, b) => {
            let a = eval(*a);
            let b = eval(*b);
            if a.is_ok() && b.is_ok() {
                return Ok(a.unwrap() * b.unwrap());
            }
            a
        }
        Node::Divide(a, b) => {
            let a = eval(*a);
            let b = eval(*b);

            if a.is_ok() && b.is_ok() {
                let unwrapped_b = b.unwrap();
                if unwrapped_b == 0.0 {
                    return Err(InterpreterError::MathError("Division by zero".to_string()));
                }
                return Ok(a.unwrap() / unwrapped_b);
            }
            a
        }
        Node::Pow(a, b) => {
            let a = eval(*a);
            let b = eval(*b);
            if a.is_ok() && b.is_ok() {
                return Ok(a.unwrap().powf(b.unwrap()));
            }
            a
        }
        Node::Negative(n) => {
            let exp = eval(*n);
            if exp.is_ok() {
                return Ok(-exp.unwrap());
            }
            exp
        }

        Node::Number(n) => n
            .parse::<f32>()
            .map_err(|e| InterpreterError::MathError(e.to_string())),

        Node::Degree(n) => {
            let degree = n.trim_end_matches('o')
                .parse::<f32>()
                .map_err(|e| InterpreterError::MathError(e.to_string()))?;
            
            return Ok(MathFunctions::to_radians(degree))
        },

        Node::Constant(c) => match c {
            MathConstants::Pi => Ok(std::f32::consts::PI),
            MathConstants::E => Ok(std::f32::consts::E),
        },
        Node::Parenthesis(n) => {
            let exp = eval(*n);
            if exp.is_ok() {
                return Ok(exp.unwrap());
            }
            exp
        } //Node::Root => 0.0,
    }
}
