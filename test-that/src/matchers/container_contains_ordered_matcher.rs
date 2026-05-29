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

// There are no visible documentation elements in this module; the declarative
// macro is documented in the matchers module.
#![doc(hidden)]

/// Matches a container's elements to each matcher in order.
///
/// This macro produces a matcher against a container. It takes as arguments a
/// sequence of matchers each of which should respectively match the
/// corresponding element of the actual value.
///
/// ```
/// # use test_that::prelude::*;
/// verify_that!(vec![1, 2, 3], contains_exactly![eq(1), anything(), gt(0).and(lt(123))].in_order())
/// #    .unwrap();
/// ```
///
/// The actual value must be a container such as a `Vec`, an array, or a
/// dereferenced slice. More precisely, a shared borrow of the actual value must
/// implement [`IntoIterator`].
///
/// ```
/// # use test_that::prelude::*;
/// let vector = vec![1, 2, 3];
/// let slice = vector.as_slice();
/// verify_that!(*slice, contains_exactly![eq(1), anything(), gt(0).and(lt(123))].in_order())
/// #    .unwrap();
/// ```
///
/// This can also be omitted in [`verify_that!`] macros and replaced with square
/// brackets.
///
/// ```
/// # use test_that::prelude::*;
/// verify_that!(vec![1, 2], [eq(1), eq(2)])
/// #     .unwrap();
/// ```
///
/// Note: This behavior is only possible in [`verify_that!`] macros. In any
/// other cases, it is still necessary to use the
/// [`elements_are!`][crate::matchers::elements_are] macro.
///
/// ```compile_fail
/// # use test_that::prelude::*;
/// verify_that!(vec![vec![1,2], vec![3]], [[eq(1), eq(2)], [eq(3)]])
/// # .unwrap();
/// ```
///
/// Use this instead:
/// ```
/// # use test_that::prelude::*;
/// verify_that!(vec![vec![1,2], vec![3]], [contains_exactly![eq(1), eq(2)].in_order(), contains_exactly![eq(3)]])
/// # .unwrap();
/// ```
///
/// This matcher does not support matching directly against an [`Iterator`]. To
/// match against an iterator, use [`Iterator::collect`] to build a [`Vec`].
///
/// Do not use this with unordered containers, since that will lead to flaky
/// tests. Use
/// [`contains_exactly!`][crate::matchers::contains_exactly]
/// instead.
///
/// [`IntoIterator`]: std::iter::IntoIterator
/// [`Iterator`]: std::iter::Iterator
/// [`Iterator::collect`]: std::iter::Iterator::collect
/// [`Vec`]: std::vec::Vec
#[macro_export]
#[doc(hidden)]
macro_rules! __elements_are {
    ($($matcher:expr),* $(,)?) => {{
        use $crate::matchers::__internal_unstable_do_not_depend_on_these::ContainerContainsOrderedMatcher;
        ContainerContainsOrderedMatcher::new([$(Box::new($matcher)),*])
    }}
}

/// Module for use only by the procedural macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::description::Description;
    use crate::matcher::{Describable, Matcher, MatcherResult};
    use crate::matcher_support::zipped_iterator::zip;
    use crate::matchers::container_contains::{OwnedItems, RefItems};
    use std::borrow::Borrow;
    use std::{fmt::Debug, marker::PhantomData};

    /// This struct is meant to be used only by the macro `elements_are!`.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub struct ContainerContainsOrderedMatcher<'a, ContainerT: ?Sized, T: Debug, ModeT, const N: usize>
    {
        elements: [Box<dyn Matcher<T> + 'a>; N],
        _phantom: PhantomData<(*const ContainerT, ModeT)>,
    }

    impl<'a, ContainerT: ?Sized, T: Debug, ModeT, const N: usize>
        ContainerContainsOrderedMatcher<'a, ContainerT, T, ModeT, N>
    {
        /// Factory only intended for use in the macro `elements_are!`.
        ///
        /// **For internal use only. API stablility is not guaranteed!**
        #[doc(hidden)]
        pub fn new(elements: [Box<dyn Matcher<T> + 'a>; N]) -> Self {
            Self { elements, _phantom: PhantomData }
        }

        fn matches_with_iter<ItemT: Borrow<T>>(
            &self,
            actual: impl Iterator<Item = ItemT>,
        ) -> MatcherResult {
            let mut zipped_iterator = zip(actual, self.elements.iter());
            for (a, e) in zipped_iterator.by_ref() {
                if e.matches(a.borrow()).is_no_match() {
                    return MatcherResult::NoMatch;
                }
            }
            if !zipped_iterator.has_size_mismatch() {
                MatcherResult::Match
            } else {
                MatcherResult::NoMatch
            }
        }

        fn explain_match_with_iter<ItemT: Borrow<T>>(
            &self,
            actual: impl Iterator<Item = ItemT>,
        ) -> Description {
            let mut zipped_iterator = zip(actual, self.elements.iter());
            let mut mismatches = Vec::new();
            for (idx, (a, e)) in zipped_iterator.by_ref().enumerate() {
                if e.matches(a.borrow()).is_no_match() {
                    mismatches.push(format!(
                        "element #{idx} is {:?}, {}",
                        a.borrow(),
                        e.explain_match(a.borrow())
                    ));
                }
            }
            if mismatches.is_empty() {
                if !zipped_iterator.has_size_mismatch() {
                    "whose elements all match".into()
                } else {
                    format!("whose size is {}", zipped_iterator.left_size()).into()
                }
            } else if mismatches.len() == 1 {
                let mismatches = mismatches.into_iter().collect::<Description>();
                format!("where {mismatches}").into()
            } else {
                let mismatches = mismatches.into_iter().collect::<Description>();
                format!("where:\n{}", mismatches.bullet_list().indent()).into()
            }
        }
    }

    impl<'a, T: Debug, ContainerT: Debug + ?Sized, const N: usize> Matcher<ContainerT>
        for ContainerContainsOrderedMatcher<'a, ContainerT, T, RefItems, N>
    where
        for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
    {
        fn matches(&self, actual: &ContainerT) -> MatcherResult {
            self.matches_with_iter(actual.into_iter())
        }

        fn explain_match(&self, actual: &ContainerT) -> Description {
            self.explain_match_with_iter(actual.into_iter())
        }
    }

    impl<'a, T: Debug, ContainerT: Debug + ?Sized, const N: usize> Matcher<ContainerT>
        for ContainerContainsOrderedMatcher<'a, ContainerT, T, OwnedItems, N>
    where
        for<'b> &'b ContainerT: IntoIterator<Item = T>,
    {
        fn matches(&self, actual: &ContainerT) -> MatcherResult {
            self.matches_with_iter(actual.into_iter())
        }

        fn explain_match(&self, actual: &ContainerT) -> Description {
            self.explain_match_with_iter(actual.into_iter())
        }
    }

    impl<'a, T: Debug, ContainerT: ?Sized, ModeT, const N: usize> Describable
        for ContainerContainsOrderedMatcher<'a, ContainerT, T, ModeT, N>
    {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "{} elements:\n{}",
                if matcher_result.into() { "has" } else { "doesn't have" },
                &self
                    .elements
                    .iter()
                    .map(|matcher| matcher.describe(MatcherResult::Match))
                    .collect::<Description>()
                    .enumerate()
                    .indent()
            )
            .into()
        }
    }
}
