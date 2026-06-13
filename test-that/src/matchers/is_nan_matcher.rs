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
use num_traits::float::Float;
use std::fmt::Debug;

/// Matches a floating point value which is NaN.
pub fn is_nan() -> __internal::IsNanMatcher {
    __internal::IsNanMatcher
}

pub mod __internal {
    use super::*;

    #[doc(hidden)]
    pub struct IsNanMatcher;

    impl<T: Float + Debug> Matcher<T> for IsNanMatcher {
        fn matches(&self, actual: &T) -> MatcherResult {
            actual.is_nan().into()
        }
    }

    impl Describable for IsNanMatcher {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            if matcher_result.into() { "is NaN" } else { "isn't NaN" }.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::is_nan;
    use crate::prelude::*;

    #[test]
    fn matches_f32_nan() -> TestResult<()> {
        verify_that!(f32::NAN, is_nan())
    }

    #[test]
    fn does_not_match_f32_number() -> TestResult<()> {
        verify_that!(0.0f32, not(is_nan()))
    }

    #[test]
    fn matches_f64_nan() -> TestResult<()> {
        verify_that!(f64::NAN, is_nan())
    }

    #[test]
    fn does_not_match_f64_number() -> TestResult<()> {
        verify_that!(0.0f64, not(is_nan()))
    }
}
