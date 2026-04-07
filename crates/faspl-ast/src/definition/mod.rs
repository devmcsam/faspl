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

//! The definitions of types, functions, traits, etc. in the FASPL lanuage AST.
pub mod fn_def;
pub mod param;
pub mod struct_def;
pub mod field;
pub mod enum_def;
pub mod variant;
pub mod impl_block;
pub mod trait_def;
pub mod trait_method;
pub mod generic_param;
pub use fn_def::*;
pub use param::*;
pub use struct_def::*;
pub use field::*;
pub use enum_def::*;
pub use variant::*;
pub use impl_block::*;
pub use trait_def::*;
pub use trait_method::*;
pub use generic_param::*;