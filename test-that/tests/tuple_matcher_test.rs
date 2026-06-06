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
use test_that::matcher::{Describable as _, Matcher, MatcherResult};
use test_that::prelude::*;

#[test]
fn empty_matcher_matches_empty_tuple() -> TestResult<()> {
    verify_that!((), ())
}

#[test]
fn singleton_matcher_matches_matching_singleton_tuple() -> TestResult<()> {
    verify_that!((123,), (eq(123),))
}

#[test]
fn singleton_matcher_does_not_match_non_matching_singleton_tuple() -> TestResult<()> {
    verify_that!((123,), not((eq(456),)))
}

#[test]
fn pair_matcher_matches_matching_pair_tuple() -> TestResult<()> {
    verify_that!((123, 456), (eq(123), eq(456)))
}

#[test]
fn pair_matcher_matches_matching_pair_tuple_with_different_types() -> TestResult<()> {
    verify_that!((123, "A string"), (eq(123), eq("A string")))
}

#[test]
fn pair_matcher_with_trailing_comma_matches_matching_pair_tuple() -> TestResult<()> {
    verify_that!((123, 456), (eq(123), eq(456),))
}

#[test]
fn tuple_matcher_matches_matching_3_tuple() -> TestResult<()> {
    verify_that!((1, 2, 3), (eq(1), eq(2), eq(3)))
}

#[test]
fn tuple_matcher_matches_matching_4_tuple() -> TestResult<()> {
    verify_that!((1, 2, 3, 4), (eq(1), eq(2), eq(3), eq(4)))
}

#[test]
fn tuple_matcher_matches_matching_5_tuple() -> TestResult<()> {
    verify_that!((1, 2, 3, 4, 5), (eq(1), eq(2), eq(3), eq(4), eq(5)))
}

#[test]
fn tuple_matcher_matches_matching_6_tuple() -> TestResult<()> {
    verify_that!((1, 2, 3, 4, 5, 6), (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6)))
}

#[test]
fn tuple_matcher_matches_matching_7_tuple() -> TestResult<()> {
    verify_that!((1, 2, 3, 4, 5, 6, 7), (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7)))
}

#[test]
fn tuple_matcher_matches_matching_8_tuple() -> TestResult<()> {
    verify_that!((1, 2, 3, 4, 5, 6, 7, 8), (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8)))
}

#[test]
fn tuple_matcher_matches_matching_9_tuple() -> TestResult<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9),
        (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9))
    )
}

#[test]
fn tuple_matcher_matches_matching_10_tuple() -> TestResult<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10),
        (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9), eq(10))
    )
}

#[test]
fn tuple_matcher_matches_matching_11_tuple() -> TestResult<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11),
        (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9), eq(10), eq(11))
    )
}

#[test]
fn tuple_matcher_matches_matching_12_tuple() -> TestResult<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),
        (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9), eq(10), eq(11), eq(12))
    )
}

#[test]
fn tuple_matcher_with_trailing_comma_matches_matching_3_tuple() -> TestResult<()> {
    verify_that!((1, 2, 3), (eq(1), eq(2), eq(3),))
}

#[test]
fn tuple_matcher_with_trailing_comma_matches_matching_4_tuple() -> TestResult<()> {
    verify_that!((1, 2, 3, 4), (eq(1), eq(2), eq(3), eq(4),))
}

#[test]
fn tuple_matcher_with_trailing_comma_matches_matching_5_tuple() -> TestResult<()> {
    verify_that!((1, 2, 3, 4, 5), (eq(1), eq(2), eq(3), eq(4), eq(5),))
}

#[test]
fn tuple_matcher_with_trailing_comma_matches_matching_6_tuple() -> TestResult<()> {
    verify_that!((1, 2, 3, 4, 5, 6), (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6),))
}

#[test]
fn tuple_matcher_with_trailing_comma_matches_matching_7_tuple() -> TestResult<()> {
    verify_that!((1, 2, 3, 4, 5, 6, 7), (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7),))
}

#[test]
fn tuple_matcher_with_trailing_comma_matches_matching_8_tuple() -> TestResult<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8),
        (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8),)
    )
}

#[test]
fn tuple_matcher_with_trailing_comma_matches_matching_9_tuple() -> TestResult<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9),
        (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9),)
    )
}

#[test]
fn tuple_matcher_with_trailing_comma_matches_matching_10_tuple() -> TestResult<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10),
        (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9), eq(10),)
    )
}

#[test]
fn tuple_matcher_with_trailing_comma_matches_matching_11_tuple() -> TestResult<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11),
        (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9), eq(10), eq(11),)
    )
}

#[test]
fn tuple_matcher_with_trailing_comma_matches_matching_12_tuple() -> TestResult<()> {
    verify_that!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),
        (eq(1), eq(2), eq(3), eq(4), eq(5), eq(6), eq(7), eq(8), eq(9), eq(10), eq(11), eq(12),)
    )
}

#[test]
fn tuple_matcher_1_has_correct_description_for_match() -> TestResult<()> {
    verify_that!(
        (eq(1),).describe(MatcherResult::Match),
        displays_as(eq(indoc!(
            "
            is a tuple whose values respectively match:
              is equal to 1"
        )))
    )
}

#[test]
fn tuple_matcher_1_has_correct_description_for_mismatch() -> TestResult<()> {
    verify_that!(
        (eq(1),).describe(MatcherResult::NoMatch),
        displays_as(eq(indoc!(
            "
            is a tuple whose values do not respectively match:
              is equal to 1"
        )))
    )
}

#[test]
fn tuple_matcher_2_has_correct_description_for_match() -> TestResult<()> {
    verify_that!(
        (eq(1), eq(2)).describe(MatcherResult::Match),
        displays_as(eq(indoc!(
            "
            is a tuple whose values respectively match:
              is equal to 1
              is equal to 2"
        )))
    )
}

#[test]
fn tuple_matcher_2_has_correct_description_for_mismatch() -> TestResult<()> {
    verify_that!(
        (eq(1), eq(2)).describe(MatcherResult::NoMatch),
        displays_as(eq(indoc!(
            "
            is a tuple whose values do not respectively match:
              is equal to 1
              is equal to 2"
        )))
    )
}

#[test]
fn describe_match_shows_which_tuple_element_did_not_match() -> TestResult<()> {
    verify_that!(
        (eq(1), eq(2)).explain_match(&(1, 3)),
        displays_as(eq(indoc!(
            "
            which
              is a tuple whose values do not respectively match:
                is equal to 1
                is equal to 2
            Element #1 is 3,
              which isn't equal to 2"
        )))
    )
}

#[test]
fn describe_match_shows_which_two_tuple_elements_did_not_match() -> TestResult<()> {
    verify_that!(
        (eq(1), eq(2)).explain_match(&(2, 3)),
        displays_as(eq(indoc!(
            "
            which
              is a tuple whose values do not respectively match:
                is equal to 1
                is equal to 2
            Element #0 is 2,
              which isn't equal to 1
            Element #1 is 3,
              which isn't equal to 2"
        )))
    )
}
