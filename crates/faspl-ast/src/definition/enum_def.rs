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

//! The definition of an enum in the FASPL language.

use faspl_common::interner::Symbol;
use crate::{GenericParam, Variant};

/// An enum definition. This represents a user-defined data structure with variants.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDef {
    /// The enum's name. This is stored as a symbol handle in the interner hashmap.
    name: Symbol,
    /// The enum's generic parameters if it has any.
    generic_params: Vec<GenericParam>,
    /// The enum's variants.
    variants: Vec<Variant>,
}

impl EnumDef {
    /// Creates a new enum definition.
    #[must_use]
    pub const fn new(name: Symbol, generic_params: Vec<GenericParam>, variants: Vec<Variant>) -> Self {
        Self {
            name,
            generic_params,
            variants,
        }
    }

    /// Returns the name of the enum.
    #[must_use]
    pub const fn name(&self) -> &Symbol {
        &self.name
    }

    /// Returns the generic parameters of the enum.
    #[must_use]
    pub fn generic_params(&self) -> &[GenericParam] {
        &self.generic_params
    }

    /// Returns the variants of the enum.
    #[must_use]
    pub fn variants(&self) -> &[Variant] {
        &self.variants
    }
}