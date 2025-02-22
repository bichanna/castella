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

//! This module provides means to create C enums.

use std::fmt::{self, Write};

use crate::{BaseType, DocComment, Format, Formatter, Type};
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<Variant>,
    pub doc: Option<DocComment>,
}

impl Enum {
    pub fn new(name: String) -> EnumBuilder {
        EnumBuilder::new(name)
    }

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

pub struct EnumBuilder {
    name: String,
    variants: Vec<Variant>,
    doc: Option<DocComment>,
}

impl EnumBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            variants: vec![],
            doc: None,
        }
    }

    pub fn new_with_str(name: &str) -> Self {
        Self {
            name: name.to_string(),
            variants: vec![],
            doc: None,
        }
    }

    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variants.push(variant);
        self
    }

    pub fn variants(mut self, variants: Vec<Variant>) -> Self {
        self.variants = variants;
        self
    }

    pub fn build(self) -> Enum {
        Enum {
            name: self.name,
            variants: self.variants,
            doc: self.doc,
        }
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Variant {
    pub name: String,
    pub value: Option<i64>,
    pub doc: Option<DocComment>,
}

impl Variant {
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

pub struct VariantBuilder {
    name: String,
    value: Option<i64>,
    doc: Option<DocComment>,
}

impl VariantBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: None,
            doc: None,
        }
    }

    pub fn new_with_str(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: None,
            doc: None,
        }
    }

    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

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
