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

use core::ops::Deref;
use regex::Regex;

/// Matches a string containing a substring which matches the given regular
/// expression.
///
/// Both the actual value and the expected regular expression may be either a
/// `String` or a string reference.
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass_1() -> TestResult<()> {
/// verify_that!("Some value", contains_regex("S.*e"))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> TestResult<()> {
/// verify_that!("Another value", contains_regex("Some"))?;   // Fails
/// #     Ok(())
/// # }
/// # fn should_pass_2() -> TestResult<()> {
/// verify_that!("Some value".to_string(), contains_regex("v.*e"))?;   // Passes
/// verify_that!("Some value", contains_regex("v.*e".to_string()))?;   // Passes
/// #     Ok(())
/// # }
/// # should_pass_1().unwrap();
/// # should_fail().unwrap_err();
/// # should_pass_2().unwrap();
/// ```
///
/// Panics if the given `pattern` is not a syntactically valid regular
/// expression.
// N.B. This returns the concrete type rather than an impl Matcher so that it
// can act simultaneously as a Matcher<str> and a Matcher<String>. Otherwise the
// compiler treats it as a Matcher<str> only and the code
//   verify_that!("Some value".to_string(), contains_regex(".*value"))?;
// doesn't compile.
pub fn contains_regex<PatternT: Deref<Target = str>>(
    pattern: PatternT,
) -> __internal::ContainsRegexMatcher {
    __internal::ContainsRegexMatcher { regex: Regex::new(pattern.deref()).unwrap() }
}

pub mod __internal {
    use crate::description::Description;
    use crate::matcher::{Describable, Matcher, MatcherResult};
    use core::fmt::Debug;
    use regex::Regex;

    #[doc(hidden)]
    pub struct ContainsRegexMatcher {
        pub(super) regex: Regex,
    }

    impl<ActualT: AsRef<str> + Debug + ?Sized> Matcher<ActualT> for ContainsRegexMatcher {
        fn matches(&self, actual: &ActualT) -> MatcherResult {
            self.regex.is_match(actual.as_ref()).into()
        }
    }

    impl Describable for ContainsRegexMatcher {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            match matcher_result {
                MatcherResult::Match => {
                    format!("contains the regular expression {:#?}", self.regex.as_str()).into()
                }
                MatcherResult::NoMatch => {
                    format!("doesn't contain the regular expression {:#?}", self.regex.as_str())
                        .into()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::contains_regex;
    use crate::matcher::{Describable as _, Matcher, MatcherResult};
    use crate::prelude::*;
    use alloc::string::ToString;

    #[test]
    fn contains_regex_matches_string_reference_with_pattern() -> TestResult<()> {
        let matcher = contains_regex("S.*val");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_regex_does_not_match_string_without_pattern() -> TestResult<()> {
        let matcher = contains_regex("Another");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_regex_matches_owned_string_with_pattern() -> TestResult<()> {
        let matcher = contains_regex("value");

        let result = matcher.matches(&"Some value".to_string());

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_regex_matches_string_reference_with_owned_string() -> TestResult<()> {
        let matcher = contains_regex("value");

        let result = matcher.matches("Some value");

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn verify_that_works_with_owned_string() -> TestResult<()> {
        verify_that!("Some value".to_string(), contains_regex("value"))
    }

    #[test]
    fn contains_regex_displays_quoted_debug_of_pattern() -> TestResult<()> {
        let matcher = contains_regex("\n");

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq("contains the regular expression \"\\n\""))
        )
    }
}
