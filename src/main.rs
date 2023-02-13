mod calculator;

use std::io::{stdin, stdout, Write, BufRead};


fn main(){
    let mut lines = stdin().lock().lines();
     
    loop{
        print!("> "); 
        
        stdout().flush().unwrap();

        let line = lines.next(); 

        match line {
            Some(Ok(line)) => {
                let mut parser = calculator::Parser::new(line.as_str());
                let result = parser.compute();

                println!("{}", result);
            }, 

            _ => break
        }
    }
}