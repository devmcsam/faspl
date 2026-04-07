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

//! Literal values in the AST.

use core::ops::Deref;
use faspl_common::interner::Symbol;

/// An optional suffix the user wrote on a numeric literal: `42u32`, `3.14f64`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntSuffix {
    /// Signed 8 bit integer
    I8,
    /// Signed 16 bit integer
    I16,
    /// Signed 32 bit integer
    I32,
    /// Signed 64 bit integer
    I64,
    /// Signed 128 bit integer
    I128,
    /// Unsigned 8 bit integer
    U8,
    /// Unsigned 16 bit integer
    U16,
    /// Unsigned 32 bit integer
    U32,
    /// Unsigned 64 bit integer
    U64,
    /// Unsigned 128 bit integer
    U128,
}

/// An optional suffix the user wrote on a floating point literal: `3.14f32`, `1.0f64`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatSuffix {
    /// A 32 bit floating point number
    F32,
    /// A 64 bit floating point number
    F64,
}

/// A fully‐resolved literal value in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Integer literal, possibly suffixed: `42`, `42u32`, `0xFF_i16`
    ///
    /// Stored as `i128` so it can hold any integer width.
    /// The suffix (if any) records what the user explicitly requested.
    /// If `suffix` is `None`, the type is inferred later during type‐checking.
    Int {
        /// The raw value, stored wide enough for any integer type.
        value: i128,
        /// The optional user‐written suffix.
        suffix: Option<IntSuffix>,
    },

    /// Floating‐point literal, possibly suffixed: `3.14`, `1.0f32`
    Float {
        /// Stored as `f64` — wide enough for `f32` values too.
        value: f64,
        /// The optional user‐written suffix.
        suffix: Option<FloatSuffix>,
    },

    /// Boolean literal: `true`, `false`
    Bool(bool),

    /// Character literal: `'a'`, `'\n'`
    Char(char),

    /// String literal: `"hello world"`
    String(Symbol),
}

/// A signed 8 bit integer.
pub struct Int8(i8);

impl Deref for Int8 {
    type Target = i8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A signed 16 bit integer.
pub struct Int16(i16);

impl Deref for Int16 {
    type Target = i16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A signed 32 bit integer.
pub struct Int32(i32);

impl Deref for Int32 {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A signed 64 bit integer.
pub struct Int64(i64);

impl Deref for Int64 {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A signed 128 bit integer.
pub struct Int128(i128);

impl Deref for Int128 {
    type Target = i128;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An unsigned 8 bit integer.
pub struct Uint8(u8);

impl Deref for Uint8 {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An unsigned 16 bit integer.
pub struct Uint16(u16);
impl Deref for Uint16 {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An unsigned 32 bit integer.
pub struct Uint32(u32);

impl Deref for Uint32 {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An unsigned 64 bit integer.
pub struct Uint64(u64);

impl Deref for Uint64 {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An unsigned 128 bit integer.
pub struct Uint128(u128);

impl Deref for Uint128 {
    type Target = u128;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A 32 bit floating point number.
pub struct Float32(f32);

impl Deref for Float32 {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A 64 bit floating point number.
pub struct Float64(f64);

impl Deref for Float64 {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A boolean value.
pub struct Bool(bool);

impl Deref for Bool {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A character value.
pub struct Char(char);

impl Deref for Char {
    type Target = char;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A string value.
pub struct Str(Symbol);
impl Deref for Str {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}