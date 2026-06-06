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
fn matches_struct_with_matching_field_value() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, result_of!(|s: &SomeStruct| s.a_property, eq(10)))
}

#[test]
fn does_not_match_struct_with_non_matching_field_value() -> Result<()> {
    let value = SomeStruct { a_property: 20 };
    verify_that!(value, not(result_of!(|s: &SomeStruct| s.a_property, eq(10))))
}

#[test]
fn matches_when_closure_returns_reference_with_self_lifetime() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, result_of!(|s: &SomeStruct| &s.a_property, points_to(eq(10))))
}

#[test]
fn matches_when_closure_returns_non_copy_struct() -> Result<()> {
    #[derive(PartialEq, Debug)]
    struct NonCopyStruct(u32);
    #[derive(Debug)]
    struct AStruct(NonCopyStruct);
    let value = AStruct(NonCopyStruct(10));
    verify_that!(value, result_of!(|s: &AStruct| s.0, eq(NonCopyStruct(10))))
}

#[test]
fn matches_struct_with_matching_property() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, result_of!(|s: &SomeStruct| s.get_property(), eq(10)))
}

#[test]
fn matches_struct_with_matching_property_with_parameters() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, result_of!(|s: &SomeStruct| s.add_product_to_field(2, 3), eq(16)))
}

#[test]
fn matches_struct_with_matching_property_with_captured_arguments() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    let arg1 = 2;
    let arg2 = 3;
    verify_that!(value, result_of!(|s: &SomeStruct| s.add_product_to_field(arg1, arg2), eq(16)))
}

#[test]
fn matches_struct_with_matching_property_ref() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, result_of!(|s: &SomeStruct| s.get_property_ref(), points_to(eq(10))))
}

#[test]
fn matches_struct_with_matching_property_ref_with_qualified_type() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(value, result_of!(|s: &crate::SomeStruct| s.get_property_ref(), points_to(eq(10))))
}

#[test]
fn matches_struct_with_matching_string_reference_property() -> Result<()> {
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
fn matches_struct_with_matching_slice_property() -> Result<()> {
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
fn matches_struct_with_matching_property_ref_with_parameters() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(
        value,
        result_of!(|s: &SomeStruct| s.get_property_ref_with_params(2, 3), points_to(eq(10)))
    )
}

#[test]
fn matches_struct_with_matching_property_ref_with_parameters_and_trailing_comma() -> Result<()> {
    let value = SomeStruct { a_property: 10 };
    verify_that!(
        value,
        result_of!(|s: &SomeStruct| s.get_property_ref_with_params(2, 3,), points_to(eq(10)))
    )
}

#[test]
fn does_not_match_struct_with_non_matching_property() -> Result<()> {
    let value = SomeStruct { a_property: 2 };
    verify_that!(value, not(result_of!(|s: &SomeStruct| s.get_property(), eq(1))))
}

#[test]
fn describes_itself_in_matching_case() -> Result<()> {
    let value = SomeStruct { a_property: 1 };
    let result = verify_that!(value, not(result_of!(|s: &SomeStruct| s.get_property(), eq(1))));
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "after running `| s : &SomeStruct | s.get_property()` results in `1`, which is equal to 1"
        )))
    )
}

#[test]
fn describes_itself_in_not_matching_case() -> Result<()> {
    let value = SomeStruct { a_property: 2 };
    let result = verify_that!(value, result_of!(|s: &SomeStruct| s.get_property(), eq(1)));
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "after running `| s : &SomeStruct | s.get_property()` results in `2`, which isn't equal to 1"
        )))
    )
}

#[test]
fn explains_mismatch_referencing_explanation_of_inner_matcher() -> Result<()> {
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
            "after running `| s : &SomeStruct | s.get_a_collection()` results in `[]`, which is missing the element 1"
        )))
    )
}

#[test]
fn describes_itself_in_matching_case_for_ref() -> Result<()> {
    let value = SomeStruct { a_property: 2 };
    let result =
        verify_that!(value, result_of!(|s: &SomeStruct| s.get_property_ref(), points_to(eq(1))));
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "after running `| s : &SomeStruct | s.get_property_ref()` results in `2`, which isn't equal to 1"
        )))
    )
}

#[test]
fn describes_itself_in_not_matching_case_for_ref() -> Result<()> {
    let value = SomeStruct { a_property: 1 };
    let result = verify_that!(
        value,
        not(result_of!(|s: &SomeStruct| s.get_property_ref(), points_to(eq(1))))
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(
            "after running `| s : &SomeStruct | s.get_property_ref()` results in `1`, which is equal to 1"
        )))
    )
}

#[test]
fn explains_mismatch_referencing_explanation_of_inner_matcher_for_ref() -> Result<()> {
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
            "after running `| s : &SomeStruct | s.get_a_collection_ref()` results in `[]`, which is missing the element 1"
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
fn matches_struct_with_vec_containing_value() -> Result<()> {
    let value = SomeStructWithAVec { a_vec_property: vec![10] };
    verify_that!(
        value,
        result_of!(|s: &SomeStructWithAVec| s.get_property_as_slice(), points_to(contains(eq(10))))
    )
}

#[test]
fn matches_struct_with_vec_with_elements_are_value() -> Result<()> {
    let value = SomeStructWithAVec { a_vec_property: vec![10] };
    verify_that!(
        value,
        result_of!(
            |s: &SomeStructWithAVec| s.get_property_as_slice(),
            points_to(contains_exactly![eq(10)].in_order())
        )
    )
}
