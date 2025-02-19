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
    Comment, DocComment, Enum, ErrorDirective, Format, Formatter, Function, IfDefDirective,
    IfDirective, Include, LineDirective, Macro, PragmaDirective, Struct, TypeDef, Union, Variable,
    WarningDirective,
};
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Scope {
    pub doc: Option<DocComment>,
    pub global_stmts: Vec<GlobalStatement>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            doc: None,
            global_stmts: vec![],
        }
    }

    pub fn new_with_global_statements(global_stmts: Vec<GlobalStatement>) -> Self {
        Self {
            doc: None,
            global_stmts,
        }
    }

    pub fn set_doc(&mut self, doc: DocComment) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    pub fn set_global_statements(&mut self, global_stmts: Vec<GlobalStatement>) -> &mut Self {
        self.global_stmts = global_stmts;
        self
    }

    pub fn push_global_statement(&mut self, global_stmt: GlobalStatement) -> &mut Self {
        self.global_stmts.push(global_stmt);
        self
    }

    pub fn push_new_line(&mut self) -> &mut Self {
        self.push_global_statement(GlobalStatement::NewLine);
        self
    }
}

impl Format for Scope {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        for stmt in &self.global_stmts {
            stmt.format(fmt)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub enum GlobalStatement {
    Comment(Comment),
    Enum(Enum),
    Struct(Struct),
    Function(Function),
    Union(Union),
    Variable(Variable),
    TypeDef(TypeDef),
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

impl Format for GlobalStatement {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use GlobalStatement::*;
        match self {
            Comment(c) => c.format(fmt),
            Enum(e) => e.format(fmt),
            Struct(s) => s.format(fmt),
            Function(f) => f.format(fmt),
            Union(u) => u.format(fmt),
            Variable(v) => v.format(fmt),
            TypeDef(t) => t.format(fmt),
            ErrorDirective(e) => e.format(fmt),
            IfDefDirective(i) => i.format(fmt),
            IfDirective(i) => i.format(fmt),
            Include(i) => i.format(fmt),
            LineDirective(l) => l.format(fmt),
            Macro(m) => m.format(fmt),
            PragmaDirective(p) => p.format(fmt),
            WarningDirective(w) => w.format(fmt),
            Raw(r) => writeln!(fmt, "{r}"),
            NewLine => writeln!(fmt),
        }
    }
}
