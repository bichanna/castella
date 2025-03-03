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

//! This module provides structures and builders for creating and managing C code blocks,
//! such as function bodies, loop bodies, and conditional branches.
//!
//! It defines the primary representation of a C program through the `Scope` struct,
//! which contains global statements and serves as the root container for generated code.
//! The module follows a builder pattern approach for constructing C code structures.

use std::fmt::{self, Write};

use crate::*;
use tamacro::DisplayFromFormat;

/// Represents a global scope in C, serving as the root container for all C code elements.
///
/// The global scope can contain various global statements like function definitions,
/// variable declarations, struct definitions, and preprocessor directives.
///
/// # Examples
/// ```c
/// int number = 0;  // Global variable declaration
///
/// int main(void) {
///   // Function body block
/// }
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Scope {
    /// Optional documentation comment associated with the scope.
    pub doc: Option<DocComment>,

    /// The collection of global statements contained within this scope.
    pub global_stmts: Vec<GlobalStatement>,
}

impl Scope {
    /// Creates and returns a new `ScopeBuilder` to construct a `Scope` using the builder pattern.
    ///
    /// This is the recommended way to create a new `Scope` instance.
    ///
    /// # Returns
    /// A new `ScopeBuilder` instance that can be used to build a `Scope`.
    ///
    /// # Examples
    /// ```rust
    /// let scope = Scope::new()
    ///     .global_statement(GlobalStatement::Include(include))
    ///     .new_line()
    ///     .global_statement(GlobalStatement::Function(function))
    ///     .build();
    /// ```
    ///
    pub fn new() -> ScopeBuilder {
        ScopeBuilder::new()
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

/// A builder for constructing a `Scope` instance using the builder pattern.
///
/// This builder provides methods to add various elements to a scope and finally build
/// the complete `Scope` instance.
pub struct ScopeBuilder {
    doc: Option<DocComment>,
    global_stmts: Vec<GlobalStatement>,
}

impl ScopeBuilder {
    /// Creates and returns a new `ScopeBuilder` to construct a `Scope`.
    ///
    /// # Returns
    /// A new `ScopeBuilder` instance with default (empty) values.
    ///
    /// # Examples
    /// ```rust
    /// let scope = ScopeBuilder::new()
    ///     .global_statement(GlobalStatement::Include(include))
    ///     .new_line()
    ///     .build();
    /// ```
    pub fn new() -> Self {
        Self {
            doc: None,
            global_stmts: vec![],
        }
    }

    /// Sets the documentation comment for the scope being built.
    ///
    /// # Parameters
    /// * `doc` - The documentation comment to associate with the scope.
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Sets all the global statements of the scope at once, replacing any existing statements.
    ///
    /// # Parameters
    /// * `global_stmts` - A vector of global statements to include in the scope.
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn global_statements(mut self, global_stmts: Vec<GlobalStatement>) -> Self {
        self.global_stmts = global_stmts;
        self
    }

    /// Appends a single global statement to the scope.
    ///
    /// # Parameters
    /// * `global_stmt` - The global statement to append to the scope.
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn global_statement(mut self, global_stmt: GlobalStatement) -> Self {
        self.global_stmts.push(global_stmt);
        self
    }

    /// Appends a new line to the scope for better formatting of the generated code.
    ///
    /// This is equivalent to adding `GlobalStatement::NewLine` to the scope.
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn new_line(self) -> Self {
        self.global_statement(GlobalStatement::NewLine)
    }

    /// Consumes the builder and returns the constructed `Scope` instance.
    ///
    /// # Returns
    /// A `Scope` instance containing the documentation and global statements
    /// configured in this builder.
    pub fn build(self) -> Scope {
        Scope {
            doc: self.doc,
            global_stmts: self.global_stmts,
        }
    }
}

/// Represents a global statement in C that can appear at the top level of a C file.
///
/// Global statements include function definitions, variable declarations,
/// type definitions, preprocessor directives, and more.
///
/// # Examples
/// ```c
/// int number = 0;  // Variable declaration
///
/// struct Person {   // Struct definition
///   char* name;
///   int age;
/// };
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub enum GlobalStatement {
    /// A comment in the code.
    Comment(Comment),

    /// An enum definition (e.g., `enum Color { RED, GREEN, BLUE };`).
    Enum(Enum),

    /// A struct definition (e.g., `struct Person { char* name; int age; };`).
    Struct(Struct),

    /// A function declaration or definition.
    Function(Function),

    /// A union definition (e.g., `union Data { int i; float f; };`).
    Union(Union),

    /// A variable declaration or definition.
    Variable(Variable),

    /// A typedef statement
    TypeDef(TypeDef),

    /// An error preprocessor directive (e.g., `#error "Not supported"`).
    ErrorDirective(ErrorDirective),

    /// An ifdef preprocessor directive (e.g., `#ifdef DEBUG`).
    IfDefDirective(IfDefDirective),

    /// An if preprocessor directive (e.g., `#if PLATFORM == WINDOWS`).
    IfDirective(IfDirective),

    /// An include preprocessor directive (e.g., `#include <stdio.h>`).
    Include(Include),

    /// A line preprocessor directive (e.g., `#line 50 "file.c"`).
    LineDirective(LineDirective),

    /// A macro definition (e.g., `#define MAX(a, b) ((a) > (b) ? (a) : (b))`).
    Macro(Macro),

    /// A pragma preprocessor directive (e.g., `#pragma once`).
    PragmaDirective(PragmaDirective),

    /// A warning preprocessor directive (e.g., `#warning "Deprecated feature"`).
    WarningDirective(WarningDirective),

    /// A raw piece of code inserted directly without processing.
    Raw(String),

    /// A new line for formatting purposes.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope() {
        let s = ScopeBuilder::new()
            .global_statement(GlobalStatement::WarningDirective(
                WarningDirectiveBuilder::new_with_str("some warning").build(),
            ))
            .new_line()
            .global_statement(GlobalStatement::Include(
                IncludeBuilder::new_system_with_str("stdio.h").build(),
            ))
            .build();
        let res = r#"#warning "some warning"

#include <stdio.h>
"#;

        assert_eq!(s.to_string(), res);

        let s = ScopeBuilder::new()
            .global_statements(vec![
                GlobalStatement::Comment(CommentBuilder::new().comment_with_str("Hello").build()),
                GlobalStatement::NewLine,
                GlobalStatement::Function(
                    FunctionBuilder::new_with_str("some_func", Type::new(BaseType::Bool).build())
                        .body(
                            BlockBuilder::new()
                                .statement(Statement::Return(None))
                                .build(),
                        )
                        .doc(
                            DocCommentBuilder::new()
                                .line_str("this is a function")
                                .build(),
                        )
                        .build(),
                ),
            ])
            .build();
        let res = r#"// Hello

/// this is a function
bool some_func(void) {
  return;
}
"#;

        assert_eq!(s.to_string(), res);
    }
}
