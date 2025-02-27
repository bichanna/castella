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

//! This module provides means to interact with preprocessor directives and C macros.

use std::fmt::{self, Write};

use crate::*;
use tamacro::DisplayFromFormat;

/// Represents the `include` preprocessor directive in C.
///
/// # Examples
/// ```c
/// #include <stdio.h>
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Include {
    /// The path to the header file.
    pub path: String,

    /// Whether it's a system import or not.
    pub is_system: bool,

    /// The optional doc comment
    pub doc: Option<DocComment>,
}

impl Include {
    /// Creates and returns a new `IncludeBuilder` to construct an `Include` using the builder
    /// pattern.
    /// ```rust
    /// let include = Include::new(/*path to the header file*/)
    ///     .build();
    /// ```
    pub fn new(path: String) -> IncludeBuilder {
        IncludeBuilder::new(path)
    }
}

impl Format for Include {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        write!(fmt, "#include ")?;

        if self.is_system {
            writeln!(fmt, "<{}>", self.path)?;
        } else {
            writeln!(fmt, "\"{}\"", self.path)?;
        }

        Ok(())
    }
}

/// A builder for constructing a `Include` instance.
pub struct IncludeBuilder {
    path: String,
    is_system: bool,
    doc: Option<DocComment>,
}

impl IncludeBuilder {
    /// Creates and returns a new `IncludeBuilder` to construct an `Include` using the builder
    /// pattern.
    /// ```rust
    /// let include = IncludeBuilder::new(/*path to the header file*/)
    ///     .build();
    /// ```
    pub fn new(path: String) -> Self {
        Self {
            path,
            is_system: false,
            doc: None,
        }
    }

    /// Creates and returns a new `IncludeBuilder` to construct an `Include` with the given path
    /// string slice.
    pub fn new_with_str(path: &str) -> Self {
        Self {
            path: path.to_string(),
            is_system: false,
            doc: None,
        }
    }

    /// Creates and returns a new `IncludeBuilder` to construct a system `Include` with the given path string.
    pub fn new_system(path: String) -> Self {
        Self {
            path,
            is_system: true,
            doc: None,
        }
    }

    /// Creates and returns a new `IncludeBuilder` to construct a system `Include` with the given
    /// path string slice.
    pub fn new_system_with_str(path: &str) -> Self {
        Self {
            path: path.to_string(),
            is_system: true,
            doc: None,
        }
    }

    /// Sets the optional doc for the include directive.
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Consumes the builder and returns an `Include` containing the path to the header file.
    pub fn build(self) -> Include {
        Include {
            path: self.path,
            is_system: self.is_system,
            doc: self.doc,
        }
    }
}

/// Represents the `error` preprocessor directive in C.
///
/// # Examples
/// ```c
/// #error "error message"
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct ErrorDirective {
    /// The error message to be displayed.
    pub message: String,
}

impl ErrorDirective {
    /// Creates and returns a new `ErrorDirectiveBuilder` to construct an `ErrorDirective` using
    /// the builder pattern.
    /// ```rust
    /// let err = ErrorDirective::new(/*error message*/)
    ///     .build();
    /// ```
    pub fn new(message: String) -> ErrorDirectiveBuilder {
        ErrorDirectiveBuilder::new(message)
    }
}

impl Format for ErrorDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#error \"{}\"", self.message)
    }
}

/// A buidle for constructing an `ErrorDirective` instance.
pub struct ErrorDirectiveBuilder {
    message: String,
}

impl ErrorDirectiveBuilder {
    /// Creates and returns a new `ErrorDirectiveBuilder` to construct an `ErrorDirective` using
    /// the buidler pattern.
    /// ```rust
    /// let err = ErrorDirectiveBuilder::new(/*error message*/)
    ///     .build();
    /// ```
    pub fn new(message: String) -> Self {
        Self { message }
    }

    /// Creates and returns a new `ErrorDirectiveBuilder` to construct an `ErrorDirective` with the
    /// given message string slice using the builder pattern.
    pub fn new_with_str(message: &str) -> Self {
        Self::new(message.to_string())
    }

    /// Consumes the builder and returns an `ErrorDirective` containing the error message.
    pub fn build(self) -> ErrorDirective {
        ErrorDirective {
            message: self.message,
        }
    }
}

/// Represents a `pragma` preprocessor directive in C.
///
/// # Examples
/// ```c
/// #pragma once // or something else
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct PragmaDirective {
    /// The token that represents a specific instruction or action for the compiler.
    pub raw: String,
}

impl PragmaDirective {
    /// Creates and returns a new `PragmaDirectiveBuilder` to construct a `PragmaDirective` using
    /// the builder pattern.
    /// ```rust
    /// let pragma = PragmaDirective::new(/*raw token*/)
    ///     .build();
    /// ```
    pub fn new(raw: String) -> PragmaDirectiveBuilder {
        PragmaDirectiveBuilder::new(raw)
    }
}

impl Format for PragmaDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#pragma {}", self.raw)
    }
}

/// A builder for constructing a `PragmaDirective` instance.
pub struct PragmaDirectiveBuilder {
    raw: String,
}

impl PragmaDirectiveBuilder {
    /// Creates and returns a new `PragmaDirectiveBuilder` to construct a `PragmaDirective` using
    /// the builder pattern.
    /// ```rust
    /// let pragma = PragmaDirectiveBuilder::new(/*raw token*/)
    ///     .build();
    /// ```
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    /// Creates and returns a new `PragmaDirectiveBuilder` to construct a `PragmaDirective` with
    /// the given raw token string slice using the builder pattern.
    pub fn new_with_str(raw: &str) -> Self {
        Self::new(raw.to_string())
    }

    /// Consumes the builder and returns a `PragmaDirective` containing the raw token string.
    pub fn build(self) -> PragmaDirective {
        PragmaDirective { raw: self.raw }
    }
}

/// Represents a macro definition in C.
#[derive(Debug, Clone, DisplayFromFormat)]
pub enum Macro {
    /// Definition of an object macro.
    Obj(ObjMacro),

    /// Definition of a function-like macro.
    Func(FuncMacro),
}

impl Format for Macro {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use Macro::*;
        match self {
            Obj(m) => m.format(fmt),
            Func(m) => m.format(fmt),
        }
    }
}

/// Represents an object macro in C.
///
/// # Examples
/// ```c
/// #define SOMETHING 123
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct ObjMacro {
    /// The name of the macro.
    pub name: String,

    /// The predefined value or code fragment.
    pub value: Option<String>,

    /// The optional doc comment for the macro.
    pub doc: Option<DocComment>,
}

impl ObjMacro {
    /// Creates and returns a new `ObjMacroBuilder` to construct an `ObjMacro` using the builder
    /// pattern.
    /// ```rust
    /// let obj_macro = ObjMacro::new(/*name of the macro*/)
    ///     .value(/*predefined value*/)
    ///     .build();
    /// ```
    pub fn new(name: String) -> ObjMacroBuilder {
        ObjMacroBuilder::new(name)
    }
}

impl Format for ObjMacro {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        write!(fmt, "#define {}", self.name)?;

        if let Some(value) = &self.value {
            write!(fmt, " ")?;

            let lines = value.lines().collect::<Vec<&str>>();

            if lines.len() > 1 {
                fmt.indent(|fmt| {
                    for line in &lines[..lines.len() - 1] {
                        writeln!(fmt, "{line} \\")?;
                    }

                    if let Some(last) = lines.last() {
                        writeln!(fmt, "{last}")?;
                    }

                    Ok(())
                })
            } else {
                writeln!(fmt, "{value}")
            }
        } else {
            writeln!(fmt)
        }
    }
}

/// A builder for constructing an `ObjMacro` instance.
pub struct ObjMacroBuilder {
    name: String,
    value: Option<String>,
    doc: Option<DocComment>,
}

impl ObjMacroBuilder {
    /// Creates and returns a new `ObjMacroBuilder` to construct an `ObjMacro` using the builder
    /// pattern.
    /// ```rust
    /// let obj_macro = ObjMacroBuilder::new(/*name of the macro*/)
    ///     .value(/*predefined value*/)
    ///     .build();
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: None,
            doc: None,
        }
    }

    /// Creates and returns a new `ObjMacroBuilder` to construct an `ObjMacro` with the given name
    /// string slice using the builder pattern.
    pub fn new_with_str(name: &str) -> Self {
        Self::new(name.to_string())
    }

    /// Sets the optional doc comment for the macro and returns the builder for more chaining.
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Sets the predefined value or code fragment for the macro and returns the builder for more
    /// chaining.
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    /// Sets the predefined value or code fragment for the macro with the given value string slice
    /// and returns the builder for more chaining.
    pub fn value_with_str(self, value: &str) -> Self {
        self.value(value.to_string())
    }

    /// Consumes the builder and returns an `ObjMacro` containing the name and the value for it.
    pub fn build(self) -> ObjMacro {
        ObjMacro {
            name: self.name,
            value: self.value,
            doc: self.doc,
        }
    }
}

/// Represents a function-like macro in C.
///
/// # Examples
/// ```c
/// #define MUL(x, y) (x) * (y)
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct FuncMacro {
    /// The name of the macro.
    pub name: String,

    /// The parameters of the macro.
    pub params: Vec<String>,

    /// The body of the function-like macro.
    pub value: String,

    /// The optional doc comment for the macro.
    pub doc: Option<DocComment>,
}

impl FuncMacro {
    /// Creates and returns a new `FuncMacroBuilder` to construct a `FuncMacro` using the builder
    /// pattern.
    /// ```rust
    /// let func_macro = FuncMacro::new(/*name of the macro*/)
    ///     .params(/*parameteres*/)
    ///     .value(/*the body of the macro*/)
    ///     .build();
    /// ```
    pub fn new(name: String) -> FuncMacroBuilder {
        FuncMacroBuilder::new(name)
    }
}

impl Format for FuncMacro {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        write!(fmt, "#define {}(", self.name)?;

        for param in &self.params[..self.params.len() - 1] {
            write!(fmt, "{param}, ")?;
        }

        if let Some(last) = self.params.last() {
            write!(fmt, "{last}")?;
        }

        write!(fmt, ") ")?;

        let lines = self.value.lines().collect::<Vec<&str>>();

        if lines.len() > 1 {
            writeln!(fmt, "\\")?;

            fmt.indent(|fmt| {
                for line in &lines[..lines.len() - 1] {
                    writeln!(fmt, "{line} \\")?;
                }

                if let Some(last) = lines.last() {
                    writeln!(fmt, "{last}")?;
                }

                Ok(())
            })
        } else {
            writeln!(fmt, "{}", self.value)
        }
    }
}

/// A builder for constructing a `FuncMacro` instance.
pub struct FuncMacroBuilder {
    name: String,
    params: Vec<String>,
    value: String,
    doc: Option<DocComment>,
}

impl FuncMacroBuilder {
    /// Creates and returns a new `FuncMacroBuilder` to construct a `FuncMacro` using the builder
    /// pattern.
    /// ```rust
    /// let func_macro = FuncMacro::new(/*name of the macro*/)
    ///     .params(/*parameters*/)
    ///     .value(/*the body of the macro*/)
    ///     .build();
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: vec![],
            value: "".to_string(),
            doc: None,
        }
    }

    /// Creates and returns a new `FuncMacroBuilder` to construct a `FuncMacro` with the given name
    /// string slice using the builder pattern.
    pub fn new_with_str(name: &str) -> Self {
        Self::new(name.to_string())
    }

    /// Sets the optional doc comment for the macro.
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Appends a parameter to the function-like macro being built and returns the builder for more
    /// chaining.
    pub fn param(mut self, param: String) -> Self {
        self.params.push(param);
        self
    }

    /// Appends a parameter to the function-like macro being built with the given param string
    /// slice and returns the builder for more chaining.
    pub fn param_with_str(self, param: &str) -> Self {
        self.param(param.to_string())
    }

    /// Sets the parameters of the function-like macro and returns the builder for more chaining.
    pub fn params(mut self, params: Vec<String>) -> Self {
        self.params = params;
        self
    }

    /// Appends a variadic argument symbol to the parameter list of the macro and returns the
    /// builder for more chaining.
    pub fn variadic_arg(self) -> Self {
        self.param_with_str("...")
    }

    /// Sets the body of the macro and returns the builder for more chaining.
    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    /// Sets the body of the macro with the given string slice and returns the builder for more
    /// chaining.
    pub fn value_with_str(self, value: &str) -> Self {
        self.value(value.to_string())
    }

    /// Consumes the builder and returns a `FuncMacro` containing the name, parameters, and the
    /// body.
    pub fn build(self) -> FuncMacro {
        FuncMacro {
            name: self.name,
            params: self.params,
            value: self.value,
            doc: self.doc,
        }
    }
}

/// Either a `Scope` or a `Block`.
#[derive(Debug, Clone, DisplayFromFormat)]
pub enum ScopeOrBlock {
    Scope(Scope),
    Block(Block),
}

impl Format for ScopeOrBlock {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ScopeOrBlock::Scope(s) => s.format(fmt),
            ScopeOrBlock::Block(b) => b.format(fmt),
        }
    }
}

/// Represents the `if` preprocessor directive in C.
///
/// # Examples
/// ```c
/// #if cond
///   // then body
/// #else
///   // other body
/// #endif
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct IfDirective {
    /// The condition for the then block.
    pub cond: String,

    /// The then body.
    pub then: ScopeOrBlock,

    /// The optional else block.
    pub other: Option<ScopeOrBlock>,
}

impl IfDirective {
    /// Creates and returns a new `IfDirectiveBuilder` to construct a `IfDirective` using the
    /// builder pattern.
    /// ```rust
    /// let if_dir = IfDirective::new(/*cond*/)
    ///     .then(/*then block*/)
    ///     .other(/*else block*/)
    ///     .build();
    /// ```
    pub fn new(cond: String) -> IfDirectiveBuilder {
        IfDirectiveBuilder::new(cond)
    }
}

impl Format for IfDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#if {}", self.cond)?;
        self.then.format(fmt)?;

        if let Some(other) = &self.other {
            writeln!(fmt, "#else")?;
            other.format(fmt)?;
        }

        writeln!(fmt, "#endif")
    }
}

/// A builder for constructing a `IfDirective` instance.
pub struct IfDirectiveBuilder {
    cond: String,
    then: ScopeOrBlock,
    other: Option<ScopeOrBlock>,
}

impl IfDirectiveBuilder {
    /// Creates and returns a new `IfDirectiveBuilder` to construct a `IfDirective` using the
    /// builder pattern.
    /// ```rust
    /// let if_dir = IfDirectiveBuilder::new(/*cond*/)
    ///     .then(/*then block*/)
    ///     .other(/*else block*/)
    ///     .build();
    /// ```
    pub fn new(cond: String) -> Self {
        Self {
            cond,
            then: ScopeOrBlock::Scope(Scope::new().build()),
            other: None,
        }
    }

    /// Creates and returns a new `IfDirectiveBuilder` to construct a `IfDirective` with the given
    /// condition string slice using the builder pattern.
    pub fn new_with_str(cond: &str) -> Self {
        Self::new(cond.to_string())
    }

    /// Appends a global statement to the then body and returns the builder for more chaining.
    pub fn global_statement(mut self, global_stmt: GlobalStatement) -> Self {
        match &mut self.then {
            ScopeOrBlock::Scope(then) => {
                then.global_stmts.push(global_stmt);
                self
            }
            ScopeOrBlock::Block(_) => self.then(ScopeOrBlock::Scope(
                Scope::new().global_statement(global_stmt).build(),
            )),
        }
    }

    /// Appends a block statement to the then body and returns the builder for more chaining.
    pub fn block_statement(mut self, stmt: Statement) -> Self {
        match &mut self.then {
            ScopeOrBlock::Block(then) => {
                then.stmts.push(stmt);
                self
            }
            ScopeOrBlock::Scope(_) => self.then(ScopeOrBlock::Block(
                Block::new().statements(vec![stmt]).build(),
            )),
        }
    }

    /// Sets the then body and returns the builder for more chaining.
    pub fn then(mut self, then: ScopeOrBlock) -> Self {
        self.then = then;
        self
    }

    /// Sets the optional else body and returns the builder for more chaining.
    pub fn other(mut self, other: ScopeOrBlock) -> Self {
        self.other = Some(other);
        self
    }

    /// Consumes the builder and returns a `IfDirective` containing the condition, then body,
    /// optional else body.
    pub fn build(self) -> IfDirective {
        IfDirective {
            cond: self.cond,
            then: self.then,
            other: self.other,
        }
    }
}

/// Represents the `ifdef` and `ifndef` preprocessor directives in C.
///
/// # Examples
/// ```c
/// #ifdef SOME_MACRO
///   // then body
/// #else
///   // else body
/// #endif
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct IfDefDirective {
    /// The symbol to be checked.
    pub symbol: String,

    /// The then body
    pub then: ScopeOrBlock,

    /// The optional else body.
    pub other: Option<ScopeOrBlock>,

    /// Whether it's `ifndef` or `ifdef`.
    pub not: bool,
}

impl IfDefDirective {
    /// Creates and returns a new `IfDefDirectiveBuilder` to construct an `IfDefDirective` using
    /// the builder pattern.
    /// ```rust
    /// let ifdef = IfDefDirective::new(/*symbol*/)
    ///     .then(/*then body*/)
    ///     .other(/*else body*/)
    ///     .build();
    /// ```
    pub fn new(symbol: String) -> IfDefDirectiveBuilder {
        IfDefDirectiveBuilder::new(symbol)
    }
}

impl Format for IfDefDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if self.not {
            writeln!(fmt, "ifndef {}", self.symbol)?;
        } else {
            writeln!(fmt, "#ifdef {}", self.symbol)?;
        }
        self.then.format(fmt)?;

        if let Some(other) = &self.other {
            writeln!(fmt, "#else")?;
            other.format(fmt)?;
        }

        writeln!(fmt, "#endif")
    }
}

/// A builder for constructing an `IfDefDirective` instance.
pub struct IfDefDirectiveBuilder {
    symbol: String,
    then: ScopeOrBlock,
    other: Option<ScopeOrBlock>,
    not: bool,
}

impl IfDefDirectiveBuilder {
    /// Creates and returns a new `IfDefDirectiveBuilder` to construct an `IfDefDirective` using
    /// the builder pattern.
    /// ```rust
    /// let ifdef = IfDefDirectiveBuilder::new(/*symbol*/)
    ///     .then(/*then body*/)
    ///     .other(/*else body*/)
    ///     .build();
    /// ```
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            then: ScopeOrBlock::Scope(Scope::new().build()),
            other: None,
            not: false,
        }
    }

    /// Creates and returns a new `IfDefDirectiveBuilder` to construct an `IfDefDirective` with the
    /// given symbol string slice using the builder pattern.
    pub fn new_with_str(symbol: &str) -> Self {
        Self::new(symbol.to_string())
    }

    /// Appends a global statement to the then body and returns the builder for more chaining.
    pub fn global_statement(mut self, global_stmt: GlobalStatement) -> Self {
        match &mut self.then {
            ScopeOrBlock::Scope(then) => {
                then.global_stmts.push(global_stmt);
                self
            }
            ScopeOrBlock::Block(_) => self.then(ScopeOrBlock::Scope(
                Scope::new().global_statement(global_stmt).build(),
            )),
        }
    }

    /// Appends a block statement to the then body and returns the builder for more chaining.
    pub fn block_statement(mut self, stmt: Statement) -> Self {
        match &mut self.then {
            ScopeOrBlock::Block(then) => {
                then.stmts.push(stmt);
                self
            }
            ScopeOrBlock::Scope(_) => {
                self.then(ScopeOrBlock::Block(Block::new().statement(stmt).build()))
            }
        }
    }

    /// Sets the then body and returns the builder for more chaining.
    pub fn then(mut self, then: ScopeOrBlock) -> Self {
        self.then = then;
        self
    }

    /// Sets the optional else body and returns the builder for more chaining.
    pub fn other(mut self, other: ScopeOrBlock) -> Self {
        self.other = Some(other);
        self
    }

    /// Makes it `ifndef` and returns the builder for more chaining.
    pub fn not(mut self) -> Self {
        self.not = true;
        self
    }

    /// Consumes the builder and returns an `IfDefDirective` containing the symbol to be checked,
    /// then body, and other body.
    pub fn build(self) -> IfDefDirective {
        IfDefDirective {
            symbol: self.symbol,
            then: self.then,
            other: self.other,
            not: self.not,
        }
    }
}

/// Represents the `line` preprocessor directive in C.
///
/// # Examples
/// ```c
/// #line 10 "header_file"
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct LineDirective {
    /// The line number.
    pub line: u64,

    /// The path to the header file.
    pub path: String,

    /// Whether this is a system import.
    pub is_system: bool,

    /// The optional doc comment for the directive.
    pub doc: Option<DocComment>,
}

impl LineDirective {
    /// Creates and returns a new `LineDirectiveBuilder` to construct a `LineDirective` using the
    /// builder pattern.
    /// ```rust
    /// let line = LineDirective::new(/*line number*/, /*path to header file*/)
    ///     .build();
    /// ```
    pub fn new(line: u64, path: String) -> LineDirectiveBuilder {
        LineDirectiveBuilder::new(line, path)
    }
}

impl Format for LineDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        write!(fmt, "#line {} ", self.line)?;

        if self.is_system {
            writeln!(fmt, "<{}>", self.path)?;
        } else {
            writeln!(fmt, "\"{}\"", self.path)?;
        }

        Ok(())
    }
}

/// A builder for constructing a `LineDirective` instance.
pub struct LineDirectiveBuilder {
    line: u64,
    path: String,
    is_system: bool,
    doc: Option<DocComment>,
}

impl LineDirectiveBuilder {
    /// Creates and returns a new `LineDirectiveBuilder` to construct a `LineDirective` using the
    /// builder pattern.
    /// ```rust
    /// let line = LineDirective::new(/*line number*/, /*path to header file*/)
    ///     .build();
    /// ```
    pub fn new(line: u64, path: String) -> Self {
        Self {
            line,
            path,
            is_system: false,
            doc: None,
        }
    }

    /// Creates and returns a new `LineDirectiveBuilder` to construct a `LineDirective` with the
    /// given path string slice using the builder pattern
    pub fn new_with_str(line: u64, path: &str) -> Self {
        Self::new(line, path.to_string())
    }

    /// Sets the optional doc comment for this directive.
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Makes this a system import.
    pub fn system(mut self) -> Self {
        self.is_system = true;
        self
    }

    /// Consumes the buidler and returns a `LineDirective` containing the line number, and path to
    /// the header file.
    pub fn build(self) -> LineDirective {
        LineDirective {
            line: self.line,
            path: self.path,
            is_system: self.is_system,
            doc: self.doc,
        }
    }
}

/// Represents the `warning` preprocessor directive in C.
///
/// # Examples
/// ```c
/// #warning "warning message"
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct WarningDirective {
    /// The warning message.
    pub message: String,
}

impl WarningDirective {
    /// Creates and returns a new `WarningDirectiveBuilder` to construct a `WarningDirective` using
    /// the builder pattern.
    /// ```rust
    /// let warning = WarningDirective::new(/*warning message*/)
    ///     .build();
    /// ```
    pub fn new(message: String) -> WarningDirectiveBuilder {
        WarningDirectiveBuilder::new(message)
    }
}

impl Format for WarningDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#warning \"{}\"", self.message)
    }
}

/// A builder for constructing a `WarningDirective` instance.
pub struct WarningDirectiveBuilder {
    message: String,
}

impl WarningDirectiveBuilder {
    /// Creates and returns a new `WarningDirectiveBuilder` to construct a `WarningDirective` using
    /// the builder pattern.
    /// ```rust
    /// let warning = WarningDirective::new(/*warning message*/)
    ///     .build();
    /// ```
    pub fn new(message: String) -> Self {
        Self { message }
    }

    /// Creates and returns a new `WarningDirectiveBuilder` to construct a `WarningDirective` with
    /// the given message string slice using the builder pattern.
    pub fn new_with_str(message: &str) -> Self {
        Self::new(message.to_string())
    }

    /// Consumes the builder and returns a `WarningDirective` containing the warning message.
    pub fn build(self) -> WarningDirective {
        WarningDirective {
            message: self.message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn include() {
        let i = IncludeBuilder::new_with_str("./some_header.h")
            .doc(
                DocCommentBuilder::new()
                    .line_str("importing some_header")
                    .build(),
            )
            .build();
        let res = r#"/// importing some_header
#include "./some_header.h"
"#;
        assert_eq!(i.to_string(), res);

        let i2 = IncludeBuilder::new_system_with_str("stdio.h").build();
        let res = "#include <stdio.h>\n";
        assert_eq!(i2.to_string(), res);
    }

    #[test]
    fn error_directive() {
        let e = ErrorDirectiveBuilder::new_with_str("some kinda compile time error").build();
        let res = "#error \"some kinda compile time error\"\n";
        assert_eq!(e.to_string(), res);
    }

    #[test]
    fn pragma_directive() {
        let p = PragmaDirectiveBuilder::new_with_str("once").build();
        let res = "#pragma once\n";
        assert_eq!(p.to_string(), res);
    }

    #[test]
    fn macros() {
        let obj_m = Macro::Obj(
            ObjMacroBuilder::new_with_str("YEAR")
                .value_with_str("2025")
                .build(),
        );
        let res = "#define YEAR 2025\n";
        assert_eq!(obj_m.to_string(), res);

        let func_m = Macro::Func(
            FuncMacroBuilder::new_with_str("AREA")
                .param_with_str("width")
                .param_with_str("height")
                .value_with_str("(width) * (height)")
                .build(),
        );
        let res2 = "#define AREA(width, height) (width) * (height)\n";
        assert_eq!(func_m.to_string(), res2);

        let func_m2 = Macro::Func(
            FuncMacroBuilder::new_with_str("SOMETHING")
                .param_with_str("a")
                .param_with_str("b")
                .param_with_str("c")
                .value_with_str("abc\nabc\nanother")
                .build(),
        );
        let res3 = r#"#define SOMETHING(a, b, c) \
  abc \
  abc \
  another
"#;
        assert_eq!(func_m2.to_string(), res3);
    }

    #[test]
    fn if_directive() {
        let i = IfDirectiveBuilder::new_with_str("SOMETHING")
            .block_statement(Statement::Expr(Expr::new_ident_with_str("identifier")))
            .block_statement(Statement::ErrorDirective(
                ErrorDirectiveBuilder::new_with_str("some error").build(),
            ))
            .build();
        let res = r#"#if SOMETHING
identifier;
#error "some error"
#endif
"#;
        assert_eq!(i.to_string(), res);
    }

    #[test]
    fn if_def_directive() {
        let i = IfDefDirectiveBuilder::new_with_str("SOMETHING")
            .global_statement(GlobalStatement::NewLine)
            .not()
            .build();
        let res = r#"ifndef SOMETHING

#endif
"#;
        assert_eq!(i.to_string(), res);
    }

    #[test]
    fn line_directive() {
        let l = LineDirectiveBuilder::new_with_str(123, "hello.h").build();
        let res = "#line 123 \"hello.h\"\n";
        assert_eq!(l.to_string(), res);
    }

    #[test]
    fn warning_directive() {
        let l = WarningDirectiveBuilder::new_with_str("some warning message").build();
        let res = "#warning \"some warning message\"\n";
        assert_eq!(l.to_string(), res);
    }
}
