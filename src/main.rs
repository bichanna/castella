use logos::Logos;

mod lexer;
mod parser;

fn main() {
    // println!("filepath(line) Error: ");

    // let source = "func main(): void { (1 + 1.0) * 3; }";
    let source = "func main(): void { ^age = &age; }";

    let lexer = lexer::Token::lexer(source);
    let res = parser::Parser::new(source, "abcl", lexer).parse();
    println!("{:?}", res);
}
