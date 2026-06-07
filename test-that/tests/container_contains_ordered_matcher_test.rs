// Copyright 2023 Google LLC
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

use indoc::indoc;
use test_that::matcher::Matcher;
use test_that::prelude::*;

#[test]
fn contains_exactly_in_order_matches_empty_vector() -> TestResult<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, contains_exactly![].in_order())
}

#[test]
fn contains_exactly_in_order_matches_empty_array() -> TestResult<()> {
    let value: [u32; 0] = [];
    verify_that!(value, contains_exactly![].in_order())
}

#[test]
fn contains_exactly_in_order_matches_vec() -> TestResult<()> {
    verify_that!(vec![1, 2, 3], contains_exactly![eq(1), eq(2), eq(3)].in_order())
}

#[test]
fn contains_exactly_in_order_does_not_match_vec_out_of_order() -> TestResult<()> {
    verify_that!(vec![1, 2, 3], not(contains_exactly![eq(2), eq(3), eq(1)].in_order()))
}

#[test]
fn contains_exactly_in_order_does_not_match_vec_with_extra_elements() -> TestResult<()> {
    verify_that!(vec![1, 2, 3], not(contains_exactly![eq(1), eq(2)].in_order()))
}

#[test]
fn contains_exactly_in_order_does_not_match_vec_with_missing_elements() -> TestResult<()> {
    verify_that!(vec![1, 2], not(contains_exactly![eq(1), eq(2), eq(3)].in_order()))
}

#[test]
fn contains_exactly_in_order_matches_array() -> TestResult<()> {
    verify_that!([1, 2, 3], contains_exactly![eq(1), eq(2), eq(3)].in_order())
}

#[test]
fn contains_exactly_in_order_matches_ref_of_array_with_points_to() -> TestResult<()> {
    verify_that!(&[1, 2, 3], points_to(contains_exactly![eq(1), eq(2), eq(3)].in_order()))
}

#[test]
fn contains_exactly_in_order_matches_ref_of_array_with_deref_notation() -> TestResult<()> {
    let value = &[1, 2, 3];
    verify_that!(*value, contains_exactly![eq(1), eq(2), eq(3)].in_order())
}

#[test]
fn contains_exactly_in_order_matches_slice_with_points_to() -> TestResult<()> {
    let value = vec![1, 2, 3];
    verify_that!(value.as_slice(), points_to(contains_exactly![eq(1), eq(2), eq(3)].in_order()))
}

#[test]
fn contains_exactly_in_order_matches_slice_with_deref_notation() -> TestResult<()> {
    let value = vec![1, 2, 3];
    let slice = value.as_slice();
    verify_that!(*slice, contains_exactly![eq(1), eq(2), eq(3)].in_order())
}

#[test]
fn contains_exactly_in_order_matches_vec_of_string_slices() -> TestResult<()> {
    verify_that!(
        vec!["String 1", "String 2", "String 3"],
        contains_exactly![
            contains_substring("1"),
            contains_substring("2"),
            contains_substring("3")
        ]
        .in_order()
    )
}

#[test]
fn contains_exactly_in_order_matches_vec_of_string_slices_with_non_static_lifetime()
-> TestResult<()> {
    let string_1 = String::from("String 1");
    let string_2 = String::from("String 2");
    let string_3 = String::from("String 3");
    verify_that!(
        vec![string_1.as_str(), string_2.as_str(), string_3.as_str()],
        contains_exactly![
            contains_substring("1"),
            contains_substring("2"),
            contains_substring("3")
        ]
        .in_order()
    )
}

#[test]
fn contains_exactly_in_order_matches_slice_of_string_slices() -> TestResult<()> {
    let value = vec!["String 1", "String 2", "String 3"];
    verify_that!(
        value.as_slice(),
        points_to(
            contains_exactly![
                contains_substring("1"),
                contains_substring("2"),
                contains_substring("3")
            ]
            .in_order()
        )
    )
}

#[test]
fn contains_exactly_in_order_matches_slice_of_string_slices_with_non_static_lifetime()
-> TestResult<()> {
    let string_1 = String::from("String 1");
    let string_2 = String::from("String 2");
    let string_3 = String::from("String 3");
    let value = vec![string_1.as_str(), string_2.as_str(), string_3.as_str()];
    verify_that!(
        value.as_slice(),
        points_to(
            contains_exactly![
                contains_substring("1"),
                contains_substring("2"),
                contains_substring("3")
            ]
            .in_order()
        )
    )
}

#[test]
fn matches_with_square_bracket_notation() -> TestResult<()> {
    verify_that!(vec![1, 2, 3], [eq(1), eq(2), eq(3)])
}

#[test]
fn contains_exactly_in_order_admits_matchers_without_static_lifetime() -> TestResult<()> {
    #[derive(Debug, PartialEq)]
    struct AStruct(i32);
    let expected_value = AStruct(123);
    verify_that!(vec![AStruct(123)], contains_exactly![eq_deref_of(&expected_value)].in_order())
}

#[test]
fn contains_exactly_in_order_produces_correct_failure_message() -> TestResult<()> {
    let result = verify_that!([1, 4, 3], contains_exactly![eq(1), eq(2), eq(3)].in_order());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
                Value of: [1, 4, 3]
                Expected: has elements:
                  0. is equal to 1
                  1. is equal to 2
                  2. is equal to 3
                Actual: [1, 4, 3],
                  where element #1 is 4, which isn't equal to 2"
        ))))
    )
}

#[test]
fn contains_exactly_in_order_produces_correct_failure_message_when_matchers_are_nested()
-> TestResult<()> {
    let result = verify_that!(
        [[0, 1], [1, 2]],
        contains_exactly![
            contains_exactly![eq(1), eq(2)].in_order(),
            contains_exactly![eq(2), eq(3)].in_order()
        ]
        .in_order()
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
                Expected: has elements:
                  0. has elements:
                       0. is equal to 1
                       1. is equal to 2
                  1. has elements:
                       0. is equal to 2
                       1. is equal to 3
                Actual: [[0, 1], [1, 2]],
                  where:
                    * element #0 is [0, 1], where:
                        * element #0 is 0, which isn't equal to 1
                        * element #1 is 1, which isn't equal to 2
                    * element #1 is [1, 2], where:
                        * element #0 is 1, which isn't equal to 2
                        * element #1 is 2, which isn't equal to 3"
        ))))
    )
}

#[test]
fn contains_exactly_in_order_explains_mismatch_due_to_wrong_size() -> TestResult<()> {
    let result = verify_that!([2, 3], contains_exactly![eq(2), eq(3), eq(4)].in_order());
    verify_that!(result, err(displays_as(contains_substring("whose size is 2"))))
}

#[test]
fn contains_exactly_in_order_works_when_matcher_is_created_in_subroutine() -> TestResult<()> {
    fn create_matcher() -> impl Matcher<Vec<i32>> {
        contains_exactly![eq(1)].in_order()
    }
    verify_that!(vec![1], create_matcher())
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
fn contains_exactly_in_order_matches_container_a_ref_of_which_produces_owned_items()
-> TestResult<()> {
    verify_that!(
        OwnedItemContainer(vec![1, 2, 3]),
        contains_exactly![eq(1), eq(2), eq(3)].in_order()
    )
}

#[test]
fn contains_each_in_order_matches_when_all_elements_present() -> TestResult<()> {
    verify_that!(vec![2, 3, 4], contains_each!(eq(2), eq(3), eq(4)).in_order())
}

#[test]
fn contains_each_in_order_matches_when_first_element_has_no_corresponding_matcher() -> TestResult<()>
{
    verify_that!(vec![2, 3, 4], contains_each!(eq(3), eq(4)).in_order())
}

#[test]
fn contains_each_in_order_matches_when_second_element_has_no_corresponding_matcher()
-> TestResult<()> {
    verify_that!(vec![2, 3, 4], contains_each!(eq(2), eq(4)).in_order())
}

#[test]
fn contains_each_in_order_matches_when_third_element_has_no_corresponding_matcher() -> TestResult<()>
{
    verify_that!(vec![2, 3, 4], contains_each!(eq(2), eq(3)).in_order())
}

#[test]
fn contains_each_in_order_matches_when_first_and_third_elements_have_no_corresponding_matchers()
-> TestResult<()> {
    verify_that!(vec![2, 3, 4], contains_each!(eq(2)).in_order())
}

#[test]
fn contains_each_in_order_matches_when_two_matchers_in_sequence_are_missing() -> TestResult<()> {
    verify_that!(vec![2, 3, 4, 5], contains_each!(eq(2), eq(5)).in_order())
}

#[test]
fn contains_each_in_order_does_not_match_when_extra_matcher_is_present() -> TestResult<()> {
    verify_that!(vec![2, 3, 4], not(contains_each!(eq(2), eq(3), eq(4), eq(5)).in_order()))
}

#[test]
fn contains_each_in_order_does_not_match_when_matchers_are_out_of_order() -> TestResult<()> {
    verify_that!(vec![2, 3, 4], not(contains_each!(eq(4), eq(3), eq(2)).in_order()))
}

#[test]
fn contains_each_in_order_produces_correct_failure_message() -> TestResult<()> {
    let result = verify_that!([1, 2, 3], contains_each![eq(4), eq(2), eq(3)].in_order());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
                Value of: [1, 2, 3]
                Expected: has elements:
                  0. is equal to 4
                  1. is equal to 2
                  2. is equal to 3
                Actual: [1, 2, 3],
                  where matcher #0 does not match any following elements"
        ))))
    )
}

#[test]
fn is_contained_in_in_order_matches_when_all_elements_present() -> TestResult<()> {
    verify_that!(vec![2, 3, 4], is_contained_in!(eq(2), eq(3), eq(4)).in_order())
}

#[test]
fn is_contained_in_in_order_matches_when_first_element_has_no_corresponding_matcher()
-> TestResult<()> {
    verify_that!(vec![3, 4], is_contained_in!(eq(2), eq(3), eq(4)).in_order())
}

#[test]
fn is_contained_in_in_order_matches_when_second_matcher_has_no_corresponding_element()
-> TestResult<()> {
    verify_that!(vec![2, 4], is_contained_in!(eq(2), eq(3), eq(4)).in_order())
}

#[test]
fn is_contained_in_in_order_matches_when_third_matcher_has_no_corresponding_element()
-> TestResult<()> {
    verify_that!(vec![2, 3], is_contained_in!(eq(2), eq(3), eq(4)).in_order())
}

#[test]
fn is_contained_in_in_order_matches_when_first_and_third_matchers_have_no_corresponding_elements()
-> TestResult<()> {
    verify_that!(vec![3], is_contained_in!(eq(2), eq(3), eq(4)).in_order())
}

#[test]
fn is_contained_in_in_order_matches_when_two_elements_in_sequence_are_missing() -> TestResult<()> {
    verify_that!(vec![2, 5], is_contained_in!(eq(2), eq(3), eq(4), eq(5)).in_order())
}

#[test]
fn is_contained_in_in_order_does_not_match_when_extra_element_is_present() -> TestResult<()> {
    verify_that!(vec![2, 3, 4, 5], not(is_contained_in!(eq(2), eq(3), eq(4)).in_order()))
}

#[test]
fn is_contained_in_in_order_does_not_match_when_matchers_are_out_of_order() -> TestResult<()> {
    verify_that!(vec![2, 3, 4], not(is_contained_in!(eq(4), eq(3), eq(2)).in_order()))
}

#[test]
fn is_contained_in_in_order_produces_correct_failure_message() -> TestResult<()> {
    let result = verify_that!([1, 2, 3], is_contained_in![eq(4), eq(2), eq(3)].in_order());
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
                Value of: [1, 2, 3]
                Expected: has elements:
                  0. is equal to 4
                  1. is equal to 2
                  2. is equal to 3
                Actual: [1, 2, 3],
                  where element #0 is not matched by any following matcher"
        ))))
    )
}
