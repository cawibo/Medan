#![feature(exclusive_range_pattern)]

mod syntax;

use lexer::lexer;

fn main() {
    // let tokens = lexer::lex("a := 1; b := 2");
    // print!("{:?}", tokens);
    let mut parser = Parser::new(lex("secs := 22984415;
hours := secs / (60 * 60);
secs := secs - 60 * 60 * hours;

mins := secs / 60;
secs := secs - 60 * mins"));
    // let res = parser.parse_aexpr();
    println!("{:#?}", parser.parse())
}