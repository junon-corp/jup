// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

//! What about private `Tokenizer::about...()` associated functions ? \
//! They return a boolean value, if the returned value is `true`, instruction
//! `continue` will be called in the run loop

use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use crate::lang::tokens::Token;

/// A way to get a tokenized file content as tokens list \
/// Could be called as `Tokenizer`
pub struct Tokenizer {
    content: String,
    tokenized: Vec<Token>,

    /// Current token as string
    token: String,

    was_double_char: bool,
    /// The assembly line will be pushed as "this"
    is_asm_code: bool,
    /// Comments are ignored in the tokenized vector
    is_comment: bool,

    // Variables for strings creation
    is_string: bool,
    string_content: String,
}

impl fmt::Debug for Tokenizer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "---- ")?;

        for token in &self.tokenized {
            if *token == Token::NewLine {
                writeln!(f, "{:?}", Token::NewLine)?;
                continue;
            }
            write!(f, "{:?} ", token)?;
        }

        writeln!(f, "---- ")?;

        Ok(())
    }
}

impl Tokenizer {
    /// Creates a new `Tokenizer` object from the content retrieved by reading a
    /// file at the given path
    ///
    /// If the file path is invalid or the file is unreadable, the function
    /// will returns an `io::Error`
    ///
    /// Note : The path is not checked, it should be valid before giving it as
    /// parameter to this associated function
    pub fn from_path(file_path: &Path) -> Result<Self, io::Error> {
        let source_code = Self::read_file_content(file_path)?;
        Ok(Self::from_source_code(&source_code))
    }

    /// Creates a new `Tokenizer` object from the given source code content
    pub fn from_source_code(source_code: &str) -> Self {
        Self {
            content: source_code.to_owned(),
            tokenized: vec![],

            token: String::new(),

            was_double_char: false,
            // The assembly line will be pushed as "this"
            is_asm_code: false,
            // Comments are ignored in the tokenized vector
            is_comment: false,

            // Variables for strings creation
            is_string: false,
            string_content: String::new(),
        }
    }

    /// Parsed content is not returned by this function but by `tokenized()`
    pub fn run(&mut self) {
        // Replaces all the tabulations to space character, to simplify the
        // operations and avoid creating matches for tabulations when spaces 
        // matches already exist
        self.content = self.content.replace("\t", " ");

        for (i, c) in self.content.clone().chars().enumerate() {
            // Comments will be everytime skipped
            if c != '\n' && self.is_comment {
                continue;
            }

            // SEE This file's documentation
            if self.about_new_lines(c)
                || self.about_asm(c)
                || self.about_strings(c)
                || self.about_others(c, i)
            {
                continue;
            }
        }

        // Push the last token
        if self.token != String::new() {
            self.push_token();
        }

        // Always put a "NewLine" token at the end if not here
        if self.token != Token::NewLine.to_string() {
            self.token = Token::NewLine.to_string();
            self.push_token();
        }

        // Because the clone of `self.tokenized` is enumerated, and not the real
        // `self.tokenized`, when `self.tokenized` is updated, the index of `i`
        // becomes wrong by `1`. The `j` value permits to shift to the right 
        // index
        let mut j = 0;
        
        let mut previous_token = &Token::None;

        for (i, token) in self.tokenized.clone().iter().enumerate() {
            // Because `<=` and `>=` aren't double characters, they are found
            // separated in the tokenized vector. Here, one of them is found,
            // the both characters are replaced by the right token           
            if (previous_token == &Token::MoreThan 
                || previous_token == &Token::LessThan) 
                && token == &Token::Assign 
            {
                // No needs for retrieving the old value but a warning is thrown
                // when the value is not retrieved 
                let _ = std::mem::replace(
                    &mut self.tokenized[i - j - 1],             
                    if previous_token == &Token::MoreThan { 
                        Token::MoreThanOrEqual 
                    } else { // means == `Token::LessThan`
                        Token::LessThanOrEqual 
                    }
                );
                   
                self.tokenized.remove(i - j);
                j += 1;
            }
            
            previous_token = token;
        }
    }

    /// Returns an immutable 2D vector of the tokenized source code
    pub fn tokenized(&self) -> &Vec<Token> {
        &self.tokenized
    }

    /// For `Self::from_path()`
    fn read_file_content(file_path: &Path) -> Result<String, io::Error> {
        let mut source_code = String::new();

        let mut stream = File::open(file_path)?;
        stream.read_to_string(&mut source_code)?;

        Ok(source_code)
    }

    fn about_new_lines(&mut self, c: char) -> bool {
        if c == '\n' {
            self.push_token(); // push the line's last token

            // By this way, comments are ignored but `Token::NewLine` is pushed
            // Don't forget it's important to know there is a line here to count
            // lines
            self.tokenized.push(Token::NewLine);

            // Resets
            self.is_asm_code = false;
            self.is_comment = false;

            return true;
        }

        false
    }

    fn about_asm(&mut self, c: char) -> bool {
        if self.is_asm_code {
            self.token += &format!("{}", c);
            return true;
        }

        if c == '@' {
            self.token = "@".to_string();
            self.push_token();
            self.is_asm_code = true;

            return true;
        }

        false
    }

    fn about_strings(&mut self, c: char) -> bool {
        if c == Token::StringDot.to_string().chars().next().unwrap() {
            if self.is_string {
                // end of string
                self.is_string = false;

                self.tokenized.push(Token::from_string(&format!(
                    "{}{}{}",
                    Token::StringDot.to_string(),
                    self.string_content,
                    Token::StringDot.to_string()
                )));

                // Reset the string for the next
                self.string_content = String::new();
            } else {
                self.is_string = true;
            }

            return true;
        }

        if self.is_string {
            // string creation
            self.string_content.push(c);

            // Don't care of the other possibilities, we want raw characters in
            // the string
            return true;
        }

        false
    }

    fn about_others(&mut self, c: char, i: usize) -> bool {
        if !c.is_alphanumeric() && c != '_' {
            // should be cut
            self.push_token(); // finish the current token...

            // ... to create another one with the character
            if c != ' ' && !self.was_double_char {
                if i != self.content.len() - 1 && c == self.content.chars().nth(i + 1).unwrap() {
                    let double_char_as_token = Token::from_string(&format!("{}{}", c, c));
                    if double_char_as_token == Token::Comment {
                        self.is_comment = true;
                        return true;
                    }
                    self.tokenized.push(double_char_as_token);

                    self.was_double_char = true;
                    return true;
                }

                self.tokenized.push(Token::from_string(&format!("{}", c)));
            }
            self.was_double_char = false;
            return true;
        }

        self.token.push(c); // it's still the same token
        false
    }

    fn push_token(&mut self) {
        if *self.token == String::new() {
            // useless if void
            return;
        }

        self.tokenized.push(Token::from_string(&self.token.clone()));
        self.token = String::new(); // reset for the next
    }
}

#[test]
fn from_file() {
    let file_path = Path::new("tests/test1.ju");
    let mut tokenizer = Tokenizer::from_path(file_path).unwrap();
    tokenizer.run();

    println!("{:?}", tokenizer);
}

#[test]
fn from_source_code() {
    let source_code = 
        "func main {\n".to_owned() + 
        "\tlet a = 5;" +
        "    ret\n" + 
        "}\n // annoying comment";

    let mut tokenizer = Tokenizer::from_source_code(&source_code);
    tokenizer.run();

    println!("{:?}", tokenizer);
}
