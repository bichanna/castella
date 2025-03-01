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

//! This module provides functionality for generating and formatting different types of comments
//! in C code. It supports standard C-style comments, heading comments for section organization,
//! and documentation comments typically used before functions, structs, and other declarations.
//!
//! The components in this module are:
//! - `Comment`: Represents regular C-style comments (single-line or multi-line)
//! - `CommentBuilder`: Facilitates constructing comments using the builder pattern
//! - `DocComment`: Represents documentation comments in C (using `///` style)
//! - `DocCommentBuilder`: Facilitates constructing documentation comments
//!
//! Use this module to add properly formatted comments to generated C code for improved
//! readability, documentation, and code organization.

use std::fmt::{self, Write};

use crate::{Format, Formatter};
use tamacro::DisplayFromFormat;

/// Represents a C-style comment in generated code. This struct supports both standard
/// single-line comments and special heading-style comments that are surrounded by
/// separator lines for visual emphasis.
///
/// ## Comment Types
///
/// This struct can represent two types of comments:
/// 1. **Standard Comments**: Normal comments prefixed with `//`
/// 2. **Heading Comments**: Comments surrounded by horizontal lines of slashes to create
///    visual separation between code sections
///
/// ## Usage
///
/// Comments are used for:
/// - Explaining code behavior and intent
/// - Marking sections of code
/// - Providing context for implementation details
/// - Temporarily disabling code (commenting out)
///
/// ## Examples
///
/// ### Standard Comment
/// ```c
/// // This is a standard comment
/// // It can span multiple lines
/// ```
///
/// ### Heading Comment
/// ```c
/// ////////////////////////////////////////////////////////////////////////////////
/// // This is a heading comment used to mark a major section of code
/// ////////////////////////////////////////////////////////////////////////////////
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Comment {
    /// The text content of the comment, which may contain multiple lines.
    /// Each line will be prefixed with `//` during formatting.
    pub comment: String,

    /// Controls whether the comment is formatted as a heading comment.
    /// When true, the comment will be surrounded by lines of slashes.
    pub is_heading: bool,
}

impl Comment {
    /// Creates and returns a new `CommentBuilder` to construct a `Comment` using the builder pattern.
    ///
    /// This method is the recommended entry point for creating comments as it provides
    /// a fluent API for setting comment text and style.
    ///
    /// ## Example
    ///
    /// ```rust
    /// // Create a standard comment
    /// let standard_comment = Comment::new()
    ///     .comment_with_str("This explains the following code")
    ///     .build();
    ///
    /// // Create a heading comment
    /// let heading_comment = Comment::new()
    ///     .comment_with_str("INITIALIZATION SECTION")
    ///     .heading(true)
    ///     .build();
    /// ```
    pub fn new() -> CommentBuilder {
        CommentBuilder::new()
    }

    /// Internal helper method to add horizontal separator lines for heading comments.
    ///
    /// This method generates a line of slashes that spans most of the available width,
    /// taking into account the current indentation level.
    ///
    /// ## Parameters
    ///
    /// - `fmt`: The formatter to write to
    ///
    /// ## Returns
    ///
    /// A `fmt::Result` indicating success or failure of the write operation
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

/// A builder for constructing `Comment` instances in a fluent, chainable manner.
///
/// This builder provides methods for setting the comment text and controlling
/// whether it's formatted as a heading comment. It follows the builder pattern
/// to allow method chaining for a more readable and flexible API.
pub struct CommentBuilder {
    comment: String,
    is_heading: bool,
}

impl CommentBuilder {
    /// Creates and returns a new `CommentBuilder` instance with default settings.
    ///
    /// The new builder starts with an empty comment and is not set as a heading comment.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let builder = CommentBuilder::new();
    /// let comment = builder
    ///     .comment_with_str("Some explanation here")
    ///     .build();
    /// ```
    pub fn new() -> Self {
        Self {
            comment: String::new(),
            is_heading: false,
        }
    }

    /// Creates a new `CommentBuilder` initialized with the provided comment text.
    ///
    /// This is a convenience method that creates a builder with pre-filled comment text,
    /// but still defaults to a standard (non-heading) comment style.
    ///
    /// ## Parameters
    ///
    /// - `comment`: The initial text content for the comment
    ///
    /// ## Returns
    ///
    /// A new `CommentBuilder` with the provided text
    ///
    /// ## Example
    ///
    /// ```rust
    /// let comment = CommentBuilder::new_with_str("Important note about this code")
    ///     .build();
    /// ```
    pub fn new_with_str(comment: &str) -> Self {
        Self {
            comment: comment.to_string(),
            is_heading: false,
        }
    }

    /// Sets the comment text for the builder using a String.
    ///
    /// This method overwrites any previously set comment text.
    ///
    /// ## Parameters
    ///
    /// - `comment`: The text content for the comment as a String
    ///
    /// ## Returns
    ///
    /// The builder instance for method chaining
    ///
    /// ## Example
    ///
    /// ```rust
    /// let dynamic_text = format!("Count: {}", some_value);
    /// let comment = CommentBuilder::new()
    ///     .comment(dynamic_text)
    ///     .build();
    /// ```
    pub fn comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    /// Sets the comment text for the builder using a string slice.
    ///
    /// This is a convenience method that converts the string slice to a String
    /// and then sets it as the comment text.
    ///
    /// ## Parameters
    ///
    /// - `comment`: The text content for the comment as a string slice
    ///
    /// ## Returns
    ///
    /// The builder instance for method chaining
    ///
    /// ## Example
    ///
    /// ```rust
    /// let comment = CommentBuilder::new()
    ///     .comment_with_str("This function calculates the average")
    ///     .build();
    /// ```
    pub fn comment_with_str(self, comment: &str) -> Self {
        self.comment(comment.to_string())
    }

    /// Controls whether the comment should be formatted as a heading comment.
    ///
    /// Heading comments are surrounded by lines of slashes for visual emphasis,
    /// making them useful for marking major sections of code.
    ///
    /// ## Parameters
    ///
    /// - `b`: Set to true for a heading comment, false for a standard comment
    ///
    /// ## Returns
    ///
    /// The builder instance for method chaining
    ///
    /// ## Example
    ///
    /// ```rust
    /// let section_header = CommentBuilder::new()
    ///     .comment_with_str("HELPER FUNCTIONS")
    ///     .heading(true)
    ///     .build();
    /// ```
    pub fn heading(mut self, b: bool) -> Self {
        self.is_heading = b;
        self
    }

    /// Consumes the builder and returns a new `Comment` instance.
    ///
    /// This method finalizes the building process and returns the constructed comment
    /// with the text and style settings configured during the build process.
    ///
    /// ## Returns
    ///
    /// A new `Comment` instance with the configured settings
    ///
    /// ## Example
    ///
    /// ```rust
    /// let comment = CommentBuilder::new()
    ///     .comment_with_str("Important implementation detail")
    ///     .build();
    /// ```
    pub fn build(self) -> Comment {
        Comment {
            comment: self.comment,
            is_heading: self.is_heading,
        }
    }
}

/// Represents a documentation comment in C code, using the `///` style syntax.
///
/// Documentation comments are special comments typically placed before function declarations,
/// struct definitions, typedefs, and other code elements to document their purpose, parameters,
/// return values, and other important information.
///
/// ## Usage
///
/// Doc comments are typically used to document:
/// - Function signatures (parameters, return values, exceptions)
/// - Data structures and their fields
/// - Global variables and constants
/// - Type definitions
/// - File/module purpose
///
/// ## Example
///
/// ```c
/// /// Calculates the square of a number
/// ///
/// /// @param value The input number to square
/// /// @return The square of the input value
/// int square(int value);
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub struct DocComment {
    /// Lines of documentation comment text.
    /// Each line will be prefixed with `///` during formatting.
    pub docs: Vec<String>,
}

impl DocComment {
    /// Creates and returns a new `DocCommentBuilder` to construct a `DocComment` using the builder pattern.
    ///
    /// This method is the recommended entry point for creating documentation comments as it provides
    /// a fluent API for building multi-line documentation.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let doc_comment = DocComment::new()
    ///     .line_str("Calculates the factorial of a number")
    ///     .line_str("")
    ///     .line_str("@param n The input number")
    ///     .line_str("@return The factorial of n")
    ///     .build();
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

/// A builder for constructing `DocComment` instances in a fluent, chainable manner.
///
/// This builder provides methods for building documentation comments line by line
/// or from larger text blocks. It follows the builder pattern to allow method chaining
/// for a more readable and flexible API.
pub struct DocCommentBuilder {
    docs: Vec<String>,
}

impl DocCommentBuilder {
    /// Creates and returns a new `DocCommentBuilder` instance with an empty set of documentation lines.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let builder = DocCommentBuilder::new();
    /// let doc = builder
    ///     .line_str("Function description here")
    ///     .line_str("@param name Description of parameter")
    ///     .build();
    /// ```
    pub fn new() -> Self {
        Self { docs: vec![] }
    }

    /// Appends a single line of documentation text to the comment using a String.
    ///
    /// ## Parameters
    ///
    /// - `line`: The line of text to add as a String
    ///
    /// ## Returns
    ///
    /// The builder instance for method chaining
    ///
    /// ## Example
    ///
    /// ```rust
    /// let param_desc = format!("@param count Maximum number (default: {})", DEFAULT_COUNT);
    /// let doc = DocCommentBuilder::new()
    ///     .line(param_desc)
    ///     .build();
    /// ```
    pub fn line(self, line: String) -> Self {
        self.line_str(&line)
    }

    /// Appends a single line of documentation text to the comment using a string slice.
    ///
    /// This method handles empty lines by preserving them in the documentation comment.
    /// Empty lines are useful for separating sections within documentation.
    ///
    /// ## Parameters
    ///
    /// - `line`: The line of text to add as a string slice
    ///
    /// ## Returns
    ///
    /// The builder instance for method chaining
    ///
    /// ## Example
    ///
    /// ```rust
    /// let doc = DocCommentBuilder::new()
    ///     .line_str("Parses a configuration file")
    ///     .line_str("")  // Empty line for visual separation
    ///     .line_str("@param path Path to the configuration file")
    ///     .build();
    /// ```
    pub fn line_str(mut self, line: &str) -> Self {
        self.docs.push(if line.is_empty() {
            String::new()
        } else {
            line.to_string()
        });
        self
    }

    /// Adds a multi-line block of text to the documentation comment using a String.
    ///
    /// This method splits the input text by newlines and adds each line separately.
    /// It also handles line wrapping for long lines to maintain readability.
    ///
    /// ## Parameters
    ///
    /// - `text`: The multi-line text block as a String
    ///
    /// ## Returns
    ///
    /// The builder instance for method chaining
    ///
    /// ## Example
    ///
    /// ```rust
    /// let description = format!(
    ///     "Processes data according to the specified algorithm.\n\n\
    ///     The processing has a complexity of O(n) where n is {}.",
    ///     complexity_description
    /// );
    /// let doc = DocCommentBuilder::new()
    ///     .text(description)
    ///     .build();
    /// ```
    pub fn text(self, text: String) -> Self {
        self.text_str(&text)
    }

    /// Adds a multi-line block of text to the documentation comment using a string slice.
    ///
    /// This method processes the text block and adds it to the documentation comment with
    /// intelligent line handling:
    /// 1. Splits the input by newlines
    /// 2. Preserves empty lines
    /// 3. Performs line wrapping for lines longer than 80 characters
    ///
    /// ## Parameters
    ///
    /// - `text`: The multi-line text block as a string slice
    ///
    /// ## Returns
    ///
    /// The builder instance for method chaining
    ///
    /// ## Example
    ///
    /// ```rust
    /// let doc = DocCommentBuilder::new()
    ///     .text_str(
    ///         "This function implements the algorithm described in:\n\
    ///         Smith, J. (2023). Efficient Algorithms for Data Processing.\n\n\
    ///         Note: The implementation uses recursion and may cause stack overflow\
    ///         with extremely large inputs."
    ///     )
    ///     .build();
    /// ```
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

    /// Consumes the builder and returns a new `DocComment` instance.
    ///
    /// This method finalizes the building process and returns the constructed documentation
    /// comment containing all the lines added during the building process.
    ///
    /// ## Returns
    ///
    /// A new `DocComment` instance with the configured documentation lines
    ///
    /// ## Example
    ///
    /// ```rust
    /// let complete_doc = DocCommentBuilder::new()
    ///     .line_str("Encrypts data using the specified algorithm")
    ///     .line_str("")
    ///     .line_str("@param data The data to encrypt")
    ///     .line_str("@param key The encryption key")
    ///     .line_str("@return The encrypted data or NULL on failure")
    ///     .build();
    /// ```
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
