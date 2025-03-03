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

//! This module provides means to create C structs programmatically in Rust.
//!
//! It offers a builder pattern approach for defining C-compatible struct types
//! with their fields, documentation, and other attributes. This is particularly
//! useful for generating C header files or FFI bindings.
//!
//! For now, nested anonymous structs are not supported, but this might change in the future.

use std::fmt::{self, Write};

use crate::{BaseType, DocComment, Format, Formatter, Type};
use tamacro::DisplayFromFormat;

/// Represents a struct in C.
///
/// This struct holds all the information needed to represent a C struct,
/// including its name, fields, and documentation.
///
/// # Examples
///
/// A C struct representation:
/// ```c
/// struct Person {
///   char* name;
///   int age;
/// };
/// ```
///
/// Creating this struct using the builder pattern:
/// ```rust
/// use crate::{Struct, Field, Type, BaseType, DocComment};
///
/// let person = Struct::new("Person".to_string())
///     .field(Field::new("name".to_string(), Type::new(BaseType::Char).make_pointer().build()))
///     .field(Field::new("age".to_string(), Type::new(BaseType::Int).build()))
///     .build();
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
    ///
    /// # Parameters
    /// * `name` - The name to be given to the struct
    ///
    /// # Returns
    /// A new `StructBuilder` instance initialized with the given name
    ///
    /// # Examples
    /// ```rust
    /// let person_struct = Struct::new("Person".to_string())
    ///     .field(Field::new("name".to_string(), Type::new(BaseType::Char).make_pointer().build()))
    ///     .field(Field::new("age".to_string(), Type::new(BaseType::Int).build()))
    ///     .build();
    ///
    /// println!("{}", person_struct);
    /// // Outputs:
    /// // struct Person {
    /// //   char* name;
    /// //   int age;
    /// // };
    /// ```
    pub fn new(name: String) -> StructBuilder {
        StructBuilder::new(name)
    }

    /// Returns the type representation of the struct.
    ///
    /// This allows using a struct definition as a type for fields or function parameters.
    ///
    /// # Returns
    /// A `Type` instance representing this struct type
    ///
    /// # Examples
    /// ```rust
    /// let person_struct = Struct::new("Person".to_string())
    ///     .field(Field::new("name".to_string(), Type::new(BaseType::Char).make_pointer().build()))
    ///     .build();
    ///
    /// // Now use this struct as a type for another field
    /// let person_field = Field::new("person".to_string(), person_struct.to_type())
    ///     .build();
    /// ```
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
///
/// This builder implements the builder pattern for creating struct
/// definitions with a fluent interface.
pub struct StructBuilder {
    name: String,
    fields: Vec<Field>,
    doc: Option<DocComment>,
}

impl StructBuilder {
    /// Creates and returns a new `StructBuilder` to construct a `Struct` using the builder
    /// pattern.
    ///
    /// # Parameters
    /// * `name` - The name to be given to the struct
    ///
    /// # Returns
    /// A new `StructBuilder` instance initialized with the given name
    ///
    /// # Examples
    /// ```rust
    /// let builder = StructBuilder::new("Person".to_string());
    /// let person_struct = builder
    ///     .field(Field::new("name".to_string(), Type::new(BaseType::Char).make_pointer().build()))
    ///     .field(Field::new("age".to_string(), Type::new(BaseType::Int).build()))
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
    ///
    /// This is a convenience method that converts a string slice to a `String`.
    ///
    /// # Parameters
    /// * `name` - The name of the struct as a string slice
    ///
    /// # Returns
    /// A new `StructBuilder` instance initialized with the given name
    ///
    /// # Examples
    /// ```rust
    /// let person_struct = StructBuilder::new_with_str("Person")
    ///     .field(Field::new_with_str("name", Type::new(BaseType::Char).make_pointer().build()))
    ///     .field(Field::new_with_str("age", Type::new(BaseType::Int).build()))
    ///     .build();
    /// ```
    pub fn new_with_str(name: &str) -> Self {
        Self::new(name.to_string())
    }

    /// Sets the optional doc comment for the struct and returns the builder for more chaining.
    ///
    /// # Parameters
    /// * `doc` - A `DocComment` instance to be associated with the struct
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Appends a struct field to the struct being built and returns the builder for more chaining.
    ///
    /// # Parameters
    /// * `field` - A `Field` instance to be added to the struct
    ///
    /// # Returns
    /// The builder instance for method chaining
    ///
    /// # Examples
    /// ```rust
    /// let name_field = Field::new_with_str("name", Type::new(BaseType::Char).make_pointer().build());
    /// let age_field = Field::new_with_str("age", Type::new(BaseType::Int).build());
    ///
    /// let person_struct = StructBuilder::new_with_str("Person")
    ///     .field(name_field)
    ///     .field(age_field)
    ///     .build();
    /// ```
    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Sets the struct fields of the struct being built and returns the builder for more chaining.
    ///
    /// This method replaces any existing fields with the provided vector of fields.
    ///
    /// # Parameters
    /// * `fields` - A vector of `Field` instances to be added to the struct
    ///
    /// # Returns
    /// The builder instance for method chaining
    ///
    /// # Examples
    /// ```rust
    /// let fields = vec![
    ///     Field::new_with_str("name", Type::new(BaseType::Char).make_pointer().build()),
    ///     Field::new_with_str("age", Type::new(BaseType::Int).build())
    /// ];
    ///
    /// let person_struct = StructBuilder::new_with_str("Person")
    ///     .fields(fields)
    ///     .build();
    /// ```
    pub fn fields(mut self, fields: Vec<Field>) -> Self {
        self.fields = fields;
        self
    }

    /// Consumes the builder and returns a `Struct` containing all the fields.
    ///
    /// # Returns
    /// A fully constructed `Struct` instance
    pub fn build(self) -> Struct {
        Struct {
            name: self.name,
            fields: self.fields,
            doc: self.doc,
        }
    }
}

/// Represents a struct field in C.
///
/// This struct holds all the information needed to represent a field
/// within a C struct, including its name, type, bitfield width (if any),
/// and documentation.
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Field {
    /// The name of the field
    pub name: String,

    /// The type of the field
    pub t: Type,

    /// The number of bits in the bitfield, if this is a bitfield
    pub width: Option<u8>,

    /// The doc comment
    pub doc: Option<DocComment>,
}

impl Field {
    /// Creates and returns a new `FieldBuilder` to construct a `Field` using the builder pattern.
    ///
    /// # Parameters
    /// * `name` - The name of the field
    /// * `t` - The type of the field
    ///
    /// # Returns
    /// A new `FieldBuilder` instance initialized with the given name and type
    ///
    /// # Examples
    /// ```rust
    /// let name_field = Field::new("name".to_string(), Type::new(BaseType::Char).make_pointer().build())
    ///     .build();
    ///
    /// let age_field = Field::new("age".to_string(), Type::new(BaseType::Int).build())
    ///     .build();
    /// ```
    pub fn new(name: String, t: Type) -> FieldBuilder {
        FieldBuilder::new(name, t)
    }

    /// Returns the type of the field.
    ///
    /// # Returns
    /// A clone of the field's type
    ///
    /// # Examples
    /// ```rust
    /// let field = Field::new("count".to_string(), Type::new(BaseType::Int).build())
    ///     .build();
    ///
    /// let field_type = field.to_type();
    /// assert_eq!(field_type.to_string(), "int");
    /// ```
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
///
/// This builder implements the builder pattern for creating struct
/// field definitions with a fluent interface.
pub struct FieldBuilder {
    name: String,
    t: Type,
    width: Option<u8>,
    doc: Option<DocComment>,
}

impl FieldBuilder {
    /// Creates and returns a new `FieldBuilder` to construct a `Field` using the builder pattern.
    ///
    /// # Parameters
    /// * `name` - The name of the field
    /// * `t` - The type of the field
    ///
    /// # Returns
    /// A new `FieldBuilder` instance initialized with the given name and type
    ///
    /// # Examples
    /// ```rust
    /// let builder = FieldBuilder::new("name".to_string(), Type::new(BaseType::Char).make_pointer().build());
    /// let name_field = builder.build();
    ///
    /// // Or in a single chain:
    /// let age_field = FieldBuilder::new("age".to_string(), Type::new(BaseType::Int).build())
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
    ///
    /// This is a convenience method that converts a string slice to a `String`.
    ///
    /// # Parameters
    /// * `name` - The name of the field as a string slice
    /// * `t` - The type of the field
    ///
    /// # Returns
    /// A new `FieldBuilder` instance initialized with the given name and type
    ///
    /// # Examples
    /// ```rust
    /// let name_field = FieldBuilder::new_with_str("name", Type::new(BaseType::Char).make_pointer().build())
    ///     .build();
    ///
    /// let age_field = FieldBuilder::new_with_str("age", Type::new(BaseType::Int).build())
    ///     .build();
    /// ```
    pub fn new_with_str(name: &str, t: Type) -> Self {
        Self::new(name.to_string(), t)
    }

    /// Sets the optional doc comment for the field and returns the builder for more chaining.
    ///
    /// # Parameters
    /// * `doc` - A `DocComment` instance to be associated with the field
    ///
    /// # Returns
    /// The builder instance for method chaining
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Sets the optional bit width for the field and returns the builder for more chaining.
    ///
    /// When specified, this indicates that the field is a bitfield with the given width.
    ///
    /// # Parameters
    /// * `width` - The number of bits to allocate for this bitfield
    ///
    /// # Returns
    /// The builder instance for method chaining
    ///
    /// # Examples
    /// ```rust
    /// // Create a 1-bit flag field
    /// let flag_field = FieldBuilder::new_with_str("is_active", Type::new(BaseType::Bool).build())
    ///     .bitfield_width(1)
    ///     .build();
    ///
    /// // Create a 4-bit enum field that can store values 0-15
    /// let type_field = FieldBuilder::new_with_str("type", Type::new(BaseType::UInt8).build())
    ///     .bitfield_width(4)
    ///     .build();
    /// ```
    pub fn bitfield_width(mut self, width: u8) -> Self {
        self.width = Some(width);
        self
    }

    /// Consumes the builder and returns a `Field` containing all the information.
    ///
    /// # Returns
    /// A fully constructed `Field` instance
    ///
    /// # Examples
    /// ```rust
    /// let field = FieldBuilder::new_with_str("name", Type::new(BaseType::Char).make_pointer().build())
    ///     .doc(DocComment::new().line_str("The person's full name").build())
    ///     .build();
    ///
    /// println!("{}", field);
    /// // Output:
    /// // /// The person's full name
    /// // char* name;
    /// ```
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
