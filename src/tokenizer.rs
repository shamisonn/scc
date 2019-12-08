pub enum Token {
    Operation {
        val: String
    },
    Integer {
        val: i32
    },
    // to be determined
    TBD,
}

// endpoint
pub fn make_token_vector(input: String) -> Vec<Token> {
    let mut token_vec: Vec<Token> = Vec::new();
    let input_without_ws: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();

    let mut raw_token: String = String::from("");
    for c in input_without_ws {
        let token = tokenize(&raw_token, c);
        match token {
            Token::TBD => raw_token.push(c),
            _ => {
                token_vec.push(token);
                raw_token = c.to_string();
            }

        }
    }
    token_vec.push(tokenize_integer(&raw_token));

    return token_vec;
}

fn tokenize(current: &String, next: char) -> Token {
    return match current.as_str() {
        "+" | "-" => tokenize_operation(current),
        _ => match next { // This pattern is Number or TBD
            '+' | '-' => tokenize_integer(current),
            _ => Token::TBD
        },
    };
}

fn tokenize_operation(v: &String) -> Token {
    return Token::Operation { val: String::from(v) };
}

fn tokenize_integer(v: &String) -> Token {
    return Token::Integer { val: v.parse().unwrap() };
}
