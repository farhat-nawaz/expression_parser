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
                'a'..='f' => {
                    let op = match c {
                        'a' => Token::Operator('+'),
                        'b' => Token::Operator('-'),
                        'c' => Token::Operator('*'),
                        'd' => Token::Operator('/'),
                        'e' => Token::LeftParen,
                        'f' => Token::RightParen,
                        _ => continue,
                    };
                    if !current_token.is_empty() {
                        tokens.push(Token::Number(current_token.parse().unwrap()));
                        current_token.clear();
                    }
                    tokens.push(op);
                }
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

                    operator_stack.push(*operator)
                }
                Token::LeftParen => {
                    operator_stack.push('(');
                    last_was_operator = false;
                }
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

                    last_was_operator = false;
                    operator_stack.pop();
                }
            }
        }

        if operator_stack.len() > 0 {
            if !operand_stack.is_empty() {
                while operand_stack.len() > 1 {
                    let right_operand = operand_stack.pop().unwrap();
                    let left_operand = operand_stack.pop().unwrap();

                    operand_stack.push(match operator_stack.pop() {
                        Some('+') => left_operand + right_operand,
                        Some('-') => left_operand - right_operand,
                        Some('*') => left_operand * right_operand,
                        Some('/') => left_operand / right_operand,
                        _ => return Err("Not enough operator to apply".to_owned()),
                    });
                }
                Ok(operand_stack.pop().unwrap())
            } else {
                Err(format!("Too many operands"))
            }
        } else {
            Ok(operand_stack.pop().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parameterize_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let expression = Expression::parse(input).unwrap();

                assert_eq!(expression.evaluate().unwrap(), expected);

            }
        )*
        }
    }

    parameterize_tests! {
        parse_3a2c4: ("3a2c4", 20_f64),
        parse_32a2d2: ("32a2d2", 17_f64),
        parse_500a10b66c32: ("500a10b66c32", 14208_f64),
        parse_3ae4c66fb32: ("3ae4c66fb32", 235_f64),
        parse_3c4d2aee2a4c41fc4f: ("3c4d2aee2a4c41fc4f", 990_f64),
    }
}
// 3+(4*66)-32
