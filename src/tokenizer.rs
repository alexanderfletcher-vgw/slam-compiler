//tokens contain two parts, the lexem (identifier) and a value (which may be nil)
#[derive(Debug)]
pub struct Token {
    ident: TokenIdent,
    value: TokenValue, // can probably rework this into some or none
}

//accounts for the fact that a tokenvalue may be nil
#[derive(Debug)]
enum TokenIdent {
    Print,
    StringLiteral,
    NumericLiteral,
    Op,
    Var,
    NewLine,
}

#[derive(Debug)]
enum TokenValue {
    Nil,
    Value(String),
}

// should produce an array of tokens
pub fn tokenize(file: String) -> Vec<Token> {
    let token_as_string = convert_to_string_tokens(file);
    let tokens = tokenize_from_string_vec(token_as_string);
    return tokens;
}

fn tokenize_from_string_vec(string_tokens: Vec<String>) -> Vec<Token> {
    let mut tokens = Vec::new();
    for string_token in string_tokens.into_iter() {
        let token = create_token(&string_token);
        tokens.push(token);
    }
    return tokens;
}

fn create_token(string_token: &String) -> Token {
    let ident = determine_token_ident(string_token);
    let value = determine_token_value(&ident, string_token);
    let token = Token { ident, value };
    return token;
}

fn determine_token_value(ident: &TokenIdent, lexeme: &String) -> TokenValue {
    match ident {
        TokenIdent::Print => return TokenValue::Nil, //should probably not be a clone
        TokenIdent::StringLiteral => return TokenValue::Value(lexeme.clone()),
        TokenIdent::NumericLiteral => return TokenValue::Value(lexeme.clone()),
        TokenIdent::Op => return TokenValue::Value(lexeme.clone()),
        TokenIdent::NewLine => return TokenValue::Nil,
        _ => return TokenValue::Value(lexeme.clone()),
    }
}

//assumes all legit token
fn determine_token_ident(lexeme: &String) -> TokenIdent {
    match lexeme.as_ref() {
        "print" => return TokenIdent::Print,
        lexeme if is_string_literal(lexeme) => return TokenIdent::StringLiteral,
        lexeme if is_numeric(lexeme) => return TokenIdent::NumericLiteral,
        "+" => return TokenIdent::Op,
        "=" => return TokenIdent::Op,
        "-" => return TokenIdent::Op,
        "\n" => return TokenIdent::NewLine,
        _ => return TokenIdent::Var,
    }
}

fn convert_to_string_tokens(file: String) -> Vec<String> {
    let chars = file.chars();
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    println!("string to be tokenised \n vvvv \n {} \n ^^^^", file);
    for c in chars {
        if current_token != " " {
            current_token.push(c);
        } else {
            current_token = String::from(c.to_string());
        }
        if is_valid_token(&current_token) {
            tokens.push(current_token.clone());
            current_token = String::new();
        }
    }

    return tokens;
}

fn is_valid_token(lexeme: &String) -> bool {
    //use static in future
    match lexeme.as_ref() {
        "print" => return true,
        lexeme if is_string_literal(lexeme) => return true,
        lexeme if is_numeric(lexeme) => return true,
        "+" => return true,
        "=" => return true,
        "-" => return true,
        "\n" => return true,
        _ => return false,
    }
}

fn is_numeric(lexeme: &str) -> bool {
    let nope = lexeme.parse::<i32>();
    if nope.is_err() {
        return false;
    }
    return true;
}

fn is_string_literal(lexeme: &str) -> bool {
    if lexeme.starts_with("\"") && lexeme.ends_with('\"') && lexeme.char_indices().count() > 1 {
        return true;
    }
    return false;
}
