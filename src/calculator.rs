pub enum Operator{
    Add, 
    Substract, 
    Multiply, 
    Divide,
}

impl Operator{
    fn operator_precedence(&self) -> u8 {
        match self{
            Operator::Multiply => 3, 
            Operator::Divide => 3, 
            Operator::Substract => 2,
            Operator::Add => 2
        }
    }
}
pub enum Token{
    Number(i64),
    Op(Operator)
}

pub struct Parser{
    expression: str,
    tokens: Vec<Token>, 
    current_token: Option<Token>, 
    index: usize,
}

impl Parser{
    pub fn new(expression: &str) -> Parser{

        let mut tokens = Vec::new();
        let mut current_number = String::new();

        for c in expression.chars(){
            match c{

                '+' | '-' | '*' | '/' => {
                    if !current_number.is_empty(){
                        tokens.push(Token::Number(current_number.parse().unwrap()));
                        current_number = String::new();
                    }
                    match c{
                        '+' => tokens.push(Token::Add), 
                        '-' => tokens.push(Token::Substract), 
                        '*' => tokens.push(Token::Multiply), 
                        '/' => tokens.push(Token::Divide),
                    }
                }
            
                '0'..='9' => current_number.push(c), 

                _ => {}
            }
        }
        if !current_number.is_empty(){
            tokens.push(Token::Number(current_number.parse().unwrap()));
        }

        Parser {
            expression, 
            tokens, 
            current_token: None, 
            index: 0,
        }
    }

    pub fn compute(&mut self) -> i64{
        let postfixed_tokens = self.postfix(); 
        let mut stack:i64= Vec::new();

        let mut result: i64 = 0;

        for token in postfixed_tokens{
            match token{
                Token::Number(n) => stack.push(n),

                Token::Op(op) => {
                    let right_number = stack.last();
                    stack.pop();

                    result = stack.last();
                    stack.pop();

                    match op{
                        Operator::Add => result += right_number, 
                        Operator::Substract => result -= right_number, 
                        Operator::Multiply => result *= right_number, 
                        Operator::Divide => result /= right_number,
                    }

                    stack.push(result);
                }
            }
        }
        stack[0]
    }
    fn postfix(&mut self) -> Vec<Token>{
        let mut tokens = self.tokens;

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::news();

        for token in tokens{
            match token{
                Token::Number(_) => queue.push(token),
                Token::Op(op) => {
                    if stack.is_empty(){
                        stack.push(op)
                    }
                    else{
                        let last_value = stack.last();
                        match op.operator_precedence(){
                           n if n > last_value.operator_precedance() => stack.push(op), 
                           n if n <= last_value.operator_precedance() => {
                                queue.push(last_value);
                                stack.pop();
                                stack.push(op);
                           }
                           _ => {}
                        }
                    }
                }
            }
        }
        for token in stack{
            queue.push(token);
        }
        queue
    }
}
