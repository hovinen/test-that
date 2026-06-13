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

//! Matchers against types implementing [`IntoIterator`].
//!
//! All matchers in this package can be used with containers `C` such
//! that:
//!
//! - `&C: IntoIterator<Item = &T>`,
//! - `&C: IntoIterator<Item = T>`.
//!
//! The former includes standard Rust containers such as arrays `[T; N]`,
//! unsized arrays `[T]`, `Vec<T>`, `HashSet<T>`, `HashMap<K, V>`, and so on.
//! The latter is a special case which may appear in some custom containers.
//!
//! One can match against slices `&[T]` by first "dereferencing" with them
//! with the [`points_to`] matcher. For example:
//!
//! ```
//! # use test_that::prelude::*;
//! let value = vec![1, 2, 3];
//! assert_that!(value.as_slice(), points_to(contains(eq(1))));
//! ```
//!
//! The case that `C` implements `IntoIterator` but `&C` does not is not
//! supported in general.
//!
//! These matchers do not support matching directly against an [`Iterator`]. To
//! match against an iterator, use [`Iterator::collect`] to build a [`Vec`]
//! first.
//!
//! [`points_to`]: crate::matchers::points_to

pub(super) mod container_contains;
pub(super) mod container_eq_matcher;
pub(super) mod contains_matcher;
pub(super) mod each_matcher;
pub(super) mod empty_matcher;
pub(super) mod len_matcher;
pub(super) mod pointwise_matcher;
pub(super) mod subset_of_matcher;
pub(super) mod superset_of_matcher;

pub use container_contains::unordered_matcher::__internal::ContainerContainsUnorderedMatcher;
pub use container_eq_matcher::container_eq;
pub use contains_matcher::{ContainsMatcher, contains};
pub use each_matcher::each;
pub use empty_matcher::empty;
pub use len_matcher::len;
pub use subset_of_matcher::subset_of;
pub use superset_of_matcher::superset_of;

// Reexport and unmangle the macros.
#[doc(inline)]
pub use crate::{
    __contains_each as contains_each, __contains_exactly as contains_exactly,
    __is_contained_in as is_contained_in, __pointwise as pointwise,
};

#[cfg(feature = "googletest-compat")]
#[doc(inline)]
#[allow(deprecated)]
pub use crate::{
    __elements_are as elements_are, __unordered_elements_are as unordered_elements_are,
};

/// Marker for containers whose iterator over `&ContainerT` yields `&T` items.
///
/// **For internal use only. API stability is not guaranteed!**
#[doc(hidden)]
pub struct RefItems;

/// Marker for containers whose iterator over `&ContainerT` yields owned `T`
/// items.
///
/// **For internal use only. API stability is not guaranteed!**
#[doc(hidden)]
pub struct OwnedItems;
