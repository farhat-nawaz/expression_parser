trait ExpressionParser {
    fn parse(input: &str) -> Result<Expression, String>;
    fn evaluate(&self) -> Result<f64, String>;
}

#[derive(Debug)]
enum Token {
    Number(f64),
    Operator(char),
    LeftParen,
    RightParen,
}

#[derive(Debug)]
struct Expression {
    tokens: Vec<Token>,
}

impl ExpressionParser for Expression {
    fn parse(input: &str) -> Result<Expression, String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();

        for c in input.chars() {
            match c {
                '0'..='9' => current_token.push(c),
                'a' | 'b' | 'c' | 'd' => {
                    let op = match c {
                        'a' => '+',
                        'b' => '-',
                        'c' => '*',
                        'd' => '/',
                        _ => continue,
                    };
                    if !current_token.is_empty() {
                        tokens.push(Token::Number(current_token.parse().unwrap()));
                        current_token.clear();
                    }
                    tokens.push(Token::Operator(op));
                }
                'e' => tokens.push(Token::LeftParen),
                'f' => tokens.push(Token::RightParen),
                ' ' => continue,
                _ => return Err(format!("Unexpected character: {}", c)),
            }
        }

        if !current_token.is_empty() {
            tokens.push(Token::Number(current_token.parse().unwrap()));
        }

        Ok(Expression { tokens })
    }

    fn evaluate(&self) -> Result<f64, String> {
        let mut operator_stack = Vec::new();
        let mut operand_stack = Vec::new();
        let mut last_was_operator = false;

        for token in &self.tokens {
            println!(
                "\nToken: {:?}\nOperand Stack: {:?}\nOperator Stack: {:?}",
                token, operand_stack, operator_stack
            );
            match token {
                Token::Number(number) => operand_stack.push(*number),
                Token::Operator(operator) => {
                    if operand_stack.is_empty() {
                        return Err(format!("Operator {} without operands", operator));
                    } else if operator_stack.len() > 1 {
                        return Err(format!("Too many consecutive operators"));
                    } else if last_was_operator && operand_stack.len() < 2 {
                        return Err(format!("Invalid expression"));
                    }

                    if !last_was_operator {
                        operator_stack.push(*operator);
                        last_was_operator = true;
                        continue;
                    }

                    let right_operand = operand_stack.pop().unwrap();
                    let left_operand = operand_stack.pop().unwrap();

                    operand_stack.push(match operator_stack.pop() {
                        Some('+') => left_operand + right_operand,
                        Some('-') => left_operand - right_operand,
                        Some('*') => left_operand * right_operand,
                        Some('/') => left_operand / right_operand,
                        _ => return Err(format!("Invalid operator: {}", operator)),
                    });

                    // last_was_operator = true;
                    println!("{:?}", operator_stack);
                    // operator_stack.pop();
                    operator_stack.push(*operator)
                }
                Token::LeftParen => operator_stack.push('('),
                Token::RightParen => {
                    if operator_stack.is_empty() {
                        return Err(format!("Unexpected right parenthesis"));
                    }

                    while operator_stack.last().unwrap() != &'(' {
                        let operator = operator_stack.pop().unwrap();
                        let right_operand = operand_stack.pop().unwrap();
                        let left_operand = operand_stack.pop().unwrap();

                        operand_stack.push(match operator {
                            '+' => left_operand + right_operand,
                            '-' => left_operand - right_operand,
                            '*' => left_operand * right_operand,
                            '/' => left_operand / right_operand,
                            _ => return Err(format!("Invalid operator: {}", operator)),
                        });
                    }

                    operator_stack.pop();
                }
            }
        }

        if operator_stack.len() == 1 {
            if operand_stack.len() == 2 {
                let right_operand = operand_stack.pop().unwrap();
                let left_operand = operand_stack.pop().unwrap();

                match operator_stack.pop() {
                    Some('+') => Ok(left_operand + right_operand),
                    Some('-') => Ok(left_operand - right_operand),
                    Some('*') => Ok(left_operand * right_operand),
                    Some('/') => Ok(left_operand / right_operand),
                    _ => return Err(format!("Invalid operator")),
                }
            } else {
                Err(format!("Too many operands"))
            }
        } else {
            Err(format!("Too many operators"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parser_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let expression = Expression::parse(input).unwrap();

                println!("{:?}", expression.tokens);
                assert_eq!(expression.evaluate().unwrap(), expected);

            }
        )*
        }
    }

    parser_tests! {
        parse_3a2c4: ("3a2c4", 20 as f64),
        parse_32a2d2: ("32a2d2", 17 as f64),
        parse_500a10b66c32: ("500a10b66c32", 14208 as f64),
        // parser_3ae4c66fb32: ("3ae4c66fb32", 235 as f64),
    }
}
