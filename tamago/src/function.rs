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

//! This module provides comprehensive tools for creating and managing C functions in Rust.
//!
//! It allows for programmatic generation of C function declarations and definitions
//! with support for various function attributes, parameters, return types, and documentation.
//! This is particularly useful for generating C code or bindings from Rust.

use std::fmt::{self, Write};

use crate::{Block, DocComment, Format, Formatter, Statement, Type};
use tamacro::DisplayFromFormat;

/// Represents a C function with all its components and attributes.
///
/// The `Function` struct enables the creation of complete C function declarations
/// and definitions, supporting inline, static, and extern modifiers, documentation,
/// parameter lists, return types, and function bodies.
///
/// # Examples
///
/// Basic function with a return value:
/// ```c
/// int main(void) {
///   return 0;
/// }
/// ```
///
/// Function with parameters and modifiers:
/// ```c
/// static inline double calculate_area(double width, double height) {
///   return width * height;
/// }
/// ```
///
/// External function declaration:
/// ```c
/// extern void process_data(char* buffer, size_t length);
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Function {
    /// The name of the function
    pub name: String,

    /// The parameters of the function as a vector of Parameter objects
    pub params: Vec<Parameter>,

    /// The return type of the function
    pub ret: Type,

    /// Whether the function is declared with the 'inline' keyword
    pub is_inline: bool,

    /// Whether the function is declared with the 'static' keyword
    pub is_static: bool,

    /// Whether the function is declared with the 'extern' keyword
    pub is_extern: bool,

    /// The body of the function represented as a Block of statements
    pub body: Block,

    /// The optional documentation comment for the function
    pub doc: Option<DocComment>,
}

impl Function {
    /// Creates and returns a new `FunctionBuilder` to construct a `Function` using the builder pattern.
    ///
    /// This method simplifies the process of creating complex function declarations
    /// by providing an interface for defining all function components.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the function
    /// * `ret` - The return type of the function
    ///
    /// # Returns
    ///
    /// A `FunctionBuilder` instance that can be used to configure and build a `Function`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let func = Function::new("calculate_sum".to_string(), Type::new(BaseType::Int).build())
    ///     .params(vec![
    ///         Parameter::new("a".to_string(), Type::new(BaseType::Int).build()),
    ///         Parameter::new("b".to_string(), Type::new(BaseType::Int).build())
    ///     ])
    ///     .body(Block::new()
    ///         .statement(Statement::Return(Some(Expr::Binary {
    ///             left: Box::new(Expr::Ident("a".to_string())),
    ///             op: BinOp::Add,
    ///             right: Box::new(Expr::Ident("b".to_string()))
    ///         })))
    ///         .build())
    ///     .build();
    /// ```
    pub fn new(name: String, ret: Type) -> FunctionBuilder {
        FunctionBuilder::new(name, ret)
    }
}

impl Format for Function {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        if self.body.stmts.is_empty() && self.is_extern {
            write!(fmt, "extern ")?;
        }

        if self.is_static {
            write!(fmt, "static ")?;
        }

        if self.is_inline {
            write!(fmt, "inline ")?;
        }

        self.ret.format(fmt)?;
        write!(fmt, " ")?;

        write!(fmt, "{}(", self.name)?;
        if self.params.is_empty() {
            write!(fmt, "void")?;
        } else if !self.params.is_empty() {
            for param in &self.params[..self.params.len() - 1] {
                param.format(fmt)?;
                write!(fmt, ", ")?;
            }

            if let Some(last) = self.params.last() {
                last.format(fmt)?;
            }
        }

        write!(fmt, ")")?;

        if !self.body.stmts.is_empty() && !self.is_extern {
            fmt.block(|fmt| self.body.format(fmt))?;
            writeln!(fmt)
        } else {
            writeln!(fmt, ";")
        }
    }
}

/// A builder for constructing a `Function` instance with a fluent interface.
///
/// The `FunctionBuilder` provides a step-by-step approach to creating complex C functions
/// by allowing incremental configuration of all function attributes and components.
/// This helps ensure that function declarations are consistent and well-formed.
pub struct FunctionBuilder {
    name: String,
    params: Vec<Parameter>,
    ret: Type,
    is_inline: bool,
    is_static: bool,
    is_extern: bool,
    body: Block,
    doc: Option<DocComment>,
}

impl FunctionBuilder {
    /// Creates and returns a new `FunctionBuilder` with the specified name and return type.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the function as a String
    /// * `ret` - The return type of the function
    ///
    /// # Returns
    ///
    /// A new `FunctionBuilder` instance with default values for other properties
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = FunctionBuilder::new("process_data".to_string(), Type::new(BaseType::Void).build());
    /// ```
    pub fn new(name: String, ret: Type) -> Self {
        Self {
            name,
            ret,
            params: vec![],
            is_inline: false,
            is_static: false,
            is_extern: false,
            body: Block::new().build(),
            doc: None,
        }
    }

    /// Creates and returns a new `FunctionBuilder` using a string slice for the name.
    ///
    /// This is a convenience method that converts the provided string slice to a String
    /// before delegating to the standard `new` method.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the function as a string slice
    /// * `ret` - The return type of the function
    ///
    /// # Returns
    ///
    /// A new `FunctionBuilder` instance with the specified name and return type
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = FunctionBuilder::new_with_str("get_value", Type::new(BaseType::Int).build());
    /// ```
    pub fn new_with_str(name: &str, ret: Type) -> Self {
        Self::new(name.to_string(), ret)
    }

    /// Sets the documentation comment for the function being built.
    ///
    /// # Parameters
    ///
    /// * `doc` - The documentation comment to attach to the function
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = FunctionBuilder::new_with_str("calculate_area", Type::new(BaseType::Double).build())
    ///     .doc(DocComment::new("Calculates the area of a rectangle").build());
    /// ```
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Makes the function inline by setting the `is_inline` flag to true.
    ///
    /// In C, the `inline` keyword provides a hint to the compiler that calls to this function
    /// should be replaced with the function body when possible.
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = FunctionBuilder::new_with_str("add", Type::new(BaseType::Int).build())
    ///     .make_inline();
    /// ```
    pub fn make_inline(mut self) -> Self {
        self.is_inline = true;
        self
    }

    /// Makes the function static by setting the `is_static` flag to true.
    ///
    /// In C, the `static` keyword for functions limits the visibility of the function
    /// to the file in which it is defined.
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = FunctionBuilder::new_with_str("helper_function", Type::new(BaseType::Void).build())
    ///     .make_static();
    /// ```
    pub fn make_static(mut self) -> Self {
        self.is_static = true;
        self
    }

    /// Makes the function extern by setting the `is_extern` flag to true.
    ///
    /// In C, the `extern` keyword indicates that the function is defined elsewhere,
    /// typically in another compilation unit.
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = FunctionBuilder::new_with_str("external_api", Type::new(BaseType::Int).build())
    ///     .make_extern();
    /// ```
    pub fn make_extern(mut self) -> Self {
        self.is_extern = true;
        self
    }

    /// Sets the body block for the function being built.
    ///
    /// # Parameters
    ///
    /// * `body` - The block of statements that form the function body
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let body = Block::new()
    ///     .statement(Statement::Return(Some(Expr::Int(0))))
    ///     .build();
    /// let builder = FunctionBuilder::new_with_str("main", Type::new(BaseType::Int).build())
    ///     .body(body);
    /// ```
    pub fn body(mut self, body: Block) -> Self {
        self.body = body;
        self
    }

    /// Appends a statement to the function's body block.
    ///
    /// This is a convenience method that adds a single statement to the body
    /// without requiring a separate Block construction.
    ///
    /// # Parameters
    ///
    /// * `stmt` - The statement to append to the function body
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = FunctionBuilder::new_with_str("print_message", Type::new(BaseType::Void).build())
    ///     .statement(Statement::Expression(Expr::FuncCall {
    ///         func: "printf".to_string(),
    ///         args: vec![Expr::String("Hello, World!\\n".to_string())]
    ///     }));
    /// ```
    pub fn statement(mut self, stmt: Statement) -> Self {
        self.body.stmts.push(stmt);
        self
    }

    /// Appends a new line to the function's body block.
    ///
    /// This method adds a special `NewLine` statement that will be rendered as a blank line
    /// in the generated C code, which can be useful for separating logical sections
    /// of code to improve readability.
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = FunctionBuilder::new_with_str("complex_function", Type::new(BaseType::Void).build())
    ///     .statement(Statement::Declaration {
    ///         name: "x".to_string(),
    ///         t: Type::new(BaseType::Int).build(),
    ///         init: Some(Expr::Int(0))
    ///     })
    ///     .new_line()
    ///     .statement(Statement::Expression(Expr::FuncCall {
    ///         func: "process".to_string(),
    ///         args: vec![Expr::Ident("x".to_string())]
    ///     }));
    /// ```
    pub fn new_line(mut self) -> Self {
        self.body.stmts.push(Statement::NewLine);
        self
    }

    /// Appends a parameter to the function being built.
    ///
    /// # Parameters
    ///
    /// * `param` - The parameter to add to the function signature
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let param = Parameter::new("value".to_string(), Type::new(BaseType::Int).build()).build();
    /// let builder = FunctionBuilder::new_with_str("increment", Type::new(BaseType::Int).build())
    ///     .param(param);
    /// ```
    pub fn param(mut self, param: Parameter) -> Self {
        self.params.push(param);
        self
    }

    /// Sets the complete list of parameters for the function being built.
    ///
    /// This method replaces any previously added parameters with the provided list.
    ///
    /// # Parameters
    ///
    /// * `params` - A vector of Parameter objects representing the function parameters
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let params = vec![
    ///     Parameter::new("x".to_string(), Type::new(BaseType::Double).build()).build(),
    ///     Parameter::new("y".to_string(), Type::new(BaseType::Double).build()).build()
    /// ];
    /// let builder = FunctionBuilder::new_with_str("calculate_distance", Type::new(BaseType::Double).build())
    ///     .params(params);
    /// ```
    pub fn params(mut self, params: Vec<Parameter>) -> Self {
        self.params = params;
        self
    }

    /// Finalizes the function definition and returns a fully constructed `Function`.
    ///
    /// This method consumes the builder and produces the final `Function` object
    /// with all the configured properties.
    ///
    /// # Returns
    ///
    /// A fully constructed `Function` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let func = FunctionBuilder::new_with_str("main", Type::new(BaseType::Int).build())
    ///     .body(Block::new()
    ///         .statement(Statement::Return(Some(Expr::Int(0))))
    ///         .build())
    ///     .build();
    /// ```
    pub fn build(self) -> Function {
        Function {
            name: self.name,
            ret: self.ret,
            params: self.params,
            is_inline: self.is_extern,
            is_static: self.is_static,
            is_extern: self.is_extern,
            body: self.body,
            doc: self.doc,
        }
    }
}

/// Represents a single parameter in a C function declaration or definition.
///
/// A parameter consists of a name and a type, with special handling for array parameters.
/// This struct is used to generate the parameter list in C function signatures.
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Parameter {
    /// The name of the parameter
    pub name: String,

    /// The type of the parameter
    pub t: Type,
}

impl Parameter {
    /// Creates and returns a new `ParameterBuilder` to construct a `Parameter` using the builder pattern.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the parameter
    /// * `t` - The type of the parameter
    ///
    /// # Returns
    ///
    /// A new `ParameterBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = Parameter::new("buffer".to_string(),
    ///     Type::new(BaseType::Char).make_pointer().build());
    /// ```
    pub fn new(name: String, t: Type) -> ParameterBuilder {
        ParameterBuilder::new(name, t)
    }
}

/// A builder for constructing a `Parameter` instance with a fluent interface.
///
/// This builder simplifies the creation of function parameters by providing
/// a consistent interface aligned with the other builders in this module.
pub struct ParameterBuilder {
    name: String,
    t: Type,
}

impl ParameterBuilder {
    /// Creates and returns a new `ParameterBuilder` with the specified name and type.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the parameter as a String
    /// * `t` - The type of the parameter
    ///
    /// # Returns
    ///
    /// A new `ParameterBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = ParameterBuilder::new("count".to_string(), Type::new(BaseType::Int).build());
    /// ```
    pub fn new(name: String, t: Type) -> Self {
        Self { name, t }
    }

    /// Creates and returns a new `ParameterBuilder` using a string slice for the name.
    ///
    /// This is a convenience method that converts the provided string slice to a String
    /// before delegating to the standard `new` method.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the parameter as a string slice
    /// * `t` - The type of the parameter
    ///
    /// # Returns
    ///
    /// A new `ParameterBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = ParameterBuilder::new_with_str("size", Type::new(BaseType::SizeT).build());
    /// ```
    pub fn new_with_str(name: &str, t: Type) -> Self {
        Self::new(name.to_string(), t)
    }

    /// Finalizes the parameter definition and returns a fully constructed `Parameter`.
    ///
    /// This method consumes the builder and produces the final `Parameter` object
    /// with the configured name and type.
    ///
    /// # Returns
    ///
    /// A fully constructed `Parameter` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let param = ParameterBuilder::new_with_str("data", Type::new(BaseType::Void).make_pointer().build())
    ///     .build();
    /// ```
    pub fn build(self) -> Parameter {
        Parameter {
            name: self.name,
            t: self.t,
        }
    }
}

impl Format for Parameter {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.t.format(fmt)?;

        write!(fmt, " {}", self.name)?;

        if self.t.is_array() {
            write!(fmt, "[{}]", self.t.array)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn function() {
        let f = FunctionBuilder::new_with_str("some_function", Type::new(BaseType::Double).build())
            .make_inline()
            .param(
                ParameterBuilder::new_with_str("val", Type::new(BaseType::Double).build()).build(),
            )
            .body(
                Block::new()
                    .statement(Statement::Return(Some(Expr::Binary {
                        left: Box::new(Expr::Double(1.23)),
                        op: BinOp::Add,
                        right: Box::new(Expr::Ident("val".to_string())),
                    })))
                    .build(),
            )
            .build();
        let res = r#"double some_function(double val) {
  return 1.23 + val;
}
"#;
        assert_eq!(f.to_string(), res);
    }
}
