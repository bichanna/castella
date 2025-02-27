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

/// Represents a non-global block of code in C, such as the body of a function, loop,
/// conditional, or similar construct. A `Block` contains a sequence of statements
/// that are executed together within the scope of the block.
///
/// # Examples
/// A `Block` might represent the body of a C function like:
/// ```c
/// int ret_integer() {
///     int x = 5;
///     return x;
/// }
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Block {
    /// Holds a bunch of statements that make up the block's content
    pub stmts: Vec<Statement>,
}

impl Block {
    /// Creates and returns a new `BlockBuilder` to construct a `Block` using the builder pattern.
    /// ```rust
    /// let block = Block::new().statement(/*some statement*/).build();
    /// ```
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

/// A builder for constructing a `Block` instance.
pub struct BlockBuilder {
    stmts: Vec<Statement>,
}

impl BlockBuilder {
    /// Creates and returns a new `BlockBuilder` to construct a `Block` using the builder pattern.
    /// ```rust
    /// let block = BlockBuilder::new().statement(/*some statement*/).build();
    /// ```
    pub fn new() -> Self {
        Self { stmts: vec![] }
    }

    /// Appends the provided `Statement` to the block and returns the builder for chaining more
    /// operations.
    pub fn statement(mut self, stmt: Statement) -> Self {
        self.stmts.push(stmt);
        self
    }

    /// Sets the block's statements to the provided statements, replacing any existing ones. This
    /// consumes and returns `BlockBuilder`.
    pub fn statements(mut self, stmts: Vec<Statement>) -> Self {
        self.stmts = stmts;
        self
    }

    /// Appends a `Statement::NewLine` to the block's statements. This consumes and returns
    /// `BlockBuilder` for chaining additional operations.
    pub fn new_line(self) -> Self {
        self.statement(Statement::NewLine)
    }

    /// Merges the statements from another `Block` into this statement list, consuming and
    /// returning `BlockBuilder` for chaining more operations.
    pub fn merge(mut self, mut other: Block) -> Self {
        self.stmts.append(&mut other.stmts);
        self
    }

    /// Consumes the builder and returns a `Block` containing all the statements added during the
    /// building process.
    pub fn build(self) -> Block {
        Block { stmts: self.stmts }
    }
}

/// Encapsulates types of statements and preprocessor directives that can appear within a `Block`,
/// like expressions, control flow, variable declarations, and more.
#[derive(Debug, Clone, DisplayFromFormat)]
pub enum Statement {
    /// A C-style comment
    Comment(Comment),

    /// A variable declaration
    Variable(Variable),

    /// An expression statement.
    Expr(Expr),

    /// A return statement, optionally with an expression
    Return(Option<Expr>),

    /// A break statement
    Break,

    /// A continue statement
    Continue,

    /// A goto statement with a label name
    GoTo(String),

    /// A label declaration
    Label(String),

    /// An if statement
    If(If),

    /// A switch statement
    Switch(Switch),

    /// A while loop
    While(While),

    /// A do-while loop
    DoWhile(DoWhile),

    /// A for loop
    For(For),

    /// A `#error` preprocessor directive
    ErrorDirective(ErrorDirective),

    /// A `#ifdef` preprocessor directive
    IfDefDirective(IfDefDirective),

    /// A `#if` preprocessor directive
    IfDirective(IfDirective),

    /// An `#include` directive
    Include(Include),

    /// A `#line` preprocessor directive
    LineDirective(LineDirective),

    /// A macro definition
    Macro(Macro),

    /// A `#pragma` directive
    PragmaDirective(PragmaDirective),

    /// A `#warning` directive
    WarningDirective(WarningDirective),

    /// A raw string of C code
    Raw(String),

    /// A standalone new line for formatting
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
            Expr(expr) => {
                expr.format(fmt)?;
                writeln!(fmt, ";")
            }
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
        s = Statement::Return(Some(Expr::UInt(123)));
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
            .statement(Statement::Raw("abc;".to_string()))
            .new_line()
            .new_line()
            .statement(Statement::Expr(Expr::FnCall {
                name: Box::new(Expr::new_ident_with_str("some_func")),
                args: vec![],
            }))
            .build();

        assert_eq!(b1.stmts.len(), 4);
        assert_eq!(b1.to_string(), "abc;\n\n\nsome_func();\n");

        let b2 = Block::new()
            .statements(vec![Statement::Raw("something else".to_string())])
            .merge(b1)
            .build();

        assert_eq!(b2.to_string(), "something else\nabc;\n\n\nsome_func();\n");
    }
}
