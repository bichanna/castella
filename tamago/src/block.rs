// Copyright (c) 2025 Nobuharu Shimazu
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! This module provides ways to create blocks of code, like function bodies, loop bodies,
//! conditional branches.

use std::fmt::{self, Write};

use crate::{
    Comment, DoWhile, ErrorDirective, Expr, For, Format, Formatter, If, IfDefDirective,
    IfDirective, Include, LineDirective, Macro, PragmaDirective, Switch, Variable,
    WarningDirective, While,
};
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Block {
    pub stmts: Vec<Statement>,
}

impl Block {
    pub fn new() -> Self {
        Self { stmts: vec![] }
    }

    pub fn new_with_statements(stmts: Vec<Statement>) -> Self {
        Self { stmts }
    }

    pub fn merge(&mut self, other: &mut Block) -> &mut Self {
        self.stmts.append(&mut other.stmts);
        self
    }

    pub fn push_statement(&mut self, stmt: Statement) -> &mut Self {
        self.stmts.push(stmt);
        self
    }

    pub fn push_new_line(&mut self) -> &mut Self {
        self.stmts.push(Statement::NewLine);
        self
    }
}

impl Format for Block {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for stmt in &self.stmts {
            stmt.format(fmt)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub enum Statement {
    Comment(Comment),
    Variable(Variable),
    Return(Option<Expr>),
    Break,
    Continue,
    GoTo(String),
    Label(String),
    If(If),
    Switch(Switch),
    While(While),
    DoWhile(DoWhile),
    For(For),
    ErrorDirective(ErrorDirective),
    IfDefDirective(IfDefDirective),
    IfDirective(IfDirective),
    Include(Include),
    LineDirective(LineDirective),
    Macro(Macro),
    PragmaDirective(PragmaDirective),
    WarningDirective(WarningDirective),
    Raw(String),
    NewLine,
}

impl Format for Statement {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use Statement::*;
        match self {
            Comment(comment) => comment.format(fmt),
            Variable(variable) => variable.format(fmt),
            Return(None) => writeln!(fmt, "return;"),
            Return(Some(expr)) => {
                write!(fmt, "return ")?;
                expr.format(fmt)?;
                writeln!(fmt, ";")
            }
            Break => writeln!(fmt, "break;"),
            Continue => writeln!(fmt, "continue;"),
            GoTo(s) => writeln!(fmt, "goto {s};"),
            Label(s) => writeln!(fmt, "{s}:"),
            If(i) => i.format(fmt),
            Switch(s) => s.format(fmt),
            While(w) => w.format(fmt),
            DoWhile(w) => w.format(fmt),
            For(f) => f.format(fmt),
            ErrorDirective(e) => e.format(fmt),
            IfDefDirective(i) => i.format(fmt),
            IfDirective(i) => i.format(fmt),
            Include(i) => i.format(fmt),
            LineDirective(l) => l.format(fmt),
            Macro(m) => m.format(fmt),
            PragmaDirective(p) => p.format(fmt),
            WarningDirective(w) => w.format(fmt),
            Raw(s) => writeln!(fmt, "{s}"),
            NewLine => writeln!(fmt),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BaseType, Type};

    #[test]
    fn statement() {
        let mut c = Comment::new();
        c.set_comment_with_str("Hello");
        let mut s = Statement::Comment(c);
        assert_eq!(s.to_string(), "// Hello\n");

        let mut t = Type::new(BaseType::Size);
        t.make_const().make_pointer();
        s = Statement::Variable(Variable::new("abc".to_string(), t));
        assert_eq!(s.to_string(), "const size_t* abc;\n");

        s = Statement::Return(None);
        assert_eq!(s.to_string(), "return;\n");
        s = Statement::Return(Some(Expr::ConstUInt(123)));
        assert_eq!(s.to_string(), "return 123;\n");

        s = Statement::Break;
        assert_eq!(s.to_string(), "break;\n");

        s = Statement::Continue;
        assert_eq!(s.to_string(), "continue;\n");

        s = Statement::GoTo("some_label".to_string());
        assert_eq!(s.to_string(), "goto some_label;\n");

        s = Statement::Label("some_label".to_string());
        assert_eq!(s.to_string(), "some_label:\n");
    }

    #[test]
    fn blocks() {
        let mut b1 = Block::new();
        b1.push_statement(Statement::Raw("abc".to_string()))
            .push_new_line()
            .push_new_line();

        assert_eq!(b1.stmts.len(), 3);
        assert_eq!(b1.to_string(), "abc\n\n\n");

        let mut b2 = Block::new_with_statements(vec![Statement::Raw("something else".to_string())]);
        b2.merge(&mut b1);

        assert_eq!(b2.to_string(), "something else\nabc\n\n\n");
    }
}
