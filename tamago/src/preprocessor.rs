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

use crate::{Block, DocComment, Format, Formatter, GlobalStatement, Scope, Statement};
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

    pub fn new_with_str(path: &str) -> Self {
        Self {
            path: path.to_string(),
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

    pub fn new_system_with_str(path: &str) -> Self {
        Self {
            path: path.to_string(),
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
pub struct ErrorDirective {
    pub message: String,
}

impl ErrorDirective {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn new_with_str(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Format for ErrorDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#error \"{}\"", self.message)
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct PragmaDirective {
    pub raw: String,
}

impl PragmaDirective {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn new_with_str(raw: &str) -> Self {
        Self {
            raw: raw.to_string(),
        }
    }
}

impl Format for PragmaDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#pragma {}", self.raw)
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
#[derive(Debug, Clone, DisplayFromFormat)]
pub enum ScopeOrBlock {
    Scope(Scope),
    Block(Block),
}

impl Format for ScopeOrBlock {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ScopeOrBlock::Scope(s) => s.format(fmt),
            ScopeOrBlock::Block(b) => b.format(fmt),
        }
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct IfDirective {
    cond: String,
    then: ScopeOrBlock,
    other: Option<ScopeOrBlock>,
}

impl IfDirective {
    pub fn new(cond: String) -> Self {
        Self {
            cond,
            then: ScopeOrBlock::Scope(Scope::new()),
            other: None,
        }
    }

    pub fn new_with_empty_then_scope(cond: String) -> Self {
        Self::new(cond)
    }

    pub fn new_with_emtpy_then_block(cond: String) -> Self {
        Self {
            cond,
            then: ScopeOrBlock::Block(Block::new()),
            other: None,
        }
    }

    pub fn new_with_then_scope(cond: String, then: Scope) -> Self {
        Self {
            cond,
            then: ScopeOrBlock::Scope(then),
            other: None,
        }
    }

    pub fn new_with_then_block(cond: String, block: Block) -> Self {
        Self {
            cond,
            then: ScopeOrBlock::Block(block),
            other: None,
        }
    }

    pub fn push_global_statement(&mut self, global_stmt: GlobalStatement) -> &mut Self {
        match &mut self.then {
            ScopeOrBlock::Scope(then) => {
                then.push_global_statement(global_stmt);
            }
            ScopeOrBlock::Block(_) => {
                self.set_then(ScopeOrBlock::Scope(Scope::new_with_global_statements(
                    vec![global_stmt],
                )));
            }
        }

        self
    }

    pub fn push_block_statement(&mut self, stmt: Statement) -> &mut Self {
        match &mut self.then {
            ScopeOrBlock::Block(then) => {
                then.push_statement(stmt);
            }
            ScopeOrBlock::Scope(_) => {
                self.set_then(ScopeOrBlock::Block(Block::new_with_statements(vec![stmt])));
            }
        }

        self
    }

    pub fn set_then(&mut self, then: ScopeOrBlock) -> &mut Self {
        self.then = then;
        self
    }

    pub fn set_other(&mut self, other: ScopeOrBlock) -> &mut Self {
        self.other = Some(other);
        self
    }
}

impl Format for IfDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#if {}", self.cond)?;
        self.then.format(fmt)?;

        if let Some(other) = &self.other {
            writeln!(fmt, "#else")?;
            other.format(fmt)?;
        }

        writeln!(fmt, "#endif")
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct IfDefDirective {
    pub symbol: String,
    pub then: ScopeOrBlock,
    pub other: Option<ScopeOrBlock>,
    pub not: bool,
}

impl IfDefDirective {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            then: ScopeOrBlock::Scope(Scope::new()),
            other: None,
            not: false,
        }
    }

    pub fn new_with_empty_then_scope(symbol: String) -> Self {
        Self::new(symbol)
    }

    pub fn new_with_emtpy_then_block(symbol: String) -> Self {
        Self {
            symbol,
            then: ScopeOrBlock::Block(Block::new()),
            other: None,
            not: false,
        }
    }

    pub fn push_global_statement(&mut self, global_stmt: GlobalStatement) -> &mut Self {
        match &mut self.then {
            ScopeOrBlock::Scope(then) => {
                then.push_global_statement(global_stmt);
            }
            ScopeOrBlock::Block(_) => {
                self.set_then(ScopeOrBlock::Scope(Scope::new_with_global_statements(
                    vec![global_stmt],
                )));
            }
        }

        self
    }

    pub fn push_block_statement(&mut self, stmt: Statement) -> &mut Self {
        match &mut self.then {
            ScopeOrBlock::Block(then) => {
                then.push_statement(stmt);
            }
            ScopeOrBlock::Scope(_) => {
                self.set_then(ScopeOrBlock::Block(Block::new_with_statements(vec![stmt])));
            }
        }

        self
    }

    pub fn set_then(&mut self, then: ScopeOrBlock) -> &mut Self {
        self.then = then;
        self
    }

    pub fn set_other(&mut self, other: ScopeOrBlock) -> &mut Self {
        self.other = Some(other);
        self
    }

    pub fn set_not(&mut self) -> &mut Self {
        self.not = true;
        self
    }
}

impl Format for IfDefDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if self.not {
            writeln!(fmt, "ifndef {}", self.symbol)?;
        } else {
            writeln!(fmt, "#ifdef {}", self.symbol)?;
        }
        self.then.format(fmt)?;

        if let Some(other) = &self.other {
            writeln!(fmt, "#else")?;
            other.format(fmt)?;
        }

        writeln!(fmt, "#endif")
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct LineDirective {
    pub line: u64,
    pub path: String,
    pub is_system: bool,
    pub doc: Option<DocComment>,
}

impl LineDirective {
    pub fn new(line: u64, path: String) -> Self {
        Self {
            line,
            path,
            is_system: false,
            doc: None,
        }
    }

    pub fn new_system(line: u64, path: String) -> Self {
        Self {
            line,
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

impl Format for LineDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(doc) = &self.doc {
            doc.format(fmt)?;
        }

        write!(fmt, "#line {} ", self.line)?;

        if self.is_system {
            writeln!(fmt, "<{}>", self.path)?;
        } else {
            writeln!(fmt, "\"{}\"", self.path)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct WarningDirective {
    pub message: String,
}

impl WarningDirective {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn new_with_str(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Format for WarningDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#warning \"{}\"", self.message)
    }
}
