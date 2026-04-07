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

//! An impl block in the FASPL language.

use faspl_common::interner::Symbol;
use crate::{GenericParam, Spanned, Type, FnDef};

/// A block of code that implements a trait or functions for a type.
#[derive(Debug, Clone, PartialEq)]
pub struct ImplBlock {
    /// The generic parameters of the impl block, if any.
    generic_params: Vec<GenericParam>,
    /// The type being impl'd, e.g. `Point` or `Foo<T>`
    target: Spanned<Type>,
    /// Optional trait being implemented: `impl Drawable for Point`
    trait_name: Option<Symbol>,
    /// The methods being implemented.
    methods: Vec<Spanned<FnDef>>,
}

impl ImplBlock {
    /// Creates a new impl block.
    #[must_use]
    pub const fn new(
        generic_params: Vec<GenericParam>,
        target: Spanned<Type>,
        trait_name: Option<Symbol>,
        methods: Vec<Spanned<FnDef>>,
    ) -> Self {
        Self {
            generic_params,
            target,
            trait_name,
            methods,
        }
    }

    /// Returns the generic parameters of the impl block.
    #[must_use]
    pub fn generic_params(&self) -> &[GenericParam] {
        &self.generic_params
    }

    /// Returns the target type of the impl block.
    #[must_use]
    pub const fn target(&self) -> &Spanned<Type> {
        &self.target
    }

    /// Returns the trait name being implemented, if any.
    #[must_use]
    pub const fn trait_name(&self) -> Option<&Symbol> {
        self.trait_name.as_ref()
    }

    /// Returns the methods in the impl block.
    #[must_use]
    pub fn methods(&self) -> &[Spanned<FnDef>] {
        &self.methods
    }
}