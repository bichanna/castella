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

//! This module provides ways to create expressions.

use std::fmt::{self, Write};

use crate::{Format, Formatter, Type, Variable};
use tamacro::{DisplayFromConstSymbol, DisplayFromFormat, FormatFromConstSymbol};

/// Encapsulates all types of expressions in C, like binary, unary, literal, ternary, variables,
/// function calls, and more.
#[derive(Debug, Clone, DisplayFromFormat)]
pub enum Expr {
    /// A signed integer literal.
    Int(i64),

    /// A unsigned integer literal.
    UInt(u64),

    /// A double precision floating pointer number literal.
    Double(f64),

    /// A single precision floating pointer number literal.
    Float(f32),

    /// A boolean.
    Bool(bool),

    /// A one-byte character.
    Char(char),

    /// A string literal.
    Str(String),

    /// An identifier.
    Ident(String),

    /// Variable declration (could be definition as well)
    Variable(Box<Variable>),

    /// A binary expression, like `1 + 1`.
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },

    /// A unary expression, like `-1`.
    Unary { op: UnaryOp, expr: Box<Expr> },

    /// A variable assignment.
    Assign {
        lvalue: Box<Expr>,
        op: AssignOp,
        value: Box<Expr>,
    },

    /// A ternary operator, like `cond ? true_expr : false_expr`
    Ternary {
        cond: Box<Expr>,
        lexpr: Box<Expr>,
        rexpr: Box<Expr>,
    },

    /// A function call.
    FnCall { name: Box<Expr>, args: Vec<Expr> },

    /// A struct member access.
    MemAccess { expr: Box<Expr>, member: String },

    /// Indexing an array
    ArrIndex { arr: Box<Expr>, idx: Box<Expr> },

    /// Type casting
    Cast { t: Type, expr: Box<Expr> },

    /// `sizeof` operator
    SizeOf(Type),

    /// Array initialization. Could be either ordered or designated.
    InitArr(Vec<(Option<usize>, Box<Expr>)>),

    /// Struct instance initialization. Could be either ordered or designated.
    InitStruct(Vec<(Option<String>, Box<Expr>)>),

    /// A raw piece of expression in C.
    Raw(String),
}

impl Expr {
    /// Creates a new identifier.
    pub fn new_ident(name: String) -> Self {
        Self::Ident(name)
    }

    /// Creates a new identifier with the given string slice
    pub fn new_ident_with_str(name: &str) -> Self {
        Self::new_ident(name.to_string())
    }

    /// Creates a new `NULL`
    pub fn new_null() -> Self {
        Self::new_ident("NULL".to_string())
    }

    /// Creates a new binary expression with the given expressions and binary operator
    pub fn new_binary(left: Expr, op: BinOp, right: Expr) -> Self {
        Self::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    /// Creates a new unary expression with the given expression and unary operator
    pub fn new_unary(expr: Expr, op: UnaryOp) -> Self {
        Self::Unary {
            expr: Box::new(expr),
            op,
        }
    }

    /// Creates a new assignment with the given lvalue, the expression, and the assignment
    /// operator
    pub fn new_assign(lvalue: Expr, op: AssignOp, value: Expr) -> Self {
        Self::Assign {
            lvalue: Box::new(lvalue),
            op,
            value: Box::new(value),
        }
    }

    /// Creates a new ternary expression with the given condition and expressions.
    pub fn new_ternary(cond: Expr, lexpr: Expr, rexpr: Expr) -> Self {
        Self::Ternary {
            cond: Box::new(cond),
            lexpr: Box::new(lexpr),
            rexpr: Box::new(rexpr),
        }
    }

    /// Creates a new function call expression with the given name and the arguments.
    pub fn new_fn_call(name: Expr, args: Vec<Expr>) -> Self {
        Self::FnCall {
            name: Box::new(name),
            args,
        }
    }

    /// Creates a new function call expression with the given name as a string slice and the
    /// arguments
    pub fn new_fn_call_with_name(name: String, args: Vec<Expr>) -> Self {
        Self::FnCall {
            name: Box::new(Self::Ident(name)),
            args,
        }
    }

    /// Creates a struct member access expression with the given member string
    pub fn new_mem_access(expr: Expr, member: String) -> Self {
        Self::MemAccess {
            expr: Box::new(expr),
            member,
        }
    }

    /// Creates a struct member access expression with the given member string slice
    pub fn new_mem_access_with_str(expr: Expr, member: &str) -> Self {
        Self::new_mem_access(expr, member.to_string())
    }

    /// Creates a new array indexing expression with the given array expression and the index
    /// expression
    pub fn new_arr_index(arr: Expr, idx: Expr) -> Self {
        Self::ArrIndex {
            arr: Box::new(arr),
            idx: Box::new(idx),
        }
    }

    /// Creates a new type cast expression with the given expression
    pub fn new_cast(t: Type, expr: Expr) -> Self {
        Self::Cast {
            t,
            expr: Box::new(expr),
        }
    }

    /// Creates a new `sizeof` operator for the given expression
    pub fn new_sizeof(t: Type) -> Self {
        Self::SizeOf(t)
    }

    /// Creates a new in-order array initialization expression
    pub fn new_init_arr_in_order(exprs: Vec<Expr>) -> Self {
        Self::InitArr(
            exprs
                .iter()
                .map(|expr| (None, Box::new(expr.clone())))
                .collect(),
        )
    }

    /// Creates a new designated array initialization expression
    pub fn new_init_arr_designated(x: Vec<usize>, y: Vec<Expr>) -> Self {
        assert!(x.len() == y.len());
        Self::InitArr(
            x.iter()
                .map(|x| Some(x.clone()))
                .into_iter()
                .zip(y.iter().map(|y| Box::new(y.clone())))
                .collect(),
        )
    }

    /// Creates a new in-order struct initialization expression
    pub fn new_init_struct_in_order(exprs: Vec<Expr>) -> Self {
        Self::InitStruct(
            exprs
                .iter()
                .map(|expr| (None, Box::new(expr.clone())))
                .collect(),
        )
    }

    /// Creates a new designated struct initializastion expression
    pub fn new_init_struct_designated(x: Vec<String>, y: Vec<Expr>) -> Self {
        Self::InitStruct(
            x.iter()
                .map(|x| Some(x.clone()))
                .into_iter()
                .zip(y.iter().map(|y| Box::new(y.clone())))
                .collect(),
        )
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
                write!(fmt, "(")?;
                left.format(fmt)?;
                write!(fmt, " ")?;
                op.format(fmt)?;
                write!(fmt, " ")?;
                right.format(fmt)?;
                write!(fmt, ")")
            }
            Unary { op, expr } => {
                write!(fmt, "(")?;
                if !matches!(op, UnaryOp::Inc | UnaryOp::Dec) {
                    op.format(fmt)?;
                }
                expr.format(fmt)?;
                if matches!(op, UnaryOp::Inc | UnaryOp::Dec) {
                    op.format(fmt)?;
                }
                write!(fmt, ")")
            }
            Assign { lvalue, op, value } => {
                write!(fmt, "(")?;
                lvalue.format(fmt)?;
                write!(fmt, " ")?;
                op.format(fmt)?;
                write!(fmt, " ")?;
                value.format(fmt)?;
                write!(fmt, ")")
            }
            Ternary { cond, lexpr, rexpr } => {
                write!(fmt, "(")?;
                cond.format(fmt)?;
                write!(fmt, " ? ")?;
                lexpr.format(fmt)?;
                write!(fmt, " : ")?;
                rexpr.format(fmt)?;
                write!(fmt, ")")
            }
            FnCall { name, args } => {
                name.format(fmt)?;
                write!(fmt, "(")?;
                if args.len() > 0 {
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
                write!(fmt, ")")
            }
            InitArr(v) => {
                write!(fmt, "{{")?;
                if v.len() > 0 {
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
                if v.len() > 0 {
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

/// Encapsulates binary operator
#[derive(Debug, Clone, DisplayFromConstSymbol, FormatFromConstSymbol)]
pub enum BinOp {
    #[symbol = "+"]
    Add,

    #[symbol = "-"]
    Sub,

    #[symbol = "*"]
    Mul,

    #[symbol = "/"]
    Div,

    #[symbol = "%"]
    Mod,

    #[symbol = "=="]
    Eq,

    #[symbol = "!="]
    NEq,

    #[symbol = ">"]
    GT,

    #[symbol = "<"]
    LT,

    #[symbol = ">="]
    GTE,

    #[symbol = "<="]
    LTE,

    #[symbol = "&&"]
    And,

    #[symbol = "||"]
    Or,

    #[symbol = "&"]
    BitAnd,

    #[symbol = "|"]
    BitOr,

    #[symbol = "^"]
    XOr,

    #[symbol = "<<"]
    LShift,

    #[symbol = ">>"]
    RShift,
}

/// Encapsulates unary operators
#[derive(Debug, Clone, DisplayFromConstSymbol, FormatFromConstSymbol)]
pub enum UnaryOp {
    #[symbol = "++"]
    Inc,

    #[symbol = "--"]
    Dec,

    #[symbol = "-"]
    Neg,

    #[symbol = "!"]
    LogicNeg,

    #[symbol = "~"]
    BitNot,

    #[symbol = "&"]
    AddrOf,

    #[symbol = "*"]
    Deref,
}

/// Encapsulates assign operators
#[derive(Debug, Clone, DisplayFromConstSymbol, FormatFromConstSymbol)]
pub enum AssignOp {
    #[symbol = "="]
    Assign,

    #[symbol = "+="]
    AddAssign,

    #[symbol = "-="]
    SubAssign,

    #[symbol = "*="]
    MulAssign,

    #[symbol = "/="]
    DivAssign,

    #[symbol = "%="]
    ModAssign,

    #[symbol = "&="]
    BitAndAssign,

    #[symbol = "|="]
    BitOrAssign,

    #[symbol = "^="]
    BitXOrAssign,

    #[symbol = "<<="]
    LShiftAssign,

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
        let res = "((123 < 321) | (1.23 - 3.21f))";
        assert_eq!(b.to_string(), res);
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
        let res = "(-(sizeof(struct some_struct)++))";
        assert_eq!(u.to_string(), res);
    }

    #[test]
    fn assign() {
        let a = Expr::new_assign(
            Expr::Ident("abc".to_string()),
            AssignOp::SubAssign,
            Expr::Int(123),
        );
        let res = "(abc -= 123)";
        assert_eq!(a.to_string(), res);

        let b = Expr::new_assign(
            Expr::Ident("abc".to_string()),
            AssignOp::BitAndAssign,
            Expr::Bool(false),
        );
        let res = "(abc &= false)";
        assert_eq!(b.to_string(), res);
    }

    #[test]
    fn ternary() {
        let t = Expr::new_ternary(
            Expr::Bool(true),
            Expr::Str("hello".to_string()),
            Expr::Str("olleh".to_string()),
        );
        let res = r#"(true ? "hello" : "olleh")"#;
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
