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

//! All built-in matchers of this crate are in submodules of this module.

mod all_matcher;
mod any_matcher;
mod anything_matcher;
mod char_count_matcher;
mod conjunction_matcher;
pub mod containers;
#[cfg(feature = "regex")]
mod contains_regex_matcher;
mod disjunction_matcher;
mod display_matcher;
mod eq_deref_of_matcher;
mod eq_matcher;
mod err_matcher;
mod field_matcher;
mod ge_matcher;
mod gt_matcher;
mod has_entry_matcher;
mod is_encoded_string_matcher;
mod is_matcher;
mod is_nan_matcher;
mod le_matcher;
mod lt_matcher;
mod matches_pattern;
#[cfg(feature = "regex")]
mod matches_regex_matcher;
mod near_matcher;
mod none_matcher;
mod not_matcher;
mod ok_matcher;
mod points_to_matcher;
mod predicate_matcher;
mod result_of_matcher;
mod some_matcher;
mod str_matcher;
mod tuple_matcher;

pub use anything_matcher::anything;
pub use char_count_matcher::char_count;
#[cfg(feature = "regex")]
pub use contains_regex_matcher::contains_regex;
pub use display_matcher::displays_as;
pub use eq_deref_of_matcher::eq_deref_of;
pub use eq_matcher::eq;
pub use err_matcher::err;
pub use ge_matcher::ge;
pub use gt_matcher::gt;
pub use has_entry_matcher::has_entry;
pub use is_encoded_string_matcher::is_utf8_string;
pub use is_nan_matcher::is_nan;
pub use le_matcher::le;
pub use lt_matcher::lt;
#[cfg(feature = "regex")]
pub use matches_regex_matcher::matches_regex;
pub use near_matcher::{NearMatcher, approx_eq, near};
pub use none_matcher::none;
pub use not_matcher::not;
pub use ok_matcher::ok;
pub use points_to_matcher::points_to;
pub use predicate_matcher::{PredicateMatcher, predicate};
pub use some_matcher::some;
pub use str_matcher::{
    StrMatcher, StrMatcherConfigurator, contains_substring, ends_with, starts_with,
};

// Reexport and unmangle the macros.
#[doc(inline)]
pub use crate::{
    __all as all, __any as any, __field as field, __matches_pattern as matches_pattern,
    __pat as pat, __result_of as result_of,
};

// Types and functions used by macros matchers.
// Do not use directly.
// We may perform incompatible changes without major release. These elements
// should only be used through their respective macros.
#[doc(hidden)]
pub mod __internal {
    pub use super::all_matcher::__internal::AllMatcher;
    pub use super::any_matcher::__internal::AnyMatcher;
    pub use super::conjunction_matcher::__internal::ConjunctionMatcher;
    pub use super::containers::container_contains::Requirements;
    pub use super::containers::container_contains::ordered_matcher::__internal::ContainerContainsOrderedMatcher;
    pub use super::containers::container_contains::unordered_matcher::__internal::{
        ContainerContainsUnorderedMatcher, MapContainsMatcher,
    };
    pub use super::containers::pointwise_matcher::__internal::PointwiseMatcher;
    pub use super::disjunction_matcher::__internal::DisjunctionMatcher;
    pub use super::field_matcher::__internal::field_matcher;
    pub use super::is_matcher::is;
    pub use super::result_of_matcher::__internal::result_of;
}
