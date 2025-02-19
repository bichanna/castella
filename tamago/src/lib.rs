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

//! Library for generating C code

mod block;
mod comment;
mod conditional;
mod enums;
mod expr;
mod formatter;
mod function;
mod loops;
mod preprocessor;
mod scope;
mod structs;
mod typedef;
mod types;
mod union;
mod variable;

pub use block::{Block, Statement};
pub use comment::{Comment, DocComment};
pub use conditional::{If, Switch};
pub use enums::{Enum, Variant};
pub use expr::{AssignOp, BinOp, Expr, UnaryOp};
pub use formatter::{Format, Formatter};
pub use function::{Function, Parameter};
pub use loops::{DoWhile, For, While};
pub use preprocessor::{
    ErrorDirective, FuncMacro, IfDefDirective, IfDirective, Include, LineDirective, Macro,
    ObjMacro, PragmaDirective, ScopeOrBlock, WarningDirective,
};
pub use scope::{GlobalStatement, Scope};
pub use structs::{Field, Struct};
pub use typedef::TypeDef;
pub use types::{BaseType, Type, TypeQualifier};
pub use union::Union;
pub use variable::Variable;
