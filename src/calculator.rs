
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
    expression: String, 
    result: i128
}

impl Parser{
    pub fn new(expression: &str) -> Parser{

        let mut tokens = Vec::new();

        let mut current_number = String::new();

        // Every number without an explicit sign is positive
        let mut sign_carry: char = '+';

        for c in expression.chars(){
    
            match c{
                '-' | '+' => {
                    if current_number.is_empty(){
                        sign_carry = compare_signs(sign_carry, c);
                        println!("Sign carry is: {}", sign_carry);
                    }
                    else{
                        tokens.push(Token::Number(parse_number_with_sign(current_number, sign_carry)));
                        sign_carry = '+';
                        current_number = String::new();
                        
                        match c{
                            '-' => tokens.push(Token::Op(Operator::Substract)), 
                            '+' => tokens.push(Token::Op(Operator::Add)), 
                            _ => {}
                        }
                                                
                    }
                }
                '*' | '/' => {
                    if !current_number.is_empty(){
                        tokens.push(Token::Number(parse_number_with_sign(current_number, sign_carry)));
                        current_number = String::new();
                    }
                    match c{ 
                        '*' => tokens.push(Token::Op(Operator::Multiply)), 
                        '/' => tokens.push(Token::Op(Operator::Divide)),
                        _ => {}
                    }
                }
                
                '(' | ')' => {
                    if !current_number.is_empty(){
                        tokens.push(Token::Number(parse_number_with_sign(current_number, sign_carry)));
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
            tokens.push(Token::Number(parse_number_with_sign(current_number, sign_carry)));
        }
        println!("{:?}", tokens);
        Parser { 
            tokens, 
            expression: String::from(expression), 
            result: 0
        }
    }

    pub fn compute(&mut self) -> i128 {
        let postfixed_tokens = self.postfix(); 
        let mut stack: Vec<i128> = Vec::new();

        let mut result: i128 = 0;

        for token in postfixed_tokens{
            match token{
                Token::Number(n) => {
                    result = *n;
                    stack.push(result);
                },

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
        self.result = result;
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
    }
}

fn compare_signs(left: char, right:char) -> char{
    let mut sign: char = '+';
    match left{
        '+' => {
            match right{
                '+' => sign = '+', 
                '-' => sign = '-',
                _ => {}
            }
        },
        '-' => {
            match right{
                '+' => sign = '-', 
                '-' => sign = '+',
                _ => {}
            }
        }, 
        _ => {}
    }
    sign
}

fn parse_number_with_sign(mut number: String, sign: char) -> i128 {
    // Useful for when you only have the unsigned representation of a number and wish to parse it signed
    number.insert(0, sign);
    
    number.parse().unwrap()
}
