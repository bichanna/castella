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

use crate::{BaseType, DocComment, Format, Formatter, Type};
use tamacro::DisplayFromFormat;

/// Represents a struct in C.
///
/// # Examples
/// ```c
/// struct Person {
///   char* name;
///   int age;
/// }
/// ```
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
    /// Creates and returns a new `StructBuilder` to construct a `Struct` using the builder
    /// pattern.
    /// ```rust
    /// let s = Struct::new(/*name of the struct*/)
    ///     .field(/*struct field*/)
    ///     .build();
    /// ```
    pub fn new(name: String) -> StructBuilder {
        StructBuilder::new(name)
    }

    /// Returns the type of the struct.
    pub fn to_type(&self) -> Type {
        Type::new(BaseType::Struct(self.name.clone())).build()
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

/// A builder for constructing a `Struct` instance.
pub struct StructBuilder {
    name: String,
    fields: Vec<Field>,
    doc: Option<DocComment>,
}

impl StructBuilder {
    /// Creates and returns a new `StructBuilder` to construct a `Struct` using the builder
    /// pattern.
    /// ```rust
    /// let s = StructBuilder::new(/*name of the struct*/)
    ///     .field(/*struct field*/)
    ///     .build();
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: vec![],
            doc: None,
        }
    }

    /// Creates and returns a new `StructBuilder` construct a `Struct` with the given name string
    /// slice using the builder pattern.
    pub fn new_with_str(name: &str) -> Self {
        Self::new(name.to_string())
    }

    /// Sets the optional doc comment for the struct and returns the builder for more chaining.
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Appends a struct field to the struct being built and returns the builder for more chaining.
    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Sets the struct fields of the struct being built and returns the builder for more chaining.
    pub fn fields(mut self, fields: Vec<Field>) -> Self {
        self.fields = fields;
        self
    }

    /// Consumes the builder and returns a `Struct` containing all the fields.
    pub fn build(self) -> Struct {
        Struct {
            name: self.name,
            fields: self.fields,
            doc: self.doc,
        }
    }
}

/// Represents a struct field in C.
#[derive(Debug, Clone, DisplayFromFormat)]
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
    /// Creates and returns a new `FieldBuilder` to construct a `Field` using the builder pattern.
    /// ```rust
    /// let field = Field::new(/*name of the field*/, /*type of the field*/)
    ///     .build();
    /// ```
    pub fn new(name: String, t: Type) -> FieldBuilder {
        FieldBuilder::new(name, t)
    }

    /// Returns the type of the field.
    pub fn to_type(&self) -> Type {
        self.t.clone()
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

/// A builder for constructing a `Field` instance.
pub struct FieldBuilder {
    name: String,
    t: Type,
    width: Option<u8>,
    doc: Option<DocComment>,
}

impl FieldBuilder {
    /// Creates and returns a new `FieldBuilder` to construct a `Field` using the builder pattern.
    /// ```rust
    /// let field = FieldBuilder::new(/*name of the field*/, /*type of the field*/)
    ///     .build();
    /// ```
    pub fn new(name: String, t: Type) -> Self {
        Self {
            name,
            t,
            width: None,
            doc: None,
        }
    }

    /// Creates and returns a new `FieldBuilder` to construct a `Field` with the given name string
    /// slice using the builder pattern.
    pub fn new_with_str(name: &str, t: Type) -> Self {
        Self::new(name.to_string(), t)
    }

    /// Sets the optional doc comment for the field and returns the builder for more chaining.
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Sets the optional bit width for the field and returns the builder for more chaining.
    pub fn bitfield_width(mut self, width: u8) -> Self {
        self.width = Some(width);
        self
    }

    /// Consumes the builder and returns a `Field` containing all the information.
    pub fn build(self) -> Field {
        Field {
            name: self.name,
            t: self.t,
            width: self.width,
            doc: self.doc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn field() {
        let f = FieldBuilder::new_with_str("some_field", Type::new(BaseType::Char).build())
            .doc(DocComment::new().line_str("Hello").build())
            .build();
        let res = r#"/// Hello
char some_field;
"#;

        assert_eq!(f.to_string(), res);

        let f2 = FieldBuilder::new_with_str("another_field", Type::new(BaseType::Bool).build())
            .bitfield_width(1)
            .build();
        let res2 = "bool another_field : 1;\n";

        assert_eq!(f2.to_string(), res2);
    }

    #[test]
    fn structs() {
        let s = StructBuilder::new_with_str("Person")
            .fields(vec![
                FieldBuilder::new_with_str(
                    "name",
                    Type::new(BaseType::Char).make_pointer().build(),
                )
                .build(),
                FieldBuilder::new_with_str("age", Type::new(BaseType::UInt8).build()).build(),
            ])
            .build();
        let res = r#"struct Person {
  char* name;
  uint8_t age;
};
"#;

        assert_eq!(s.to_string(), res);
    }
}
