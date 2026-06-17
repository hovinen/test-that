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

/// Matches a container all of whose items are in the given container
/// `superset`.
///
/// The element type `ElementT` must implement `PartialEq` to allow element
/// comparison.
///
/// See [module documentation][crate::matchers::containers] for information
/// about what types this matcher can match. The actual and expected values need
/// not have the same container type, only compatible element types.
///
/// ```
/// # use test_that::prelude::*;
/// # use std::collections::HashSet;
/// # fn should_pass_1() -> TestResult<()> {
/// let value = vec![1, 2, 3];
/// verify_that!(value, subset_of([1, 2, 3, 4]))?;  // Passes
/// let array_value = [1, 2, 3];
/// verify_that!(array_value, subset_of([1, 2, 3, 4]))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> TestResult<()> {
/// # let value = vec![1, 2, 3];
/// verify_that!(value, subset_of([1, 2]))?;  // Fails: 3 is not in the superset
/// #     Ok(())
/// # }
/// # should_pass_1().unwrap();
/// # should_fail().unwrap_err();
///
/// # fn should_pass_2() -> TestResult<()> {
/// let value: HashSet<i32> = [1, 2, 3].into();
/// verify_that!(value, subset_of([1, 2, 3]))?;  // Passes
/// #     Ok(())
/// # }
/// # should_pass_2().unwrap();
/// ```
///
/// Item multiplicity in both the actual and expected containers is ignored:
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// let value: Vec<i32> = vec![0, 0, 1];
/// verify_that!(value, subset_of([0, 1]))?;  // Passes
/// verify_that!(value, subset_of([0, 1, 1]))?;  // Passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// One can also verify the contents of a slice by dereferencing it:
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// let value = &[1, 2, 3];
/// verify_that!(*value, subset_of([1, 2, 3]))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// A note on performance: This matcher uses a naive algorithm with a worst-case
/// runtime proportional to the *product* of the sizes of the actual and
/// expected containers as well as the time to check equality of each pair of
/// items. It should not be used on especially large containers.
pub fn subset_of<ExpectedT, Mode>(
    superset: ExpectedT,
) -> __internal::SubsetOfMatcher<ExpectedT, Mode> {
    __internal::SubsetOfMatcher { superset, phantom: Default::default() }
}

pub mod __internal {
    use crate::{
        description::Description,
        matcher::{Describable, Matcher, MatcherResult},
        matchers::containers::{OwnedItems, RefItems},
    };
    use alloc::vec::Vec;
    use core::{fmt::Debug, marker::PhantomData};

    #[doc(hidden)]
    pub struct SubsetOfMatcher<ExpectedT, Mode> {
        pub(super) superset: ExpectedT,
        pub(super) phantom: PhantomData<Mode>,
    }

    impl<ElementT: Debug + PartialEq, ActualT: Debug + ?Sized, ExpectedT: Debug> Matcher<ActualT>
        for SubsetOfMatcher<ExpectedT, RefItems>
    where
        for<'a> &'a ActualT: IntoIterator<Item = &'a ElementT>,
        for<'a> &'a ExpectedT: IntoIterator<Item = &'a ElementT>,
    {
        fn matches(&self, actual: &ActualT) -> MatcherResult {
            for actual_item in actual {
                if self.expected_is_missing(actual_item) {
                    return MatcherResult::NoMatch;
                }
            }
            MatcherResult::Match
        }

        fn explain_match(&self, actual: &ActualT) -> Description {
            let unexpected_elements = actual
                .into_iter()
                .enumerate()
                .filter(|&(_, actual_item)| self.expected_is_missing(actual_item))
                .map(|(idx, actual_item)| format!("{actual_item:#?} at #{idx}"))
                .collect::<Vec<_>>();

            match unexpected_elements.len() {
                0 => "which no element is unexpected".into(),
                1 => format!("whose element {} is unexpected", &unexpected_elements[0]).into(),
                _ => format!("whose elements {} are unexpected", unexpected_elements.join(", "))
                    .into(),
            }
        }
    }

    impl<ElementT: Debug + PartialEq, ActualT: Debug + ?Sized, ExpectedT: Debug> Matcher<ActualT>
        for SubsetOfMatcher<ExpectedT, OwnedItems>
    where
        for<'a> &'a ActualT: IntoIterator<Item = ElementT>,
        for<'a> &'a ExpectedT: IntoIterator<Item = &'a ElementT>,
    {
        fn matches(&self, actual: &ActualT) -> MatcherResult {
            for actual_item in actual {
                if self.expected_is_missing(&actual_item) {
                    return MatcherResult::NoMatch;
                }
            }
            MatcherResult::Match
        }

        fn explain_match(&self, actual: &ActualT) -> Description {
            let unexpected_elements = actual
                .into_iter()
                .enumerate()
                .filter(|(_, actual_item)| self.expected_is_missing(actual_item))
                .map(|(idx, actual_item)| format!("{actual_item:#?} at #{idx}"))
                .collect::<Vec<_>>();

            match unexpected_elements.len() {
                0 => "which no element is unexpected".into(),
                1 => format!("whose element {} is unexpected", &unexpected_elements[0]).into(),
                _ => format!("whose elements {} are unexpected", unexpected_elements.join(", "))
                    .into(),
            }
        }
    }

    impl<ElementT: PartialEq, ExpectedT, Mode> SubsetOfMatcher<ExpectedT, Mode>
    where
        for<'a> &'a ExpectedT: IntoIterator<Item = &'a ElementT>,
    {
        fn expected_is_missing(&self, needle: &ElementT) -> bool {
            !self.superset.into_iter().any(|item| *item == *needle)
        }
    }

    impl<ExpectedT: Debug, Mode> Describable for SubsetOfMatcher<ExpectedT, Mode> {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            match matcher_result {
                MatcherResult::Match => format!("is a subset of {:#?}", self.superset).into(),
                MatcherResult::NoMatch => format!("isn't a subset of {:#?}", self.superset).into(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::subset_of;
    use crate::prelude::*;
    use alloc::{string::String, vec::Vec};
    use indoc::indoc;

    #[test]
    fn subset_of_matches_empty_vec() -> TestResult<()> {
        let value: Vec<i32> = vec![];
        verify_that!(value, subset_of([]))
    }

    #[test]
    fn subset_of_matches_vec_with_one_element_with_array() -> TestResult<()> {
        verify_that!(vec![1], subset_of([1]))
    }

    #[test]
    fn subset_of_matches_vec_with_one_element_with_vec() -> TestResult<()> {
        verify_that!(vec![1], subset_of(vec![1]))
    }

    #[test]
    fn subset_of_matches_array_of_one_element_with_array() -> TestResult<()> {
        verify_that!([1], subset_of([1]))
    }

    #[test]
    fn subset_of_matches_vec_with_two_elements() -> TestResult<()> {
        verify_that!(vec![1, 2], subset_of([1, 2]))
    }

    #[test]
    fn subset_of_matches_vec_when_expected_has_excess_element() -> TestResult<()> {
        verify_that!(vec![1, 2], subset_of([1, 2, 3]))
    }

    #[test]
    fn subset_of_matches_vec_when_expected_has_excess_element_first() -> TestResult<()> {
        verify_that!(vec![1, 2], subset_of([3, 1, 2]))
    }

    #[test]
    fn subset_of_matches_array_ref_with_one_element_using_points_to() -> TestResult<()> {
        let value = &[1];
        verify_that!(value, points_to(subset_of([1])))
    }

    #[test]
    fn subset_of_matches_array_ref_with_one_element_using_deref_notation() -> TestResult<()> {
        let value = &[1];
        verify_that!(*value, subset_of([1]))
    }

    #[test]
    fn subset_of_matches_slice_with_one_element_using_points_to() -> TestResult<()> {
        let value = vec![1];
        let slice = value.as_slice();
        verify_that!(slice, points_to(subset_of([1])))
    }

    #[test]
    fn subset_of_matches_slice_with_one_element_using_deref_notation() -> TestResult<()> {
        let value = vec![1];
        let slice = value.as_slice();
        verify_that!(*slice, subset_of([1]))
    }

    #[derive(Debug, PartialEq)]
    struct OwnedItemContainer(Vec<i32>);

    impl<'a> IntoIterator for &'a OwnedItemContainer {
        type Item = i32;
        type IntoIter = core::iter::Copied<core::slice::Iter<'a, i32>>;
        fn into_iter(self) -> Self::IntoIter {
            self.0.iter().copied()
        }
    }

    #[test]
    fn subset_of_matches_on_container_when_ref_to_container_has_into_iterator_producing_owned_values()
    -> TestResult<()> {
        verify_that!(OwnedItemContainer(vec![1]), subset_of([1]))
    }

    #[test]
    fn subset_of_matches_vec_of_string_slices() -> TestResult<()> {
        verify_that!(
            vec!["String 1", "String 2", "String 3"],
            subset_of(["String 1", "String 2", "String 3"])
        )
    }

    #[test]
    fn subset_of_matches_vec_of_string_slices_with_non_static_lifetime() -> TestResult<()> {
        let string_1 = String::from("String 1");
        let string_2 = String::from("String 2");
        let string_3 = String::from("String 3");
        verify_that!(
            vec![string_1.as_str(), string_2.as_str(), string_3.as_str()],
            subset_of(["String 1", "String 2", "String 3"])
        )
    }

    #[test]
    fn subset_of_matches_slice_of_string_slices() -> TestResult<()> {
        let value = vec!["String 1", "String 2", "String 3"];
        verify_that!(value.as_slice(), points_to(subset_of(["String 1", "String 2", "String 3"])))
    }

    #[test]
    fn subset_of_matches_slice_of_string_slices_with_non_static_lifetime() -> TestResult<()> {
        let string_1 = String::from("String 1");
        let string_2 = String::from("String 2");
        let string_3 = String::from("String 3");
        let value = vec![string_1.as_str(), string_2.as_str(), string_3.as_str()];
        verify_that!(value.as_slice(), points_to(subset_of(["String 1", "String 2", "String 3"])))
    }

    #[cfg(feature = "std")]
    #[test]
    fn subset_of_matches_hash_set_with_one_element() -> TestResult<()> {
        use std::collections::HashSet;
        verify_that!(HashSet::from([1]), subset_of([1]))
    }

    #[test]
    fn subset_of_does_not_match_when_first_element_does_not_match() -> TestResult<()> {
        verify_that!(vec![0], not(subset_of([1])))
    }

    #[test]
    fn subset_of_does_not_match_when_second_element_does_not_match() -> TestResult<()> {
        verify_that!(vec![2, 0], not(subset_of([2])))
    }

    #[test]
    fn subset_of_shows_correct_message_when_first_item_does_not_match() -> TestResult<()> {
        let result = verify_that!(vec![0, 2, 3], subset_of([1, 2, 3]));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Value of: vec![0, 2, 3]
                    Expected: is a subset of [
                        1,
                        2,
                        3,
                    ]
                    Actual: [0, 2, 3],
                      whose element 0 at #0 is unexpected
                "
            ))))
        )
    }

    #[test]
    fn subset_of_shows_correct_message_when_second_item_does_not_match() -> TestResult<()> {
        let result = verify_that!(vec![1, 0, 3], subset_of([1, 2, 3]));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Value of: vec![1, 0, 3]
                    Expected: is a subset of [
                        1,
                        2,
                        3,
                    ]
                    Actual: [1, 0, 3],
                      whose element 0 at #1 is unexpected
                "
            ))))
        )
    }

    #[test]
    fn subset_of_shows_correct_message_when_first_two_items_do_not_match() -> TestResult<()> {
        let result = verify_that!(vec![0, 0, 3], subset_of([1, 2, 3]));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Value of: vec![0, 0, 3]
                    Expected: is a subset of [
                        1,
                        2,
                        3,
                    ]
                    Actual: [0, 0, 3],
                      whose elements 0 at #0, 0 at #1 are unexpected
                "
            ))))
        )
    }
}
