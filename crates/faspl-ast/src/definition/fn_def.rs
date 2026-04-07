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

//! The declaration of a function in the FASPL language.

use faspl_common::interner::Symbol;
use crate::{Block, Spanned, Type, Param, GenericParam};

/// A definition of a function.
#[derive(Debug, Clone, PartialEq)]
pub struct FnDef {
    /// The function's name. This is stored as a symbol handle in the interner hashmap.
    name: Symbol,
    /// The function's generic parameters if it has any.
    generic_params: Vec<GenericParam>,
    /// The function's parameters.
    params: Vec<Param>,
    /// The function's return type/signature.
    return_ty: Option<Spanned<Type>>,
    /// The function's body.
    body: Block,
}

impl FnDef {
    /// Creates a new function definition from the given parameters.
    #[must_use]
    pub fn new(name: Symbol, generic_params: &[GenericParam], params: &[Param], return_ty: Option<Spanned<Type>>, body: Block) -> Self {
        Self { name, generic_params: generic_params.to_vec(), params: params.to_vec(), return_ty, body }
    }
    /// Returns the name of the function.
    #[must_use]
    pub const fn name(&self) -> &Symbol {
        &self.name
    }

    /// Returns the parameters of the function.
    #[must_use]
    pub const fn params(&self) -> &[Param] {
        self.params.as_slice()
    }

    /// Returns the return type of the function.
    #[must_use]
    pub const fn return_ty(&self) -> Option<&Spanned<Type>> {
        self.return_ty.as_ref()
    }

    /// Returns the body of the function.
    #[must_use]
    pub const fn body(&self) -> &Block {
        &self.body
    }
}