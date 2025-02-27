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

/// Represents a while loop in C.
///
/// # Examples
/// ```c
/// while (cond) {
///   // body
/// }
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct While {
    /// The condition of the while loop.
    pub cond: Expr,

    /// The body of the while loop.
    pub body: Block,
}

impl While {
    /// Creates and returns a new `WhileBuilder` to construct a `While` using the builder pattern.
    /// ```rust
    /// let while_loop = While::new(/*cond*/)
    ///     .body(/*loop body*/)
    ///     .build();
    /// ```
    pub fn new(cond: Expr) -> WhileBuilder {
        WhileBuilder::new(cond)
    }
}

impl Format for While {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "while (")?;
        self.cond.format(fmt)?;
        write!(fmt, ")")?;

        fmt.block(|fmt| self.body.format(fmt))?;
        writeln!(fmt)
    }
}

/// A builder constructing a `While` instance.
pub struct WhileBuilder {
    cond: Expr,
    body: Block,
}

impl WhileBuilder {
    /// Creates and returns a new `WhileBuilder` to construct a `While` using the builder pattern.
    /// ```rust
    /// let while_loop = WhileBuilder::new(/*cond*/)
    ///     .body(/*loop body*/)
    ///     .build();
    /// ```
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            body: Block::new().build(),
        }
    }

    /// Sets the body block of the while loop being built and returns the builder for more
    /// chaining.
    pub fn body(mut self, body: Block) -> Self {
        self.body = body;
        self
    }

    /// Appends a statement to the body block and returns the builder for chaining more operations.
    pub fn statement(mut self, stmt: Statement) -> Self {
        self.body.stmts.push(stmt);
        self
    }

    /// Consumes the builder and returns a `While` containing the condition expression and the body
    /// block.
    pub fn build(self) -> While {
        While {
            cond: self.cond,
            body: self.body,
        }
    }
}

/// Represents a do-while expression in C.
///
/// # Examples
/// ```c
/// do {
///   // do-while body block
/// } while(cond);
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct DoWhile {
    pub cond: Expr,
    pub body: Block,
}

impl DoWhile {
    /// Creates and returns a new `DoWhileBuilder` to construct a `DoWhile` using the builder
    /// pattern.
    /// ```rust
    /// let do_while = DoWhile::new(/*cond*/)
    ///     .body(/*body block of the do-while loop*/)
    ///     .build();
    /// ```
    pub fn new(cond: Expr) -> DoWhileBuilder {
        DoWhileBuilder::new(cond)
    }
}

impl Format for DoWhile {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "do")?;
        fmt.block(|fmt| self.body.format(fmt))?;

        write!(fmt, " while (")?;
        self.cond.format(fmt)?;
        writeln!(fmt, ");")
    }
}

/// A builder for construcing a `DoWhile` instance.
pub struct DoWhileBuilder {
    cond: Expr,
    body: Block,
}

impl DoWhileBuilder {
    /// Creates and returns a new `DoWhileBuilder` to construct a `DoWhile` using the builder
    /// pattern.
    /// ```rust
    /// let do_while = DoWhileBuilder::new(/*cond*/)
    ///     .body(/*body block of the do-while loop*/)
    ///     .build();
    /// ```
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            body: Block::new().build(),
        }
    }

    /// Sets the body block of the do-while being built and returns the builder for chaining more
    /// operations.
    pub fn body(mut self, body: Block) -> Self {
        self.body = body;
        self
    }

    /// Appends a statement to the body block and returns the builder for chaining more operations.
    pub fn statement(mut self, stmt: Statement) -> Self {
        self.body.stmts.push(stmt);
        self
    }

    /// Consumes the builder and returns a `DoWhile` containg the condition expression and the body
    /// block.
    pub fn build(self) -> DoWhile {
        DoWhile {
            cond: self.cond,
            body: self.body,
        }
    }
}

/// Represents a for loop in C.
///
/// # Examples
/// ```c
/// for (init; cond; step) {
///   // for body block
/// }
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct For {
    /// The initialization part of the for loop.
    pub init: Option<Expr>,

    /// The condition part of the for loop.
    pub cond: Option<Expr>,

    /// The step part of the for loop.
    pub step: Option<Expr>,

    /// The body block of the for loop.
    pub body: Block,
}

impl For {
    /// Creates and returns a new `ForBuilder` to construct a `For` using the builder pattern.
    /// ```rust
    /// let for_loop = For::new()
    ///     .init(/*init part*/)
    ///     .cond(/*cond part*/)
    ///     .step(/*step part*/)
    ///     .body(/*body block of the for loop*/)
    ///     .build();
    /// ```
    pub fn new() -> ForBuilder {
        ForBuilder::new()
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
        write!(fmt, ")")?;

        fmt.block(|fmt| self.body.format(fmt))?;
        writeln!(fmt)
    }
}

/// A builder for construcing a `For` instance.
pub struct ForBuilder {
    init: Option<Expr>,
    cond: Option<Expr>,
    step: Option<Expr>,
    body: Block,
}

impl ForBuilder {
    /// Creates and returns a new `ForBuilder` to construct a `For` using the builder pattern.
    /// ```rust
    /// let for_loop = ForBuilder::new()
    ///     .init(/*init part*/)
    ///     .cond(/*cond part*/)
    ///     .step(/*step part*/)
    ///     .body(/*body block of the for loop*/)
    ///     .build();
    /// ```
    pub fn new() -> Self {
        Self {
            init: None,
            cond: None,
            step: None,
            body: Block::new().build(),
        }
    }

    /// Sets the initialization part of the for loop being built and returns the builder for more
    /// chaining.
    pub fn init(mut self, init: Expr) -> Self {
        self.init = Some(init);
        self
    }

    /// Sets the condition part of the for loop being built and returns the builder for more
    /// chaining.
    pub fn cond(mut self, cond: Expr) -> Self {
        self.cond = Some(cond);
        self
    }

    /// Sets the step part of the for loop being built and returns the builder for chaining more
    /// operations.
    pub fn step(mut self, step: Expr) -> Self {
        self.step = Some(step);
        self
    }

    /// Sets the body block of the for loop being built and returns the builder for chaining more
    /// operations.
    pub fn body(mut self, body: Block) -> Self {
        self.body = body;
        self
    }

    /// Appends a statement to the body block of the for loop being built and returns the builder
    /// for more chaining.
    pub fn statement(mut self, stmt: Statement) -> Self {
        self.body.stmts.push(stmt);
        self
    }

    /// Consumes the builder and returns a `For` containing the optional init, cond, and step
    /// parts, and the body block.
    pub fn build(self) -> For {
        For {
            init: self.init,
            cond: self.cond,
            step: self.step,
            body: self.body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn while_stmt() {
        let w = WhileBuilder::new(Expr::Bool(true))
            .body(
                Block::new()
                    .statement(Statement::Return(Some(Expr::Float(1.23))))
                    .build(),
            )
            .build();
        let res = r#"while (true) {
  return 1.23f;
}
"#;
        assert_eq!(w.to_string(), res);
    }

    #[test]
    fn do_while() {
        let w = DoWhileBuilder::new(Expr::Bool(true)).build();
        let res = "do {\n} while (true);\n";
        assert_eq!(w.to_string(), res);
    }

    #[test]
    fn for_stmt() {
        let f = ForBuilder::new()
            .init(Expr::Variable(Box::new(
                VariableBuilder::new_with_str("i", Type::new(BaseType::Int).build())
                    .value(Expr::Int(0))
                    .build(),
            )))
            .cond(Expr::Binary {
                left: Box::new(Expr::Ident("i".to_string())),
                op: BinOp::LT,
                right: Box::new(Expr::Int(10)),
            })
            .step(Expr::Unary {
                op: UnaryOp::Inc,
                expr: Box::new(Expr::Ident("i".to_string())),
            })
            .body(Block::new().statement(Statement::Continue).build())
            .build();
        let res = r#"for (int i = 0; (i < 10); (i++)) {
  continue;
}
"#;
        assert_eq!(f.to_string(), res);
    }
}
