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

use test_that::prelude::*;

#[derive(Debug)]
struct SomeStruct {
    a_property: u32,
}

impl SomeStruct {
    fn get_property(&self) -> u32 {
        self.a_property
    }

    fn get_property_ref(&self) -> &u32 {
        &self.a_property
    }

    fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
        self.a_property + a * b
    }

    fn get_property_ref_with_params(&self, _a: u32, _b: u32) -> &u32 {
        &self.a_property
    }
}

#[test]
fn matches_struct_with_matching_field_value() -> TestResult<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, result_of!(|s: &SomeStruct| s.a_property, eq(10)))
}

#[test]
fn does_not_match_struct_with_non_matching_field_value() -> TestResult<()> {
    let value = SomeStruct { a_property: 20 };
    verify_that!(value, not(result_of!(|s: &SomeStruct| s.a_property, eq(10))))
}

#[test]
fn matches_when_closure_returns_reference_with_self_lifetime() -> TestResult<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, result_of!(|s: &SomeStruct| &s.a_property, points_to(eq(10))))
}

#[test]
fn matches_when_closure_returns_non_copy_struct() -> TestResult<()> {
    #[derive(PartialEq, Debug)]
    struct NonCopyStruct(u32);
    #[derive(Debug)]
    struct AStruct(NonCopyStruct);
    let value = AStruct(NonCopyStruct(10));
    verify_that!(value, result_of!(|s: &AStruct| s.0, eq(NonCopyStruct(10))))
}

#[test]
fn matches_when_closure_returns_field_with_foreign_lifetime() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a>(&'a u32);
    let content = 10;
    let value = AStruct(&content);
    verify_that!(value, result_of!(|s: &AStruct| s.0, points_to(eq(10))))
}

#[test]
fn matches_when_closure_returns_field_whose_type_is_struct_with_foreign_lifetime() -> TestResult<()>
{
    #[derive(PartialEq, Debug)]
    struct StructWithLifetime<'a>(&'a u32);
    #[derive(Debug)]
    struct AStruct<'a>(StructWithLifetime<'a>);
    let content = 10;
    let value = AStruct(StructWithLifetime(&content));
    verify_that!(value, result_of!(|s: &AStruct| s.0, eq(StructWithLifetime(&10))))
}

#[test]
fn matches_when_closure_returns_field_whose_type_is_struct_with_narrowed_lifetime() -> TestResult<()>
{
    #[derive(PartialEq, Debug)]
    struct StructWithLifetime<'a>(&'a u32);
    #[derive(Debug)]
    struct AStruct<'a>(&'a u32);
    impl<'a> AStruct<'a> {
        fn get_field<'b>(&'b self) -> StructWithLifetime<'b> {
            StructWithLifetime(self.0)
        }
    }
    let content = 10;
    let value = AStruct(&content);
    verify_that!(value, result_of!(|s: &AStruct| s.get_field(), eq(StructWithLifetime(&10))))
}

#[test]
fn matches_when_closure_returns_option_and_using_some_matcher() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a>(&'a u32);
    impl<'a> AStruct<'a> {
        fn get_option_of_field(&self) -> Option<&'a u32> {
            Some(self.0)
        }
    }
    let content = 10;
    let value = AStruct(&content);
    verify_that!(value, result_of!(|s: &AStruct| s.get_option_of_field(), some(points_to(eq(10)))))
}

#[test]
fn matches_when_closure_returns_result_and_using_ok_matcher() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a>(&'a u32);
    impl<'a> AStruct<'a> {
        fn get_result_of_field(&self) -> std::result::Result<&'a u32, ()> {
            Ok(self.0)
        }
    }
    let content = 10;
    let value = AStruct(&content);
    verify_that!(value, result_of!(|s: &AStruct| s.get_result_of_field(), ok(points_to(eq(10)))))
}

#[test]
fn matches_when_closure_returns_result_and_using_err_matcher() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a>(&'a u32);
    impl<'a> AStruct<'a> {
        fn get_result_of_field(&self) -> std::result::Result<(), &'a u32> {
            Err(self.0)
        }
    }
    let content = 10;
    let value = AStruct(&content);
    verify_that!(value, result_of!(|s: &AStruct| s.get_result_of_field(), err(points_to(eq(10)))))
}

#[test]
fn matches_struct_with_matching_property() -> TestResult<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, result_of!(|s: &SomeStruct| s.get_property(), eq(10)))
}

#[test]
fn matches_struct_with_matching_property_with_parameters() -> TestResult<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, result_of!(|s: &SomeStruct| s.add_product_to_field(2, 3), eq(16)))
}

#[test]
fn matches_struct_with_matching_property_with_captured_arguments() -> TestResult<()> {
    let value = SomeStruct { a_property: 10 };
    let arg1 = 2;
    let arg2 = 3;
    verify_that!(value, result_of!(|s: &SomeStruct| s.add_product_to_field(arg1, arg2), eq(16)))
}

#[test]
fn matches_struct_with_matching_property_ref() -> TestResult<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, result_of!(|s: &SomeStruct| s.get_property_ref(), points_to(eq(10))))
}

#[test]
fn matches_struct_with_matching_property_ref_with_qualified_type() -> TestResult<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, result_of!(|s: &crate::SomeStruct| s.get_property_ref(), points_to(eq(10))))
}

#[test]
fn matches_struct_with_matching_string_reference_property() -> TestResult<()> {
    #[derive(Debug)]
    struct StructWithString {
        property: String,
    }
    impl StructWithString {
        fn get_property_ref(&self) -> &String {
            &self.property
        }
    }
    let value = StructWithString { property: "Something".into() };
    verify_that!(value, result_of!(|s: &StructWithString| s.get_property_ref(), eq("Something")))
}

#[test]
fn matches_struct_with_matching_slice_property() -> TestResult<()> {
    #[derive(Debug)]
    struct StructWithVec {
        property: Vec<u32>,
    }
    impl StructWithVec {
        fn get_property_ref(&self) -> &[u32] {
            &self.property
        }
    }
    let value = StructWithVec { property: vec![1, 2, 3] };
    verify_that!(value, result_of!(|s: &StructWithVec| s.get_property_ref(), eq([1, 2, 3])))
}

#[test]
fn matches_struct_with_matching_property_ref_with_parameters() -> TestResult<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(
        value,
        result_of!(|s: &SomeStruct| s.get_property_ref_with_params(2, 3), points_to(eq(10)))
    )
}

#[test]
fn matches_struct_with_matching_property_ref_with_parameters_and_trailing_comma() -> TestResult<()>
{
    let value = SomeStruct { a_property: 10 };
    verify_that!(
        value,
        result_of!(|s: &SomeStruct| s.get_property_ref_with_params(2, 3,), points_to(eq(10)))
    )
}

#[test]
fn does_not_match_struct_with_non_matching_property() -> TestResult<()> {
    let value = SomeStruct { a_property: 2 };
    verify_that!(value, not(result_of!(|s: &SomeStruct| s.get_property(), eq(1))))
}

#[test]
fn describes_itself_in_matching_case() -> TestResult<()> {
    let value = SomeStruct { a_property: 1 };
    let result = verify_that!(value, not(result_of!(|s: &SomeStruct| s.get_property(), eq(1))));
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "after applying `|s: &SomeStruct| s.get_property()` results in `1`, which is equal to 1"
        )))
    )
}

#[test]
fn describes_itself_in_not_matching_case() -> TestResult<()> {
    let value = SomeStruct { a_property: 2 };
    let result = verify_that!(value, result_of!(|s: &SomeStruct| s.get_property(), eq(1)));
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "after applying `|s: &SomeStruct| s.get_property()` results in `2`, which isn't equal to 1"
        )))
    )
}

#[test]
fn explains_mismatch_referencing_explanation_of_inner_matcher() -> TestResult<()> {
    #[derive(Debug)]
    struct SomeStruct;
    impl SomeStruct {
        fn get_a_collection(&self) -> Vec<u32> {
            vec![]
        }
    }
    let value = SomeStruct;
    let result =
        verify_that!(value, result_of!(|s: &SomeStruct| s.get_a_collection(), container_eq([1])));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "after applying `|s: &SomeStruct| s.get_a_collection()` results in `[]`, which is missing the element 1"
        )))
    )
}

#[test]
fn describes_itself_in_matching_case_for_ref() -> TestResult<()> {
    let value = SomeStruct { a_property: 2 };
    let result =
        verify_that!(value, result_of!(|s: &SomeStruct| s.get_property_ref(), points_to(eq(1))));
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "after applying `|s: &SomeStruct| s.get_property_ref()` results in `2`, which isn't equal to 1"
        )))
    )
}

#[test]
fn describes_itself_in_not_matching_case_for_ref() -> TestResult<()> {
    let value = SomeStruct { a_property: 1 };
    let result = verify_that!(
        value,
        not(result_of!(|s: &SomeStruct| s.get_property_ref(), points_to(eq(1))))
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "after applying `|s: &SomeStruct| s.get_property_ref()` results in `1`, which is equal to 1"
        )))
    )
}

#[test]
fn explains_mismatch_referencing_explanation_of_inner_matcher_for_ref() -> TestResult<()> {
    static EMPTY_COLLECTION: Vec<u32> = vec![];
    #[derive(Debug)]
    struct SomeStruct;
    impl SomeStruct {
        fn get_a_collection_ref(&self) -> &[u32] {
            &EMPTY_COLLECTION
        }
    }
    let value = SomeStruct;
    let result = verify_that!(
        value,
        result_of!(|s: &SomeStruct| s.get_a_collection_ref(), points_to(container_eq([1])))
    );

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "after applying `|s: &SomeStruct| s.get_a_collection_ref()` results in `[]`, which is missing the element 1"
        )))
    )
}

#[derive(Debug)]
struct SomeStructWithAVec {
    a_vec_property: Vec<u32>,
}

impl SomeStructWithAVec {
    fn get_property_as_slice(&self) -> &[u32] {
        &self.a_vec_property
    }
}

#[test]
fn matches_struct_with_vec_containing_value() -> TestResult<()> {
    let value = SomeStructWithAVec { a_vec_property: vec![10] };
    verify_that!(
        value,
        result_of!(|s: &SomeStructWithAVec| s.get_property_as_slice(), points_to(contains(eq(10))))
    )
}

#[test]
fn matches_struct_with_vec_with_elements_are_value() -> TestResult<()> {
    let value = SomeStructWithAVec { a_vec_property: vec![10] };
    verify_that!(
        value,
        result_of!(
            |s: &SomeStructWithAVec| s.get_property_as_slice(),
            points_to(contains_exactly![eq(10)].in_order())
        )
    )
}

#[derive(Debug)]
struct SomeStructWithVecMethod {
    items: Vec<u32>,
}

impl SomeStructWithVecMethod {
    fn get_items(&self) -> Vec<u32> {
        self.items.clone()
    }
}

#[test]
fn result_of_with_ordered_container_shorthand() -> TestResult<()> {
    let value = SomeStructWithVecMethod { items: vec![1, 2, 3] };
    verify_that!(value, result_of!(|s: &SomeStructWithVecMethod| s.get_items(), [eq(1), eq(2), eq(3)]))
}

#[test]
fn result_of_with_unordered_container_shorthand() -> TestResult<()> {
    let value = SomeStructWithVecMethod { items: vec![3, 1, 2] };
    verify_that!(value, result_of!(|s: &SomeStructWithVecMethod| s.get_items(), {eq(1), eq(2), eq(3)}))
}

#[test]
fn result_of_with_empty_container_shorthand() -> TestResult<()> {
    let value = SomeStructWithVecMethod { items: vec![] };
    verify_that!(value, result_of!(|s: &SomeStructWithVecMethod| s.get_items(), []))
}

