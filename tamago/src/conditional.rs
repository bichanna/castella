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

//! This module provides ways to create conditional statements like if-else and switch in C.

use std::fmt::{self, Write};

use crate::{Block, Expr, Format, Formatter, Statement};
use tamacro::DisplayFromFormat;

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct If {
    pub cond: Expr,
    pub then: Block,
    pub other: Option<Block>,
}

impl If {
    pub fn new(cond: Expr) -> IfBuilder {
        IfBuilder::new(cond)
    }
}

impl Format for If {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "if (")?;
        self.cond.format(fmt)?;
        write!(fmt, ")")?;

        fmt.block(|fmt| self.then.format(fmt))?;

        if let Some(other) = &self.other {
            write!(fmt, " else")?;
            fmt.block(|fmt| other.format(fmt))?;
        }

        writeln!(fmt)
    }
}

pub struct IfBuilder {
    cond: Expr,
    then: Block,
    other: Option<Block>,
}

impl IfBuilder {
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            then: Block::new().build(),
            other: None,
        }
    }

    pub fn new_with_then(cond: Expr, then: Block) -> Self {
        Self {
            cond,
            then,
            other: None,
        }
    }

    pub fn then(mut self, then: Block) -> Self {
        self.then = then;
        self
    }

    pub fn other(mut self, other: Block) -> Self {
        self.other = Some(other);
        self
    }

    pub fn statement_to_then(mut self, stmt: Statement) -> Self {
        self.then.stmts.push(stmt);
        self
    }

    pub fn build(self) -> If {
        If {
            cond: self.cond,
            then: self.then,
            other: self.other,
        }
    }
}

#[derive(Debug, Clone, DisplayFromFormat)]
pub struct Switch {
    pub cond: Expr,
    pub cases: Vec<(Expr, Block)>,
    pub default: Option<Block>,
}

impl Switch {
    pub fn new(cond: Expr) -> SwitchBuilder {
        SwitchBuilder::new(cond)
    }
}

impl Format for Switch {
    fn format(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "switch (")?;
        self.cond.format(fmt)?;
        writeln!(fmt, ") {{")?;

        for (label, block) in &self.cases {
            write!(fmt, "case ")?;
            label.format(fmt)?;
            write!(fmt, ":")?;

            fmt.block(|fmt| block.format(fmt))?;
            writeln!(fmt)?;
        }

        if let Some(def) = &self.default {
            write!(fmt, "default:")?;
            fmt.block(|fmt| def.format(fmt))?;
            writeln!(fmt)?;
        }

        writeln!(fmt, "}}")
    }
}

pub struct SwitchBuilder {
    cond: Expr,
    cases: Vec<(Expr, Block)>,
    default: Option<Block>,
}

impl SwitchBuilder {
    pub fn new(cond: Expr) -> Self {
        Self {
            cond,
            cases: vec![],
            default: None,
        }
    }

    pub fn new_with_cases(cond: Expr, cases: Vec<(Expr, Block)>) -> Self {
        Self {
            cond,
            cases,
            default: None,
        }
    }

    pub fn case(mut self, c: Expr, b: Block) -> Self {
        self.cases.push((c, b));
        self
    }

    pub fn cases(mut self, cases: Vec<(Expr, Block)>) -> Self {
        self.cases = cases;
        self
    }

    pub fn default(mut self, default: Block) -> Self {
        self.default = Some(default);
        self
    }

    pub fn build(self) -> Switch {
        Switch {
            cond: self.cond,
            cases: self.cases,
            default: self.default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn if_condition() {
        let another_if = IfBuilder::new(Expr::new_binary(
            Expr::new_ident_with_str("another_var"),
            BinOp::Eq,
            Expr::new_ident_with_str("some_var"),
        ))
        .then(
            BlockBuilder::new()
                .statements(vec![
                    Statement::GoTo("hello".to_string()),
                    Statement::WarningDirective(
                        WarningDirectiveBuilder::new_with_str("some warning").build(),
                    ),
                ])
                .build(),
        )
        .build();

        let i = IfBuilder::new(Expr::ConstBool(true))
            .then(
                BlockBuilder::new()
                    .statements(vec![
                        Statement::Comment(CommentBuilder::new_with_str("Some comment").build()),
                        Statement::ErrorDirective(
                            ErrorDirectiveBuilder::new_with_str("some error").build(),
                        ),
                        Statement::Return(None),
                    ])
                    .build(),
            )
            .other(Block::new().statement(Statement::If(another_if)).build())
            .build();

        let res = r#"if (true) {
  // Some comment
  #error "some error"
  return;
} else {
  if ((another_var == some_var)) {
    goto hello;
    #warning "some warning"
  }
}
"#;

        assert_eq!(i.to_string(), res);
    }

    #[test]
    fn switch_condition() {
        let s = SwitchBuilder::new(Expr::ConstBool(true))
            .case(
                Expr::new_null(),
                Block::new()
                    .statements(vec![
                        Statement::Comment(CommentBuilder::new_with_str("Hello, world").build()),
                        Statement::Comment(CommentBuilder::new_with_str("Another comment").build()),
                    ])
                    .build(),
            )
            .case(
                Expr::new_cast(Type::new(BaseType::UInt8).build(), Expr::ConstInt(123)),
                Block::new()
                    .statement(Statement::Macro(Macro::Obj(
                        ObjMacroBuilder::new_with_str("AGE")
                            .value_with_str("18")
                            .build(),
                    )))
                    .build(),
            )
            .default(
                Block::new()
                    .statement(Statement::Raw("abc;".to_string()))
                    .build(),
            )
            .build();

        let res = r#"switch (true) {
case NULL: {
  // Hello, world
  // Another comment
}
case (uint8_t)(123): {
  #define AGE 18
}
default: {
  abc;
}
}
"#;

        assert_eq!(s.to_string(), res);
    }
}
