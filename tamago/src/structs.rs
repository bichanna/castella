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

//! This module provides means to create C structs. For now, nested anonymous structs are not
//! supported, but this might change in the future.

use std::fmt::{self, Write};

use crate::comment::DocComment;
use crate::formatter::{Format, Formatter};
use crate::types::Type;
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Struct {
    /// The name of the struct
    name: String,

    /// The fields of the struct
    fields: Vec<Field>,

    /// The doc comment of the struct
    doc: Option<DocComment>,
}

impl Struct {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: vec![],
            doc: None,
        }
    }

    pub fn new_with_fields(name: String, fields: Vec<Field>) -> Self {
        Self {
            name,
            fields,
            doc: None,
        }
    }

    pub fn set_doc(&mut self, doc: DocComment) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    pub fn push_field(&mut self, field: Field) -> &mut Self {
        self.fields.push(field);
        self
    }
}

impl Format for Struct {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        write!(fmt, "struct {}", self.name)?;

        if !self.fields.is_empty() {
            fmt.block(|fmt| {
                for field in &self.fields {
                    field.format(fmt)?;
                }
                Ok(())
            })?;
        }

        writeln!(fmt, ";")
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    /// The name of the field
    pub name: String,

    /// The type of the field
    pub t: Type,

    /// The number of bits in the bitfield
    pub width: Option<u8>,

    /// The doc comment
    pub doc: Option<DocComment>,
}

impl Field {
    pub fn new(name: String, t: Type) -> Self {
        Self {
            name,
            t,
            width: None,
            doc: None,
        }
    }

    pub fn new_with_width(name: String, t: Type, width: u8) -> Self {
        Self {
            name,
            t,
            width: Some(width),
            doc: None,
        }
    }

    pub fn set_bitfield_width(&mut self, width: u8) -> &mut Self {
        self.width = Some(width);
        self
    }
}

impl Format for Field {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        self.t.format(fmt)?;
        write!(fmt, " {}", self.name)?;

        if let Some(w) = self.width {
            write!(fmt, " : {w}")?;
        }

        if self.t.is_array() {
            write!(fmt, "[{}]", self.t.array)?;
        }
        writeln!(fmt, ";")
    }
}
