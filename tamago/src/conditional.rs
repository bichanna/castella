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

/// Represents an if statement in C.
///
/// # Examples
/// ```c
/// if (cond) {
///   // then block
/// } else {
///   // other block
/// }
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct If {
    /// The condition of the if statement
    pub cond: Expr,
    /// The then block of the if statement
    pub then: Block,
    /// Optional else block of the if statement
    pub other: Option<Block>,
}

impl If {
    /// Creates and returns a new `IfBuilder` to construct a `If` using the builder pattern.
    /// ```rust
    /// let i = If::new(/*cond expr*/)
    ///     .then(/*then block*/)
    ///     .other(/*other block*/)
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

/// A builder for constructing a `If` instance.
pub struct IfBuilder {
    cond: Expr,
    then: Block,
    other: Option<Block>,
}

impl IfBuilder {
    /// Creates and returns a new `IfBuilder` to construct a `If` using the builder pattern.
    /// ```rust
    /// let i = IfBuilder::new(/*cond expr*/)
    ///     .then(/*then block*/)
    ///     .other(/*other block*/)
    ///     .build();
    /// ```
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            then: Block::new().build(),
            other: None,
        }
    }

    /// Creates and returns a new `IfBuilder` to construct a `If` using the builder pattern with
    /// the given condition and then block.
    pub fn new_with_then(cond: Expr, then: Block) -> Self {
        Self {
            cond,
            then,
            other: None,
        }
    }

    /// Sets the then block for the builder, and returns the builder for more chaining.
    pub fn then(mut self, then: Block) -> Self {
        self.then = then;
        self
    }

    /// Sets the other block for the builder, and returns the builder for more chaining.
    pub fn other(mut self, other: Block) -> Self {
        self.other = Some(other);
        self
    }

    /// Appends a statement to the then block for the builder, and returns the builder for more
    /// chaining.
    pub fn statement_to_then(mut self, stmt: Statement) -> Self {
        self.then.stmts.push(stmt);
        self
    }

    /// Consumes the builder and returns a `If` containing all the condition, then block, and other
    /// block added during building process.
    pub fn build(self) -> If {
        If {
            cond: self.cond,
            then: self.then,
            other: self.other,
        }
    }
}

/// Represents a switch statement in C code.
/// ```c
/// switch (cond) {
/// case 1: {
///   // a block
/// }
/// case 2: {
///   // another block
/// }
/// default: {
///   // default block
/// }
/// }
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Switch {
    pub cond: Expr,
    pub cases: Vec<(Expr, Block)>,
    pub default: Option<Block>,
}

impl Switch {
    /// Creates and returns a new `SwitchBuilder` to construct a `Switch` using the buidler
    /// pattern.
    /// ```rust
    /// let s = Switch::new(/*cond*/)
    ///     .case(/*case and its block*/)
    ///     .case(/*another case and its block*/)
    ///     .default(/*default block*/)
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

/// Represents a switch statement in C code.
/// ```c
/// switch (cond) {
/// case 1: {
///   // a block
/// }
/// case 2: {
///   // another block
/// }
/// default: {
///   // default block
/// }
/// }
/// ```
pub struct SwitchBuilder {
    cond: Expr,
    cases: Vec<(Expr, Block)>,
    default: Option<Block>,
}

impl SwitchBuilder {
    /// Creates and returns a new `SwitchBuilder` to construct a `Switch` using the buidler
    /// pattern.
    /// ```rust
    /// let s = Switch::new(/*cond*/)
    ///     .case(/*case and its block*/)
    ///     .case(/*another case and its block*/)
    ///     .default(/*default block*/)
    ///     .build();
    /// ```
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            cases: vec![],
            default: None,
        }
    }

    /// Creates and returns a new `SwitchBuilder` to construct a `Switch` using the builder pattern
    /// with the given condition and cases.
    pub fn new_with_cases(cond: Expr, cases: Vec<(Expr, Block)>) -> Self {
        Self {
            cond,
            cases,
            default: None,
        }
    }

    /// Appends the provided `Expr` condition and `Block` as a case to the switch statement and
    /// returns the builder for chaining more operations.
    pub fn case(mut self, c: Expr, b: Block) -> Self {
        self.cases.push((c, b));
        self
    }

    /// Sets the switch cases to the provided cases, replacing any existing ones. This consumes and
    /// returns the builder for chaining more operations.
    pub fn cases(mut self, cases: Vec<(Expr, Block)>) -> Self {
        self.cases = cases;
        self
    }

    /// Sets the default case of the switch statement to the provided case.
    pub fn default(mut self, default: Block) -> Self {
        self.default = Some(default);
        self
    }

    /// Consumes the builder and returns a `Switch` containing all the cases added during the
    /// building process.
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
  if ((another_var == some_var)) {
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
