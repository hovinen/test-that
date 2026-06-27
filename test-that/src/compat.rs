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

//! Aliases to ease porting from [GoogleTest Rust](https://docs.rs/googletest).

use crate::OrFailExt;

/// Alias for [`contains_exactly!`] with [`in_order()`].
///
/// This exists to ease porting from [googletest].
///
/// [`contains_exactly!`]: crate::matchers::containers::contains_exactly
/// [`in_order()`]: crate::matchers::containers::ContainerContainsUnorderedMatcher::in_order
/// [googletest]: https://docs.rs/googletest
#[cfg_attr(
    feature = "googletest-migrate",
    deprecated(note = "Use contains_exactly![...].in_order() instead")
)]
#[macro_export]
#[doc(hidden)]
macro_rules! __elements_are {
    ($($content:tt)*) => {{
        $crate::matchers::containers::contains_exactly![$($content)*].in_order()
    }}
}

/// Alias for [`contains_exactly!`].
///
/// This exists to ease porting from [googletest].
///
/// [`contains_exactly!`]: crate::matchers::containers::contains_exactly
/// [googletest]: https://docs.rs/googletest
#[cfg_attr(feature = "googletest-migrate", deprecated(note = "Use contains_exactly![...] instead"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __unordered_elements_are {
    ($(,)?) => {{
        $crate::matchers::__internal::ContainerContainsUnorderedMatcher::new(
            [],
            $crate::matchers::__internal::Requirements::PerfectMatch
        )
    }};

    ($(($key_matcher:expr, $value_matcher:expr)),* $(,)?) => {{
        $crate::matchers::__internal::MapContainsMatcher::new(
            [$(($crate::__alloc::boxed::Box::new($key_matcher), $crate::__alloc::boxed::Box::new($value_matcher))),*],
            $crate::matchers::__internal::Requirements::PerfectMatch
        )
    }};

    ($($matcher:expr),* $(,)?) => {{
        $crate::matchers::__internal::ContainerContainsUnorderedMatcher::new(
            vec![$($crate::__alloc::boxed::Box::new($matcher)),*],
            $crate::matchers::__internal::Requirements::PerfectMatch
        )
    }};
}

/// Alias for [OrFailExt].
#[cfg_attr(feature = "googletest-migrate", deprecated(note = "Use OrFailExt instead"))]
pub trait IntoTestResult<T> {
    /// Alias for [`or_fail`][crate::OrFailExt::or_fail].
    #[cfg_attr(feature = "googletest-migrate", deprecated(note = "Use or_fail() instead"))]
    fn into_test_result(self) -> crate::TestResult<T>;
}

#[allow(deprecated)]
impl<S: OrFailExt<T>, T> IntoTestResult<T> for S {
    fn into_test_result(self) -> crate::TestResult<T> {
        self.or_fail()
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use crate::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn elements_are_maps_to_contains_exactly_in_order() -> TestResult<()> {
        verify_that!(vec![1, 2, 3], elements_are![eq(1), eq(2), eq(3)])?;
        verify_that!(vec![1, 2, 3], not(elements_are![eq(3), eq(2), eq(1)]))
    }

    #[test]
    fn unordered_elements_are_maps_to_contains_exactly() -> TestResult<()> {
        verify_that!(vec![1, 2, 3], unordered_elements_are![eq(1), eq(2), eq(3)])?;
        verify_that!(vec![1, 2, 3], unordered_elements_are![eq(3), eq(2), eq(1)])
    }

    #[test]
    fn unordered_elements_are_uses_map_matcher_when_matching_pairs() -> TestResult<()> {
        verify_that!(HashMap::from([(1, 1)]), unordered_elements_are![(eq(1), eq(1))])
    }

    #[test]
    #[cfg(feature = "anyhow")]
    fn into_test_result_works() -> TestResult<()> {
        let _ = Err::<(), anyhow::Error>(anyhow::anyhow!("Expected failure")).into_test_result();
        Ok(())
    }
}
