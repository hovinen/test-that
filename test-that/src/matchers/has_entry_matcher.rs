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

/// Matches a map-like collection containing the given `key` whose value is
/// matched by the matcher `inner`.
///
/// This works in particular with [`HashMap`][std::collections::HashMap]:
///
/// ```
/// # use test_that::prelude::*;
/// # use std::collections::HashMap;
/// # fn should_pass() -> TestResult<()> {
/// let value = HashMap::from([(0, 1), (1, -1)]);
/// verify_that!(value, has_entry(0, eq(1)))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> TestResult<()> {
/// # let value = HashMap::from([(0, 1), (1, -1)]);
/// verify_that!(value, has_entry(1, gt(0)))?;  // Fails: value not matched
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> TestResult<()> {
/// # let value = HashMap::from([(0, 1), (1, -1)]);
/// verify_that!(value, has_entry(2, eq(0)))?;  // Fails: key not present
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// ```
///
/// This also works with [`BTreeMap`][alloc::collections::BTreeMap]:
///
/// ```
/// # use test_that::prelude::*;
/// # use std::collections::BTreeMap;
/// # fn should_pass() -> TestResult<()> {
/// let mut value = BTreeMap::new();
/// value.insert(0, 1);
/// value.insert(1, -1);
/// verify_that!(value, has_entry(0, eq(1)))?;  // Passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// In general, this matches any collection `M` such that `&M` iterates over
/// key-value pairs `(&K, &V)`.
///
/// Note: One could obtain the same effect by collecting entries into a `Vec`
/// and using `contains`:
///
/// ```
/// # use test_that::prelude::*;
/// # use std::collections::HashMap;
/// # fn should_pass() -> TestResult<()> {
/// let value = HashMap::from([(0, 1), (1, -1)]);
/// verify_that!(value.into_iter().collect::<Vec<_>>(), contains(eq((0, 1))))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// However, `has_entry` will offer somewhat better diagnostic messages in the
/// case of assertion failure. And it avoids the extra allocation hidden in the
/// code above.
///
/// **Note:** In order to be generic over the type of map, this uses an O(n)
/// linear scan over all key-value pairs. It is therefore not performant with
/// large maps. If the test requires efficient access to map elements, consider
/// using [`result_of!`][crate::matchers::result_of] instead:
///
/// ```
/// # use test_that::prelude::*;
/// # use std::collections::HashMap;
/// let value = HashMap::from([(0, 1), (1, -1)]);
/// verify_that!(
///     value,
///     result_of!(|value: &HashMap<i32, i32>| value.get(&0), some(points_to(eq(1))))
/// )
/// # .unwrap();
/// ```
pub fn has_entry<KeyT, MatcherT>(
    key: KeyT,
    inner: MatcherT,
) -> __internal::HasEntryMatcher<KeyT, MatcherT> {
    __internal::HasEntryMatcher { key, inner }
}

pub mod __internal {
    use crate::description::Description;
    use crate::matcher::{Describable, Matcher, MatcherResult};
    use core::fmt::Debug;

    #[doc(hidden)]
    pub struct HasEntryMatcher<KeyT, MatcherT> {
        pub(super) key: KeyT,
        pub(super) inner: MatcherT,
    }

    impl<KeyT: Debug + PartialEq, ValueT: Debug, MatcherT: Matcher<ValueT>, MapT: Debug>
        Matcher<MapT> for HasEntryMatcher<KeyT, MatcherT>
    where
        for<'a> &'a MapT: IntoIterator<Item = (&'a KeyT, &'a ValueT)>,
    {
        fn matches(&self, actual: &MapT) -> MatcherResult {
            if let Some(value) = find_value(actual, &self.key) {
                self.inner.matches(value)
            } else {
                MatcherResult::NoMatch
            }
        }

        fn explain_match(&self, actual: &MapT) -> Description {
            if let Some(value) = find_value(actual, &self.key) {
                format!(
                    "which contains key {:?}, but is mapped to value {:#?}, {}",
                    self.key,
                    value,
                    self.inner.explain_match(value)
                )
                .into()
            } else {
                format!("which doesn't contain key {:?}", self.key).into()
            }
        }
    }

    fn find_value<'a, KeyT: PartialEq + 'a, ValueT, MapT>(
        map: &'a MapT,
        key: &KeyT,
    ) -> Option<&'a ValueT>
    where
        &'a MapT: IntoIterator<Item = (&'a KeyT, &'a ValueT)>,
    {
        map.into_iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    impl<KeyT: Debug, MatcherT: Describable> Describable for HasEntryMatcher<KeyT, MatcherT> {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            match matcher_result {
                MatcherResult::Match => format!(
                    "contains key {:?}, which value {}",
                    self.key,
                    self.inner.describe(MatcherResult::Match)
                )
                .into(),
                MatcherResult::NoMatch => format!(
                    "doesn't contain key {:?} or contains key {:?}, which value {}",
                    self.key,
                    self.key,
                    self.inner.describe(MatcherResult::NoMatch)
                )
                .into(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::has_entry;
    use crate::prelude::*;
    use alloc::collections::BTreeMap;
    use indoc::indoc;

    #[cfg(feature = "std")]
    use std::collections::HashMap;

    #[test]
    fn has_entry_does_not_match_empty_btree_map() -> TestResult<()> {
        let value: BTreeMap<i32, i32> = BTreeMap::new();
        verify_that!(value, not(has_entry(0, eq(0))))
    }

    #[test]
    fn has_entry_matches_btree_map_with_value() -> TestResult<()> {
        let value = BTreeMap::from([(0, 0)]);
        verify_that!(value, has_entry(0, eq(0)))
    }

    #[test]
    fn has_entry_does_not_match_btree_map_with_wrong_value() -> TestResult<()> {
        let value = BTreeMap::from([(0, 1)]);
        verify_that!(value, not(has_entry(0, eq(0))))
    }

    #[test]
    fn has_entry_does_not_match_btree_map_with_wrong_key() -> TestResult<()> {
        let value = BTreeMap::from([(1, 0)]);
        verify_that!(value, not(has_entry(0, eq(0))))
    }

    #[cfg(feature = "std")]
    #[test]
    fn has_entry_does_not_match_empty_hash_map() -> TestResult<()> {
        let value: HashMap<i32, i32> = HashMap::new();
        verify_that!(value, not(has_entry(0, eq(0))))
    }

    #[cfg(feature = "std")]
    #[test]
    fn has_entry_matches_hash_map_with_value() -> TestResult<()> {
        let value: HashMap<i32, i32> = HashMap::from([(0, 0)]);
        verify_that!(value, has_entry(0, eq(0)))
    }

    #[cfg(feature = "std")]
    #[test]
    fn has_entry_does_not_match_hash_map_with_wrong_value() -> TestResult<()> {
        let value: HashMap<i32, i32> = HashMap::from([(0, 1)]);
        verify_that!(value, not(has_entry(0, eq(0))))
    }

    #[cfg(feature = "std")]
    #[test]
    fn has_entry_does_not_match_hash_map_with_wrong_key() -> TestResult<()> {
        let value: HashMap<i32, i32> = HashMap::from([(1, 0)]);
        verify_that!(value, not(has_entry(0, eq(0))))
    }

    #[test]
    fn has_entry_shows_correct_message_when_key_is_not_present() -> TestResult<()> {
        let result = verify_that!(BTreeMap::from([(0, 0)]), has_entry(1, eq(0)));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Expected: contains key 1, which value is equal to 0
                Actual: {0: 0},
                  which doesn't contain key 1
                "
            ))))
        )
    }

    #[test]
    fn has_entry_shows_correct_message_when_key_has_non_matching_value() -> TestResult<()> {
        let result = verify_that!(BTreeMap::from([(0, 0)]), has_entry(0, eq(1)));

        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Expected: contains key 0, which value is equal to 1
                Actual: {0: 0},
                  which contains key 0, but is mapped to value 0, which isn't equal to 1
                "
            ))))
        )
    }
}
