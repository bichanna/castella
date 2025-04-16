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

//! # Expression Module
//!
//! This module provides a set of tools for creating, manipulating, and
//! formatting C expressions. It implements an API for representing various
//! types of C expressions, including:
//!
//! - Literals (integers, floats, booleans, characters, strings)
//! - Identifiers and variables
//! - Binary operations (arithmetic, comparison, bitwise)
//! - Parenthesized expressions
//! - Unary operations (increment, decrement, negation, etc.)
//! - Assignment operations
//! - Ternary conditionals
//! - Function calls
//! - Member access
//! - Array indexing
//! - Type casting
//! - Size queries
//! - Array and struct initializations
//!
//! The module is designed to facilitate C code generation with a type-safe Rust interface.

use std::fmt::{self, Write};

use crate::{Format, Formatter, Type, Variable};
use tamacro::{DisplayFromConstSymbol, DisplayFromFormat, FormatFromConstSymbol};

/// Encapsulates all types of expressions in C.
///
/// This enum represents the complete set of expression types in C, providing a
/// comprehensive way to generate syntactically correct C expressions. Each variant
/// represents a distinct expression type with its associated data.
///
/// # Examples
///
/// ```
/// // Create a binary expression: (a + b)
/// let add_expr = Expr::new_binary(
///     Expr::new_ident_with_str("a"),
///     BinOp::Add,
///     Expr::new_ident_with_str("b")
/// );
///
/// // Create a function call: printf("Hello, %s", name)
/// let printf_call = Expr::new_fn_call_with_name(
///     "printf".to_string(),
///     vec![
///         Expr::Str("Hello, %s".to_string()),
///         Expr::new_ident_with_str("name")
///     ]
/// );
/// ```
#[derive(Debug, Clone, DisplayFromFormat)]
pub enum Expr {
    /// A signed integer literal (e.g., `42`, `-17`).
    Int(i64),

    /// An unsigned integer literal (e.g., `42u`, `0xFF`).
    UInt(u64),

    /// A double precision floating point number literal (e.g., `3.14159`).
    Double(f64),

    /// A single precision floating point number literal (e.g., `3.14f`).
    Float(f32),

    /// A boolean value (`true` or `false`).
    Bool(bool),

    /// A one-byte character literal (e.g., `'a'`, `'\n'`).
    Char(char),

    /// A string literal (e.g., `"hello"`).
    Str(String),

    /// An identifier representing a variable or function name.
    Ident(String),

    /// Variable declaration or definition with type information.
    Variable(Box<Variable>),

    /// A binary expression combining two expressions with an operator.
    ///
    /// Examples: `a + b`, `x * y`, `ptr != NULL`
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },

    /// A parenthesized expression.
    ///
    /// Examples: `(a + b) * c`
    Parenthesized { expr: Box<Expr> },

    /// A unary expression applying an operator to a single expression.
    ///
    /// Examples: `-x`, `!condition`, `++counter`, `&variable`
    Unary { op: UnaryOp, expr: Box<Expr> },

    /// A variable assignment expression.
    ///
    /// Examples: `x = 5`, `ptr += offset`, `flags &= mask`
    Assign {
        lvalue: Box<Expr>,
        op: AssignOp,
        value: Box<Expr>,
    },

    /// A ternary conditional expression.
    ///
    /// Example: `condition ? true_value : false_value`
    Ternary {
        cond: Box<Expr>,
        lexpr: Box<Expr>,
        rexpr: Box<Expr>,
    },

    /// A function call expression.
    ///
    /// Example: `printf("Hello, %s", name)`
    FnCall { name: Box<Expr>, args: Vec<Expr> },

    /// A struct member access expression.
    ///
    /// Example: `person.name`
    MemAccess { expr: Box<Expr>, member: String },

    /// An array indexing expression.
    ///
    /// Example: `array[index]`
    ArrIndex { arr: Box<Expr>, idx: Box<Expr> },

    /// A type casting expression.
    ///
    /// Example: `(int*)pointer`
    Cast { t: Type, expr: Box<Expr> },

    /// A `sizeof` operator expression.
    ///
    /// Example: `sizeof(int)`
    SizeOf(Type),

    /// An array initialization expression.
    ///
    /// Examples:
    /// - In-order: `{1, 2, 3}`
    /// - Designated: `{[0]=1, [5]=2}`
    InitArr(Vec<(Option<usize>, Expr)>),

    /// A struct initialization expression.
    ///
    /// Examples:
    /// - In-order: `{1, "hello", 3.14}`
    /// - Designated: `{.x=1, .name="hello"}`
    InitStruct(Vec<(Option<String>, Expr)>),

    /// A raw C expression as a string (for cases not covered by other variants).
    Raw(String),
}

impl Expr {
    /// Creates a new identifier expression.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the identifier as a String.
    ///
    /// # Returns
    ///
    /// A new `Expr::Ident` with the given name.
    pub fn new_ident(name: String) -> Self {
        Self::Ident(name)
    }

    /// Creates a new identifier expression with a string slice.
    ///
    /// This is a convenience method that converts the string slice to a String.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the identifier as a string slice.
    ///
    /// # Returns
    ///
    /// A new `Expr::Ident` with the given name.
    pub fn new_ident_with_str(name: &str) -> Self {
        Self::new_ident(name.to_string())
    }

    /// Creates a new NULL pointer expression.
    ///
    /// # Returns
    ///
    /// A new `Expr::Ident` representing the NULL constant in C.
    pub fn new_null() -> Self {
        Self::new_ident("NULL".to_string())
    }

    /// Creates a new binary expression with the given expressions and binary operator.
    ///
    /// # Arguments
    ///
    /// * `left` - The expression on the left side of the operator.
    /// * `op` - The binary operator.
    /// * `right` - The expression on the right side of the operator.
    ///
    /// # Returns
    ///
    /// A new `Expr::Binary` combining the left and right expressions with the given operator.
    pub fn new_binary(left: Expr, op: BinOp, right: Expr) -> Self {
        Self::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    /// Creates a new parenthesized expression with the given expression.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression to be parenthesized.
    ///
    /// # Returns
    ///
    /// A new `Expr::Parenthesized` with the given expression.
    pub fn new_parenthesized(expr: Expr) -> Self {
        Self::Parenthesized {
            expr: Box::new(expr),
        }
    }

    /// Creates a new unary expression with the given expression and unary operator.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression to which the operator is applied.
    /// * `op` - The unary operator.
    ///
    /// # Returns
    ///
    /// A new `Expr::Unary` applying the operator to the expression.
    pub fn new_unary(expr: Expr, op: UnaryOp) -> Self {
        Self::Unary {
            expr: Box::new(expr),
            op,
        }
    }

    /// Creates a new assignment expression.
    ///
    /// # Arguments
    ///
    /// * `lvalue` - The expression on the left side (must be a valid lvalue in C).
    /// * `op` - The assignment operator.
    /// * `value` - The expression on the right side.
    ///
    /// # Returns
    ///
    /// A new `Expr::Assign` representing the assignment operation.
    pub fn new_assign(lvalue: Expr, op: AssignOp, value: Expr) -> Self {
        Self::Assign {
            lvalue: Box::new(lvalue),
            op,
            value: Box::new(value),
        }
    }

    /// Creates a new ternary conditional expression.
    ///
    /// # Arguments
    ///
    /// * `cond` - The condition expression.
    /// * `lexpr` - The expression to evaluate if the condition is true.
    /// * `rexpr` - The expression to evaluate if the condition is false.
    ///
    /// # Returns
    ///
    /// A new `Expr::Ternary` representing the conditional expression.
    pub fn new_ternary(cond: Expr, lexpr: Expr, rexpr: Expr) -> Self {
        Self::Ternary {
            cond: Box::new(cond),
            lexpr: Box::new(lexpr),
            rexpr: Box::new(rexpr),
        }
    }

    /// Creates a new function call expression.
    ///
    /// # Arguments
    ///
    /// * `name` - The expression representing the function name or pointer.
    /// * `args` - A vector of expressions representing the function arguments.
    ///
    /// # Returns
    ///
    /// A new `Expr::FnCall` representing the function call.
    pub fn new_fn_call(name: Expr, args: Vec<Expr>) -> Self {
        Self::FnCall {
            name: Box::new(name),
            args,
        }
    }

    /// Creates a new function call expression with a string name.
    ///
    /// This is a convenience method for calling a function by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function as a String.
    /// * `args` - A vector of expressions representing the function arguments.
    ///
    /// # Returns
    ///
    /// A new `Expr::FnCall` representing the function call.
    pub fn new_fn_call_with_name(name: String, args: Vec<Expr>) -> Self {
        Self::FnCall {
            name: Box::new(Self::Ident(name)),
            args,
        }
    }

    /// Creates a new struct member access expression.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression representing the struct instance.
    /// * `member` - The name of the struct member as a String.
    ///
    /// # Returns
    ///
    /// A new `Expr::MemAccess` representing the member access.
    pub fn new_mem_access(expr: Expr, member: String) -> Self {
        Self::MemAccess {
            expr: Box::new(expr),
            member,
        }
    }

    /// Creates a new struct member access expression with a string slice.
    ///
    /// This is a convenience method that converts the string slice to a String.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression representing the struct instance.
    /// * `member` - The name of the struct member as a string slice.
    ///
    /// # Returns
    ///
    /// A new `Expr::MemAccess` representing the member access.
    pub fn new_mem_access_with_str(expr: Expr, member: &str) -> Self {
        Self::new_mem_access(expr, member.to_string())
    }

    /// Creates a new array indexing expression.
    ///
    /// # Arguments
    ///
    /// * `arr` - The expression representing the array.
    /// * `idx` - The expression representing the index.
    ///
    /// # Returns
    ///
    /// A new `Expr::ArrIndex` representing the array indexing operation.
    pub fn new_arr_index(arr: Expr, idx: Expr) -> Self {
        Self::ArrIndex {
            arr: Box::new(arr),
            idx: Box::new(idx),
        }
    }

    /// Creates a new type casting expression.
    ///
    /// # Arguments
    ///
    /// * `t` - The target type for the cast.
    /// * `expr` - The expression to be cast.
    ///
    /// # Returns
    ///
    /// A new `Expr::Cast` representing the type casting operation.
    pub fn new_cast(t: Type, expr: Expr) -> Self {
        Self::Cast {
            t,
            expr: Box::new(expr),
        }
    }

    /// Creates a new `sizeof` operator expression.
    ///
    /// # Arguments
    ///
    /// * `t` - The type whose size to query.
    ///
    /// # Returns
    ///
    /// A new `Expr::SizeOf` representing the sizeof operation.
    pub fn new_sizeof(t: Type) -> Self {
        Self::SizeOf(t)
    }

    /// Creates a new in-order array initialization expression.
    ///
    /// # Arguments
    ///
    /// * `exprs` - A vector of expressions representing the array elements in order.
    ///
    /// # Returns
    ///
    /// A new `Expr::InitArr` representing the array initialization.
    pub fn new_init_arr_in_order(exprs: Vec<Expr>) -> Self {
        Self::InitArr(exprs.into_iter().map(|expr| (None, expr)).collect())
    }

    /// Creates a new designated array initialization expression.
    ///
    /// # Arguments
    ///
    /// * `x` - A vector of indices for the designated initializers.
    /// * `y` - A vector of expressions representing the values for each index.
    ///
    /// # Returns
    ///
    /// A new `Expr::InitArr` representing the designated array initialization.
    ///
    /// # Panics
    ///
    /// Panics if the lengths of `x` and `y` are not equal.
    pub fn new_init_arr_designated(x: Vec<usize>, y: Vec<Expr>) -> Self {
        assert!(x.len() == y.len());
        Self::InitArr(x.into_iter().map(Some).zip(y).collect())
    }

    /// Creates a new in-order struct initialization expression.
    ///
    /// # Arguments
    ///
    /// * `exprs` - A vector of expressions representing the struct members in order.
    ///
    /// # Returns
    ///
    /// A new `Expr::InitStruct` representing the struct initialization.
    pub fn new_init_struct_in_order(exprs: Vec<Expr>) -> Self {
        Self::InitStruct(exprs.into_iter().map(|expr| (None, expr)).collect())
    }

    /// Creates a new designated struct initialization expression.
    ///
    /// # Arguments
    ///
    /// * `x` - A vector of member names for the designated initializers.
    /// * `y` - A vector of expressions representing the values for each member.
    ///
    /// # Returns
    ///
    /// A new `Expr::InitStruct` representing the designated struct initialization.
    ///
    /// # Panics
    ///
    /// Panics if the lengths of `x` and `y` are not equal.
    pub fn new_init_struct_designated(x: Vec<String>, y: Vec<Expr>) -> Self {
        assert!(x.len() == y.len());
        Self::InitStruct(x.into_iter().map(Some).zip(y).collect())
    }
}

impl Format for Expr {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use Expr::*;
        match self {
            Int(num) => write!(fmt, "{num}"),
            UInt(num) => write!(fmt, "{num}"),
            Double(num) => write!(fmt, "{num}"),
            Float(num) => write!(fmt, "{num}f"),
            Bool(b) => write!(fmt, "{}", if *b { "true" } else { "false" }),
            Char(c) => write!(fmt, "'{c}'"),
            Str(s) => write!(fmt, "\"{s}\""),
            Ident(name) => write!(fmt, "{name}"),
            Variable(var) => var.format(fmt),
            Binary { left, op, right } => {
                left.format(fmt)?;
                write!(fmt, " ")?;
                op.format(fmt)?;
                write!(fmt, " ")?;
                right.format(fmt)
            }
            Parenthesized { expr } => {
                write!(fmt, "(")?;
                expr.format(fmt)?;
                write!(fmt, ")")
            }
            Unary { op, expr } => {
                if !matches!(op, UnaryOp::Inc | UnaryOp::Dec) {
                    op.format(fmt)?;
                }
                expr.format(fmt)?;
                if matches!(op, UnaryOp::Inc | UnaryOp::Dec) {
                    op.format(fmt)?;
                }
                Ok(())
            }
            Assign { lvalue, op, value } => {
                lvalue.format(fmt)?;
                write!(fmt, " ")?;
                op.format(fmt)?;
                write!(fmt, " ")?;
                value.format(fmt)
            }
            Ternary { cond, lexpr, rexpr } => {
                cond.format(fmt)?;
                write!(fmt, " ? ")?;
                lexpr.format(fmt)?;
                write!(fmt, " : ")?;
                rexpr.format(fmt)
            }
            FnCall { name, args } => {
                name.format(fmt)?;
                write!(fmt, "(")?;
                if !args.is_empty() {
                    for arg in &args[..args.len() - 1] {
                        arg.format(fmt)?;
                        write!(fmt, ", ")?;
                    }
                    if let Some(arg) = args.last() {
                        arg.format(fmt)?;
                    }
                }
                write!(fmt, ")")
            }
            MemAccess { expr, member } => {
                expr.format(fmt)?;
                write!(fmt, ".{member}")
            }
            ArrIndex { arr, idx } => {
                arr.format(fmt)?;
                write!(fmt, "[")?;
                idx.format(fmt)?;
                write!(fmt, "]")
            }
            Cast { t, expr } => {
                write!(fmt, "(")?;
                t.format(fmt)?;
                write!(fmt, ")")?;
                write!(fmt, "(")?;
                expr.format(fmt)?;
                write!(fmt, ")")
            }
            SizeOf(t) => {
                write!(fmt, "sizeof(")?;
                t.format(fmt)?;
                if t.is_array() {
                    write!(fmt, "[{}]", t.array)?;
                }
                write!(fmt, ")")
            }
            InitArr(v) => {
                write!(fmt, "{{")?;
                if !v.is_empty() {
                    for x in &v[..v.len() - 1] {
                        if let Some(idx) = x.0 {
                            write!(fmt, "[{idx}]=")?;
                        }
                        x.1.format(fmt)?;
                        write!(fmt, ", ")?;
                    }
                    if let Some(last) = v.last() {
                        if let Some(idx) = last.0 {
                            write!(fmt, "[{idx}]=")?;
                        }
                        last.1.format(fmt)?;
                    }
                }
                write!(fmt, "}}")
            }
            InitStruct(v) => {
                write!(fmt, "{{")?;
                if !v.is_empty() {
                    for x in &v[..v.len() - 1] {
                        if let Some(name) = &x.0 {
                            write!(fmt, ".{name}=")?;
                        }
                        x.1.format(fmt)?;
                        write!(fmt, ", ")?;
                    }
                    if let Some(last) = v.last() {
                        if let Some(name) = &last.0 {
                            write!(fmt, ".{name}=")?;
                        }
                        last.1.format(fmt)?;
                    }
                }
                write!(fmt, "}}")
            }
            Raw(s) => write!(fmt, "{s}"),
        }
    }
}

/// Encapsulates binary operators used in C expressions.
///
/// This enum represents all binary operators in C, including arithmetic,
/// comparison, logical, and bitwise operators.
#[derive(Debug, Clone, DisplayFromConstSymbol, FormatFromConstSymbol)]
pub enum BinOp {
    /// Addition operator (`+`)
    #[symbol = "+"]
    Add,

    /// Subtraction operator (`-`)
    #[symbol = "-"]
    Sub,

    /// Multiplication operator (`*`)
    #[symbol = "*"]
    Mul,

    /// Division operator (`/`)
    #[symbol = "/"]
    Div,

    /// Modulo operator (`%`)
    #[symbol = "%"]
    Mod,

    /// Equality comparison operator (`==`)
    #[symbol = "=="]
    Eq,

    /// Inequality comparison operator (`!=`)
    #[symbol = "!="]
    NEq,

    /// Greater than comparison operator (`>`)
    #[symbol = ">"]
    GT,

    /// Less than comparison operator (`<`)
    #[symbol = "<"]
    LT,

    /// Greater than or equal comparison operator (`>=`)
    #[symbol = ">="]
    GTE,

    /// Less than or equal comparison operator (`<=`)
    #[symbol = "<="]
    LTE,

    /// Logical AND operator (`&&`)
    #[symbol = "&&"]
    And,

    /// Logical OR operator (`||`)
    #[symbol = "||"]
    Or,

    /// Bitwise AND operator (`&`)
    #[symbol = "&"]
    BitAnd,

    /// Bitwise OR operator (`|`)
    #[symbol = "|"]
    BitOr,

    /// Bitwise XOR operator (`^`)
    #[symbol = "^"]
    XOr,

    /// Left shift operator (`<<`)
    #[symbol = "<<"]
    LShift,

    /// Right shift operator (`>>`)
    #[symbol = ">>"]
    RShift,
}

/// Encapsulates unary operators used in C expressions.
///
/// This enum represents all unary operators in C, including prefix
/// and postfix operators for various operations.
#[derive(Debug, Clone, DisplayFromConstSymbol, FormatFromConstSymbol)]
pub enum UnaryOp {
    /// Increment operator (`++`), can be prefix or postfix
    #[symbol = "++"]
    Inc,

    /// Decrement operator (`--`), can be prefix or postfix
    #[symbol = "--"]
    Dec,

    /// Unary negation operator (`-`)
    #[symbol = "-"]
    Neg,

    /// Logical negation operator (`!`)
    #[symbol = "!"]
    LogicNeg,

    /// Bitwise NOT operator (`~`)
    #[symbol = "~"]
    BitNot,

    /// Address-of operator (`&`)
    #[symbol = "&"]
    AddrOf,

    /// Dereference operator (`*`)
    #[symbol = "*"]
    Deref,
}

/// Encapsulates assignment operators used in C expressions.
///
/// This enum represents all assignment operators in C, including
/// simple assignment and compound assignments combining assignment
/// with other operations.
#[derive(Debug, Clone, DisplayFromConstSymbol, FormatFromConstSymbol)]
pub enum AssignOp {
    /// Simple assignment operator (`=`)
    #[symbol = "="]
    Assign,

    /// Addition assignment operator (`+=`)
    #[symbol = "+="]
    AddAssign,

    /// Subtraction assignment operator (`-=`)
    #[symbol = "-="]
    SubAssign,

    /// Multiplication assignment operator (`*=`)
    #[symbol = "*="]
    MulAssign,

    /// Division assignment operator (`/=`)
    #[symbol = "/="]
    DivAssign,

    /// Modulo assignment operator (`%=`)
    #[symbol = "%="]
    ModAssign,

    /// Bitwise AND assignment operator (`&=`)
    #[symbol = "&="]
    BitAndAssign,

    /// Bitwise OR assignment operator (`|=`)
    #[symbol = "|="]
    BitOrAssign,

    /// Bitwise XOR assignment operator (`^=`)
    #[symbol = "^="]
    BitXOrAssign,

    /// Left shift assignment operator (`<<=`)
    #[symbol = "<<="]
    LShiftAssign,

    /// Right shift assignment operator (`>>=`)
    #[symbol = ">>="]
    RShiftAssign,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn binary() {
        let b = Expr::new_binary(
            Expr::new_binary(Expr::Int(123), BinOp::LT, Expr::Int(321)),
            BinOp::BitOr,
            Expr::new_binary(Expr::Double(1.23), BinOp::Sub, Expr::Float(3.21)),
        );
        let res = "123 < 321 | 1.23 - 3.21f";
        assert_eq!(b.to_string(), res);
    }

    #[test]
    fn parenthesized() {
        let p = Expr::new_binary(
            Expr::Int(3),
            BinOp::Mul,
            Expr::new_parenthesized(Expr::new_binary(Expr::Int(123), BinOp::Add, Expr::Int(321))),
        );
        let res = "3 * (123 + 321)";
        assert_eq!(p.to_string(), res);
    }

    #[test]
    fn unary() {
        let u = Expr::new_unary(
            Expr::new_unary(
                Expr::new_sizeof(Type::new(BaseType::Struct("some_struct".to_string())).build()),
                UnaryOp::Inc,
            ),
            UnaryOp::Neg,
        );
        let res = "-sizeof(struct some_struct)++";
        assert_eq!(u.to_string(), res);
    }

    #[test]
    fn assign() {
        let a = Expr::new_assign(
            Expr::Ident("abc".to_string()),
            AssignOp::SubAssign,
            Expr::Int(123),
        );
        let res = "abc -= 123";
        assert_eq!(a.to_string(), res);

        let b = Expr::new_assign(
            Expr::Ident("abc".to_string()),
            AssignOp::BitAndAssign,
            Expr::Bool(false),
        );
        let res = "abc &= false";
        assert_eq!(b.to_string(), res);
    }

    #[test]
    fn ternary() {
        let t = Expr::new_ternary(
            Expr::Bool(true),
            Expr::Str("hello".to_string()),
            Expr::Str("olleh".to_string()),
        );
        let res = r#"true ? "hello" : "olleh""#;
        assert_eq!(t.to_string(), res);
    }

    #[test]
    fn fncall() {
        let f = Expr::new_fn_call(Expr::Ident("some_func".to_string()), vec![]);
        let res = "some_func()";
        assert_eq!(f.to_string(), res);

        let f2 = Expr::new_fn_call(
            Expr::Ident("some_func".to_string()),
            vec![
                Expr::Char('a'),
                Expr::new_sizeof(Type::new(BaseType::Char).build()),
            ],
        );
        let res2 = "some_func('a', sizeof(char))";
        assert_eq!(f2.to_string(), res2);
    }

    #[test]
    fn mem_access() {
        let m = Expr::new_mem_access(Expr::Ident("person".to_string()), "age".to_string());
        let res = "person.age";
        assert_eq!(m.to_string(), res);
    }

    #[test]
    fn arr_index() {
        let a = Expr::new_arr_index(Expr::Ident("some_arr".to_string()), Expr::Int(5));
        let res = "some_arr[5]";
        assert_eq!(a.to_string(), res);
    }

    #[test]
    fn cast() {
        let c = Expr::new_cast(
            Type::new(BaseType::Void).make_pointer().build(),
            Expr::Ident("something".to_string()),
        );
        let res = "(void*)(something)";
        assert_eq!(c.to_string(), res);
    }

    #[test]
    fn sizeof() {
        let s = Expr::new_sizeof(Type::new(BaseType::Struct("some_struct".to_string())).build());
        let res = "sizeof(struct some_struct)";
        assert_eq!(s.to_string(), res);
    }

    #[test]
    fn init_arr() {
        let i = Expr::new_init_arr_in_order(vec![Expr::Int(1), Expr::Int(3), Expr::Int(2)]);
        let res = "{1, 3, 2}";
        assert_eq!(i.to_string(), res);

        let i2 = Expr::new_init_arr_designated(
            vec![0, 1, 2],
            vec![Expr::Float(1.1), Expr::Float(2.1), Expr::Float(4.4)],
        );
        let res2 = "{[0]=1.1f, [1]=2.1f, [2]=4.4f}";
        assert_eq!(i2.to_string(), res2);
    }

    #[test]
    fn init_struct() {
        let i = Expr::new_init_struct_in_order(vec![
            Expr::Str("abc".to_string()),
            Expr::Int(15),
            Expr::Char('x'),
        ]);
        let res = "{\"abc\", 15, 'x'}";
        assert_eq!(i.to_string(), res);

        let i2 = Expr::new_init_struct_designated(
            vec!["name".to_string(), "age".to_string()],
            vec![Expr::Str("bichanna".to_string()), Expr::Int(18)],
        );
        let res2 = "{.name=\"bichanna\", .age=18}";
        assert_eq!(i2.to_string(), res2);
    }
}
