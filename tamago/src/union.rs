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

//! This module provides tools for creating and managing C unions in Rust.
//!
//! It allows for programmatic generation of C union declarations with support
//! for fields, documentation, and conversion to a `Type`. This is particularly
//! useful for generating C code or bindings from Rust.

use std::fmt::{self, Write};

use crate::{BaseType, DocComment, Field, Format, Formatter, Type};
use tamacro::DisplayFromFormat;

/// Represents a C union with its fields and attributes.
///
/// The `Union` struct enables the creation of complete C union declarations,
/// supporting a name, a list of fields, and optional documentation comments.
/// Unions in C allow multiple fields to share the same memory location.
///
/// # Examples
///
/// Basic union declaration:
/// ```c
/// union Data {
///   int i;
///   float f;
/// };
/// ```
///
/// Union with array field:
/// ```c
/// union Buffer {
///   char data[16];
///   int status;
/// };
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Union {
    /// The name of the union.
    pub name: String,

    /// The fields of the union.
    pub fields: Vec<Field>,

    /// The optional documentation comment for the union.
    pub doc: Option<DocComment>,
}

impl Union {
    /// Creates and returns a new `UnionBuilder` to construct a `Union` using the builder pattern.
    ///
    /// This method simplifies the process of creating complex union declarations
    /// by providing an interface for defining all union properties.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the union
    ///
    /// # Returns
    ///
    /// A `UnionBuilder` instance that can be used to configure and build a `Union`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let union = Union::new("Value".to_string())
    ///     .field(FieldBuilder::new_with_str("x", Type::new(BaseType::Int).build()).build())
    ///     .field(FieldBuilder::new_with_str("y", Type::new(BaseType::Float).build()).build())
    ///     .build();
    /// ```
    pub fn new(name: String) -> UnionBuilder {
        UnionBuilder::new(name)
    }

    /// Converts the union to a `Type` for use in other declarations.
    ///
    /// # Returns
    ///
    /// A `Type` instance representing the union as `union <name>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let union = Union::new("Data".to_string()).build();
    /// let union_type = union.to_type();
    /// assert_eq!(union_type.to_string(), "union Data");
    /// ```
    pub fn to_type(&self) -> Type {
        Type::new(BaseType::Union(self.name.clone())).build()
    }
}

impl Format for Union {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        write!(fmt, "union {}", self.name)?;

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

/// A builder for constructing a `Union` instance with a fluent interface.
///
/// The `UnionBuilder` provides a step-by-step approach to creating complex C unions
/// by allowing incremental configuration of the union's name, fields, and documentation.
pub struct UnionBuilder {
    name: String,
    fields: Vec<Field>,
    doc: Option<DocComment>,
}

impl UnionBuilder {
    /// Creates and returns a new `UnionBuilder` with the specified name.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the union as a String
    ///
    /// # Returns
    ///
    /// A new `UnionBuilder` instance with default values for fields and documentation
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = UnionBuilder::new("Status".to_string());
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: vec![],
            doc: None,
        }
    }

    /// Creates and returns a new `UnionBuilder` using a string slice for the name.
    ///
    /// This is a convenience method that converts the provided string slice to a String
    /// before delegating to the standard `new` method.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the union as a string slice
    ///
    /// # Returns
    ///
    /// A new `UnionBuilder` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = UnionBuilder::new_with_str("Info");
    /// ```
    pub fn new_with_str(name: &str) -> Self {
        Self::new(name.to_string())
    }

    /// Sets the documentation comment for the union being built.
    ///
    /// # Parameters
    ///
    /// * `doc` - The documentation comment to attach to the union
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = UnionBuilder::new_with_str("Result")
    ///     .doc(DocComment::new("Represents a computation result").build());
    /// ```
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Adds a field to the union being built.
    ///
    /// # Parameters
    ///
    /// * `field` - The field to add to the union
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = UnionBuilder::new_with_str("Data")
    ///     .field(FieldBuilder::new_with_str("id", Type::new(BaseType::Int).build()).build());
    /// ```
    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Sets the complete list of fields for the union being built.
    ///
    /// This method replaces any previously added fields with the provided list.
    ///
    /// # Parameters
    ///
    /// * `fields` - A vector of `Field` objects representing the union's fields
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let fields = vec![
    ///     FieldBuilder::new_with_str("x", Type::new(BaseType::Int).build()).build(),
    ///     FieldBuilder::new_with_str("y", Type::new(BaseType::Float).build()).build()
    /// ];
    /// let builder = UnionBuilder::new_with_str("Point").fields(fields);
    /// ```
    pub fn fields(mut self, fields: Vec<Field>) -> Self {
        self.fields = fields;
        self
    }

    /// Finalizes the union definition and returns a fully constructed `Union`.
    ///
    /// # Returns
    ///
    /// A fully constructed `Union` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let union = UnionBuilder::new_with_str("Value")
    ///     .field(FieldBuilder::new_with_str("a", Type::new(BaseType::Int).build()).build())
    ///     .build();
    /// ```
    pub fn build(self) -> Union {
        Union {
            name: self.name,
            fields: self.fields,
            doc: self.doc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn union() {
        let u = UnionBuilder::new_with_str("some_union")
            .fields(vec![
                FieldBuilder::new_with_str(
                    "a",
                    TypeBuilder::new(BaseType::Char).make_array(20).build(),
                )
                .build(),
                FieldBuilder::new_with_str("b", TypeBuilder::new(BaseType::Int).build()).build(),
                FieldBuilder::new_with_str("c", TypeBuilder::new(BaseType::Bool).build()).build(),
            ])
            .build();
        let res = r#"union some_union {
  char a[20];
  int b;
  bool c;
};
"#;

        assert_eq!(u.to_string(), res);
    }
}
