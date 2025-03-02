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

//! # Preprocessor Directives and C Macros
//!
//! This module provides comprehensive functionality for working with C preprocessor directives
//! and macros when generating C code from Rust. It includes support for various preprocessor
//! constructs such as `#include`, `#error`, `#pragma`, as well as object-like and function-like
//! macro definitions, and more.
//!
//! The module leverages builder patterns throughout to enable chainable APIs for
//! constructing these C language elements.

use std::fmt::{self, Write};

use crate::*;
use tamacro::DisplayFromFormat;

/// Represents the `#include` preprocessor directive in C.
///
/// This directive instructs the C preprocessor to include the contents of another file
/// in the current translation unit. The file can be specified as either a system header
/// (using angle brackets) or a user header (using double quotes).
///
/// # Examples
///
/// System header:
/// ```c
/// #include <stdio.h>
/// ```
///
/// Local/user header:
/// ```c
/// #include "my_header.h"
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Include {
    /// The path to the header file to be included.
    pub path: String,

    /// Determines the include style: `true` for system headers (`<header.h>`),
    /// `false` for user headers (`"header.h"`).
    pub is_system: bool,

    /// Optional documentation comment associated with this include directive.
    pub doc: Option<DocComment>,
}

impl Include {
    /// Creates a new `IncludeBuilder` to construct an `Include` using the builder pattern.
    ///
    /// By default, the include is created as a user header (with double quotes).
    /// Use additional builder methods to modify this behavior.
    ///
    /// # Parameters
    ///
    /// * `path` - The path to the header file
    ///
    /// # Returns
    ///
    /// An `IncludeBuilder` instance that can be used to build an `Include`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let include = Include::new("some_header.h".to_string())
    ///     .build();
    /// // Generates: #include "some_header.h"
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

/// A builder for constructing an `Include` instance with a fluent interface.
///
/// This builder allows for step-by-step construction of an `#include` directive with
/// various options like system vs. user header specification and documentation comments.
pub struct IncludeBuilder {
    path: String,
    is_system: bool,
    doc: Option<DocComment>,
}

impl IncludeBuilder {
    /// Creates a new `IncludeBuilder` with the specified header path.
    ///
    /// # Parameters
    ///
    /// * `path` - The path to the header file
    ///
    /// # Returns
    ///
    /// A new `IncludeBuilder` instance that, by default, is a user header include.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let include = IncludeBuilder::new("myheader.h".to_string())
    ///     .build();
    /// // Generates: #include "myheader.h"
    /// ```
    pub fn new(path: String) -> Self {
        Self {
            path,
            is_system: false,
            doc: None,
        }
    }

    /// Creates a new `IncludeBuilder` with the specified header path as a string slice.
    ///
    /// This is a convenience method that converts the string slice to a `String` internally.
    ///
    /// # Parameters
    ///
    /// * `path`: The path to the header file as a string slice
    ///
    /// # Returns
    ///
    /// A new `IncludeBuilder` instance that, by default, is a user header include.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let include = IncludeBuilder::new_with_str("myheader.h")
    ///     .build();
    /// // Generates: #include "myheader.h"
    /// ```
    pub fn new_with_str(path: &str) -> Self {
        Self {
            path: path.to_string(),
            is_system: false,
            doc: None,
        }
    }

    /// Creates a new `IncludeBuilder` for a system header include (with angle brackets).
    ///
    /// # Parameters
    ///
    /// * `path`: The path to the header file
    ///
    /// # Returns
    ///
    /// A new `IncludeBuilder` instance that is a system header include.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let include = IncludeBuilder::new_system("stdio.h".to_string())
    ///     .build();
    /// // Generates: #include <stdio.h>
    /// ```
    pub fn new_system(path: String) -> Self {
        Self {
            path,
            is_system: true,
            doc: None,
        }
    }

    /// Creates a new `IncludeBuilder` for a system header include with the path as a string slice.
    ///
    /// This is a convenience method that converts the provided string slice to a `String` internally.
    ///
    /// # Parameters
    ///
    /// * `path`: The path to the header file as a string slice
    ///
    /// # Returns
    ///
    /// A new `IncludeBuilder` instance that is a system header include.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let include = IncludeBuilder::new_system_with_str("stdio.h")
    ///     .build();
    /// // Generates: #include <stdio.h>
    /// ```
    pub fn new_system_with_str(path: &str) -> Self {
        Self {
            path: path.to_string(),
            is_system: true,
            doc: None,
        }
    }

    /// Sets the documentation comment for the include directive being built.
    ///
    /// The documentation comment will be placed before the include directive in the generated code.
    ///
    /// # Parameters
    ///
    /// * `doc` - The documentation comment to attach to the include
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let doc = DocComment::new().line("Standard I/O functions".to_string()).build();
    /// let include = IncludeBuilder::new_system_with_str("stdio.h")
    ///     .doc(doc)
    ///     .build();
    /// // Generates:
    /// // /// Standard I/O functions
    /// // #include <stdio.h>
    /// ```
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Finalizes the include and returns a fully constructed `Include`.
    ///
    /// This method consumes the builder and produces the final `Include` object
    /// with all the configured properties.
    ///
    /// # Returns
    ///
    /// A fully constructed `Include` instance
    pub fn build(self) -> Include {
        Include {
            path: self.path,
            is_system: self.is_system,
            doc: self.doc,
        }
    }
}

/// Represents the `#error` preprocessor directive in C.
///
/// This directive causes the C preprocessor to emit an error message with the specified text
/// and halt compilation. It's commonly used to enforce compile-time constraints or to indicate
/// unsupported configurations.
///
/// # Examples
///
/// ```c
/// #error "This platform is not supported"
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct ErrorDirective {
    /// The error message to be displayed when the directive is encountered.
    pub message: String,
}

impl ErrorDirective {
    /// Creates and returns a new `ErrorDirectiveBuilder` to construct an `ErrorDirective` using the builder pattern.
    ///
    /// # Parameters
    ///
    /// * `message` - The error message to be displayed
    ///
    /// # Returns
    ///
    /// A new `ParameterBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let error = ErrorDirective::new("Unsupported configuration".to_string())
    ///     .build();
    /// // Generates: #error "Unsupported configuration"
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

/// A builder for constructing a `Parameter` instance with a fluent interface.
///
/// This builder allows for step-by-step construction of an `#error` directive.
pub struct ErrorDirectiveBuilder {
    message: String,
}

impl ErrorDirectiveBuilder {
    /// Creates a new `ErrorDirectiveBuilder` with the specified error message.
    ///
    /// # Parameters
    ///
    /// * `message` - The message to be displayed
    ///
    /// # Returns
    ///
    /// A new `ErrorDirectiveBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let error = ErrorDirectiveBuilder::new("This code requires C11 or later".to_string())
    ///     .build();
    /// // Generates: #error "This code requires C11 or later"
    /// ```
    pub fn new(message: String) -> Self {
        Self { message }
    }

    /// Creates and returns a new `ErrorDirectiveBuilder` with the specified error message as a string slice.
    ///
    /// This is a convenience method that converts the string slice to a `String` internally.
    ///
    /// # Parameters
    ///
    /// * `message` - The message to be displayed as a string slice
    ///
    /// # Returns
    ///
    /// A new `ErrorDirectiveBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let error = ErrorDirectiveBuilder::new_with_str("This code requires C11 or later")
    ///     .build();
    /// // Generates: #error "This code requires C11 or later"
    /// ```
    pub fn new_with_str(message: &str) -> Self {
        Self::new(message.to_string())
    }

    /// Finalizes the parameter definition and returns a fully constructed `ErrorDirective`.
    ///
    /// This method consumes the builder and produces the final `ErrorDirective` object
    /// with the configured name and type.
    ///
    /// # Returns
    ///
    /// A fully constructed `ErrorDirective` instance
    pub fn build(self) -> ErrorDirective {
        ErrorDirective {
            message: self.message,
        }
    }
}

/// Represents a `#pragma` preprocessor directive in C.
///
/// The pragma directive provides a way to request special behavior from the compiler.
/// These are typically compiler-specific instructions that don't affect the semantics of
/// the program but rather how the compiler processes the code.
///
/// # Examples
///
/// ```c
/// #pragma once                                        // Include guard
/// #pragma pack(1)                                     // Control struct packing
/// #pragma GCC diagnostic ignored "-Wunused-variable"  // Control warnings
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct PragmaDirective {
    /// The raw pragma content that represents specific instructions or actions for the compiler.
    pub raw: String,
}

impl PragmaDirective {
    /// Creates and returns a new `PragmaDirectiveBuilder` to construct a `PragmaDirective` using the builder pattern.
    ///
    /// # Parameters
    ///
    /// * `raw` - The pragma content
    ///
    /// # Returns
    ///
    /// A new `PragmaDirectiveBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let pragma = PragmaDirective::new("once".to_string())
    ///     .build();
    /// // Generates: #pragma once
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

/// A builder for constructing a `PragmaDirective` with a fluent interface.
///
/// This builder allows for step-by-step construction of a `#pragma` directive.
pub struct PragmaDirectiveBuilder {
    raw: String,
}

impl PragmaDirectiveBuilder {
    /// Creates and returns a new `PragmaDirectiveBuilder` with the specified raw pragma content.
    ///
    /// # Parameters
    ///
    /// * `raw` - The pragma content
    ///
    /// # Returns
    ///
    /// A new `PragmaDirectiveBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let pragma = PragmaDirectiveBuilder::new("pack(1)".to_string())
    ///     .build();
    /// // Generates: #pragma pack(1)
    /// ```
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    /// Creates and returns a new `PragmaDirectiveBuilder` with the specified raw pragma content.
    ///
    /// This is a convenience method that converts the provided string slice to a String
    /// before delegating to the standard `new` method.
    ///
    /// # Parameters
    ///
    /// * `raw` - The pragma content as a string slice
    ///
    /// # Returns
    ///
    /// A new `PragmaDirectiveBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let pragma = PragmaDirectiveBuilder::new_with_str("pack(1)")
    ///     .build();
    /// // Generates: #pragma pack(1)
    /// ```
    pub fn new_with_str(raw: &str) -> Self {
        Self::new(raw.to_string())
    }

    /// Finalizes the parameter definition and returns a fully constructed `PragmaDirective`.
    ///
    /// This method consumes the builder and produces the final `PragmaDirective` object
    /// with the configured name and type.
    ///
    /// # Returns
    ///
    /// A fully constructed `PragmaDirective` instance
    pub fn build(self) -> PragmaDirective {
        PragmaDirective { raw: self.raw }
    }
}

/// Represents a macro definition in C, either object-like or function-like.
///
/// In C, macros are preprocessor directives that allow for text substitution before compilation.
/// There are two main types of macros:
/// - Object-like macros: Simple text substitutions
/// - Function-like macros: Text substitutions that take arguments
#[derive(Debug, Clone, DisplayFromFormat)]
pub enum Macro {
    /// An object-like macro definition, which substitutes a single token or block of text.
    Obj(ObjMacro),

    /// A function-like macro definition, which takes parameters and substitutes text based on those parameters.
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

/// Represents an object-like macro in C.
///
/// Object-like macros are simple text or token substitutions in C. They are defined with
/// the `#define` directive followed by an identifier and optionally a replacement value.
/// Object-like macros do not take parameters.
///
/// # Examples
///
/// ```c
/// #define PI 3.14159              // With a value
/// #define DEBUG                   // Without a value (often used for conditional compilation)
/// #define MESSAGE "Hello, world!" // String replacement
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct ObjMacro {
    /// The identifier name of the macro.
    pub name: String,

    /// The optional replacement value or code fragment.
    /// If None, the macro is defined without a value (commonly used for feature flags).
    pub value: Option<String>,

    /// Optional documentation comment associated with this macro.
    pub doc: Option<DocComment>,
}

impl ObjMacro {
    /// Creates and returns a new `ObjMacroBuilder` to construct an `ObjMacro` using the builder pattern.
    ///
    /// # Parameters
    ///
    /// * `name` - The identifier name of the macro
    ///
    /// # Returns
    ///
    /// A new `ObjMacroBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj_macro = ObjMacro::new("DEBUG".to_string())
    ///     .build();
    /// // Generates: #define DEBUG
    ///
    /// let obj_macro = ObjMacro::new("PI".to_string())
    ///     .value("3.14159".to_string())
    ///     .build();
    /// // Generates: #define PI 3.14159
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

/// A builder for constructing an `ObjMacro` with a fluent interface.
///
/// This builder allows for step-by-step construction of an object-like macro definition
/// with various options like replacement value and documentation comments.
pub struct ObjMacroBuilder {
    name: String,
    value: Option<String>,
    doc: Option<DocComment>,
}

impl ObjMacroBuilder {
    /// Creates and returns a new `ObjMacroBuilder` to construct an `ObjMacro` using the builder pattern.
    ///
    /// # Parameters
    ///
    /// * `name` - The identifier name of the macro
    ///
    /// # Returns
    ///
    /// A new `ObjMacroBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj_macro = ObjMacroBuilder::new("BUFFER_SIZE".to_string())
    ///     .value("1024".to_string())
    ///     .build();
    /// // Generates: #define BUFFER_SIZE 1024
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: None,
            doc: None,
        }
    }

    /// Creates a new `ObjMacroBuilder` with the specified macro name as a string slice.
    ///
    /// This is a convenience method that converts the string slice to a `String` internally.
    ///
    /// # Parameters
    ///
    /// * `name` - The identifier name of the macro as a string slice
    ///
    /// # Returns
    ///
    /// A new `ObjMacroBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj_macro = ObjMacroBuilder::new_with_str("BUFFER_SIZE")
    ///     .value("1024".to_string())
    ///     .build();
    /// // Generates: #define BUFFER_SIZE 1024
    /// ```
    pub fn new_with_str(name: &str) -> Self {
        Self::new(name.to_string())
    }

    /// Sets the documentation comment for the macro being built.
    ///
    /// The documentation comment will be placed before the macro in the generated code.
    ///
    /// # Parameters
    ///
    /// * `doc` - The documentation comment to attach to the macro
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let doc = DocComment::new().line("Maximum buffer size in bytes".to_string()).build();
    /// let obj_macro = ObjMacroBuilder::new_with_str("BUFFER_SIZE")
    ///     .doc(doc)
    ///     .value("1024".to_string())
    ///     .build();
    /// // Generates:
    /// // /* Maximum buffer size in bytes */
    /// // #define BUFFER_SIZE 1024
    /// ```
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Sets the replacement value for the macro.
    ///
    /// # Parameters
    ///
    /// * `value` - The replace value or code fragment
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj_macro = ObjMacroBuilder::new_with_str("VERSION")
    ///     .value("\"1.2.3\"".to_string())
    ///     .build();
    /// // Generates: #define VERSION "1.2.3"
    /// ```
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    /// Sets the replacement value for the macro using a string slice.
    ///
    /// This is a convenience method that converts the string slice to a `String` internally.
    ///
    /// # Parameters
    ///
    /// * `value` - The replace value or code fragment
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj_macro = ObjMacroBuilder::new_with_str("VERSION")
    ///     .value_with_str("\"1.2.3\"")
    ///     .build();
    /// // Generates: #define VERSION "1.2.3"
    /// ```
    pub fn value_with_str(self, value: &str) -> Self {
        self.value(value.to_string())
    }

    /// Finalizes the function definition and returns a fully constructed `ObjMacro`.
    ///
    /// This method consumes the builder and produces the final `ObjMacro` object
    /// with all the configured properties.
    ///
    /// # Returns
    ///
    /// A fully constructed `ObjMacro` instance
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
/// Function-like macros are preprocessor definitions that take parameters and perform
/// text substitution based on those parameters. They are defined with the `#define` directive
/// followed by an identifier, a parameter list in parentheses, and a replacement value.
///
/// # Examples
///
/// ```c
/// #define MAX(a, b) ((a) > (b) ? (a) : (b))
/// #define PRINT_DEBUG(msg) printf("DEBUG: %s\n", msg)
/// #define SUM_VARARGS(count, ...) sum_function(count, __VA_ARGS__)
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct FuncMacro {
    /// The identifier name of the macro.
    pub name: String,

    /// The list of parameter names for the macro.
    /// For variadic macros, the last parameter can be "..." to indicate variable arguments.
    pub params: Vec<String>,

    /// The replacement body/value of the function-like macro.
    pub value: String,

    /// Optional documentation comment associated with this macro.
    pub doc: Option<DocComment>,
}

impl FuncMacro {
    /// Creates and returns a new `FuncMacroBuilder` to construct a `FuncMacro` using the builder pattern.
    ///
    /// # Parameters
    ///
    /// * `name` - The identifier name of the macro
    ///
    /// # Returns
    ///
    /// A new `FuncMacroBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let func_macro = FuncMacro::new("MAX".to_string())
    ///     .param_with_str("a")
    ///     .param_with_str("b")
    ///     .value_with_str("((a) > (b) ? (a) : (b))")
    ///     .build();
    /// // Generates: #define MAX(a, b) ((a) > (b) ? (a) : (b))
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

        for param in &self.params[..self.params.len().saturating_sub(1)] {
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

/// A builder for constructing a `FuncMacro` with a fluent interface.
///
/// This builder allows for step-by-step construction of a function-like macro definition
/// with various options like parameters, replacement value, and documentation comments.
pub struct FuncMacroBuilder {
    name: String,
    params: Vec<String>,
    value: String,
    doc: Option<DocComment>,
}

impl FuncMacroBuilder {
    /// Creates and returns a new `FuncMacroBuilder` to construct a `FuncMacro` using the builder pattern.
    ///
    /// # Parameters
    ///
    /// * `name` - The identifier name of the macro
    ///
    /// # Returns
    ///
    /// A new `FuncMacroBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let func_macro = FuncMacroBuilder::new("MIN".to_string())
    ///     .param_with_str("a")
    ///     .param_with_str("b")
    ///     .value_with_str("((a) < (b) ? (a) : (b))")
    ///     .build();
    /// // Generates: #define MIN(a, b) ((a) < (b) ? (a) : (b))
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: vec![],
            value: "".to_string(),
            doc: None,
        }
    }

    /// Creates and returns a new `FuncMacroBuilder` with the specified macro name as a string slice.
    ///
    /// This is a convenience method that converts the string slice to a `String` internally.
    ///
    /// # Parameters
    ///
    /// * `name` - The identifier name of the macro as a string slice
    ///
    /// # Returns
    ///
    /// A new `FuncMacroBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let func_macro = FuncMacroBuilder::new_with_str("MIN")
    ///     .param_with_str("a")
    ///     .param_with_str("b")
    ///     .value_with_str("((a) < (b) ? (a) : (b))")
    ///     .build();
    /// // Generates: #define MIN(a, b) ((a) < (b) ? (a) : (b))
    /// ```
    pub fn new_with_str(name: &str) -> Self {
        Self::new(name.to_string())
    }

    /// Sets the documentation comment for the macro being built.
    ///
    /// # Parameters
    ///
    /// * `doc` - The documentation comment to attach to the macro
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let doc = DocComment::new().line_str("Returns the minimum of two values").build();
    /// let func_macro = FuncMacroBuilder::new_with_str("MIN")
    ///     .doc(doc)
    ///     .param_with_str("a")
    ///     .param_with_str("b")
    ///     .value_with_str("((a) < (b) ? (a) : (b))")
    ///     .build();
    /// // Generates:
    /// // /// Returns the minimum of two values
    /// // #define MIN(a, b) ((a) < (b) ? (a) : (b))
    /// ```
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Adds a parameter to the function-like macro's parameter list.
    ///
    /// # Parameters
    ///
    /// * `param` - The parameter name
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let func_macro = FuncMacroBuilder::new_with_str("SQUARE")
    ///     .param("x".to_string())
    ///     .value_with_str("((x) * (x))")
    ///     .build();
    /// // Generates: #define SQUARE(x) ((x) * (x))
    /// ```
    pub fn param(mut self, param: String) -> Self {
        self.params.push(param);
        self
    }

    /// Adds a parameter to the function-like macro's parameter list using a string slice.
    ///
    /// This is a convenience method that converts the string slice to a `String` internally.
    ///
    /// # Parameters
    ///
    /// * `param` - The parameter name as a string slice
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let func_macro = FuncMacroBuilder::new_with_str("SQUARE")
    ///     .param_with_str("x")
    ///     .value_with_str("((x) * (x))")
    ///     .build();
    /// // Generates: #define SQUARE(x) ((x) * (x))
    /// ```
    pub fn param_with_str(self, param: &str) -> Self {
        self.param(param.to_string())
    }

    /// Sets the complete list of parameters for the function-like macro.
    ///
    /// This replaces any previously added parameters with the provided list.
    ///
    /// # Parameters
    ///
    /// * `params` - A vector of parameters
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let params = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    /// let func_macro = FuncMacroBuilder::new_with_str("SUM3")
    ///     .params(params)
    ///     .value_with_str("((a) + (b) + (c))")
    ///     .build();
    /// // Generates: #define SUM3(a, b, c) ((a) + (b) + (c))
    /// ```
    pub fn params(mut self, params: Vec<String>) -> Self {
        self.params = params;
        self
    }

    /// Adds a variadic argument symbol ("...") to the parameter list, creating a variadic macro.
    ///
    /// This is used to define macros that can accept a variable number of arguments.
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let func_macro = FuncMacroBuilder::new_with_str("PRINTF_WRAPPER")
    ///     .param_with_str("fmt")
    ///     .variadic_arg()
    ///     .value_with_str("printf(fmt, __VA_ARGS__)")
    ///     .build();
    /// // Generates: #define PRINTF_WRAPPER(fmt, ...) printf(fmt, __VA_ARGS__)
    /// ```
    pub fn variadic_arg(self) -> Self {
        self.param_with_str("...")
    }

    /// Sets the replacement body/value of the function-like macro.
    ///
    /// # Parameters
    ///
    /// * `value` - The replacement body/value of the macro
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let func_macro = FuncMacroBuilder::new_with_str("CUBE")
    ///     .param_with_str("x")
    ///     .value("((x) * (x) * (x))".to_string())
    ///     .build();
    /// // Generates: #define CUBE(x) ((x) * (x) * (x))
    /// ```
    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    /// Sets the replacement body/value of the function-like macro using a string slice.
    ///
    /// This is a convenience method that converts the string slice to a `String` internally.
    ///
    /// # Parameters
    ///
    /// * `value` - The replacement body/value of the macro as a string slice
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let func_macro = FuncMacroBuilder::new_with_str("CUBE")
    ///     .param_with_str("x")
    ///     .value_with_str("((x) * (x) * (x))")
    ///     .build();
    /// // Generates: #define CUBE(x) ((x) * (x) * (x))
    /// ```
    pub fn value_with_str(self, value: &str) -> Self {
        self.value(value.to_string())
    }

    /// Finalizes the function definition and returns a fully constructed `FuncMacro`.
    ///
    /// This method consumes the builder and produces the final `FuncMacro` object
    /// with all the configured properties.
    ///
    /// # Returns
    ///
    /// A fully constructed `FuncMacro` instance
    pub fn build(self) -> FuncMacro {
        FuncMacro {
            name: self.name,
            params: self.params,
            value: self.value,
            doc: self.doc,
        }
    }
}

/// Represents a container that holds either a `Scope` or a `Block`.
///
/// This enum is used throughout the preprocessor directive system to allow
/// flexibility in how directive bodies are represented. A directive's body
/// can either be a full scope (containing global statements) or a block
/// (containing regular statements).
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

/// Represents the C preprocessor `#if` directive.
///
/// The `#if` directive conditionally includes or excludes code based on a compile-time
/// expression evaluation. If the condition evaluates to non-zero (true), the code
/// in the "then" block is included; otherwise, if an `#else` directive exists,
/// the code in the "other" block is included.
///
/// # Examples
/// ```c
/// // Simple conditional inclusion
/// #if ENABLE_FEATURE
///   feature_function();
/// #endif
///
/// // Conditional inclusion with else
/// #if DEBUG_LEVEL > 2
///   log_debug("Detailed info");
/// #else
///   log_info("Basic info");
/// #endif
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct IfDirective {
    /// The condition expression to be evaluated at compile-time.
    /// This can be any valid C preprocessor expression.
    pub cond: String,

    /// The code block to include if the condition evaluates to true (non-zero).
    pub then: ScopeOrBlock,

    /// The optional code block to include if the condition evaluates to false (zero).
    /// This corresponds to the `#else` part of the directive.
    pub other: Option<ScopeOrBlock>,
}

impl IfDirective {
    /// Creates and returns a new `IfDirectiveBuilder` to construct an `IfDirective` using the
    /// builder pattern.
    ///
    /// # Parameters
    /// * `cond` - The condition expression as a String.
    ///
    /// # Returns
    /// A new `IfDirectiveBuilder` instance with the specified condition.
    ///
    /// # Examples
    /// ```rust
    /// let if_dir = IfDirective::new("PLATFORM == WINDOWS".to_string())
    ///     .then(windows_specific_scope)
    ///     .other(non_windows_scope)
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

/// A builder for constructing a complete `IfDirective` instance.
///
/// This builder provides methods to incrementally construct an `IfDirective` with
/// various components including condition, then body, and optional else body.
/// The builder pattern allows for a fluent API with method chaining.
pub struct IfDirectiveBuilder {
    cond: String,
    then: ScopeOrBlock,
    other: Option<ScopeOrBlock>,
}

impl IfDirectiveBuilder {
    /// Creates and returns a new `IfDirectiveBuilder` with the specified condition.
    ///
    /// # Parameters
    /// * `cond` - The condition expression as a String.
    ///
    /// # Returns
    /// A new `IfDirectiveBuilder` instance with default empty scope for the then block
    /// and no else block.
    ///
    /// # Examples
    /// ```rust
    /// let if_dir = IfDirectiveBuilder::new("VERSION >= 3".to_string())
    ///     .then(version3_scope)
    ///     .other(legacy_version_scope)
    ///     .build();
    /// ```
    pub fn new(cond: String) -> Self {
        Self {
            cond,
            then: ScopeOrBlock::Scope(Scope::new().build()),
            other: None,
        }
    }

    /// Creates and returns a new `IfDirectiveBuilder` with the specified condition as a string slice.
    ///
    /// This is a convenience method that converts the string slice to a String internally.
    ///
    /// # Parameters
    /// * `cond` - The condition expression as a string slice
    ///
    /// # Returns
    /// A new `IfDirectiveBuilder` instance with the specified condition
    pub fn new_with_str(cond: &str) -> Self {
        Self::new(cond.to_string())
    }

    /// Appends a global statement to the then body and returns the builder for more chaining.
    ///
    /// If the current `then` value is a `Block`, it will be replaced with a new `Scope`
    /// containing the provided global statement.
    ///
    /// # Parameters
    /// * `global_stmt` - The global statement to append.
    ///
    /// # Returns
    /// The builder instance for method chaining.
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
    ///
    /// If the current `then` value is a `Scope`, it will be replaced with a new `Block`
    /// containing the provided statement.
    ///
    /// # Parameters
    /// * `stmt` - The statement to append.
    ///
    /// # Returns
    /// The builder instance for method chaining
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
    ///
    /// # Parameters
    /// * `then` - The `ScopeOrBlock` to use as the then body.
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn then(mut self, then: ScopeOrBlock) -> Self {
        self.then = then;
        self
    }

    /// Sets the optional else body and returns the builder for more chaining.
    ///
    /// # Parameters
    /// * `other` - The `ScopeOrBlock` to use as the else body.
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn other(mut self, other: ScopeOrBlock) -> Self {
        self.other = Some(other);
        self
    }

    /// Consumes the builder and returns a fully constructed `IfDirective`.
    ///
    /// # Returns
    /// An `IfDirective` instance with all the components set during the building process.
    pub fn build(self) -> IfDirective {
        IfDirective {
            cond: self.cond,
            then: self.then,
            other: self.other,
        }
    }
}

/// Represents the C preprocessor `#ifdef` and `#ifndef` directives.
///
/// These directives conditionally include or exclude code based on whether
/// a macro is defined (`#ifdef`) or not defined (`#ifndef`). They are simplified
/// forms of the `#if` directive specialized for checking macro definitions.
///
/// # Examples
/// ```c
/// // Include code if DEBUG is defined
/// #ifdef DEBUG
///   printf("Debug mode is enabled\n");
/// #endif
///
/// // Include code if NDEBUG is not defined
/// #ifndef NDEBUG
///   assert(x > 0);
/// #else
///   // Skip assertions in release mode
/// #endif
///
/// // Check for platform-specific macros
/// #ifdef _WIN32
///   #include <windows.h>
/// #elif defined(__APPLE__)
///   #include <CoreFoundation/CoreFoundation.h>
/// #else
///   #include <unistd.h>
/// #endif
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct IfDefDirective {
    /// The macro symbol name to check for definition.
    pub symbol: String,

    /// The code block to include if the condition is met
    /// (symbol is defined for `#ifdef`, not defined for `#ifndef`).
    pub then: ScopeOrBlock,

    /// The optional code block to include if the condition is not met.
    /// This corresponds to the `#else` part of the directive.
    pub other: Option<ScopeOrBlock>,

    /// Whether this is a `#ifndef` (true) or `#ifdef` (false) directive.
    pub not: bool,
}

impl IfDefDirective {
    /// Creates and returns a new `IfDefDirectiveBuilder` to construct an `IfDefDirective` using
    /// the builder pattern.
    ///
    /// This method initiates the builder pattern for constructing an `#ifdef` directive.
    /// Use the `.not()` method on the builder to create an `#ifndef` directive instead.
    ///
    /// # Parameters
    /// * `symbol` - The macro symbol name to check.
    ///
    /// # Returns
    /// A new `IfDefDirectiveBuilder` instance
    ///
    /// # Examples
    /// ```rust
    /// let ifdef = IfDefDirective::new("DEBUG".to_string())
    ///     .then(debug_scope)
    ///     .other(release_scope)
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

/// A builder for constructing a complete `IfDefDirective` instance.
///
/// This builder provides methods to incrementally construct an `IfDefDirective` with
/// various components including the symbol to check, then body, optional else body,
/// and whether it's an `#ifndef` directive.
pub struct IfDefDirectiveBuilder {
    symbol: String,
    then: ScopeOrBlock,
    other: Option<ScopeOrBlock>,
    not: bool,
}

impl IfDefDirectiveBuilder {
    /// Creates and returns a new `IfDefDirectiveBuilder` with the specified symbol.
    ///
    /// # Parameters
    /// * `symbol` - The macro symbol name to check.
    ///
    /// # Returns
    /// A new `IfDefDirectiveBuilder` instance configured for an `#ifdef` directive
    /// with default empty scope for the then block and no else block.
    ///
    /// # Examples
    /// ```rust
    /// let ifdef = IfDefDirectiveBuilder::new("FEATURE_ENABLED".to_string())
    ///     .then(feature_code)
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

    /// Creates and returns a new `IfDefDirectiveBuilder` with the specified symbol as a string slice.
    ///
    /// This is a convenience method that converts the string slice to a String internally.
    ///
    /// # Parameters
    /// * `symbol` - The macro symbol name to check as a string slice
    ///
    /// # Returns
    /// A new `IfDefDirectiveBuilder` instance with the specified symbol.
    pub fn new_with_str(symbol: &str) -> Self {
        Self::new(symbol.to_string())
    }

    /// Appends a global statement to the then body and returns the builder for more chaining.
    ///
    /// If the current `then` value is a `Block`, it will be replaced with a new `Scope`
    /// containing the provided global statement.
    ///
    /// # Parameters
    /// * `global_stmt` - The global statement to append.
    ///
    /// # Returns
    /// The builder instance for method chaining.
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
    ///
    /// If the current `then` value is a `Scope`, it will be replaced with a new `Block`
    /// containing the provided statement.
    ///
    /// # Parameters
    /// * `stmt` - The statement to append.
    ///
    /// # Returns
    /// The builder instance for method chaining
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
    ///
    /// # Parameters
    /// * `then` - The `ScopeOrBlock` to use as the then body.
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn then(mut self, then: ScopeOrBlock) -> Self {
        self.then = then;
        self
    }

    /// Sets the optional else body and returns the builder for more chaining.
    ///
    /// # Parameters
    /// * `other` - The `ScopeOrBlock` to use as the else body.
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn other(mut self, other: ScopeOrBlock) -> Self {
        self.other = Some(other);
        self
    }

    /// Configures the builder to create an `#ifndef` directive instead of an `#ifdef` directive.
    ///
    /// # Returns
    /// The builder instance for method chaining.
    ///
    /// # Examples
    /// ```rust
    /// let ifndef = IfDefDirectiveBuilder::new("NDEBUG".to_string())
    ///     .not()  // Make it #ifndef instead of #ifdef
    ///     .then(debug_assertions)
    ///     .build();
    /// ```
    pub fn not(mut self) -> Self {
        self.not = true;
        self
    }

    /// Consumes the builder and returns a fully constructed `IfDefDirective`.
    ///
    /// # Returns
    /// An `IfDefDirective` instance with all the components set during the building process.
    pub fn build(self) -> IfDefDirective {
        IfDefDirective {
            symbol: self.symbol,
            then: self.then,
            other: self.other,
            not: self.not,
        }
    }
}

/// Represents the C preprocessor `#line` directive.
///
/// The `#line` directive changes the compiler's internal line number counter and
/// file name for error reporting and debugging information. This is commonly used
/// in generated code to make error messages refer to the original source file
/// rather than the generated file.
///
/// # Examples
/// ```c
/// // Set line number to 10 and file name to "header_file"
/// #line 10 "header_file"
/// void function_with_line_info();
///
/// // Can be used for better error reporting in generated code
/// #line 157 "original_template.h"
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct LineDirective {
    /// The line number to set in the compiler's internal counter.
    pub line: u64,

    /// The path or name of the file to set in the compiler's internal state.
    pub path: String,

    /// The optional documentation comment for this directive.
    pub doc: Option<DocComment>,
}

impl LineDirective {
    /// Creates and returns a new `LineDirectiveBuilder` to construct a `LineDirective` using the
    /// builder pattern.
    ///
    /// # Parameters
    /// * `line` - The line number to set.
    /// * `path` - The path or file name to set as a String.
    ///
    /// # Returns
    /// A new `LineDirectiveBuilder` instance with the specified line number and path.
    ///
    /// # Examples
    /// ```rust
    /// let line_dir = LineDirective::new(42, "original_file.c".to_string())
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
        writeln!(fmt, "\"{}\"", self.path)
    }
}

/// A builder for constructing a complete `LineDirective` instance.
///
/// This builder provides methods to incrementally construct a `LineDirective` with
/// various components including line number, path, system path status, and documentation.
pub struct LineDirectiveBuilder {
    line: u64,
    path: String,
    doc: Option<DocComment>,
}

impl LineDirectiveBuilder {
    /// Creates and returns a new `LineDirectiveBuilder` with the specified line number and path.
    ///
    /// # Parameters
    /// * `line` - The line number to set.
    /// * `path` - The path or file name to set as a String.
    ///
    /// # Returns
    /// A new `LineDirectiveBuilder` instance with the specified line number and path,
    /// configured for a regular (non-system) file path with no documentation.
    ///
    /// # Examples
    /// ```rust
    /// let line_dir = LineDirectiveBuilder::new(100, "source.c".to_string())
    ///     .build();
    /// ```
    pub fn new(line: u64, path: String) -> Self {
        Self {
            line,
            path,
            doc: None,
        }
    }

    /// Creates and returns a new `LineDirectiveBuilder` with the specified line number and path as a string slice.
    ///
    /// This is a convenience method that converts the string slice to a String internally.
    ///
    /// # Parameters
    /// * `line` - The line number to set.
    /// * `path` - The path or file name to set as a string slice
    ///
    /// # Returns
    /// A new `LineDirectiveBuilder` instance with the specified line number and path
    pub fn new_with_str(line: u64, path: &str) -> Self {
        Self::new(line, path.to_string())
    }

    /// Sets the optional documentation comment for this directive.
    ///
    /// # Parameters
    /// * `doc` - The documentation comment to associate with this directive
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Consumes the builder and returns a fully constructed `LineDirective`.
    ///
    /// # Returns
    /// A `LineDirective` instance with all the components set during the building process.
    pub fn build(self) -> LineDirective {
        LineDirective {
            line: self.line,
            path: self.path,
            doc: self.doc,
        }
    }
}

/// Represents the C preprocessor `#warning` directive.
///
/// The `#warning` directive causes the compiler to issue a warning with the
/// specified message during compilation. This is useful for alerting developers
/// about potential issues, deprecated features, or implementation notes without
/// stopping the compilation process.
///
/// This directive was standardized in C99, but has been widely supported as
/// a compiler extension in earlier C standards.
///
/// # Examples
/// ```c
/// // Simple warning
/// #warning "This code is experimental"
///
/// // Warning about deprecated features
/// #ifdef USING_DEPRECATED_API
///   #warning "The XYZ API is deprecated and will be removed in version 2.0"
/// #endif
///
/// // Compiler-specific feature warning
/// #if defined(__GNUC__) && !defined(__clang__)
///   #warning "This optimization only works with GCC"
/// #endif
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct WarningDirective {
    /// The warning message that will be displayed by the compiler.
    pub message: String,
}

impl WarningDirective {
    /// Creates and returns a new `WarningDirectiveBuilder` to construct a `WarningDirective` using
    /// the builder pattern.
    ///
    /// # Parameters
    /// * `message` - The warning message
    ///
    /// # Returns
    /// A new `WarningDirectiveBuilder` instance with the specified message.
    ///
    /// # Examples
    /// ```rust
    /// let warning = WarningDirective::new("Feature will be deprecated soon".to_string())
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
///
/// This builder provides methods to construct a `WarningDirective` with
/// the specified warning message.
pub struct WarningDirectiveBuilder {
    message: String,
}

impl WarningDirectiveBuilder {
    /// Creates and returns a new `WarningDirectiveBuilder` with the specified message.
    ///
    /// # Parameters
    /// * `message` - The warning message as a String.
    ///
    /// # Returns
    /// A new `WarningDirectiveBuilder` instance with the specified message.
    ///
    /// # Examples
    /// ```rust
    /// let warning = WarningDirectiveBuilder::new("Compatibility issues may occur".to_string())
    ///     .build();
    /// ```
    pub fn new(message: String) -> Self {
        Self { message }
    }

    /// Creates and returns a new `WarningDirectiveBuilder` with the specified message as a string slice.
    ///
    /// This is a convenience method that converts the string slice to a String internally.
    ///
    /// # Parameters
    /// * `message` - The warning message as a string slice
    ///
    /// # Returns
    /// A new `WarningDirectiveBuilder` instance with the specified message
    pub fn new_with_str(message: &str) -> Self {
        Self::new(message.to_string())
    }

    /// Consumes the builder and returns a fully constructed `WarningDirective`.
    ///
    /// # Returns
    /// A `WarningDirective` instance with the warning message set during construction.
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
