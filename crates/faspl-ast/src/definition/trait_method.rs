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

//! The definition of a trait method in the FASPL language.

use faspl_common::interner::Symbol;
use crate::{Block, Spanned, Type, Param};

/// A method signature in a trait definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TraitMethod {
    /// The method's name. This is stored as a symbol handle in the interner hashmap.
    name: Symbol,
    /// The method's parameters.
    params: Vec<Param>,
    /// The method's return type/signature.
    return_ty: Option<Spanned<Type>>,
    /// `None` = required, `Some` = default implementation
    default_body: Option<Block>,
}

impl TraitMethod {
    /// Creates a new trait method signature.
    #[must_use]
    pub const fn new(
        name: Symbol,
        params: Vec<Param>,
        return_ty: Option<Spanned<Type>>,
        default_body: Option<Block>,
    ) -> Self {
        Self {
            name,
            params,
            return_ty,
            default_body,
        }
    }

    /// Returns the name of the method.
    #[must_use]
    pub const fn name(&self) -> &Symbol {
        &self.name
    }

    /// Returns the parameters of the method.
    #[must_use]
    pub fn params(&self) -> &[Param] {
        &self.params
    }

    /// Returns the return type of the method.
    #[must_use]
    pub const fn return_ty(&self) -> Option<&Spanned<Type>> {
        self.return_ty.as_ref()
    }

    /// Returns the default body of the method, if it has one.
    #[must_use]
    pub const fn default_body(&self) -> Option<&Block> {
        self.default_body.as_ref()
    }
}