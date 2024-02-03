#[cfg(test)]
mod tests {
    use math_interpreter::structs::{Node, Token};

    use math_interpreter::eval::eval;
    use math_interpreter::lexer::lexer;
    use math_interpreter::parser::parse;

    #[test]
    fn test_lexer() {
        let input = "1 + 2";
        let tokens = lexer(&input);
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token::Number("1".to_string()),
                Token::PlusOp,
                Token::Number("2".to_string())
            ]
        );
    }

    #[test]
    fn test_parser() {
        let tokens = vec![
            Token::Number("1".to_string()),
            Token::PlusOp,
            Token::Number("2".to_string()),
        ];
        let ast = parse(tokens);
        assert_eq!(
            ast.unwrap(),
            Node::Add(
                Box::new(Node::Number("1".to_string())),
                Box::new(Node::Number("2".to_string()))
            )
        );
    }

    #[test]
    fn test_eval1() {
        let tokens = lexer("(1 + 2) * 9");
        let ast = parse(tokens.unwrap());
        let result = eval(ast.unwrap());
        assert_eq!(result.unwrap(), 27.0);
    }

    #[test]
    fn test_eval2() {
        let tokens = lexer("(1 / 2) * 2 + 5");
        let ast = parse(tokens.unwrap());
        let result = eval(ast.unwrap());
        assert_eq!(result.unwrap(), 6.0);
    }

    #[test]
    fn test_eval3() {
        let tokens = lexer("- 5 * 8  / 2");
        let ast = parse(tokens.unwrap());
        let result = eval(ast.unwrap());
        assert_eq!(result.unwrap(), -20.0);
    }

    #[test]
    fn test_eval4() {
        let tokens = lexer("1 - (5 ^ 2) - 0.1");
        let ast = parse(tokens.unwrap());
        let result = eval(ast.unwrap());
        assert_eq!(result.unwrap(), -24.1);
    }

    #[test]
    fn test_eval5() {
        let tokens = lexer("1 - 1 - 1 - 1");
        let ast = parse(tokens.unwrap());
        let result = eval(ast.unwrap());
        assert_eq!(result.unwrap(), -2.0);
    }

    #[test]
    fn test_eval6() {
        let tokens = lexer("1 - (- 1)");
        let ast = parse(tokens.unwrap());
        let result = eval(ast.unwrap());
        assert_eq!(result.unwrap(), 2.0);
    }

    #[test]
    fn test_eval7() {
        let tokens = lexer("-0-0-0+1");
        let ast = parse(tokens.unwrap());
        let result = eval(ast.unwrap());
        assert_eq!(result.unwrap(), 1.0);
    }
    #[test]
    fn test_eval8() {
        let tokens = lexer("-(-(-3))");
        let ast = parse(tokens.unwrap());
        let result = eval(ast.unwrap());
        assert_eq!(result.unwrap(), -3.0);
    }

    // Pruebo el grado
    #[test]
    fn test_eval9() {
        let tokens = lexer("90.0o");
        let ast = parse(tokens.unwrap());
        let result = eval(ast.unwrap());
        assert_eq!(result.unwrap(), 90.0 * std::f32::consts::PI / 180.0);
    }
}
