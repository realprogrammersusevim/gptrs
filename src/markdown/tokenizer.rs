use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Bullet,
    Numbered,
    Bold,
    Italic,
    InlineCode,
    CodeBlock,
    InlineMath,
    MathBlock,
    Text,
    Newline,
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct Tokenizer<'a> {
    pub input: Chars<'a>,
    pub cursor: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tokenizer = Tokenizer {
            input: input.chars(),
            cursor: None,
        };
        tokenizer.advance();
        tokenizer
    }

    pub fn advance(&mut self) {
        self.cursor = self.input.next();
    }

    pub fn peek(&self) -> Option<char> {
        self.input.clone().next()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            match self.cursor {
                Some('_') => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::Italic,
                        value: String::from("_"),
                    });
                }
                Some('*') => {
                    self.advance();
                    if let Some(c) = self.cursor {
                        if c == '*' {
                            self.advance();
                            tokens.push(Token {
                                token_type: TokenType::Bold,
                                value: String::from("**"),
                            });
                        } else {
                            tokens.push(Token {
                                token_type: TokenType::Text,
                                value: String::from("*"),
                            });
                        }
                    }
                }
                Some('`') => {
                    self.advance();
                    match self.cursor {
                        Some('`') => {
                            self.advance();
                            match self.cursor {
                                Some('`') => {
                                    while self.cursor != Some('\n') {
                                        self.advance(); // Consume the language
                                                        // name
                                    }
                                    tokens.push(Token {
                                        token_type: TokenType::CodeBlock,
                                        value: String::from("```"),
                                    });
                                }
                                _ => {
                                    tokens.append(&mut vec![
                                        Token {
                                            token_type: TokenType::InlineCode,
                                            value: String::from("`"),
                                        },
                                        Token {
                                            token_type: TokenType::InlineCode,
                                            value: String::from("`"),
                                        },
                                    ]);
                                }
                            }
                        }
                        _ => {
                            tokens.push(Token {
                                token_type: TokenType::InlineCode,
                                value: String::from("`"),
                            });
                        }
                    }
                }
                Some('$') => {
                    self.advance();
                    match self.cursor {
                        Some('$') => {
                            self.advance();
                            tokens.push(Token {
                                token_type: TokenType::MathBlock,
                                value: String::from("$$"),
                            });
                        }
                        _ => tokens.push(Token {
                            token_type: TokenType::InlineMath,
                            value: String::from("$"),
                        }),
                    }
                }
                Some('\n') => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::Newline,
                        value: String::from("\n"),
                    });
                }
                Some(_) => {
                    let mut text = String::new();
                    while let Some(c) = self.cursor {
                        if c == '*' || c == '`' || c == '$' || c == '_' || c == '\n' {
                            break;
                        }
                        text.push(c);
                        self.advance();
                    }
                    tokens.push(Token {
                        token_type: TokenType::Text,
                        value: text,
                    });
                }
                None => {
                    tokens.push(Token {
                        token_type: TokenType::Eof,
                        value: String::from(""),
                    });
                    break;
                }
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bold() {
        let mut tokenizer = Tokenizer::new("**bold**");
        let expected = vec![
            Token {
                token_type: TokenType::Bold,
                value: String::from("**"),
            },
            Token {
                token_type: TokenType::Text,
                value: String::from("bold"),
            },
            Token {
                token_type: TokenType::Bold,
                value: String::from("**"),
            },
            Token {
                token_type: TokenType::Eof,
                value: String::from(""),
            },
        ];
        assert_eq!(tokenizer.tokenize(), expected);
    }

    #[test]
    fn italic() {
        let mut tokenizer = Tokenizer::new("_italic_");
        let expected = vec![
            Token {
                token_type: TokenType::Italic,
                value: String::from("_"),
            },
            Token {
                token_type: TokenType::Text,
                value: String::from("italic"),
            },
            Token {
                token_type: TokenType::Italic,
                value: String::from("_"),
            },
            Token {
                token_type: TokenType::Eof,
                value: String::from(""),
            },
        ];
        assert_eq!(tokenizer.tokenize(), expected);
    }

    #[test]
    fn inline_code() {
        let mut tokenizer = Tokenizer::new("`inline code`");
        let expected = vec![
            Token {
                token_type: TokenType::InlineCode,
                value: String::from("`"),
            },
            Token {
                token_type: TokenType::Text,
                value: String::from("inline code"),
            },
            Token {
                token_type: TokenType::InlineCode,
                value: String::from("`"),
            },
            Token {
                token_type: TokenType::Eof,
                value: String::from(""),
            },
        ];
        assert_eq!(tokenizer.tokenize(), expected);
    }

    #[test]
    fn codeblock() {
        let mut tokenizer = Tokenizer::new("```\ncodeblock\n```");
        let expected = vec![
            Token {
                token_type: TokenType::CodeBlock,
                value: String::from("```"),
            },
            Token {
                token_type: TokenType::Newline,
                value: String::from("\n"),
            },
            Token {
                token_type: TokenType::Text,
                value: String::from("codeblock"),
            },
            Token {
                token_type: TokenType::Newline,
                value: String::from("\n"),
            },
            Token {
                token_type: TokenType::CodeBlock,
                value: String::from("```"),
            },
            Token {
                token_type: TokenType::Eof,
                value: String::from(""),
            },
        ];
        assert_eq!(tokenizer.tokenize(), expected);
    }

    #[test]
    fn inline_math() {
        let mut tokenizer = Tokenizer::new("$inline math$");
        let expected = vec![
            Token {
                token_type: TokenType::InlineMath,
                value: String::from("$"),
            },
            Token {
                token_type: TokenType::Text,
                value: String::from("inline math"),
            },
            Token {
                token_type: TokenType::InlineMath,
                value: String::from("$"),
            },
            Token {
                token_type: TokenType::Eof,
                value: String::from(""),
            },
        ];
        assert_eq!(tokenizer.tokenize(), expected);
    }

    #[test]
    fn math_block() {
        let mut tokenizer = Tokenizer::new("$$\nmath block\n$$");
        let expected = vec![
            Token {
                token_type: TokenType::MathBlock,
                value: String::from("$$"),
            },
            Token {
                token_type: TokenType::Newline,
                value: String::from("\n"),
            },
            Token {
                token_type: TokenType::Text,
                value: String::from("math block"),
            },
            Token {
                token_type: TokenType::Newline,
                value: String::from("\n"),
            },
            Token {
                token_type: TokenType::MathBlock,
                value: String::from("$$"),
            },
            Token {
                token_type: TokenType::Eof,
                value: String::from(""),
            },
        ];
        assert_eq!(tokenizer.tokenize(), expected);
    }
}
