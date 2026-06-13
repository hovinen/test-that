// Copyright 2023 Google LLC
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

use indoc::indoc;
use test_that::prelude::*;

#[test]
fn pointwise_matches_array_with_single_element() -> TestResult<()> {
    verify_that!([1], pointwise!(lt, vec![2]))
}

#[test]
fn pointwise_matches_array_with_two_elements() -> TestResult<()> {
    verify_that!([1, 2], pointwise!(lt, vec![2, 3]))
}

#[test]
fn pointwise_matches_vec_with_single_element() -> TestResult<()> {
    verify_that!(vec![1], pointwise!(lt, vec![2]))
}

#[test]
fn pointwise_matches_vec_with_two_elements() -> TestResult<()> {
    verify_that!(vec![1, 2], pointwise!(lt, vec![2, 3]))
}

#[test]
fn pointwise_matches_vec_with_two_elements_with_array() -> TestResult<()> {
    verify_that!(vec![1, 2], pointwise!(lt, [2, 3]))
}

#[test]
fn pointwise_matches_two_element_slice_using_points_to() -> TestResult<()> {
    let value = vec![1, 2];
    let slice = value.as_slice();
    verify_that!(slice, points_to(pointwise!(lt, [2, 3])))
}

#[test]
fn pointwise_matches_two_element_slice_using_deref_notation() -> TestResult<()> {
    let value = vec![1, 2];
    let slice = value.as_slice();
    verify_that!(*slice, pointwise!(lt, [2, 3]))
}

#[test]
fn pointwise_matches_ref_of_array_with_using_points_to() -> TestResult<()> {
    verify_that!(&[1, 2], points_to(pointwise!(lt, vec![2, 3])))
}

#[test]
fn pointwise_matches_ref_of_array_with_using_deref_notation() -> TestResult<()> {
    let value = &[1, 2];
    verify_that!(*value, pointwise!(lt, vec![2, 3]))
}

#[derive(Debug)]
struct OwnedItemContainer(Vec<i32>);

impl<'a> IntoIterator for &'a OwnedItemContainer {
    type Item = i32;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, i32>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

#[test]
fn pointswise_matches_on_container_when_ref_to_container_has_into_iterator_producing_owned_values()
-> TestResult<()> {
    verify_that!(OwnedItemContainer(vec![1]), pointwise!(eq, [1]))
}

#[test]
fn pointwise_does_not_match_value_of_wrong_length() -> TestResult<()> {
    let value = vec![1];
    verify_that!(value, not(pointwise!(lt, vec![2, 3])))
}

#[test]
fn pointwise_does_not_match_value_not_matching_in_first_position() -> TestResult<()> {
    let value = vec![1, 2];
    verify_that!(value, not(pointwise!(lt, vec![1, 3])))
}

#[test]
fn pointwise_does_not_match_value_not_matching_in_second_position() -> TestResult<()> {
    let value = vec![1, 2];
    verify_that!(value, not(pointwise!(lt, vec![2, 2])))
}

#[test]
fn pointwise_allows_qualified_matcher_name() -> TestResult<()> {
    mod submodule {
        pub(super) use super::lt;
    }
    let value = vec![1];
    verify_that!(value, pointwise!(submodule::lt, vec![2]))
}

#[test]
fn pointwise_returns_mismatch_when_actual_value_has_wrong_length() -> TestResult<()> {
    let result = verify_that!(vec![1, 2, 3], pointwise!(eq, vec![1, 2]));

    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: vec![1, 2, 3]
            Expected: has elements satisfying respectively:
              0. is equal to 1
              1. is equal to 2
            Actual: [1, 2, 3],
              which has size 3 (expected 2)
            "
        ))))
    )
}

#[test]
fn pointwise_returns_mismatch_when_actual_value_does_not_match_on_first_item() -> TestResult<()> {
    let result = verify_that!(vec![1, 2, 3], pointwise!(eq, vec![2, 2, 3]));

    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: vec![1, 2, 3]
            Expected: has elements satisfying respectively:
              0. is equal to 2
              1. is equal to 2
              2. is equal to 3
            Actual: [1, 2, 3],
              where element #0 is 1, which isn't equal to 2
            "
        ))))
    )
}

#[test]
fn pointwise_returns_mismatch_when_actual_value_does_not_match_on_second_item() -> TestResult<()> {
    let result = verify_that!(vec![1, 2, 3], pointwise!(eq, vec![1, 3, 3]));

    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: vec![1, 2, 3]
            Expected: has elements satisfying respectively:
              0. is equal to 1
              1. is equal to 3
              2. is equal to 3
            Actual: [1, 2, 3],
              where element #1 is 2, which isn't equal to 3
            "
        ))))
    )
}

#[test]
fn pointwise_returns_mismatch_when_actual_value_does_not_match_on_first_and_second_items()
-> TestResult<()> {
    let result = verify_that!(vec![1, 2, 3], pointwise!(eq, vec![2, 3, 3]));

    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: vec![1, 2, 3]
            Expected: has elements satisfying respectively:
              0. is equal to 2
              1. is equal to 3
              2. is equal to 3
            Actual: [1, 2, 3],
              where:
                * element #0 is 1, which isn't equal to 2
                * element #1 is 2, which isn't equal to 3"
        ))))
    )
}

#[test]
#[cfg(feature = "num-traits")]
fn pointwise_matches_single_element_with_lambda_expression_with_extra_value() -> TestResult<()> {
    let value = vec![1.00001f32];
    verify_that!(value, pointwise!(|v| near(v, 0.0001), vec![1.0]))
}

#[test]
#[cfg(feature = "num-traits")]
fn pointwise_matches_single_element_with_two_containers() -> TestResult<()> {
    let value = vec![1.00001f32];
    verify_that!(value, pointwise!(near, vec![1.0], vec![0.0001]))
}

#[test]
#[cfg(feature = "num-traits")]
fn pointwise_matches_single_element_with_three_containers() -> TestResult<()> {
    let value = vec![1.00001f32];
    verify_that!(
        value,
        pointwise!(|v, t, u| near(v, t * u), vec![1.0f32], vec![0.0001f32], vec![0.5f32])
    )
}
