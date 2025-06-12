use std::collections::hash_map::Entry;
use std::collections::HashMap;

use tamago::{AssignOp, BinOp, UnaryOp};

use crate::parser::*;
use crate::semantic_analyzer::*;

type ReturnType = Option<(Type, bool)>;

#[derive(Debug)]
enum UserDefinedType<'ast> {
    Enum {
        span: &'ast Span,
        variants: &'ast Vec<(String, Option<i64>)>,
    },
    Struct {
        span: &'ast Span,
        fields: &'ast Vec<(String, LocatedType)>,
    },
    Union {
        span: &'ast Span,
        fields: &'ast Vec<(String, LocatedType)>,
    },
    Function {
        span: &'ast Span,
        params: &'ast Vec<(String, LocatedType)>,
        ret: &'ast LocatedType,
    },
    Alias {
        span: &'ast Span,
        t: &'ast LocatedType,
    },
    Import {
        span: &'ast Span,
        name: &'ast str,
    },
}

#[derive(Debug, Default)]
struct Types<'ast> {
    types: HashMap<&'ast str, LocatedType>,
    enclosing: Option<Box<Types<'ast>>>,
}

#[derive(Debug)]
pub struct TypeChecker<'ast> {
    ast: &'ast Vec<LocatedGlobalStmt>,
    types: Types<'ast>,
    user_def_types: HashMap<&'ast str, UserDefinedType<'ast>>,

    errors: Vec<Message>,
    warnings: Vec<Message>,
}

impl<'ast> TypeChecker<'ast> {
    pub fn new(ast: &'ast Vec<LocatedGlobalStmt>) -> Self {
        Self {
            ast,
            types: Types::new(),
            user_def_types: HashMap::new(),
            errors: vec![],
            warnings: vec![],
        }
    }

    pub fn check(mut self) -> (Vec<Message>, Result<(), Vec<Message>>) {
        for stmt in self.ast {
            self.check_global_stmt(stmt);
        }

        if self.errors.is_empty() {
            (self.warnings, Ok(()))
        } else {
            (self.warnings, Err(self.errors))
        }
    }

    fn check_global_stmt(&mut self, stmt: &'ast LocatedGlobalStmt) {
        use GlobalStmt::*;

        let Located { node: gstmt, span } = stmt;

        match gstmt {
            Enum { name, .. }
            | Struct { name, .. }
            | Union { name, .. }
            | Alias { name, .. }
            | Import { name, .. } => {
                let _ = self.define_user_type(name, stmt);
            }

            Function {
                name,
                params,
                ret,
                body,
            } => {
                if let Err(w) = self.types.declare(
                    name,
                    Located {
                        node: Type::UserDefinedType(name.to_string()),
                        span: span.clone(),
                    },
                ) {
                    self.warnings.push(w);
                }

                let _ = self.define_user_type(name, stmt);

                let old_types = std::mem::take(&mut self.types);
                self.types = Types::new_with_types(old_types);

                for (name, t) in params {
                    if let Err(err) = self.types.declare(name, t.clone()) {
                        self.errors.push(err);
                    }
                }

                self.check_func_body(ret, body);
            }

            Variable {
                name,
                t,
                value: Some(value),
                ..
            } => {
                let var_t: Type;
                if let Some(t) = t {
                    var_t = t.clone();
                    let given_t: Type;
                    match self.check_expr(value) {
                        Err(err) => {
                            self.errors.push(err);
                            return;
                        }
                        Ok(t) => given_t = t,
                    }

                    if var_t != given_t {
                        self.errors.push((
                            span.clone(),
                            format!("Expected '{var_t}' but got '{given_t}'"),
                        ));
                        return;
                    }
                } else {
                    match self.check_expr(value) {
                        Err(err) => {
                            self.errors.push(err);
                            return;
                        }
                        Ok(t) => var_t = t,
                    }
                }

                if let Err(err) = self.types.declare(
                    name,
                    Located {
                        node: var_t,
                        span: span.clone(),
                    },
                ) {
                    self.errors.push(err);
                }
            }

            Variable {
                name,
                t,
                value: None,
                ..
            } => {
                if let Some(t) = t {
                    if let Err(err) = self.types.declare(
                        name,
                        Located {
                            node: t.clone(),
                            span: span.clone(),
                        },
                    ) {
                        self.errors.push(err);
                    }
                } else {
                    self.errors.push((
                        span.clone(),
                        format!("Expected an explicit type but got nothing"),
                    ));
                }
            }

            Constant { name, t, value, .. } => {
                todo!() // TODO: Only allow compile-time expressions
            }
        }
    }

    fn check_func_body(&mut self, ret: &'ast LocatedType, body: &'ast Vec<LocatedStmt>) {
        todo!()
    }

    /// Some(true) -> full return
    /// Some(false) -> partial return
    /// None -> no return (same as void)
    fn check_stmt(
        &mut self,
        expected_ret: &'ast LocatedType,
        stmt: &'ast LocatedStmt,
    ) -> Result<Option<bool>, Message> {
        use Stmt::*;

        let Located { node: s, span } = stmt;

        match s {
            Variable {
                name,
                t,
                value: Some(value),
                ..
            } => {
                let var_t: Type;
                if let Some(t) = t {
                    var_t = t.clone();
                    let given_t = self.check_expr(value)?;

                    if var_t != given_t {
                        return Err((
                            span.clone(),
                            format!("Expected '{var_t}' but got '{given_t}'"),
                        ));
                    }
                } else {
                    var_t = self.check_expr(value)?;
                }

                self.types.declare(
                    name,
                    Located {
                        node: var_t,
                        span: span.clone(),
                    },
                )?;

                Ok(None)
            }

            Variable {
                name,
                t,
                value: None,
                ..
            } => {
                if let Some(t) = t {
                    self.types.declare(
                        name,
                        Located {
                            node: t.clone(),
                            span: span.clone(),
                        },
                    )?;

                    Ok(None)
                } else {
                    Err((
                        span.clone(),
                        format!("Expected an explicit type but got nothing"),
                    ))
                }
            }

            Expression { expr } => {
                self.check_expr(expr)?;
                Ok(None)
            }

            Return { value } => {
                if let Some(val) = value {
                    let t = self.check_expr(val)?;
                    if expected_ret.node == t {
                        Ok(Some(true))
                    } else {
                        Err((
                            span.clone(),
                            format!(
                                "Expected {} as return type but got {}",
                                expected_ret.node, t
                            ),
                        ))
                    }
                } else {
                    if expected_ret.node == Type::Void {
                        Ok(Some(true))
                    } else {
                        Err((
                            span.clone(),
                            format!(
                                "Expected {} as return type but got {}",
                                expected_ret.node,
                                Type::Void
                            ),
                        ))
                    }
                }
            }

            Break | Continue => Ok(None),

            If { cond, then, other } => {
                let cond_t = self.check_expr(cond)?;
                if cond_t != Type::Bool {
                    return Err((
                        span.clone(),
                        format!("If condition must be boolean but got {}", cond_t),
                    ));
                }

                let then_returns = self.check_branch(expected_ret, then)?;

                let mut else_returns = false;
                if let Some(other) = other {
                    else_returns = self.check_branch(expected_ret, other)?;
                }

                if then_returns && else_returns {
                    // both then & else have a return stmt
                    Ok(Some(true))
                } else if !then_returns && !else_returns {
                    // neither then or else have a return stmt
                    Ok(None)
                } else {
                    // either then or else has a return stmt
                    Ok(Some(false))
                }
            }

            While { cond, body, .. } => {
                let cond_t = self.check_expr(cond)?;
                if cond_t != Type::Bool {
                    return Err((
                        span.clone(),
                        format!("While condition must be boolean but got {}", cond_t),
                    ));
                }

                if self.check_branch(expected_ret, body)? {
                    Ok(Some(false))
                } else {
                    Ok(None)
                }
            }

            Defer { body } => {
                for stmt in body {
                    if matches!(self.check_stmt(expected_ret, stmt)?, Some(..)) {
                        return Err((
                            stmt.span.clone(),
                            "No return statement is allowed in defer body".to_string(),
                        ));
                    }
                }
                Ok(None)
            }

            Destroy { expr } => {
                let t = self.check_expr(expr);
                Ok(None)
            }

            Free { expr } => {
                let t = self.check_expr(expr);
                Ok(None)
            }
        }
    }

    fn check_expr(&self, expr: &'ast LocatedExpr) -> Result<Type, Message> {
        use Expr::*;

        let Located { node: e, span } = expr;

        match e {
            Int(..) => Ok(Type::Int32),
            Double(..) => Ok(Type::Double),
            Bool(..) => Ok(Type::Bool),
            Char(..) => Ok(Type::Char),
            Str(..) => Ok(Type::Str),
            Ident(name) => todo!(),
            Binary { left, op, right } => self.check_binary(span, left, op, right),
            Parenthesized { expr } => self.check_expr(expr),
            Unary { op, expr } => self.check_unary(span, op, expr),
            Assign { lvalue, op, value } => self.check_assign(span, lvalue, op, value),
            Ternary { cond, lexpr, rexpr } => self.check_ternary(span, lexpr, rexpr),
            FnCall { name, args } => self.check_fn_call(span, name, args),
            MemAccess { expr, member } => self.check_mem_access(span, expr, member),
            EnumVarAccess { ident, variant } => self.check_enum_var_access(span, ident, variant),
            ArrIndex { arr, idx } => self.check_arr_index(span, arr, idx),
            Cast { t, expr } => self.check_cast(span, t, expr),
            Sizeof { t } => self.check_sizeof(span, t),
            InitArr { elems } => self.check_init_arr(span, elems),
            InitArrDesignated { idxs, elems } => self.check_init_arr_designated(span, idxs, elems),
            InitStruct { ident, args } => self.check_init_struct(span, ident, args),
            Make { t } => self.check_make(span, t),
            New { t } => self.check_new(span, t),
        }
    }

    fn check_new(&self, span: &'ast Span, t: &'ast Type) -> Result<Type, Message> {
        todo!()
    }

    fn check_make(&self, span: &'ast Span, t: &'ast Type) -> Result<Type, Message> {
        todo!()
    }

    fn check_init_struct(
        &self,
        span: &'ast Span,
        ident: &'ast String,
        args: &'ast Vec<(String, LocatedExpr)>,
    ) -> Result<Type, Message> {
        todo!()
    }

    fn check_init_arr_designated(
        &self,
        span: &'ast Span,
        idx: &'ast Vec<usize>,
        elems: &'ast Vec<LocatedExpr>,
    ) -> Result<Type, Message> {
        todo!()
    }

    fn check_init_arr(
        &self,
        span: &'ast Span,
        elems: &'ast Vec<LocatedExpr>,
    ) -> Result<Type, Message> {
        todo!()
    }

    fn check_sizeof(&self, span: &'ast Span, t: &'ast Type) -> Result<Type, Message> {
        todo!()
    }

    fn check_cast(
        &self,
        span: &'ast Span,
        t: &'ast LocatedType,
        expr: &'ast LocatedExpr,
    ) -> Result<Type, Message> {
        todo!()
    }

    fn check_arr_index(
        &self,
        span: &'ast Span,
        args: &'ast LocatedExpr,
        idx: &'ast LocatedExpr,
    ) -> Result<Type, Message> {
        todo!()
    }

    fn check_enum_var_access(
        &self,
        span: &'ast Span,
        ident: &'ast String,
        variant: &'ast String,
    ) -> Result<Type, Message> {
        todo!()
    }

    fn check_mem_access(
        &self,
        span: &'ast Span,
        expr: &'ast LocatedExpr,
        member: &'ast String,
    ) -> Result<Type, Message> {
        todo!()
    }

    fn check_fn_call(
        &self,
        span: &'ast Span,
        name: &'ast LocatedExpr,
        args: &'ast Vec<LocatedExpr>,
    ) -> Result<Type, Message> {
        todo!()
    }

    fn check_ternary(
        &self,
        span: &'ast Span,
        lexpr: &'ast LocatedExpr,
        rexpr: &'ast LocatedExpr,
    ) -> Result<Type, Message> {
        todo!()
    }

    fn check_binary(
        &self,
        span: &'ast Span,
        left: &'ast LocatedExpr,
        op: &'ast BinOp,
        right: &'ast LocatedExpr,
    ) -> Result<Type, Message> {
        todo!()
    }

    fn check_unary(
        &self,
        span: &'ast Span,
        op: &'ast UnaryOp,
        expr: &'ast LocatedExpr,
    ) -> Result<Type, Message> {
        todo!()
    }

    fn check_assign(
        &self,
        span: &'ast Span,
        lvalue: &'ast LocatedExpr,
        op: &'ast AssignOp,
        value: &'ast LocatedExpr,
    ) -> Result<Type, Message> {
        todo!()
    }

    /// true -> the branch has a return
    /// false -> the branch doesn't have a return
    fn check_branch(
        &mut self,
        expected_ret: &'ast LocatedType,
        branch: &'ast Vec<LocatedStmt>,
    ) -> Result<bool, Message> {
        let mut result: Result<bool, Message> = Ok(false);
        for stmt in branch {
            if result != Ok(false) {
                self.warnings
                    .push((stmt.span.clone(), format!("Unreachable code after return")));
                break;
            }

            match self.check_stmt(expected_ret, stmt)? {
                Some(p) if p => {
                    result = Ok(true);
                }
                _ => {}
            }
        }

        result
    }

    fn define_user_type(
        &mut self,
        name: &'ast str,
        stmt: &'ast LocatedGlobalStmt,
    ) -> Result<(), Message> {
        let span = stmt.span.clone();
        let t: UserDefinedType<'ast>;

        if let Ok(ty) = UserDefinedType::<'ast>::try_from(stmt) {
            t = ty;
        } else {
            todo!(); // TODO: probably just ignore?
                     // return Err((span, format!("")));
        }

        match self.user_def_types.entry(name) {
            Entry::Occupied(_) => Err((span, format!("Type '{name}' is already declared"))),
            Entry::Vacant(entry) => {
                entry.insert(t);
                Ok(())
            }
        }
    }
}

impl<'ast> Types<'ast> {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_types(types: Types<'ast>) -> Self {
        Self {
            types: HashMap::new(),
            enclosing: Some(Box::new(types)),
        }
    }

    pub fn declare(&mut self, name: &'ast str, t: LocatedType) -> Result<(), Message> {
        match self.types.entry(name) {
            Entry::Occupied(_) => Err((t.span.clone(), format!("'{name}' is already declared"))),
            Entry::Vacant(entry) => {
                entry.insert(t);
                Ok(())
            }
        }
    }

    pub fn is_declared(&mut self, name: &'ast str, span: Span) -> Result<(), Message> {
        if self.types.contains_key(name) {
            Ok(())
        } else if let Some(types) = &mut self.enclosing {
            types.is_declared(name, span)
        } else {
            Err((span, format!("'{name}' is not declared")))
        }
    }
}

impl<'ast> TryFrom<&'ast LocatedGlobalStmt> for UserDefinedType<'ast> {
    type Error = ();

    fn try_from(value: &'ast LocatedGlobalStmt) -> Result<Self, Self::Error> {
        use GlobalStmt::*;

        let Located { span, node: value } = value;

        match value {
            Enum { variants, .. } => Ok(UserDefinedType::Enum { span, variants }),
            Struct { fields, .. } => Ok(UserDefinedType::Struct { span, fields }),
            Union { fields, .. } => Ok(UserDefinedType::Union { span, fields }),
            Function { params, ret, .. } => Ok(UserDefinedType::Function { span, params, ret }),
            Alias { t, .. } => Ok(UserDefinedType::Alias { span, t }),
            Import { name, .. } => Ok(UserDefinedType::Import { span, name }),
            _ => Err(()),
        }
    }
}
