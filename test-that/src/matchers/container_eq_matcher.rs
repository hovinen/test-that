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

use crate::description::Description;
use crate::matcher::{Describable, Matcher, MatcherResult};
use crate::matchers::container_contains::{OwnedItems, RefItems};
use std::fmt::Debug;
use std::marker::PhantomData;

/// Matches a container equal (in the sense of `==`) to `expected`.
///
/// This is similar to [`crate::matchers::eq`] except that an assertion failure
/// message generated from this matcher will include the missing and unexpected
/// items in the actual value, e.g.:
///
/// ```text
/// Expected container to equal [1, 2, 3]
///   but was: [1, 2, 4]
///   Missing: [3]
///   Unexpected: [4]
/// ```
///
/// The actual value must be a container such as a `Vec`, an array, or a
/// dereferenced slice. More precisely, a shared borrow of the actual value must
/// implement [`IntoIterator`] whose `Item` type implements
/// [`PartialEq<ExpectedT>`], where `ExpectedT` is the element type of the
/// expected value.
///
/// If the container type is a `Vec`, then the expected type may be a slice of
/// the same element type. For example:
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let vec = vec![1, 2, 3];
/// verify_that!(vec, container_eq([1, 2, 3]))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// As an exception, if the actual type is a `Vec<String>`, the expected type
/// may be a slice of `&str`:
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let vec: Vec<String> = vec!["A string".into(), "Another string".into()];
/// verify_that!(vec, container_eq(["A string", "Another string"]))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// These exceptions allow one to avoid unnecessary allocations in test
/// assertions.
///
/// One can also check container equality of a slice with an array. To do so,
/// dereference the slice:
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> Result<()> {
/// let value = &[1, 2, 3];
/// verify_that!(*value, container_eq([1, 2, 3]))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Otherwise, the actual and expected types must be identical.
///
/// *Performance note*: In the event of a mismatch leading to an assertion
/// failure, the construction of the lists of missing and unexpected values
/// uses a naive algorithm requiring time proportional to the product of the
/// sizes of the expected and actual values. This should therefore only be used
/// when the containers are small enough that this is not a problem.
// This returns ContainerEqMatcher and not impl Matcher because
// ContainerEqMatcher has some specialisations for slice types (see
// documentation above). Returning impl Matcher would hide those from the
// compiler.
pub fn container_eq<ExpectedContainerT, Mode>(
    expected: ExpectedContainerT,
) -> ContainerEqMatcher<ExpectedContainerT, Mode>
where
    ExpectedContainerT: Debug,
{
    ContainerEqMatcher { expected, phantom: Default::default() }
}

pub struct ContainerEqMatcher<ExpectedContainerT, Mode> {
    expected: ExpectedContainerT,
    phantom: PhantomData<Mode>,
}

impl<ActualElementT, ActualContainerT, ExpectedElementT, ExpectedContainerT>
    Matcher<ActualContainerT> for ContainerEqMatcher<ExpectedContainerT, RefItems>
where
    ActualElementT: PartialEq<ExpectedElementT> + Debug + ?Sized,
    ActualContainerT: PartialEq<ExpectedContainerT> + Debug + ?Sized,
    ExpectedElementT: Debug,
    ExpectedContainerT: Debug,
    for<'a> &'a ActualContainerT: IntoIterator<Item = &'a ActualElementT>,
    for<'a> &'a ExpectedContainerT: IntoIterator<Item = &'a ExpectedElementT>,
{
    fn matches(&self, actual: &ActualContainerT) -> MatcherResult {
        (*actual == self.expected).into()
    }

    fn explain_match(&self, actual: &ActualContainerT) -> Description {
        build_explanation(self.get_missing_items(actual), self.get_unexpected_items(actual)).into()
    }
}

impl<ExpectedElementT, ExpectedContainerT> ContainerEqMatcher<ExpectedContainerT, RefItems>
where
    for<'a> &'a ExpectedContainerT: IntoIterator<Item = &'a ExpectedElementT>,
{
    fn get_missing_items<ActualElementT, ActualContainerT>(
        &self,
        actual: &ActualContainerT,
    ) -> Vec<&ExpectedElementT>
    where
        ActualElementT: PartialEq<ExpectedElementT> + ?Sized,
        ActualContainerT: PartialEq<ExpectedContainerT> + ?Sized,
        for<'a> &'a ActualContainerT: IntoIterator<Item = &'a ActualElementT>,
    {
        self.expected.into_iter().filter(|&i| !actual.into_iter().any(|j| j == i)).collect()
    }

    fn get_unexpected_items<'a, ActualElementT, ActualContainerT>(
        &self,
        actual: &'a ActualContainerT,
    ) -> Vec<&'a ActualElementT>
    where
        ActualElementT: PartialEq<ExpectedElementT> + ?Sized,
        ActualContainerT: PartialEq<ExpectedContainerT> + ?Sized,
        for<'b> &'b ActualContainerT: IntoIterator<Item = &'b ActualElementT>,
    {
        actual.into_iter().filter(|&i| !self.expected.into_iter().any(|j| i == j)).collect()
    }
}

impl<ActualElementT, ActualContainerT, ExpectedElementT, ExpectedContainerT>
    Matcher<ActualContainerT> for ContainerEqMatcher<ExpectedContainerT, OwnedItems>
where
    ActualElementT: PartialEq<ExpectedElementT> + Debug,
    ActualContainerT: PartialEq<ExpectedContainerT> + Debug + ?Sized,
    ExpectedElementT: Debug,
    ExpectedContainerT: Debug,
    for<'a> &'a ActualContainerT: IntoIterator<Item = ActualElementT>,
    for<'a> &'a ExpectedContainerT: IntoIterator<Item = ExpectedElementT>,
{
    fn matches(&self, actual: &ActualContainerT) -> MatcherResult {
        (*actual == self.expected).into()
    }

    fn explain_match(&self, actual: &ActualContainerT) -> Description {
        build_explanation(self.get_missing_items(actual), self.get_unexpected_items(actual)).into()
    }
}

impl<ExpectedElementT, ExpectedContainerT> ContainerEqMatcher<ExpectedContainerT, OwnedItems>
where
    for<'a> &'a ExpectedContainerT: IntoIterator<Item = ExpectedElementT>,
{
    fn get_missing_items<ActualElementT, ActualContainerT>(
        &self,
        actual: &ActualContainerT,
    ) -> Vec<ExpectedElementT>
    where
        ActualElementT: PartialEq<ExpectedElementT>,
        ActualContainerT: PartialEq<ExpectedContainerT> + ?Sized,
        for<'a> &'a ActualContainerT: IntoIterator<Item = ActualElementT>,
    {
        self.expected.into_iter().filter(|i| !actual.into_iter().any(|j| j == *i)).collect()
    }

    fn get_unexpected_items<'a, ActualElementT, ActualContainerT>(
        &self,
        actual: &'a ActualContainerT,
    ) -> Vec<ActualElementT>
    where
        ActualElementT: PartialEq<ExpectedElementT>,
        ActualContainerT: PartialEq<ExpectedContainerT> + ?Sized,
        for<'b> &'b ActualContainerT: IntoIterator<Item = ActualElementT>,
    {
        actual.into_iter().filter(|i| !self.expected.into_iter().any(|j| *i == j)).collect()
    }
}

impl<ExpectedContainerT: Debug, Mode> Describable for ContainerEqMatcher<ExpectedContainerT, Mode> {
    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!("is equal to {:?}", self.expected).into(),
            MatcherResult::NoMatch => format!("isn't equal to {:?}", self.expected).into(),
        }
    }
}

fn build_explanation<T: Debug, U: Debug>(missing: Vec<T>, unexpected: Vec<U>) -> String {
    match (missing.len(), unexpected.len()) {
        // TODO(b/261175849) add more data here (out of order elements, duplicated elements, etc...)
        (0, 0) => "which contains all the elements".to_string(),
        (0, 1) => format!("which contains the unexpected element {:?}", unexpected[0]),
        (0, _) => format!("which contains the unexpected elements {unexpected:?}",),
        (1, 0) => format!("which is missing the element {:?}", missing[0]),
        (1, 1) => {
            format!(
                "which is missing the element {:?} and contains the unexpected element {:?}",
                missing[0], unexpected[0]
            )
        }
        (1, _) => {
            format!(
                "which is missing the element {:?} and contains the unexpected elements {unexpected:?}",
                missing[0]
            )
        }
        (_, 0) => format!("which is missing the elements {missing:?}"),
        (_, 1) => {
            format!(
                "which is missing the elements {missing:?} and contains the unexpected element {:?}",
                unexpected[0]
            )
        }
        (_, _) => {
            format!(
                "which is missing the elements {missing:?} and contains the unexpected elements {unexpected:?}",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use indoc::indoc;
    use std::collections::HashSet;

    #[test]
    fn container_eq_returns_match_when_containers_match() -> Result<()> {
        verify_that!(vec![1, 2, 3], container_eq(vec![1, 2, 3]))
    }

    #[test]
    fn container_eq_matches_array_with_array() -> Result<()> {
        verify_that!([1, 2, 3], container_eq([1, 2, 3]))
    }

    #[test]
    fn container_eq_matches_vec_with_array() -> Result<()> {
        verify_that!(vec![1, 2, 3], container_eq([1, 2, 3]))
    }

    #[test]
    fn container_eq_matches_slice_using_points_to() -> Result<()> {
        let value = vec![1, 2, 3];
        let slice = value.as_slice();
        verify_that!(slice, points_to(container_eq([1, 2, 3])))
    }

    #[test]
    fn container_eq_matches_slice_using_deref_notation() -> Result<()> {
        let value = vec![1, 2, 3];
        let slice = value.as_slice();
        verify_that!(*slice, container_eq([1, 2, 3]))
    }

    #[test]
    fn container_eq_matches_ref_to_array_using_points_to() -> Result<()> {
        verify_that!(&[1, 2, 3], points_to(container_eq([1, 2, 3])))
    }

    #[test]
    fn container_eq_matches_ref_to_array_using_deref_notation() -> Result<()> {
        let value = &[1, 2, 3];
        verify_that!(*value, container_eq([1, 2, 3]))
    }

    #[test]
    fn container_eq_matches_owned_vec_of_owned_strings_with_array_of_string_slices() -> Result<()> {
        verify_that!(
            vec!["A string".to_string(), "Another string".to_string()],
            container_eq(["A string", "Another string"])
        )
    }

    #[test]
    fn container_eq_does_not_match_vec_of_owned_strings_with_shorter_array_of_string_slices()
    -> Result<()> {
        verify_that!(
            vec!["A string".to_string(), "Another string".to_string()],
            not(container_eq(["A string"]))
        )
    }

    #[test]
    fn container_eq_matches_vec_of_string_slices_with_array_of_string_slices() -> Result<()> {
        verify_that!(
            vec!["A string", "Another string"],
            container_eq(["A string", "Another string"])
        )
    }

    #[test]
    fn container_eq_matches_array_of_string_slices_with_array_of_string_slices() -> Result<()> {
        verify_that!(["A string", "Another string"], container_eq(["A string", "Another string"]))
    }

    #[test]
    fn container_eq_matches_array_of_string_slices_with_non_static_lifetime_with_array_of_string_slices()
    -> Result<()> {
        let string_1 = String::from("A string");
        let string_2 = String::from("Another string");
        verify_that!(
            [string_1.as_str(), string_2.as_str()],
            container_eq(["A string", "Another string"])
        )
    }

    #[test]
    fn container_eq_matches_hash_set_with_array() -> Result<()> {
        verify_that!(HashSet::from([1, 2, 3]), container_eq([1, 2, 3].into()))
    }

    #[test]
    fn container_eq_produces_correct_failure_message() -> Result<()> {
        let result = verify_that!(vec![1, 3, 2], container_eq(vec![1, 2, 3]));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                    Value of: vec![1, 3, 2]
                    Expected: is equal to [1, 2, 3]
                    Actual: [1, 3, 2],
                      which contains all the elements
                "
            ))))
        )
    }

    #[test]
    fn container_eq_returns_mismatch_when_elements_out_of_order() -> Result<()> {
        let result = verify_that!(vec![1, 3, 2], container_eq(vec![1, 2, 3]));
        verify_that!(
            result,
            err(displays_as(contains_substring("which contains all the elements")))
        )
    }

    #[test]
    fn container_eq_mismatch_shows_missing_elements_in_container() -> Result<()> {
        let result = verify_that!(vec![1, 2], container_eq(vec![1, 2, 3]));
        verify_that!(result, err(displays_as(contains_substring("which is missing the element 3"))))
    }

    #[test]
    fn container_eq_mismatch_shows_missing_elements_in_container_when_matching_vec_with_array()
    -> Result<()> {
        let result = verify_that!(vec![1, 2], container_eq([1, 2, 3]));
        verify_that!(result, err(displays_as(contains_substring("which is missing the element 3"))))
    }

    #[test]
    fn container_eq_mismatch_shows_surplus_elements_in_container() -> Result<()> {
        let result = verify_that!(vec![1, 2, 3], container_eq(vec![1, 2]));
        verify_that!(
            result,
            err(displays_as(contains_substring("which contains the unexpected element 3")))
        )
    }

    #[test]
    fn container_eq_mismatch_shows_missing_and_surplus_elements_in_container() -> Result<()> {
        let result = verify_that!(vec![1, 2, 4], container_eq(vec![1, 2, 3]));
        verify_that!(
            result,
            err(displays_as(contains_substring(
                "which is missing the element 3 and contains the unexpected element 4"
            )))
        )
    }

    #[test]
    fn container_eq_mismatch_does_not_show_duplicated_element() -> Result<()> {
        let result = verify_that!(vec![1, 2, 3, 3], container_eq(vec![1, 2, 3]));
        verify_that!(
            result,
            err(displays_as(contains_substring("which contains all the elements")))
        )
    }

    #[test]
    fn container_eq_mismatch_with_str_slice_shows_missing_elements_in_container() -> Result<()> {
        let result =
            verify_that!(vec!["A".to_string(), "B".to_string()], container_eq(["A", "B", "C"]));
        verify_that!(
            result,
            err(displays_as(contains_substring(r#"which is missing the element "C""#)))
        )
    }

    #[test]
    fn container_eq_mismatch_with_str_slice_shows_surplus_elements_in_container() -> Result<()> {
        let result = verify_that!(
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            container_eq(["A", "B"])
        );
        verify_that!(
            result,
            err(displays_as(contains_substring(r#"which contains the unexpected element "C""#)))
        )
    }

    #[derive(Debug, PartialEq)]
    struct OwnedItemContainer(Vec<i32>);

    impl<'a> IntoIterator for &'a OwnedItemContainer {
        type Item = i32;
        type IntoIter = std::iter::Copied<std::slice::Iter<'a, i32>>;
        fn into_iter(self) -> Self::IntoIter {
            self.0.iter().copied()
        }
    }

    #[test]
    fn container_eq_matches_on_container_when_ref_to_container_has_into_iterator_producing_owned_values()
    -> Result<()> {
        verify_that!(OwnedItemContainer(vec![1]), container_eq(OwnedItemContainer(vec![1])))
    }
}
