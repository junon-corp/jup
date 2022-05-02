// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::path::Path;

use jup::parser::Parser;
use jup::checking::syntax::SyntaxChecker;

fn main() -> Result<(), std::io::Error> {
    for i in 1..2 {
        let source: String = format!("junon/test{}.ju", i);

        let mut parser = Parser::from_path(&Path::new(&source))?;
        parser.run();

        // println!("{:?}", parser);
    
        let mut checker = SyntaxChecker::new(&source, parser.parsed());
        checker.run();

        println!("\t<-------------->");
    }

    Ok(())
}
