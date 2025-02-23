# Tamago

Tamago is a code generator library for C, written in Rust. It is designed to simplify the process of generating C code programmatically, leveraging Rust's safety and expressiveness. This crate makes heavy use of the builder pattern to provide a pretty API (I hope) for constructing C code structures.

Tamago is primarily developed as a core component for the [Castella transpiler](https://github.com/bichanna/castella), but it is designed to be reusable for any project that needs to generate C code dynamically.

## Features

- Generate C code programmatically with a type-safe Rust API.
- Builder pattern for ergonomic and readable code generation.
- Lightweight and focused on simplicity.

## Installation

Add `tamago` to your project by including it in your `Cargo.toml`:

```toml
[dependencies]
tamago = "0.1.0"  # Replace with the actual version
```

## Usage
```rust
use tamago::*;

let scope = ScopeBuilder::new()
    .global_statement(GlobalStatement::Struct(
        StructBuilder::new_with_str("Person")
            .doc(
                DocCommentBuilder::new()
                    .line_str("Represents a person")
                    .build(),
            )
            .field(
                FieldBuilder::new_with_str(
                    "name",
                    Type::new(BaseType::Char)
                        .make_pointer()
                        .make_const()
                        .build(),
                )
                .doc(
                    DocCommentBuilder::new()
                        .line_str("The name of the person")
                        .build(),
                )
                .build(),
            )
            .field(
                FieldBuilder::new_with_str("age", Type::new(BaseType::Int).build())
                    .doc(
                        DocCommentBuilder::new()
                            .line_str("The age of the person")
                            .build(),
                    )
                    .build(),
            )
            .build(),
    ))
    .new_line()
    .global_statement(GlobalStatement::TypeDef(
        TypeDefBuilder::new_with_str(
            Type::new(BaseType::Struct("Person".to_string())).build(),
            "Person",
        )
        .build(),
    ))
    .build();

println!("{}", scope.to_string());
```
And here's output:
```c
/// Represents a person
struct Person {
  /// The name of the person
  const char* name;
  /// The age of the person
  int age;
};

typedef struct Person Person;
```
