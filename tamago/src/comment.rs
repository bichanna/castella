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

//! This module is provides ways to add general and documentation comments to the generated code.

use std::fmt::{self, Write};

use crate::formatter::{Format, Formatter};
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Comment {
    /// The comment string
    comment: String,

    /// Whether a heading comment or not
    is_heading: bool,
}

impl Comment {
    pub fn new() -> Self {
        Self {
            comment: String::new(),
            is_heading: false,
        }
    }

    pub fn set_comment(&mut self, comment: String) -> &mut Self {
        self.comment = comment;
        self
    }

    pub fn set_comment_with_str(&mut self, comment: &str) -> &mut Self {
        self.set_comment(comment.to_string())
    }

    pub fn set_heading(&mut self) -> &mut Self {
        self.is_heading = true;
        self
    }

    fn push_heading(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if self.is_heading {
            write!(fmt, "{}", "/".repeat(100 - fmt.spaces))?;
        }

        writeln!(fmt)?;

        Ok(())
    }
}

impl Format for Comment {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.push_heading(fmt)?;
        for line in self.comment.lines() {
            writeln!(fmt, "// {line}")?;
        }
        self.push_heading(fmt)
    }
}

#[derive(Debug, Clone)]
pub struct DocComment {
    docs: Vec<String>,
}

impl DocComment {
    pub fn new() -> Self {
        Self { docs: vec![] }
    }

    pub fn push_line(&mut self, line: &str) -> &mut Self {
        self.docs.push(if line.is_empty() {
            String::new()
        } else {
            line.to_string()
        });
        self
    }

    pub fn push_text(&mut self, text: &str) -> &mut Self {
        let mut res = self;
        for line in text.lines() {
            if line.is_empty() || line == "\n" {
                res = res.push_line("");
                continue;
            }

            let mut start = 0;
            let mut end = 0;
            for (offset, c) in line.chars().enumerate() {
                if c == ' ' && (offset - start) > 80 {
                    res = res.push_line(&line[start..=end]);
                    start = end;
                }
                end = offset;
            }

            if start == end {
                res = res.push_line("");
            } else {
                res = res.push_line(&line[start..=end]);
            }
        }

        res
    }
}

impl Format for DocComment {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for line in &self.docs {
            writeln!(fmt, "/// {line}")?;
        }
        Ok(())
    }
}
