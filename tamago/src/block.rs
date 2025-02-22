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
    pub fn new() -> BlockBuilder {
        BlockBuilder::new()
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

pub struct BlockBuilder {
    stmts: Vec<Statement>,
}

impl BlockBuilder {
    pub fn new() -> Self {
        Self { stmts: vec![] }
    }

    pub fn statement(mut self, stmt: Statement) -> Self {
        self.stmts.push(stmt);
        self
    }

    pub fn statements(mut self, stmts: Vec<Statement>) -> Self {
        self.stmts = stmts;
        self
    }

    pub fn new_line(self) -> Self {
        self.statement(Statement::NewLine)
    }

    pub fn merge(mut self, mut other: Block) -> Self {
        self.stmts.append(&mut other.stmts);
        self
    }

    pub fn build(self) -> Block {
        Block { stmts: self.stmts }
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
    use crate::*;

    #[test]
    fn statement() {
        let mut s = Statement::Comment(Comment::new().comment_with_str("Hello").build());
        assert_eq!(s.to_string(), "// Hello\n");

        let t = Type::new(BaseType::Size)
            .make_const()
            .make_pointer()
            .build();
        s = Statement::Variable(VariableBuilder::new_with_str("abc", t).build());
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
        let b1 = Block::new()
            .statement(Statement::Raw("abc".to_string()))
            .new_line()
            .new_line()
            .build();

        assert_eq!(b1.stmts.len(), 3);
        assert_eq!(b1.to_string(), "abc\n\n\n");

        let b2 = Block::new()
            .statements(vec![Statement::Raw("something else".to_string())])
            .merge(b1)
            .build();

        assert_eq!(b2.to_string(), "something else\nabc\n\n\n");
    }
}
