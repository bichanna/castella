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

//! This module provides means to create C functions.

use std::fmt::{self, Write};

use crate::{Block, DocComment, Format, Formatter, Statement, Type};
use tamacro::DisplayFromFormat;

/// Represents a C function
/// ```c
/// int main(void) {
///   return 0;
/// }
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Function {
    /// The name of the function
    pub name: String,

    /// The parameters of the function
    pub params: Vec<Parameter>,

    /// The return type of the function
    pub ret: Type,

    /// Whether it's inline
    pub is_inline: bool,

    /// Whether it's static
    pub is_static: bool,

    /// Whether it's extern
    pub is_extern: bool,

    /// The body of the function
    pub body: Block,

    /// The optional doc comment of the function
    pub doc: Option<DocComment>,
}

impl Function {
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
        } else {
            if self.params.len() > 0 {
                for param in &self.params[..self.params.len() - 1] {
                    param.format(fmt)?;
                    write!(fmt, ", ")?;
                }

                if let Some(last) = self.params.last() {
                    last.format(fmt)?;
                }
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

/// A builder for constructing a `Function` instance
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
    /// Creates and returns a new `FunctionBuilder` to construct a `Function` using the builder
    /// pattern.
    /// ```rust
    /// let func = FunctionBuilder::new(/*name of the function*/, /*return type of the function*/)
    ///     .params(/*parameters of the function*/)
    ///     .body(/*function body*/)
    ///     .build();
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

    /// Creates and returns a new `FunctionBuilder` to construct a `Function` using the builder
    /// pattern with the given name string slice and the return type.
    pub fn new_with_str(name: &str, ret: Type) -> Self {
        Self::new(name.to_string(), ret)
    }

    /// Sets the doc comment of the function being built, and returns the builder for more
    /// chaining.
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Makes the function inline, prepending the `inline` keyword to the return type.
    pub fn make_inline(mut self) -> Self {
        self.is_inline = true;
        self
    }

    /// Makes the function static, prepending the `static` keyword to the return type.
    pub fn make_static(mut self) -> Self {
        self.is_static = true;
        self
    }

    /// Makes the function extern, prepending the `extern` keyword to the return type.
    pub fn make_extern(mut self) -> Self {
        self.is_extern = true;
        self
    }

    /// Sets the body block for the function being built, and returns the builder for more
    /// chaining.
    pub fn body(mut self, body: Block) -> Self {
        self.body = body;
        self
    }

    /// Appends a statement to the body block, and returns the builder for chaining more
    /// operations.
    pub fn statement(mut self, stmt: Statement) -> Self {
        self.body.stmts.push(stmt);
        self
    }

    /// Appends a new line to the body block, and returns the builder for chaining more operations.
    pub fn new_line(mut self) -> Self {
        self.body.stmts.push(Statement::NewLine);
        self
    }

    /// Appends a parameter to the function being built, and returns the builder for more chaining.
    pub fn param(mut self, param: Parameter) -> Self {
        self.params.push(param);
        self
    }

    /// Sets the parameters of the function being built and returns the buidler for more chaining.
    pub fn params(mut self, params: Vec<Parameter>) -> Self {
        self.params = params;
        self
    }

    /// COnsumes the builder and returns a `Function` containing the name, return type, parameters,
    /// and body of the function.
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

/// Represents a function parameter.
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Parameter {
    /// The name of the parameter
    pub name: String,

    /// The type of the parameter
    pub t: Type,
}

impl Parameter {
    /// Creates and returns a new `ParameterBuilder` to construct a `Parameter` using the builder pattern.
    /// ```rust
    /// let param = Parameter::new(/*name of the parameter*/, /*the type of the parameter*/)
    ///     .build();
    /// ```
    pub fn new(name: String, t: Type) -> ParameterBuilder {
        ParameterBuilder::new(name, t)
    }
}

/// A builder for constructing a `Parameter` instance.
pub struct ParameterBuilder {
    name: String,
    t: Type,
}

impl ParameterBuilder {
    /// Creates and returns a new `ParameterBuilder` to construct a `Parameter` using the builder pattern.
    /// ```rust
    /// let param = ParameterBuilder::new(/*name of the parameter*/, /*the type of the parameter*/)
    ///     .build();
    /// ```
    pub fn new(name: String, t: Type) -> Self {
        Self { name, t }
    }

    /// Creates and returns a new `ParameterBuilder` to construct a `Parameter` with the given name
    /// string slice and the type of the parameter using the builder pattern.
    pub fn new_with_str(name: &str, t: Type) -> Self {
        Self::new(name.to_string(), t)
    }

    /// Consumes the builder and returns a `Parameter` from the builder.
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
                        left: Box::new(Expr::ConstDouble(1.23)),
                        op: BinOp::Add,
                        right: Box::new(Expr::Ident("val".to_string())),
                    })))
                    .build(),
            )
            .build();
        let res = r#"double some_function(double val) {
  return (1.23 + val);
}
"#;
        assert_eq!(f.to_string(), res);
    }
}
