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

use crate::*;
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Include {
    pub path: String,
    pub is_system: bool,
    pub doc: Option<DocComment>,
}

impl Include {
    pub fn new(path: String) -> IncludeBuilder {
        IncludeBuilder::new(path)
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

pub struct IncludeBuilder {
    path: String,
    is_system: bool,
    doc: Option<DocComment>,
}

impl IncludeBuilder {
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

    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    pub fn build(self) -> Include {
        Include {
            path: self.path,
            is_system: self.is_system,
            doc: self.doc,
        }
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct ErrorDirective {
    pub message: String,
}

impl ErrorDirective {
    pub fn new(message: String) -> ErrorDirectiveBuilder {
        ErrorDirectiveBuilder::new(message)
    }
}

impl Format for ErrorDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#error \"{}\"", self.message)
    }
}

pub struct ErrorDirectiveBuilder {
    message: String,
}

impl ErrorDirectiveBuilder {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn new_with_str(message: &str) -> Self {
        Self::new(message.to_string())
    }

    pub fn build(self) -> ErrorDirective {
        ErrorDirective {
            message: self.message,
        }
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct PragmaDirective {
    pub raw: String,
}

impl PragmaDirective {
    pub fn new(raw: String) -> PragmaDirectiveBuilder {
        PragmaDirectiveBuilder::new(raw)
    }
}

impl Format for PragmaDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#pragma {}", self.raw)
    }
}

pub struct PragmaDirectiveBuilder {
    pub raw: String,
}

impl PragmaDirectiveBuilder {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn new_with_str(raw: &str) -> Self {
        Self::new(raw.to_string())
    }

    pub fn build(self) -> PragmaDirective {
        PragmaDirective { raw: self.raw }
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
    pub fn new(name: String) -> ObjMacroBuilder {
        ObjMacroBuilder::new(name)
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

pub struct ObjMacroBuilder {
    name: String,
    value: Option<String>,
    doc: Option<DocComment>,
}

impl ObjMacroBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: None,
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

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    pub fn value_with_str(self, value: &str) -> Self {
        self.value(value.to_string())
    }

    pub fn build(self) -> ObjMacro {
        ObjMacro {
            name: self.name,
            value: self.value,
            doc: self.doc,
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
    pub fn new(name: String) -> FuncMacroBuilder {
        FuncMacroBuilder::new(name)
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

        write!(fmt, ") ")?;

        let lines = self.value.lines().collect::<Vec<&str>>();

        if lines.len() > 1 {
            writeln!(fmt, "\\")?;

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

pub struct FuncMacroBuilder {
    name: String,
    args: Vec<String>,
    value: String,
    doc: Option<DocComment>,
}

impl FuncMacroBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            args: vec![],
            value: "".to_string(),
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

    pub fn arg(mut self, arg: String) -> Self {
        self.args.push(arg);
        self
    }

    pub fn arg_with_str(self, arg: &str) -> Self {
        self.arg(arg.to_string())
    }

    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn variadic_arg(self) -> Self {
        self.arg_with_str("...")
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    pub fn value_with_str(self, value: &str) -> Self {
        self.value(value.to_string())
    }

    pub fn build(self) -> FuncMacro {
        FuncMacro {
            name: self.name,
            args: self.args,
            value: self.value,
            doc: self.doc,
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
    pub cond: String,
    pub then: ScopeOrBlock,
    pub other: Option<ScopeOrBlock>,
}

impl IfDirective {
    pub fn new(cond: String) -> IfDirectiveBuilder {
        IfDirectiveBuilder::new(cond)
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

pub struct IfDirectiveBuilder {
    cond: String,
    then: ScopeOrBlock,
    other: Option<ScopeOrBlock>,
}

impl IfDirectiveBuilder {
    pub fn new(cond: String) -> Self {
        Self {
            cond,
            then: ScopeOrBlock::Scope(Scope::new().build()),
            other: None,
        }
    }

    pub fn new_with_str(cond: &str) -> Self {
        Self::new(cond.to_string())
    }

    pub fn global_statement(mut self, global_stmt: GlobalStatement) -> Self {
        match &mut self.then {
            ScopeOrBlock::Scope(then) => {
                then.global_stmts.push(global_stmt);
                self
            }
            ScopeOrBlock::Block(_) => self.then(ScopeOrBlock::Scope(
                Scope::new().global_statement(global_stmt).build(),
            )),
        }
    }

    pub fn block_statement(mut self, stmt: Statement) -> Self {
        match &mut self.then {
            ScopeOrBlock::Block(then) => {
                then.stmts.push(stmt);
                self
            }
            ScopeOrBlock::Scope(_) => self.then(ScopeOrBlock::Block(
                Block::new().statements(vec![stmt]).build(),
            )),
        }
    }

    pub fn then(mut self, then: ScopeOrBlock) -> Self {
        self.then = then;
        self
    }

    pub fn other(mut self, other: ScopeOrBlock) -> Self {
        self.other = Some(other);
        self
    }

    pub fn build(self) -> IfDirective {
        IfDirective {
            cond: self.cond,
            then: self.then,
            other: self.other,
        }
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
    pub fn new(symbol: String) -> IfDefDirectiveBuilder {
        IfDefDirectiveBuilder::new(symbol)
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

pub struct IfDefDirectiveBuilder {
    symbol: String,
    then: ScopeOrBlock,
    other: Option<ScopeOrBlock>,
    not: bool,
}

impl IfDefDirectiveBuilder {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            then: ScopeOrBlock::Scope(Scope::new().build()),
            other: None,
            not: false,
        }
    }

    pub fn new_with_str(symbol: &str) -> Self {
        Self::new(symbol.to_string())
    }

    pub fn global_statement(mut self, global_stmt: GlobalStatement) -> Self {
        match &mut self.then {
            ScopeOrBlock::Scope(then) => {
                then.global_stmts.push(global_stmt);
                self
            }
            ScopeOrBlock::Block(_) => self.then(ScopeOrBlock::Scope(
                Scope::new().global_statement(global_stmt).build(),
            )),
        }
    }

    pub fn block_statement(mut self, stmt: Statement) -> Self {
        match &mut self.then {
            ScopeOrBlock::Block(then) => {
                then.stmts.push(stmt);
                self
            }
            ScopeOrBlock::Scope(_) => {
                self.then(ScopeOrBlock::Block(Block::new().statement(stmt).build()))
            }
        }
    }

    pub fn then(mut self, then: ScopeOrBlock) -> Self {
        self.then = then;
        self
    }

    pub fn other(mut self, other: ScopeOrBlock) -> Self {
        self.other = Some(other);
        self
    }

    pub fn not(mut self) -> Self {
        self.not = true;
        self
    }

    pub fn build(self) -> IfDefDirective {
        IfDefDirective {
            symbol: self.symbol,
            then: self.then,
            other: self.other,
            not: self.not,
        }
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
    pub fn new(line: u64, path: String) -> LineDirectiveBuilder {
        LineDirectiveBuilder::new(line, path)
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

pub struct LineDirectiveBuilder {
    line: u64,
    path: String,
    is_system: bool,
    doc: Option<DocComment>,
}

impl LineDirectiveBuilder {
    pub fn new(line: u64, path: String) -> Self {
        Self {
            line,
            path,
            is_system: false,
            doc: None,
        }
    }

    pub fn new_with_str(line: u64, path: &str) -> Self {
        Self::new(line, path.to_string())
    }

    pub fn doc(mut self, doc: DocComment) -> Self {
        self.doc = Some(doc);
        self
    }

    pub fn system(mut self) -> Self {
        self.is_system = true;
        self
    }

    pub fn build(self) -> LineDirective {
        LineDirective {
            line: self.line,
            path: self.path,
            is_system: self.is_system,
            doc: self.doc,
        }
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct WarningDirective {
    pub message: String,
}

impl WarningDirective {
    pub fn new(message: String) -> WarningDirectiveBuilder {
        WarningDirectiveBuilder::new(message)
    }
}

impl Format for WarningDirective {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#warning \"{}\"", self.message)
    }
}

pub struct WarningDirectiveBuilder {
    message: String,
}

impl WarningDirectiveBuilder {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn new_with_str(message: &str) -> Self {
        Self::new(message.to_string())
    }

    pub fn build(self) -> WarningDirective {
        WarningDirective {
            message: self.message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn include() {
        let i = IncludeBuilder::new_with_str("./some_header.h")
            .doc(
                DocCommentBuilder::new()
                    .line_str("importing some_header")
                    .build(),
            )
            .build();
        let res = r#"/// importing some_header
#include "./some_header.h"
"#;
        assert_eq!(i.to_string(), res);

        let i2 = IncludeBuilder::new_system_with_str("stdio.h").build();
        let res = "#include <stdio.h>\n";
        assert_eq!(i2.to_string(), res);
    }

    #[test]
    fn error_directive() {
        let e = ErrorDirectiveBuilder::new_with_str("some kinda compile time error").build();
        let res = "#error \"some kinda compile time error\"\n";
        assert_eq!(e.to_string(), res);
    }

    #[test]
    fn pragma_directive() {
        let p = PragmaDirectiveBuilder::new_with_str("once").build();
        let res = "#pragma once\n";
        assert_eq!(p.to_string(), res);
    }

    #[test]
    fn macros() {
        let obj_m = Macro::Obj(
            ObjMacroBuilder::new_with_str("YEAR")
                .value_with_str("2025")
                .build(),
        );
        let res = "#define YEAR 2025\n";
        assert_eq!(obj_m.to_string(), res);

        let func_m = Macro::Func(
            FuncMacroBuilder::new_with_str("AREA")
                .arg_with_str("width")
                .arg_with_str("height")
                .value_with_str("(width) * (height)")
                .build(),
        );
        let res2 = "#define AREA(width, height) (width) * (height)\n";
        assert_eq!(func_m.to_string(), res2);

        let func_m2 = Macro::Func(
            FuncMacroBuilder::new_with_str("SOMETHING")
                .arg_with_str("a")
                .arg_with_str("b")
                .arg_with_str("c")
                .value_with_str("abc\nabc\nanother")
                .build(),
        );
        let res3 = r#"#define SOMETHING(a, b, c) \
  abc \
  abc \
  another
"#;
        assert_eq!(func_m2.to_string(), res3);
    }

    #[test]
    fn if_directive() {
        let i = IfDirectiveBuilder::new_with_str("SOMETHING")
            .block_statement(Statement::Expr(Expr::new_ident_with_str("identifier")))
            .block_statement(Statement::ErrorDirective(
                ErrorDirectiveBuilder::new_with_str("some error").build(),
            ))
            .build();
        let res = r#"#if SOMETHING
identifier;
#error "some error"
#endif
"#;
        assert_eq!(i.to_string(), res);
    }

    #[test]
    fn if_def_directive() {
        let i = IfDefDirectiveBuilder::new_with_str("SOMETHING")
            .global_statement(GlobalStatement::NewLine)
            .not()
            .build();
        let res = r#"ifndef SOMETHING

#endif
"#;
        assert_eq!(i.to_string(), res);
    }

    #[test]
    fn line_directive() {
        let l = LineDirectiveBuilder::new_with_str(123, "hello.h").build();
        let res = "#line 123 \"hello.h\"\n";
        assert_eq!(l.to_string(), res);
    }

    #[test]
    fn warning_directive() {
        let l = WarningDirectiveBuilder::new_with_str("some warning message").build();
        let res = "#warning \"some warning message\"\n";
        assert_eq!(l.to_string(), res);
    }
}
