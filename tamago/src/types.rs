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

//! This module provides ways to express types.

use std::fmt::{self, Write};

use crate::{Format, Formatter};
use tamacro::DisplayFromFormat;

/// Encapsulates all types of base types used in C.
#[derive(Debug, Clone, DisplayFromFormat)]
pub enum BaseType {
    /// Represents the `void` type.
    Void,

    /// Represents the `double` type, double precision floating point number.
    Double,

    /// Represents the `float` type, single precision floating point number.
    Float,

    /// Represents the `char` type, a single byte character.
    Char,

    /// Represents the `int` type.
    Int,

    /// Represents the `uint8_t` type from `stdint.h`, an unsigned 8-bit integer.
    UInt8,

    /// Represents the `uint16_t` type from `stdint.h`, an unsigned 16-bit integer.
    UInt16,

    /// Represents the `uint32_t` type from `stdint.h`, an unsigned 32-bit integer.
    UInt32,

    /// Represents the `uint64_t` type from `stdint.h`, an unsigned 64-bit integer.
    UInt64,

    /// Represents the `int8_t` type from `stdint.h`, a signed 8-bit integer.
    Int8,

    /// Represents the `int16_t` type from `stdint.h`, a signed 16-bit integer.
    Int16,

    /// Represents the `int32_t` type from `stdint.h`, a signed 32-bit integer.
    Int32,

    /// Represents the `int64_t` type from `stdint.h`, a signed 64-bit integer.
    Int64,

    /// Represents the `size_t` type from `stddef.h`.
    Size,

    /// Represents the `uintptr_t` type from `stdint.h`.
    UIntPtr,

    /// Represents lthe `bool` type from `stdbool.h`.
    Bool,

    /// An enumeration type.
    Enum(String),

    /// A struct type.
    Struct(String),

    /// A union type.
    Union(String),

    /// `typedef`
    TypeDef(String),
}

impl BaseType {
    /// Creates a new unsigned integer with the given bit size.
    /// 8 -> UInt8
    /// 16 -> UInt16
    /// 32 -> UInt32
    /// 64 -> UInt64
    pub fn new_uint(bits: u8) -> Self {
        use BaseType::*;
        match bits {
            8 => UInt8,
            16 => UInt16,
            32 => UInt32,
            64 => UInt64,
            _ => UInt64,
        }
    }

    /// Creates a new signed integer with the given bit size.
    /// 8 -> Int8
    /// 16 -> Int16
    /// 32 -> Int32
    /// 64 -> Int64
    pub fn new_int(bits: u8) -> Self {
        use BaseType::*;
        match bits {
            8 => Int8,
            16 => Int16,
            32 => Int32,
            64 => Int64,
            _ => Int64,
        }
    }

    /// Whether an integer type or not.
    pub fn is_integer(&self) -> bool {
        use BaseType::*;
        matches!(
            self,
            Int | UInt8
                | UInt16
                | UInt32
                | UInt64
                | Int8
                | Int16
                | Int32
                | Int64
                | Size
                | UIntPtr
                | Bool
                | Char
        )
    }

    /// Whether a tag type or not.
    pub fn is_tag_type(&self) -> bool {
        use BaseType::*;
        matches!(self, Enum(_) | Struct(_) | Union(_) | TypeDef(_))
    }
}

impl Format for BaseType {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use BaseType::*;
        match self {
            Void => write!(fmt, "void"),
            Double => write!(fmt, "double"),
            Float => write!(fmt, "float"),
            Char => write!(fmt, "char"),
            Int => write!(fmt, "int"),
            UInt8 => write!(fmt, "uint8_t"),
            UInt16 => write!(fmt, "uint16_t"),
            UInt32 => write!(fmt, "uint32_t"),
            UInt64 => write!(fmt, "uint64_t"),
            Int8 => write!(fmt, "int8_t"),
            Int16 => write!(fmt, "int16_t"),
            Int32 => write!(fmt, "int32_t"),
            Int64 => write!(fmt, "int64_t"),
            Size => write!(fmt, "size_t"),
            UIntPtr => write!(fmt, "uintptr_t"),
            Bool => write!(fmt, "bool"),
            Enum(s) => write!(fmt, "enum {s}"),
            Struct(s) => write!(fmt, "struct {s}"),
            Union(s) => write!(fmt, "union {s}"),
            TypeDef(s) => write!(fmt, "{s}"),
        }
    }
}

/// Encapsulates all type qualifiers in C.
#[derive(Debug, Clone, Copy, DisplayFromFormat)]
pub enum TypeQualifier {
    /// The `volatile` keyword.
    Volatile,

    /// The `const` keyword.
    Const,
}

impl Format for TypeQualifier {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use TypeQualifier::*;
        match self {
            Volatile => write!(fmt, "volatile"),
            Const => write!(fmt, "const"),
        }
    }
}

/// Represents a type in C.
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Type {
    /// The base type used to construct a type.
    pub base: BaseType,

    /// All the qualifiers for the type.
    pub qualifiers: Vec<TypeQualifier>,

    /// Pointers
    pub pointers: u8,

    /// Array
    pub array: usize,
}

impl Type {
    /// Creates and returns a new `TypeBuilder` to construct a `Type` using the builder pattern.
    /// ```rust
    /// let t = Type::new(/*base type*/)
    ///     .build();
    /// ```
    pub fn new(base: BaseType) -> TypeBuilder {
        TypeBuilder::new(base)
    }

    /// Whether it's an array or not.
    pub fn is_array(&self) -> bool {
        self.array != 0
    }
}

impl Format for Type {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for q in &self.qualifiers {
            q.format(fmt)?;
            write!(fmt, " ")?;
        }

        self.base.format(fmt)?;

        write!(fmt, "{}", "*".repeat(self.pointers.into()))?;

        Ok(())
    }
}

/// A builder for constructing a `Type` instance.
pub struct TypeBuilder {
    base: BaseType,
    qualifiers: Vec<TypeQualifier>,
    pointers: u8,
    array: usize,
}

impl TypeBuilder {
    /// Creates and returns a new `TypeBuilder` to construct a `Type` using the builder pattern.
    pub fn new(base: BaseType) -> Self {
        Self {
            base,
            qualifiers: vec![],
            pointers: 0,
            array: 0,
        }
    }

    /// Adds a type qualifier to the type and returns the builder for chaining more operations.
    pub fn type_qualifier(mut self, q: TypeQualifier) -> Self {
        self.qualifiers.push(q);
        self
    }

    /// Makes the type volatile.
    pub fn make_volatile(self) -> Self {
        self.type_qualifier(TypeQualifier::Volatile)
    }

    /// Makes the type const.
    pub fn make_const(self) -> Self {
        self.type_qualifier(TypeQualifier::Const)
    }

    /// Makes the type a pointer.
    pub fn make_pointer(mut self) -> Self {
        self.pointers += 1;
        self
    }

    /// Makes the type an array with the given size.
    pub fn make_array(mut self, size: usize) -> Self {
        self.array = size;
        self
    }

    /// Consumes the builder and returns a `Type` containing all the information.
    pub fn build(self) -> Type {
        Type {
            base: self.base,
            qualifiers: self.qualifiers,
            pointers: self.pointers,
            array: self.array,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_type() {
        use BaseType::*;
        assert_eq!(Void.to_string(), "void");
        assert_eq!(Double.to_string(), "double");
        assert_eq!(Float.to_string(), "float");
        assert_eq!(Char.to_string(), "char");
        assert_eq!(Int.to_string(), "int");
        assert_eq!(UInt8.to_string(), "uint8_t");
        assert_eq!(UInt16.to_string(), "uint16_t");
        assert_eq!(UInt32.to_string(), "uint32_t");
        assert_eq!(UInt64.to_string(), "uint64_t");
        assert_eq!(Int8.to_string(), "int8_t");
        assert_eq!(Int16.to_string(), "int16_t");
        assert_eq!(Int32.to_string(), "int32_t");
        assert_eq!(Int64.to_string(), "int64_t");
        assert_eq!(Size.to_string(), "size_t");
        assert_eq!(UIntPtr.to_string(), "uintptr_t");
        assert_eq!(Bool.to_string(), "bool");
        assert_eq!(Enum("abc".to_string()).to_string(), "enum abc");
        assert_eq!(Struct("abc".to_string()).to_string(), "struct abc");
        assert_eq!(Union("abc".to_string()).to_string(), "union abc");
        assert_eq!(TypeDef("abc".to_string()).to_string(), "abc");
    }

    #[test]
    fn t() {
        use BaseType::*;
        let mut t = Type::new(Void).build();
        assert_eq!(t.to_string(), "void");

        t = Type::new(Void).make_pointer().make_pointer().build();
        assert_eq!(t.to_string(), "void**");

        t = Type::new(Void)
            .make_pointer()
            .make_pointer()
            .make_const()
            .build();
        assert_eq!(t.to_string(), "const void**");

        t = Type::new(Void)
            .make_pointer()
            .make_const()
            .make_array(10)
            .build();
        assert_eq!(t.to_string(), "const void*");
        assert!(t.is_array())
    }
}
