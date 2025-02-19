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

//! This module provides ways to create loop constructs like for, while, and do-while loops.

use std::fmt::{self, Write};

use crate::{Block, Expr, Format, Formatter, Statement};
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct While {
    pub cond: Expr,
    pub body: Block,
}

impl While {
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            body: Block::new(),
        }
    }

    pub fn new_with_body(cond: Expr, body: Block) -> Self {
        Self { cond, body }
    }

    pub fn set_body(&mut self, body: Block) -> &mut Self {
        self.body = body;
        self
    }

    pub fn push_statement(&mut self, stmt: Statement) -> &mut Self {
        self.body.push_statement(stmt);
        self
    }
}

impl Format for While {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "while (")?;
        self.cond.format(fmt)?;
        write!(fmt, ") ")?;

        fmt.block(|fmt| self.body.format(fmt))
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct DoWhile {
    pub cond: Expr,
    pub body: Block,
}

impl DoWhile {
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            body: Block::new(),
        }
    }

    pub fn new_with_body(cond: Expr, body: Block) -> Self {
        Self { cond, body }
    }

    pub fn set_body(&mut self, body: Block) -> &mut Self {
        self.body = body;
        self
    }

    pub fn push_statement(&mut self, stmt: Statement) -> &mut Self {
        self.body.push_statement(stmt);
        self
    }
}

impl Format for DoWhile {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "do ")?;
        fmt.block(|fmt| self.body.format(fmt))?;

        write!(fmt, " while (")?;
        self.cond.format(fmt)?;
        writeln!(fmt, ");")
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct For {
    pub init: Option<Expr>,
    pub cond: Option<Expr>,
    pub step: Option<Expr>,
    pub body: Block,
}

impl For {
    pub fn new() -> Self {
        Self {
            init: None,
            cond: None,
            step: None,
            body: Block::new(),
        }
    }

    pub fn set_init(&mut self, init: Expr) -> &mut Self {
        self.init = Some(init);
        self
    }

    pub fn set_cond(&mut self, cond: Expr) -> &mut Self {
        self.cond = Some(cond);
        self
    }

    pub fn set_step(&mut self, step: Expr) -> &mut Self {
        self.step = Some(step);
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
}

impl Format for For {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "for (")?;
        if let Some(init) = &self.init {
            init.format(fmt)?;
        }
        write!(fmt, ";")?;

        if let Some(cond) = &self.cond {
            write!(fmt, " ")?;
            cond.format(fmt)?;
        }
        write!(fmt, ";")?;

        if let Some(step) = &self.step {
            write!(fmt, " ")?;
            step.format(fmt)?;
        }
        write!(fmt, ") ")?;

        fmt.block(|fmt| self.body.format(fmt))
    }
}
