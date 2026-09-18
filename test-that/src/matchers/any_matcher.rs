// Copyright 2023 Google LLC
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

/// Matches a value which at least one of the given matchers match.
///
/// Each argument is a [`Matcher`][crate::matcher::Matcher] which matches
/// against the actual value.
///
/// For example:
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!("A string", any!(starts_with("A"), ends_with("string")))?; // Passes
/// verify_that!("A string", any!(starts_with("A"), starts_with("string")))?; // Passes
/// verify_that!("A string", any!(ends_with("A"), ends_with("string")))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> TestResult<()> {
/// verify_that!("A string", any!(starts_with("An"), ends_with("not a string")))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// Using this macro is equivalent to using the
/// [`or`][crate::matcher::MatcherExt::or] method:
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(10, gt(9).or(lt(8)))?; // Also passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Assertion failure messages are not guaranteed to be identical, however.
#[macro_export]
#[doc(hidden)]
macro_rules! __any {
    ($($matcher:expr),* $(,)?) => {{
        $crate::matchers::__internal::AnyMatcher::new($crate::__matcher_list!($($matcher),*))
    }}
}

/// Functionality needed by the [`any`] macro.
///
/// For internal use only. API stability is not guaranteed!
#[doc(hidden)]
pub mod __internal {
    use super::super::all_matcher::__internal::{
        Components, descriptions, explanations, failure_explanations,
    };
    use crate::description::Description;
    use crate::matcher::{Describable, Matcher, MatcherResult};
    use crate::matchers::anything;
    use core::fmt::Debug;
    use core::marker::PhantomData;

    /// A matcher which matches an input value matched by at least one of its
    /// component matchers.
    ///
    /// For internal use only. API stability is not guaranteed!
    #[doc(hidden)]
    pub struct AnyMatcher<T: Debug + ?Sized, ComponentsT> {
        components: ComponentsT,
        phantom: PhantomData<fn(&T)>,
    }

    impl<T: Debug + ?Sized, ComponentsT> AnyMatcher<T, ComponentsT> {
        /// Constructs an [`AnyMatcher`] with the given component matchers.
        ///
        /// Intended for use only by the [`any`] macro.
        pub fn new(components: ComponentsT) -> Self {
            Self { components, phantom: PhantomData }
        }
    }

    impl<T: Debug + ?Sized, ComponentsT: Components<T>> Matcher<T> for AnyMatcher<T, ComponentsT> {
        fn matches(&self, actual: &T) -> MatcherResult {
            let mut result = MatcherResult::NoMatch;
            self.components.for_each(&mut |component| {
                if component.matches(actual).is_match() {
                    result = MatcherResult::Match;
                }
            });
            result
        }

        fn explain_match(&self, actual: &T) -> Description {
            match self.components.count() {
                0 => format!("which {}", anything().describe(MatcherResult::NoMatch)).into(),
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

    impl<T: Debug + ?Sized, ComponentsT: Components<T>> Describable for AnyMatcher<T, ComponentsT> {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            match self.components.count() {
                0 => anything().describe(matcher_result),
                1 => descriptions(&self.components, matcher_result).remove(0),
                _ => {
                    let properties = descriptions(&self.components, matcher_result)
                        .into_iter()
                        .collect::<Description>()
                        .bullet_list()
                        .indent();
                    format!(
                        "{}:\n{properties}",
                        if matcher_result.into() {
                            "has at least one of the following properties"
                        } else {
                            "has none of the following properties"
                        }
                    )
                    .into()
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
        let matcher: __internal::AnyMatcher<String, _> = any!(first_matcher, second_matcher);

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq(indoc!(
                "
                has at least one of the following properties:
                  * starts with prefix \"A\"
                  * ends with suffix \"string\""
            )))
        )
    }

    #[test]
    fn description_shows_one_matcher_directly() -> TestResult<()> {
        let first_matcher = starts_with("A");
        let matcher: __internal::AnyMatcher<String, _> = any!(first_matcher);

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
        let matcher: __internal::AnyMatcher<str, _> = any!(first_matcher, second_matcher);

        verify_that!(
            matcher.explain_match("A string"),
            displays_as(eq("which does not start with \"Another\""))
        )
    }

    #[test]
    fn mismatch_description_is_simple_when_only_one_constituent() -> TestResult<()> {
        let first_matcher = starts_with("Another");
        let matcher: __internal::AnyMatcher<str, _> = any!(first_matcher);

        verify_that!(
            matcher.explain_match("A string"),
            displays_as(eq("which does not start with \"Another\""))
        )
    }
}
