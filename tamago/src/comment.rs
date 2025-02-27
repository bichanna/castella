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

/// Represents a C-style comment, supporting both regular and heading-style comments.
///
/// # Examples
/// ```c
/// // Some comment
///
/// ////////////////////////////////////////////////////////////////////////////////
/// // Some heading comment
/// ////////////////////////////////////////////////////////////////////////////////
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Comment {
    /// The comment string
    pub comment: String,

    /// Whether a heading comment or not
    pub is_heading: bool,
}

impl Comment {
    /// Creates and returns a new `CommentBuilder` to construct a `Comment` using the builder
    /// pattern.
    /// ```rust
    /// let comment = Comment::new().comment_with_str("Some comment").build();
    /// ```
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

/// A builder for constructing a `Comment` instance.
pub struct CommentBuilder {
    comment: String,
    is_heading: bool,
}

impl CommentBuilder {
    /// Creates and returns a new `CommentBuilder` to construct a `Comment` using the builder
    /// pattern.
    /// ```rust
    /// let comment = Comment::new().comment_with_str("Some comment").build();
    /// ```
    pub fn new() -> Self {
        Self {
            comment: String::new(),
            is_heading: false,
        }
    }

    /// Creates a new `CommentBuilder` from a string slice, defaulting to a non-heading comment.
    pub fn new_with_str(comment: &str) -> Self {
        Self {
            comment: comment.to_string(),
            is_heading: false,
        }
    }

    /// Sets the comment string for the builder, and returns the builder for more chaining.
    pub fn comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    /// Sets the comment string with a string slice for the builder, and returns the builder for
    /// more chaining.
    pub fn comment_with_str(self, comment: &str) -> Self {
        self.comment(comment.to_string())
    }

    /// Sets whether the comment is a heading comment, and returns the builder for more chaining.
    pub fn heading(mut self, b: bool) -> Self {
        self.is_heading = b;
        self
    }

    /// Consumes the builder and returns a `Comment` containing all the statements added during the
    /// building process.
    pub fn build(self) -> Comment {
        Comment {
            comment: self.comment,
            is_heading: self.is_heading,
        }
    }
}

/// Represents a documentation comment block in C code.
/// ```c
/// /// Some doc comment
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct DocComment {
    /// Lines of doc comment
    pub docs: Vec<String>,
}

impl DocComment {
    /// Creates and returns a new `DocCommentBuilder` to construct a `DocComment` using the builder
    /// pattern.
    /// ```rust
    /// let doc_comment = DocComment::new().line_str("Some doc comment").build();
    /// ```
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

/// A builder for constructing a `DocComment` instance.
pub struct DocCommentBuilder {
    docs: Vec<String>,
}

impl DocCommentBuilder {
    /// Creates and returns a new `DocCommentBuilder` to construct a `DocComment` using the builder
    /// pattern.
    /// ```rust
    /// let doc_comment = DocComment::new().line_str("Some doc comment").build();
    /// ```
    pub fn new() -> Self {
        Self { docs: vec![] }
    }

    /// Appends the provided string to the doc comment and returns the builder for chaining more.
    pub fn line(self, line: String) -> Self {
        self.line_str(&line)
    }

    /// Appends the provided string slice to the doc comment and returns the builder for chaining
    /// more operations.
    pub fn line_str(mut self, line: &str) -> Self {
        self.docs.push(if line.is_empty() {
            String::new()
        } else {
            line.to_string()
        });
        self
    }

    /// Appends the provided multi-line string text to the doc comment and returns the builder for
    /// chaining more.
    pub fn text(self, text: String) -> Self {
        self.text_str(&text)
    }

    /// Appends the provided multi-line string slice text to the doc comment and returns the
    /// builder for chaining more operations.
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

    /// Consumes the builder and returns a `DocComment` containing the documentation comment built
    /// during the building process.
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
