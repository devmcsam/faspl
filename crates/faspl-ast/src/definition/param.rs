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

//! The definition of a parameter in the FASPL language.

use faspl_common::interner::Symbol;
use crate::{Spanned, Type};

/// A parameter in a function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    /// The parameter's name. This is stored as a symbol handle in the interner hashmap.
    name: Symbol,
    /// The parameter's type/signature.
    ty: Spanned<Type>,
}

impl Param {
    /// Creates a new parameter from the given name and type.
    #[must_use]
    pub const fn new(name: Symbol, ty: Spanned<Type>) -> Self {
        Self { name, ty }
    }

    /// Returns the name of the parameter. This is a symbol handle in the interner hashmap.
    #[must_use]
    pub const fn name(&self) -> &Symbol {
        &self.name
    }
    /// Returns the type of the parameter.
    #[must_use]
    pub const fn ty(&self) -> &Spanned<Type> {
        &self.ty
    }
}