pub mod resolver;
pub mod type_checker;

use crate::parser::Span;

/// Could be either a warning or an error
type Message = (Span, String);
