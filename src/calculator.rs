#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bracket {
    OpeningParenthesis, 
    ClosingParenthesis,
}

impl Bracket{
    fn operator_precedence(&self) -> u8 {
        6
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Token{
    Number(i128),
    Br(Bracket),
    Op(Operator),
}

pub struct Parser{
    tokens: Vec<Token>, 
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

                '(' | ')' => {
                    if !current_number.is_empty(){
                        tokens.push(Token::Number(current_number.parse().unwrap()));
                        current_number = String::new();
                    }

                    match c {
                        '(' =>  tokens.push(Token::Br(Bracket::OpeningParenthesis)), 
                        ')' => tokens.push(Token::Br(Bracket::ClosingParenthesis)),

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
        println!("{:?}", tokens);
        Parser { 
            tokens, 
        }
    }

    pub fn compute(&mut self) -> i128 {
        let postfixed_tokens = self.postfix(); 
        let mut stack: Vec<i128> = Vec::new();

        let mut result: i128 = 0;

        for token in postfixed_tokens{
            match token{
                Token::Number(n) => stack.push(*n),

                Token::Op(op) => {
                    let right_number = stack.pop().unwrap();

                    result = stack.pop().unwrap();
                    

                    match op{
                        Operator::Add => result += right_number, 
                        Operator::Substract => result -= right_number, 
                        Operator::Multiply => result *= right_number, 
                        Operator::Divide => result /= right_number,
                    }

                    stack.push(result);
                }

                Token::Br(_) => {}
            }
        }
        result
    }
    fn postfix(&mut self) -> Vec<&Token>{


        let mut queue: Vec<&Token> = Vec::new();
        let mut stack: Vec<&Token> = Vec::new();

        for token in &self.tokens {
            match token{
                Token::Number(_) => queue.push(token), 
                Token::Op(operator) => {
                    if stack.is_empty(){
                        stack.push(token);
                    }
                    else{
                        let last_token_precedence = match stack.last().unwrap(){
                            Token::Br(bracket) => bracket.operator_precedence(),
                            Token::Op(operator) => operator.operator_precedence(),
                            Token::Number(_) => 0, 
                        };

                        match operator.operator_precedence() {
                            operator_precedence if operator_precedence > last_token_precedence => stack.push(token), 
                            operator_precedence if operator_precedence <= last_token_precedence => {
                                queue.push(stack.pop().unwrap());
                                stack.push(token); 
                            }
                            _ => {}
                        }
                    }
                }
                Token::Br(bracket) => {
                    match bracket{
                        Bracket::OpeningParenthesis => stack.push(token), 
                        Bracket::ClosingParenthesis => {
                            while let Some(Token::Op(_operator)) = stack.last() {
                                // Push any operator to the queue
                                queue.push(stack.pop().unwrap());
                            }
                            // The while loop won't pop the opening parenthesis, so we do it manually
                            stack.pop();
                        }
                    }
                }
            }

        }
        for token in stack.iter().rev() {
            queue.push(*token);
        }
        println!("{:?}", queue); 
        queue
        /* 
        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();

        for token in &self.tokens{
            match token{
                Token::Number(_) => queue.push(*token),
                Token::Op(op) => {
                    if stack.is_empty(){
                        stack.push(*token);
                    }
                    let last_stack_token = match stack.last().unwrap(){
                        Token::Op(op) => op, 
                        _ => &Operator::Add
                    };
                    match op.operator_precedence() {
                        token_precedence if token_precedence > last_stack_token.operator_precedence() => stack.push(Token::Op(*op)), 
                        token_precedence if token_precedence <= last_stack_token.operator_precedence() => {
                            let last_value = stack.pop().unwrap();
                            queue.push(last_value);
                            stack.push(Token::Op(*op));
                        }
                        _ => {}
                    }
                }
                Token::Br(br) => {
                    match br{
                        Bracket::OpeningParenthesis => stack.push(Token::Br(*br)), 
                        Bracket::ClosingParenthesis => {
                            while let Some(&Token::Br(bracket)) = stack.last() {
                                println!("{:?}", stack.last());
                                if bracket == Bracket::OpeningParenthesis{
                                    stack.pop();
                                    break; 
                                }
                                queue.push(stack.pop().unwrap());                                
                                
                            }
                        }
                    }
                }
            }
        }
        for token in stack.iter().rev() {
            queue.push(*token);
        }
        println!("{:?}", queue);
        queue
        */
    }
}
