use crate::{
    error::{LoxError, Report},
    source::Source,
    token::{Token, TokenType},
};

pub fn scan(source: Source) -> Result<Vec<Token>, Report> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut report = Report::new();

    scan_tokens_rec(source, &mut tokens, &mut report);

    if report.is_empty() {
        Ok(tokens)
    } else {
        Err(report)
    }
}

fn scan_tokens_rec(mut source: Source, tokens: &mut Vec<Token>, report: &mut Report) {
    match source.next_char() {
        None => tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: source.flush_lexeme(),
            line: source.get_current_line(),
        }),
        Some(c) => {
            let token_type = match c {
                '(' => Some(TokenType::LeftParen),
                ')' => Some(TokenType::RightParen),
                '{' => Some(TokenType::LeftBrace),
                '}' => Some(TokenType::RightBrace),
                ',' => Some(TokenType::Comma),
                '.' => Some(TokenType::Dot),
                '-' => Some(TokenType::Minus),
                '+' => Some(TokenType::Plus),
                ';' => Some(TokenType::Semicolon),
                '*' => Some(TokenType::Star),
                '!' => match source.maybe_next_char('=') {
                    Some(_) => Some(TokenType::BangEqual),
                    _ => Some(TokenType::Bang),
                },
                '=' => match source.maybe_next_char('=') {
                    Some(_) => Some(TokenType::EqualEqual),
                    _ => Some(TokenType::Equal),
                },
                '<' => match source.maybe_next_char('=') {
                    Some(_) => Some(TokenType::LessEqual),
                    _ => Some(TokenType::Less),
                },
                '>' => match source.maybe_next_char('=') {
                    Some(_) => Some(TokenType::GreaterEqual),
                    _ => Some(TokenType::Greater),
                },
                '/' => match source.maybe_next_char('/') {
                    Some(_) => {
                        source.consume_until('\n');
                        source.flush_lexeme();
                        None
                    }
                    _ => Some(TokenType::Slash),
                },
                '"' => {
                    source.consume_until('"');
                    match source.maybe_next_char('"') {
                        Some(_) => Some(TokenType::String(
                            source.flush_lexeme().trim_matches('"').to_owned(),
                        )),
                        None => {
                            report.push(LoxError {
                                line: source.get_current_line(),
                                message: format!("Unterminated string {}", source.flush_lexeme()),
                            });
                            None
                        }
                    }
                }
                c if c.is_digit(10) => {
                    source.consume_digits();
                    if let Some(_) = source.maybe_next_char('.') {
                        source.consume_digits();
                    };

                    let number_string = source.flush_lexeme();
                    let number: Result<f64, _> = number_string.parse();

                    match number {
                        Ok(number) => Some(TokenType::Number(number)),
                        Err(_) => {
                            report.push(LoxError {
                                message: format!("{} is not a valid number", number_string),
                                line: source.get_current_line(),
                            });
                            None
                        }
                    }
                }
                c if c.is_alphabetic() || c == '_' => {
                    source.consume_alphanumeric();
                    let identifier = source.flush_lexeme();
                    match &identifier[..] {
                        "and" => Some(TokenType::And),
                        "class" => Some(TokenType::Class),
                        "else" => Some(TokenType::Else),
                        "false" => Some(TokenType::False),
                        "for" => Some(TokenType::For),
                        "fun" => Some(TokenType::Fun),
                        "if" => Some(TokenType::If),
                        "nil" => Some(TokenType::Nil),
                        "or" => Some(TokenType::Or),
                        "print" => Some(TokenType::Print),
                        "return" => Some(TokenType::Return),
                        "super" => Some(TokenType::Super),
                        "this" => Some(TokenType::This),
                        "true" => Some(TokenType::True),
                        "var" => Some(TokenType::Var),
                        "while" => Some(TokenType::While),
                        _ => Some(TokenType::Identifier(identifier)),
                    }
                }
                ' ' | '\r' | '\t' | '\n' => {
                    source.flush_lexeme();
                    None
                }
                _ => {
                    report.push(LoxError {
                        message: format!("Unexpected character {}", c),
                        line: source.get_current_line(),
                    });
                    None
                }
            };

            if let Some(token_type) = token_type {
                tokens.push(Token {
                    token_type,
                    lexeme: source.flush_lexeme(),
                    line: source.get_current_line(),
                })
            }

            scan_tokens_rec(source, tokens, report);
        }
    };
}
