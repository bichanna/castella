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

//! This module provides means to create C functions.

use std::fmt::{self, Write};

use crate::{Block, DocComment, Format, Formatter, Statement, Type};
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub ret: Type,
    pub is_inline: bool,
    pub is_static: bool,
    pub is_extern: bool,
    pub body: Block,
    pub doc: Option<DocComment>,
}

impl Function {
    pub fn new(name: String, ret: Type) -> Self {
        Self {
            name,
            ret,
            params: vec![],
            is_inline: false,
            is_static: false,
            is_extern: false,
            body: Block::new(),
            doc: None,
        }
    }

    pub fn set_doc(&mut self, doc: DocComment) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    pub fn set_inline(&mut self) -> &mut Self {
        self.is_inline = true;
        self
    }

    pub fn set_static(&mut self) -> &mut Self {
        self.is_static = true;
        self
    }

    pub fn set_extern(&mut self) -> &mut Self {
        self.is_extern = true;
        self
    }

    pub fn set_body(&mut self, body: Block) -> &mut Self {
        self.body = body;
        self
    }

    pub fn push_statement(&mut self, stmt: Statement) -> &mut Self {
        self.body.push_statement(stmt);
        self
    }

    pub fn push_new_line(&mut self) -> &mut Self {
        self.body.push_new_line();
        self
    }
}

impl Format for Function {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        if self.body.stmts.is_empty() && self.is_extern {
            write!(fmt, "extern ")?;
        }

        if self.is_static {
            write!(fmt, "static ")?;
        }

        if self.is_inline {
            write!(fmt, "inline ")?;
        }

        self.ret.format(fmt)?;

        write!(fmt, "{}(", self.name)?;
        if self.params.is_empty() {
            write!(fmt, "void")?;
        } else {
            for param in &self.params[..self.params.len() - 1] {
                param.format(fmt)?;
                write!(fmt, ", ")?;
            }

            if let Some(last) = self.params.last() {
                last.format(fmt)?;
            }
        }

        write!(fmt, ")")?;

        if !self.body.stmts.is_empty() && !self.is_extern {
            fmt.block(|fmt| self.body.format(fmt))?;
            writeln!(fmt)
        } else {
            writeln!(fmt, ";")
        }
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Parameter {
    pub name: String,
    pub t: Type,
    pub doc: Option<DocComment>,
}

impl Parameter {
    pub fn new(name: String, t: Type) -> Self {
        Self { name, t, doc: None }
    }

    pub fn set_doc(&mut self, doc: DocComment) -> &mut Self {
        self.doc = Some(doc);
        self
    }
}

impl Format for Parameter {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.t.format(fmt)?;

        write!(fmt, " {}", self.name)?;

        if self.t.is_array() {
            write!(fmt, "[{}]", self.t.array)?;
        }

        Ok(())
    }
}
