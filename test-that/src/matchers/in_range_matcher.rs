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

use core::{marker::PhantomData, ops::RangeBounds};

/// Matches a value contained in the given range.
///
/// The range can be any of the standard range types, including
/// [`Range`][std::ops::Range], [`RangeInclusive`][std::ops::RangeInclusive],
/// [`RangeFrom`][std::ops::RangeFrom], and [`RangeTo`][std::ops::RangeTo]. The
/// underlying type must satisfy the requirements for these types,
/// namely, [`PartialOrd`][std::cmp::PartialOrd].
///
/// The actual value is normally the same type as the range endpoints It can,
/// however, be of any type which can be ordered with respect to the range
/// endpoints.
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(10, in_range(0..20))?;
/// verify_that!(100, not(in_range(0..20)))?;
/// verify_that!(100, in_range(0..=100))?;
/// verify_that!(100, in_range(0..))?;
/// verify_that!(100, in_range(..1000))?;
/// # Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
pub fn in_range<RangeT: RangeBounds<ExpectedT>, ExpectedT>(
    range: RangeT,
) -> __internal::InRangeMatcher<RangeT, ExpectedT> {
    __internal::InRangeMatcher(range, PhantomData)
}

#[doc(hidden)]
mod __internal {
    use crate::matcher::{Describable, Matcher, MatcherResult};
    use core::{fmt::Debug, marker::PhantomData, ops::RangeBounds};

    pub struct InRangeMatcher<RangeT, ExpectedT>(
        pub(super) RangeT,
        pub(super) PhantomData<ExpectedT>,
    );

    impl<
        ExpectedT: Debug + PartialOrd<ActualT>,
        ActualT: Debug + PartialOrd<ExpectedT> + ?Sized,
        RangeT: RangeBounds<ExpectedT> + Debug,
    > Matcher<ActualT> for InRangeMatcher<RangeT, ExpectedT>
    {
        fn matches(&self, actual: &ActualT) -> MatcherResult {
            self.0.contains(actual).into()
        }
    }

    impl<RangeT: Debug, ExpectedT> Describable for InRangeMatcher<RangeT, ExpectedT> {
        fn describe(&self, matcher_result: MatcherResult) -> crate::description::Description {
            match matcher_result {
                MatcherResult::Match => {
                    format!("which is contained in the range {:?}", self.0).into()
                }
                MatcherResult::NoMatch => {
                    format!("which is not contained in the range {:?}", self.0).into()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn matches_within_interval() -> TestResult<()> {
        verify_that!(10, in_range(0..20))
    }

    #[test]
    fn does_not_match_outside_interval() -> TestResult<()> {
        verify_that!(10, not(in_range(0..10)))
    }

    #[test]
    fn matches_at_bound_of_closed_interval() -> TestResult<()> {
        verify_that!(10, in_range(0..=10))
    }

    #[test]
    fn shows_correct_description_when_not_matched() -> TestResult<()> {
        let result = verify_that!(10, in_range(0..10));

        verify_that!(
            result,
            err(displays_as(contains_substring("which is contained in the range 0..10")))
        )
    }

    #[test]
    fn shows_correct_description_when_matched() -> TestResult<()> {
        let result = verify_that!(10, not(in_range(0..=10)));

        verify_that!(
            result,
            err(displays_as(contains_substring("which is not contained in the range 0..=10")))
        )
    }
}
