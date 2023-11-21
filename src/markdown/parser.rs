use crate::markdown::tokenizer::{Token, TokenType};
use ratatui::style::{Modifier, Style};
use ratatui::text::Text;
use std::slice::Iter;

pub struct Parser<'a> {
    pub tokens: Iter<'a, Token>,
    pub current: Option<&'a Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens: tokens.iter(),
            current: None,
        };
        parser.advance();
        parser
    }

    pub fn advance(&mut self) {
        self.current = self.tokens.next();
    }

    pub fn parse(&mut self) -> Text {
        let mut text = Text::default();
        let bold = Style::default().add_modifier(Modifier::BOLD);
        let italic = Style::default().add_modifier(Modifier::ITALIC);
        let default = Style::default();
        loop {
            match self.current {
                Some(Token {
                    value: _,
                    token_type: TokenType::Bold,
                }) => {
                    self.handle_token(&TokenType::Bold, bold, bold, &mut text);
                }
                Some(Token {
                    value: _,
                    token_type: TokenType::Italic,
                }) => {
                    self.handle_token(&TokenType::Italic, italic, italic, &mut text);
                }
                Some(Token {
                    value: _,
                    token_type: TokenType::InlineCode,
                }) => {
                    self.handle_token(&TokenType::InlineCode, bold, default, &mut text);
                }
                Some(Token {
                    value: _,
                    token_type: TokenType::CodeBlock,
                }) => {
                    self.handle_token(&TokenType::CodeBlock, bold, default, &mut text);
                }
                Some(Token {
                    value: _,
                    token_type: TokenType::InlineMath,
                }) => {
                    self.handle_token(&TokenType::InlineMath, bold, default, &mut text);
                }
                Some(Token {
                    value: _,
                    token_type: TokenType::MathBlock,
                }) => {
                    self.handle_token(&TokenType::MathBlock, bold, default, &mut text);
                }
                Some(Token {
                    value: val,
                    token_type: TokenType::Text | TokenType::Newline,
                }) => {
                    text.extend(Text::styled(val.clone(), default));
                }
                Some(Token {
                    value: _,
                    token_type: TokenType::Eof,
                }) => break,
                _ => {}
            }
        }
        text
    }

    fn handle_token(
        &mut self,
        end_token_type: &TokenType,
        text_style: Style,
        marker_style: Style,
        text: &mut Text,
    ) {
        text.extend(Text::styled(
            self.current.unwrap().value.clone(),
            marker_style,
        ));
        loop {
            self.advance();
            match self.current {
                Some(Token {
                    token_type: end_type,
                    value: val,
                }) if end_type == end_token_type => {
                    text.extend(Text::styled(val.clone(), marker_style));
                    break;
                }
                Some(Token {
                    token_type: _,
                    value: val,
                }) => text.extend(Text::styled(val.clone(), text_style)),
                None => break,
            }
        }
    }
}
