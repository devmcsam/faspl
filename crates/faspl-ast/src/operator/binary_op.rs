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

//! Built-in binary operators.

/// A binary operator. Binary here means it operates on two operands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    /// Addition: `a + b`
    Add,
    /// Subtraction: `a - b`
    Sub,
    /// Multiplication: `a * b`
    Mul,
    /// Division: `a / b`
    Div,
    /// Remainder: `a % b`
    Mod,
    /// Equality: `a == b`
    Eq,
    /// Inequality: `a != b`
    Ne,
    /// Less-than: `a < b`
    Lt,
    /// Greater-than: `a > b`
    Le,
    /// Less-than-or-equal: `a <= b`
    Gt,
    /// Greater-than-or-equal: `a >= b`
    Ge,
    /// Logical AND: `a && b`
    And,
    /// Logical OR: `a || b`
    Or,
    /// Bitwise AND: `a & b`
    BitAnd,
    /// Bitwise OR: `a | b`
    BitOr,
    /// Bitwise XOR: `a ^ b`
    BitXor,
    /// Left-shift: `a << b`
    Shl,
    /// Right-shift: `a >> b`
    Shr,
}