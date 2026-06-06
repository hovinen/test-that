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
use std::collections::HashMap;
use test_that::matcher::Matcher;
use test_that::prelude::*;

#[test]
fn contains_exactly_matches_empty_vector() -> Result<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, contains_exactly![])
}

#[test]
fn contains_exactly_matches_empty_vector_with_trailing_comma() -> Result<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, contains_exactly![,])
}

#[test]
fn contains_exactly_matches_empty_array() -> Result<()> {
    let value: [u32; 0] = [];
    verify_that!(value, contains_exactly![])
}

#[test]
fn contains_exactly_matches_vec() -> Result<()> {
    verify_that!(vec![1, 2, 3], contains_exactly![eq(1), eq(2), eq(3)])
}

#[test]
fn contains_exactly_matches_vec_with_trailing_comma() -> Result<()> {
    verify_that!(vec![1, 2, 3], contains_exactly![eq(1), eq(2), eq(3),])
}

#[test]
fn contains_exactly_matches_vec_out_of_order() -> Result<()> {
    verify_that!(vec![1, 2, 3], contains_exactly![eq(2), eq(3), eq(1)])
}

#[test]
fn contains_exactly_matches_vec_with_repetition() -> Result<()> {
    verify_that!(
        vec![1, 2, 3, 1, 2, 3],
        contains_exactly![eq(1), eq(1), eq(2), eq(2), eq(3), eq(3)]
    )
}

#[test]
fn contains_exactly_matches_vec_with_ambigous_matchers() -> Result<()> {
    verify_that!(vec![1, 2, 3], contains_exactly![anything(), anything(), eq(1)])
}

#[test]
fn contains_exactly_does_not_match_vec_when_no_perfect_match_is_possible() -> Result<()> {
    verify_that!(vec![1, 2, 3], not(contains_exactly![ge(3), eq(3), eq(1)]))
}

#[test]
fn contains_exactly_does_not_match_vector_with_extra_elements() -> Result<()> {
    verify_that!(vec![1, 2, 3], not(contains_exactly![eq(1), eq(2)]))
}

#[test]
fn contains_exactly_does_not_match_vector_with_missing_elements() -> Result<()> {
    verify_that!(vec![1, 2], not(contains_exactly![eq(1), eq(2), eq(3)]))
}

#[test]
fn contains_exactly_matches_array() -> Result<()> {
    verify_that!([1, 2, 3], contains_exactly![eq(1), eq(2), eq(3)])
}

#[test]
fn contains_exactly_matches_ref_of_array_with_points_to() -> Result<()> {
    verify_that!(&[1, 2, 3], points_to(contains_exactly![eq(1), eq(2), eq(3)]))
}

#[test]
fn contains_exactly_matches_ref_of_array_with_deref_notation() -> Result<()> {
    let value = &[1, 2, 3];
    verify_that!(*value, contains_exactly![eq(1), eq(2), eq(3)])
}

#[test]
fn contains_exactly_matches_slice_with_points_to() -> Result<()> {
    let value = vec![1, 2, 3];
    verify_that!(value.as_slice(), points_to(contains_exactly![eq(1), eq(2), eq(3)]))
}

#[test]
fn contains_exactly_matches_slice_with_deref_notation() -> Result<()> {
    let value = vec![1, 2, 3];
    let slice = value.as_slice();
    verify_that!(*slice, contains_exactly![eq(1), eq(2), eq(3)])
}

#[test]
fn contains_exactly_matches_vec_with_references() -> Result<()> {
    verify_that!(
        vec![&1, &2, &3],
        contains_exactly![points_to(eq(1)), points_to(eq(2)), points_to(eq(3))]
    )
}

#[test]
fn contains_exactly_matches_vec_of_string_slices() -> Result<()> {
    verify_that!(
        vec!["String 1", "String 2", "String 3"],
        contains_exactly![
            contains_substring("1"),
            contains_substring("2"),
            contains_substring("3")
        ]
    )
}

#[test]
fn contains_exactly_matches_vec_of_string_slices_with_non_static_lifetime() -> Result<()> {
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
    )
}

#[test]
fn contains_exactly_matches_slice_of_string_slices() -> Result<()> {
    let value = vec!["String 1", "String 2", "String 3"];
    verify_that!(
        value.as_slice(),
        points_to(contains_exactly![
            contains_substring("1"),
            contains_substring("2"),
            contains_substring("3")
        ])
    )
}

#[test]
fn contains_exactly_matches_slice_of_string_slices_with_non_static_lifetime() -> Result<()> {
    let string_1 = String::from("String 1");
    let string_2 = String::from("String 2");
    let string_3 = String::from("String 3");
    let value = vec![string_1.as_str(), string_2.as_str(), string_3.as_str()];
    verify_that!(
        value.as_slice(),
        points_to(contains_exactly![
            contains_substring("1"),
            contains_substring("2"),
            contains_substring("3")
        ])
    )
}

#[test]
fn matches_with_curly_bracket_notation() -> Result<()> {
    verify_that!(vec![1, 2, 3], {eq(3), eq(2), eq(1)})
}

#[test]
fn contains_exactly_admits_matchers_without_static_lifetime() -> Result<()> {
    #[derive(Debug, PartialEq)]
    struct AStruct(i32);
    let expected_value = AStruct(123);
    verify_that!(vec![AStruct(123)], contains_exactly![eq_deref_of(&expected_value)])
}

#[test]
fn contains_exactly_produces_correct_failure_explanation() -> Result<()> {
    let result = verify_that!([1, 4, 3], contains_exactly![eq(1), eq(2), eq(3)]);
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: [1, 4, 3]
            Expected: contains elements matching in any order:
              0. is equal to 1
              1. is equal to 2
              2. is equal to 3
            Actual: [1, 4, 3],
              whose element #1 does not match any expected elements and no elements match the expected element #1"
        ))))
    )
}

#[test]
fn contains_exactly_produces_correct_failure_explanation_for_no_perfect_match() -> Result<()> {
    let result = verify_that!([1, 1, 2], contains_exactly![eq(1), eq(2), eq(2)]);
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "
  which does not have a perfect match with the expected elements. The best match found was:
    Actual element 1 at index 0 matched expected element `is equal to 1` at index 0.
    Actual element 2 at index 2 matched expected element `is equal to 2` at index 1.
    Actual element 1 at index 1 did not match any remaining expected element.
    Expected element `is equal to 2` at index 2 did not match any remaining actual element."
        )))
    )
}

#[test]
fn contains_exactly_failure_explanation_contains_clause_about_missing_element() -> Result<()> {
    let result = verify_that!([1, 1, 3], contains_exactly![eq(1), eq(2), eq(3)]);
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "which has no element matching the expected element #1"
        )))
    )
}

#[test]
fn contains_exactly_produces_correct_failure_message_when_matchers_are_nested() -> Result<()> {
    let result = verify_that!(
        [[0, 1], [1, 2]],
        contains_exactly![contains_exactly![eq(1), eq(2)], contains_exactly![eq(2), eq(3)]]
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
                Expected: contains elements matching in any order:
                  0. contains elements matching in any order:
                       0. is equal to 1
                       1. is equal to 2
                  1. contains elements matching in any order:
                       0. is equal to 2
                       1. is equal to 3
                Actual: [[0, 1], [1, 2]],
                  whose element #0 does not match any expected elements and no elements match the expected element #1"
        ))))
    )
}

#[test]
fn contains_exactly_explains_mismatch_due_to_wrong_size() -> Result<()> {
    let result = verify_that!([2, 3], contains_exactly![eq(2), eq(3), eq(4)]);
    verify_that!(result, err(displays_as(contains_substring("which has size 2 (expected 3)"))))
}

#[test]
fn contains_exactly_works_when_matcher_is_created_in_subroutine() -> Result<()> {
    fn create_matcher() -> impl Matcher<Vec<i32>> {
        contains_exactly![eq(1)]
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
fn contains_exactly_matches_container_a_ref_of_which_produces_owned_items() -> Result<()> {
    verify_that!(OwnedItemContainer(vec![1, 2, 3]), contains_exactly![eq(3), eq(2), eq(1)])
}

#[test]
fn contains_exactly_matches_hash_map() -> Result<()> {
    verify_that!(
        HashMap::from([(1, "One"), (2, "Two"), (3, "Three")]),
        contains_exactly![(eq(2) => eq("Two")), (eq(1) => eq("One")), (eq(3) => eq("Three"))]
    )
}

#[test]
fn contains_exactly_matches_hash_map_with_trailing_comma() -> Result<()> {
    verify_that!(
        HashMap::from([(1, "One"), (2, "Two"), (3, "Three")]),
        contains_exactly![(eq(2) => eq("Two")), (eq(1) => eq("One")), (eq(3) => eq("Three")),]
    )
}

#[test]
fn contains_exactly_does_not_match_hash_map_with_wrong_key() -> Result<()> {
    verify_that!(
        HashMap::from([(1, "One"), (2, "Two"), (4, "Three")]),
        not(contains_exactly![(eq(2) => eq("Two")), (eq(1) => eq("One")), (eq(3) => eq("Three"))])
    )
}

#[test]
fn contains_exactly_does_not_match_hash_map_with_wrong_value() -> Result<()> {
    verify_that!(
        HashMap::from([(1, "One"), (2, "Two"), (3, "Four")]),
        not(contains_exactly![(eq(2) => eq("Two")), (eq(1) => eq("One")), (eq(3) => eq("Three"))])
    )
}

#[test]
fn contains_exactly_does_not_match_hash_map_missing_element() -> Result<()> {
    verify_that!(
        HashMap::from([(1, "One"), (2, "Two")]),
        not(contains_exactly![(eq(2) => eq("Two")), (eq(1) => eq("One")), (eq(3) => eq("Three"))])
    )
}

#[test]
fn contains_exactly_does_not_match_hash_map_with_extra_element() -> Result<()> {
    verify_that!(
        HashMap::from([(1, "One"), (2, "Two"), (3, "Three")]),
        not(contains_exactly![(eq(2) => eq("Two")), (eq(1) => eq("One"))])
    )
}

#[test]
fn contains_exactly_does_not_match_hash_map_with_mismatched_key_and_value() -> Result<()> {
    verify_that!(
        HashMap::from([(1, "One"), (2, "Three"), (3, "Two")]),
        not(contains_exactly![(eq(2) => eq("Two")), (eq(1) => eq("One")), (eq(3) => eq("Three"))])
    )
}

#[test]
fn contains_exactly_with_map_admits_matchers_without_static_lifetime() -> Result<()> {
    #[derive(Debug, PartialEq)]
    struct AStruct(i32);
    let expected_value = AStruct(123);
    verify_that!(
        HashMap::from([(1, AStruct(123))]),
        contains_exactly![(eq(1) => eq_deref_of(&expected_value))]
    )
}

#[test]
fn contains_exactly_works_when_matcher_for_maps_is_created_in_subroutine() -> Result<()> {
    fn create_matcher() -> impl Matcher<HashMap<i32, i32>> {
        contains_exactly![(eq(1) => eq(1))]
    }
    verify_that!(HashMap::from([(1, 1)]), create_matcher())
}

#[test]
fn contains_each_matches_when_one_to_one_correspondence_present() -> Result<()> {
    verify_that!(vec![2, 3, 4], contains_each!(eq(2), eq(3), eq(4)))
}

#[test]
fn contains_each_supports_trailing_comma() -> Result<()> {
    verify_that!(vec![2, 3, 4], contains_each!(eq(2), eq(3), eq(4),))
}

#[test]
fn contains_each_matches_hash_map() -> Result<()> {
    verify_that!(
        HashMap::from([(1, "One"), (2, "Two"), (3, "Three")]),
        contains_each![(eq(2) => eq("Two")), (eq(1) => eq("One"))]
    )
}

#[test]
fn contains_each_matches_hash_map_with_trailing_comma() -> Result<()> {
    verify_that!(
        HashMap::from([(1, "One"), (2, "Two"), (3, "Three")]),
        contains_each![(eq(2) => eq("Two")), (eq(1) => eq("One")),]
    )
}

#[test]
fn contains_each_matches_when_no_matchers_present() -> Result<()> {
    verify_that!(vec![2, 3, 4], contains_each!())
}

#[test]
fn contains_each_matches_when_no_matchers_present_and_trailing_comma() -> Result<()> {
    verify_that!(vec![2, 3, 4], contains_each!(,))
}

#[test]
fn contains_each_matches_when_list_is_empty_and_no_matchers_present() -> Result<()> {
    verify_that!(Vec::<u32>::new(), contains_each!())
}

#[test]
fn contains_each_matches_when_excess_elements_present() -> Result<()> {
    verify_that!(vec![1, 2, 3, 4], contains_each!(eq(2), eq(3), eq(4)))
}

#[test]
fn contains_each_does_not_match_when_matchers_are_unmatched() -> Result<()> {
    verify_that!(vec![1, 2, 3], not(contains_each!(eq(2), eq(3), eq(4))))
}

#[test]
fn contains_each_explains_mismatch_due_to_wrong_size() -> Result<()> {
    let result = verify_that!([2, 3], contains_each![eq(2), eq(3), eq(4)]);
    verify_that!(
        result,
        err(displays_as(contains_substring("which has size 2 (expected at least 3)")))
    )
}

#[test]
fn contains_each_explains_missing_element_in_mismatch() -> Result<()> {
    let result = verify_that!([1, 2, 3], contains_each![eq(2), eq(3), eq(4)]);
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "which has no element matching the expected element #2"
        )))
    )
}

#[test]
fn contains_each_explains_missing_elements_in_mismatch() -> Result<()> {
    let result = verify_that!([0, 1, 2, 3], contains_each![eq(2), eq(3), eq(4), eq(5)]);
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "which has no elements matching the expected elements #2, #3"
        )))
    )
}

#[test]
fn contains_each_explains_mismatch_due_to_no_graph_matching_found() -> Result<()> {
    let result = verify_that!([1, 2], contains_each![ge(2), ge(2)]);
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "
  which does not have a superset match with the expected elements. The best match found was:
    Actual element 2 at index 1 matched expected element `is greater than or equal to 2` at index 0.
    Actual element 1 at index 0 did not match any remaining expected element.
    Expected element `is greater than or equal to 2` at index 1 did not match any remaining actual element."))
    ))
}

#[test]
fn is_contained_in_matches_with_empty_vector() -> Result<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, is_contained_in!())
}

#[test]
fn is_contained_in_matches_with_empty_vector_and_trailing_comma() -> Result<()> {
    let value: Vec<u32> = vec![];
    verify_that!(value, is_contained_in!(,))
}

#[test]
fn is_contained_in_matches_when_one_to_one_correspondence_present() -> Result<()> {
    verify_that!(vec![2, 3, 4], is_contained_in!(eq(2), eq(3), eq(4)))
}

#[test]
fn is_contained_supports_trailing_comma() -> Result<()> {
    verify_that!(vec![2, 3, 4], is_contained_in!(eq(2), eq(3), eq(4),))
}

#[test]
fn is_contained_in_matches_hash_map() -> Result<()> {
    verify_that!(
        HashMap::from([(1, "One"), (2, "Two")]),
        is_contained_in![(eq(2) => eq("Two")), (eq(1) => eq("One")), (eq(3) => eq("Three"))]
    )
}

#[test]
fn is_contained_in_matches_hash_map_with_trailing_comma() -> Result<()> {
    verify_that!(
        HashMap::from([(1, "One"), (2, "Two")]),
        is_contained_in![(eq(2) => eq("Two")), (eq(1) => eq("One")), (eq(3) => eq("Three")),]
    )
}

#[test]
fn is_contained_in_matches_when_container_is_empty() -> Result<()> {
    let value = Vec::<i32>::new();
    verify_that!(value, is_contained_in!(eq(2), eq(3), eq(4)))
}

#[test]
fn is_contained_in_matches_when_excess_matchers_present() -> Result<()> {
    verify_that!(vec![3, 4], is_contained_in!(eq(2), eq(3), eq(4)))
}

#[test]
fn is_contained_in_does_not_match_when_elements_are_unmatched() -> Result<()> {
    verify_that!(vec![1, 2, 3], not(is_contained_in!(eq(2), eq(3), eq(4))))
}

#[test]
fn is_contained_in_explains_mismatch_due_to_wrong_size() -> Result<()> {
    let result = verify_that!([2, 3, 4], is_contained_in![eq(2), eq(3)]);
    verify_that!(
        result,
        err(displays_as(contains_substring("which has size 3 (expected at most 2)")))
    )
}

#[test]
fn is_contained_in_explains_missing_element_in_mismatch() -> Result<()> {
    let result = verify_that!([1, 2, 3], is_contained_in![eq(2), eq(3), eq(4)]);
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "whose element #0 does not match any expected elements"
        )))
    )
}

#[test]
fn is_contained_in_explains_missing_elements_in_mismatch() -> Result<()> {
    let result = verify_that!([0, 1, 2, 3], is_contained_in![eq(2), eq(3), eq(4), eq(5)]);
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "whose elements #0, #1 do not match any expected elements"
        )))
    )
}

#[test]
fn is_contained_in_explains_mismatch_due_to_no_graph_matching_found() -> Result<()> {
    let result = verify_that!([1, 2], is_contained_in![ge(1), ge(3)]);
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "
  which does not have a subset match with the expected elements. The best match found was:
    Actual element 1 at index 0 matched expected element `is greater than or equal to 1` at index 0.
    Actual element 2 at index 1 did not match any remaining expected element.
    Expected element `is greater than or equal to 3` at index 1 did not match any remaining actual element."))
    ))
}
