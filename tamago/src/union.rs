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

//! This module provides make C unions.

use std::fmt::{self, Write};

use crate::{BaseType, DocComment, Field, Format, Formatter, Type};
use tamacro::DisplayFromFormat;

/// Represents a `union` in C.
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Union {
    /// The name of the union.
    pub name: String,

    /// The fields of the union.
    pub fields: Vec<Field>,

    /// The optional doc comment.
    pub doc: Option<DocComment>,
}

impl Union {
    pub fn new(name: String) -> UnionBuilder {
        UnionBuilder::new(name)
    }

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

pub struct UnionBuilder {
    name: String,
    fields: Vec<Field>,
    doc: Option<DocComment>,
}

impl UnionBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: vec![],
            doc: None,
        }
    }

    pub fn new_with_str(name: &str) -> Self {
        Self::new(name.to_string())
    }

    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    pub fn fields(mut self, fields: Vec<Field>) -> Self {
        self.fields = fields;
        self
    }

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
