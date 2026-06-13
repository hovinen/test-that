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

#![doc(hidden)]

pub(crate) mod ordered_matcher;
pub(crate) mod unordered_matcher;

use crate::{description::Description, matcher_support::count_elements::count_elements};
use std::fmt::Display;

/// Matches a container whose elements in any order have a 1:1 correspondence
/// with the provided element matchers.
///
/// By default, the elements and matchers can be in any order.
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(vec![3, 2, 1], contains_exactly![eq(1), ge(2), anything()])?;   // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> TestResult<()> {
/// verify_that!(vec![1], contains_exactly![eq(1), ge(2)])?;              // Fails: container has wrong size
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> TestResult<()> {
/// verify_that!(vec![3, 2, 1], contains_exactly![eq(1), ge(4), eq(2)])?; // Fails: second matcher not matched
/// #     Ok(())
/// # }
/// # fn should_fail_3() -> TestResult<()> {
/// verify_that!(vec![3, 2, 1], contains_exactly![ge(3), ge(3), ge(3)])?; // Fails: no 1:1 correspondence
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_fail_3().unwrap_err();
/// ```
///
/// See [module documentation][crate::matchers::containers] for information about
/// what types this matcher can match.
///
/// This can also match against [`HashMap`][std::collections::HashMap] and
/// similar collections. The arguments are a sequence of mappings of matchers
/// corresponding to the keys and their respective values.
///
/// ```
/// # use test_that::prelude::*;
/// # use std::collections::HashMap;
/// let value: HashMap<u32, &'static str> = [(1, "One"), (2, "Two"), (3, "Three")].into();
/// verify_that!(
///     value,
///     contains_exactly![(eq(2) => eq("Two")), (eq(1) => eq("One")), (eq(3) => eq("Three"))]
/// )
/// #     .unwrap();
/// ```
///
/// As a shorthand, one can use set notation when the matcher is to be used
/// directly in [`verify_that!`] and related macros:
///
/// ```
/// # use test_that::prelude::*;
///  verify_that!(vec![1, 2], {eq(2), eq(1)})
/// #     .unwrap();
/// ```
///
/// **Note:** This only works as a top-level matcher in [`verify_that!`] and
/// related macros. When nested inside other matchers, it is still necessary to
/// use the [`contains_exactly!`][crate::matchers::containers::contains_exactly]
/// macro.
///
/// ```compile_fail
/// # use test_that::prelude::*;
/// verify_that!(vec![vec![1,2], vec![3]], {{eq(2), eq(1)}, {eq(3)}})
/// # .unwrap();
/// ```
///
/// This matcher does not support matching directly against an [`Iterator`]. To
/// match against an iterator, use [`Iterator::collect`] to build a [`Vec`].
///
/// The matcher proceeds in three stages:
///
/// 1. It first checks whether the actual value is of the right size to possibly
///    be matched by each of the given matchers. If not, then it immediately
///    fails explaining that the size is incorrect.
///
/// 2. It then checks whether each matcher matches at least one corresponding
///    element in the actual container and each element in the actual container
///    is matched by at least one matcher. If not, it fails with a message
///    indicating which matcher respectively container elements had no
///    counterparts.
///
/// 3. Finally, it checks whether the mapping of matchers to corresponding
///    actual elements is a 1-1 correspondence and fails if that is not the
///    case. The failure message then shows the best matching it could find,
///    including which matchers did not have corresponding unique elements in
///    the container and which container elements had no corresponding matchers.
///
/// ## Enforcing the order of elements
///
/// To enforce that the elements appear in the same order as the matchers, use
/// [`in_order`][crate::matchers::containers::ContainerContainsUnorderedMatcher::in_order]:
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(vec![1, 2, 3], contains_exactly![eq(1), ge(2), anything()].in_order())?;   // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> TestResult<()> {
/// verify_that!(vec![3, 2, 1], contains_exactly![eq(1), ge(2), anything()].in_order())?;   // Fails: wrong order
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// As a shorthand, one can use array notation when the matcher is to be used
/// directly in [`verify_that!`] and related macros:
///
/// ```
/// # use test_that::prelude::*;
///  verify_that!(vec![1, 2], [eq(1), eq(2)])
/// #     .unwrap();
/// ```
///
/// The same caveat applies as with the set notation above.
///
/// [`IntoIterator`]: std::iter::IntoIterator
/// [`Iterator`]: std::iter::Iterator
/// [`Iterator::collect`]: std::iter::Iterator::collect
/// [`Vec`]: std::vec::Vec
#[macro_export]
#[doc(hidden)]
macro_rules! __contains_exactly {
    ($(,)?) => {{
        $crate::matchers::containers::ContainerContainsUnorderedMatcher::new(
            [],
            $crate::matchers::__internal::Requirements::PerfectMatch
        )
    }};

    ($(($key_matcher:expr => $value_matcher:expr)),* $(,)?) => {{
        $crate::matchers::__internal::MapContainsMatcher::new(
            [$((Box::new($key_matcher), Box::new($value_matcher))),*],
            $crate::matchers::__internal::Requirements::PerfectMatch
        )
    }};

    ($($matcher:expr),* $(,)?) => {{
        $crate::matchers::__internal::ContainerContainsUnorderedMatcher::new(
            [$(Box::new($matcher)),*],
            $crate::matchers::__internal::Requirements::PerfectMatch
        )
    }};
}

/// Matches a container containing elements matched by the given matchers.
///
/// To match, each given matcher must have a corresponding element in the
/// container which it matches. There must be a mapping uniquely matching each
/// matcher to a container element. The container can, however, contain
/// additional elements that don't correspond to any matcher.
///
/// Put another way, `contains_each!` matches if there is a subset of the actual
/// container which
/// [`contains_exactly`][crate::matchers::containers::contains_exactly] would
/// match.
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(vec![3, 2, 1], contains_each![eq(2), ge(3)])?;   // Passes
/// verify_that!(vec![3, 2, 1], contains_each![ge(2), ge(2)])?;   // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> TestResult<()> {
/// verify_that!(vec![1], contains_each![eq(1), ge(2)])?;         // Fails: container too small
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> TestResult<()> {
/// verify_that!(vec![3, 2, 1], contains_each![eq(1), ge(4)])?;   // Fails: second matcher unmatched
/// #     Ok(())
/// # }
/// # fn should_fail_3() -> TestResult<()> {
/// verify_that!(vec![3, 2, 1], contains_each![ge(3), ge(3), ge(3)])?; // Fails: no matching
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_fail_3().unwrap_err();
/// ```
///
/// See [module documentation][crate::matchers::containers] for information about
/// what types this matcher can match.
///
/// This can also match against [`HashMap`][std::collections::HashMap] and
/// similar collections. The arguments are a sequence of mappings of matchers
/// corresponding to the keys and their respective values.
///
/// ```
/// # use test_that::prelude::*;
/// # use std::collections::HashMap;
/// let value: HashMap<u32, &'static str> = [(1, "One"), (2, "Two"), (3, "Three")].into();
/// verify_that!(value, contains_each![(eq(2) => eq("Two")), (eq(1) => eq("One"))])
/// #     .unwrap();
/// ```
///
/// This matcher does not support matching directly against an [`Iterator`]. To
/// match against an iterator, use [`Iterator::collect`] to build a [`Vec`].
///
/// The matcher proceeds in three stages:
///
/// 1. It first checks whether the actual value is large enough to possibly be
///    matched by each of the given matchers. If not, then it immediately fails
///    explaining that the size is too small.
///
/// 2. It then checks whether each matcher matches at least one corresponding
///    element in the actual container and fails if that is not the case. The
///    failure message indicates which matcher had no corresponding element.
///
/// 3. Finally, it checks whether the mapping of matchers to corresponding
///    actual elements is 1-1 and fails if that is not the case. The failure
///    message then shows the best matching it could find, including which
///    matchers did not have corresponding unique elements in the container.
///
/// ## Enforcing the order of elements
///
/// To enforce that the elements appear in the same order as the matchers, use
/// [`in_order`][crate::matchers::containers::ContainerContainsUnorderedMatcher::in_order]:
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(vec![1, 2, 3], contains_each![eq(1), anything()].in_order())?;   // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> TestResult<()> {
/// verify_that!(vec![3, 2, 1], contains_each![eq(1), anything()].in_order())?;   // Fails: wrong order
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// [`IntoIterator`]: std::iter::IntoIterator
/// [`Iterator`]: std::iter::Iterator
/// [`Iterator::collect`]: std::iter::Iterator::collect
/// [`Vec`]: std::vec::Vec
#[macro_export]
#[doc(hidden)]
macro_rules! __contains_each {
    ($(,)?) => {{
        $crate::matchers::__internal::ContainerContainsUnorderedMatcher::new(
            [],
            $crate::matchers::__internal::Requirements::Superset
        )
    }};

    ($(($key_matcher:expr => $value_matcher:expr)),* $(,)?) => {{
        $crate::matchers::__internal::MapContainsMatcher::new(
            [$((Box::new($key_matcher), Box::new($value_matcher))),*],
            $crate::matchers::__internal::Requirements::Superset
        )
    }};

    ($($matcher:expr),* $(,)?) => {{
        $crate::matchers::__internal::ContainerContainsUnorderedMatcher::new(
            [$(Box::new($matcher)),*],
            $crate::matchers::__internal::Requirements::Superset
        )
    }}
}

/// Matches a container all of whose elements are matched by the given matchers.
///
/// To match, each element in the container must have a corresponding matcher
/// which matches it. There must be a 1-1 mapping from container elements to
/// matchers, so that no matcher has more than one corresponding element.
///
/// There may, however, be matchers not corresponding to any elements in the
/// container.
///
/// Put another way, `is_contained_in!` matches if there is a subset of the
/// matchers which would match with
/// [`contains_exactly`][crate::matchers::containers::contains_exactly].
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(vec![2, 1], is_contained_in![eq(1), ge(2)])?;   // Passes
/// verify_that!(vec![2, 1], is_contained_in![ge(1), ge(1)])?;   // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> TestResult<()> {
/// verify_that!(vec![1, 2, 3], is_contained_in![eq(1), ge(2)])?; // Fails: container too large
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> TestResult<()> {
/// verify_that!(vec![2, 1], is_contained_in![eq(1), ge(4)])?;    // Fails: second matcher unmatched
/// #     Ok(())
/// # }
/// # fn should_fail_3() -> TestResult<()> {
/// verify_that!(vec![3, 1], is_contained_in![ge(3), ge(3), ge(3)])?; // Fails: no matching
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// # should_fail_3().unwrap_err();
/// ```
///
/// See [module documentation][crate::matchers::containers] for information about
/// what types this matcher can match.
///
/// This can also match against [`HashMap`][std::collections::HashMap] and
/// similar collections. The arguments are a sequence of mappings of matchers
/// corresponding to the keys and their respective values.
///
/// ```
/// # use test_that::prelude::*;
/// # use std::collections::HashMap;
/// let value: HashMap<u32, &'static str> = [(1, "One"), (2, "Two")].into();
/// verify_that!(
///     value,
///     is_contained_in![(eq(2) => eq("Two")), (eq(1) => eq("One")), (eq(3) => eq("Three"))]
/// )
/// #     .unwrap();
/// ```
///
/// This matcher does not support matching directly against an [`Iterator`]. To
/// match against an iterator, use [`Iterator::collect`] to build a [`Vec`].
///
/// The matcher proceeds in three stages:
///
/// 1. It first checks whether the actual value is too large to possibly be
///    matched by each of the given matchers. If so, it immediately fails
///    explaining that the size is too large.
///
/// 2. It then checks whether each actual container element is matched by at
///    least one matcher and fails if that is not the case. The failure message
///    indicates which element had no corresponding matcher.
///
/// 3. Finally, it checks whether the mapping of elements to corresponding
///    matchers is 1-1 and fails if that is not the case. The failure message
///    then shows the best matching it could find, including which container
///    elements did not have corresponding matchers.
///
/// ## Enforcing the order of elements
///
/// To enforce that the elements appear in the same order as the matchers, use
/// [`in_order`][crate::matchers::containers::ContainerContainsUnorderedMatcher::in_order]:
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(vec![1, 2, 3], is_contained_in![eq(0), eq(1), ge(2), anything(), eq(4)].in_order())?;   // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> TestResult<()> {
/// verify_that!(vec![3, 2, 1], is_contained_in![eq(0), eq(1), ge(2), anything(), eq(4)].in_order())?;   // Fails: wrong order
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// [`IntoIterator`]: std::iter::IntoIterator
/// [`Iterator`]: std::iter::Iterator
/// [`Iterator::collect`]: std::iter::Iterator::collect
/// [`Vec`]: std::vec::Vec
#[macro_export]
#[doc(hidden)]
macro_rules! __is_contained_in {
    ($(,)?) => {{
        use $crate::matchers::__internal::{
            ContainerContainsUnorderedMatcher, Requirements
        };
        ContainerContainsUnorderedMatcher::new([], Requirements::Subset)
    }};

    ($(($key_matcher:expr => $value_matcher:expr)),* $(,)?) => {{
        use $crate::matchers::__internal::{
            MapContainsMatcher, Requirements
        };
        MapContainsMatcher::new(
            [$((Box::new($key_matcher), Box::new($value_matcher))),*],
            Requirements::Subset
        )
    }};

    ($($matcher:expr),* $(,)?) => {{
        use $crate::matchers::__internal::{
            ContainerContainsUnorderedMatcher, Requirements
        };
        ContainerContainsUnorderedMatcher::new([$(Box::new($matcher)),*], Requirements::Subset)
    }}
}

/// Abstracts over map iterator items, allowing both reference pairs `(&K, &V)`
/// and owned pairs `(K, V)` to be matched uniformly.
///
/// **For internal use only. API stability is not guaranteed!**
#[doc(hidden)]
pub trait PairBorrow<K, V> {
    fn borrow_key(&self) -> &K;
    fn borrow_value(&self) -> &V;
}

impl<K, V> PairBorrow<K, V> for (&K, &V) {
    fn borrow_key(&self) -> &K {
        self.0
    }
    fn borrow_value(&self) -> &V {
        self.1
    }
}

impl<K, V> PairBorrow<K, V> for (K, V) {
    fn borrow_key(&self) -> &K {
        &self.0
    }
    fn borrow_value(&self) -> &V {
        &self.1
    }
}

/// The requirements of the mapping between matchers and actual values by
/// which [`UnorderedElemetnsAre`] is deemed to match its input.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum Requirements {
    /// There must be a 1:1 correspondence between the actual values and the
    /// matchers.
    PerfectMatch,

    /// The mapping from matched actual values to their corresponding
    /// matchers must be surjective.
    Superset,

    /// The mapping from matchers to matched actual values must be
    /// surjective.
    Subset,
}

impl Requirements {
    pub(crate) fn explain_size_mismatch<ContainerT: ?Sized>(
        &self,
        actual: &ContainerT,
        expected_size: usize,
    ) -> Option<Description>
    where
        for<'b> &'b ContainerT: IntoIterator,
    {
        let actual_size = count_elements(actual);
        match self {
            Requirements::PerfectMatch if actual_size != expected_size => {
                Some(format!("which has size {} (expected {})", actual_size, expected_size).into())
            }

            Requirements::Superset if actual_size < expected_size => Some(
                format!("which has size {} (expected at least {})", actual_size, expected_size)
                    .into(),
            ),

            Requirements::Subset if actual_size > expected_size => Some(
                format!("which has size {} (expected at most {})", actual_size, expected_size)
                    .into(),
            ),

            _ => None,
        }
    }
}

impl Display for Requirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Requirements::PerfectMatch => {
                write!(f, "perfect")
            }
            Requirements::Superset => {
                write!(f, "superset")
            }
            Requirements::Subset => {
                write!(f, "subset")
            }
        }
    }
}
