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

## Tokens
Not all tokens are implemented, you can add your own tokens following the Junon
language and make a pull request to add them. Tokens will be added in the same
time of [juc](https://github.com/junon-corp/juc) progress.
