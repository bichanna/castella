use logos::Lexer;
use tamago::{AssignOp, BinOp, UnaryOp};

use crate::lexer::*;

macro_rules! expect {
    ($self: expr, $current: expr, $expected: pat, $line_num: expr, $msg: expr, $($f: expr),*) => {{
        if !matches!($current, $expected) {
            return Err($self.create_error_with_line_num(format!($msg, $($f),*), $line_num));
        }

        $current
    }};

    ($self: expr, $current: expr, $expected: pat, $line_num: expr, $msg: expr) => {{
        if !matches!($current, $expected) {
            return Err($self.create_error_with_line_num($msg, $line_num));
        }

        $current
    }};
}

pub type ParseError = (usize, String);
pub type ParseErrors = Vec<ParseError>;

pub struct Parser<'source> {
    lexer: Lexer<'source, Token>,
    source_path: &'source str,
    source: &'source str,
    ast: Vec<GlobalStatement>,
    current_token: Option<Result<Token, LexError>>,
    errors: ParseErrors,
}

impl<'source> Parser<'source> {
    pub fn new(
        source: &'source str,
        source_path: &'source str,
        mut lexer: Lexer<'source, Token>,
    ) -> Self {
        let current_token = lexer.next();
        Self {
            lexer,
            source,
            source_path,
            ast: vec![],
            current_token,
            errors: vec![],
        }
    }

    pub fn parse(mut self) -> Result<Vec<GlobalStatement>, ParseErrors> {
        while !self.is_end() {
            let stmt = self.parse_global_statement();
            match stmt {
                Ok(stmt) => self.ast.push(stmt),
                Err(err) => {
                    self.errors.push(err);
                    self.synchronize();
                }
            }
        }

        if self.errors.is_empty() {
            Ok(self.ast)
        } else {
            Err(self.errors)
        }
    }

    fn parse_global_statement(&mut self) -> Result<GlobalStatement, ParseError> {
        match self.current()? {
            Token::Enum => self.parse_enum(),
            Token::Struct => self.parse_struct(),
            Token::Union => self.parse_union(),
            Token::Func => self.parse_func(),
            Token::Let => self.parse_let(),
            Token::Const => self.parse_const(),
            Token::Alias => self.parse_alias(),
            Token::Import => self.parse_import(),
            _ => Err(self.create_error(format!(
                "Expected a global statement but got {}",
                self.current()?
            ))),
        }
    }

    fn parse_enum(&mut self) -> Result<GlobalStatement, ParseError> {
        todo!()
    }

    fn parse_struct(&mut self) -> Result<GlobalStatement, ParseError> {
        todo!()
    }

    fn parse_union(&mut self) -> Result<GlobalStatement, ParseError> {
        todo!()
    }

    fn parse_func(&mut self) -> Result<GlobalStatement, ParseError> {
        self.next();

        let Token::Ident(func_name) = expect!(
            self,
            self.current()?,
            Token::Ident(..),
            self.get_line_number(),
            "Expected an identifier for the function name but got {}",
            self.current()?
        ) else {
            unreachable!()
        };

        self.next();

        let params = self.parse_func_params()?;
        let ret = self.parse_ret_type()?;
        let body = self.parse_curly_body()?;

        Ok(GlobalStatement::Function {
            name: func_name,
            params,
            ret,
            body,
        })
    }

    fn parse_let(&mut self) -> Result<GlobalStatement, ParseError> {
        todo!()
    }

    fn parse_const(&mut self) -> Result<GlobalStatement, ParseError> {
        todo!()
    }

    fn parse_alias(&mut self) -> Result<GlobalStatement, ParseError> {
        todo!()
    }

    fn parse_import(&mut self) -> Result<GlobalStatement, ParseError> {
        todo!()
    }

    fn parse_func_params(&mut self) -> Result<Vec<(String, Type)>, ParseError> {
        expect!(
            self,
            self.current()?,
            Token::LeftParen,
            self.get_line_number(),
            "Expected {} after function name but got {}",
            Token::LeftParen,
            self.current()?
        );

        self.next();

        let mut params: Vec<(String, Type)> = vec![];

        while !matches!(self.current()?, Token::RightParen) {
            let Token::Ident(param_name) = expect!(
                self,
                self.current()?,
                Token::Ident(..),
                self.get_line_number(),
                "Expected an identifier for a parameter name but got {}",
                self.current()?
            ) else {
                unreachable!()
            };

            self.next();

            expect!(
                self,
                self.current()?,
                Token::Colon,
                self.get_line_number(),
                "Expected {} after parameter name but got {}",
                Token::Colon,
                self.current()?
            );

            self.next();

            let param_type = self.parse_type()?;

            params.push((param_name, param_type));

            if !matches!(self.current()?, Token::Comma) {
                break;
            } else {
                self.next();
            }
        }

        self.next();

        Ok(params)
    }

    fn parse_ret_type(&mut self) -> Result<Type, ParseError> {
        expect!(
            self,
            self.current()?,
            Token::Colon,
            self.get_line_number(),
            "Expected {} for specifying return type but got {}",
            Token::Colon,
            self.current()?
        );

        self.next();

        self.parse_type()
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        match {
            let token = self.current()?;
            self.next();
            token
        } {
            Token::TVoid => Ok(Type::Void),
            Token::TDouble => Ok(Type::Double),
            Token::TFloat => Ok(Type::Float),
            Token::TChar => Ok(Type::Char),
            Token::TStr => Ok(Type::Str),
            Token::TInt8 => Ok(Type::Int8),
            Token::TInt16 => Ok(Type::Int16),
            Token::TInt32 => Ok(Type::Int32),
            Token::TInt64 => Ok(Type::Int64),
            Token::TUInt8 => Ok(Type::UInt8),
            Token::TUInt16 => Ok(Type::UInt16),
            Token::TUInt32 => Ok(Type::UInt32),
            Token::TUInt64 => Ok(Type::UInt64),
            Token::TBool => Ok(Type::Bool),
            Token::Caret => Ok(Type::Pointer(Box::new(self.parse_type()?))),
            Token::Ident(user_def_type) => Ok(Type::UserDefinedType(user_def_type)),
            Token::LeftBrak => {
                let mut num: usize = 0;
                let is_darray = match self.current()? {
                    Token::Caret => {
                        self.next();
                        true
                    }
                    Token::Int(n) => {
                        self.next();
                        num = n as usize;
                        false
                    }
                    _ => {
                        return Err(
                            self.create_error(format!("Expected an integer or {}", Token::Caret))
                        )
                    }
                };

                expect!(
                    self,
                    self.current()?,
                    Token::RightBrak,
                    self.get_line_number(),
                    "Expected {} for the {} type but got {}",
                    Token::RightBrak,
                    if is_darray { "dynamic array" } else { "array" },
                    self.current()?
                );

                self.next();

                let elem_type = self.parse_type()?;

                if is_darray {
                    Ok(Type::DArray(Box::new(elem_type)))
                } else {
                    Ok(Type::Array(num, Box::new(elem_type)))
                }
            }
            token => Err(self.create_error(format!("Expected type expression but got {}", token))),
        }
    }

    fn parse_curly_body(&mut self) -> Result<Vec<Statement>, ParseError> {
        expect!(
            self,
            self.current()?,
            Token::LeftBrace,
            self.get_line_number(),
            "Expected {} for a block but got {}",
            Token::LeftBrace,
            self.current()?
        );

        self.next();

        let mut body: Vec<Statement> = vec![];

        while !matches!(self.current()?, Token::RightBrace) {
            body.push(self.parse_statement()?);
        }

        self.next();

        Ok(body)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current()? {
            Token::Let => todo!(),
            Token::Const => todo!(),
            Token::Return => todo!(),
            Token::Break => todo!(),
            Token::Continue => todo!(),
            Token::If => todo!(),
            Token::While => todo!(),
            Token::Defer => todo!(),
            Token::Destroy => todo!(),
            Token::Free => todo!(),
            _ => {
                let expr = self.parse_expression()?;

                expect!(
                    self,
                    self.current()?,
                    Token::SemiColon,
                    self.get_line_number(),
                    "Expected {} at the end of a statement but got {}",
                    Token::SemiColon,
                    self.current()?
                );

                self.next();

                Ok(Statement::Expression { expr })
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_or_expr()?;

        match self.current()? {
            Token::Eq => self.parse_assign(expr, AssignOp::Assign),
            Token::PlusEq => self.parse_assign(expr, AssignOp::AddAssign),
            Token::MinusEq => self.parse_assign(expr, AssignOp::SubAssign),
            Token::MulEq => self.parse_assign(expr, AssignOp::MulAssign),
            Token::DivEq => self.parse_assign(expr, AssignOp::DivAssign),
            Token::ModEq => self.parse_assign(expr, AssignOp::ModAssign),
            // TODO: Add bitwise things as well
            _ => Ok(expr),
        }
    }

    fn parse_assign(&mut self, lexpr: Expr, op: AssignOp) -> Result<Expr, ParseError> {
        self.next();

        let rexpr = self.parse_expression()?;

        Ok(Expr::Assign {
            lvalue: Box::new(lexpr),
            op,
            value: Box::new(rexpr),
        })
    }

    fn parse_or_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_and_expr()?;

        while matches!(self.current()?, Token::Or) {
            self.next();

            let rexpr = self.parse_and_expr()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinOp::Or,
                right: Box::new(rexpr),
            };
        }

        Ok(expr)
    }

    fn parse_and_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_equality()?;

        while matches!(self.current()?, Token::And) {
            self.next();

            let rexpr = self.parse_equality()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinOp::And,
                right: Box::new(rexpr),
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;

        while matches!(self.current()?, Token::NotEq | Token::DEq) {
            let bin_op = if matches!(self.current()?, Token::NotEq) {
                BinOp::NEq
            } else {
                BinOp::Eq
            };

            self.next();

            let rexpr = self.parse_comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op: bin_op,
                right: Box::new(rexpr),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term_expr()?;

        while matches!(
            self.current()?,
            Token::GT | Token::LT | Token::GE | Token::LE
        ) {
            let bin_op = match self.current()? {
                Token::GT => BinOp::GT,
                Token::LT => BinOp::LT,
                Token::GE => BinOp::GTE,
                Token::LE => BinOp::LTE,
                _ => unreachable!(),
            };

            self.next();

            let rexpr = self.parse_term_expr()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op: bin_op,
                right: Box::new(rexpr),
            };
        }

        Ok(expr)
    }

    fn parse_term_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor_expr()?;

        while matches!(self.current()?, Token::Minus | Token::Plus) {
            let bin_op = if matches!(self.current()?, Token::Minus) {
                BinOp::Sub
            } else {
                BinOp::Add
            };

            self.next();

            let rexpr = self.parse_factor_expr()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op: bin_op,
                right: Box::new(rexpr),
            };
        }

        Ok(expr)
    }

    fn parse_factor_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;

        while matches!(self.current()?, Token::Div | Token::Mod | Token::Mul) {
            let bin_op = match self.current()? {
                Token::Div => BinOp::Div,
                Token::Mul => BinOp::Mul,
                Token::Mod => BinOp::Mod,
                _ => unreachable!(),
            };

            self.next();

            let rexpr = self.parse_unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op: bin_op,
                right: Box::new(rexpr),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if matches!(self.current()?, Token::Not | Token::Minus) {
            let unary_op = if matches!(self.current()?, Token::Not) {
                UnaryOp::LogicNeg
            } else {
                UnaryOp::Neg
            };

            self.next();

            let val = self.parse_unary()?;

            Ok(Expr::Unary {
                op: unary_op,
                expr: Box::new(val),
            })
        } else {
            self.parse_call()
        }
    }

    fn parse_call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary_expr()?;

        loop {
            match self.current()? {
                Token::LeftParen => {
                    expr = self.parse_fn_call(expr)?;
                }

                Token::LeftBrak => {
                    expr = self.parse_indexing(expr)?;
                }

                Token::LeftBrace => {
                    expr = self.parse_struct_init(expr)?;
                }

                Token::RightArrow => {
                    expr = self.parse_enum_variant(expr)?;
                }

                Token::Dot => {
                    expr = self.parse_struct_member_field(expr)?;
                }

                Token::DColon => {
                    expr = self.parse_module_access(expr)?;
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_fn_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        self.next();

        let mut args: Vec<Expr> = vec![];

        while !matches!(self.current()?, Token::RightParen) {
            let arg = self.parse_expression()?;
            args.push(arg);

            if !matches!(self.current()?, Token::Comma) {
                break;
            } else {
                self.next();
            }
        }

        self.next();

        Ok(Expr::FnCall {
            name: Box::new(callee),
            args,
        })
    }

    fn parse_indexing(&mut self, expr: Expr) -> Result<Expr, ParseError> {
        self.next();

        let index = self.parse_expression()?;

        expect!(
            self,
            self.current()?,
            Token::RightBrak,
            self.get_line_number(),
            "Expected a pairing {} for indexing but got {}",
            Token::RightBrak,
            self.current()?
        );

        self.next();

        Ok(Expr::ArrIndex {
            arr: Box::new(expr),
            idx: Box::new(index),
        })
    }

    fn parse_struct_init(&mut self, expr: Expr) -> Result<Expr, ParseError> {
        let Expr::Ident(ident) = expr else {
            unreachable!()
        };

        self.next();

        let mut args: Vec<(String, Expr)> = vec![];

        while !matches!(self.current()?, Token::RightBrace) {
            args.push(self.parse_struct_init_arg()?);

            if !matches!(self.current()?, Token::Comma) {
                break;
            } else {
                self.next();
            }
        }

        Ok(Expr::InitStruct { ident, args })
    }

    fn parse_enum_variant(&mut self, expr: Expr) -> Result<Expr, ParseError> {
        let Expr::Ident(ident) = expr else {
            unreachable!()
        };

        self.next();

        self.next();

        let Token::Ident(variant) = self.current()? else {
            unreachable!()
        };

        self.next();

        Ok(Expr::EnumVarAccess { ident, variant })
    }

    fn parse_struct_member_field(&mut self, expr: Expr) -> Result<Expr, ParseError> {
        self.next();

        let Token::Ident(member) = self.current()? else {
            unreachable!()
        };

        self.next();

        Ok(Expr::MemAccess {
            expr: Box::new(expr),
            member,
        })
    }

    fn parse_module_access(&mut self, expr: Expr) -> Result<Expr, ParseError> {
        todo!()
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, ParseError> {
        match self.current()? {
            Token::True => {
                self.next();
                Ok(Expr::Bool(true))
            }

            Token::False => {
                self.next();
                Ok(Expr::Bool(false))
            }

            Token::Int(val) => {
                self.next();
                Ok(Expr::Int(val))
            }

            Token::Double(val) => {
                self.next();
                Ok(Expr::Double(val))
            }

            Token::Str(val) => {
                self.next();
                Ok(Expr::Str(val))
            }

            Token::Ident(ident) => {
                self.next();
                Ok(Expr::Ident(ident))
            }

            Token::LeftParen => self.parse_parenthesized(),

            Token::Make => self.parse_make_expr(),

            Token::New => self.parse_new_expr(),

            t => Err(self.create_error(format!("Unexpected token: {}", t))),
        }
    }

    fn parse_parenthesized(&mut self) -> Result<Expr, ParseError> {
        self.next();

        let expr = self.parse_expression()?;

        expect!(
            self,
            self.current()?,
            Token::RightParen,
            self.get_line_number(),
            "Expected {} for parenthesized expression but got {}",
            Token::RightParen,
            self.current()?
        );

        self.next();

        Ok(Expr::Parenthesized {
            expr: Box::new(expr),
        })
    }

    fn parse_make_expr(&mut self) -> Result<Expr, ParseError> {
        self.next();

        let t = self.parse_type()?;

        Ok(Expr::Make { t })
    }

    fn parse_new_expr(&mut self) -> Result<Expr, ParseError> {
        self.next();

        let t = self.parse_type()?;

        Ok(Expr::New { t })
    }

    fn parse_struct_init_arg(&mut self) -> Result<(String, Expr), ParseError> {
        let Token::Ident(ident) = self.current()? else {
            unreachable!()
        };

        self.next();

        expect!(
            self,
            self.current()?,
            Token::Eq,
            self.get_line_number(),
            "Expected {} got but {}",
            Token::Eq,
            self.current()?
        );

        self.next();

        let expr = self.parse_expression()?;

        Ok((ident, expr))
    }

    #[inline]
    fn current(&self) -> Result<Token, ParseError> {
        if let Some(res) = &self.current_token {
            if let Ok(res) = res {
                Ok(res.clone())
            } else {
                let e = res.clone().unwrap_err();
                Err((self.get_line_number(), e.msg))
            }
        } else {
            Err((self.get_line_number(), "Unexpected end of file".to_string()))
        }
    }

    #[inline]
    fn next(&mut self) {
        self.current_token = self.lexer.next();
    }

    fn create_error_with_line_num(&self, msg: String, line_num: usize) -> ParseError {
        (line_num, msg)
    }

    fn create_error(&self, msg: String) -> ParseError {
        (self.get_line_number(), msg)
    }

    fn synchronize(&mut self) {
        while !self.is_end() {
            match {
                self.next();
                self.current()
            } {
                Ok(t) => {
                    if matches!(
                        t,
                        Token::Enum
                            | Token::Struct
                            | Token::Union
                            | Token::Func
                            | Token::Alias
                            | Token::Import
                    ) {
                        return;
                    } else {
                        continue;
                    }
                }
                Err(e) => {
                    self.errors.push(e);
                }
            }
        }
    }

    fn is_end(&self) -> bool {
        self.current_token.is_none()
    }

    fn get_line_number(&self) -> usize {
        let span = self.lexer.span();
        let before = &self.source[0..span.start];
        before.chars().filter(|&c| c == '\n').count() + 1
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Double(f64),
    Bool(bool),
    Char(u8),
    Str(String),
    Ident(String),
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Parenthesized {
        expr: Box<Expr>,
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
    EnumVarAccess {
        ident: String,
        variant: String,
    },
    ArrIndex {
        arr: Box<Expr>,
        idx: Box<Expr>,
    },
    Cast {
        t: Type,
        expr: Box<Expr>,
    },
    Sizeof {
        t: Type,
    },
    InitArr {
        elems: Vec<Expr>,
    },
    InitArrDesignated {
        idxs: Vec<usize>,
        elems: Vec<Expr>,
    },
    InitStruct {
        ident: String,
        args: Vec<(String, Expr)>,
    },
    Make {
        t: Type,
    },
    New {
        t: Type,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Variable {
        name: String,
        t: Option<Type>,
        value: Option<Expr>,
        is_static: bool,
        is_const: bool,
    },
    Expression {
        expr: Expr,
    },
    Return {
        value: Option<Expr>,
    },
    Break,
    Continue,
    If {
        cond: Expr,
        then: Vec<Statement>,
        other: Option<Vec<Statement>>,
    },
    While {
        cond: Expr,
        body: Vec<Statement>,
        do_while: bool,
    },
    Defer {
        body: Vec<Statement>,
    },
    Destroy {
        expr: Expr,
    },
    Free {
        expr: Expr,
    },
}

#[derive(Debug, Clone)]
pub enum GlobalStatement {
    Enum {
        name: String,
        variants: Vec<(String, Option<i64>)>,
    },
    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },
    Union {
        name: String,
        fields: Vec<(String, Type)>,
    },
    Function {
        name: String,
        params: Vec<(String, Type)>,
        ret: Type,
        body: Vec<Statement>,
    },
    Variable {
        name: String,
        t: Option<Type>,
        value: Option<Expr>,
        is_static: bool,
    },
    Constant {
        name: String,
        t: Option<Type>,
        value: Expr,
    },
    Alias {
        t: Type,
        name: String,
    },
    Import {
        name: Option<String>,
        path: String,
    },
}

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Double,
    Float,
    Char,
    Str,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Bool,
    Pointer(Box<Type>),
    Array(usize, Box<Type>),
    DArray(Box<Type>),
    UserDefinedType(String),
}
