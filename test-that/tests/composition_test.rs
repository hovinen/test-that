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

use test_that::prelude::*;

#[test]
fn all_matcher_works_as_inner_matcher() -> TestResult<()> {
    let value = vec![1];
    verify_that!(value, contains_each![all!(gt(0), lt(2))])
}

#[test]
fn matches_pattern_works_as_inner_matcher() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(i32);
    verify_that!(vec![AStruct(123)], contains_each![matches_pattern!(AStruct(eq(123)))])
}

#[test]
fn matches_pattern_works_with_property_as_inner_matcher() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(i32);
    impl AStruct {
        fn get_value(&self) -> i32 {
            self.0
        }
    }
    verify_that!(
        vec![AStruct(123)],
        contains_each![matches_pattern!(AStruct {
            get_value(): eq(123)
        })]
    )
}

#[test]
fn contains_each_works_as_inner_matcher() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(Vec<i32>);
    verify_that!(AStruct(vec![123]), matches_pattern!(AStruct(contains_each![eq(123)])))
}

#[test]
fn pointwise_works_as_inner_matcher() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(Vec<i32>);
    verify_that!(AStruct(vec![123]), matches_pattern!(AStruct(pointwise!(eq, [123]))))
}

#[test]
fn elements_are_works_as_inner_matcher() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(Vec<i32>);
    verify_that!(AStruct(vec![123]), matches_pattern!(AStruct(contains_exactly![eq(123)])))
}

#[test]
fn tuple_works_as_inner_matcher() -> TestResult<()> {
    verify_that!(vec![(123,)], contains_exactly![(eq(123),)])
}

#[test]
fn result_of_can_be_nested() -> TestResult<()> {
    #[derive(Debug)]
    struct InnerStruct<'a> {
        string: &'a str,
    }
    #[derive(Debug)]
    struct OuterStruct<'a> {
        inner: InnerStruct<'a>,
    }
    let string = String::from("Hello, world");
    let value = OuterStruct { inner: InnerStruct { string: string.as_str() } };
    verify_that!(
        value,
        result_of!(
            |s: &OuterStruct| s.inner,
            result_of!(|s: &InnerStruct| s.string, starts_with("Hello"))
        )
    )
}

#[test]
fn result_of_can_be_nested_inside_field() -> TestResult<()> {
    #[derive(Debug)]
    struct InnerStruct<'a> {
        string: &'a str,
    }
    #[derive(Debug)]
    struct OuterStruct<'a> {
        inner: InnerStruct<'a>,
    }
    let string = String::from("Hello, world");
    let value = OuterStruct { inner: InnerStruct { string: string.as_str() } };
    verify_that!(
        value,
        field!(OuterStruct.inner, result_of!(|s: &InnerStruct| s.string, starts_with("Hello")))
    )
}

#[test]
fn field_can_be_nested_inside_result_of() -> TestResult<()> {
    #[derive(Debug)]
    struct InnerStruct<'a> {
        string: &'a str,
    }
    #[derive(Debug)]
    struct OuterStruct<'a> {
        inner: InnerStruct<'a>,
    }
    let string = String::from("Hello, world");
    let value = OuterStruct { inner: InnerStruct { string: string.as_str() } };
    verify_that!(
        value,
        result_of!(|s: &OuterStruct| s.inner, field!(InnerStruct.string, starts_with("Hello")))
    )
}

#[test]
fn eq_matcher_into_str_matcher_works_outside_crate() -> TestResult<()> {
    verify_that!("Hello, world!", eq("hello, world!").ignoring_ascii_case())
}
