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

//! # Block Module
//!
//! This module provides functionality for creating and managing code blocks in C programs.
//! Code blocks represent structured segments of code such as function bodies, loop bodies,
//! conditional branches, and other scoped statements.
//!
//! The primary components in this module include:
//! - `Block`: Represents a sequence of statements within curly braces in C
//! - `BlockBuilder`: Facilitates constructing blocks using the builder pattern
//! - `Statement`: Enumerates the various statement types that can appear within a block
//!
//! Use this module to programmatically generate well-structured C code with proper
//! scoping and nesting of statements.

use std::fmt::{self, Write};

use crate::{
    Comment, DoWhile, ErrorDirective, Expr, For, Format, Formatter, If, IfDefDirective,
    IfDirective, Include, LineDirective, Macro, PragmaDirective, Switch, Variable,
    WarningDirective, While,
};
use tamacro::DisplayFromFormat;

/// Represents a scoped block of code in C, delimited by curly braces `{}`.
/// A `Block` contains a sequence of statements that are executed together
/// within the same lexical scope.
///
/// Blocks are fundamental to C's structure and are used in various contexts:
/// - Function bodies
/// - Loop bodies (for, while, do-while)
/// - Conditional branches (if, else, switch cases)
///
/// ## Memory Management
///
/// In C, blocks create a new scope for variables, meaning variables declared within
/// a block are destroyed when execution leaves the block, unless they're declared with
/// `static` storage duration.
///
/// ## Examples
///
/// ### Function Body
/// ```c
/// int calculate_sum(int a, int b) {
///     int result;      // Variable declaration
///     result = a + b;  // Assignment expression
///     return result;   // Return statement
/// }
/// ```
///
/// ### Conditional Branch
/// ```c
/// if (condition) {
///     perform_action();
///     update_state();
/// }
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Block {
    /// Collection of statements that make up the block's content.
    /// These statements are executed sequentially when the block is entered.
    pub stmts: Vec<Statement>,
}

impl Block {
    /// Creates and returns a new `BlockBuilder` to construct a `Block` using the builder pattern.
    ///
    /// This method is the recommended entry point for creating blocks as it provides
    /// a fluent API for adding statements and other content.
    ///
    /// ## Example
    ///
    /// ```rust
    /// // Create a block with multiple statements
    /// let block = Block::new()
    ///     .statement(Statement::Variable(var_declaration))
    ///     .statement(Statement::Expr(some_expression))
    ///     .statement(Statement::Return(Some(return_value)))
    ///     .build();
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

/// A builder for constructing `Block` instances in a fluent, chainable manner.
///
/// The builder pattern provides a convenient way to incrementally construct blocks
/// by adding statements one at a time or in batches. It also provides utility methods
/// for common operations like adding blank lines or merging blocks.
///
/// ## Usage Pattern
///
/// 1. Create a new builder with `BlockBuilder::new()` or `Block::new()`
/// 2. Add statements using the various builder methods
/// 3. Call `build()` to create the final `Block` instance
pub struct BlockBuilder {
    stmts: Vec<Statement>,
}

impl BlockBuilder {
    /// Creates and returns a new `BlockBuilder` instance.
    ///
    /// The new builder starts with an empty collection of statements.
    ///
    /// ## Example
    ///
    /// ```rust
    /// // Create a new block builder
    /// let builder = BlockBuilder::new();
    ///
    /// // Add statements and build the block
    /// let block = builder
    ///     .statement(Statement::Expr(Expr::UInt(42)))
    ///     .statement(Statement::Return(None))
    ///     .build();
    /// ```
    pub fn new() -> Self {
        Self { stmts: vec![] }
    }

    /// Appends a single statement to the block being built.
    ///
    /// This method consumes the builder and returns it, allowing for method chaining.
    ///
    /// ## Parameters
    ///
    /// - `stmt`: The statement to add to the block
    ///
    /// ## Returns
    ///
    /// The builder instance for method chaining
    ///
    /// ## Example
    ///
    /// ```rust
    /// let block = BlockBuilder::new()
    ///     .statement(Statement::Expr(Expr::UInt(1)))
    ///     .statement(Statement::Break)
    ///     .build();
    /// ```
    pub fn statement(mut self, stmt: Statement) -> Self {
        self.stmts.push(stmt);
        self
    }

    /// Replaces all existing statements with the provided collection of statements.
    ///
    /// This method is useful when you want to set multiple statements at once,
    /// potentially overwriting any statements that were previously added.
    ///
    /// ## Parameters
    ///
    /// - `stmts`: A vector of statements to set as the block's content
    ///
    /// ## Returns
    ///
    /// The builder instance for method chaining
    ///
    /// ## Example
    ///
    /// ```rust
    /// let predefined_statements = vec![
    ///     Statement::Variable(var_decl),
    ///     Statement::Expr(expr),
    ///     Statement::Return(Some(return_val))
    /// ];
    ///
    /// let block = BlockBuilder::new()
    ///     .statements(predefined_statements)
    ///     .build();
    /// ```
    pub fn statements(mut self, stmts: Vec<Statement>) -> Self {
        self.stmts = stmts;
        self
    }

    /// Adds an empty line (newline character) to the block.
    ///
    /// This method is useful for formatting purposes, allowing you to add
    /// vertical spacing between statements for improved readability.
    ///
    /// ## Returns
    ///
    /// The builder instance for method chaining
    ///
    /// ## Example
    ///
    /// ```rust
    /// let block = BlockBuilder::new()
    ///     .statement(Statement::Variable(var_decl))
    ///     .new_line()  // Add vertical space for readability
    ///     .statement(Statement::Expr(some_calculation))
    ///     .build();
    /// ```
    pub fn new_line(self) -> Self {
        self.statement(Statement::NewLine)
    }

    /// Merges the statements from another `Block` into this builder.
    /// The statements from the other block are appended after any existing statements.
    ///
    /// This method is useful for combining blocks or for reusing parts of existing blocks.
    ///
    /// ## Parameters
    ///
    /// - `other`: Another block whose statements will be merged into this builder
    ///
    /// ## Returns
    ///
    /// The builder instance for method chaining
    ///
    /// ## Example
    ///
    /// ```rust
    /// // Create a common initialization block
    /// let init_block = Block::new()
    ///     .statement(Statement::Variable(var_init1))
    ///     .statement(Statement::Variable(var_init2))
    ///     .build();
    ///
    /// // Reuse the initialization block in another context
    /// let full_block = BlockBuilder::new()
    ///     .merge(init_block)
    ///     .statement(Statement::Expr(main_logic))
    ///     .statement(Statement::Return(Some(result)))
    ///     .build();
    /// ```
    pub fn merge(mut self, mut other: Block) -> Self {
        self.stmts.append(&mut other.stmts);
        self
    }

    /// Consumes the builder and creates a new `Block` instance.
    ///
    /// This method finalizes the building process and returns the constructed block
    /// containing all the statements that were added to the builder.
    ///
    /// ## Returns
    ///
    /// A new `Block` instance containing the statements added to the builder
    ///
    /// ## Example
    ///
    /// ```rust
    /// // Build a complete function body
    /// let function_body = BlockBuilder::new()
    ///     .statement(Statement::Variable(local_var))
    ///     .statement(Statement::Expr(calculation))
    ///     .statement(Statement::Return(Some(result)))
    ///     .build();
    /// ```
    pub fn build(self) -> Block {
        Block { stmts: self.stmts }
    }
}

/// Represents the various types of statements and preprocessor directives
/// that can appear within a C code block.
///
/// This enum covers the full range of C language constructs, including:
/// - Basic statements (expressions, returns, breaks, continues)
/// - Control flow statements (if, switch, loops)
/// - Variable declarations
/// - Labels and gotos
/// - Preprocessor directives
/// - Comments and raw code insertion
///
/// Each variant corresponds to a specific type of C statement or directive,
/// with associated data where necessary.
#[derive(Debug, Clone, DisplayFromFormat)]
pub enum Statement {
    /// A C-style comment (either line comment `//` or block comment `/* */`)
    Comment(Comment),

    /// A variable declaration statement, which may include initialization
    ///
    /// Example: `int counter = 0;`
    Variable(Variable),

    /// An expression statement, typically ending with a semicolon
    ///
    /// Examples:
    /// - `func();`
    /// - `a = b + c;`
    /// - `counter++;`
    Expr(Expr),

    /// A return statement, optionally with an expression to return
    ///
    /// Examples:
    /// - `return;` (represented as `Return(None)`)
    /// - `return value;` (represented as `Return(Some(expr))`)
    Return(Option<Expr>),

    /// A break statement to exit a loop or switch
    ///
    /// Example: `break;`
    Break,

    /// A continue statement to skip to the next iteration of a loop
    ///
    /// Example: `continue;`
    Continue,

    /// A goto statement that jumps to a labeled statement
    ///
    /// Example: `goto error_handler;`
    GoTo(String),

    /// A label declaration that can be targeted by goto statements
    ///
    /// Example: `some_label:`
    Label(String),

    /// An if statement with optional else and else if branches
    ///
    /// Example: `if (condition) { ... } else if (another) { ... } else { ... }`
    If(If),

    /// A switch statement with multiple case branches
    ///
    /// Example: `switch (value) { case 1: ... break; default: ... }`
    Switch(Switch),

    /// A while loop
    ///
    /// Example: `while (condition) { ... }`
    While(While),

    /// A do-while loop
    ///
    /// Example: `do { ... } while (condition);`
    DoWhile(DoWhile),

    /// A for loop
    ///
    /// Example: `for (int i = 0; i < 10; i++) { ... }`
    For(For),

    /// A `#error` preprocessor directive that causes compilation to fail with a message
    ///
    /// Example: `#error "This platform is not supported"`
    ErrorDirective(ErrorDirective),

    /// A `#ifdef`, `#ifndef`, or `#elifdef` preprocessor directive for conditional compilation
    ///
    /// Example: `#ifdef DEBUG ... #endif`
    IfDefDirective(IfDefDirective),

    /// A `#if`, `#elif`, or `#else` preprocessor directive for conditional compilation
    ///
    /// Example: `#if PLATFORM == WINDOWS ... #else ... #endif`
    IfDirective(IfDirective),

    /// An `#include` directive to include a header file
    ///
    /// Examples:
    /// - `#include <stdio.h>`
    /// - `#include "myheader.h"`
    Include(Include),

    /// A `#line` preprocessor directive to control line numbering in error messages
    ///
    /// Example: `#line 50 "some_file.c"`
    LineDirective(LineDirective),

    /// A macro definition
    ///
    /// Examples:
    /// - `#define MAX_SIZE 100`
    /// - `#define SUM(a, b) ((a) + (b))`
    Macro(Macro),

    /// A `#pragma` directive for implementation-specific behaviors
    ///
    /// Example: `#pragma once`
    PragmaDirective(PragmaDirective),

    /// A `#warning` directive that emits a compiler warning
    ///
    /// Example: `#warning "This code is deprecated!"`
    WarningDirective(WarningDirective),

    /// Raw C code inserted verbatim without processing
    ///
    /// Useful for edge cases not covered by other statement types
    Raw(String),

    /// A standalone newline for formatting purposes
    ///
    /// Adds vertical whitespace between statements for improved readability
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
