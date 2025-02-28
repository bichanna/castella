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

//! This module provides ways to create blocks of code, like function bodies, loop bodies,
//! conditional branches.

use std::fmt::{self, Write};

use crate::*;
use tamacro::DisplayFromFormat;

/// Represents a global scope in C.
///
/// # Examples
/// ```c
/// int number = 0;
///
/// int main(void) {
///   // body block
/// }
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Scope {
    /// The optional doc comment.
    pub doc: Option<DocComment>,

    /// The global statements in the scope.
    pub global_stmts: Vec<GlobalStatement>,
}

impl Scope {
    /// Creates and returns a new `ScopeBuilder` to construct a `Scope` using the builder pattern.
    /// ```rust
    /// let scope = Scope::new()
    ///     .global_statement(/*global statement*/)
    ///     .new_line()
    ///     .build();
    /// ```
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

/// A builder for constructing a `Scope` instance.
pub struct ScopeBuilder {
    doc: Option<DocComment>,
    global_stmts: Vec<GlobalStatement>,
}

impl ScopeBuilder {
    /// Creates and returns a new `ScopeBuilder` to construct a `Scope` using the builder pattern.
    /// ```rust
    /// let scope = ScopeBuilder::new()
    ///     .global_statement(/*global statement*/)
    ///     .new_line()
    ///     .build();
    /// ```
    pub fn new() -> Self {
        Self {
            doc: None,
            global_stmts: vec![],
        }
    }

    /// Sets the doc cocmment for the scope being built and returns the builder for more chaining.
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Sets the global statements of the scope and returns the builder for more chaining.
    pub fn global_statements(mut self, global_stmts: Vec<GlobalStatement>) -> Self {
        self.global_stmts = global_stmts;
        self
    }

    /// Appends a global statement to the scope and returns the builder for more chaining.
    pub fn global_statement(mut self, global_stmt: GlobalStatement) -> Self {
        self.global_stmts.push(global_stmt);
        self
    }

    /// Appends a new line to the scope and returns the builder for more chaining.
    pub fn new_line(self) -> Self {
        self.global_statement(GlobalStatement::NewLine)
    }

    /// Consumes the builder and returns a `Scope` containing all the global statements.
    pub fn build(self) -> Scope {
        Scope {
            doc: self.doc,
            global_stmts: self.global_stmts,
        }
    }
}

/// Represents a global statement in C.
///
/// # Examples
/// ```c
/// int number = 0;
/// struct Person {
///   char* name;
///   int age;
/// }
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub enum GlobalStatement {
    /// A comment
    Comment(Comment),

    /// An enum definition.
    Enum(Enum),

    /// A struct definition.
    Struct(Struct),

    /// A function declaration/definition.
    Function(Function),

    /// A union definition.
    Union(Union),

    /// A variable declaration/definition.
    Variable(Variable),

    /// A typedef statement.
    TypeDef(TypeDef),

    /// An error preprocessor directive.
    ErrorDirective(ErrorDirective),

    /// An ifdef preprocessor directive.
    IfDefDirective(IfDefDirective),

    /// An if preprocessor directive.
    IfDirective(IfDirective),

    /// An include preprocessor directive.
    Include(Include),

    /// A line preprocessor directive.
    LineDirective(LineDirective),

    /// Macro definition.
    Macro(Macro),

    /// A pragram preprocessor directive.
    PragmaDirective(PragmaDirective),

    /// A warning preprocessor directive.
    WarningDirective(WarningDirective),

    /// A raw piece of code.
    Raw(String),

    /// A new line.
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
