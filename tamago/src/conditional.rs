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

//! # Conditional Statements Module
//!
//! This module provides abstractions and builders for creating C-style conditional statements
//! such as `if-else` and `switch-case` constructs. These abstractions allow for programmatic
//! generation of C code with proper formatting and indentation.
//!
//! The module primarily offers two main structures:
//! - `If`: Represents a C if-statement with optional else clause
//! - `Switch`: Represents a C switch-statement with multiple cases and an optional default case
//!
//! Both structures come with corresponding builder patterns to facilitate their construction.

use std::fmt::{self, Write};

use crate::{Block, Expr, Format, Formatter, Statement};
use tamacro::DisplayFromFormat;

/// Represents an if statement in C programming language.
///
/// This structure models the complete if-else construct, containing a condition expression,
/// a "then" block for code executed when the condition is true, and an optional "else" block
/// for when the condition is false.
///
/// # Examples
///
/// Basic if statement:
/// ```rust
/// let if_stmt = If::new(Expr::new_binary(
///     Expr::new_ident_with_str("x"),
///     BinOp::Gt,
///     Expr::Int(5)
/// ))
/// .then(
///     Block::new()
///         .statement(Statement::Return(Some(Expr::Int(1))))
///         .build()
/// )
/// .build();
/// ```
///
/// This would generate C code like:
/// ```c
/// if (x > 5) {
///   return 1;
/// }
/// ```
///
/// With an else clause:
/// ```rust
/// let if_stmt = If::new(Expr::new_binary(
///     Expr::new_ident_with_str("x"),
///     BinOp::Gt,
///     Expr::Int(5)
/// ))
/// .then(
///     Block::new()
///         .statement(Statement::Return(Some(Expr::Int(1))))
///         .build()
/// )
/// .other(
///     Block::new()
///         .statement(Statement::Return(Some(Expr::Int(0))))
///         .build()
/// )
/// .build();
/// ```
///
/// This would generate C code like:
/// ```c
/// if (x > 5) {
///   return 1;
/// } else {
///   return 0;
/// }
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct If {
    /// The condition expression that determines which branch to execute
    pub cond: Expr,

    /// The block of code to execute when the condition evaluates to true
    pub then: Block,

    /// Optional block of code to execute when the condition evaluates to false
    pub other: Option<Block>,
}

impl If {
    /// Creates and returns a new `IfBuilder` to construct an `If` statement using the builder pattern.
    ///
    /// This method provides a convenient entry point to start building an if statement.
    /// The returned builder allows for fluent and clear construction of if statements.
    ///
    /// # Parameters
    /// - `cond`: The condition expression for the if statement
    ///
    /// # Returns
    /// A new `IfBuilder` instance initialized with the given condition
    ///
    /// # Examples
    /// ```rust
    /// let if_stmt = If::new(Expr::Bool(true))
    ///     .then(Block::new().statement(Statement::Return(None)).build())
    ///     .other(Block::new().statement(Statement::Break).build())
    ///     .build();
    /// ```
    pub fn new(cond: Expr) -> IfBuilder {
        IfBuilder::new(cond)
    }
}

impl Format for If {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "if (")?;
        self.cond.format(fmt)?;
        write!(fmt, ")")?;

        fmt.block(|fmt| self.then.format(fmt))?;

        if let Some(other) = &self.other {
            write!(fmt, " else")?;
            fmt.block(|fmt| other.format(fmt))?;
        }

        writeln!(fmt)
    }
}

/// A builder for constructing an `If` statement instance incrementally.
///
/// This builder provides a fluent API for creating if statements with optional
/// else blocks. It follows the builder pattern to make the construction process
/// more readable and easier to maintain.
pub struct IfBuilder {
    cond: Expr,
    then: Block,
    other: Option<Block>,
}

impl IfBuilder {
    /// Creates and returns a new `IfBuilder` to construct an `If` statement.
    ///
    /// This method initializes a builder with just the condition expression,
    /// with an empty "then" block and no "else" block.
    ///
    /// # Parameters
    /// - `cond`: The condition expression for the if statement
    ///
    /// # Returns
    /// A new `IfBuilder` instance initialized with the given condition
    ///
    /// # Examples
    /// ```rust
    /// let builder = IfBuilder::new(Expr::Bool(true));
    /// // Further configure the builder...
    /// let if_stmt = builder
    ///     .then(Block::new().statement(Statement::Return(None)).build())
    ///     .build();
    /// ```
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            then: Block::new().build(),
            other: None,
        }
    }

    /// Creates and returns a new `IfBuilder` with a predefined condition and "then" block.
    ///
    /// This convenience constructor initializes a builder with both the condition expression
    /// and the "then" block, allowing for quicker initialization when both elements are known.
    ///
    /// # Parameters
    /// - `cond`: The condition expression for the if statement
    /// - `then`: The block of code to execute when the condition is true
    ///
    /// # Returns
    /// A new `IfBuilder` instance initialized with the given condition and "then" block
    ///
    /// # Examples
    /// ```rust
    /// let then_block = Block::new().statement(Statement::Return(None)).build();
    /// let builder = IfBuilder::new_with_then(Expr::Bool(true), then_block);
    /// // Further configure the builder if needed...
    /// let if_stmt = builder.build();
    /// ```
    pub fn new_with_then(cond: Expr, then: Block) -> Self {
        Self {
            cond,
            then,
            other: None,
        }
    }

    /// Sets the "then" block for the if statement.
    ///
    /// This method specifies the block of code to execute when the condition evaluates to true.
    ///
    /// # Parameters
    /// - `then`: The block of code to execute when the condition is true
    ///
    /// # Returns
    /// `self` for method chaining
    ///
    /// # Examples
    /// ```rust
    /// let if_stmt = IfBuilder::new(Expr::Bool(true))
    ///     .then(Block::new().statement(Statement::Return(None)).build())
    ///     .build();
    /// ```
    pub fn then(mut self, then: Block) -> Self {
        self.then = then;
        self
    }

    /// Sets the "else" block for the if statement.
    ///
    /// This method specifies the optional block of code to execute when the condition evaluates to false.
    ///
    /// # Parameters
    /// - `other`: The block of code to execute when the condition is false
    ///
    /// # Returns
    /// `self` for method chaining
    ///
    /// # Examples
    /// ```rust
    /// let if_stmt = IfBuilder::new(Expr::Bool(true))
    ///     .then(Block::new().statement(Statement::Return(Some(Expr::Int(1)))).build())
    ///     .other(Block::new().statement(Statement::Return(Some(Expr::Int(0)))).build())
    ///     .build();
    /// ```
    pub fn other(mut self, other: Block) -> Self {
        self.other = Some(other);
        self
    }

    /// Appends a statement to the "then" block of the if statement.
    ///
    /// This convenience method adds a statement directly to the "then" block without
    /// requiring explicit block creation.
    ///
    /// # Parameters
    /// - `stmt`: The statement to add to the "then" block
    ///
    /// # Returns
    /// `self` for method chaining
    ///
    /// # Examples
    /// ```rust
    /// let if_stmt = IfBuilder::new(Expr::Bool(true))
    ///     .statement_to_then(Statement::Return(None))
    ///     .statement_to_then(Statement::Break)
    ///     .build();
    /// ```
    pub fn statement_to_then(mut self, stmt: Statement) -> Self {
        self.then.stmts.push(stmt);
        self
    }

    /// Finalizes the building process and returns the constructed `If` statement.
    ///
    /// This method consumes the builder and produces an `If` instance with all
    /// the properties configured during the building process.
    ///
    /// # Returns
    /// A new `If` instance with the configured properties
    ///
    /// # Examples
    /// ```rust
    /// let if_stmt = IfBuilder::new(Expr::Bool(true))
    ///     .then(Block::new().statement(Statement::Return(None)).build())
    ///     .build();
    /// ```
    pub fn build(self) -> If {
        If {
            cond: self.cond,
            then: self.then,
            other: self.other,
        }
    }
}

/// Represents a switch statement in C programming language.
///
/// This structure models the complete switch-case construct, containing a condition expression,
/// multiple case blocks with their respective case expressions, and an optional default case.
///
/// # Examples
///
/// Basic switch statement:
/// ```rust
/// let switch_stmt = Switch::new(Expr::new_ident_with_str("x"))
///     .case(
///         Expr::Int(1),
///         Block::new()
///             .statement(Statement::Return(Some(Expr::Int(100))))
///             .build()
///     )
///     .case(
///         Expr::Int(2),
///         Block::new()
///             .statement(Statement::Return(Some(Expr::Int(200))))
///             .build()
///     )
///     .default(
///         Block::new()
///             .statement(Statement::Return(Some(Expr::Int(0))))
///             .build()
///     )
///     .build();
/// ```
///
/// This would generate C code like:
/// ```c
/// switch (x) {
/// case 1: {
///   return 100;
/// }
/// case 2: {
///   return 200;
/// }
/// default: {
///   return 0;
/// }
/// }
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Switch {
    /// The expression to switch on
    pub cond: Expr,

    /// Vector of case expressions and their corresponding code blocks
    pub cases: Vec<(Expr, Block)>,

    /// Optional default block for when no cases match
    pub default: Option<Block>,
}

impl Switch {
    /// Creates and returns a new `SwitchBuilder` to construct a `Switch` statement using the builder pattern.
    ///
    /// This method provides a convenient entry point to start building a switch statement.
    /// The returned builder allows for fluent and clear construction of switch statements.
    ///
    /// # Parameters
    /// - `cond`: The expression to switch on
    ///
    /// # Returns
    /// A new `SwitchBuilder` instance initialized with the given condition
    ///
    /// # Examples
    /// ```rust
    /// let switch_stmt = Switch::new(Expr::new_ident_with_str("status"))
    ///     .case(Expr::Int(0), Block::new().statement(Statement::Return(None)).build())
    ///     .case(Expr::Int(1), Block::new().statement(Statement::Break).build())
    ///     .default(Block::new().statement(Statement::Return(Some(Expr::Int(-1)))).build())
    ///     .build();
    /// ```
    pub fn new(cond: Expr) -> SwitchBuilder {
        SwitchBuilder::new(cond)
    }
}

impl Format for Switch {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "switch (")?;
        self.cond.format(fmt)?;
        writeln!(fmt, ") {{")?;

        for (label, block) in &self.cases {
            write!(fmt, "case ")?;
            label.format(fmt)?;
            write!(fmt, ":")?;

            fmt.block(|fmt| block.format(fmt))?;
            writeln!(fmt)?;
        }

        if let Some(def) = &self.default {
            write!(fmt, "default:")?;
            fmt.block(|fmt| def.format(fmt))?;
            writeln!(fmt)?;
        }

        writeln!(fmt, "}}")
    }
}

/// A builder for constructing a `Switch` statement instance incrementally.
///
/// This builder provides an API for creating switch statements with multiple
/// case blocks and an optional default block. It follows the builder pattern to make
/// the construction process more readable and easier to maintain.
pub struct SwitchBuilder {
    cond: Expr,
    cases: Vec<(Expr, Block)>,
    default: Option<Block>,
}

impl SwitchBuilder {
    /// Creates and returns a new `SwitchBuilder` to construct a `Switch` statement.
    ///
    /// This method initializes a builder with just the condition expression,
    /// with no case blocks and no default block.
    ///
    /// # Parameters
    /// - `cond`: The expression to switch on
    ///
    /// # Returns
    /// A new `SwitchBuilder` instance initialized with the given condition
    ///
    /// # Examples
    /// ```rust
    /// let builder = SwitchBuilder::new(Expr::new_ident_with_str("status"));
    /// // Further configure the builder...
    /// let switch_stmt = builder
    ///     .case(Expr::Int(0), Block::new().statement(Statement::Return(None)).build())
    ///     .build();
    /// ```
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            cases: vec![],
            default: None,
        }
    }

    /// Creates and returns a new `SwitchBuilder` with a predefined condition and cases.
    ///
    /// This convenience constructor initializes a builder with both the condition expression
    /// and a collection of case expressions and their corresponding blocks.
    ///
    /// # Parameters
    /// - `cond`: The expression to switch on
    /// - `cases`: A vector of pairs, each containing a case expression and its block
    ///
    /// # Returns
    /// A new `SwitchBuilder` instance initialized with the given condition and cases
    ///
    /// # Examples
    /// ```rust
    /// let cases = vec![
    ///     (Expr::Int(0), Block::new().statement(Statement::Return(None)).build()),
    ///     (Expr::Int(1), Block::new().statement(Statement::Break).build())
    /// ];
    /// let builder = SwitchBuilder::new_with_cases(Expr::new_ident_with_str("status"), cases);
    /// // Further configure the builder if needed...
    /// let switch_stmt = builder.build();
    /// ```
    pub fn new_with_cases(cond: Expr, cases: Vec<(Expr, Block)>) -> Self {
        Self {
            cond,
            cases,
            default: None,
        }
    }

    /// Adds a new case to the switch statement.
    ///
    /// This method appends a new case expression and its corresponding block to the switch statement.
    ///
    /// # Parameters
    /// - `c`: The case expression to match against
    /// - `b`: The block of code to execute when the case matches
    ///
    /// # Returns
    /// `self` for method chaining
    ///
    /// # Examples
    /// ```rust
    /// let switch_stmt = SwitchBuilder::new(Expr::new_ident_with_str("status"))
    ///     .case(Expr::Int(0), Block::new().statement(Statement::Return(None)).build())
    ///     .case(Expr::Int(1), Block::new().statement(Statement::Break).build())
    ///     .build();
    /// ```
    pub fn case(mut self, c: Expr, b: Block) -> Self {
        self.cases.push((c, b));
        self
    }

    /// Sets all the cases for the switch statement at once.
    ///
    /// This method replaces any existing cases with the provided collection.
    ///
    /// # Parameters
    /// - `cases`: A vector of pairs, each containing a case expression and its block
    ///
    /// # Returns
    /// `self` for method chaining
    ///
    /// # Examples
    /// ```rust
    /// let new_cases = vec![
    ///     (Expr::Int(0), Block::new().statement(Statement::Return(None)).build()),
    ///     (Expr::Int(1), Block::new().statement(Statement::Break).build())
    /// ];
    /// let switch_stmt = SwitchBuilder::new(Expr::new_ident_with_str("status"))
    ///     .cases(new_cases)
    ///     .build();
    /// ```
    pub fn cases(mut self, cases: Vec<(Expr, Block)>) -> Self {
        self.cases = cases;
        self
    }

    /// Sets the default case for the switch statement.
    ///
    /// This method specifies the block of code to execute when none of the cases match.
    ///
    /// # Parameters
    /// - `default`: The block of code to execute as the default case
    ///
    /// # Returns
    /// `self` for method chaining
    ///
    /// # Examples
    /// ```rust
    /// let switch_stmt = SwitchBuilder::new(Expr::new_ident_with_str("status"))
    ///     .case(Expr::Int(0), Block::new().statement(Statement::Return(None)).build())
    ///     .default(Block::new().statement(Statement::Return(Some(Expr::Int(-1)))).build())
    ///     .build();
    /// ```
    pub fn default(mut self, default: Block) -> Self {
        self.default = Some(default);
        self
    }

    /// Finalizes the building process and returns the constructed `Switch` statement.
    ///
    /// This method consumes the builder and produces a `Switch` instance with all
    /// the properties configured during the building process.
    ///
    /// # Returns
    /// A new `Switch` instance with the configured properties
    ///
    /// # Examples
    /// ```rust
    /// let switch_stmt = SwitchBuilder::new(Expr::new_ident_with_str("status"))
    ///     .case(Expr::Int(0), Block::new().statement(Statement::Return(None)).build())
    ///     .case(Expr::Int(1), Block::new().statement(Statement::Break).build())
    ///     .default(Block::new().statement(Statement::Return(Some(Expr::Int(-1)))).build())
    ///     .build();
    /// ```
    pub fn build(self) -> Switch {
        Switch {
            cond: self.cond,
            cases: self.cases,
            default: self.default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn if_condition() {
        let another_if = IfBuilder::new(Expr::new_binary(
            Expr::new_ident_with_str("another_var"),
            BinOp::Eq,
            Expr::new_ident_with_str("some_var"),
        ))
        .then(
            BlockBuilder::new()
                .statements(vec![
                    Statement::GoTo("hello".to_string()),
                    Statement::WarningDirective(
                        WarningDirectiveBuilder::new_with_str("some warning").build(),
                    ),
                ])
                .build(),
        )
        .build();

        let i = IfBuilder::new(Expr::Bool(true))
            .then(
                BlockBuilder::new()
                    .statements(vec![
                        Statement::Comment(CommentBuilder::new_with_str("Some comment").build()),
                        Statement::ErrorDirective(
                            ErrorDirectiveBuilder::new_with_str("some error").build(),
                        ),
                        Statement::Return(None),
                    ])
                    .build(),
            )
            .other(Block::new().statement(Statement::If(another_if)).build())
            .build();

        let res = r#"if (true) {
  // Some comment
  #error "some error"
  return;
} else {
  if (another_var == some_var) {
    goto hello;
    #warning "some warning"
  }
}
"#;

        assert_eq!(i.to_string(), res);
    }

    #[test]
    fn switch_condition() {
        let s = SwitchBuilder::new(Expr::Bool(true))
            .case(
                Expr::new_null(),
                Block::new()
                    .statements(vec![
                        Statement::Comment(CommentBuilder::new_with_str("Hello, world").build()),
                        Statement::Comment(CommentBuilder::new_with_str("Another comment").build()),
                    ])
                    .build(),
            )
            .case(
                Expr::new_cast(Type::new(BaseType::UInt8).build(), Expr::Int(123)),
                Block::new()
                    .statement(Statement::Macro(Macro::Obj(
                        ObjMacroBuilder::new_with_str("AGE")
                            .value_with_str("18")
                            .build(),
                    )))
                    .build(),
            )
            .default(
                Block::new()
                    .statement(Statement::Raw("abc;".to_string()))
                    .build(),
            )
            .build();

        let res = r#"switch (true) {
case NULL: {
  // Hello, world
  // Another comment
}
case (uint8_t)(123): {
  #define AGE 18
}
default: {
  abc;
}
}
"#;

        assert_eq!(s.to_string(), res);
    }
}
