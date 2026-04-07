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

//! The definition of a variant in the FASPL language.

use faspl_common::interner::Symbol;
use crate::{Spanned, Type};

/// A variant in an enum definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    /// The variant's name. This is stored as a symbol handle in the interner hashmap.
    name: Symbol,
    /// Variants can carry data: `Some(T)` or be unit-like: `None`
    fields: Vec<Spanned<Type>>,
}

impl Variant {
    /// Creates a new enum variant.
    #[must_use]
    pub const fn new(name: Symbol, fields: Vec<Spanned<Type>>) -> Self {
        Self { name, fields }
    }

    /// Returns the name of the variant.
    #[must_use]
    pub const fn name(&self) -> &Symbol {
        &self.name
    }

    /// Returns the fields of the variant.
    #[must_use]
    pub fn fields(&self) -> &[Spanned<Type>] {
        &self.fields
    }
}