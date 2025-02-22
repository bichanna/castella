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

use crate::{Format, Formatter};
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Comment {
    /// The comment string
    pub comment: String,

    /// Whether a heading comment or not
    pub is_heading: bool,
}

impl Comment {
    pub fn new() -> CommentBuilder {
        CommentBuilder::new()
    }

    fn push_heading(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if self.is_heading {
            writeln!(fmt, "{}", "/".repeat(80 - fmt.spaces))?;
        }

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

pub struct CommentBuilder {
    comment: String,
    is_heading: bool,
}

impl CommentBuilder {
    pub fn new() -> Self {
        Self {
            comment: String::new(),
            is_heading: false,
        }
    }

    pub fn new_with_str(comment: &str) -> Self {
        Self {
            comment: comment.to_string(),
            is_heading: false,
        }
    }

    pub fn comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    pub fn comment_with_str(self, comment: &str) -> Self {
        self.comment(comment.to_string())
    }

    pub fn heading(mut self, b: bool) -> Self {
        self.is_heading = b;
        self
    }

    pub fn build(self) -> Comment {
        Comment {
            comment: self.comment,
            is_heading: self.is_heading,
        }
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct DocComment {
    pub docs: Vec<String>,
}

impl DocComment {
    pub fn new() -> DocCommentBuilder {
        DocCommentBuilder::new()
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

pub struct DocCommentBuilder {
    docs: Vec<String>,
}

impl DocCommentBuilder {
    pub fn new() -> Self {
        Self { docs: vec![] }
    }

    pub fn line(self, line: String) -> Self {
        self.line_str(&line)
    }

    pub fn line_str(mut self, line: &str) -> Self {
        self.docs.push(if line.is_empty() {
            String::new()
        } else {
            line.to_string()
        });
        self
    }

    pub fn text(self, text: String) -> Self {
        self.text_str(&text)
    }

    pub fn text_str(self, text: &str) -> Self {
        let mut res = self;
        for line in text.lines() {
            if line.is_empty() || line == "\n" {
                res = res.line_str("");
                continue;
            }

            let mut start = 0;
            let mut end = 0;
            for (offset, c) in line.chars().enumerate() {
                if c == ' ' && (offset - start) > 80 {
                    res = res.line_str(&line[start..=end]);
                    start = end;
                }
                end = offset;
            }

            if start == end {
                res = res.line_str("");
            } else {
                res = res.line_str(&line[start..=end]);
            }
        }

        res
    }

    pub fn build(self) -> DocComment {
        DocComment { docs: self.docs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comment() {
        let mut c = CommentBuilder::new_with_str("Hello, world").build();
        assert_eq!(c.to_string(), "// Hello, world\n");

        c = CommentBuilder::new_with_str("abc").heading(true).build();
        assert_eq!(c.to_string(), "////////////////////////////////////////////////////////////////////////////////\n// abc\n////////////////////////////////////////////////////////////////////////////////\n");
    }

    #[test]
    fn doc_comment() {
        let mut c = DocComment::new().text_str("Hello\nworld").build();
        assert_eq!(c.to_string(), "/// Hello\n/// world\n");

        c = DocComment::new().line_str("ABC").build();
        assert_eq!(c.to_string(), "/// ABC\n");
    }
}
