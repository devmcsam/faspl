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

//! The definition of a trait in the FASPL language.

use faspl_common::interner::Symbol;
use crate::{GenericParam, Spanned, TraitMethod};

/// A trait definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TraitDef {
    /// The trait's name. This is stored as a symbol handle in the interner hashmap.
    name: Symbol,
    /// The trait's generic parameters if it has any.
    generic_params: Vec<GenericParam>,
    /// Method signatures (body is optional for default impls)
    methods: Vec<Spanned<TraitMethod>>,
}

impl TraitDef {
    /// Creates a new trait definition.
    #[must_use]
    pub const fn new(
        name: Symbol,
        generic_params: Vec<GenericParam>,
        methods: Vec<Spanned<TraitMethod>>,
    ) -> Self {
        Self {
            name,
            generic_params,
            methods,
        }
    }

    /// Returns the name of the trait.
    #[must_use]
    pub const fn name(&self) -> &Symbol {
        &self.name
    }

    /// Returns the generic parameters of the trait.
    #[must_use]
    pub fn generic_params(&self) -> &[GenericParam] {
        &self.generic_params
    }

    /// Returns the methods defined in the trait.
    #[must_use]
    pub fn methods(&self) -> &[Spanned<TraitMethod>] {
        &self.methods
    }
}