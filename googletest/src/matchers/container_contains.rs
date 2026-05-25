#![doc(hidden)]

use crate::{description::Description, matcher_support::count_elements::count_elements};
use std::fmt::Display;

/// Marker: container yields `&T` items. See [`ContainerContainsOrderedMatcher`].
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub struct RefItems;

/// Marker: container yields owned `T` items. See [`ContainerContainsOrderedMatcher`].
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub struct OwnedItems;

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
