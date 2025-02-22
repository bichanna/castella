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
    pub fn new(cond: Expr) -> WhileBuilder {
        WhileBuilder::new(cond)
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

pub struct WhileBuilder {
    cond: Expr,
    body: Block,
}

impl WhileBuilder {
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            body: Block::new().build(),
        }
    }

    pub fn body(mut self, body: Block) -> Self {
        self.body = body;
        self
    }

    pub fn statement(mut self, stmt: Statement) -> Self {
        self.body.stmts.push(stmt);
        self
    }

    pub fn build(self) -> Self {
        WhileBuilder {
            cond: self.cond,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct DoWhile {
    pub cond: Expr,
    pub body: Block,
}

impl DoWhile {
    pub fn new(cond: Expr) -> DoWhileBuilder {
        DoWhileBuilder::new(cond)
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

pub struct DoWhileBuilder {
    cond: Expr,
    body: Block,
}

impl DoWhileBuilder {
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            body: Block::new().build(),
        }
    }

    pub fn body(mut self, body: Block) -> Self {
        self.body = body;
        self
    }

    pub fn statement(mut self, stmt: Statement) -> Self {
        self.body.stmts.push(stmt);
        self
    }

    pub fn build(self) -> DoWhileBuilder {
        DoWhileBuilder {
            cond: self.cond,
            body: self.body,
        }
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
    pub fn new() -> ForBuilder {
        ForBuilder::new()
    }
}

pub struct ForBuilder {
    init: Option<Expr>,
    cond: Option<Expr>,
    step: Option<Expr>,
    body: Block,
}

impl ForBuilder {
    pub fn new() -> Self {
        Self {
            init: None,
            cond: None,
            step: None,
            body: Block::new().build(),
        }
    }

    pub fn init(mut self, init: Expr) -> Self {
        self.init = Some(init);
        self
    }

    pub fn cond(mut self, cond: Expr) -> Self {
        self.cond = Some(cond);
        self
    }

    pub fn step(mut self, step: Expr) -> Self {
        self.step = Some(step);
        self
    }

    pub fn body(mut self, body: Block) -> Self {
        self.body = body;
        self
    }

    pub fn statement(mut self, stmt: Statement) -> Self {
        self.body.stmts.push(stmt);
        self
    }

    pub fn build(self) -> ForBuilder {
        ForBuilder {
            init: self.init,
            cond: self.cond,
            step: self.step,
            body: self.body,
        }
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
