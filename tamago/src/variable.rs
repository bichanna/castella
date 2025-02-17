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

    pub fn new_with_value(name: String, t: Type, value: Expr) -> Self {
        Self {
            name,
            t,
            value: Some(value),
            is_static: false,
            is_extern: false,
            doc: None,
        }
    }

    pub fn set_doc(&mut self, doc: DocComment) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    pub fn set_static(&mut self) -> &mut Self {
        self.is_static = true;
        self
    }

    pub fn set_extern(&mut self) -> &mut Self {
        self.is_extern = true;
        self
    }

    pub fn set_raw_value(&mut self, value: String) -> &mut Self {
        self.value = Some(Expr::Raw(value));
        self
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

        if let Some(value) = &self.value {
            write!(fmt, " = ")?;
            value.format(fmt)?;
        }

        writeln!(fmt, ";")
    }
}
