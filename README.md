<img src="https://raw.githubusercontent.com/junon-corp/jur/main/assets/logo_circle.png" align="right" width="20%" alt="Junon logo" />

# jup
Source code parser for the Junon language to be easily used in community projects

## About
Read the description at first. As the way of the [rust-analyzer](https://github.com/rust-lang/rust-analyzer) project, this crate has to be used for your 
projects to easily parse a Junon source code. You will not have to update your
own parser when the Junon language will change because this crate will be 
updated following the language's changes.

## Usage
To add this crate in your project, you have to open the "Cargo.toml" file and
add the following content :
```toml
[dependencies]
jup = { git = "https://github.com/junon-corp/jup" }
```

- ## Tokenizer
    To "translate" source code to series of tokens (All tokens are defined [here](src/lang/tokens.rs)) :

    From a file path :
    ```rust
    let example = Path::new("example.ju");
    let mut tokenizer = Tokenizer::from_path(example).unwrap();
    tokenizer.run();

    let tokens: Vec<Token> = tokenizer.tokenized();
    ```
    From direct source code :
    ```rust
    let file_content = "...";
    let mut tokenizer = Tokenizer::from_source_code(&file_content);
    tokenizer.run();

    let tokens: Vec<Token> = tokenizer.tokenized();
    ```
- ## Parser
    From the tokenized source code, we can transform these tokens to elements (All elements are defined [here](src/lang/elements/mod.rs)) :
    ```rust
    let mut parser = Parser::new(tokens.clone());
    parser.run();

    let elements = parser.parsed();
    ```
    **tokens** is defined in the above example.

## Tokens
Not all tokens are implemented, you can add your own tokens following the Junon
language and make a pull request to add them. Tokens will be added in the same
time of [juc](https://github.com/junon-corp/juc) progress.
