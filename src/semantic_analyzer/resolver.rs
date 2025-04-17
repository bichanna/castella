use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::parser::*;
use crate::semantic_analyzer::*;

#[derive(Debug, Default)]
pub struct Scope<'ast> {
    names: HashMap<&'ast str, ()>,
    enclosing: Option<Box<Scope<'ast>>>,
}

#[derive(Debug)]
pub struct Resolver<'ast> {
    ast: &'ast Vec<LocatedGlobalStmt>,
    scope: Scope<'ast>,

    errors: Vec<Message>,
    warnings: Vec<Message>,
}

impl<'ast> Resolver<'ast> {
    pub fn new(ast: &'ast Vec<LocatedGlobalStmt>) -> Self {
        Self {
            ast,
            scope: Scope::new(),
            errors: vec![],
            warnings: vec![],
        }
    }

    pub fn resolve(mut self) -> Result<Vec<Message>, (Vec<Message>, Vec<Message>)> {
        for stmt in self.ast {
            self.resolve_global_stmt(stmt);
        }

        if self.errors.is_empty() {
            Ok(self.warnings)
        } else {
            Err((self.warnings, self.errors))
        }
    }

    fn resolve_global_stmt(&mut self, stmt: &'ast LocatedGlobalStmt) {
        use crate::parser::GlobalStmt::*;

        let Located { node: stmt, span } = stmt;

        match stmt {
            Enum { name, .. }
            | Struct { name, .. }
            | Union { name, .. }
            | Variable { name, .. }
            | Constant { name, .. } => {
                if let Err(err) = self.scope.declare(&name, span.clone()) {
                    self.errors.push(err);
                }
            }
            Function {
                name,
                params,
                ret: _,
                body,
            } => self.resolve_func(span, name, params, body),
            Import { name, path } => self.resolve_import(span, name, path),
            Alias { .. } => {} // Will be handled in the type checker
        }
    }

    fn resolve_func(
        &mut self,
        span: &Span,
        name: &'ast String,
        params: &'ast Vec<(String, LocatedType)>,
        body: &'ast Vec<LocatedStmt>,
    ) {
        if let Err(err) = self.scope.declare(&name, span.clone()) {
            self.errors.push(err);
        }

        let old_scope = std::mem::take(&mut self.scope);
        self.scope = Scope::new_with_scope(old_scope);

        for p in params {
            if let Err(err) = self.scope.declare(&p.0, p.1.span.clone()) {
                self.errors.push(err);
            }
        }

        for stmt in body {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_import(&mut self, span: &Span, name: &Option<String>, path: &String) {
        // TODO: Handle whether the module being imported even exists or not
        todo!()
    }

    fn resolve_stmt(&mut self, stmt: &'ast LocatedStmt) {
        use crate::parser::Stmt::*;

        let Located { node: stmt, span } = stmt;

        match stmt {
            Variable { name, value, .. } => {
                if let Err(err) = self.scope.has(&name, span.clone()) {
                    self.errors.push(err);
                }
                if let Some(value) = value {
                    self.resolve_expr(value);
                }
            }
            Expression { expr }
            | Return { value: Some(expr) }
            | Destroy { expr }
            | Free { expr } => {
                self.resolve_expr(expr);
            }
            If { cond, then, other } => self.resolve_if(cond, then, other),
            While { cond, body, .. } => self.resolve_while(cond, body),
            Defer { body } => {
                for stmt in body {
                    self.resolve_stmt(stmt);
                }
            }
            Break | Continue | Return { value: None } => {}
        }
    }

    fn resolve_if(
        &mut self,
        cond: &'ast LocatedExpr,
        then: &'ast Vec<LocatedStmt>,
        other: &'ast Option<Vec<LocatedStmt>>,
    ) {
        self.resolve_expr(cond);

        for stmt in then {
            self.resolve_stmt(stmt);
        }

        if let Some(other) = other {
            for stmt in other {
                self.resolve_stmt(stmt);
            }
        }
    }

    fn resolve_while(&mut self, cond: &'ast LocatedExpr, body: &'ast Vec<LocatedStmt>) {
        self.resolve_expr(cond);

        for stmt in body {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_expr(&mut self, expr: &'ast LocatedExpr) {
        use crate::parser::Expr::*;

        let Located { node: expr, span } = expr;

        match expr {
            Int(_)
            | Double(_)
            | Bool(_)
            | Char(_)
            | Str(_)
            | Sizeof { .. }
            | Make { .. }
            | New { .. } => {}
            Ident(name) => {
                if let Err(err) = self.scope.has(&name, span.clone()) {
                    self.errors.push(err);
                }
            }
            Binary { left, right, .. } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Parenthesized { expr }
            | Unary { expr, .. }
            | MemAccess { expr, .. }
            | Cast { expr, .. } => {
                self.resolve_expr(expr);
            }
            Assign { lvalue, value, .. } => {
                self.resolve_assign(lvalue, value);
            }
            Ternary { cond, lexpr, rexpr } => {
                self.resolve_expr(cond);
                self.resolve_expr(lexpr);
                self.resolve_expr(rexpr);
            }
            FnCall { name, args } => {
                self.resolve_expr(name);
                for arg in args {
                    self.resolve_expr(arg);
                }
            }
            EnumVarAccess { ident, .. } => {
                if let Err(err) = self.scope.has(&ident, span.clone()) {
                    self.errors.push(err);
                }
            }
            ArrIndex { arr, idx } => {
                self.resolve_expr(arr);
                self.resolve_expr(idx);
            }
            InitArr { elems } | InitArrDesignated { elems, .. } => {
                for elem in elems {
                    self.resolve_expr(elem);
                }
            }
            InitStruct { ident, args } => {
                if let Err(err) = self.scope.has(&ident, span.clone()) {
                    self.errors.push(err);
                }
                for arg in args {
                    self.resolve_expr(&arg.1);
                }
            }
        }
    }

    fn resolve_assign(&mut self, lvalue: &'ast Box<LocatedExpr>, value: &'ast Box<LocatedExpr>) {
        use crate::parser::Expr::*;

        let Located {
            node: lvalue,
            span: lspan,
        } = &**lvalue;

        match lvalue {
            Ident(var) => {
                if let Err(err) = self.scope.has(&var, lspan.clone()) {
                    self.errors.push(err);
                }
            }
            // Should I allow Expr::Assign to be a lvalue?
            MemAccess { expr, .. } => {
                self.resolve_expr(&expr);
            }
            ArrIndex { arr, idx } => {
                self.resolve_expr(&arr);
                self.resolve_expr(&idx);
            }
            _ => self
                .errors
                .push((lspan.clone(), format!("Invalid lvalue for assignment"))),
        }

        self.resolve_expr(value);
    }
}

impl<'ast> Scope<'ast> {
    pub fn new() -> Self {
        Self {
            names: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_scope(scope: Scope<'ast>) -> Self {
        Self {
            names: HashMap::new(),
            enclosing: Some(Box::new(scope)),
        }
    }

    pub fn declare(&mut self, name: &'ast str, span: Span) -> Result<(), Message> {
        match self.names.entry(name) {
            Entry::Occupied(_) => Err((span, format!("'{name}' is already declared"))),
            Entry::Vacant(entry) => {
                entry.insert(());
                Ok(())
            }
        }
    }

    pub fn has(&self, name: &'ast str, span: Span) -> Result<(), Message> {
        if self.names.contains_key(name) {
            Ok(())
        } else if let Some(scope) = &self.enclosing {
            scope.has(name, span)
        } else {
            Err((span, format!("'{name}' is not declared")))
        }
    }
}
