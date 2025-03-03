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

//! This module provides functionality for generating C-style typedef declarations.
//!
//! A typedef in C creates an alias for a type, allowing developers to use more
//! descriptive or shorter names for complex types. This module allows programmers
//! to programmatically generate such typedef declarations with proper formatting.

use std::fmt::{self, Write};

use crate::{BaseType, Format, Formatter, Type};
use tamacro::DisplayFromFormat;

/// Represents a C-style `typedef` declaration.
///
/// A `TypeDef` encapsulates both the original type being aliased and the new alias name.
/// It provides methods for creating and manipulating typedef declarations and can be
/// formatted as valid C code.
///
/// # Examples
///
/// Basic typedef for a struct:
/// ```rust
/// // Creates a typedef equivalent to: typedef struct Person Person;
/// let person_type = Type::new(BaseType::Struct("Person".to_string())).build();
/// let typedef = TypeDef::new(person_type, "Person".to_string()).build();
/// println!("{}", typedef); // Outputs: typedef struct Person Person;
/// ```
///
/// Creating a typedef for a function pointer:
/// ```rust
/// // Creates a typedef for a callback function
/// let callback_type = Type::new(BaseType::FunctionPointer(/* function pointer details */)).build();
/// let typedef = TypeDef::new(callback_type, "Callback".to_string()).build();
/// // Outputs something like: typedef void (*Callback)(int, void*);
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct TypeDef {
    /// The type to be aliased.
    pub t: Type,

    /// The name of the new type alias.
    pub name: String,
}

impl TypeDef {
    /// Creates and returns a new `TypeDefBuilder` to construct a `TypeDef` using the builder pattern.
    ///
    /// This method initializes a builder with the provided original type and alias name.
    /// The builder pattern allows for a clear and flexible way to construct `TypeDef` instances.
    ///
    /// # Parameters
    ///
    /// * `t` - The original `Type` that will be aliased by the typedef.
    /// * `name` - The name for the new type alias as a `String`.
    ///
    /// # Returns
    ///
    /// Returns a `TypeDefBuilder` instance configured with the provided type and name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let original_type = Type::new(BaseType::Int).build();
    /// let typedef_builder = TypeDef::new(original_type, "Integer".to_string());
    /// let typedef = typedef_builder.build();
    ///
    /// assert_eq!(typedef.to_string(), "typedef int Integer;\n");
    /// ```
    pub fn new(t: Type, name: String) -> TypeDefBuilder {
        TypeDefBuilder::new(t, name)
    }

    /// Converts this typedef declaration to a `Type` instance.
    ///
    /// This method creates a new `Type` representing a reference to this typedef.
    /// This is useful when you want to use the typedef name in other type declarations.
    ///
    /// # Returns
    ///
    /// Returns a new `Type` instance that references this typedef by name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Create a typedef for a struct
    /// let struct_type = Type::new(BaseType::Struct("Point".to_string())).build();
    /// let typedef = TypeDef::new(struct_type, "Point".to_string()).build();
    ///
    /// // Use the typedef in another declaration
    /// let point_type = typedef.to_type();
    /// let pointer_to_point = Type::new(BaseType::Pointer(Box::new(point_type))).build();
    ///
    /// // This would represent: Point* pointPtr;
    /// ```
    pub fn to_type(&self) -> Type {
        Type::new(BaseType::TypeDef(self.name.clone())).build()
    }
}

impl Format for TypeDef {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "typedef ")?;
        self.t.format(fmt)?;
        writeln!(fmt, " {};", self.name)
    }
}

/// A builder for constructing a `TypeDef` instance.
///
/// This builder provides a clear and concise way to create `TypeDef` instances.
/// It allows setting the original type and alias name before building the final typedef.
///
/// # Examples
///
/// ```rust
/// // Using the builder directly
/// let builder = TypeDefBuilder::new_with_str(
///     Type::new(BaseType::Char).build(),
///     "Byte"
/// );
/// let typedef = builder.build();
///
/// // Or using the TypeDef::new method
/// let typedef = TypeDef::new(
///     Type::new(BaseType::Double).build(),
///     "Real".to_string()
/// ).build();
/// ```
pub struct TypeDefBuilder {
    t: Type,
    name: String,
}

impl TypeDefBuilder {
    /// Creates a new `TypeDefBuilder` with the specified type and name.
    ///
    /// # Parameters
    ///
    /// * `t` - The original `Type` that will be aliased by the typedef.
    /// * `name` - The name for the new type alias as a `String`.
    ///
    /// # Returns
    ///
    /// Returns a new `TypeDefBuilder` configured with the provided type and name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let original_type = Type::new(BaseType::Long).build();
    /// let builder = TypeDefBuilder::new(original_type, "BigInt".to_string());
    /// let typedef = builder.build();
    /// ```
    pub fn new(t: Type, name: String) -> Self {
        Self { t, name }
    }

    /// Creates a new `TypeDefBuilder` with the specified type and a string slice name.
    ///
    /// This is a convenience method that converts the string slice to a `String` internally.
    ///
    /// # Parameters
    ///
    /// * `t` - The original `Type` that will be aliased by the typedef.
    /// * `name` - The name for the new type alias as a string slice.
    ///
    /// # Returns
    ///
    /// Returns a new `TypeDefBuilder` configured with the provided type and name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let original_type = Type::new(BaseType::Unsigned(BaseType::Int)).build();
    /// let builder = TypeDefBuilder::new_with_str(original_type, "UInt");
    /// let typedef = builder.build();
    /// ```
    pub fn new_with_str(t: Type, name: &str) -> Self {
        Self::new(t, name.to_string())
    }

    /// Consumes the builder and returns a fully constructed `TypeDef` instance.
    ///
    /// # Returns
    ///
    /// Returns a new `TypeDef` with the type and name configured in this builder.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = TypeDefBuilder::new_with_str(
    ///     Type::new(BaseType::Void).build(),
    ///     "Nothing"
    /// );
    /// let typedef = builder.build();
    ///
    /// assert_eq!(typedef.to_string(), "typedef void Nothing;\n");
    /// ```
    pub fn build(self) -> TypeDef {
        TypeDef {
            t: self.t,
            name: self.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn typedef() {
        let t = TypeDefBuilder::new_with_str(
            TypeBuilder::new(BaseType::Struct("Person".to_string())).build(),
            "Person",
        )
        .build();
        let res = "typedef struct Person Person;\n";

        assert_eq!(t.to_string(), res);
    }
}
