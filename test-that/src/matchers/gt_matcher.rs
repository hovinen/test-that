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

/// Matches a value greater (in the sense of `>`) than `expected`.
///
/// The types of the actual and expected values must be comparable via the
/// `PartialOrd` trait. Namely, type of the actual value must implement
/// `PartialOrd<Expected>`, where `Expected` is the type of the expected
/// value passed as an argument to `gt`.
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(38, gt(1))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> TestResult<()> {
/// verify_that!(234, gt(234))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// In most cases the params neeed to be the same type or they need to be cast
/// explicitly. This can be surprising when comparing integer types or
/// references:
///
/// ```compile_fail
/// # use test_that::prelude::*;
/// # fn should_not_compile() -> TestResult<()> {
/// verify_that!(123u32, gt(0u64))?; // Does not compile
/// verify_that!(123u32 as u64, gt(0u64))?; // Passes
/// #     Ok(())
/// # }
/// ```
///
/// ```compile_fail
/// # use test_that::prelude::*;
/// # fn should_not_compile() -> TestResult<()> {
/// let actual: &u32 = &2;
/// let expected: u32 = 1;
/// verify_that!(actual, gt(expected))?; // Does not compile
/// #     Ok(())
/// # }
/// ```
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// let actual: &u32 = &2;
/// let expected: u32 = 1;
/// verify_that!(actual, gt(&expected))?; // Compiles and passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// You can find the standard library `PartialOrd` implementation in
/// <https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html#implementors>
pub fn gt<ExpectedT>(expected: ExpectedT) -> GtMatcher<ExpectedT> {
    GtMatcher { expected }
}

#[doc(hidden)]
pub struct GtMatcher<ExpectedT> {
    expected: ExpectedT,
}

impl<ActualT: Debug + PartialOrd<ExpectedT>, ExpectedT: Debug> Matcher<ActualT>
    for GtMatcher<ExpectedT>
{
    fn matches(&self, actual: &ActualT) -> MatcherResult {
        (*actual > self.expected).into()
    }
}

impl<ExpectedT: Debug> Describable for GtMatcher<ExpectedT> {
    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!("is greater than {:?}", self.expected).into(),
            MatcherResult::NoMatch => {
                format!("is less than or equal to {:?}", self.expected).into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::gt;
    use crate::matcher::{Describable as _, Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;
    use std::ffi::OsString;

    #[test]
    fn gt_matches_i32_with_i32() -> TestResult<()> {
        let actual: i32 = 321;
        let expected: i32 = 123;
        verify_that!(actual, gt(expected))
    }

    #[test]
    fn gt_does_not_match_equal_i32() -> TestResult<()> {
        let matcher = gt(10);
        let result = matcher.matches(&10);
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn gt_does_not_match_lower_i32() -> TestResult<()> {
        let matcher = gt(-50);
        let result = matcher.matches(&-51);
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn gt_matches_greater_str() -> TestResult<()> {
        verify_that!("B", gt("A"))
    }

    #[test]
    fn gt_does_not_match_lesser_str() -> TestResult<()> {
        let matcher = gt("B");
        let result = matcher.matches(&"A");
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn gt_mismatch_contains_actual_and_expected() -> TestResult<()> {
        let result = verify_that!(481, gt(632));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: 481
                Expected: is greater than 632
                Actual: 481,
                "
            ))))
        )
    }

    #[test]
    fn gt_mismatch_combined_with_each() -> TestResult<()> {
        let result = verify_that!(vec![19, 23, 11], each(gt(15)));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: vec![19, 23, 11]
                Expected: only contains elements that is greater than 15
                Actual: [19, 23, 11],
                  whose element #2 is 11, which is less than or equal to 15
                "
            ))))
        )
    }

    #[test]
    fn gt_describe_matches() -> TestResult<()> {
        verify_that!(gt(232).describe(MatcherResult::Match), displays_as(eq("is greater than 232")))
    }

    #[test]
    fn gt_describe_does_not_match() -> TestResult<()> {
        verify_that!(
            gt(232).describe(MatcherResult::NoMatch),
            displays_as(eq("is less than or equal to 232"))
        )
    }

    // Test `gt` matcher where actual is `&OsString` and expected is `&str`.
    // Note that stdlib is a little bit inconsistent: `PartialOrd` exists for
    // `OsString` and `str`, but only in one direction: it's only possible to
    // compare `OsString` with `str` if `OsString` is on the left side of the
    // ">" operator (`impl PartialOrd<str> for OsString`).
    //
    // The comparison in the other direction is not defined.
    //
    // This means that the test case bellow effectively ensures that
    // `verify_that(actual, gt(expected))` works if `actual > expected` works
    // (regardless whether the `expected > actual` works`).
    #[test]
    fn gt_matches_owned_osstring_reference_with_string_reference() -> TestResult<()> {
        let expected = "A";
        let actual: OsString = "B".to_string().into();
        verify_that!(&actual, gt(expected))
    }

    #[test]
    fn gt_matches_ipv6addr_with_ipaddr() -> TestResult<()> {
        use std::net::IpAddr;
        use std::net::Ipv6Addr;
        let actual: Ipv6Addr = "2001:4860:4860::8888".parse().unwrap();
        let expected: IpAddr = "127.0.0.1".parse().unwrap();
        verify_that!(actual, gt(expected))
    }

    #[test]
    fn gt_matches_with_custom_partial_ord() -> TestResult<()> {
        /// A custom "number" that is smaller than all other numbers. The only
        /// things we define about this "special" number is `PartialOrd` and
        /// `PartialEq` against `u32`.
        #[derive(Debug)]
        struct VeryLowNumber {}

        impl std::cmp::PartialEq<u32> for VeryLowNumber {
            fn eq(&self, _other: &u32) -> bool {
                false
            }
        }

        // PartialOrd (required for >) requires PartialEq.
        impl std::cmp::PartialOrd<u32> for VeryLowNumber {
            fn partial_cmp(&self, _other: &u32) -> Option<std::cmp::Ordering> {
                Some(std::cmp::Ordering::Less)
            }
        }

        impl std::cmp::PartialEq<VeryLowNumber> for u32 {
            fn eq(&self, _other: &VeryLowNumber) -> bool {
                false
            }
        }

        impl std::cmp::PartialOrd<VeryLowNumber> for u32 {
            fn partial_cmp(&self, _other: &VeryLowNumber) -> Option<std::cmp::Ordering> {
                Some(std::cmp::Ordering::Greater)
            }
        }

        let actual: u32 = 42;
        let expected = VeryLowNumber {};

        verify_that!(actual, gt(expected))
    }
}
