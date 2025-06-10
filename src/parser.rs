use logos::Lexer;
use std::ops::Range;
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

pub type ParseError = (Span, String);
pub type ParseErrors = Vec<ParseError>;

pub struct Parser<'source> {
    lexer: Lexer<'source, Token>,
    current_token: Option<Result<Token, LexError>>,
    ast: Vec<LocatedGlobalStmt>,
    errors: ParseErrors,
}

impl<'source> Parser<'source> {
    pub fn new(mut lexer: Lexer<'source, Token>) -> Self {
        let current_token = lexer.next();
        Self {
            lexer,
            ast: vec![],
            current_token,
            errors: vec![],
        }
    }

    pub fn parse(mut self) -> Result<Vec<LocatedGlobalStmt>, ParseErrors> {
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

    fn parse_global_statement(&mut self) -> Result<LocatedGlobalStmt, ParseError> {
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

    fn parse_enum(&mut self) -> Result<LocatedGlobalStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        let Token::Ident(enum_name) = expect!(
            self,
            self.current()?,
            Token::Ident(..),
            self.lexer.span(),
            "Expected name for enum but got {}",
            self.current()?
        ) else {
            unreachable!();
        };

        self.next();

        expect!(
            self,
            self.current()?,
            Token::LeftBrace,
            self.lexer.span(),
            "Expected {} after enum but got {}",
            Token::LeftBrace,
            self.current()?
        );

        self.next();

        let mut variants = vec![];

        while !matches!(self.current()?, Token::RightBrace) {
            let Token::Ident(var_name) = expect!(
                self,
                self.current()?,
                Token::Ident(..),
                self.lexer.span(),
                "Expected an identifier but got {}",
                self.current()?
            ) else {
                unreachable!();
            };

            self.next();

            let var_num = if matches!(self.current()?, Token::Eq) {
                self.next();
                let Token::Int(num) = expect!(
                    self,
                    self.current()?,
                    Token::Int(..),
                    self.lexer.span(),
                    "Expected an integer but got {}",
                    self.current()?
                ) else {
                    unreachable!();
                };
                self.next();

                Some(num)
            } else {
                None
            };

            expect!(
                self,
                self.current()?,
                Token::SemiColon,
                self.lexer.span(),
                "Expected {} after enum variant but got {}",
                Token::SemiColon,
                self.current()?
            );

            self.next();

            variants.push((var_name, var_num));
        }

        self.next();

        Ok(Located {
            node: GlobalStmt::Enum {
                variants,
                name: enum_name,
            },
            span,
        })
    }

    fn parse_struct(&mut self) -> Result<LocatedGlobalStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        let Token::Ident(struct_name) = expect!(
            self,
            self.current()?,
            Token::Ident(..),
            self.lexer.span(),
            "Expected name for struct but got {}",
            self.current()?
        ) else {
            unreachable!();
        };

        self.next();

        expect!(
            self,
            self.current()?,
            Token::LeftBrace,
            self.lexer.span(),
            "Expected {} after struct but got {}",
            Token::LeftBrace,
            self.current()?
        );

        self.next();

        let mut fields = vec![];

        while !matches!(self.current()?, Token::RightBrace) {
            let Token::Ident(field_name) = expect!(
                self,
                self.current()?,
                Token::Ident(..),
                self.lexer.span(),
                "Expected field name but got {}",
                self.current()?
            ) else {
                unreachable!();
            };

            self.next();

            expect!(
                self,
                self.current()?,
                Token::Colon,
                self.lexer.span(),
                "Expected {} after field name but got {}",
                Token::Colon,
                self.current()?
            );

            self.next();

            let field_type = self.parse_type()?;

            expect!(
                self,
                self.current()?,
                Token::SemiColon,
                self.lexer.span(),
                "Expected {} after struct field but got {}",
                Token::SemiColon,
                self.current()?
            );

            self.next();

            fields.push((field_name, field_type));
        }

        self.next();

        Ok(Located {
            node: GlobalStmt::Struct {
                fields,
                name: struct_name,
            },
            span,
        })
    }

    fn parse_union(&mut self) -> Result<LocatedGlobalStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        let Token::Ident(union_name) = expect!(
            self,
            self.current()?,
            Token::Ident(..),
            self.lexer.span(),
            "Expected name for union but got {}",
            self.current()?
        ) else {
            unreachable!();
        };

        self.next();

        expect!(
            self,
            self.current()?,
            Token::LeftBrace,
            self.lexer.span(),
            "Expected {} after union but got {}",
            Token::LeftBrace,
            self.current()?
        );

        self.next();

        let mut fields = vec![];

        while !matches!(self.current()?, Token::RightBrace) {
            let Token::Ident(field_name) = expect!(
                self,
                self.current()?,
                Token::Ident(..),
                self.lexer.span(),
                "Expected field name but got {}",
                self.current()?
            ) else {
                unreachable!();
            };

            self.next();

            expect!(
                self,
                self.current()?,
                Token::Colon,
                self.lexer.span(),
                "Expected {} after field name but got {}",
                Token::Colon,
                self.current()?
            );

            self.next();

            let field_type = self.parse_type()?;

            expect!(
                self,
                self.current()?,
                Token::SemiColon,
                self.lexer.span(),
                "Expected {} after union field but got {}",
                Token::SemiColon,
                self.current()?
            );

            self.next();

            fields.push((field_name, field_type));
        }

        self.next();

        Ok(Located {
            node: GlobalStmt::Union {
                fields,
                name: union_name,
            },
            span,
        })
    }

    fn parse_func(&mut self) -> Result<LocatedGlobalStmt, ParseError> {
        self.next();

        let Token::Ident(func_name) = expect!(
            self,
            self.current()?,
            Token::Ident(..),
            self.lexer.span(),
            "Expected an identifier for the function name but got {}",
            self.current()?
        ) else {
            unreachable!()
        };
        let span = self.lexer.span();

        self.next();

        let params = self.parse_func_params()?;
        let ret = self.parse_ret_type()?;
        let body = self.parse_curly_body()?;

        Ok(Located {
            node: GlobalStmt::Function {
                name: func_name,
                params,
                ret,
                body,
            },
            span,
        })
    }

    fn parse_let(&mut self) -> Result<LocatedGlobalStmt, ParseError> {
        todo!()
    }

    fn parse_const(&mut self) -> Result<LocatedGlobalStmt, ParseError> {
        todo!()
    }

    fn parse_alias(&mut self) -> Result<LocatedGlobalStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        let Token::Ident(name) = expect!(
            self,
            self.current()?,
            Token::Ident(..),
            self.lexer.span(),
            "Expected an identifier after alias but got {}",
            self.current()?
        ) else {
            unreachable!();
        };

        self.next();

        expect!(
            self,
            self.current()?,
            Token::Eq,
            self.lexer.span(),
            "Expected {} but got {} after alias name",
            Token::Eq,
            self.current()?
        );

        self.next();

        let t = self.parse_type()?;

        expect!(
            self,
            self.current()?,
            Token::SemiColon,
            self.lexer.span(),
            "Expected {} but got {}",
            Token::SemiColon,
            self.current()?
        );

        self.next();

        Ok(Located {
            node: GlobalStmt::Alias { name, t },
            span,
        })
    }

    fn parse_import(&mut self) -> Result<LocatedGlobalStmt, ParseError> {
        todo!()
    }

    fn parse_func_params(&mut self) -> Result<Vec<(String, LocatedType)>, ParseError> {
        expect!(
            self,
            self.current()?,
            Token::LeftParen,
            self.lexer.span(),
            "Expected {} after function name but got {}",
            Token::LeftParen,
            self.current()?
        );

        self.next();

        let mut params: Vec<(String, LocatedType)> = vec![];

        while !matches!(self.current()?, Token::RightParen) {
            let Token::Ident(param_name) = expect!(
                self,
                self.current()?,
                Token::Ident(..),
                self.lexer.span(),
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
                self.lexer.span(),
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

    fn parse_ret_type(&mut self) -> Result<LocatedType, ParseError> {
        expect!(
            self,
            self.current()?,
            Token::Colon,
            self.lexer.span(),
            "Expected {} for specifying return type but got {}",
            Token::Colon,
            self.current()?
        );

        self.next();

        self.parse_type()
    }

    fn parse_type(&mut self) -> Result<LocatedType, ParseError> {
        let span = self.lexer.span();

        match {
            let token = self.current()?;
            self.next();
            token
        } {
            Token::TVoid => Ok(Located {
                node: Type::Void,
                span,
            }),
            Token::TDouble => Ok(Located {
                node: Type::Double,
                span,
            }),
            Token::TFloat => Ok(Located {
                node: Type::Float,
                span,
            }),
            Token::TChar => Ok(Located {
                node: Type::Char,
                span,
            }),
            Token::TStr => Ok(Located {
                node: Type::Str,
                span,
            }),
            Token::TInt8 => Ok(Located {
                node: Type::Int8,
                span,
            }),
            Token::TInt16 => Ok(Located {
                node: Type::Int16,
                span,
            }),
            Token::TInt32 => Ok(Located {
                node: Type::Int32,
                span,
            }),
            Token::TInt64 => Ok(Located {
                node: Type::Int64,
                span,
            }),
            Token::TUInt8 => Ok(Located {
                node: Type::UInt8,
                span,
            }),
            Token::TUInt16 => Ok(Located {
                node: Type::UInt16,
                span,
            }),
            Token::TUInt32 => Ok(Located {
                node: Type::UInt32,
                span,
            }),
            Token::TUInt64 => Ok(Located {
                node: Type::UInt64,
                span,
            }),
            Token::TBool => Ok(Located {
                node: Type::Bool,
                span,
            }),
            Token::Caret => Ok(Located {
                node: Type::Pointer(Box::new(self.parse_type()?.node)),
                span,
            }),
            Token::Ident(user_def_type) => Ok(Located {
                node: Type::UserDefinedType(user_def_type),
                span,
            }),
            Token::LeftBrak => {
                let mut num: usize = 0;
                let is_darray = match self.current()? {
                    Token::Caret => {
                        self.next();
                        true
                    }
                    Token::Int(n) => {
                        self.next();
                        if n < 0 {
                            self.create_error(format!("Expected array size to be non-negative"));
                        }
                        num = n as usize;
                        false
                    }
                    _ => {
                        return Err(self.create_error(format!(
                            "Expected an integer or {} for array type",
                            Token::Caret
                        )))
                    }
                };

                expect!(
                    self,
                    self.current()?,
                    Token::RightBrak,
                    self.lexer.span(),
                    "Expected {} for the {} type but got {}",
                    Token::RightBrak,
                    if is_darray { "dynamic array" } else { "array" },
                    self.current()?
                );

                self.next();

                let elem_type = self.parse_type()?;

                if is_darray {
                    Ok(Located {
                        node: Type::DArray(Box::new(elem_type.node)),
                        span,
                    })
                } else {
                    Ok(Located {
                        node: Type::Array(num, Box::new(elem_type.node)),
                        span,
                    })
                }
            }
            token => Err(self.create_error(format!("Expected type expression but got {}", token))),
        }
    }

    fn parse_curly_body(&mut self) -> Result<Vec<LocatedStmt>, ParseError> {
        expect!(
            self,
            self.current()?,
            Token::LeftBrace,
            self.lexer.span(),
            "Expected {} for a block but got {}",
            Token::LeftBrace,
            self.current()?
        );

        self.next();

        let mut body: Vec<LocatedStmt> = vec![];

        while !matches!(self.current()?, Token::RightBrace) {
            body.push(self.parse_statement()?);
        }

        self.next();

        Ok(body)
    }

    fn parse_statement(&mut self) -> Result<LocatedStmt, ParseError> {
        match self.current()? {
            Token::Let => todo!(),
            Token::Const => todo!(),
            Token::Return => self.parse_return(),
            Token::Break => self.parse_break(),
            Token::Continue => self.parse_continue(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Do => self.parse_do_while(),
            Token::Defer => self.parse_defer(),
            Token::Destroy => self.parse_destroy(),
            Token::Free => self.parse_free(),
            _ => {
                let expr = self.parse_expression()?;
                let span = expr.span.clone();

                expect!(
                    self,
                    self.current()?,
                    Token::SemiColon,
                    self.lexer.span(),
                    "Expected {} at the end of a statement but got {}",
                    Token::SemiColon,
                    self.current()?
                );

                self.next();

                Ok(Located {
                    node: Stmt::Expression { expr },
                    span,
                })
            }
        }
    }

    fn parse_break(&mut self) -> Result<LocatedStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        expect!(
            self,
            self.current()?,
            Token::SemiColon,
            self.lexer.span(),
            "Expected {} after break but got {}",
            Token::SemiColon,
            self.current()?
        );

        self.next();

        Ok(Located {
            node: Stmt::Break,
            span,
        })
    }

    fn parse_continue(&mut self) -> Result<LocatedStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        expect!(
            self,
            self.current()?,
            Token::SemiColon,
            self.lexer.span(),
            "Expected {} after continue but got {}",
            Token::SemiColon,
            self.current()?
        );

        self.next();

        Ok(Located {
            node: Stmt::Continue,
            span,
        })
    }

    fn parse_return(&mut self) -> Result<LocatedStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        let value = if matches!(self.current()?, Token::SemiColon) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        expect!(
            self,
            self.current()?,
            Token::SemiColon,
            self.lexer.span(),
            "Expected {} after return but got {}",
            Token::SemiColon,
            self.current()?
        );

        self.next();

        Ok(Located {
            node: Stmt::Return { value },
            span,
        })
    }

    fn parse_if(&mut self) -> Result<LocatedStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        expect!(
            self,
            self.current()?,
            Token::LeftParen,
            self.lexer.span(),
            "Expected {} before condition but got {}",
            Token::LeftParen,
            self.current()?
        );

        self.next();

        let cond = self.parse_expression()?;

        expect!(
            self,
            self.current()?,
            Token::RightParen,
            self.lexer.span(),
            "Expected {} after condition but got {}",
            Token::RightParen,
            self.current()?
        );

        self.next();

        let then = if matches!(self.current()?, Token::LeftBrace) {
            self.parse_curly_body()?
        } else {
            vec![self.parse_statement()?]
        };

        let mut other = None;

        if matches!(self.current()?, Token::Else) {
            self.next();
            other = Some(if matches!(self.current()?, Token::LeftBrace) {
                self.parse_curly_body()?
            } else {
                vec![self.parse_statement()?]
            });
        }

        Ok(Located {
            node: Stmt::If { cond, then, other },
            span,
        })
    }

    fn parse_while(&mut self) -> Result<LocatedStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        expect!(
            self,
            self.current()?,
            Token::LeftParen,
            self.lexer.span(),
            "Expected {} before condition but got {}",
            Token::LeftParen,
            self.current()?
        );

        self.next();

        let cond = self.parse_expression()?;

        expect!(
            self,
            self.current()?,
            Token::RightParen,
            self.lexer.span(),
            "Expected {} after condition but got {}",
            Token::RightParen,
            self.current()?
        );

        self.next();

        let body = if matches!(self.current()?, Token::LeftBrace) {
            self.parse_curly_body()?
        } else {
            vec![self.parse_statement()?]
        };

        Ok(Located {
            node: Stmt::While {
                cond,
                body,
                do_while: false,
            },
            span,
        })
    }

    fn parse_do_while(&mut self) -> Result<LocatedStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        let body = if matches!(self.current()?, Token::LeftBrace) {
            self.parse_curly_body()?
        } else {
            vec![self.parse_statement()?]
        };

        expect!(
            self,
            self.current()?,
            Token::LeftParen,
            self.lexer.span(),
            "Expected {} before condition but got {}",
            Token::LeftParen,
            self.current()?
        );

        self.next();

        let cond = self.parse_expression()?;

        expect!(
            self,
            self.current()?,
            Token::RightParen,
            self.lexer.span(),
            "Expected {} after condition but got {}",
            Token::RightParen,
            self.current()?
        );

        self.next();

        Ok(Located {
            node: Stmt::While {
                cond,
                body,
                do_while: true,
            },
            span,
        })
    }

    fn parse_defer(&mut self) -> Result<LocatedStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        let body = if matches!(self.current()?, Token::LeftBrace) {
            self.parse_curly_body()?
        } else {
            vec![self.parse_statement()?]
        };

        Ok(Located {
            node: Stmt::Defer { body },
            span,
        })
    }

    fn parse_destroy(&mut self) -> Result<LocatedStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        let expr = self.parse_expression()?;

        expect!(
            self,
            self.current()?,
            Token::SemiColon,
            self.lexer.span(),
            "Expected {} after body but got {}",
            Token::SemiColon,
            self.current()?
        );

        self.next();

        Ok(Located {
            node: Stmt::Destroy { expr },
            span,
        })
    }

    fn parse_free(&mut self) -> Result<LocatedStmt, ParseError> {
        let span = self.lexer.span();
        self.next();

        let expr = self.parse_expression()?;

        expect!(
            self,
            self.current()?,
            Token::SemiColon,
            self.lexer.span(),
            "Expected {} after body but got {}",
            Token::SemiColon,
            self.current()?
        );

        self.next();

        Ok(Located {
            node: Stmt::Free { expr },
            span,
        })
    }

    fn parse_expression(&mut self) -> Result<LocatedExpr, ParseError> {
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

    fn parse_assign(
        &mut self,
        lexpr: LocatedExpr,
        op: AssignOp,
    ) -> Result<LocatedExpr, ParseError> {
        let span = self.lexer.span();
        self.next();

        let rexpr = self.parse_expression()?;

        Ok(Located {
            node: Expr::Assign {
                lvalue: Box::new(lexpr),
                op,
                value: Box::new(rexpr),
            },
            span,
        })
    }

    fn parse_or_expr(&mut self) -> Result<LocatedExpr, ParseError> {
        let mut expr = self.parse_and_expr()?;

        while matches!(self.current()?, Token::Or) {
            let span = self.lexer.span();
            self.next();

            let rexpr = self.parse_and_expr()?;

            expr = Located {
                node: Expr::Binary {
                    left: Box::new(expr),
                    op: BinOp::Or,
                    right: Box::new(rexpr),
                },
                span,
            };
        }

        Ok(expr)
    }

    fn parse_and_expr(&mut self) -> Result<LocatedExpr, ParseError> {
        let mut expr = self.parse_equality()?;

        while matches!(self.current()?, Token::And) {
            let span = self.lexer.span();
            self.next();

            let rexpr = self.parse_equality()?;

            expr = Located {
                node: Expr::Binary {
                    left: Box::new(expr),
                    op: BinOp::And,
                    right: Box::new(rexpr),
                },
                span,
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<LocatedExpr, ParseError> {
        let mut expr = self.parse_comparison()?;

        while matches!(self.current()?, Token::NotEq | Token::DEq) {
            let bin_op = if matches!(self.current()?, Token::NotEq) {
                BinOp::NEq
            } else {
                BinOp::Eq
            };

            let span = self.lexer.span();
            self.next();

            let rexpr = self.parse_comparison()?;

            expr = Located {
                node: Expr::Binary {
                    left: Box::new(expr),
                    op: bin_op,
                    right: Box::new(rexpr),
                },
                span,
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<LocatedExpr, ParseError> {
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

            let span = self.lexer.span();
            self.next();

            let rexpr = self.parse_term_expr()?;

            expr = Located {
                node: Expr::Binary {
                    left: Box::new(expr),
                    op: bin_op,
                    right: Box::new(rexpr),
                },
                span,
            };
        }

        Ok(expr)
    }

    fn parse_term_expr(&mut self) -> Result<LocatedExpr, ParseError> {
        let mut expr = self.parse_factor_expr()?;

        while matches!(self.current()?, Token::Minus | Token::Plus) {
            let bin_op = if matches!(self.current()?, Token::Minus) {
                BinOp::Sub
            } else {
                BinOp::Add
            };

            let span = self.lexer.span();
            self.next();

            let rexpr = self.parse_factor_expr()?;

            expr = Located {
                node: Expr::Binary {
                    left: Box::new(expr),
                    op: bin_op,
                    right: Box::new(rexpr),
                },
                span,
            };
        }

        Ok(expr)
    }

    fn parse_factor_expr(&mut self) -> Result<LocatedExpr, ParseError> {
        let mut expr = self.parse_unary()?;

        while matches!(self.current()?, Token::Div | Token::Mod | Token::Mul) {
            let bin_op = match self.current()? {
                Token::Div => BinOp::Div,
                Token::Mul => BinOp::Mul,
                Token::Mod => BinOp::Mod,
                _ => unreachable!(),
            };

            let span = self.lexer.span();
            self.next();

            let rexpr = self.parse_unary()?;

            expr = Located {
                node: Expr::Binary {
                    left: Box::new(expr),
                    op: bin_op,
                    right: Box::new(rexpr),
                },
                span,
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<LocatedExpr, ParseError> {
        if matches!(
            self.current()?,
            Token::Not | Token::Minus | Token::Ampersand | Token::Caret
        ) {
            let unary_op = match self.current()? {
                Token::Not => UnaryOp::LogicNeg,
                Token::Minus => UnaryOp::Neg,
                Token::Ampersand => UnaryOp::AddrOf,
                Token::Caret => UnaryOp::Deref,
                _ => unreachable!(),
            };

            self.next();

            let val = self.parse_unary()?;
            let span = val.span.clone();

            Ok(Located {
                node: Expr::Unary {
                    op: unary_op,
                    expr: Box::new(val),
                },
                span,
            })
        } else {
            self.parse_call()
        }
    }

    fn parse_call(&mut self) -> Result<LocatedExpr, ParseError> {
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

    fn parse_fn_call(&mut self, callee: LocatedExpr) -> Result<LocatedExpr, ParseError> {
        let span = callee.span.clone();

        self.next();

        let mut args: Vec<LocatedExpr> = vec![];

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

        Ok(Located {
            node: Expr::FnCall {
                name: Box::new(callee),
                args,
            },
            span,
        })
    }

    fn parse_indexing(&mut self, expr: LocatedExpr) -> Result<LocatedExpr, ParseError> {
        self.next();

        let index = self.parse_expression()?;
        let span = index.span.clone();

        expect!(
            self,
            self.current()?,
            Token::RightBrak,
            self.lexer.span(),
            "Expected a pairing {} for indexing but got {}",
            Token::RightBrak,
            self.current()?
        );

        self.next();

        Ok(Located {
            node: Expr::ArrIndex {
                arr: Box::new(expr),
                idx: Box::new(index),
            },
            span,
        })
    }

    fn parse_struct_init(&mut self, expr: LocatedExpr) -> Result<LocatedExpr, ParseError> {
        let span = expr.span;
        let Expr::Ident(ident) = expr.node else {
            unreachable!()
        };

        self.next();

        let mut args: Vec<(String, LocatedExpr)> = vec![];

        while !matches!(self.current()?, Token::RightBrace) {
            args.push(self.parse_struct_init_arg()?);

            if !matches!(self.current()?, Token::Comma) {
                break;
            } else {
                self.next();
            }
        }

        self.next();

        Ok(Located {
            node: Expr::InitStruct { ident, args },
            span,
        })
    }

    fn parse_enum_variant(&mut self, expr: LocatedExpr) -> Result<LocatedExpr, ParseError> {
        let span = expr.span;
        let Expr::Ident(ident) = expr.node else {
            unreachable!()
        };

        self.next();

        self.next();

        let Token::Ident(variant) = self.current()? else {
            unreachable!()
        };

        self.next();

        Ok(Located {
            node: Expr::EnumVarAccess { ident, variant },
            span,
        })
    }

    fn parse_struct_member_field(&mut self, expr: LocatedExpr) -> Result<LocatedExpr, ParseError> {
        self.next();

        let span = expr.span.clone();
        let Token::Ident(member) = self.current()? else {
            unreachable!()
        };

        self.next();

        Ok(Located {
            node: Expr::MemAccess {
                expr: Box::new(expr),
                member,
            },
            span,
        })
    }

    fn parse_module_access(&mut self, expr: LocatedExpr) -> Result<LocatedExpr, ParseError> {
        todo!()
    }

    fn parse_primary_expr(&mut self) -> Result<LocatedExpr, ParseError> {
        match self.current()? {
            Token::True => {
                let span = self.lexer.span();
                self.next();
                Ok(Located {
                    node: Expr::Bool(true),
                    span,
                })
            }

            Token::False => {
                let span = self.lexer.span();
                self.next();
                Ok(Located {
                    node: Expr::Bool(false),
                    span,
                })
            }

            Token::Int(val) => {
                let span = self.lexer.span();
                self.next();
                Ok(Located {
                    node: Expr::Int(val),
                    span,
                })
            }

            Token::Double(val) => {
                let span = self.lexer.span();
                self.next();
                Ok(Located {
                    node: Expr::Double(val),
                    span,
                })
            }

            Token::Str(val) => {
                let span = self.lexer.span();
                self.next();
                Ok(Located {
                    node: Expr::Str(val),
                    span,
                })
            }

            Token::Ident(ident) => {
                let span = self.lexer.span();
                self.next();
                Ok(Located {
                    node: Expr::Ident(ident),
                    span,
                })
            }

            Token::LeftParen => self.parse_parenthesized(),

            Token::Make => self.parse_make_expr(),

            Token::New => self.parse_new_expr(),

            t => Err(self.create_error(format!("Unexpected token: {}", t))),
        }
    }

    fn parse_parenthesized(&mut self) -> Result<LocatedExpr, ParseError> {
        self.next();

        let l = self.lexer.span();
        let expr = self.parse_expression()?;

        expect!(
            self,
            self.current()?,
            Token::RightParen,
            self.lexer.span(),
            "Expected {} for parenthesized expression but got {}",
            Token::RightParen,
            self.current()?
        );

        self.next();

        Ok(Located {
            node: Expr::Parenthesized {
                expr: Box::new(expr),
            },
            span: l,
        })
    }

    fn parse_make_expr(&mut self) -> Result<LocatedExpr, ParseError> {
        self.next();

        let l = self.lexer.span();
        let t = self.parse_type()?.node;

        Ok(Located {
            node: Expr::Make { t },
            span: l,
        })
    }

    fn parse_new_expr(&mut self) -> Result<LocatedExpr, ParseError> {
        self.next();

        let l = self.lexer.span();
        let t = self.parse_type()?.node;

        Ok(Located {
            node: Expr::New { t },
            span: l,
        })
    }

    fn parse_struct_init_arg(&mut self) -> Result<(String, LocatedExpr), ParseError> {
        let Token::Ident(ident) = self.current()? else {
            unreachable!()
        };

        self.next();

        expect!(
            self,
            self.current()?,
            Token::Eq,
            self.lexer.span(),
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
                Err((self.lexer.span(), e.msg))
            }
        } else {
            Err((self.lexer.span(), "Unexpected end of file".to_string()))
        }
    }

    #[inline]
    fn next(&mut self) {
        self.current_token = self.lexer.next();
    }

    fn create_error_with_line_num(&self, msg: String, span: Span) -> ParseError {
        (span, msg)
    }

    fn create_error(&self, msg: String) -> ParseError {
        (self.lexer.span(), msg)
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
}

pub type LocatedExpr = Located<Expr>;
pub type LocatedStmt = Located<Stmt>;
pub type LocatedGlobalStmt = Located<GlobalStmt>;
pub type LocatedType = Located<Type>;
pub type Span = Range<usize>;

#[derive(Debug, Clone)]
pub struct Located<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Double(f64),
    Bool(bool),
    Char(u8),
    Str(String),
    Ident(String),
    Binary {
        left: Box<LocatedExpr>,
        op: BinOp,
        right: Box<LocatedExpr>,
    },
    Parenthesized {
        expr: Box<LocatedExpr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<LocatedExpr>,
    },
    Assign {
        lvalue: Box<LocatedExpr>,
        op: AssignOp,
        value: Box<LocatedExpr>,
    },
    Ternary {
        cond: Box<LocatedExpr>,
        lexpr: Box<LocatedExpr>,
        rexpr: Box<LocatedExpr>,
    },
    FnCall {
        name: Box<LocatedExpr>,
        args: Vec<LocatedExpr>,
    },
    MemAccess {
        expr: Box<LocatedExpr>,
        member: String,
    },
    EnumVarAccess {
        ident: String,
        variant: String,
    },
    ArrIndex {
        arr: Box<LocatedExpr>,
        idx: Box<LocatedExpr>,
    },
    Cast {
        t: LocatedType,
        expr: Box<LocatedExpr>,
    },
    Sizeof {
        t: Type,
    },
    InitArr {
        elems: Vec<LocatedExpr>,
    },
    InitArrDesignated {
        idxs: Vec<usize>,
        elems: Vec<LocatedExpr>,
    },
    InitStruct {
        ident: String,
        args: Vec<(String, LocatedExpr)>,
    },
    Make {
        t: Type,
    },
    New {
        t: Type,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Variable {
        name: String,
        t: Option<Type>,
        value: Option<LocatedExpr>,
        private: bool,
        is_const: bool,
    },
    Expression {
        expr: LocatedExpr,
    },
    Return {
        value: Option<LocatedExpr>,
    },
    Break,
    Continue,
    If {
        cond: LocatedExpr,
        then: Vec<LocatedStmt>,
        other: Option<Vec<LocatedStmt>>,
    },
    While {
        cond: LocatedExpr,
        body: Vec<LocatedStmt>,
        do_while: bool,
    },
    Defer {
        body: Vec<LocatedStmt>,
    },
    Destroy {
        expr: LocatedExpr,
    },
    Free {
        expr: LocatedExpr,
    },
}

#[derive(Debug)]
pub enum GlobalStmt {
    Enum {
        name: String,
        variants: Vec<(String, Option<i64>)>,
    },
    Struct {
        name: String,
        fields: Vec<(String, LocatedType)>,
    },
    Union {
        name: String,
        fields: Vec<(String, LocatedType)>,
    },
    Function {
        name: String,
        params: Vec<(String, LocatedType)>,
        ret: LocatedType,
        body: Vec<LocatedStmt>,
    },
    Variable {
        name: String,
        t: Option<Type>,
        value: Option<LocatedExpr>,
        private: bool,
    },
    Constant {
        name: String,
        t: Option<LocatedType>,
        value: LocatedExpr,
        private: bool,
    },
    Alias {
        t: LocatedType,
        name: String,
    },
    Import {
        name: String,
        path: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
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

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Type::*;

        match self {
            Void => write!(f, "void"),
            Double => write!(f, "double"),
            Float => write!(f, "float"),
            Char => write!(f, "char"),
            Str => write!(f, "str"),
            Int8 => write!(f, "i8"),
            Int16 => write!(f, "i16"),
            Int32 => write!(f, "i32"),
            Int64 => write!(f, "i64"),
            UInt8 => write!(f, "u8"),
            UInt16 => write!(f, "u16"),
            UInt32 => write!(f, "u32"),
            UInt64 => write!(f, "u64"),
            Bool => write!(f, "bool"),
            Pointer(t) => write!(f, "^{t}"),
            Array(l, t) => write!(f, "[{l}]{t}"),
            DArray(t) => write!(f, "[^]{t}"),
            UserDefinedType(n) => write!(f, "{n}"),
        }
    }
}
