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

//! This module provides means to interact with preprocessor directives and C macros.

use std::fmt::{self, Write};

use crate::{DocComment, Format, Formatter};
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Include {
    pub path: String,
    pub is_system: bool,
    pub doc: Option<DocComment>,
}

impl Include {
    pub fn new(path: String) -> Self {
        Self {
            path,
            is_system: false,
            doc: None,
        }
    }

    pub fn new_system(path: String) -> Self {
        Self {
            path,
            is_system: true,
            doc: None,
        }
    }

    pub fn set_doc(&mut self, doc: DocComment) -> &mut Self {
        self.doc = Some(doc);
        self
    }
}

impl Format for Include {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        write!(fmt, "#include ")?;

        if self.is_system {
            writeln!(fmt, "<{}>", self.path)?;
        } else {
            writeln!(fmt, "\"{}\"", self.path)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Format for Error {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "#error {}", self.message)
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Pragma {
    pub raw: String,
}

impl Pragma {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }
}

impl Format for Pragma {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "#pragma {}", self.raw)
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub enum Macro {
    Obj(ObjMacro),
    Func(FuncMacro),
}

impl Format for Macro {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use Macro::*;
        match self {
            Obj(m) => m.format(fmt),
            Func(m) => m.format(fmt),
        }
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct ObjMacro {
    pub name: String,
    pub value: Option<String>,
    pub doc: Option<DocComment>,
}

impl ObjMacro {
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: None,
            doc: None,
        }
    }

    pub fn new_with_value(name: String, value: String) -> Self {
        Self {
            name,
            value: Some(value),
            doc: None,
        }
    }

    pub fn set_doc(&mut self, doc: DocComment) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.value = Some(value);
        self
    }
}

impl Format for ObjMacro {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        write!(fmt, "#define {}", self.name)?;

        if let Some(value) = &self.value {
            write!(fmt, " ")?;

            let lines = value.lines().collect::<Vec<&str>>();

            if lines.len() > 1 {
                fmt.indent(|fmt| {
                    for line in &lines[..lines.len() - 1] {
                        writeln!(fmt, "{line} \\")?;
                    }

                    if let Some(last) = lines.last() {
                        writeln!(fmt, "{last}")?;
                    }

                    Ok(())
                })
            } else {
                writeln!(fmt, "{value}")
            }
        } else {
            writeln!(fmt)
        }
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct FuncMacro {
    pub name: String,
    pub args: Vec<String>,
    pub value: String,
    pub doc: Option<DocComment>,
}

impl FuncMacro {
    pub fn new(name: String) -> Self {
        Self {
            name,
            args: vec![],
            value: "".to_string(),
            doc: None,
        }
    }

    pub fn new_with_args(name: String, args: Vec<String>) -> Self {
        Self {
            name,
            args,
            value: "".to_string(),
            doc: None,
        }
    }

    pub fn new_with_value(name: String, value: String) -> Self {
        Self {
            name,
            args: vec![],
            value,
            doc: None,
        }
    }

    pub fn set_doc(&mut self, doc: DocComment) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    pub fn push_arg(&mut self, arg: String) -> &mut Self {
        self.args.push(arg);
        self
    }

    pub fn push_variadic_arg(&mut self) -> &mut Self {
        self.push_arg("...".to_string())
    }

    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.value = value;
        self
    }
}

impl Format for FuncMacro {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        write!(fmt, "#define {}(", self.name)?;

        for arg in &self.args[..self.args.len() - 1] {
            write!(fmt, "{arg}, ")?;
        }

        if let Some(last) = self.args.last() {
            write!(fmt, "{last}")?;
        }

        writeln!(fmt, ") \\")?;

        let lines = self.value.lines().collect::<Vec<&str>>();

        if lines.len() > 1 {
            fmt.indent(|fmt| {
                for line in &lines[..lines.len() - 1] {
                    writeln!(fmt, "{line} \\")?;
                }

                if let Some(last) = lines.last() {
                    writeln!(fmt, "{last}")?;
                }

                Ok(())
            })
        } else {
            writeln!(fmt, "{}", self.value)
        }
    }
}
