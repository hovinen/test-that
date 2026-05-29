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
use std::{fmt::Debug, marker::PhantomData};

/// Matches anything. This matcher always succeeds.
///
/// This is useful to check if `actual` matches the specific structure (like
/// `Some(...)`)  but without caring about the internal value.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let option = Some("Some value");
/// verify_that!(option, some(anything()))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
pub fn anything<T: Debug + ?Sized>() -> impl Matcher<T> {
    Anything::<T>(Default::default())
}

struct Anything<T: ?Sized>(PhantomData<T>);

impl<T: Debug + ?Sized> Matcher<T> for Anything<T> {
    fn matches(&self, _: &T) -> MatcherResult {
        MatcherResult::Match
    }
}

impl<T: Debug + ?Sized> Describable for Anything<T> {
    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => "is anything".into(),
            MatcherResult::NoMatch => "never matches".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::anything;
    use crate::prelude::*;

    #[test]
    fn anything_matches_i32() -> Result<()> {
        let value = 32;
        verify_that!(value, anything())?;
        Ok(())
    }

    #[test]
    fn anything_matches_str() -> Result<()> {
        let value = "32";
        verify_that!(value, anything())?;
        Ok(())
    }

    #[test]
    fn anything_matches_option() -> Result<()> {
        let value = Some(32);
        verify_that!(value, some(anything()))?;
        Ok(())
    }
}
