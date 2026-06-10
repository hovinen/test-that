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

#![doc(hidden)]

use crate::{description::Description, matcher_support::count_elements::count_elements};
use std::fmt::Display;

/// Abstracts over map iterator items, allowing both reference pairs `(&K, &V)`
/// and owned pairs `(K, V)` to be matched uniformly.
///
/// **For internal use only. API stability is not guaranteed!**
#[doc(hidden)]
pub trait PairBorrow<K, V> {
    fn borrow_key(&self) -> &K;
    fn borrow_value(&self) -> &V;
}

impl<K, V> PairBorrow<K, V> for (&K, &V) {
    fn borrow_key(&self) -> &K {
        self.0
    }
    fn borrow_value(&self) -> &V {
        self.1
    }
}

impl<K, V> PairBorrow<K, V> for (K, V) {
    fn borrow_key(&self) -> &K {
        &self.0
    }
    fn borrow_value(&self) -> &V {
        &self.1
    }
}

/// The requirements of the mapping between matchers and actual values by
/// which [`UnorderedElemetnsAre`] is deemed to match its input.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum Requirements {
    /// There must be a 1:1 correspondence between the actual values and the
    /// matchers.
    PerfectMatch,

    /// The mapping from matched actual values to their corresponding
    /// matchers must be surjective.
    Superset,

    /// The mapping from matchers to matched actual values must be
    /// surjective.
    Subset,
}

impl Requirements {
    pub(crate) fn explain_size_mismatch<ContainerT: ?Sized>(
        &self,
        actual: &ContainerT,
        expected_size: usize,
    ) -> Option<Description>
    where
        for<'b> &'b ContainerT: IntoIterator,
    {
        let actual_size = count_elements(actual);
        match self {
            Requirements::PerfectMatch if actual_size != expected_size => {
                Some(format!("which has size {} (expected {})", actual_size, expected_size).into())
            }

            Requirements::Superset if actual_size < expected_size => Some(
                format!("which has size {} (expected at least {})", actual_size, expected_size)
                    .into(),
            ),

            Requirements::Subset if actual_size > expected_size => Some(
                format!("which has size {} (expected at most {})", actual_size, expected_size)
                    .into(),
            ),

            _ => None,
        }
    }
}

impl Display for Requirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Requirements::PerfectMatch => {
                write!(f, "perfect")
            }
            Requirements::Superset => {
                write!(f, "superset")
            }
            Requirements::Subset => {
                write!(f, "subset")
            }
        }
    }
}

/// Marker for containers whose iterator over `&ContainerT` yields `&T` items.
///
/// **For internal use only. API stability is not guaranteed!**
#[doc(hidden)]
pub struct RefItems;

/// Marker for containers whose iterator over `&ContainerT` yields owned `T`
/// items.
///
/// **For internal use only. API stability is not guaranteed!**
#[doc(hidden)]
pub struct OwnedItems;
