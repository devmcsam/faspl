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

//! The definition of a struct in the FASPL language.

use faspl_common::interner::Symbol;
use crate::{Field, GenericParam};

/// A struct definition. This represents a user-defined data structure with named fields.
#[derive(Debug, Clone, PartialEq)]
pub struct StructDef {
    /// The struct's name. This is stored as a symbol handle in the interner hashmap.
    name: Symbol,
    /// The struct's generic parameters if it has any.
    generic_params: Vec<GenericParam>,
    /// The struct's fields.
    fields: Vec<Field>,
}

impl StructDef {
    /// Creates a new struct definition from the given parameters.
    #[must_use]
    pub fn new(name: Symbol, generic_params: &[GenericParam], fields: &[Field]) -> Self {
        Self { name, generic_params: generic_params.to_vec(), fields: fields.to_vec() }
    }

    /// Returns the name of the struct. This is a symbol handle in the interner hashmap.
    #[must_use]
    pub const fn name(&self) -> &Symbol {
        &self.name
    }

    /// returns the generic parameters of the struct, if any.
    #[must_use]
    pub const fn generic_params(&self) -> &[GenericParam] {
        self.generic_params.as_slice()
    }

    /// returns the fields of the struct.
    #[must_use]
    pub const fn fields(&self) -> &[Field] {
        self.fields.as_slice()
    }
}