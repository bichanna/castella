use colored::{ColoredString, Colorize};
use logos::Logos;

mod lexer;
mod parser;
mod semantic_analyzer;

type Message = (parser::Span, String);

fn get_line_number(source: &str, span: parser::Span) -> usize {
    source[0..span.start].chars().filter(|&c| c == '\n').count() + 1
}

#[inline]
fn show_message(
    source: &str,
    source_path: &str,
    span: parser::Span,
    pre: ColoredString,
    msg: String,
) {
    eprintln!(
        "{}({}) {}: {}",
        source_path,
        get_line_number(source, span),
        pre,
        msg
    );
}

fn show_errors(source: &str, source_path: &str, errors: Vec<Message>) {
    for e in errors {
        show_message(source, source_path, e.0, "Error".red(), e.1);
    }
}

fn show_warnings(source: &str, source_path: &str, warnings: Vec<Message>) {
    for w in warnings {
        show_message(source, source_path, w.0, "Warning".yellow(), w.1);
    }
}

fn compile(source: &str) -> (Vec<Message>, Result<(), Vec<Message>>) {
    let mut warnings: Vec<Message> = vec![];

    let lexer = lexer::Token::lexer(source);

    let ast: Vec<parser::LocatedGlobalStmt>;
    match parser::Parser::new(lexer).parse() {
        Ok(res) => ast = res,
        Err(errs) => return (warnings, Err(errs)),
    }

    match semantic_analyzer::resolver::Resolver::new(&ast).resolve() {
        (mut w, Ok(())) => warnings.append(&mut w),
        (mut w, Err(errs)) => {
            warnings.append(&mut w);
            return (warnings, Err(errs));
        }
    }

    match semantic_analyzer::type_checker::TypeChecker::new(&ast).check() {
        (mut w, Ok(())) => warnings.append(&mut w),
        (mut w, Err(errs)) => {
            warnings.append(&mut w);
            return (warnings, Err(errs));
        }
    }

    todo!()
}

fn main() {
    // println!("filepath(line) Error: ");

    // let source = "func main(): void { return (1 + 1.0) * 3; }";
    // let source = "func main(): void { age; Person { name = \"Nobu\", age = 18 }; }";
    // let source = "func a(): void {} func main(): void { free a; }";
    // let source = "enum Week { Mon; Tue; Wed; Thur=4; Fri; }";
    let source = "alias Age = int; union Person { name: str; age: Age; }";

    let lexer = lexer::Token::lexer(source);
    let res = parser::Parser::new(lexer).parse();
    if let Err(err) = res {
        show_errors(source, "main.clla", err);
        return;
    }

    let a = semantic_analyzer::resolver::Resolver::new(&res.unwrap()).resolve();

    if let (w, Err(err)) = a {
        show_warnings(source, "main.clla", w);
        show_errors(source, "main.clla", err);
    } else {
        let a = a.0;
        show_warnings(source, "main.clla", a);
    }

    // println!("{:#?}", a);
}
