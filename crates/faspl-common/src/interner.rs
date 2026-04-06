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

//! The interner for user functions and variables for the faspl language.

use rustc_hash::FxHashMap;

/// A handle to an interned string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Symbol(u32);

/// The interner for user functions and variables.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Interner {
    /// The map that stores the interned symbols.
    map: FxHashMap<String, Symbol>,
    /// The vector that stores the interned strings.
    strings: Vec<String>,
}

impl Interner {
    /// Creates a new interner.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Intern a string and receive a `Symbol` handle to it.
    #[must_use]
    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(symbol) = self.map.get(string) {
            return *symbol;
        }

        let symbol = Symbol(self.strings.len() as u32);
        let owned = string.to_owned();
        self.strings.push(owned.clone());
        self.map.insert(owned, symbol);
        symbol
    }

    /// Resolve a `Symbol` back to its string.
    #[must_use]
    pub fn resolve(&self, sym: Symbol) -> &str {
        &self.strings[sym.0 as usize]
    }
}