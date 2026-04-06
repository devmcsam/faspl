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

//! The span type for the faspl language compiler

/// A byte offset span in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// The start byte offset.
    start: u32,
    /// The end byte offset.
    end: u32,
}

impl Span {
    /// Creates a new span with the given start and end byte offsets.
    #[must_use]
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// Returns the start byte offset of the span.
    #[must_use]
    pub const fn start(self) -> u32 {
        self.start
    }

    /// Returns the end byte offset of the span.
    #[must_use]
    pub const fn end(self) -> u32 {
        self.end
    }

    /// Returns the length of the span in bytes.
    #[must_use]
    pub const fn length(self) -> u32 {
        self.end - self.start
    }

    /// Returns true if the span is zero-length.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        // unsigned integers cannot be negative, and thus addition can't offset and result in 0 like with signed integers.
        self.start + self.end == 0
    }

    /// Returns true if the other `Span` is greater than or equal to the start
    /// and less than or equal to the end of this `Span`.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    /// Sets the start byte offset of the span.
    pub const fn set_start(&mut self, start: u32) {
        self.start = start;
    }

    /// Sets the end byte offset of the span.
    pub const fn set_end(&mut self, end: u32) {
        self.end = end;
    }
}