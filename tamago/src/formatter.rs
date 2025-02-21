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

//! This module provides a formatter implementation that emits a string of C code with the right
//! indentation.

use std::fmt::{self, Write};

const DEFAULT_INDENT: usize = 2;

pub trait Format {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result;
}

/// Formatter for a scope
#[derive(Debug)]
pub struct Formatter<'a> {
    /// The buffer for the formatter
    dst: &'a mut String,

    /// The current identation level
    pub spaces: usize,

    /// The current scope
    scope: Vec<String>,

    /// Indentation level
    pub indent: usize,
}

impl<'a> Formatter<'a> {
    pub fn new(dst: &'a mut String) -> Self {
        Self {
            dst,
            spaces: 0,
            scope: vec![],
            indent: DEFAULT_INDENT,
        }
    }

    pub fn scope<F, R>(&mut self, name: &str, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.scope.push(name.to_string());
        let res = f(self);
        self.scope.pop();
        return res;
    }

    pub fn block<F>(&mut self, f: F) -> fmt::Result
    where
        F: FnOnce(&mut Self) -> fmt::Result,
    {
        if !self.is_start_of_line() {
            write!(self, " ")?;
        }

        writeln!(self, "{{")?;
        self.indent(f)?;
        write!(self, "}}")?;

        Ok(())
    }

    pub fn indent<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.spaces += self.indent;
        let res = f(self);
        self.spaces -= self.indent;
        return res;
    }

    pub fn is_start_of_line(&self) -> bool {
        self.dst.is_empty() || self.dst.ends_with('\n')
    }

    fn push_spaces(&mut self) {
        self.dst.push_str(&" ".repeat(self.spaces))
    }
}

impl Write for Formatter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut should_indent = self.is_start_of_line();

        for (idx, line) in s.lines().enumerate() {
            if idx != 0 {
                self.dst.push('\n');
            }

            if should_indent && !line.is_empty() && line.as_bytes()[0] != b'\n' {
                self.push_spaces();
            }

            should_indent = true;

            self.dst.push_str(line);
        }

        if s.as_bytes().last() == Some(&b'\n') {
            self.dst.push('\n');
        }

        Ok(())
    }
}
