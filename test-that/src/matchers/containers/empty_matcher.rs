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

use crate::{
    description::Description,
    matcher::{Describable, Matcher, MatcherResult},
};
use std::fmt::Debug;

/// Matches an empty container.
///
/// `T` can be any container such that `&T` implements `IntoIterator`. For
/// instance, `T` can be a common container like `Vec` and
/// [`HashSet`][std::collections::HashSet].
///
/// ```
/// # use test_that::prelude::*;
/// # use std::collections::HashSet;
/// # fn should_pass() -> TestResult<()> {
/// let value: Vec<i32> = vec![];
/// verify_that!(value, empty())?;
/// let value: HashSet<i32> = HashSet::new();
/// verify_that!(value, empty())?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// One can also check whether a slice is empty by dereferencing it:
///
/// ```
/// # use test_that::prelude::*;
/// # use std::collections::HashSet;
/// # fn should_pass() -> TestResult<()> {
/// let value: &[u32] = &[];
/// verify_that!(*value, empty())?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```

pub fn empty() -> EmptyMatcher {
    EmptyMatcher
}

#[doc(hidden)]
pub struct EmptyMatcher;

impl<T: Debug + ?Sized> Matcher<T> for EmptyMatcher
where
    for<'a> &'a T: IntoIterator,
{
    fn matches(&self, actual: &T) -> MatcherResult {
        actual.into_iter().next().is_none().into()
    }
}

impl Describable for EmptyMatcher {
    fn describe(&self, matcher_result: MatcherResult) -> Description {
        if matcher_result.into() { "is empty" } else { "isn't empty" }.into()
    }
}

#[cfg(test)]
mod tests {
    use super::empty;
    use crate::prelude::*;
    use std::collections::HashSet;

    #[test]
    fn empty_matcher_matches_empty_vec() -> TestResult<()> {
        let value: Vec<i32> = vec![];
        verify_that!(value, empty())
    }

    #[test]
    fn empty_matcher_matches_empty_array() -> TestResult<()> {
        verify_that!([] as [u32; 0], empty())
    }

    #[test]
    fn empty_matcher_matches_empty_ref_to_array_with_points_to() -> TestResult<()> {
        verify_that!(&([] as [u32; 0]), points_to(empty()))
    }

    #[test]
    fn empty_matcher_matches_empty_ref_to_array_with_deref_notation() -> TestResult<()> {
        let value: [u32; 0] = [];
        let reference = &value;
        verify_that!(*reference, empty())
    }

    #[test]
    fn empty_matcher_matches_empty_slice_with_points_to() -> TestResult<()> {
        let value: Vec<u32> = vec![];
        let slice = value.as_slice();
        verify_that!(slice, points_to(empty()))
    }

    #[test]
    fn empty_matcher_matches_empty_slice_with_deref_notation() -> TestResult<()> {
        let value: Vec<u32> = vec![];
        let slice = value.as_slice();
        verify_that!(*slice, empty())
    }

    #[test]
    fn empty_matcher_does_not_match_non_empty_vec() -> TestResult<()> {
        verify_that!(vec![1, 2, 3], not(empty()))
    }

    #[test]
    fn empty_matcher_does_not_match_non_empty_array() -> TestResult<()> {
        verify_that!([1, 2, 3], not(empty()))
    }

    #[test]
    fn empty_matcher_does_not_match_non_empty_slice() -> TestResult<()> {
        let value: Vec<u32> = vec![1, 2, 3];
        let slice = value.as_slice();
        verify_that!(*slice, not(empty()))
    }

    #[test]
    fn empty_matcher_matches_empty_hash_set() -> TestResult<()> {
        let value: HashSet<i32> = HashSet::new();
        verify_that!(value, empty())
    }

    #[derive(Debug)]
    struct OwnedItemContainer(Vec<i32>);

    impl<'a> IntoIterator for &'a OwnedItemContainer {
        type Item = i32;
        type IntoIter = std::iter::Copied<std::slice::Iter<'a, i32>>;
        fn into_iter(self) -> Self::IntoIter {
            self.0.iter().copied()
        }
    }

    #[test]
    fn empty_matches_on_container_when_ref_to_container_has_into_iterator_producing_owned_values()
    -> TestResult<()> {
        verify_that!(OwnedItemContainer(vec![]), empty())
    }
}
