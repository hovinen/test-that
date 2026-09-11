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
// macro is documented in the matcher module.
#![doc(hidden)]

/// Matches a value which all of the given matchers match.
///
/// Each argument is a [`Matcher`][crate::matcher::Matcher] which matches
/// against the actual value.
///
/// For example:
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!("A string", all!(starts_with("A"), ends_with("string")))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> TestResult<()> {
/// verify_that!("A string", all!(starts_with("A"), ends_with("not a string")))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// Using this macro is equivalent to using the
/// [`and`][crate::matcher::MatcherExt::and] method:
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(10, gt(9).and(lt(11)))?; // Also passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Assertion failure messages are not guaranteed to be identical, however.
#[macro_export]
#[doc(hidden)]
macro_rules! __all {
    ($($matcher:expr),* $(,)?) => {{
        $crate::matchers::__internal::AllMatcher::new($crate::__matcher_list!($($matcher),*))
    }}
}

/// Arranges the given matchers into the nested tuple consumed by
/// [`AllMatcher`][crate::matchers::__internal::AllMatcher] and
/// [`AnyMatcher`][crate::matchers::__internal::AnyMatcher].
///
/// The matchers are held by value in a nested tuple rather than as an array of
/// boxed trait objects. Boxing them would coerce each to `dyn Matcher<T>` at
/// the point the list is built, where `T` is not yet known; that coercion has
/// to solve `Matcher<T>` for an unresolved `T`, which can make inference
/// diverge. Keeping them concrete defers the coercion to
/// [`Components::for_each`][crate::matchers::__internal::Components::for_each],
/// where `T` is a bound parameter.
///
/// The nesting is what makes the number of matchers unbounded: a tuple of each
/// arity would need an implementation of each arity.
///
/// For internal use only. API stability is not guaranteed!
#[macro_export]
#[doc(hidden)]
macro_rules! __matcher_list {
    () => { () };
    ($matcher:expr $(,)?) => { ($matcher,) };
    ($matcher:expr, $($rest:expr),+ $(,)?) => {
        ($matcher, $crate::__matcher_list!($($rest),+))
    };
}

/// Functionality needed by the [`all`] macro.
///
/// For internal use only. API stability is not guaranteed!
#[doc(hidden)]
pub mod __internal {
    use crate::description::Description;
    use crate::matcher::{Describable, Matcher, MatcherResult};
    use crate::matchers::anything;
    use alloc::vec::Vec;
    use core::fmt::Debug;
    use core::marker::PhantomData;

    /// The component matchers of an [`AllMatcher`] or [`AnyMatcher`], held as a
    /// nested tuple: `()`, `(a,)`, `(a, (b,))`, `(a, (b, (c,)))` and so on.
    ///
    /// For internal use only. API stability is not guaranteed!
    #[doc(hidden)]
    pub trait Components<T: Debug + ?Sized> {
        /// The number of component matchers.
        fn count(&self) -> usize;

        /// Invokes `visit` on each component matcher in turn.
        fn for_each(&self, visit: &mut dyn FnMut(&dyn Matcher<T>));
    }

    impl<T: Debug + ?Sized> Components<T> for () {
        fn count(&self) -> usize {
            0
        }

        fn for_each(&self, _visit: &mut dyn FnMut(&dyn Matcher<T>)) {}
    }

    impl<T: Debug + ?Sized, MatcherT: Matcher<T>> Components<T> for (MatcherT,) {
        fn count(&self) -> usize {
            1
        }

        fn for_each(&self, visit: &mut dyn FnMut(&dyn Matcher<T>)) {
            visit(&self.0);
        }
    }

    impl<T: Debug + ?Sized, MatcherT: Matcher<T>, RestT: Components<T>> Components<T>
        for (MatcherT, RestT)
    {
        fn count(&self) -> usize {
            1 + self.1.count()
        }

        fn for_each(&self, visit: &mut dyn FnMut(&dyn Matcher<T>)) {
            visit(&self.0);
            self.1.for_each(visit);
        }
    }

    /// The descriptions of every component matcher.
    pub(crate) fn descriptions<T: Debug + ?Sized>(
        components: &impl Components<T>,
        matcher_result: MatcherResult,
    ) -> Vec<Description> {
        let mut output = Vec::with_capacity(components.count());
        components.for_each(&mut |component| output.push(component.describe(matcher_result)));
        output
    }

    /// The explanations of every component matcher which does not match.
    pub(crate) fn failure_explanations<T: Debug + ?Sized>(
        components: &impl Components<T>,
        actual: &T,
    ) -> Vec<Description> {
        let mut output = Vec::new();
        components.for_each(&mut |component| {
            if component.matches(actual).is_no_match() {
                output.push(component.explain_match(actual));
            }
        });
        output
    }

    /// The explanations of every component matcher.
    pub(crate) fn explanations<T: Debug + ?Sized>(
        components: &impl Components<T>,
        actual: &T,
    ) -> Vec<Description> {
        let mut output = Vec::with_capacity(components.count());
        components.for_each(&mut |component| output.push(component.explain_match(actual)));
        output
    }

    /// A matcher which matches an input value matched by all of its component
    /// matchers.
    ///
    /// For internal use only. API stability is not guaranteed!
    #[doc(hidden)]
    pub struct AllMatcher<T: Debug + ?Sized, ComponentsT> {
        components: ComponentsT,
        phantom: PhantomData<fn(&T)>,
    }

    impl<T: Debug + ?Sized, ComponentsT> AllMatcher<T, ComponentsT> {
        /// Constructs an [`AllMatcher`] with the given component matchers.
        ///
        /// Intended for use only by the [`all`] macro.
        pub fn new(components: ComponentsT) -> Self {
            Self { components, phantom: PhantomData }
        }
    }

    impl<T: Debug + ?Sized, ComponentsT: Components<T>> Matcher<T> for AllMatcher<T, ComponentsT> {
        fn matches(&self, actual: &T) -> MatcherResult {
            let mut result = MatcherResult::Match;
            self.components.for_each(&mut |component| {
                if component.matches(actual).is_no_match() {
                    result = MatcherResult::NoMatch;
                }
            });
            result
        }

        fn explain_match(&self, actual: &T) -> Description {
            match self.components.count() {
                0 => anything().explain_match(actual),
                1 => explanations(&self.components, actual).remove(0),
                _ => {
                    let mut failures = failure_explanations(&self.components, actual);
                    if failures.len() == 1 {
                        failures.remove(0)
                    } else {
                        Description::new().collect(failures).bullet_list()
                    }
                }
            }
        }
    }

    impl<T: Debug + ?Sized, ComponentsT: Components<T>> Describable for AllMatcher<T, ComponentsT> {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            match self.components.count() {
                0 => anything().describe(matcher_result),
                1 => descriptions(&self.components, matcher_result).remove(0),
                _ => {
                    let header = if matcher_result.into() {
                        "has all the following properties:"
                    } else {
                        "has at least one of the following properties:"
                    };
                    Description::new().text(header).nested(
                        Description::new()
                            .bullet_list()
                            .collect(descriptions(&self.components, matcher_result)),
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::__internal;
    use crate::matcher::{Describable as _, Matcher, MatcherResult};
    use crate::prelude::*;
    use alloc::string::String;
    use indoc::indoc;

    #[test]
    fn description_shows_more_than_one_matcher() -> TestResult<()> {
        let first_matcher = starts_with("A");
        let second_matcher = ends_with("string");
        let matcher: __internal::AllMatcher<String, _> = all!(first_matcher, second_matcher);

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq(indoc!(
                "
                has all the following properties:
                  * starts with prefix \"A\"
                  * ends with suffix \"string\""
            )))
        )
    }

    #[test]
    fn description_shows_one_matcher_directly() -> TestResult<()> {
        let first_matcher = starts_with("A");
        let matcher: __internal::AllMatcher<String, _> = all!(first_matcher);

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq("starts with prefix \"A\""))
        )
    }

    #[test]
    fn mismatch_description_shows_which_matcher_failed_if_more_than_one_constituent()
    -> TestResult<()> {
        let first_matcher = starts_with("Another");
        let second_matcher = ends_with("string");
        let matcher: __internal::AllMatcher<str, _> = all!(first_matcher, second_matcher);

        verify_that!(
            matcher.explain_match("A string"),
            displays_as(eq("which does not start with \"Another\""))
        )
    }

    #[test]
    fn mismatch_description_is_simple_when_only_one_consistuent() -> TestResult<()> {
        let first_matcher = starts_with("Another");
        let matcher: __internal::AllMatcher<str, _> = all!(first_matcher);

        verify_that!(
            matcher.explain_match("A string"),
            displays_as(eq("which does not start with \"Another\""))
        )
    }
}
