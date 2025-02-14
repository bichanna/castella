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

//! This module is provides ways to express types.

use std::fmt::{self, Write};

use crate::formatter::{Format, Formatter};
use tamacro::DisplayFromFormat;

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

#[derive(Debug, Clone, Copy, DisplayFromFormat)]
pub enum LeftTypeQualifier {
    Volatile,
    Const,
}

impl Format for LeftTypeQualifier {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use LeftTypeQualifier::*;
        match self {
            Volatile => write!(fmt, "volatile"),
            Const => write!(fmt, "const"),
        }
    }
}

#[derive(Debug, Clone, Copy, DisplayFromFormat)]
pub enum RightTypeQualifier {
    Pointer,
    Array(usize),
}

impl Format for RightTypeQualifier {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use RightTypeQualifier::*;
        match self {
            Pointer => write!(fmt, "*"),
            Array(size) => write!(fmt, "[{size}]"),
        }
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Type {
    base: BaseType,
    left_qualifiers: Vec<LeftTypeQualifier>,
    right_qualifiers: Vec<RightTypeQualifier>,
}

impl Type {
    pub fn new(base: BaseType) -> Self {
        Self {
            base,
            left_qualifiers: vec![],
            right_qualifiers: vec![],
        }
    }

    pub fn push_left_type_qualifier(&mut self, q: LeftTypeQualifier) -> &mut Self {
        self.left_qualifiers.push(q);
        self
    }

    pub fn push_right_type_qualifier(&mut self, q: RightTypeQualifier) -> &mut Self {
        self.right_qualifiers.push(q);
        self
    }

    pub fn make_volatile(&mut self) -> &mut Self {
        self.push_left_type_qualifier(LeftTypeQualifier::Volatile)
    }

    pub fn make_const(&mut self) -> &mut Self {
        self.push_left_type_qualifier(LeftTypeQualifier::Const)
    }

    pub fn make_pointer(&mut self) -> &mut Self {
        self.push_right_type_qualifier(RightTypeQualifier::Pointer)
    }

    pub fn make_array(&mut self, size: usize) -> &mut Self {
        self.push_right_type_qualifier(RightTypeQualifier::Array(size))
    }
}

impl Format for Type {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for q in &self.left_qualifiers {
            q.format(fmt)?;
            write!(fmt, " ")?;
        }

        self.base.format(fmt)?;

        for q in &self.right_qualifiers {
            q.format(fmt)?;
        }

        Ok(())
    }
}
