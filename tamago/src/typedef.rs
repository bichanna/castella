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

//! This module provides make C typedefs.

use std::fmt::{self, Write};

use crate::{BaseType, Format, Formatter, Type};
use tamacro::DisplayFromFormat;

/// Represents a `typedef` in C.
///
/// # Examples
/// ```c
/// typedef struct Person Person;
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct TypeDef {
    /// The type to be aliased.
    pub t: Type,

    /// The name of the new type alias.
    pub name: String,
}

impl TypeDef {
    /// Creates and returns a new `TypeDefBuilder` to construct a `TypeDef` using the builder
    /// pattern.
    /// ```rust
    /// let typedef = TypeDef::new(/*name of the typedef*/)
    ///     .build();
    /// ```
    pub fn new(t: Type, name: String) -> TypeDefBuilder {
        TypeDefBuilder::new(t, name)
    }

    /// Returns the type of the typedef.
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
pub struct TypeDefBuilder {
    t: Type,
    name: String,
}

impl TypeDefBuilder {
    /// Creates and returns a new `TypeDefBuilder` to construct a `TypeDef` using the builder
    /// pattern.
    /// ```rust
    /// let typedef = TypeDef::new(/*name of the typedef*/)
    ///     .build();
    /// ```
    pub fn new(t: Type, name: String) -> Self {
        Self { t, name }
    }

    /// Creates and returns a new `TypeDefBuilder` to construct a `TypeDef` with the given name
    /// string slice using the builder pattern.
    pub fn new_with_str(t: Type, name: &str) -> Self {
        Self::new(t, name.to_string())
    }

    /// Consumes the builder and returns a `TypeDef` containing all the information.
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
