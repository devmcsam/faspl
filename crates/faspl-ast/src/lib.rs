//   Copyright 2026 Samuel McNally
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

//! The AST for the faspl programming language.

pub mod definition;
pub mod control_flow;
pub mod operator;
pub mod literals;

use faspl_common::interner::Symbol;
use faspl_common::span::Span;
pub use definition::*;
pub use control_flow::*;
pub use operator::*;
use crate::literals::Literal;

/// Any AST node wrapped with its source location.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Spanned<T> {
    /// The AST node that is being annotated with a span.
    pub node: T,
    /// The source code span that the node is located in.
    pub span: Span,
}

/// A full source file / compilation unit.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// The vector that contains all the source code items
    pub items: Vec<Spanned<Item>>,
}

/// An item in the source code.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    /// `fn foo<T: Trait>(x: i32, y: T) -> bool { ... }`
    Function(FnDef),

    /// `struct Point { x: i32, y: i32 }`
    Struct(StructDef),

    /// `enum Option<T> { Some(T), None }`
    Enum(EnumDef),

    /// `impl Point { ... }` or `impl<T: Trait> Foo<T> { ... }`
    Impl(ImplBlock),

    /// `trait Drawable { ... }`
    Trait(TraitDef),
}

/// A type/type annotation.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// A named type, possibly generic: `i32`, `Vec<T>`, `Point`
    Named {
        /// The name of the type. This is stored as a symbol handle in the interner hashmap.
        name: Symbol,
        /// The generic arguments of the type, if any.
        type_args: Vec<Spanned<Self>>,
    },
    /// Raw pointer: `*T` or `*mut T`
    Pointer {
        /// Whether the pointer is mutable: `*mut T`
        mutable: bool,
        /// The type pointed to: `T`
        pointee: Box<Spanned<Self>>,
    },
    /// `void` / unit — no return value
    Void,
}

/// A block of code, which can contain multiple statements.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The list of expressions in the block.
    pub stmts: Vec<Spanned<Stmt>>,
}

/// A statement in a block.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// `let x: i32 = 5;` or `let mut x = 5;`
    Let {
        /// The name of the variable. This is stored as a symbol handle in the interner hashmap.
        name: Symbol,
        /// Whether the variable is mutable: `let mut x = 5;`
        mutable: bool,
        /// The type of the variable: `let x: i32 = 5;`
        ty: Option<Spanned<Type>>,
        /// The expression that is assigned to the variable.
        value: Option<Spanned<Expr>>,
    },
    /// Expression used as a statement (includes calls, assignments)
    Expr(Spanned<Expr>),
    /// `return expr;`
    Return(Option<Spanned<Expr>>),
}

/// An expression or node in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// literal: `42`, `true`, `"hello world"`, this means essentially just a value that is known at compile time.
    Literal(Literal),
    /// Variable / function reference: `x`, `foo`
    Ident(Symbol),

    /// Binary op: `a + b`, `x == y`
    BinOp {
        /// The operator being applied.
        op: BinOp,
        /// The left-hand side of the operation.
        lhs: Box<Spanned<Self>>,
        /// The right-hand side of the operation.
        rhs: Box<Spanned<Self>>,
    },
    /// Unary op: `-x`, `!flag`, `*ptr`
    UnaryOp {
        /// The operator being applied.
        op: UnaryOp,
        /// The operand of the operation.
        operand: Box<Spanned<Self>>,
    },

    /// Function call: `foo(a, b)`
    Call {
        /// The function being called.
        callee: Box<Spanned<Self>>,
        /// The arguments of the function call.
        args: Vec<Spanned<Self>>,
    },
    /// Method call: `x.foo(a)`
    MethodCall {
        /// The receiver of the method call.
        receiver: Box<Spanned<Self>>,
        /// The name/symbol of the method being called.
        method: Symbol,
        /// The arguments of the method call.
        args: Vec<Spanned<Self>>,
    },
    /// Field access: `point.x`
    FieldAccess {
        /// The expression being accessed.
        expr: Box<Spanned<Self>>,
        /// The name/symbol of the field being accessed.
        field: Symbol,
    },

    /// Struct literal: `Point { x: 1, y: 2 }`
    StructLit {
        /// The name of the struct being constructed. This is stored as a symbol handle in the interner hashmap.
        name: Symbol,
        /// The fields of the struct being constructed.
        fields: Vec<(Symbol, Spanned<Self>)>,
    },

    /// `if cond { ... } else { ... }`
    If {
        /// The condition of the if statement.
        condition: Box<Spanned<Self>>,
        /// The block of code executed if the condition is true.
        then_block: Block,
        /// The block of code executed if the condition is false.
        else_block: Option<Block>,
    },

    /// `match value { Pattern => expr, ... }` — exhaustiveness checked
    Match {
        /// The value being matched against.
        scrutinee: Box<Spanned<Self>>,
        /// The patterns and their corresponding expressions.
        arms: Vec<MatchArm>,
    },

    /// C-style loops: `while cond { ... }`
    While {
        /// The condition of the loop.
        condition: Box<Spanned<Self>>,
        /// The block of code executed while the condition is true.
        body: Block,
    },
    /// `loop { ... }`
    Loop {
        /// The block of code executed repeatedly.
        body: Block,
    },

    /// `break`
    Break,
    /// `continue`
    Continue,

    /// Assignment: `x = 5`
    Assign {
        /// The variable being assigned to.
        target: Box<Spanned<Self>>,
        /// The expression being assigned.
        value: Box<Spanned<Self>>,
    },

    /// Address-of: `&x`, `&mut x`
    AddrOf {
        /// Whether the address-of is mutable: `&mut x`
        mutable: bool,
        /// The expression being address-of'd.
        expr: Box<Spanned<Self>>,
    },

    /// Block expression: `{ stmts; final_expr }`
    Block(Block),

    /// Cast: `x as u32`
    Cast {
        /// The expression being casted.
        expr: Box<Spanned<Self>>,
        /// The type being casted to.
        ty: Spanned<Type>,
    },
}