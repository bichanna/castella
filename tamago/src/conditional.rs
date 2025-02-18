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

//! This module provides ways to create conditional statements like if-else and switch in C.

use std::fmt::{self, Write};

use crate::{Block, Expr, Format, Formatter, Statement};
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct If {
    pub cond: Expr,
    pub then: Block,
    pub other: Option<Block>,
}

impl If {
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            then: Block::new(),
            other: None,
        }
    }

    pub fn new_with_then(cond: Expr, then: Block) -> Self {
        Self {
            cond,
            then,
            other: None,
        }
    }

    pub fn set_then(&mut self, then: Block) -> &mut Self {
        self.then = then;
        self
    }

    pub fn set_other(&mut self, other: Block) -> &mut Self {
        self.other = Some(other);
        self
    }

    pub fn push_to_then(&mut self, stmt: Statement) -> &mut Self {
        self.then.push_statement(stmt);
        self
    }
}

impl Format for If {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "if (")?;
        self.cond.format(fmt)?;
        write!(fmt, ")")?;

        fmt.block(|fmt| self.then.format(fmt))?;

        if let Some(other) = &self.other {
            write!(fmt, " else ")?;
            fmt.block(|fmt| other.format(fmt))?;
        }

        writeln!(fmt)
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Switch {
    pub cond: Expr,
    pub cases: Vec<(Expr, Block)>,
    pub default: Option<Block>,
}

impl Switch {
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            cases: vec![],
            default: None,
        }
    }

    pub fn new_with_cases(cond: Expr, cases: Vec<(Expr, Block)>) -> Self {
        Self {
            cond,
            cases,
            default: None,
        }
    }

    pub fn set_cases(&mut self, cases: Vec<(Expr, Block)>) -> &mut Self {
        self.cases = cases;
        self
    }

    pub fn set_default(&mut self, default: Block) -> &mut Self {
        self.default = Some(default);
        self
    }

    pub fn push_case(&mut self, case: (Expr, Block)) -> &mut Self {
        self.cases.push(case);
        self
    }
}

impl Format for Switch {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "swtich (")?;
        self.cond.format(fmt)?;
        write!(fmt, ") {{")?;

        for (label, block) in &self.cases {
            write!(fmt, "case ")?;
            label.format(fmt)?;
            writeln!(fmt, ":")?;

            fmt.block(|fmt| block.format(fmt))?;
        }

        if let Some(def) = &self.default {
            writeln!(fmt, "default:")?;
            fmt.block(|fmt| def.format(fmt))?;
        }

        writeln!(fmt, "}}")
    }
}
