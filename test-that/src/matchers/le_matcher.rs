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

/// Matches a value less than or equal to (in the sense of `<=`) `expected`.
///
/// The types of the actual and expected values must be comparable via the
/// `PartialOrd` trait. Namely, type of the actual value must implement
/// `PartialOrd<Expected>`, where `Expected` is the type of the expected
/// value passed as an argument to `le`.
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(0, le(0))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> TestResult<()> {
/// verify_that!(1, le(0))?; // Fails
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
/// verify_that!(1u32, le(2u64))?; // Does not compile
/// verify_that!(1u32 as u64, le(2u64))?; // Passes
/// #     Ok(())
/// # }
/// ```
///
/// ```compile_fail
/// # use test_that::prelude::*;
/// # fn should_not_compile() -> TestResult<()> {
/// let actual: &u32 = &1;
/// let expected: u32 = 2;
/// verify_that!(actual, le(expected))?; // Does not compile
/// #     Ok(())
/// # }
/// ```
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// let actual: &u32 = &1;
/// let expected: u32 = 2;
/// verify_that!(actual, le(&expected))?; // Compiles and passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// You can find the standard library `PartialOrd` implementation in
/// <https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html#implementors>
pub fn le<ExpectedT>(expected: ExpectedT) -> __internal::LeMatcher<ExpectedT> {
    __internal::LeMatcher { expected }
}

pub mod __internal {
    use crate::{
        description::Description,
        matcher::{Describable, Matcher, MatcherResult},
    };
    use core::fmt::Debug;

    #[doc(hidden)]
    pub struct LeMatcher<ExpectedT> {
        pub(super) expected: ExpectedT,
    }

    impl<ActualT: Debug + PartialOrd<ExpectedT>, ExpectedT: Debug> Matcher<ActualT>
        for LeMatcher<ExpectedT>
    {
        fn matches(&self, actual: &ActualT) -> MatcherResult {
            (*actual <= self.expected).into()
        }
    }

    impl<ExpectedT: Debug> Describable for LeMatcher<ExpectedT> {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            match matcher_result {
                MatcherResult::Match => {
                    format!("is less than or equal to {:?}", self.expected).into()
                }
                MatcherResult::NoMatch => format!("is greater than {:?}", self.expected).into(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::le;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn le_matches_i32_with_i32() -> TestResult<()> {
        let actual: i32 = 0;
        let expected: i32 = 0;
        verify_that!(actual, le(expected))
    }

    #[test]
    fn le_does_not_match_bigger_i32() -> TestResult<()> {
        let matcher = le(0);
        let result = matcher.matches(&1);
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn le_matches_smaller_str() -> TestResult<()> {
        verify_that!("A", le("B"))
    }

    #[test]
    fn le_does_not_match_bigger_str() -> TestResult<()> {
        let matcher = le("a");
        let result = matcher.matches(&"z");
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn le_mismatch_contains_actual_and_expected() -> TestResult<()> {
        let result = verify_that!(489, le(294));
        let formatted_message = format!("{}", result.unwrap_err());

        verify_that!(
            formatted_message.as_str(),
            contains_substring(indoc!(
                "
                Value of: 489
                Expected: is less than or equal to 294
                Actual: 489,
                  which is greater than 294
                "
            ))
        )
    }

    // Test `le` matcher where actual is `&OsString` and expected is `&str`.
    // Note that stdlib is a little bit inconsistent: `PartialOrd` exists for
    // `OsString` and `str`, but only in one direction: it's only possible to
    // compare `OsString` with `str` if `OsString` is on the left side of the
    // "<=" operator (`impl PartialOrd<str> for OsString`).
    //
    // The comparison in the other direction is not defined.
    //
    // This means that the test case bellow effectively ensures that
    // `verify_that(actual, le(expected))` works if `actual <= expected` works
    // (regardless whether the `expected <= actual` works`).
    #[cfg(feature = "std")]
    #[test]
    fn le_matches_owned_osstring_reference_with_string_reference() -> TestResult<()> {
        use std::ffi::OsString;
        let expected = "B";
        let actual: OsString = "A".into();
        verify_that!(&actual, le(expected))
    }

    #[cfg(feature = "std")]
    #[test]
    fn le_matches_ipv6addr_with_ipaddr() -> TestResult<()> {
        use std::net::IpAddr;
        use std::net::Ipv6Addr;
        let actual: IpAddr = "127.0.0.1".parse().unwrap();
        let expected: Ipv6Addr = "2001:4860:4860::8844".parse().unwrap();
        verify_that!(actual, le(expected))
    }

    #[test]
    fn le_matches_with_custom_partial_ord() -> TestResult<()> {
        /// A custom "number" that is lower than all other numbers. The only
        /// things we define about this "special" number is `PartialOrd` and
        /// `PartialEq` against `u32`.
        #[derive(Debug)]
        struct VeryLowNumber {}

        impl core::cmp::PartialEq<u32> for VeryLowNumber {
            fn eq(&self, _other: &u32) -> bool {
                false
            }
        }

        // PartialOrd (required for >) requires PartialEq.
        impl core::cmp::PartialOrd<u32> for VeryLowNumber {
            fn partial_cmp(&self, _other: &u32) -> Option<core::cmp::Ordering> {
                Some(core::cmp::Ordering::Less)
            }
        }

        impl core::cmp::PartialEq<VeryLowNumber> for u32 {
            fn eq(&self, _other: &VeryLowNumber) -> bool {
                false
            }
        }

        impl core::cmp::PartialOrd<VeryLowNumber> for u32 {
            fn partial_cmp(&self, _other: &VeryLowNumber) -> Option<core::cmp::Ordering> {
                Some(core::cmp::Ordering::Greater)
            }
        }

        let actual = VeryLowNumber {};
        let expected: u32 = 42;

        verify_that!(actual, le(expected))
    }
}
