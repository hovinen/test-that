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

/// Module for use only by the procedural macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod __internal {
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use crate::description::Description;
    use crate::matcher::{Describable, Matcher, MatcherResult};
    use crate::matcher_support::zipped_iterator::zip;
    use crate::matchers::containers::{OwnedItems, RefItems, container_contains::Requirements};
    use core::borrow::Borrow;
    use core::{fmt::Debug, marker::PhantomData};

    /// This struct is meant to be used only by the macro `elements_are!`.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub struct ContainerContainsOrderedMatcher<
        'matchers,
        ContainerT: ?Sized,
        T: Debug,
        ModeT,
        const N: usize,
    > {
        elements: [Box<dyn Matcher<T> + 'matchers>; N],
        requirements: Requirements,
        _phantom: PhantomData<(*const ContainerT, ModeT)>,
    }

    impl<'matchers, ContainerT: ?Sized, T: Debug, ModeT, const N: usize>
        ContainerContainsOrderedMatcher<'matchers, ContainerT, T, ModeT, N>
    {
        /// Factory only intended for use in the macro `elements_are!`.
        ///
        /// **For internal use only. API stablility is not guaranteed!**
        #[doc(hidden)]
        pub fn new(
            elements: [Box<dyn Matcher<T> + 'matchers>; N],
            requirements: Requirements,
        ) -> Self {
            Self { elements, requirements, _phantom: PhantomData }
        }

        fn matches_with_iter<ItemT: Borrow<T>>(
            &self,
            mut actual_iter: impl Iterator<Item = ItemT>,
        ) -> MatcherResult {
            let mut expected_iter = self.elements.iter();
            let mut maybe_actual = actual_iter.next();
            let mut maybe_expected = expected_iter.next();
            loop {
                let Some(actual) = maybe_actual.as_ref() else {
                    match self.requirements {
                        Requirements::PerfectMatch | Requirements::Superset => {
                            return maybe_expected.is_none().into();
                        }
                        Requirements::Subset => return MatcherResult::Match,
                    }
                };
                let Some(expected) = maybe_expected else {
                    match self.requirements {
                        Requirements::PerfectMatch | Requirements::Subset => {
                            return MatcherResult::NoMatch;
                        }
                        Requirements::Superset => return MatcherResult::Match,
                    }
                };
                if expected.matches(actual.borrow()).is_match() {
                    maybe_actual = actual_iter.next();
                    maybe_expected = expected_iter.next();
                } else {
                    match self.requirements {
                        Requirements::PerfectMatch => return MatcherResult::NoMatch,
                        Requirements::Superset => {
                            maybe_actual = actual_iter.next();
                        }
                        Requirements::Subset => {
                            maybe_expected = expected_iter.next();
                        }
                    }
                }
            }
        }

        fn explain_match_with_iter<ItemT: Borrow<T>>(
            &self,
            mut actual_iter: impl Iterator<Item = ItemT>,
        ) -> Description {
            match self.requirements {
                Requirements::PerfectMatch => {
                    let mut zipped_iterator = zip(actual_iter, self.elements.iter());
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
                Requirements::Superset => {
                    let mut expected_iter = self.elements.iter().enumerate();
                    let mut maybe_actual = actual_iter.next();
                    let mut maybe_expected = expected_iter.next();
                    let matcher = loop {
                        let Some(actual) = maybe_actual.as_ref() else {
                            break maybe_expected;
                        };
                        let Some((_, expected)) = maybe_expected else {
                            break None;
                        };
                        if expected.matches(actual.borrow()).is_match() {
                            maybe_actual = actual_iter.next();
                            maybe_expected = expected_iter.next();
                        } else {
                            maybe_actual = actual_iter.next();
                        }
                    };
                    if let Some((index, _)) = matcher {
                        format!("where matcher #{index} does not match any following elements")
                            .into()
                    } else {
                        "where all elements are present".into()
                    }
                }
                Requirements::Subset => {
                    let mut actual_index_iter = actual_iter.enumerate();
                    let mut expected_iter = self.elements.iter();
                    let mut maybe_actual = actual_index_iter.next();
                    let mut maybe_expected = expected_iter.next();
                    let item = loop {
                        let Some((_, actual)) = maybe_actual.as_ref() else {
                            break None;
                        };
                        let Some(expected) = maybe_expected else {
                            break maybe_actual;
                        };
                        if expected.matches(actual.borrow()).is_match() {
                            maybe_actual = actual_index_iter.next();
                            maybe_expected = expected_iter.next();
                        } else {
                            maybe_expected = expected_iter.next();
                        }
                    };
                    if let Some((index, _)) = item {
                        format!("where element #{index} is not matched by any following matcher")
                            .into()
                    } else {
                        "where all elements are matched".into()
                    }
                }
            }
        }
    }

    impl<'matchers, T: Debug, ContainerT: Debug + ?Sized, const N: usize> Matcher<ContainerT>
        for ContainerContainsOrderedMatcher<'matchers, ContainerT, T, RefItems, N>
    where
        for<'elements> &'elements ContainerT: IntoIterator<Item = &'elements T>,
    {
        fn matches(&self, actual: &ContainerT) -> MatcherResult {
            self.matches_with_iter(actual.into_iter())
        }

        fn explain_match(&self, actual: &ContainerT) -> Description {
            self.explain_match_with_iter(actual.into_iter())
        }
    }

    impl<'matchers, T: Debug, ContainerT: Debug + ?Sized, const N: usize> Matcher<ContainerT>
        for ContainerContainsOrderedMatcher<'matchers, ContainerT, T, OwnedItems, N>
    where
        for<'container> &'container ContainerT: IntoIterator<Item = T>,
    {
        fn matches(&self, actual: &ContainerT) -> MatcherResult {
            self.matches_with_iter(actual.into_iter())
        }

        fn explain_match(&self, actual: &ContainerT) -> Description {
            self.explain_match_with_iter(actual.into_iter())
        }
    }

    impl<'matchers, T: Debug, ContainerT: ?Sized, ModeT, const N: usize> Describable
        for ContainerContainsOrderedMatcher<'matchers, ContainerT, T, ModeT, N>
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
