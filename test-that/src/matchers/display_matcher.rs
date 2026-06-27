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
use alloc::string::String;
use core::fmt::{Debug, Display};

/// Matches the string representation of types that implement `Display`.
///
/// ```
/// # use test_that::prelude::*;
/// let result = "Hello, world!";
/// assert_that!(result, displays_as(eq("Hello, world!")));
/// ```
pub fn displays_as<InnerMatcher>(inner: InnerMatcher) -> DisplayMatcher<InnerMatcher> {
    DisplayMatcher { inner, alternate: false }
}

/// A matcher which renders the actual value as a `String` with `Display` and
/// matches the result against the given inner matcher.
pub struct DisplayMatcher<InnerMatcher> {
    inner: InnerMatcher,
    alternate: bool,
}

impl<InnerMatcher> DisplayMatcher<InnerMatcher> {
    /// Indicates that the match should be made against the alternate rendering
    /// of the actual value. That is the result of formatting it with
    /// `{:#}`.
    pub fn alternate(mut self) -> Self {
        self.alternate = true;
        self
    }
}

impl<T: Debug + Display + ?Sized, InnerMatcher: Matcher<String>> Matcher<T>
    for DisplayMatcher<InnerMatcher>
{
    fn matches(&self, actual: &T) -> MatcherResult {
        let rendered = if self.alternate { format!("{actual:#}") } else { format!("{actual}") };
        self.inner.matches(&rendered)
    }

    fn explain_match(&self, actual: &T) -> Description {
        let rendered = if self.alternate { format!("{actual:#}") } else { format!("{actual}") };
        let inner_explanation = self.inner.explain_match(&rendered);
        Description::new()
            .text("which displays as:")
            .nested(Description::from(rendered).indent())
            .append(inner_explanation)
    }
}

impl<InnerMatcher: Describable> Describable for DisplayMatcher<InnerMatcher> {
    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => {
                format!("displays as a string which {}", self.inner.describe(MatcherResult::Match))
                    .into()
            }
            MatcherResult::NoMatch => format!(
                "doesn't display as a string which {}",
                self.inner.describe(MatcherResult::Match)
            )
            .into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::displays_as;
    use crate::prelude::*;
    use core::fmt::{Debug, Display, Error, Formatter};
    use indoc::indoc;
    use serial_test::serial;

    #[test]
    fn display_matches_i32() -> TestResult<()> {
        let value = 32;
        verify_that!(value, displays_as(eq("32")))?;
        Ok(())
    }

    #[test]
    fn display_matches_str() -> TestResult<()> {
        let value = "32";
        verify_that!(value, displays_as(eq("32")))?;
        Ok(())
    }

    #[test]
    fn display_matches_struct() -> TestResult<()> {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct Struct {
            a: i32,
            b: i64,
        }
        impl Display for Struct {
            fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), Error> {
                write!(f, "{:?}", self)
            }
        }
        verify_that!(Struct { a: 123, b: 321 }, displays_as(eq("Struct { a: 123, b: 321 }")))?;
        Ok(())
    }

    #[test]
    fn display_matches_struct_with_alternate_rendering() -> TestResult<()> {
        #[derive(Debug)]
        struct Struct;
        impl Display for Struct {
            fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), Error> {
                if f.alternate() { write!(f, "Correct") } else { write!(f, "Not correct") }
            }
        }

        verify_that!(Struct, displays_as(eq("Correct")).alternate())
    }

    #[cfg(feature = "std")]
    #[test]
    #[serial]
    fn display_displays_error_message_with_explanation_from_inner_matcher() -> TestResult<()> {
        let result = verify_that!("123\n234", displays_as(eq("123\n345")));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                r#"
                  Actual: "123\n234",
                    which displays as:
                      123
                      234
                    which isn't equal to "123\n345"
                    Difference(-actual / +expected):
                     123
                    -234
                    +345
                "#
            ))))
        )
    }

    #[cfg(feature = "std")]
    #[test]
    #[serial]
    fn display_puts_alternate_rendering_in_explanation_when_requested() -> TestResult<()> {
        #[derive(Debug)]
        struct Struct;
        impl Display for Struct {
            fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), Error> {
                if f.alternate() { write!(f, "Correct") } else { write!(f, "Not correct") }
            }
        }

        let result = verify_that!(Struct, displays_as(eq("Not correct")).alternate());

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                r#"
                  Actual: Struct,
                    which displays as:
                      Correct
                    which isn't equal to "Not correct"
                "#
            ))))
        )
    }
}
