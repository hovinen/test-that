// Copyright 2022 Google LLC
// Copyright 2026 Bradford Hovinen <bradford@hovinen.me>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_std]
#![doc = include_str!("../crate_docs.md")]
#![cfg_attr(docsrs, feature(doc_cfg), doc(auto_cfg))]

#[macro_use]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

/// Re-export of `alloc` for use by this crate's macros.
/// Do not use directly; it is not part of the public API.
#[doc(hidden)]
pub extern crate alloc as __alloc;

#[cfg(feature = "test-that-macro")]
extern crate test_that_macro;

#[cfg(test)]
extern crate quickcheck;

#[macro_use]
pub mod assertions;
pub mod description;
pub mod internal;
pub mod matcher;
pub mod matcher_support;
pub mod matchers;
pub mod result;

#[cfg(feature = "googletest-compat")]
pub mod compat;

/// Re-exports of the symbols in this crate which are most likely to be used.
///
/// This includes:
///  * All assertion macros,
///  * Traits and type definitions normally used by tests, and
///  * All built-in matchers.
///
/// Typically, one imports everything in the prelude in one's test module:
///
/// ```
/// mod tests {
///     use test_that::prelude::*;
/// }
/// ```
pub mod prelude {
    pub use super::OrFailExt;
    pub use super::TestResult;
    pub use super::TestResultExt;
    #[cfg(feature = "googletest-compat")]
    #[allow(deprecated)]
    pub use super::compat::IntoTestResult;
    pub use super::matcher::Matcher;
    pub use super::matcher::MatcherExt;
    pub use super::matchers::containers::*;
    pub use super::matchers::*;
    #[cfg(feature = "std")]
    pub use super::verify_current_test_outcome;
    pub use super::{assert_that, fail, verify_pred, verify_that};
    #[cfg(feature = "non-fatal-assertions")]
    pub use super::{expect_pred, expect_that};
    #[cfg(feature = "googletest-compat")]
    #[allow(deprecated)]
    pub use crate::result::Result;
}

pub use result::{OrFailExt, TestResult, TestResultExt, verify_current_test_outcome};

#[cfg(feature = "googletest-compat")]
#[allow(deprecated)]
pub use result::Result;

#[cfg(feature = "non-fatal-assertions")]
pub use test_that_macro::test;
