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

//! This module provides means to create C enums.

use std::fmt::{self, Write};

use crate::{BaseType, DocComment, Format, Formatter, Type};
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Enum {
    name: String,
    variants: Vec<Variant>,
    doc: Option<DocComment>,
}

impl Enum {
    pub fn new(name: String) -> Self {
        Self {
            name,
            variants: vec![],
            doc: None,
        }
    }

    pub fn new_with_variants(name: String, variants: Vec<Variant>) -> Self {
        Self {
            name,
            variants,
            doc: None,
        }
    }

    pub fn set_doc(&mut self, doc: DocComment) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    pub fn push_variant(&mut self, variant: Variant) -> &mut Self {
        self.variants.push(variant);
        self
    }

    pub fn to_type(&self) -> Type {
        Type::new(BaseType::Enum(self.name.clone()))
    }
}

impl Format for Enum {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        write!(fmt, "enum {}", self.name)?;

        fmt.block(|fmt| {
            for variant in &self.variants {
                variant.format(fmt)?;
                writeln!(fmt, ",")?;
            }

            Ok(())
        })?;

        writeln!(fmt, ";")
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Variant {
    pub name: String,
    pub value: Option<i64>,
    pub doc: Option<DocComment>,
}

impl Variant {
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: None,
            doc: None,
        }
    }

    pub fn new_with_value(name: String, value: i64) -> Self {
        Self {
            name,
            value: Some(value),
            doc: None,
        }
    }

    pub fn set_doc(&mut self, doc: DocComment) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    pub fn set_value(&mut self, value: i64) -> &mut Self {
        self.value = Some(value);
        self
    }
}

impl Format for Variant {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        write!(fmt, "{}", self.name)?;

        if let Some(value) = self.value {
            write!(fmt, " = {value}")?;
        }

        Ok(())
    }
}
