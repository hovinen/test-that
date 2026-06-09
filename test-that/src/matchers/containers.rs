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

//! All built-in matchers of this crate are in submodules of this module.

pub(super) mod container_contains;
pub(super) mod container_contains_ordered_matcher;
pub(super) mod container_contains_unordered_matcher;
pub(super) mod container_eq_matcher;
pub(super) mod contains_matcher;
pub(super) mod each_matcher;
pub(super) mod empty_matcher;
pub(super) mod len_matcher;
pub(super) mod pointwise_matcher;
pub(super) mod subset_of_matcher;
pub(super) mod superset_of_matcher;

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
