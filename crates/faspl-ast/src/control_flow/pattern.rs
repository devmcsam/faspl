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

//! A pattern to match to in a `match` expression.

use faspl_common::interner::Symbol;
use crate::Spanned;

/// A pattern to match to in a `match` expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Wildcard: `_`
    Wildcard,
    /// Bind a name: `x`
    Ident(Symbol),
    /// Literal: `42`
    IntLit(i64),
    /// Literal: `true`
    BoolLit(bool),
    /// Variant destructure: `Some(x)`, `None`
    Variant {
        /// The name of the variant. This is stored as a symbol handle in the interner hashmap.
        name: Symbol,
        /// The fields of the variant, if any.
        fields: Vec<Spanned<Self>>,
    },
    /// Struct destructure: `Point { x, y }`
    Struct {
        /// The name of the struct. This is stored as a symbol handle in the interner hashmap.
        name: Symbol,
        /// The fields of the struct, if any.
        fields: Vec<(Symbol, Spanned<Self>)>,
    },
}