use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    True,
    False,
    Integer(i64),
    String(String),
    Regex(String, String),
    Eq,
    NotEq,
    Match,
    NotMatch,
    And,
    Or,
    Not,
    Contains,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Dot,
    Comma,
    Ident(String),
    Eof,
}

const KNOWN_REFS: &[&str] = &[
    "build.branch",
    "build.tag",
    "build.message",
    "build.state",
    "build.source",
    "build.creator.name",
    "build.creator.email",
    "build.creator.teams",
    "build.pull_request.id",
    "build.pull_request.draft",
    "build.pull_request.base_branch",
    "build.pull_request.repository",
    "build.pull_request.labels",
    "build.env",
    "build.number",
    "build.id",
    "pipeline.default_branch",
    "pipeline.repository",
    "pipeline.slug",
    "pipeline.id",
];

const KNOWN_FUNCTIONS: &[&str] = &["env", "meta-data"];

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            position: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.position += 1;
        self.input.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self, quote: char) -> Result<String, String> {
        self.advance();
        let mut s = String::new();
        loop {
            match self.advance() {
                Some(c) if c == quote => break,
                Some('\\') => {
                    if let Some(escaped) = self.advance() {
                        match escaped {
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            'r' => s.push('\r'),
                            '\\' => s.push('\\'),
                            c if c == quote => s.push(c),
                            other => {
                                s.push('\\');
                                s.push(other);
                            }
                        }
                    }
                }
                Some(c) => s.push(c),
                None => return Err(format!("Unterminated string starting at position {}", self.position)),
            }
        }
        Ok(s)
    }

    fn read_regex(&mut self) -> Result<(String, String), String> {
        self.advance();
        let mut pattern = String::new();
        let mut escaped = false;
        loop {
            match self.advance() {
                Some('/') if !escaped => break,
                Some('\\') if !escaped => {
                    escaped = true;
                    pattern.push('\\');
                }
                Some(c) => {
                    escaped = false;
                    pattern.push(c);
                }
                None => return Err("Unterminated regex".to_string()),
            }
        }
        let mut flags = String::new();
        while let Some(&c) = self.peek() {
            if c == 'i' || c == 'm' || c == 's' || c == 'x' {
                flags.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Ok((pattern, flags))
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        while let Some(&c) = self.peek() {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    fn read_number(&mut self) -> i64 {
        let mut s = String::new();
        while let Some(&c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s.parse().unwrap_or(0)
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        match self.peek() {
            None => Ok(Token::Eof),
            Some(&c) => match c {
                '(' => {
                    self.advance();
                    Ok(Token::LParen)
                }
                ')' => {
                    self.advance();
                    Ok(Token::RParen)
                }
                '[' => {
                    self.advance();
                    Ok(Token::LBracket)
                }
                ']' => {
                    self.advance();
                    Ok(Token::RBracket)
                }
                '.' => {
                    self.advance();
                    Ok(Token::Dot)
                }
                ',' => {
                    self.advance();
                    Ok(Token::Comma)
                }
                '"' | '\'' => {
                    let s = self.read_string(c)?;
                    Ok(Token::String(s))
                }
                '/' => {
                    let (pattern, flags) = self.read_regex()?;
                    Ok(Token::Regex(pattern, flags))
                }
                '=' => {
                    self.advance();
                    match self.peek() {
                        Some(&'=') => {
                            self.advance();
                            Ok(Token::Eq)
                        }
                        Some(&'~') => {
                            self.advance();
                            Ok(Token::Match)
                        }
                        _ => Err("Expected '==' or '=~'".to_string()),
                    }
                }
                '!' => {
                    self.advance();
                    match self.peek() {
                        Some(&'=') => {
                            self.advance();
                            Ok(Token::NotEq)
                        }
                        Some(&'~') => {
                            self.advance();
                            Ok(Token::NotMatch)
                        }
                        _ => Ok(Token::Not),
                    }
                }
                '&' => {
                    self.advance();
                    if self.peek() == Some(&'&') {
                        self.advance();
                        Ok(Token::And)
                    } else {
                        Err("Expected '&&'".to_string())
                    }
                }
                '|' => {
                    self.advance();
                    if self.peek() == Some(&'|') {
                        self.advance();
                        Ok(Token::Or)
                    } else {
                        Err("Expected '||'".to_string())
                    }
                }
                '@' => {
                    self.advance();
                    if self.peek() == Some(&'>') {
                        self.advance();
                        Ok(Token::Contains)
                    } else {
                        Err("Expected '@>'".to_string())
                    }
                }
                c if c.is_ascii_digit() => {
                    let n = self.read_number();
                    Ok(Token::Integer(n))
                }
                c if c.is_alphabetic() || c == '_' => {
                    let ident = self.read_ident();
                    match ident.as_str() {
                        "true" => Ok(Token::True),
                        "false" => Ok(Token::False),
                        _ => Ok(Token::Ident(ident)),
                    }
                }
                other => Err(format!("Unexpected character: '{}'", other)),
            },
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, String> {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token()?;
        Ok(Parser {
            lexer,
            current,
            errors: Vec::new(),
        })
    }

    fn advance(&mut self) -> Result<(), String> {
        self.current = self.lexer.next_token()?;
        Ok(())
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if &self.current == expected {
            self.advance()?;
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.current))
        }
    }

    pub fn parse(&mut self) -> Result<(), Vec<String>> {
        if let Err(e) = self.parse_or_expr() {
            self.errors.push(e);
        }
        if self.current != Token::Eof {
            self.errors.push(format!("Unexpected token after expression: {:?}", self.current));
        }
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn parse_or_expr(&mut self) -> Result<(), String> {
        self.parse_and_expr()?;
        while self.current == Token::Or {
            self.advance()?;
            self.parse_and_expr()?;
        }
        Ok(())
    }

    fn parse_and_expr(&mut self) -> Result<(), String> {
        self.parse_comparison()?;
        while self.current == Token::And {
            self.advance()?;
            self.parse_comparison()?;
        }
        Ok(())
    }

    fn parse_comparison(&mut self) -> Result<(), String> {
        self.parse_unary()?;
        match &self.current {
            Token::Eq | Token::NotEq | Token::Match | Token::NotMatch | Token::Contains => {
                let op = self.current.clone();
                self.advance()?;
                self.parse_unary()?;
                if matches!(op, Token::Match | Token::NotMatch) {
                    // Already validated regex during lexing
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_unary(&mut self) -> Result<(), String> {
        if self.current == Token::Not {
            self.advance()?;
            self.parse_unary()?;
            return Ok(());
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<(), String> {
        match &self.current {
            Token::True | Token::False | Token::Integer(_) | Token::String(_) | Token::Regex(_, _) => {
                if let Token::Regex(pattern, flags) = &self.current {
                    let pattern = pattern.clone();
                    let flags = flags.clone();
                    self.validate_regex(&pattern, &flags)?;
                }
                self.advance()?;
                Ok(())
            }
            Token::LParen => {
                self.advance()?;
                self.parse_or_expr()?;
                self.expect(&Token::RParen)
            }
            Token::LBracket => {
                self.advance()?;
                if self.current != Token::RBracket {
                    self.parse_or_expr()?;
                    while self.current == Token::Comma {
                        self.advance()?;
                        self.parse_or_expr()?;
                    }
                }
                self.expect(&Token::RBracket)
            }
            Token::Ident(name) => {
                let name = name.clone();
                self.advance()?;
                if self.current == Token::LParen {
                    self.validate_function(&name)?;
                    self.advance()?;
                    if self.current != Token::RParen {
                        self.parse_or_expr()?;
                        while self.current == Token::Comma {
                            self.advance()?;
                            self.parse_or_expr()?;
                        }
                    }
                    self.expect(&Token::RParen)
                } else {
                    let mut ref_path = name;
                    while self.current == Token::Dot {
                        self.advance()?;
                        if let Token::Ident(part) = &self.current {
                            ref_path.push('.');
                            ref_path.push_str(part);
                            self.advance()?;
                        } else {
                            return Err("Expected identifier after '.'".to_string());
                        }
                    }
                    self.validate_reference(&ref_path)?;
                    Ok(())
                }
            }
            Token::Eof => Err("Unexpected end of expression".to_string()),
            other => Err(format!("Unexpected token: {:?}", other)),
        }
    }

    fn validate_regex(&mut self, pattern: &str, flags: &str) -> Result<(), String> {
        let full_pattern = if flags.contains('i') {
            format!("(?i){}", pattern)
        } else {
            pattern.to_string()
        };
        match regex::Regex::new(&full_pattern) {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = format!("Invalid regex /{}/{}: {}", pattern, flags, e);
                self.errors.push(msg.clone());
                Err(msg)
            }
        }
    }

    fn validate_function(&mut self, name: &str) -> Result<(), String> {
        if !KNOWN_FUNCTIONS.contains(&name) {
            let msg = format!(
                "Unknown function '{}'. Known functions: {}",
                name,
                KNOWN_FUNCTIONS.join(", ")
            );
            self.errors.push(msg);
        }
        Ok(())
    }

    fn validate_reference(&mut self, ref_path: &str) -> Result<(), String> {
        let is_known = KNOWN_REFS.iter().any(|r| {
            ref_path == *r || ref_path.starts_with(&format!("{}.", r))
        });
        if !is_known {
            let root = ref_path.split('.').next().unwrap_or("");
            if root != "build" && root != "pipeline" {
                let msg = format!(
                    "Unknown reference '{}'. References must start with 'build.' or 'pipeline.'",
                    ref_path
                );
                self.errors.push(msg);
            }
        }
        Ok(())
    }
}

pub fn validate_condition(expr: &str) -> Result<(), Vec<String>> {
    let mut parser = Parser::new(expr).map_err(|e| vec![e])?;
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_equality() {
        assert!(validate_condition("build.branch == 'main'").is_ok());
        assert!(validate_condition("build.branch == \"main\"").is_ok());
    }

    #[test]
    fn test_regex_match() {
        assert!(validate_condition("build.branch =~ /^feature\\//").is_ok());
        assert!(validate_condition("build.branch !~ /^(deploy|release)/").is_ok());
    }

    #[test]
    fn test_logical_operators() {
        assert!(validate_condition("build.branch == 'main' && build.state == 'passed'").is_ok());
        assert!(validate_condition("build.branch == 'main' || build.branch == 'develop'").is_ok());
    }

    #[test]
    fn test_negation() {
        assert!(validate_condition("!build.pull_request.draft").is_ok());
    }

    #[test]
    fn test_functions() {
        assert!(validate_condition("env('CI') == 'true'").is_ok());
        assert!(validate_condition("meta-data('version') != ''").is_ok());
    }

    #[test]
    fn test_parentheses() {
        assert!(validate_condition("(build.branch == 'main' || build.branch == 'develop') && build.state == 'passed'").is_ok());
    }

    #[test]
    fn test_array_contains() {
        assert!(validate_condition("['main', 'develop'] @> build.branch").is_ok());
    }

    #[test]
    fn test_invalid_regex() {
        assert!(validate_condition("build.branch =~ /[/").is_err());
    }

    #[test]
    fn test_unknown_reference() {
        let result = validate_condition("unknown.field == 'x'");
        assert!(result.is_err());
    }

    #[test]
    fn test_incomplete_expression() {
        assert!(validate_condition("build.branch ==").is_err());
    }
}
