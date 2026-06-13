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

use crate::description::Description;
use crate::matcher::{Describable, Matcher, MatcherResult};
use crate::matcher_support::count_elements::count_elements;
use std::fmt::Debug;

/// Matches a container whose number of elements matches `expected`.
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// let array = [1,2,3];
/// verify_that!(array, len(eq(3)))?;
/// let vec = vec![1,2,3];
/// verify_that!(vec, len(eq(3)))?;
/// let slice = vec.as_slice();
/// verify_that!(*slice, len(eq(3)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// See [module documentation][crate::matchers::containers] for information about
/// what types this matcher can match.
///
/// The parameter `expected` can be any integer numeric matcher.
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// let vec = vec![1,2,3];
/// verify_that!(vec, len(gt(1)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
pub fn len<E: Matcher<usize>>(expected: E) -> LenMatcher<E> {
    LenMatcher { expected }
}

#[doc(hidden)]
pub struct LenMatcher<E> {
    expected: E,
}

impl<T: Debug + ?Sized, E: Matcher<usize>> Matcher<T> for LenMatcher<E>
where
    for<'a> &'a T: IntoIterator,
{
    fn matches(&self, actual: &T) -> MatcherResult {
        self.expected.matches(&count_elements(actual))
    }

    fn explain_match(&self, actual: &T) -> Description {
        let actual_size = count_elements(actual);
        format!("which has length {}, {}", actual_size, self.expected.explain_match(&actual_size))
            .into()
    }
}

impl<E: Describable> Describable for LenMatcher<E> {
    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => {
                format!("has length, which {}", self.expected.describe(MatcherResult::Match)).into()
            }
            MatcherResult::NoMatch => {
                format!("has length, which {}", self.expected.describe(MatcherResult::NoMatch))
                    .into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::len;
    use crate::description::Description;
    use crate::matcher::{Describable, Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;
    use std::collections::{
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
    };
    use std::fmt::Debug;
    use std::marker::PhantomData;

    #[test]
    fn len_matcher_matches_vec() -> TestResult<()> {
        verify_that!(vec![1, 2, 3], len(eq(3)))
    }

    #[test]
    fn len_matcher_matches_array_reference_with_deref_notation() -> TestResult<()> {
        let value = &[1, 2, 3];
        verify_that!(*value, len(eq(3)))
    }

    #[test]
    fn len_matcher_matches_array_reference_points_to() -> TestResult<()> {
        verify_that!(&[1, 2, 3], points_to(len(eq(3))))
    }

    #[test]
    fn len_matcher_matches_slice_of_array() -> TestResult<()> {
        let value = &[1, 2, 3];
        verify_that!(value[0..1], len(eq(1)))
    }

    #[test]
    fn len_matcher_matches_slice_of_vec() -> TestResult<()> {
        let value = vec![1, 2, 3];
        let slice = value.as_slice();
        verify_that!(*slice, len(eq(3)))
    }

    #[test]
    fn len_matcher_matches_array() -> TestResult<()> {
        verify_that!([1, 2, 3], len(eq(3)))
    }

    #[test]
    fn len_matcher_matches_btreemap() -> TestResult<()> {
        let value = BTreeMap::from([(1, 2), (2, 3), (3, 4)]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_matches_btreeset() -> TestResult<()> {
        let value = BTreeSet::from([1, 2, 3]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_matches_binaryheap() -> TestResult<()> {
        let value = BinaryHeap::from([1, 2, 3]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_matches_hashmap() -> TestResult<()> {
        let value = HashMap::from([(1, 2), (2, 3), (3, 4)]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_matches_hashset() -> TestResult<()> {
        let value = HashSet::from([1, 2, 3]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_matches_linkedlist() -> TestResult<()> {
        let value = LinkedList::from([1, 2, 3]);
        verify_that!(value, len(eq(3)))
    }

    #[test]
    fn len_matcher_matches_vecdeque() -> TestResult<()> {
        let value = VecDeque::from([1, 2, 3]);
        verify_that!(value, len(eq(3)))
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
    fn len_matches_on_container_when_ref_to_container_has_into_iterator_producing_owned_values()
    -> TestResult<()> {
        verify_that!(OwnedItemContainer(vec![1, 2, 3]), len(eq(3)))
    }

    #[test]
    fn len_matcher_explain_match() -> TestResult<()> {
        struct TestMatcher<T>(PhantomData<T>);
        impl<T: Debug> Matcher<T> for TestMatcher<T> {
            fn matches(&self, _: &T) -> MatcherResult {
                false.into()
            }

            fn explain_match(&self, _: &T) -> Description {
                "called explain_match".into()
            }
        }
        impl<T> Describable for TestMatcher<T> {
            fn describe(&self, _: MatcherResult) -> Description {
                "called described".into()
            }
        }
        verify_that!(
            len(TestMatcher(Default::default())).explain_match(&[1, 2, 3]),
            displays_as(eq("which has length 3, called explain_match"))
        )
    }

    #[test]
    fn len_matcher_error_message() -> TestResult<()> {
        let result = verify_that!(vec![1, 2, 3, 4], len(eq(3)));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: vec![1, 2, 3, 4]
                Expected: has length, which is equal to 3
                Actual: [1, 2, 3, 4],
                  which has length 4, which isn't equal to 3"
            ))))
        )
    }
}
