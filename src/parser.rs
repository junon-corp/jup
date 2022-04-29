// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

//! What about private `Parser::about...()` associated functions ? \
//! They return a boolean value, if the returned value is `true`, instruction
//! `continue` will be called in the run loop

use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use crate::tokens::Token;

/// A way to get a parsed file content as tokens list \
/// Could be called as `Tokenizer`
pub struct Parser<'a> {
    content: &'a str,
    parsed: Vec<Vec<Token>>,

    token: String,
    line: Vec<Token>,

    was_double_char: bool,
    /// The assembly line will be pushed as "this"
    is_asm_code: bool,
    /// Comments are ignored in the parsed vector
    is_comment: bool,

    // Variables for strings creation
    is_string: bool,
    string_content: String,
}

impl<'a> fmt::Debug for Parser<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.parsed {
            if line.is_empty() {
                writeln!(f)?;
            } else {
                writeln!(f, "{:?}", line)?;
            }
        }
        Ok(())
    }
}

impl<'a> Parser<'a> {
    /// Create a new `Parser` object from the content retrieved by reading a
    /// file at the given path \
    /// If the file path is invalid or the file is unreadable, the function
    /// will returns an `io::Error` \
    /// NOTE The path is not checked, it should be valid before giving it as
    /// parameter to this associated function
    pub fn source_code_from_path(file_path: &Path) -> Result<String, io::Error> {
        let source_code = Self::read_file_content(file_path)?;
        Ok(source_code)
    }

    pub fn from_source_code(source_code: &'a str) -> Self {
        Self {
            content: source_code,
            parsed: vec![],

            token: String::new(),
            line: vec![],

            was_double_char: false,
            // The assembly line will be pushed as "this"
            is_asm_code: false,
            // Comments are ignored in the parsed vector
            is_comment: false,

            // Variables for strings creation
            is_string: false,
            string_content: String::new(),
        }
    }

    /// Parsed content is not returned in this function, SEE `::parsed()`
    pub fn run(&mut self) {
        for (i, c) in self.content.chars().enumerate() {
            // Comments will be everytime skipped
            if c != '\n' && self.is_comment {
                continue;
            }

            // SEE This file's documentation
            if
                self.about_strings(c) ||
                self.about_asm(c) ||
                self.about_new_lines(c) ||
                self.about_others(c, i)
            {
                continue;
            }
        }

        // Push the last token and the last line
        if !self.token.is_empty() {
            self.line.push(Token::from_string(&self.token));
        }
        if !self.line.is_empty() {
            self.parsed.push(self.line.clone());
        }
    }

    /// Return an immutable 2D vector of the tokenized source code
    pub fn parsed(&self) -> &Vec<Vec<Token>> {
        &self.parsed
    }

    /// For `Self::from_path()`
    fn read_file_content(file_path: &Path) -> Result<String, io::Error> {
        let mut source_code = String::new();

        let mut stream = File::open(file_path)?;
        stream.read_to_string(&mut source_code)?;

        Ok(source_code)
    }

    fn about_strings(&mut self, c: char) -> bool {
        if c == Token::StringDot.to_string().chars().next().unwrap() {
            if self.is_string { // end of string
                self.is_string = false;

                self.line.push(
                    Token::from_string(
                        &format!(
                            "{}{}{}",
                            Token::StringDot.to_string(),
                            self.string_content,
                            Token::StringDot.to_string()
                        )
                    )
                );

                // Reset the string for the next
                self.string_content = String::new();
            } else {
                self.is_string = true;
            }

            return true;
        }

        if self.is_string { // string creation
            self.string_content.push(c);

            // Don't care of the other possibilities, we want raw characters in
            // the string
            return true;
        }

        false
    }

    fn about_asm(&mut self, c: char) -> bool {
        if c == '@' {
            self.token = "@".to_string();
            self.push_token();
            self.is_asm_code = true;

            return true;
        }

        false
    }

    fn about_new_lines(&mut self, c: char) -> bool {
        if c == '\n' {
            self.push_token(); // push the line's last token

            // Push the new line into `self.parsed`
            if !self.line.is_empty() {
                self.parsed.push(self.line.clone());
                self.line = vec![]; // reset line
            }

            // Resets
            self.is_asm_code = false;
            self.is_comment = false;

            return true;
        }

        false
    }

    fn about_others(&mut self, c: char, i: usize) -> bool {
        if !c.is_alphanumeric() { // should be cut
            self.push_token(); // finish the current token...

            // ... to create another one with the character
            if c != ' ' && !self.was_double_char {
                if
                    i != self.content.len() - 1 &&
                    c == self.content.chars().nth(i + 1).unwrap()
                {
                    let double_char_as_token = Token::from_string(
                        &format!("{}{}", c, c)
                    );
                    if double_char_as_token == Token::Comment {
                        self.is_comment = true;
                        return true;
                    }
                    self.line.push(double_char_as_token);

                    self.was_double_char = true;
                    return true;
                }

                if self.is_asm_code {
                    let token_string = format!("{}", c);
                    if Token::from_string(&token_string) == Token::Comma {
                        self.line.push(Token::Comma);
                    } else {
                        self.line.push(Token::Other(format!("{}", c)));
                    }
                } else {
                    self.line.push(Token::from_string(&format!("{}", c)));
                }
            }
            self.was_double_char = false;
            return true;
        }

        self.token.push(c); // it's still the same token
        false
    }

    fn push_token(&mut self) {
        if self.token.is_empty() { // useless if void
            return;
        }

        if self.is_asm_code {
            self.line.push(Token::Other(self.token.clone()));
        } else {
            self.line.push(Token::from_string(&self.token.clone()))
        }

        self.token = String::new(); // reset for the next
    }
}

#[test]
fn from_file() {
    let file_path = Path::new("tests/test1.ju");
    let source = Parser::source_code_from_path(file_path).unwrap();
    let mut parser = Parser::from_source_code(&source);
    parser.run();

    println!("{:?}", parser);
}

#[test]
fn from_source_code() {
    let source_code = "func main {\n".to_owned() +
        "    ret ok\n" + "}\n // annoying comment";

    let mut parser = Parser::from_source_code(&source_code);
    parser.run();

    println!("{:?}", parser);
}
