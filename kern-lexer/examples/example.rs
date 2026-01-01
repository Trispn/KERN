use kern_lexer::{Lexer, Token, TokenType};

fn main() {
    let input = r#"
entity Farmer {
    id
    location
    produce
}

rule CheckLocation:
    if farmer.location == "valid"
    then approve_farmer(farmer)
"#;

    let mut lexer = Lexer::new(input);
    
    println!("Tokenizing input:\n{}", input);
    println!("\nTokens:");
    
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        
        if matches!(token.token_type, TokenType::Eof) {
            break;
        }
    }
}