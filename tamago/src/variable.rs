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

//! This module provides ways to add variables.

use std::fmt::{self, Write};

use crate::{DocComment, Expr, Format, Formatter, Type};
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Variable {
    pub name: String,
    pub t: Type,
    pub value: Option<Expr>,
    pub is_static: bool,
    pub is_extern: bool,
    pub doc: Option<DocComment>,
}

impl Variable {
    pub fn new(name: String, t: Type) -> VariableBuilder {
        VariableBuilder::new(name, t)
    }

    pub fn to_type(&self) -> Type {
        self.t.clone()
    }
}

impl Format for Variable {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        if self.is_extern {
            write!(fmt, "extern ")?;
        }

        if self.is_static {
            write!(fmt, "static ")?;
        }

        self.t.format(fmt)?;
        write!(fmt, " {}", self.name)?;

        if self.t.is_array() {
            write!(fmt, "[{}]", self.t.array)?;
        }

        if !self.is_extern {
            if let Some(value) = &self.value {
                write!(fmt, " = ")?;
                value.format(fmt)?;
            }
        }

        Ok(())
    }
}

pub struct VariableBuilder {
    name: String,
    t: Type,
    value: Option<Expr>,
    is_static: bool,
    is_extern: bool,
    doc: Option<DocComment>,
}

impl VariableBuilder {
    pub fn new(name: String, t: Type) -> Self {
        Self {
            name,
            t,
            value: None,
            is_static: false,
            is_extern: false,
            doc: None,
        }
    }

    pub fn new_with_str(name: &str, t: Type) -> Self {
        Self::new(name.to_string(), t)
    }

    pub fn value(mut self, value: Expr) -> Self {
        self.value = Some(value);
        self
    }

    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    pub fn make_static(mut self) -> Self {
        self.is_static = true;
        self
    }

    pub fn make_extern(mut self) -> Self {
        self.is_extern = true;
        self
    }

    pub fn raw_value(mut self, value: String) -> Self {
        self.value = Some(Expr::Raw(value));
        self
    }

    pub fn build(self) -> Variable {
        Variable {
            name: self.name,
            t: self.t,
            value: self.value,
            is_static: self.is_static,
            is_extern: self.is_extern,
            doc: self.doc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn var() {
        let var = VariableBuilder::new_with_str(
            "some_var",
            TypeBuilder::new(BaseType::Char)
                .make_pointer()
                .make_const()
                .build(),
        )
        .value(Expr::Str("Hello, world".to_string()))
        .build();

        let res = "const char* some_var = \"Hello, world\"";

        assert_eq!(var.to_string(), res);

        let another_var =
            VariableBuilder::new_with_str("another_var", TypeBuilder::new(BaseType::Bool).build())
                .make_static()
                .build();

        let another_res = "static bool another_var";

        assert_eq!(another_var.to_string(), another_res);
    }
}
