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

//! A generic parameter in the FASPL language.

use faspl_common::interner::Symbol;

/// A generic parameter in a type or trait definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericParam {
    /// The generic parameter's name. This is stored as a symbol handle in the interner hashmap.
    name: Symbol,
    /// Trait bounds: `T: Hash + Eq`
    bounds: Vec<Symbol>,
}

impl GenericParam {
    /// Creates a new generic parameter.
    #[must_use]
    pub const fn new(name: Symbol, bounds: Vec<Symbol>) -> Self {
        Self { name, bounds }
    }

    /// Returns the name of the generic parameter.
    #[must_use]
    pub const fn name(&self) -> &Symbol {
        &self.name
    }

    /// Returns the trait bounds of the generic parameter.
    #[must_use]
    pub fn bounds(&self) -> &[Symbol] {
        &self.bounds
    }
}