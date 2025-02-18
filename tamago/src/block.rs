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

use crate::{Comment, Expr, Format, Formatter, Variable};
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
    Raw(String),
    NewLine,
}

impl Format for Statement {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use Statement::*;
        match self {
            Comment(comment) => comment.format(fmt),
            Variable(variable) => {
                variable.format(fmt)?;
                writeln!(fmt, ";")
            }
            Return(None) => writeln!(fmt, "return;"),
            Return(Some(expr)) => {
                write!(fmt, "return ")?;
                expr.format(fmt)?;
                writeln!(fmt)
            }
            Break => writeln!(fmt, "break;"),
            Continue => writeln!(fmt, "continue;"),
            GoTo(s) => writeln!(fmt, "goto {s};"),
            Label(s) => writeln!(fmt, "{s}:"),
            Raw(s) => writeln!(fmt, "{s}"),
            NewLine => writeln!(fmt),
        }
    }
}
