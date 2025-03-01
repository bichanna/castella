// Copyright (c) 2025 Nobuharu Shimazu
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
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

//! # Enum Definition Module
//!
//! This module provides abstractions and builders for creating C-style enum definitions.
//! It enables programmatic generation of complete enum declarations with variants,
//! optional documentation comments, and explicit values for enum constants.
//!
//! The module contains:
//! - `Enum`: Represents a complete C enum declaration
//! - `Variant`: Represents an individual enum variant with optional value
//!
//! Both structures come with corresponding builder patterns to facilitate construction
//! of enum declarations.

use std::fmt::{self, Write};

use crate::{BaseType, DocComment, Format, Formatter, Type};
use tamacro::DisplayFromFormat;

/// Represents a complete enum declaration in C programming language.
///
/// An enum in C is a user-defined type consisting of a set of named integer constants.
/// This structure models the complete enum definition, including its name, variants,
/// and optional documentation.
///
/// # Examples
///
/// Basic enum with sequential values:
/// ```rust
/// let enum_def = Enum::new("Color".to_string())
///     .variant(Variant::new("RED".to_string()).build())
///     .variant(Variant::new("GREEN".to_string()).build())
///     .variant(Variant::new("BLUE".to_string()).build())
///     .build();
/// ```
///
/// This would generate C code like:
/// ```c
/// enum Color {
///   RED,
///   GREEN,
///   BLUE,
/// };
/// ```
///
/// Enum with explicit values:
/// ```rust
/// let enum_def = Enum::new("HttpStatus".to_string())
///     .variant(Variant::new("OK".to_string()).value(200).build())
///     .variant(Variant::new("NOT_FOUND".to_string()).value(404).build())
///     .variant(Variant::new("SERVER_ERROR".to_string()).value(500).build())
///     .build();
/// ```
///
/// This would generate C code like:
/// ```c
/// enum HttpStatus {
///   OK = 200,
///   NOT_FOUND = 404,
///   SERVER_ERROR = 500,
/// };
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Enum {
    /// The identifier name for the enum type
    pub name: String,

    /// The collection of enum variants (constants) defined within this enum
    pub variants: Vec<Variant>,

    /// Optional documentation comment for the enum
    pub doc: Option<DocComment>,
}

impl Enum {
    /// Creates and returns a new `EnumBuilder` to construct an enum using the builder pattern.
    ///
    /// This method provides a convenient entry point to start building an enum declaration.
    /// The returned builder allows for fluent and clear construction of enum types.
    ///
    /// # Parameters
    /// - `name`: The identifier name for the enum type
    ///
    /// # Returns
    /// A new `EnumBuilder` instance initialized with the given name
    ///
    /// # Examples
    /// ```rust
    /// let enum_def = Enum::new("Direction".to_string())
    ///     .variant(Variant::new("NORTH".to_string()).build())
    ///     .variant(Variant::new("EAST".to_string()).build())
    ///     .variant(Variant::new("SOUTH".to_string()).build())
    ///     .variant(Variant::new("WEST".to_string()).build())
    ///     .build();
    /// ```
    pub fn new(name: String) -> EnumBuilder {
        EnumBuilder::new(name)
    }

    /// Converts this enum definition into its corresponding type representation.
    ///
    /// This method creates a Type instance that represents the enum type, which can be
    /// used when declaring variables or parameters of this enum type.
    ///
    /// # Returns
    /// A `Type` instance representing this enum type
    ///
    /// # Examples
    /// ```rust
    /// let enum_def = Enum::new("Color".to_string())
    ///     .variant(Variant::new("RED".to_string()).build())
    ///     .variant(Variant::new("GREEN".to_string()).build())
    ///     .variant(Variant::new("BLUE".to_string()).build())
    ///     .build();
    ///
    /// let color_type = enum_def.to_type();
    /// ```
    pub fn to_type(&self) -> Type {
        Type::new(BaseType::Enum(self.name.clone())).build()
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

/// A builder for constructing an `Enum` instance incrementally.
///
/// This builder provides an API for creating enum declarations with multiple
/// variants and optional documentation. It follows the builder pattern to make
/// the construction process more readable and easier to maintain.
pub struct EnumBuilder {
    name: String,
    variants: Vec<Variant>,
    doc: Option<DocComment>,
}

impl EnumBuilder {
    /// Creates and returns a new `EnumBuilder` to construct an enum declaration.
    ///
    /// This method initializes a builder with just the enum name and no variants.
    ///
    /// # Parameters
    /// - `name`: The identifier name for the enum type
    ///
    /// # Returns
    /// A new `EnumBuilder` instance initialized with the given name
    ///
    /// # Examples
    /// ```rust
    /// let builder = EnumBuilder::new("Direction".to_string());
    /// // Further configure the builder...
    /// let enum_def = builder
    ///     .variant(Variant::new("NORTH".to_string()).build())
    ///     .build();
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            variants: vec![],
            doc: None,
        }
    }

    /// Creates and returns a new `EnumBuilder` with a string slice as the enum name.
    ///
    /// This is a convenience constructor that converts the string slice to a `String` internally.
    ///
    /// # Parameters
    /// - `name`: String slice containing the identifier name for the enum type
    ///
    /// # Returns
    /// A new `EnumBuilder` instance initialized with the given name
    ///
    /// # Examples
    /// ```rust
    /// let builder = EnumBuilder::new_with_str("Direction");
    /// // Further configure the builder...
    /// let enum_def = builder
    ///     .variant(Variant::new_with_str("NORTH").build())
    ///     .build();
    /// ```
    pub fn new_with_str(name: &str) -> Self {
        Self {
            name: name.to_string(),
            variants: vec![],
            doc: None,
        }
    }

    /// Sets the documentation comment for the enum declaration.
    ///
    /// This method specifies an optional documentation comment that will be included
    /// before the enum declaration in the generated code.
    ///
    /// # Parameters
    /// - `doc`: The documentation comment to associate with the enum
    ///
    /// # Returns
    /// `self` for method chaining
    ///
    /// # Examples
    /// ```rust
    /// use crate::DocComment;
    ///
    /// let enum_def = EnumBuilder::new_with_str("LogLevel")
    ///     .doc(DocComment::new("Log severity levels for the application".to_string()).build())
    ///     .variant(Variant::new_with_str("DEBUG").build())
    ///     .variant(Variant::new_with_str("INFO").build())
    ///     .variant(Variant::new_with_str("WARNING").build())
    ///     .variant(Variant::new_with_str("ERROR").build())
    ///     .build();
    /// ```
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Adds a single variant to the enum declaration.
    ///
    /// This method appends a new variant to the list of enum variants.
    ///
    /// # Parameters
    /// - `variant`: The enum variant to add
    ///
    /// # Returns
    /// `self` for method chaining
    ///
    /// # Examples
    /// ```rust
    /// let enum_def = EnumBuilder::new_with_str("Direction")
    ///     .variant(Variant::new_with_str("NORTH").build())
    ///     .variant(Variant::new_with_str("EAST").build())
    ///     .variant(Variant::new_with_str("SOUTH").build())
    ///     .variant(Variant::new_with_str("WEST").build())
    ///     .build();
    /// ```
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variants.push(variant);
        self
    }

    /// Sets all the variants for the enum declaration at once.
    ///
    /// This method replaces any existing variants with the provided collection.
    ///
    /// # Parameters
    /// - `variants`: A vector of variants to include in the enum
    ///
    /// # Returns
    /// `self` for method chaining
    ///
    /// # Examples
    /// ```rust
    /// let month_variants = vec![
    ///     Variant::new_with_str("JAN").build(),
    ///     Variant::new_with_str("FEB").build(),
    ///     Variant::new_with_str("MAR").build(),
    ///     // ... other months
    ///     Variant::new_with_str("DEC").build(),
    /// ];
    ///
    /// let enum_def = EnumBuilder::new_with_str("Month")
    ///     .variants(month_variants)
    ///     .build();
    /// ```
    pub fn variants(mut self, variants: Vec<Variant>) -> Self {
        self.variants = variants;
        self
    }

    /// Finalizes the building process and returns the constructed `Enum` declaration.
    ///
    /// This method consumes the builder and produces an `Enum` instance with all
    /// the properties configured during the building process.
    ///
    /// # Returns
    /// A new `Enum` instance with the configured properties
    ///
    /// # Examples
    /// ```rust
    /// let enum_def = EnumBuilder::new_with_str("Boolean")
    ///     .variant(Variant::new_with_str("FALSE").value(0).build())
    ///     .variant(Variant::new_with_str("TRUE").value(1).build())
    ///     .build();
    /// ```
    pub fn build(self) -> Enum {
        Enum {
            name: self.name,
            variants: self.variants,
            doc: self.doc,
        }
    }
}

/// Represents an individual enum variant (constant) within a C enum declaration.
///
/// Each variant has a name and can optionally have an explicit integer value.
/// If no explicit value is provided, C will automatically assign sequential
/// values starting from 0 or continuing from the last explicit value.
///
/// # Examples
///
/// Basic variant without explicit value:
/// ```rust
/// let variant = Variant::new("RED".to_string()).build();
/// ```
///
/// Variant with explicit value:
/// ```rust
/// let variant = Variant::new("NOT_FOUND".to_string()).value(404).build();
/// ```
///
/// Variant with documentation:
/// ```rust
/// use crate::DocComment;
///
/// let variant = Variant::new("SUCCESS".to_string())
///     .value(0)
///     .doc(DocComment::new("Operation completed successfully".to_string()).build())
///     .build();
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Variant {
    /// The identifier name for this enum constant
    pub name: String,

    /// Optional explicit integer value for this enum constant
    pub value: Option<i64>,

    /// Optional documentation comment for this variant
    pub doc: Option<DocComment>,
}

impl Variant {
    /// Creates and returns a new `VariantBuilder` to construct an enum variant.
    ///
    /// This method provides a convenient entry point to start building an enum variant.
    /// The returned builder allows for fluent and clear construction of enum constants.
    ///
    /// # Parameters
    /// - `name`: The identifier name for the enum constant
    ///
    /// # Returns
    /// A new `VariantBuilder` instance initialized with the given name
    ///
    /// # Examples
    /// ```rust
    /// let variant = Variant::new("ERROR".to_string())
    ///     .value(-1)
    ///     .build();
    /// ```
    pub fn new(name: String) -> VariantBuilder {
        VariantBuilder::new(name)
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

/// A builder for constructing a `Variant` instance incrementally.
///
/// This builder provides a fluent API for creating enum variants with optional
/// explicit values and documentation. It follows the builder pattern to make
/// the construction process more readable and easier to maintain.
pub struct VariantBuilder {
    name: String,
    value: Option<i64>,
    doc: Option<DocComment>,
}

impl VariantBuilder {
    /// Creates and returns a new `VariantBuilder` to construct an enum variant.
    ///
    /// This method initializes a builder with just the variant name and no explicit value.
    ///
    /// # Parameters
    /// - `name`: The identifier name for the enum constant
    ///
    /// # Returns
    /// A new `VariantBuilder` instance initialized with the given name
    ///
    /// # Examples
    /// ```rust
    /// let builder = VariantBuilder::new("SUCCESS".to_string());
    /// // Further configure the builder...
    /// let variant = builder
    ///     .value(0)
    ///     .build();
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: None,
            doc: None,
        }
    }

    /// Creates and returns a new `VariantBuilder` with a string slice as the variant name.
    ///
    /// This is a convenience constructor that converts the string slice to a `String` internally.
    ///
    /// # Parameters
    /// - `name`: String slice containing the identifier name for the enum constant
    ///
    /// # Returns
    /// A new `VariantBuilder` instance initialized with the given name
    ///
    /// # Examples
    /// ```rust
    /// let builder = VariantBuilder::new_with_str("SUCCESS");
    /// // Further configure the builder...
    /// let variant = builder
    ///     .value(0)
    ///     .build();
    /// ```
    pub fn new_with_str(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: None,
            doc: None,
        }
    }

    /// Sets the documentation comment for the enum variant.
    ///
    /// This method specifies an optional documentation comment that will be included
    /// before the variant declaration in the generated code.
    ///
    /// # Parameters
    /// - `doc`: The documentation comment to associate with the variant
    ///
    /// # Returns
    /// `self` for method chaining
    ///
    /// # Examples
    /// ```rust
    /// use crate::DocComment;
    ///
    /// let variant = VariantBuilder::new_with_str("SUCCESS")
    ///     .doc(DocComment::new("Operation completed successfully".to_string()).build())
    ///     .value(0)
    ///     .build();
    /// ```
    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Sets the explicit integer value for the enum variant.
    ///
    /// In C, enum variants can have explicit values assigned. If not specified,
    /// C will automatically assign sequential values starting from 0 or continuing
    /// from the last explicit value.
    ///
    /// # Parameters
    /// - `value`: The explicit integer value to assign to this enum constant
    ///
    /// # Returns
    /// `self` for method chaining
    ///
    /// # Examples
    /// ```rust
    /// let variant = VariantBuilder::new_with_str("NOT_FOUND")
    ///     .value(404)
    ///     .build();
    /// ```
    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// Finalizes the building process and returns the constructed `Variant`.
    ///
    /// This method consumes the builder and produces a `Variant` instance with all
    /// the properties configured during the building process.
    ///
    /// # Returns
    /// A new `Variant` instance with the configured properties
    ///
    /// # Examples
    /// ```rust
    /// let variant = VariantBuilder::new_with_str("INTERNAL_ERROR")
    ///     .value(500)
    ///     .build();
    /// ```
    pub fn build(self) -> Variant {
        Variant {
            name: self.name,
            value: self.value,
            doc: self.doc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_stmt() {
        let e = EnumBuilder::new_with_str("Weekday")
            .variants(vec![
                VariantBuilder::new_with_str("MONDAY").build(),
                VariantBuilder::new_with_str("TUESDAY").build(),
                VariantBuilder::new_with_str("WEDNESDAY").build(),
                VariantBuilder::new_with_str("THURSDAY").build(),
                VariantBuilder::new_with_str("FRIDAY").build(),
            ])
            .build();
        let res = r#"enum Weekday {
  MONDAY,
  TUESDAY,
  WEDNESDAY,
  THURSDAY,
  FRIDAY,
};
"#;
        assert_eq!(e.to_string(), res);
    }
}
