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

//! This module provides structured representations and builders for C-style loop constructs.
//! It includes implementations for:
//!
//! - `While` loops - Conditional loops that check the condition before each iteration
//! - `DoWhile` loops - Conditional loops that check the condition after each iteration
//! - `For` loops - Iteration loops with initialization, condition, and increment expressions
//!
//! Each loop type follows a builder pattern for flexible and readable construction.
//! All loop constructs implement the `Format` trait for generating properly formatted C code.

use std::fmt::{self, Write};

use crate::{Block, Expr, Format, Formatter, Statement};
use tamacro::DisplayFromFormat;

/// Represents a standard while loop in C.
///
/// A while loop evaluates its condition before each iteration of the loop body.
/// If the condition evaluates to false before the first iteration, the loop body
/// will not execute at all.
///
/// ## C Syntax
/// ```c
/// while (condition) {
///     // loop body statements
/// }
/// ```
///
/// ## Example Usage
/// ```rust
/// // Create a while loop that executes while x < 10
/// let while_loop = While::new(expr!("x < 10"))
///     .body(Block::new()
///         .statement(Statement::expr(expr!("x++")))
///         .build())
///     .build();
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct While {
    /// The condition expression that determines whether the loop continues.
    /// The loop executes as long as this condition evaluates to true.
    pub cond: Expr,

    /// The body block containing statements to be executed in each iteration.
    pub body: Block,
}

impl While {
    /// Creates a new `WhileBuilder` with the specified condition.
    ///
    /// This is the starting point for constructing a while loop using the builder pattern.
    /// After calling this method, chain additional builder methods and finally call `build()`
    /// to create the while loop.
    ///
    /// ## Parameters
    /// - `cond`: The condition expression that controls the loop execution
    ///
    /// ## Returns
    /// A new `WhileBuilder` instance with the given condition
    ///
    /// ## Example
    /// ```rust
    /// let while_loop = While::new(expr!("count < 10"))
    ///     .body(loop_body)
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

/// A builder for constructing a `While` loop instance.
///
/// This builder implements the builder pattern for creating while loops
/// with a fluent interface. It allows for clear and concise loop creation
/// with method chaining.
pub struct WhileBuilder {
    cond: Expr,
    body: Block,
}

impl WhileBuilder {
    /// Creates a new `WhileBuilder` with the specified condition expression.
    ///
    /// ## Parameters
    /// - `cond`: The condition expression that will control the loop execution
    ///
    /// ## Returns
    /// A new `WhileBuilder` instance with the given condition and an empty body
    ///
    /// ## Example
    /// ```rust
    /// let builder = WhileBuilder::new(expr!("i < array_size"));
    /// ```
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            body: Block::new().build(),
        }
    }

    /// Sets the body block of the while loop.
    ///
    /// ## Parameters
    /// - `body`: A complete `Block` instance containing the statements to execute in the loop
    ///
    /// ## Returns
    /// The builder instance for method chaining
    ///
    /// ## Example
    /// ```rust
    /// let body_block = Block::new()
    ///     .statement(Statement::expr(expr!("process_item(array[i])")))
    ///     .statement(Statement::expr(expr!("i++")))
    ///     .build();
    ///
    /// let builder = WhileBuilder::new(expr!("i < array_size"))
    ///     .body(body_block);
    /// ```
    pub fn body(mut self, body: Block) -> Self {
        self.body = body;
        self
    }

    /// Appends a single statement to the body block of the while loop.
    ///
    /// This is a convenience method for adding one statement at a time
    /// to the loop body instead of creating a complete `Block` first.
    ///
    /// ## Parameters
    /// - `stmt`: The statement to add to the loop body
    ///
    /// ## Returns
    /// The builder instance for method chaining
    ///
    /// ## Example
    /// ```rust
    /// let builder = WhileBuilder::new(expr!("i < 10"))
    ///     .statement(Statement::expr(expr!("sum += array[i]")))
    ///     .statement(Statement::expr(expr!("i++")));
    /// ```
    pub fn statement(mut self, stmt: Statement) -> Self {
        self.body.stmts.push(stmt);
        self
    }

    /// Consumes the builder and creates a `While` instance.
    ///
    /// This finalizes the building process and returns the complete while loop.
    ///
    /// ## Returns
    /// A fully constructed `While` loop instance
    ///
    /// ## Example
    /// ```rust
    /// let while_loop = WhileBuilder::new(expr!("data_available()"))
    ///     .statement(Statement::expr(expr!("process_data()")))
    ///     .build();
    /// ```
    pub fn build(self) -> While {
        While {
            cond: self.cond,
            body: self.body,
        }
    }
}

/// Represents a do-while loop in C.
///
/// A do-while loop evaluates its condition after each iteration of the loop body.
/// This guarantees that the loop body executes at least once, regardless of the
/// condition's initial value.
///
/// ## C Syntax
/// ```c
/// do {
///     // loop body statements
/// } while(condition);
/// ```
///
/// ## Example Usage
/// ```rust
/// // Create a do-while loop that executes at least once
/// let do_while = DoWhile::new(expr!("response != 'q'"))
///     .body(Block::new()
///         .statement(Statement::expr(expr!("display_menu()")))
///         .statement(Statement::expr(expr!("response = get_input()")))
///         .build())
///     .build();
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct DoWhile {
    /// The condition expression that determines whether the loop continues.
    /// The loop executes as long as this condition evaluates to true,
    /// with the condition checked after each iteration.
    pub cond: Expr,

    /// The body block containing statements to be executed in each iteration.
    /// This body is guaranteed to execute at least once.
    pub body: Block,
}

impl DoWhile {
    /// Creates a new `DoWhileBuilder` with the specified condition.
    ///
    /// This is the starting point for constructing a do-while loop using the builder pattern.
    /// After calling this method, chain additional builder methods and finally call `build()`
    /// to create the do-while loop.
    ///
    /// ## Parameters
    /// - `cond`: The condition expression that controls the loop continuation
    ///
    /// ## Returns
    /// A new `DoWhileBuilder` instance with the given condition
    ///
    /// ## Example
    /// ```rust
    /// let do_while = DoWhile::new(expr!("data != EOF"))
    ///     .body(loop_body)
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

/// A builder for constructing a `DoWhile` loop instance.
///
/// This builder implements the builder pattern for creating do-while loops
/// with a fluent interface. It allows for clear and concise loop creation
/// with method chaining.
pub struct DoWhileBuilder {
    cond: Expr,
    body: Block,
}

impl DoWhileBuilder {
    /// Creates a new `DoWhileBuilder` with the specified condition expression.
    ///
    /// ## Parameters
    /// - `cond`: The condition expression that will control the loop continuation
    ///
    /// ## Returns
    /// A new `DoWhileBuilder` instance with the given condition and an empty body
    ///
    /// ## Example
    /// ```rust
    /// let builder = DoWhileBuilder::new(expr!("valid_input == false"));
    /// ```
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            body: Block::new().build(),
        }
    }

    /// Sets the body block of the do-while loop.
    ///
    /// ## Parameters
    /// - `body`: A complete `Block` instance containing the statements to execute in the loop
    ///
    /// ## Returns
    /// The builder instance for method chaining
    ///
    /// ## Example
    /// ```rust
    /// let body_block = Block::new()
    ///     .statement(Statement::expr(expr!("display_prompt()")))
    ///     .statement(Statement::expr(expr!("input = read_user_input()")))
    ///     .statement(Statement::expr(expr!("valid_input = validate_input(input)")))
    ///     .build();
    ///
    /// let builder = DoWhileBuilder::new(expr!("valid_input == false"))
    ///     .body(body_block);
    /// ```
    pub fn body(mut self, body: Block) -> Self {
        self.body = body;
        self
    }

    /// Appends a single statement to the body block of the do-while loop.
    ///
    /// This is a convenience method for adding one statement at a time
    /// to the loop body instead of creating a complete `Block` first.
    ///
    /// ## Parameters
    /// - `stmt`: The statement to add to the loop body
    ///
    /// ## Returns
    /// The builder instance for method chaining
    ///
    /// ## Example
    /// ```rust
    /// let builder = DoWhileBuilder::new(expr!("has_more_input()"))
    ///     .statement(Statement::expr(expr!("value = read_next_value()")))
    ///     .statement(Statement::expr(expr!("process_value(value)")));
    /// ```
    pub fn statement(mut self, stmt: Statement) -> Self {
        self.body.stmts.push(stmt);
        self
    }

    /// Consumes the builder and creates a `DoWhile` instance.
    ///
    /// This finalizes the building process and returns the complete do-while loop.
    ///
    /// ## Returns
    /// A fully constructed `DoWhile` loop instance
    ///
    /// ## Example
    /// ```rust
    /// let do_while_loop = DoWhileBuilder::new(expr!("!is_valid(input)"))
    ///     .statement(Statement::expr(expr!("prompt_user()")))
    ///     .statement(Statement::expr(expr!("input = get_input()")))
    ///     .build();
    /// ```
    pub fn build(self) -> DoWhile {
        DoWhile {
            cond: self.cond,
            body: self.body,
        }
    }
}

/// Represents a for loop in C, which consists of initialization, condition, step expressions,
/// and a body block of statements.
///
/// A for loop provides a compact way to iterate with a counter or other form of state
/// that is initialized before the loop, checked before each iteration, and updated after
/// each iteration.
///
/// ## C Syntax
/// ```c
/// for (initialization; condition; step) {
///     // loop body statements
/// }
/// ```
///
/// Each of the three expressions (initialization, condition, step) is optional.
///
/// ## Example Usage
/// ```rust
/// // Create a for loop that counts from 0 to 9
/// let for_loop = For::new()
///     .init(expr!("int i = 0"))
///     .cond(expr!("i < 10"))
///     .step(expr!("i++"))
///     .body(Block::new()
///         .statement(Statement::expr(expr!("process(array[i])")))
///         .build())
///     .build();
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct For {
    /// The initialization expression that runs once before the loop begins.
    /// This is typically used to declare and initialize loop variables or counters.
    /// Can be `None` if no initialization is needed.
    pub init: Option<Expr>,

    /// The condition expression that determines whether the loop continues.
    /// Evaluated before each iteration. The loop executes as long as this
    /// condition evaluates to true. Can be `None` for an infinite loop.
    pub cond: Option<Expr>,

    /// The step (or update) expression that runs after each iteration.
    /// This is typically used to increment counters or update loop variables.
    /// Can be `None` if no update is needed.
    pub step: Option<Expr>,

    /// The body block containing statements to be executed in each iteration.
    pub body: Block,
}

impl For {
    /// Creates a new `ForBuilder` for constructing a for loop.
    ///
    /// This is the starting point for constructing a for loop using the builder pattern.
    /// After calling this method, chain additional builder methods to set the initialization,
    /// condition, step expressions, and body, then finally call `build()` to create the for loop.
    ///
    /// ## Returns
    /// A new `ForBuilder` instance with all parts set to `None` and an empty body
    ///
    /// ## Example
    /// ```rust
    /// let for_loop = For::new()
    ///     .init(expr!("int i = 0"))
    ///     .cond(expr!("i < array_size"))
    ///     .step(expr!("i++"))
    ///     .body(loop_body)
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

/// A builder for constructing a `For` loop instance.
///
/// This builder implements the builder pattern for creating for loops
/// with a fluent interface. It allows for clear and concise loop creation
/// with method chaining, and supports optional components (initialization,
/// condition, and step expressions).
pub struct ForBuilder {
    init: Option<Expr>,
    cond: Option<Expr>,
    step: Option<Expr>,
    body: Block,
}

impl ForBuilder {
    /// Creates a new `ForBuilder` with all parts initialized to `None` or empty.
    ///
    /// ## Returns
    /// A new `ForBuilder` instance
    ///
    /// ## Example
    /// ```rust
    /// let builder = ForBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            init: None,
            cond: None,
            step: None,
            body: Block::new().build(),
        }
    }

    /// Sets the initialization expression of the for loop.
    ///
    /// This expression is executed once before the loop begins, and is typically
    /// used to declare and initialize loop variables or counters.
    ///
    /// ## Parameters
    /// - `init`: The initialization expression
    ///
    /// ## Returns
    /// The builder instance for method chaining
    ///
    /// ## Example
    /// ```rust
    /// let builder = ForBuilder::new()
    ///     .init(expr!("int i = 0"));
    /// ```
    pub fn init(mut self, init: Expr) -> Self {
        self.init = Some(init);
        self
    }

    /// Sets the condition expression of the for loop.
    ///
    /// This expression is evaluated before each iteration of the loop.
    /// The loop continues as long as this condition evaluates to true.
    ///
    /// ## Parameters
    /// - `cond`: The condition expression
    ///
    /// ## Returns
    /// The builder instance for method chaining
    ///
    /// ## Example
    /// ```rust
    /// let builder = ForBuilder::new()
    ///     .cond(expr!("i < array_length"));
    /// ```
    pub fn cond(mut self, cond: Expr) -> Self {
        self.cond = Some(cond);
        self
    }

    /// Sets the step (or update) expression of the for loop.
    ///
    /// This expression is executed after each iteration of the loop,
    /// and is typically used to increment counters or update loop variables.
    ///
    /// ## Parameters
    /// - `step`: The step expression
    ///
    /// ## Returns
    /// The builder instance for method chaining
    ///
    /// ## Example
    /// ```rust
    /// let builder = ForBuilder::new()
    ///     .step(expr!("i++"));
    /// ```
    pub fn step(mut self, step: Expr) -> Self {
        self.step = Some(step);
        self
    }

    /// Sets the body block of the for loop.
    ///
    /// ## Parameters
    /// - `body`: A complete `Block` instance containing the statements to execute in the loop
    ///
    /// ## Returns
    /// The builder instance for method chaining
    ///
    /// ## Example
    /// ```rust
    /// let body_block = Block::new()
    ///     .statement(Statement::expr(expr!("sum += array[i]")))
    ///     .build();
    ///
    /// let builder = ForBuilder::new()
    ///     .body(body_block);
    /// ```
    pub fn body(mut self, body: Block) -> Self {
        self.body = body;
        self
    }

    /// Appends a single statement to the body block of the for loop.
    ///
    /// This is a convenience method for adding one statement at a time
    /// to the loop body instead of creating a complete `Block` first.
    ///
    /// ## Parameters
    /// - `stmt`: The statement to add to the loop body
    ///
    /// ## Returns
    /// The builder instance for method chaining
    ///
    /// ## Example
    /// ```rust
    /// let builder = ForBuilder::new()
    ///     .init(expr!("int i = 0"))
    ///     .cond(expr!("i < 10"))
    ///     .step(expr!("i++"))
    ///     .statement(Statement::expr(expr!("print_element(array[i])")));
    /// ```
    pub fn statement(mut self, stmt: Statement) -> Self {
        self.body.stmts.push(stmt);
        self
    }

    /// Consumes the builder and creates a `For` instance.
    ///
    /// This finalizes the building process and returns the complete for loop.
    ///
    /// ## Returns
    /// A fully constructed `For` loop instance
    ///
    /// ## Example
    /// ```rust
    /// let for_loop = ForBuilder::new()
    ///     .init(expr!("int i = 0"))
    ///     .cond(expr!("i < size"))
    ///     .step(expr!("i++"))
    ///     .statement(Statement::expr(expr!("process(data[i])")))
    ///     .build();
    /// ```
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
