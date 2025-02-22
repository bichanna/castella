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

pub use block::{Block, BlockBuilder, Statement};
pub use comment::{Comment, CommentBuilder, DocComment, DocCommentBuilder};
pub use conditional::{If, IfBuilder, Switch, SwitchBuilder};
pub use enums::{Enum, EnumBuilder, Variant, VariantBuilder};
pub use expr::{AssignOp, BinOp, Expr, UnaryOp};
pub use formatter::{Format, Formatter};
pub use function::{Function, FunctionBuilder, Parameter, ParameterBuilder};
pub use loops::{DoWhile, DoWhileBuilder, For, ForBuilder, While, WhileBuilder};
pub use preprocessor::{
    ErrorDirective, ErrorDirectiveBuilder, FuncMacro, FuncMacroBuilder, IfDefDirective,
    IfDefDirectiveBuilder, IfDirective, IfDirectiveBuilder, Include, IncludeBuilder, LineDirective,
    Macro, ObjMacro, ObjMacroBuilder, PragmaDirective, PragmaDirectiveBuilder, ScopeOrBlock,
    WarningDirective, WarningDirectiveBuilder,
};
pub use scope::{GlobalStatement, Scope, ScopeBuilder};
pub use structs::{Field, FieldBuilder, Struct, StructBuilder};
pub use typedef::{TypeDef, TypeDefBuilder};
pub use types::{BaseType, Type, TypeBuilder, TypeQualifier};
pub use union::{Union, UnionBuilder};
pub use variable::{Variable, VariableBuilder};
