#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum Token{
    Number(i64),
    Op(Operator)
}

pub struct Parser{
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
                        '+' => tokens.push(Token::Op(Operator::Add)), 
                        '-' => tokens.push(Token::Op(Operator::Substract)), 
                        '*' => tokens.push(Token::Op(Operator::Multiply)), 
                        '/' => tokens.push(Token::Op(Operator::Divide)),
                        _ => {}
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
            tokens, 
            current_token: None, 
            index: 0,
        }
    }

    pub fn compute(&mut self) -> i64{
        let postfixed_tokens = self.postfix(); 
        let mut stack: Vec<i64> = Vec::new();

        let mut result: i64 = 0;

        for token in postfixed_tokens{
            match token{
                Token::Number(n) => stack.push(n),

                Token::Op(op) => {
                    let right_number = *stack.last().unwrap_or(&0);
                    stack.pop();

                    result = *stack.last().unwrap_or(&0);
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
        result
    }
    fn postfix(&mut self) -> Vec<Token>{

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Operator> = Vec::new();

        for token in &self.tokens{
            match token{
                Token::Number(_) => queue.push(*token),
                Token::Op(op) => {
                    if stack.is_empty(){
                        stack.push(*op);
                    }
                    else{
                        let last_value = stack.last().unwrap_or(&Operator::Add);
                        match op.operator_precedence(){
                           n if n > last_value.operator_precedence() => stack.push(*op), 
                           n if n <= last_value.operator_precedence() => {
                                queue.push(Token::Op(*op));
                                stack.pop();
                                stack.push(*op);
                           }
                           _ => {}
                        }
                    }
                }
            }
        }
        for op in stack{
            queue.push(Token::Op(op));
        }
        queue
    }
}
