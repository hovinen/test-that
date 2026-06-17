// Copyright 2022 Google LLC
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

/// Matches a floating point value which is infinite.
pub fn is_infinite() -> __internal::IsInfiniteMatcher {
    __internal::IsInfiniteMatcher
}

pub mod __internal {
    use crate::{
        description::Description,
        matcher::{Describable, Matcher, MatcherResult},
    };
    use core::fmt::Debug;
    use num_traits::float::Float;

    #[doc(hidden)]
    pub struct IsInfiniteMatcher;

    impl<T: Float + Debug> Matcher<T> for IsInfiniteMatcher {
        fn matches(&self, actual: &T) -> MatcherResult {
            actual.is_infinite().into()
        }
    }

    impl Describable for IsInfiniteMatcher {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            if matcher_result.into() { "is infinite" } else { "is finite or NaN" }.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::is_infinite;
    use crate::prelude::*;

    #[test]
    fn matches_f32_infinity() -> TestResult<()> {
        verify_that!(f32::INFINITY, is_infinite())
    }

    #[test]
    fn does_not_match_f32_number() -> TestResult<()> {
        verify_that!(0.0f32, not(is_infinite()))
    }

    #[test]
    fn does_not_match_f32_nan() -> TestResult<()> {
        verify_that!(f32::NAN, not(is_infinite()))
    }

    #[test]
    fn matches_f64_infinity() -> TestResult<()> {
        verify_that!(f64::INFINITY, is_infinite())
    }

    #[test]
    fn does_not_match_f64_number() -> TestResult<()> {
        verify_that!(0.0f64, not(is_infinite()))
    }

    #[test]
    fn does_not_match_f64_nan() -> TestResult<()> {
        verify_that!(f64::NAN, not(is_infinite()))
    }
}
