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

use crate::{Format, Formatter, Type};
use tamacro::{DisplayFromConstSymbol, DisplayFromFormat, FormatFromConstSymbol};

#[derive(Debug, Clone, DisplayFromFormat)]
pub enum Expr {
    ConstInt(i64),

    ConstUInt(u64),

    ConstDouble(f64),

    ConstFloat(f32),

    ConstBool(bool),

    ConstChar(char),

    ConstStr(String),

    Ident(String),

    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },

    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    Assign {
        lvalue: Box<Expr>,
        op: AssignOp,
        value: Box<Expr>,
    },

    Ternary {
        cond: Box<Expr>,
        lexpr: Box<Expr>,
        rexpr: Box<Expr>,
    },

    FnCall {
        name: Box<Expr>,
        args: Vec<Expr>,
    },

    MemAccess {
        expr: Box<Expr>,
        member: String,
    },

    ArrIndex {
        arr: Box<Expr>,
        idx: Box<Expr>,
    },

    Cast {
        t: Type,
        expr: Box<Expr>,
    },

    SizeOf(Type),

    InitArr(Vec<(Option<usize>, Box<Expr>)>),

    InitStruct(Vec<(Option<String>, Box<Expr>)>),

    Raw(String),
}

impl Expr {
    pub fn new_ident(name: String) -> Self {
        Self::Ident(name)
    }

    pub fn new_null() -> Self {
        Self::new_ident("NULL".to_string())
    }

    pub fn new_binary(left: Expr, op: BinOp, right: Expr) -> Self {
        Self::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    pub fn new_unary(expr: Expr, op: UnaryOp) -> Self {
        Self::Unary {
            expr: Box::new(expr),
            op,
        }
    }

    pub fn new_assign(lvalue: Expr, op: AssignOp, value: Expr) -> Self {
        Self::Assign {
            lvalue: Box::new(lvalue),
            op,
            value: Box::new(value),
        }
    }

    pub fn new_ternary(cond: Expr, lexpr: Expr, rexpr: Expr) -> Self {
        Self::Ternary {
            cond: Box::new(cond),
            lexpr: Box::new(lexpr),
            rexpr: Box::new(rexpr),
        }
    }

    pub fn new_fn_call(name: Expr, args: Vec<Expr>) -> Self {
        Self::FnCall {
            name: Box::new(name),
            args,
        }
    }

    pub fn new_fn_call_with_name(name: String, args: Vec<Expr>) -> Self {
        Self::FnCall {
            name: Box::new(Self::Ident(name)),
            args,
        }
    }

    pub fn new_mem_access(expr: Expr, member: String) -> Self {
        Self::MemAccess {
            expr: Box::new(expr),
            member,
        }
    }

    pub fn new_arr_index(arr: Expr, idx: Expr) -> Self {
        Self::ArrIndex {
            arr: Box::new(arr),
            idx: Box::new(idx),
        }
    }

    pub fn new_cast(t: Type, expr: Expr) -> Self {
        Self::Cast {
            t,
            expr: Box::new(expr),
        }
    }

    pub fn new_sizeof(t: Type) -> Self {
        Self::SizeOf(t)
    }

    pub fn new_init_arr_in_order(exprs: Vec<Expr>) -> Self {
        Self::InitArr(
            exprs
                .iter()
                .map(|expr| (None, Box::new(expr.clone())))
                .collect(),
        )
    }

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

    pub fn new_init_struct_in_order(exprs: Vec<Expr>) -> Self {
        Self::InitStruct(
            exprs
                .iter()
                .map(|expr| (None, Box::new(expr.clone())))
                .collect(),
        )
    }

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
            ConstInt(num) => write!(fmt, "{num}"),
            ConstUInt(num) => write!(fmt, "{num}"),
            ConstDouble(num) => write!(fmt, "{num}"),
            ConstFloat(num) => write!(fmt, "{num}f"),
            ConstBool(b) => write!(fmt, "{}", if *b { "true" } else { "false" }),
            ConstChar(c) => write!(fmt, "'{c}'"),
            ConstStr(s) => write!(fmt, "\"{s}\""),
            Ident(name) => write!(fmt, "{name}"),
            Binary { left, op, right } => {
                write!(fmt, "(")?;
                left.format(fmt)?;
                op.format(fmt)?;
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
                op.format(fmt)?;
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
                for arg in &args[..args.len() - 1] {
                    arg.format(fmt)?;
                    write!(fmt, ", ")?;
                }
                if let Some(arg) = args.last() {
                    arg.format(fmt)?;
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
                write!(fmt, "(")
            }
            SizeOf(t) => {
                write!(fmt, "sizeof(")?;
                t.format(fmt)?;
                write!(fmt, ")")
            }
            InitArr(v) => {
                write!(fmt, "{{")?;
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
                write!(fmt, "}}")
            }
            InitStruct(v) => {
                write!(fmt, "{{")?;
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
                write!(fmt, "}}")
            }
            Raw(s) => write!(fmt, "{s}"),
        }
    }
}

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
