use std::fmt::Debug;



#[derive(Debug, Clone)]
pub enum NumType {
    FLOAT,
    INT
}




#[derive(Clone)]
pub enum Token {
    NUM(f32, NumType),
    STRING(String),
    NAME(String),
    CCP,
    OCP,
    OP,
    CP,
    OSP,
    CSP,
    EQ,
    EQEQ,
    PLUSEQ,
    MINUSEQ,
    STAREQ,
    SLASHEQ,
    DOT,
    CIRCUMFLEX,
    COMMA,
    COLON,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    LESS,
    LESSEQUAL,
    GREATER,
    GREATEREQUAL,
    NOTEQ,

    PERCENT,
    TILDE,

    AMBERSAND,
    COLLUM,
    NOT,
    AND,
    OR,
    TRUE,
    FALSE,


    LET,
    IF,
    ELSE,
    WHILE,
    BREAK,
    PRINT,
    RETRUN,
    INPUT,
    NONE,
    FUNC,
    FOR,
    IN,
    TO,
    AS,
    STR,
    INT,
    FLOAT
}

pub static mut AST_PRINT: bool = false;


impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            if AST_PRINT {
                return match self {
                    Self::NUM(inner, _) => write!(f, "{}", inner),
                    Self::STRING(inner) => write!(f, "{}", inner),
                    Self::NAME(inner) => write!(f, "{}", inner),
                    other => write!(f, ""),
                };
            }
            else {
                return match self {
                    Self::NUM(inner, inner2) => write!(f, "NUM({}, {:?})", inner, inner2),
                    Self::STRING(inner) => write!(f, "STRING({})", inner),
                    Self::NAME(inner) => write!(f, "NAME({})", inner),
                    other => write!(f, "{}", other.name()),
                };
            }
        }
    }
}

macro_rules! impl_variant_name {
    ($enum:ident, $($variant:ident$(($($field:ident),+))?),*) => {
        impl $enum {
            fn variant_name(&self) -> &str {
                match self {
                    $(
                        $enum::$variant$(($($field),+))? => stringify!($variant),
                    )*
                    _ => panic!("Variant not included in macro call"),
                }
            }
        }
    };
}


impl Token {
    pub fn get_num(&self) -> Option<f32> {
        match self {
            Token::NUM(number, type_of_number) => Some(*number),
            other => None
        }
    }
    pub fn get_num_type(&self) -> Option<NumType> {
        match self {
            Token::NUM(number, type_of_number) => Some(type_of_number.clone()),
            other => None
        }
    }
    pub fn get_string(&self) -> Option<String> {
        match self {
            Token::STRING(string) => Some(string.to_string()),
            other => None
        }
    }
    pub fn get_name(&self) -> Option<String> {
        match self {
            Token::NAME(name) => Some(name.to_string()),
            other => None
        }
    }
    pub fn value(&self) -> String {
        match self {
            Token::NAME(_) => self.get_name().unwrap(),
            Token::STRING(_) => self.get_string().unwrap(),
            Token::NUM(_, _) => self.get_num().unwrap().to_string(),
            other => other.variant_name().to_string()
        }
    }
    pub fn name(&self) -> String {
        match self {
            Token::NAME(_) => "NAME".to_string(),
            Token::STRING(_) => "STRING".to_string(),
            Token::NUM(_, _) => "NUM".to_string(),
            other => other.variant_name().to_string()
        }
    }
}

impl_variant_name!
(
    Token, 
    NUM(f32, NumType),
    STRING(String),
    NAME(String),
    CCP,
    OCP,
    OP,
    CP,
    OSP,
    CSP,
    EQ,
    EQEQ,
    PLUSEQ,
    MINUSEQ,
    STAREQ,
    SLASHEQ,
    DOT,
    CIRCUMFLEX,
    COMMA,
    COLON,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    LESS,
    LESSEQUAL,
    GREATER,
    GREATEREQUAL,
    NOTEQ,

    PERCENT,
    TILDE,

    AMBERSAND,
    COLLUM,
    NOT,
    AND,
    OR,
    TRUE,
    FALSE,


    LET,
    IF,
    ELSE,
    WHILE,
    RETRUN,
    BREAK,
    PRINT,
    INPUT,
    NONE,
    FUNC,
    FOR,
    IN,
    TO,
    AS,
    STR,
    INT,
    FLOAT
);

macro_rules! add_num_to_tokens {
    ($num: expr, $tokens: expr, $index: expr) => {
        if $num != "" {
                    $tokens.push(Token::NUM(
            $num.parse().expect("couldnt parse num"), 
            
            if $num.contains(".") 
            {
                NumType::FLOAT
            } 
            else {
                NumType::INT
            }
        ));

        $num = "".to_string();
        }
    };
}

pub fn lex(line: &mut str) -> Vec<Token> {
    let mut index = 0;

    let mut tokens = vec![];
    
    let mut num = "".to_string();

    let mut chars = line.chars().peekable();

    while let Some(&char) = chars.peek() {
        //println!("char: {} num: {}, line: {}", char, num, line);
        //println!("toknasns: {:?}", tokens);


        if char == ' ' {
            add_num_to_tokens!(num, tokens, index);
            chars.next();
        }
        else if (num.len() == 0 && "-.0123456789".contains(char)) || (num.len() > 0 && ".0123456789".contains(char)) {
            num += &char.to_string();
            chars.next();
        }
        else if char == '"' {
            add_num_to_tokens!(num, tokens, index);
            let mut string = "".to_string();
            index += 1;
            chars.next();

            while let Some(char) = chars.peek() {
                if char != &'"' {
                    string += &char.to_string();
                }
                else {
                    tokens.push(Token::STRING(string));
                    chars.next();
                    break;
                }

                index += 1;
                chars.next();
            }
        }
        else if "_qwertzuiopüasdfghjklöäyxcvbnmQWERTZUIOPÜASDFGHJKLÖÄYXCVBNMß".contains(char) {
            
            add_num_to_tokens!(num, tokens, index);
            let mut name = char.to_string();
            index += 1;
            chars.next();
            
            while let Some(char) = chars.peek() {


                if r"_1234567890qwertzuiopüasdfghjklöäyxcvbnmQWERTZUIOPÜASDFGHJKLÖÄYXCVBNMß".contains(*char) {
                    name += &char.to_string();
                }
                else {
                    index -= 1;
                    break;
                }

                index += 1;
                chars.next();
            }

            match name.as_str() {
                "print" => tokens.push(Token::PRINT),
                "input" => tokens.push(Token::INPUT),
                "let" => tokens.push(Token::LET),
                "if" => tokens.push(Token::IF),
                "while" => tokens.push(Token::WHILE),
                "break" => tokens.push(Token::BREAK),
                "return" => tokens.push(Token::RETRUN),
                "none" => tokens.push(Token::NONE),
                "func" => tokens.push(Token::FUNC),
                "else" => tokens.push(Token::ELSE),
                "for" => tokens.push(Token::FOR),
                "in" => tokens.push(Token::IN),
                "to" => tokens.push(Token::TO),
                "as" => tokens.push(Token::AS),
                "int" => tokens.push(Token::INT),
                "str" => tokens.push(Token::STR),
                "float" => tokens.push(Token::FLOAT),
                "not" => tokens.push(Token::NOT),
                "and" => tokens.push(Token::AND),
                "or" => tokens.push(Token::OR),
                "true" => tokens.push(Token::TRUE),
                "false" => tokens.push(Token::FALSE),
                
                
                name => tokens.push(Token::NAME(name.to_string()))
            }
        }
        else {
            add_num_to_tokens!(num, tokens, index);

            match char {
                '(' => {tokens.push(Token::OP); chars.next();},
                ')' => {tokens.push(Token::CP); chars.next();},
                '[' => {tokens.push(Token::OSP); chars.next();},
                ']' => {tokens.push(Token::CSP); chars.next();},
                ':' => {tokens.push(Token::COLON); chars.next();},
                ',' => {tokens.push(Token::COMMA); chars.next();},
                '+' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::PLUSEQ);
                    } else {
                        tokens.push(Token::PLUS);
                    }
                },
                '!' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::NOTEQ);
                    } else {
                        tokens.push(Token::NOT);
                    }
                },
                '-' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::MINUSEQ);
                    } else {
                        tokens.push(Token::MINUS);
                    }
                },
                '*' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::STAREQ);
                    } else {
                        tokens.push(Token::STAR);
                    }
                },
                '/' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::SLASHEQ);
                    } else {
                        tokens.push(Token::SLASH);
                    }
                },
                '<' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::LESSEQUAL);
                    } else {
                        tokens.push(Token::LESS);
                    }
                },
                '>' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::GREATEREQUAL);
                    } else {
                        tokens.push(Token::GREATER);
                    }
                },
                '=' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::EQEQ);
                    } else {
                        tokens.push(Token::EQ);
                    }
                },
                '&' => {
                    chars.next();
                    if chars.peek() == Some(&'&') {
                        chars.next();
                        tokens.push(Token::AND);
                    } else {
                        tokens.push(Token::AMBERSAND);
                    }
                },
                '|' => {
                    chars.next();
                    if chars.peek() == Some(&'|') {
                        chars.next();
                        tokens.push(Token::OR);
                    } else {
                        tokens.push(Token::COLLUM);
                    }
                },
                '.' => {tokens.push(Token::DOT); chars.next();},
                '%' => {tokens.push(Token::PERCENT); chars.next();},
                '{' => {tokens.push(Token::OCP); chars.next();},
                '}' => {tokens.push(Token::CCP); chars.next();},
                '~' => {tokens.push(Token::TILDE); chars.next();},
                '^' => {tokens.push(Token::CIRCUMFLEX); chars.next();},
                _ => panic!("Unknown character: {}", char),
            }
            
        }
    

        

        index += 1;
    }

    add_num_to_tokens!(num, tokens, index);

    tokens
} 