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

//! This module provides tools for expressing and managing C types in Rust.
//!
//! It defines structures and utilities to represent C base types, type qualifiers,
//! and complex type constructions such as pointers and arrays. This is particularly
//! useful for generating C code or bindings programmatically from Rust.

use std::fmt::{self, Write};

use crate::{Format, Formatter};
use tamacro::DisplayFromFormat;

/// Represents all base types used in C.
///
/// The `BaseType` enum encapsulates the fundamental types in C, including primitive
/// types like `int` and `float`, sized integer types from `stdint.h`, and aggregate
/// types like `enum`, `struct`, and `union`. It serves as the foundation for building
/// more complex types.
///
/// # Examples
///
/// Basic integer type:
/// ```c
/// int
/// ```
///
/// Sized unsigned integer:
/// ```c
/// uint32_t
/// ```
///
/// Struct type:
/// ```c
/// struct Point
/// ```
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

    /// Represents the `bool` type from `stdbool.h`.
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
    ///
    /// This method maps the provided bit size to the corresponding unsigned integer type:
    /// - 8 -> `UInt8`
    /// - 16 -> `UInt16`
    /// - 32 -> `UInt32`
    /// - 64 -> `UInt64`
    /// Any other value defaults to `UInt64`.
    ///
    /// # Parameters
    ///
    /// * `bits` - The size of the unsigned integer in bits
    ///
    /// # Returns
    ///
    /// A `BaseType` instance representing the specified unsigned integer type
    ///
    /// # Examples
    ///
    /// ```rust
    /// let uint_type = BaseType::new_uint(32);
    /// assert_eq!(uint_type.to_string(), "uint32_t");
    /// ```
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
    ///
    /// This method maps the provided bit size to the corresponding signed integer type:
    /// - 8 -> `Int8`
    /// - 16 -> `Int16`
    /// - 32 -> `Int32`
    /// - 64 -> `Int64`
    /// Any other value defaults to `Int64`.
    ///
    /// # Parameters
    ///
    /// * `bits` - The size of the signed integer in bits
    ///
    /// # Returns
    ///
    /// A `BaseType` instance representing the specified signed integer type
    ///
    /// # Examples
    ///
    /// ```rust
    /// let int_type = BaseType::new_int(16);
    /// assert_eq!(int_type.to_string(), "int16_t");
    /// ```
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

    /// Checks whether the type is an integer type.
    ///
    /// This includes all signed and unsigned integer types, `char`, `size_t`, `uintptr_t`, and `bool`.
    ///
    /// # Returns
    ///
    /// `true` if the type is an integer type, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// let int_type = BaseType::Int;
    /// assert!(int_type.is_integer());
    /// let float_type = BaseType::Float;
    /// assert!(!float_type.is_integer());
    /// ```
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

    /// Checks whether the type is a tag type (`enum`, `struct`, `union`, or `typedef`).
    ///
    /// # Returns
    ///
    /// `true` if the type is a tag type, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// let struct_type = BaseType::Struct("Point".to_string());
    /// assert!(struct_type.is_tag_type());
    /// let int_type = BaseType::Int;
    /// assert!(!int_type.is_tag_type());
    /// ```
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

/// Represents type qualifiers in C.
///
/// The `TypeQualifier` enum encapsulates C's type qualifiers `const` and `volatile`,
/// which modify the behavior of variables or pointers in a C program.
///
/// # Examples
///
/// Const qualifier:
/// ```c
/// const int
/// ```
///
/// Volatile qualifier:
/// ```c
/// volatile char
/// ```
#[derive(Debug, Clone, Copy, DisplayFromFormat)]
pub enum TypeQualifier {
    /// The `volatile` keyword, indicating the variable may change unexpectedly.
    Volatile,

    /// The `const` keyword, indicating the variable's value cannot be modified after initialization.
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

/// Represents a complete type in C, including base type, qualifiers, pointers, and arrays.
///
/// The `Type` struct combines a `BaseType` with optional qualifiers, pointer levels,
/// and array sizes to fully describe a C type as it would appear in a declaration.
///
/// # Examples
///
/// Pointer to const char:
/// ```c
/// const char*
/// ```
///
/// Array of integers:
/// ```c
/// int[10]
/// ```
///
/// Double pointer to volatile float:
/// ```c
/// volatile float**
/// ```
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
    ///
    /// This method provides a fluent interface for defining complex C types incrementally.
    ///
    /// # Parameters
    ///
    /// * `base` - The base type to start with
    ///
    /// # Returns
    ///
    /// A `TypeBuilder` instance for configuring and building a `Type`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let t = Type::new(BaseType::Char)
    ///     .make_const()
    ///     .make_pointer()
    ///     .build();
    /// assert_eq!(t.to_string(), "const char*");
    /// ```
    pub fn new(base: BaseType) -> TypeBuilder {
        TypeBuilder::new(base)
    }

    /// Checks whether the type is an array.
    ///
    /// # Returns
    ///
    /// `true` if the type is an array (array size > 0), `false` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// let array_type = Type::new(BaseType::Int).make_array(5).build();
    /// assert!(array_type.is_array());
    /// let simple_type = Type::new(BaseType::Int).build();
    /// assert!(!simple_type.is_array());
    /// ```
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

/// A builder for constructing a `Type` instance with a fluent interface.
///
/// The `TypeBuilder` allows incremental configuration of a C type's properties,
/// such as qualifiers, pointers, and array sizes, before finalizing the type.
pub struct TypeBuilder {
    base: BaseType,
    qualifiers: Vec<TypeQualifier>,
    pointers: u8,
    array: usize,
}

impl TypeBuilder {
    /// Creates and returns a new `TypeBuilder` to construct a `Type`.
    ///
    /// # Parameters
    ///
    /// * `base` - The base type to start with
    ///
    /// # Returns
    ///
    /// A new `TypeBuilder` instance with default values for qualifiers, pointers, and array size
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = TypeBuilder::new(BaseType::Int);
    /// ```
    pub fn new(base: BaseType) -> Self {
        Self {
            base,
            qualifiers: vec![],
            pointers: 0,
            array: 0,
        }
    }

    /// Adds a type qualifier to the type being built.
    ///
    /// # Parameters
    ///
    /// * `q` - The type qualifier to add
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = TypeBuilder::new(BaseType::Float)
    ///     .type_qualifier(TypeQualifier::Const);
    /// ```
    pub fn type_qualifier(mut self, q: TypeQualifier) -> Self {
        self.qualifiers.push(q);
        self
    }

    /// Makes the type volatile.
    ///
    /// Adds the `volatile` qualifier to the type.
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = TypeBuilder::new(BaseType::Int).make_volatile();
    /// assert_eq!(builder.build().to_string(), "volatile int");
    /// ```
    pub fn make_volatile(self) -> Self {
        self.type_qualifier(TypeQualifier::Volatile)
    }

    /// Makes the type const.
    ///
    /// Adds the `const` qualifier to the type.
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = TypeBuilder::new(BaseType::Char).make_const();
    /// assert_eq!(builder.build().to_string(), "const char");
    /// ```
    pub fn make_const(self) -> Self {
        self.type_qualifier(TypeQualifier::Const)
    }

    /// Makes the type a pointer.
    ///
    /// Increases the pointer level by one.
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = TypeBuilder::new(BaseType::Void).make_pointer();
    /// assert_eq!(builder.build().to_string(), "void*");
    /// ```
    pub fn make_pointer(mut self) -> Self {
        self.pointers += 1;
        self
    }

    /// Makes the type an array with the given size.
    ///
    /// # Parameters
    ///
    /// * `size` - The size of the array
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = TypeBuilder::new(BaseType::Int).make_array(10);
    /// ```
    pub fn make_array(mut self, size: usize) -> Self {
        self.array = size;
        self
    }

    /// Finalizes the type definition and returns a fully constructed `Type`.
    ///
    /// # Returns
    ///
    /// A fully constructed `Type` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let t = TypeBuilder::new(BaseType::Double)
    ///     .make_const()
    ///     .make_pointer()
    ///     .build();
    /// assert_eq!(t.to_string(), "const double*");
    /// ```
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
