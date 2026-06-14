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
    matchers::containers::{OwnedItems, RefItems},
};
use alloc::boxed::Box;
use core::{fmt::Debug, marker::PhantomData};

/// Matches an iterable type whose elements contain a value matched by `inner`.
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(["Some value"], contains(eq("Some value")))?;  // Passes
/// verify_that!(vec!["Some value"], contains(eq("Some value")))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> TestResult<()> {
/// verify_that!([] as [String; 0], contains(eq("Some value")))?;   // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> TestResult<()> {
/// verify_that!(["Some value"], contains(eq("Some other value")))?;   // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// ```
///
/// By default, this matches a container with any number of elements matched
/// by `inner`. Use the method [`ContainsMatcher::times`] to constrain the
/// matched containers to a specific number of matching elements.
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!([1, 1, 1], contains(eq(1)))?;              // Passes
/// verify_that!([1, 1, 1], contains(eq(1)).times(eq(3)))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> TestResult<()> {
/// verify_that!([1, 1, 1], contains(eq(1)).times(eq(2)))?; // Fails: wrong count
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// See [module documentation][crate::matchers::containers] for information
/// about what types this matcher can match.
pub fn contains<InnerMatcherT, ModeT>(
    inner: InnerMatcherT,
) -> ContainsMatcher<InnerMatcherT, ModeT> {
    ContainsMatcher { inner, count: None, phantom: Default::default() }
}

/// A matcher which matches a container containing one or more elements a given
/// inner [`Matcher`] matches.
pub struct ContainsMatcher<InnerMatcherT, ModeT> {
    inner: InnerMatcherT,
    count: Option<Box<dyn Matcher<usize>>>,
    phantom: PhantomData<ModeT>,
}

impl<InnerMatcherT, ModeT> ContainsMatcher<InnerMatcherT, ModeT> {
    /// Configures this instance to match containers which contain a number of
    /// matching items matched by `count`.
    ///
    /// For example, to assert that exactly three matching items must be
    /// present, use:
    ///
    /// ```ignore
    /// contains(...).times(eq(3))
    /// ```
    ///
    /// One can also use `times(eq(0))` to test for the *absence* of an item
    /// matching the expected value.
    pub fn times(mut self, count: impl Matcher<usize> + 'static) -> Self {
        self.count = Some(Box::new(count));
        self
    }
}

// Case 4: `&Container: IntoIterator<Item = T>` (owned items from a borrowed
// container). Used when the container yields owned T (not &T) when iterated via
// reference, e.g. a custom container that copies or clones its elements on
// iteration.
impl<T: Debug, InnerMatcherT: Matcher<T>, ContainerT: Debug + ?Sized> Matcher<ContainerT>
    for ContainsMatcher<InnerMatcherT, OwnedItems>
where
    for<'a> &'a ContainerT: IntoIterator<Item = T>,
{
    fn matches(&self, actual: &ContainerT) -> MatcherResult {
        if let Some(count) = &self.count {
            count.matches(&self.count_matches_owned(actual))
        } else {
            for v in actual.into_iter() {
                if self.inner.matches(&v).into() {
                    return MatcherResult::Match;
                }
            }
            MatcherResult::NoMatch
        }
    }

    fn explain_match(&self, actual: &ContainerT) -> Description {
        let count = self.count_matches_owned(actual);
        match (count, &self.count) {
            (_, Some(_)) => format!("which contains {} matching elements", count).into(),
            (0, None) => "which does not contain a matching element".into(),
            (_, None) => "which contains a matching element".into(),
        }
    }
}

// TODO(hovinen): Revisit the trait bounds to see whether this can be made more
//  flexible. Namely, the following doesn't compile currently:
//
//      let matcher = contains(eq(&42));
//      let val = 42;
//      let _ = matcher.matches(&vec![&val]);
//
//  because val is dropped before matcher but the trait bound requires that
//  the argument to matches outlive the matcher. It works fine if one defines
//  val before matcher.
impl<T: Debug, InnerMatcherT: Matcher<T>, ContainerT: Debug + ?Sized> Matcher<ContainerT>
    for ContainsMatcher<InnerMatcherT, RefItems>
where
    for<'a> &'a ContainerT: IntoIterator<Item = &'a T>,
{
    fn matches(&self, actual: &ContainerT) -> MatcherResult {
        if let Some(count) = &self.count {
            count.matches(&self.count_matches_ref(actual))
        } else {
            for v in actual.into_iter() {
                if self.inner.matches(v).into() {
                    return MatcherResult::Match;
                }
            }
            MatcherResult::NoMatch
        }
    }

    fn explain_match(&self, actual: &ContainerT) -> Description {
        let count = self.count_matches_ref(actual);
        match (count, &self.count) {
            (_, Some(_)) => format!("which contains {} matching elements", count).into(),
            (0, None) => "which does not contain a matching element".into(),
            (_, None) => "which contains a matching element".into(),
        }
    }
}

impl<InnerMatcherT: Describable, ModeT> Describable for ContainsMatcher<InnerMatcherT, ModeT> {
    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match (matcher_result, &self.count) {
            (MatcherResult::Match, Some(count)) => format!(
                "contains n elements which {}\n  where n {}",
                self.inner.describe(MatcherResult::Match),
                count.describe(MatcherResult::Match)
            )
            .into(),
            (MatcherResult::NoMatch, Some(count)) => format!(
                "doesn't contain n elements which {}\n  where n {}",
                self.inner.describe(MatcherResult::Match),
                count.describe(MatcherResult::Match)
            )
            .into(),
            (MatcherResult::Match, None) => format!(
                "contains at least one element which {}",
                self.inner.describe(MatcherResult::Match)
            )
            .into(),
            (MatcherResult::NoMatch, None) => {
                format!("contains no element which {}", self.inner.describe(MatcherResult::Match))
                    .into()
            }
        }
    }
}

impl<InnerMatcherT, ModeT> ContainsMatcher<InnerMatcherT, ModeT> {
    fn count_matches_ref<T: Debug, ContainerT: ?Sized>(&self, actual: &ContainerT) -> usize
    where
        for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
        InnerMatcherT: Matcher<T>,
    {
        let mut count = 0;
        for v in actual.into_iter() {
            if self.inner.matches(v).into() {
                count += 1;
            }
        }
        count
    }

    fn count_matches_owned<T: Debug, ContainerT: ?Sized>(&self, actual: &ContainerT) -> usize
    where
        for<'b> &'b ContainerT: IntoIterator<Item = T>,
        InnerMatcherT: Matcher<T>,
    {
        let mut count = 0;
        for v in actual.into_iter() {
            if self.inner.matches(&v).into() {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::{ContainsMatcher, contains};
    use crate::matcher::{Describable as _, Matcher, MatcherResult};
    use crate::matchers::containers::RefItems;
    use crate::prelude::*;
    use alloc::{string::String, vec::Vec};

    #[test]
    fn contains_matches_singleton_slice_with_value() -> TestResult<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&vec![1]);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_matches_singleton_vec_with_value() -> TestResult<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&vec![1]);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_matches_two_element_slice_with_value() -> TestResult<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&[0, 1]);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_does_not_match_singleton_slice_with_wrong_value() -> TestResult<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&[0]);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_does_not_match_empty_slice() -> TestResult<()> {
        let value = Vec::<i32>::new();
        verify_that!(value.as_slice(), not(points_to(contains(eq(1)))))
    }

    #[test]
    fn contains_matches_slice_with_repeated_value() -> TestResult<()> {
        let matcher = contains(eq(1)).times(eq(2));

        let result = matcher.matches(&[1, 1]);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_does_not_match_slice_with_too_few_of_value() -> TestResult<()> {
        let matcher = contains(eq(1)).times(eq(2));

        let result = matcher.matches(&[0, 1]);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_does_not_match_slice_with_too_many_of_value() -> TestResult<()> {
        let matcher = contains(eq(1)).times(eq(1));

        let result = matcher.matches(&[1, 1]);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_matches_on_vec_of_values() -> TestResult<()> {
        verify_that!(vec![1, 2, 3], contains(eq(1)))
    }

    #[test]
    fn contains_matches_on_array_of_values() -> TestResult<()> {
        verify_that!([1, 2, 3], contains(eq(1)))
    }

    #[test]
    fn contains_matches_on_slice_of_values_with_points_to_slice() -> TestResult<()> {
        verify_that!(&[1, 2, 3], points_to(contains(eq(1))))
    }

    #[test]
    fn contains_matches_on_slice_of_values_with_deref_notation_on_macro() -> TestResult<()> {
        let slice = &[1, 2, 3];
        verify_that!(*slice, contains(eq(1)))
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
    fn contains_matches_on_container_when_ref_to_container_has_into_iterator_producing_owned_values()
    -> TestResult<()> {
        verify_that!(OwnedItemContainer(vec![1, 2, 3]), contains(eq(1)))
    }

    #[test]
    fn contains_matches_vec_of_string_slices() -> TestResult<()> {
        verify_that!(vec!["String 1", "String 2", "String 3"], contains(contains_substring("1")))
    }

    #[test]
    fn contains_matches_vec_of_string_slices_with_non_static_lifetime() -> TestResult<()> {
        let string_1 = String::from("String 1");
        let string_2 = String::from("String 2");
        let string_3 = String::from("String 3");
        verify_that!(
            vec![string_1.as_str(), string_2.as_str(), string_3.as_str()],
            contains(contains_substring("1"))
        )
    }

    #[test]
    fn contains_matches_slice_of_string_slices() -> TestResult<()> {
        let value = vec!["String 1", "String 2", "String 3"];
        verify_that!(value.as_slice(), points_to(contains(contains_substring("1"))))
    }

    #[test]
    fn contains_matches_slice_of_string_slices_with_non_static_lifetime() -> TestResult<()> {
        let string_1 = String::from("String 1");
        let string_2 = String::from("String 2");
        let string_3 = String::from("String 3");
        let value = vec![string_1.as_str(), string_2.as_str(), string_3.as_str()];
        verify_that!(value.as_slice(), points_to(contains(contains_substring("1"))))
    }

    #[test]
    fn contains_formats_without_multiplicity_by_default() -> TestResult<()> {
        let matcher: ContainsMatcher<_, RefItems> = contains(eq(1));

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq("contains at least one element which is equal to 1"))
        )
    }

    #[test]
    fn contains_formats_with_multiplicity_when_specified() -> TestResult<()> {
        let matcher: ContainsMatcher<_, RefItems> = contains(eq(1)).times(eq(2));

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq("contains n elements which is equal to 1\n  where n is equal to 2"))
        )
    }

    #[test]
    fn contains_mismatch_shows_number_of_times_element_was_found() -> TestResult<()> {
        verify_that!(
            contains(eq(3)).times(eq(1)).explain_match(&vec![1, 2, 3, 3]),
            displays_as(eq("which contains 2 matching elements"))
        )
    }

    #[test]
    fn contains_mismatch_shows_when_matches() -> TestResult<()> {
        verify_that!(
            contains(eq(3)).explain_match(&vec![1, 2, 3, 3]),
            displays_as(eq("which contains a matching element"))
        )
    }

    #[test]
    fn contains_mismatch_shows_when_no_matches() -> TestResult<()> {
        verify_that!(
            contains(eq(3)).explain_match(&vec![1, 2]),
            displays_as(eq("which does not contain a matching element"))
        )
    }
}
