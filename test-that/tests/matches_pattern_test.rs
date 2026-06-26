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
use std::{convert::Infallible, marker::PhantomData, ops::Deref};
use test_that::prelude::*;

#[test]
fn matches_struct_containing_single_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }
    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct { a_field: eq(123) }))
}

#[test]
fn matches_struct_containing_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(actual, matches_pattern!(AStruct { a_field: eq(123), another_field: eq(234) }))
}

#[test]
#[rustfmt::skip]// Otherwise fmt strips the trailing comma
fn supports_trailing_comma_with_one_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }
    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct {
        a_field: eq(123), // Block reformatting
    }))
}

#[test]
fn supports_trailing_comma_with_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            a_field: eq(123),
            another_field: eq(234), // Block reformatting
        })
    )
}

#[test]
fn supports_trailing_comma_with_three_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }
    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            a_field: eq(123),
            another_field: eq(234),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_containing_nested_struct_with_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_nested_struct: ANestedStruct,
    }
    #[derive(Debug)]
    struct ANestedStruct {
        a_field: u32,
    }
    let actual = AStruct { a_nested_struct: ANestedStruct { a_field: 123 } };

    verify_that!(
        actual,
        matches_pattern!(AStruct { a_nested_struct: pat!(ANestedStruct { a_field: eq(123) }) })
    )
}

#[test]
fn has_correct_assertion_failure_message_for_single_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }
    let actual = AStruct { a_field: 123 };
    let result = verify_that!(actual, matches_pattern!(AStruct { a_field: eq(234) }));

    verify_that!(
        result,
        err(displays_as(contains_substring(indoc! {"
            Value of: actual
            Expected: is AStruct which has field `a_field`, which is equal to 234
            Actual: AStruct { a_field: 123 },
              which has field `a_field`, which isn't equal to 234
            "
        })))
    )
}

#[test]
fn has_correct_assertion_failure_message_for_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    let actual = AStruct { a_field: 123, another_field: 234 };
    let result = verify_that!(
        actual,
        matches_pattern!(AStruct { a_field: eq(234), another_field: eq(123) })
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: actual
            Expected: is AStruct which has all the following properties:
              * has field `a_field`, which is equal to 234
              * has field `another_field`, which is equal to 123
            Actual: AStruct { a_field: 123, another_field: 234 },
              * which has field `a_field`, which isn't equal to 234
              * which has field `another_field`, which isn't equal to 123"
        ))))
    )
}

#[test]
fn has_correct_assertion_failure_message_for_field_and_property() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }
    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }
    let actual = AStruct { a_field: 123, another_field: 234 };
    let result = verify_that!(
        actual,
        matches_pattern!(AStruct { get_field(): eq(234), another_field: eq(123) })
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
            Value of: actual
            Expected: is AStruct which has all the following properties:
              * result of applying `get_field ()` is equal to 234
              * has field `another_field`, which is equal to 123
            Actual: AStruct { a_field: 123, another_field: 234 },
              * which after applying `get_field ()` results in `123`, which isn't equal to 234
              * which has field `another_field`, which isn't equal to 123"
        ))))
    )
}

#[test]
fn has_meaningful_assertion_failure_message_when_wrong_enum_variant_is_used() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(#[allow(unused)] u32),
        #[allow(unused)]
        B(u32),
    }
    let actual = AnEnum::A(123);
    let result = verify_that!(actual, matches_pattern!(AnEnum::B(eq(123))));

    verify_that!(
        result,
        err(displays_as(contains_substring(indoc! {"
            Actual: A(123),
              which has the wrong enum variant `A`
            "
        })))
    )
}

#[test]
fn supports_qualified_struct_names() -> TestResult<()> {
    mod a_module {
        #[derive(Debug)]
        pub(super) struct AStruct {
            pub(super) a_field: u32,
        }
    }
    let actual = a_module::AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(a_module::AStruct { a_field: eq(123) }))
}

#[test]
fn matches_tuple_struct_containing_single_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(u32);
    let actual = AStruct(123);

    verify_that!(actual, matches_pattern!(AStruct(eq(123))))
}

#[test]
fn matches_tuple_struct_containing_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32);
    let actual = AStruct(123, 234);

    verify_that!(actual, matches_pattern!(AStruct(eq(123), eq(234))))
}

#[test]
fn matches_tuple_struct_containing_three_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32);
    let actual = AStruct(123, 234, 345);

    verify_that!(actual, matches_pattern!(AStruct(eq(123), eq(234), eq(345))))
}

#[test]
fn matches_tuple_struct_containing_four_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456);

    verify_that!(actual, matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456))))
}

#[test]
fn matches_tuple_struct_containing_five_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567);

    verify_that!(actual, matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456), eq(567))))
}

#[test]
fn matches_tuple_struct_containing_six_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678);

    verify_that!(
        actual,
        matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456), eq(567), eq(678)))
    )
}

#[test]
fn matches_tuple_struct_containing_seven_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789);

    verify_that!(
        actual,
        matches_pattern!(AStruct(eq(123), eq(234), eq(345), eq(456), eq(567), eq(678), eq(789)))
    )
}

#[test]
fn matches_tuple_struct_containing_eight_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789, 890);

    verify_that!(
        actual,
        matches_pattern!(AStruct(
            eq(123),
            eq(234),
            eq(345),
            eq(456),
            eq(567),
            eq(678),
            eq(789),
            eq(890)
        ))
    )
}

#[test]
fn matches_tuple_struct_containing_nine_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789, 890, 901);

    verify_that!(
        actual,
        matches_pattern!(AStruct(
            eq(123),
            eq(234),
            eq(345),
            eq(456),
            eq(567),
            eq(678),
            eq(789),
            eq(890),
            eq(901)
        ))
    )
}

#[test]
fn matches_tuple_struct_containing_ten_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);
    let actual = AStruct(123, 234, 345, 456, 567, 678, 789, 890, 901, 12);

    verify_that!(
        actual,
        matches_pattern!(AStruct(
            eq(123),
            eq(234),
            eq(345),
            eq(456),
            eq(567),
            eq(678),
            eq(789),
            eq(890),
            eq(901),
            eq(12)
        ))
    )
}

#[test]
fn matches_tuple_struct_with_trailing_comma() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(u32);
    let actual = AStruct(123);

    verify_that!(
        actual,
        matches_pattern!(AStruct(
            eq(123), // Keep the trailing comma, block reformatting
        ))
    )
}

#[test]
fn matches_tuple_struct_with_two_fields_and_trailing_comma() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct(u32, u32);
    let actual = AStruct(123, 234);

    verify_that!(
        actual,
        matches_pattern!(AStruct(
            eq(123),
            eq(234), // Keep the trailing comma, block reformatting
        ))
    )
}

#[test]
fn matches_enum_without_field() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A,
    }
    let actual = AnEnum::A;

    verify_that!(actual, matches_pattern!(AnEnum::A))
}

const ANENUM_A_REPR: &str = "AnEnum::A";

#[test]
fn generates_correct_failure_output_when_enum_variant_without_field_is_not_matched()
-> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        #[allow(unused)]
        A,
        B,
    }
    let actual = AnEnum::B;

    let result = verify_that!(actual, matches_pattern!(AnEnum::A));

    verify_that!(result, err(displays_as(contains_substring(format!("is not {ANENUM_A_REPR}")))))
}

#[test]
fn generates_correct_failure_output_when_enum_variant_without_field_is_matched() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A,
    }
    let actual = AnEnum::A;

    let result = verify_that!(actual, not(matches_pattern!(AnEnum::A)));

    verify_that!(result, err(displays_as(contains_substring(format!("is {ANENUM_A_REPR}")))))
}

#[test]
fn matches_enum_with_field() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32),
    }
    let actual = AnEnum::A(123);

    verify_that!(actual, matches_pattern!(AnEnum::A(eq(123))))
}

#[test]
fn does_not_match_wrong_enum_value() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        #[allow(unused)]
        A(u32),
        B,
    }
    let actual = AnEnum::B;

    verify_that!(actual, not(matches_pattern!(AnEnum::A(eq(123)))))
}

#[test]
fn includes_enum_variant_in_description_with_field() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32),
    }
    let actual = AnEnum::A(123);

    let result = verify_that!(actual, matches_pattern!(AnEnum::A(eq(234))));

    verify_that!(
        result,
        err(displays_as(contains_substring(format!(
            "Expected: is {ANENUM_A_REPR} which has field `0`"
        ))))
    )
}

#[test]
fn includes_enum_variant_in_negative_description_with_field() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32),
    }
    let actual = AnEnum::A(123);

    let result = verify_that!(actual, not(matches_pattern!(AnEnum::A(eq(123)))));

    verify_that!(
        result,
        err(displays_as(contains_substring(format!(
            "Expected: is not {ANENUM_A_REPR} which has field `0`, which is equal to"
        ))))
    )
}

#[test]
fn includes_enum_variant_in_description_with_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32, u32),
    }
    let actual = AnEnum::A(123, 234);

    let result = verify_that!(actual, matches_pattern!(AnEnum::A(eq(234), eq(234))));

    verify_that!(
        result,
        err(displays_as(contains_substring(format!(
            "Expected: is {ANENUM_A_REPR} which has all the following properties"
        ))))
    )
}

#[test]
fn includes_enum_variant_in_description_with_three_fields() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32, u32, u32),
    }
    let actual = AnEnum::A(123, 234, 345);

    let result = verify_that!(actual, matches_pattern!(AnEnum::A(eq(234), eq(234), eq(345))));

    verify_that!(
        result,
        err(displays_as(contains_substring(format!(
            "Expected: is {ANENUM_A_REPR} which has all the following properties"
        ))))
    )
}

#[test]
fn includes_enum_variant_in_description_with_named_field() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A { field: u32 },
    }
    let actual = AnEnum::A { field: 123 };

    let result = verify_that!(actual, matches_pattern!(AnEnum::A { field: eq(234) }));

    verify_that!(
        result,
        err(displays_as(contains_substring(format!(
            "Expected: is {ANENUM_A_REPR} which has field `field`"
        ))))
    )
}

#[test]
fn includes_enum_variant_in_description_with_two_named_fields() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A { field: u32, another_field: u32 },
    }
    let actual = AnEnum::A { field: 123, another_field: 234 };

    let result = verify_that!(
        actual,
        matches_pattern!(AnEnum::A { field: eq(234), another_field: eq(234) })
    );

    verify_that!(
        result,
        err(displays_as(contains_substring(format!(
            "Expected: is {ANENUM_A_REPR} which has all the following properties"
        ))))
    )
}

#[test]
fn includes_struct_name_in_description_with_property() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        field: u32,
    }
    impl AStruct {
        fn get_field(&self) -> u32 {
            self.field
        }
    }
    let actual = AStruct { field: 123 };

    let result = verify_that!(actual, matches_pattern!(AStruct { get_field(): eq(234) }));

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `get_field ()` results in")))
    )
}

#[test]
fn includes_struct_name_in_description_with_ref_property() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        field: u32,
    }
    impl AStruct {
        fn get_field(&self) -> &u32 {
            &self.field
        }
    }
    let actual = AStruct { field: 123 };

    let result = verify_that!(actual, matches_pattern!(AStruct { *get_field(): eq(234) }));

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `get_field ()` results in")))
    )
}

#[test]
fn includes_struct_name_in_description_with_property_after_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        field: u32,
    }
    impl AStruct {
        fn get_field(&self) -> u32 {
            self.field
        }
    }
    let actual = AStruct { field: 123 };

    let result =
        verify_that!(actual, matches_pattern!(AStruct { field: eq(123), get_field(): eq(234) }));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "Expected: is AStruct which has all the following properties"
        )))
    )
}

#[test]
fn includes_struct_name_in_description_with_ref_property_after_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        field: u32,
    }
    impl AStruct {
        fn get_field(&self) -> &u32 {
            &self.field
        }
    }
    let actual = AStruct { field: 123 };

    let result =
        verify_that!(actual, matches_pattern!(AStruct { field: eq(123), *get_field(): eq(234) }));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "Expected: is AStruct which has all the following properties"
        )))
    )
}

#[test]
fn matches_struct_with_a_method() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct { get_field(): eq(123) }))
}

#[test]
fn matches_struct_with_a_method_and_trailing_comma() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct { get_field(): eq(123), }))
}

#[test]
fn matches_struct_with_a_method_taking_parameter() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn add_to_field(&self, a: u32) -> u32 {
            self.a_field + a
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { add_to_field(2): eq(3) }))
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { add_product_to_field(2, 3): eq(7) }))
}

#[test]
fn matches_struct_with_a_method_taking_enum_value_parameter() -> TestResult<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_a_field(&self, _value: AnEnum) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { get_a_field(AnEnum::AVariant): eq(1) }))
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_with_trailing_comma() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { add_product_to_field(2, 3,): eq(7) }))
}

#[test]
fn matches_struct_with_a_method_returning_a_reference() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct { *get_field_ref(): eq(123) }))
}

#[test]
fn matches_struct_with_a_reference_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a> {
        a_ref: &'a u32,
    }

    let inner = 123;
    let actual = AStruct { a_ref: &inner };

    verify_that!(actual, matches_pattern!(AStruct { *a_ref: eq(123) }))
}

#[test]
fn matches_struct_with_a_reference_field_in_last_position() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a> {
        first_field: u32,
        a_ref: &'a u32,
    }

    let inner = 123;
    let actual = AStruct { first_field: 321, a_ref: &inner };

    verify_that!(actual, matches_pattern!(AStruct { first_field: eq(321), *a_ref: eq(123) }))
}

#[test]
fn matches_struct_with_a_reference_field_in_first_position() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a> {
        a_ref: &'a u32,
        last_field: u32,
    }

    let inner = 123;
    let actual = AStruct { a_ref: &inner, last_field: 321 };

    verify_that!(actual, matches_pattern!(AStruct { *a_ref: eq(123), last_field: eq(321) }))
}

#[test]
fn matches_struct_with_a_reference_field_in_middle_position() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a> {
        first_field: u32,
        a_ref: &'a u32,
        last_field: u32,
    }

    let inner = 123;
    let actual = AStruct { first_field: 321, a_ref: &inner, last_field: 432 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { first_field: eq(321), *a_ref: eq(123), last_field: eq(432) })
    )
}

#[test]
fn matches_struct_with_a_method_returning_a_reference_with_trailing_comma() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123 };

    verify_that!(actual, matches_pattern!(AStruct { *get_field_ref(): eq(123), }))
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_ret_ref() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { *get_field_ref(2, 3): eq(1) }))
}

#[test]
fn matches_struct_with_a_method_returning_reference_taking_enum_value_parameter() -> TestResult<()>
{
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _value: AnEnum) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { *get_field_ref(AnEnum::AVariant): eq(1) }))
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_with_trailing_comma_ret_ref() -> TestResult<()>
{
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1 };

    verify_that!(actual, matches_pattern!(AStruct { *get_field_ref(2, 3,): eq(1) }))
}

#[test]
fn matches_struct_with_a_method_followed_by_a_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(actual, matches_pattern!(AStruct { get_field(): eq(123), another_field: eq(234) }))
}

#[test]
fn matches_struct_with_a_method_followed_by_a_field_with_trailing_comma() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { get_field(): eq(123), another_field: eq(234), })
    )
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_and_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1, another_field: 123 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { add_product_to_field(2, 3): eq(7), another_field: eq(123) })
    )
}

#[test]
fn matches_struct_with_a_method_taking_enum_value_parameter_followed_by_field() -> TestResult<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self, _value: AnEnum) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 2 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { get_field(AnEnum::AVariant): eq(1), another_field: eq(2) })
    )
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_with_trailing_comma_and_field()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1, another_field: 123 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { add_product_to_field(2, 3,): eq(7), another_field: eq(123) })
    )
}

#[test]
fn matches_struct_with_a_method_returning_reference_followed_by_a_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_field_ref(): eq(123), another_field: eq(234) })
    )
}

#[test]
fn matches_struct_with_a_method_returning_reference_followed_by_a_field_with_trailing_comma()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_field_ref(): eq(123), another_field: eq(234), })
    )
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_ret_ref_and_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 123 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_field_ref(2, 3): eq(1), another_field: eq(123) })
    )
}

#[test]
fn matches_struct_with_a_method_taking_enum_value_param_ret_ref_followed_by_field() -> TestResult<()>
{
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _value: AnEnum) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 2 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_field_ref(AnEnum::AVariant): eq(1), another_field: eq(2) })
    )
}

#[test]
fn matches_struct_with_a_method_taking_two_parameters_with_trailing_comma_ret_ref_and_field()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 123 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_field_ref(2, 3,): eq(1), another_field: eq(123) })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(actual, matches_pattern!(AStruct { another_field: eq(234), get_field(): eq(123) }))
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_trailing_comma() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), get_field(): eq(123), })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), add_product_to_field(2, 3): eq(7) })
    )
}

#[test]
fn matches_struct_with_field_followed_by_method_taking_enum_value_param() -> TestResult<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field(&self, _value: AnEnum) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 2 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(2), get_field(AnEnum::AVariant): eq(1) })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_and_trailing_comma()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), add_product_to_field(2, 3,): eq(7) })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_returning_reference() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), *get_field_ref(): eq(123) })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_returning_ref_and_trailing_comma()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), *get_field_ref(): eq(123), })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_ret_ref() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), *get_field_ref(2, 3): eq(123) })
    )
}

#[test]
fn matches_struct_with_field_followed_by_method_taking_enum_value_param_ret_ref() -> TestResult<()>
{
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _value: AnEnum) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 2 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(2), *get_field_ref(AnEnum::AVariant): eq(1) })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_and_trailing_comma_ret_ref()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(234), *get_field_ref(2, 3,): eq(123) })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_followed_by_a_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            get_field(): eq(123),
            a_third_field: eq(345)
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_followed_by_a_field_with_trailing_comma()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field(&self) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            get_field(): eq(123),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_followed_by_a_field()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            add_product_to_field(2, 3): eq(7),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_and_trailing_comma_followed_by_a_field()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn add_product_to_field(&self, a: u32, b: u32) -> u32 {
            self.a_field + a * b
        }
    }

    let actual = AStruct { a_field: 1, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            add_product_to_field(2, 3,): eq(7),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_with_field_followed_by_method_taking_enum_value_param_followed_by_field()
-> TestResult<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field(&self, _value: AnEnum) -> u32 {
            self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 2, a_third_field: 3 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(2),
            get_field(AnEnum::AVariant): eq(1),
            a_third_field: eq(3),
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_ret_ref_followed_by_a_field() -> TestResult<()>
{
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            *get_field_ref(): eq(123),
            a_third_field: eq(345)
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_ret_ref_followed_by_a_field_with_trailing_comma()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            *get_field_ref(): eq(123),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_ret_ref_followed_by_a_field()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            *get_field_ref(2, 3): eq(123),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_with_field_followed_by_method_taking_enum_value_param_ret_ref_followed_by_field()
-> TestResult<()> {
    enum AnEnum {
        AVariant,
    }

    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _value: AnEnum) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 1, another_field: 2, a_third_field: 3 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(2),
            *get_field_ref(AnEnum::AVariant): eq(1),
            a_third_field: eq(3),
        })
    )
}

#[test]
fn matches_struct_with_a_field_followed_by_a_method_with_params_trailing_comma_ret_ref_followed_by_a_field()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
        another_field: u32,
        a_third_field: u32,
    }

    impl AStruct {
        fn get_field_ref(&self, _a: u32, _b: u32) -> &u32 {
            &self.a_field
        }
    }

    let actual = AStruct { a_field: 123, another_field: 234, a_third_field: 345 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(234),
            *get_field_ref(2, 3,): eq(123),
            a_third_field: eq(345),
        })
    )
}

#[test]
fn matches_struct_with_method_returning_option_of_reference() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_option_of_field(&self) -> Option<&u32> {
            Some(&self.a_field)
        }
    }

    let actual = AStruct { a_field: 123 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            get_option_of_field(): some(points_to(eq(123))),
        })
    )
}

#[test]
fn matches_struct_with_method_returning_result_of_reference() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_result_with_field(&self) -> std::result::Result<&u32, Infallible> {
            Ok(&self.a_field)
        }
    }

    let actual = AStruct { a_field: 123 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            get_result_with_field(): ok(points_to(eq(123))),
        })
    )
}

#[test]
fn matches_struct_with_method_returning_result_with_err_of_reference() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_field: u32,
    }

    impl AStruct {
        fn get_result_with_field(&self) -> std::result::Result<(), &u32> {
            Err(&self.a_field)
        }
    }

    let actual = AStruct { a_field: 123 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            get_result_with_field(): err(points_to(eq(123))),
        })
    )
}

#[test]
fn matches_struct_containing_reference() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a> {
        a_field: &'a u32,
    }

    impl<'a> AStruct<'a> {
        fn get_field(&self) -> &'a u32 {
            self.a_field
        }
    }

    let value = 123;
    let actual = AStruct { a_field: &value };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            *get_field(): eq(123),
        })
    )
}

#[test]
fn matches_struct_with_method_returning_view_on_reference() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a> {
        a_field: &'a u32,
    }
    #[derive(Debug)]
    struct View<'a>(&'a u32);
    impl<'a> Deref for View<'a> {
        type Target = u32;

        fn deref(&self) -> &Self::Target {
            self.0
        }
    }

    impl<'a> AStruct<'a> {
        fn get_field(&self) -> View<'a> {
            View(self.a_field)
        }
    }

    let value = 123;
    let actual = AStruct { a_field: &value };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            get_field(): points_to(eq(123)),
        })
    )
}

#[test]
fn matches_struct_with_method_returning_option_of_view_on_reference() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a> {
        a_field: &'a u32,
    }
    #[derive(Debug)]
    struct View<'a>(&'a u32);
    impl<'a> Deref for View<'a> {
        type Target = u32;

        fn deref(&self) -> &Self::Target {
            self.0
        }
    }

    impl<'a> AStruct<'a> {
        fn get_field(&self) -> Option<View<'a>> {
            Some(View(self.a_field))
        }
    }

    let value = 123;
    let actual = AStruct { a_field: &value };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            get_field(): some(points_to(eq(123))),
        })
    )
}

#[test]
fn matches_struct_with_method_returning_view_on_reference_with_smaller_lifetime() -> TestResult<()>
{
    #[derive(Debug)]
    struct AStruct<'a> {
        a_field: &'a u32,
    }
    #[derive(Debug)]
    struct View<'a>(&'a u32);
    impl<'a> Deref for View<'a> {
        type Target = u32;

        fn deref(&self) -> &Self::Target {
            self.0
        }
    }

    impl<'a> AStruct<'a> {
        fn get_field<'b>(&'b self) -> View<'b> {
            View(self.a_field)
        }
    }

    let value = 123;
    let actual = AStruct { a_field: &value };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            get_field(): points_to(eq(123)),
        })
    )
}

#[test]
fn matches_struct_with_method_returning_view_on_reference_using_deref_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a> {
        a_field: &'a u32,
    }
    #[derive(Debug)]
    struct View<'a>(&'a u32);
    impl<'a> Deref for View<'a> {
        type Target = u32;

        fn deref(&self) -> &Self::Target {
            self.0
        }
    }

    impl<'a> AStruct<'a> {
        fn get_field(&self) -> View<'a> {
            View(self.a_field)
        }
    }

    let value = 123;
    let actual = AStruct { a_field: &value };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            *get_field(): eq(123),
        })
    )
}

#[test]
fn matches_struct_with_method_returning_reference_to_nested_struct() -> TestResult<()> {
    #[derive(Debug)]
    struct ANestedStruct {
        a_field: u32,
    }
    #[derive(Debug)]
    struct AStruct<'a> {
        a_field: &'a ANestedStruct,
    }

    impl<'a> AStruct<'a> {
        fn get_field(&self) -> &'a ANestedStruct {
            self.a_field
        }
    }

    let substruct = ANestedStruct { a_field: 123 };
    let actual = AStruct { a_field: &substruct };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            *get_field(): matches_pattern!(ANestedStruct {
                a_field: eq(123),
            }),
        })
    )
}

#[test]
fn matches_struct_with_method_returning_option_of_reference_to_nested_struct() -> TestResult<()> {
    #[derive(Debug)]
    struct ANestedStruct {
        a_field: u32,
    }
    #[derive(Debug)]
    struct AStruct<'a> {
        a_field: &'a ANestedStruct,
    }

    impl<'a> AStruct<'a> {
        fn get_field(&self) -> Option<&'a ANestedStruct> {
            Some(self.a_field)
        }
    }

    let substruct = ANestedStruct { a_field: 123 };
    let actual = AStruct { a_field: &substruct };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            get_field(): some(points_to(matches_pattern!(ANestedStruct {
                a_field: eq(123),
            }))),
        })
    )
}

#[cfg(feature = "anyhow")]
#[test]
fn allows_asserting_on_error_source() -> TestResult<()> {
    fn returns_anyhow_error() -> anyhow::Result<()> {
        anyhow::bail!("Anyhow error")
    }
    fn returns_wrapping_anyhow_error() -> anyhow::Result<()> {
        use anyhow::Context as _;
        returns_anyhow_error().context("Wrapping context")?;
        Ok(())
    }
    let result = returns_wrapping_anyhow_error();

    verify_that!(
        result,
        err(matches_pattern!(anyhow::Error {
            source(): some(displays_as(eq("Anyhow error")))
        }))
    )
}

#[test]
fn matches_method_returning_string_slice() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_string: String,
    }

    impl AStruct {
        fn get_string(&self) -> &str {
            &self.a_string
        }
    }

    let actual = AStruct { a_string: "Some string".into() };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            get_string(): eq("Some string"),
        })
    )
}

#[test]
fn matches_method_returning_array_slice() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }

    impl AStruct {
        fn get_slice(&self) -> &[u32] {
            &self.a_vec
        }
    }

    let actual = AStruct { a_vec: vec![1, 2, 3] };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            *get_slice(): contains_exactly![eq(1), eq(2), eq(3)].in_order(),
        })
    )
}

#[test]
fn matches_method_returning_array_slice_with_points_to() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }

    impl AStruct {
        fn get_slice(&self) -> &[u32] {
            &self.a_vec
        }
    }

    let actual = AStruct { a_vec: vec![1, 2, 3] };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            get_slice(): points_to(contains_exactly![eq(1), eq(2), eq(3)].in_order()),
        })
    )
}

#[test]
fn matches_method_returning_array_slice_with_points_to_for_single_element() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }

    impl AStruct {
        fn get_slice(&self) -> &[u32] {
            &self.a_vec
        }
    }

    let actual = AStruct { a_vec: vec![1, 2, 3] };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            get_slice(): points_to(contains(eq(1)))
        })
    )
}

#[test]
fn matches_vec_field_with_shorthand_array_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }

    let actual = AStruct { a_vec: vec![1, 2, 3] };

    verify_that!(actual, matches_pattern!(AStruct { a_vec: [eq(1), eq(2), eq(3)] }))
}

#[test]
fn matches_vec_field_with_shorthand_set_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }

    let actual = AStruct { a_vec: vec![1, 2, 3] };

    verify_that!(actual, matches_pattern!(AStruct { a_vec: {eq(3), eq(2), eq(1)} }))
}

#[test]
fn matches_slice_field_with_shorthand_array_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a> {
        a_slice: &'a [u32],
    }

    let value = vec![1, 2, 3];
    let actual = AStruct { a_slice: &value };

    verify_that!(actual, matches_pattern!(AStruct { *a_slice: [eq(1), eq(2), eq(3)] }))
}

#[test]
fn matches_slice_field_with_shorthand_set_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a> {
        a_slice: &'a [u32],
    }

    let value = vec![1, 2, 3];
    let actual = AStruct { a_slice: &value };

    verify_that!(actual, matches_pattern!(AStruct { *a_slice: {eq(3), eq(2), eq(1)} }))
}

#[test]
fn matches_method_returning_vec_with_shorthand_array_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }
    impl AStruct {
        fn get_a_vec(&self) -> Vec<u32> {
            self.a_vec.clone()
        }
    }

    let actual = AStruct { a_vec: vec![1, 2, 3] };

    verify_that!(actual, matches_pattern!(AStruct { get_a_vec(): [eq(1), eq(2), eq(3)] }))
}

#[test]
fn matches_method_returning_vec_with_shorthand_set_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }
    impl AStruct {
        fn get_a_vec(&self) -> Vec<u32> {
            self.a_vec.clone()
        }
    }

    let actual = AStruct { a_vec: vec![1, 2, 3] };

    verify_that!(actual, matches_pattern!(AStruct { get_a_vec(): {eq(3), eq(2), eq(1)} }))
}

#[test]
fn matches_method_returning_slice_with_shorthand_array_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }
    impl AStruct {
        fn get_a_slice(&self) -> &[u32] {
            &self.a_vec
        }
    }

    let actual = AStruct { a_vec: vec![1, 2, 3] };

    verify_that!(actual, matches_pattern!(AStruct { *get_a_slice(): [eq(1), eq(2), eq(3)] }))
}

#[test]
fn matches_method_returning_slice_with_shorthand_set_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }
    impl AStruct {
        fn get_a_slice(&self) -> &[u32] {
            &self.a_vec
        }
    }

    let actual = AStruct { a_vec: vec![1, 2, 3] };

    verify_that!(actual, matches_pattern!(AStruct { *get_a_slice(): {eq(3), eq(2), eq(1)} }))
}

// Container shorthand in multi-field structs

#[test]
fn shorthand_array_syntax_in_first_position_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
        another_field: u32,
    }

    let actual = AStruct { a_vec: vec![1, 2, 3], another_field: 42 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { a_vec: [eq(1), eq(2), eq(3)], another_field: eq(42) })
    )
}

#[test]
fn shorthand_set_syntax_in_first_position_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
        another_field: u32,
    }

    let actual = AStruct { a_vec: vec![1, 2, 3], another_field: 42 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { a_vec: {eq(3), eq(1), eq(2)}, another_field: eq(42) })
    )
}

#[test]
fn shorthand_array_syntax_in_middle_position_of_three_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        first_field: u32,
        a_vec: Vec<u32>,
        last_field: u32,
    }

    let actual = AStruct { first_field: 10, a_vec: vec![1, 2, 3], last_field: 20 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            first_field: eq(10),
            a_vec: [eq(1), eq(2), eq(3)],
            last_field: eq(20)
        })
    )
}

#[test]
fn shorthand_set_syntax_in_middle_position_of_three_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        first_field: u32,
        a_vec: Vec<u32>,
        last_field: u32,
    }

    let actual = AStruct { first_field: 10, a_vec: vec![1, 2, 3], last_field: 20 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            first_field: eq(10),
            a_vec: {eq(3), eq(1), eq(2)},
            last_field: eq(20)
        })
    )
}

#[test]
fn shorthand_array_syntax_in_last_position_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        another_field: u32,
        a_vec: Vec<u32>,
    }

    let actual = AStruct { another_field: 42, a_vec: vec![1, 2, 3] };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(42), a_vec: [eq(1), eq(2), eq(3)] })
    )
}

#[test]
fn shorthand_set_syntax_in_last_position_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        another_field: u32,
        a_vec: Vec<u32>,
    }

    let actual = AStruct { another_field: 42, a_vec: vec![1, 2, 3] };

    verify_that!(
        actual,
        matches_pattern!(AStruct { another_field: eq(42), a_vec: {eq(3), eq(1), eq(2)} })
    )
}

#[test]
fn deref_field_shorthand_array_syntax_in_first_position_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a> {
        a_slice: &'a [u32],
        another_field: u32,
    }

    let value = vec![1, 2, 3];
    let actual = AStruct { a_slice: &value, another_field: 42 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *a_slice: [eq(1), eq(2), eq(3)], another_field: eq(42) })
    )
}

#[test]
fn deref_field_shorthand_set_syntax_in_first_position_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct<'a> {
        a_slice: &'a [u32],
        another_field: u32,
    }

    let value = vec![1, 2, 3];
    let actual = AStruct { a_slice: &value, another_field: 42 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *a_slice: {eq(3), eq(1), eq(2)}, another_field: eq(42) })
    )
}

#[test]
fn property_shorthand_array_syntax_in_first_position_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
        another_field: u32,
    }
    impl AStruct {
        fn get_a_vec(&self) -> Vec<u32> {
            self.a_vec.clone()
        }
    }

    let actual = AStruct { a_vec: vec![1, 2, 3], another_field: 42 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { get_a_vec(): [eq(1), eq(2), eq(3)], another_field: eq(42) })
    )
}

#[test]
fn property_shorthand_set_syntax_in_first_position_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
        another_field: u32,
    }
    impl AStruct {
        fn get_a_vec(&self) -> Vec<u32> {
            self.a_vec.clone()
        }
    }

    let actual = AStruct { a_vec: vec![1, 2, 3], another_field: 42 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { get_a_vec(): {eq(3), eq(1), eq(2)}, another_field: eq(42) })
    )
}

#[test]
fn deref_property_shorthand_array_syntax_in_first_position_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
        another_field: u32,
    }
    impl AStruct {
        fn get_a_slice(&self) -> &[u32] {
            &self.a_vec
        }
    }

    let actual = AStruct { a_vec: vec![1, 2, 3], another_field: 42 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_a_slice(): [eq(1), eq(2), eq(3)], another_field: eq(42) })
    )
}

#[test]
fn deref_property_shorthand_set_syntax_in_first_position_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
        another_field: u32,
    }
    impl AStruct {
        fn get_a_slice(&self) -> &[u32] {
            &self.a_vec
        }
    }

    let actual = AStruct { a_vec: vec![1, 2, 3], another_field: 42 };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_a_slice(): {eq(3), eq(1), eq(2)}, another_field: eq(42) })
    )
}

// Trailing comma

#[test]
#[rustfmt::skip]
fn shorthand_array_syntax_with_trailing_comma_in_first_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
        another_field: u32,
    }

    let actual = AStruct { a_vec: vec![1, 2, 3], another_field: 42 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            a_vec: [eq(1), eq(2), eq(3),],
            another_field: eq(42),
        })
    )
}

#[test]
#[rustfmt::skip]
fn shorthand_set_syntax_with_trailing_comma_in_first_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
        another_field: u32,
    }

    let actual = AStruct { a_vec: vec![1, 2, 3], another_field: 42 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            a_vec: {eq(3), eq(1), eq(2),},
            another_field: eq(42),
        })
    )
}

#[test]
#[rustfmt::skip]
fn shorthand_array_syntax_without_trailing_comma_in_last_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        another_field: u32,
        a_vec: Vec<u32>,
    }

    let actual = AStruct { another_field: 42, a_vec: vec![1, 2, 3] };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(42),
            a_vec: [eq(1), eq(2), eq(3)]
        })
    )
}

#[test]
#[rustfmt::skip]
fn shorthand_set_syntax_without_trailing_comma_in_last_of_two_fields() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        another_field: u32,
        a_vec: Vec<u32>,
    }

    let actual = AStruct { another_field: 42, a_vec: vec![1, 2, 3] };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            another_field: eq(42),
            a_vec: {eq(3), eq(1), eq(2)}
        })
    )
}

// Empty container shorthand

#[test]
fn empty_array_shorthand_matches_empty_vec() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }

    let actual = AStruct { a_vec: vec![] };

    verify_that!(actual, matches_pattern!(AStruct { a_vec: [] }))
}

#[test]
fn empty_set_shorthand_matches_empty_vec() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }

    let actual = AStruct { a_vec: vec![] };

    verify_that!(actual, matches_pattern!(AStruct { a_vec: {} }))
}

#[test]
#[rustfmt::skip]
fn empty_array_shorthand_with_trailing_comma_matches_empty_vec() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }

    let actual = AStruct { a_vec: vec![] };

    verify_that!(actual, matches_pattern!(AStruct { a_vec: [,] }))
}

#[test]
#[rustfmt::skip]
fn empty_set_shorthand_with_trailing_comma_matches_empty_vec() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
    }

    let actual = AStruct { a_vec: vec![] };

    verify_that!(actual, matches_pattern!(AStruct { a_vec: {,} }))
}

#[test]
fn empty_array_shorthand_in_first_of_two_fields_matches_empty_vec() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        a_vec: Vec<u32>,
        another_field: u32,
    }

    let actual = AStruct { a_vec: vec![], another_field: 42 };

    verify_that!(actual, matches_pattern!(AStruct { a_vec: [], another_field: eq(42) }))
}

// Tuple structs

#[test]
fn tuple_struct_single_element_with_shorthand_array_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct ATupleStruct(Vec<u32>);

    let actual = ATupleStruct(vec![1, 2, 3]);

    verify_that!(actual, matches_pattern!(ATupleStruct([eq(1), eq(2), eq(3)])))
}

#[test]
fn tuple_struct_single_element_with_shorthand_set_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct ATupleStruct(Vec<u32>);

    let actual = ATupleStruct(vec![1, 2, 3]);

    verify_that!(actual, matches_pattern!(ATupleStruct({eq(3), eq(1), eq(2)})))
}

#[test]
fn tuple_struct_first_element_of_two_with_shorthand_array_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct ATupleStruct(Vec<u32>, u32);

    let actual = ATupleStruct(vec![1, 2, 3], 42);

    verify_that!(actual, matches_pattern!(ATupleStruct([eq(1), eq(2), eq(3)], eq(42))))
}

#[test]
fn tuple_struct_first_element_of_two_with_shorthand_set_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct ATupleStruct(Vec<u32>, u32);

    let actual = ATupleStruct(vec![1, 2, 3], 42);

    verify_that!(actual, matches_pattern!(ATupleStruct({eq(3), eq(1), eq(2)}, eq(42))))
}

#[test]
fn tuple_struct_last_element_of_two_with_shorthand_array_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct ATupleStruct(u32, Vec<u32>);

    let actual = ATupleStruct(42, vec![1, 2, 3]);

    verify_that!(actual, matches_pattern!(ATupleStruct(eq(42), [eq(1), eq(2), eq(3)])))
}

#[test]
fn tuple_struct_last_element_of_two_with_shorthand_set_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct ATupleStruct(u32, Vec<u32>);

    let actual = ATupleStruct(42, vec![1, 2, 3]);

    verify_that!(actual, matches_pattern!(ATupleStruct(eq(42), {eq(3), eq(1), eq(2)})))
}

#[test]
fn tuple_struct_middle_element_of_three_with_shorthand_array_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct ATupleStruct(u32, Vec<u32>, u32);

    let actual = ATupleStruct(10, vec![1, 2, 3], 20);

    verify_that!(actual, matches_pattern!(ATupleStruct(eq(10), [eq(1), eq(2), eq(3)], eq(20))))
}

#[test]
fn tuple_struct_middle_element_of_three_with_shorthand_set_syntax() -> TestResult<()> {
    #[derive(Debug)]
    struct ATupleStruct(u32, Vec<u32>, u32);

    let actual = ATupleStruct(10, vec![1, 2, 3], 20);

    verify_that!(actual, matches_pattern!(ATupleStruct(eq(10), {eq(3), eq(1), eq(2)}, eq(20))))
}

#[test]
fn tuple_struct_single_element_empty_array_shorthand_matches_empty_vec() -> TestResult<()> {
    #[derive(Debug)]
    struct ATupleStruct(Vec<u32>);

    let actual = ATupleStruct(vec![]);

    verify_that!(actual, matches_pattern!(ATupleStruct([])))
}

#[test]
fn tuple_struct_single_element_empty_set_shorthand_matches_empty_vec() -> TestResult<()> {
    #[derive(Debug)]
    struct ATupleStruct(Vec<u32>);

    let actual = ATupleStruct(vec![]);

    verify_that!(actual, matches_pattern!(ATupleStruct({})))
}

// ── Enums ─────────────────────────────────────────────────────────────────────

#[test]
fn enum_tuple_variant_single_element_with_shorthand_array_syntax() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(Vec<u32>),
    }

    let actual = AnEnum::A(vec![1, 2, 3]);

    verify_that!(actual, matches_pattern!(AnEnum::A([eq(1), eq(2), eq(3)])))
}

#[test]
fn enum_tuple_variant_single_element_with_shorthand_set_syntax() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(Vec<u32>),
    }

    let actual = AnEnum::A(vec![1, 2, 3]);

    verify_that!(actual, matches_pattern!(AnEnum::A({eq(3), eq(1), eq(2)})))
}

#[test]
fn enum_tuple_variant_first_element_of_two_with_shorthand_array_syntax() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(Vec<u32>, u32),
    }

    let actual = AnEnum::A(vec![1, 2, 3], 42);

    verify_that!(actual, matches_pattern!(AnEnum::A([eq(1), eq(2), eq(3)], eq(42))))
}

#[test]
fn enum_tuple_variant_last_element_of_two_with_shorthand_array_syntax() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(u32, Vec<u32>),
    }

    let actual = AnEnum::A(42, vec![1, 2, 3]);

    verify_that!(actual, matches_pattern!(AnEnum::A(eq(42), [eq(1), eq(2), eq(3)])))
}

#[test]
fn enum_tuple_variant_empty_array_shorthand_matches_empty_vec() -> TestResult<()> {
    #[derive(Debug)]
    enum AnEnum {
        A(Vec<u32>),
    }

    let actual = AnEnum::A(vec![]);

    verify_that!(actual, matches_pattern!(AnEnum::A([])))
}

#[test]
fn ordered_container_shorthand_notation_works_in_nested_matcher() -> TestResult<()> {
    #[derive(Debug)]
    struct InnerStruct {
        vec: Vec<u32>,
    }
    #[derive(Debug)]
    struct OuterStruct {
        inner: InnerStruct,
    }

    let actual = OuterStruct { inner: InnerStruct { vec: vec![1, 2, 3] } };

    verify_that!(
        actual,
        matches_pattern!(OuterStruct { inner: pat!(InnerStruct { vec: [eq(1), eq(2), eq(3)] }) })
    )
}

#[test]
fn unordered_container_shorthand_notation_works_in_nested_matcher() -> TestResult<()> {
    #[derive(Debug)]
    struct InnerStruct {
        vec: Vec<u32>,
    }
    #[derive(Debug)]
    struct OuterStruct {
        inner: InnerStruct,
    }

    let actual = OuterStruct { inner: InnerStruct { vec: vec![1, 2, 3] } };

    verify_that!(
        actual,
        matches_pattern!(OuterStruct {
            inner: pat!(InnerStruct {
                vec: {eq(3), eq(2), eq(1)},
            })
        })
    )
}

#[derive(Debug)]
struct AStruct {
    value: u32,
}

impl AStruct {
    fn get_value(&self) -> u32 {
        self.value
    }
}

#[test]
fn matches_pattern_supports_leading_module_separator_for_field() -> TestResult<()> {
    let actual = AStruct { value: 123 };

    verify_that!(actual, matches_pattern!(::AStruct { value: eq(123) }))
}

#[test]
fn matches_pattern_supports_leading_module_separator_for_property() -> TestResult<()> {
    let actual = AStruct { value: 123 };

    verify_that!(actual, matches_pattern!(::AStruct { get_value(): eq(123) }))
}

#[test]
fn matches_pattern_supports_matching_against_generic_struct_with_type_parameters() -> TestResult<()>
{
    #[derive(Debug)]
    struct AGenericStruct<T> {
        phantom: PhantomData<T>,
    }
    impl<T: Default> AGenericStruct<T> {
        fn get_value(&self) -> T {
            T::default()
        }
    }
    let actual = AGenericStruct { phantom: PhantomData };

    verify_that!(actual, matches_pattern!(AGenericStruct<u32> { get_value(): eq(0) }))
}

#[test]
fn matches_pattern_supports_matching_against_struct_with_generic_method_with_type_parameters()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        value: u32,
    }
    impl AStruct {
        fn a_generic_method<T: PartialEq<u32>>(&self, other: T) -> bool {
            other == self.value
        }
    }
    let actual = AStruct { value: 123 };

    verify_that!(actual, matches_pattern!(AStruct { a_generic_method::<u32>(123): eq(true) }))
}

#[test]
fn matches_pattern_supports_turbofish_method_with_unordered_container_shorthand() -> TestResult<()>
{
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_items<T: PartialEq<u32>>(&self, _filter: T) -> Vec<u32> {
            self.items.clone()
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    verify_that!(actual, matches_pattern!(AStruct { get_items::<u32>(0): {eq(1), eq(2), eq(3)} }))
}

#[test]
fn matches_pattern_supports_turbofish_method_with_ordered_container_shorthand() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_items<T: PartialEq<u32>>(&self, _filter: T) -> Vec<u32> {
            self.items.clone()
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    verify_that!(actual, matches_pattern!(AStruct { get_items::<u32>(0): [eq(1), eq(2), eq(3)] }))
}

#[test]
fn matches_pattern_supports_turbofish_method_not_as_last_field() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        value: u32,
    }
    impl AStruct {
        fn a_generic_method<T: PartialEq<u32>>(&self, other: T) -> bool {
            other == self.value
        }
    }
    let actual = AStruct { value: 123 };

    verify_that!(
        actual,
        matches_pattern!(AStruct {
            a_generic_method::<u32>(123): eq(true),
            value: eq(123),
        })
    )
}

#[test]
fn matches_pattern_supports_deref_turbofish_method() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        value: u32,
    }
    impl AStruct {
        fn get_value_ref<T: PartialEq<u32>>(&self, _other: T) -> &u32 {
            &self.value
        }
    }
    let actual = AStruct { value: 123 };

    verify_that!(actual, matches_pattern!(AStruct { *get_value_ref::<u32>(0): eq(123) }))
}

#[test]
fn matches_pattern_supports_deref_turbofish_method_with_unordered_container_shorthand()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_items_ref<T: PartialEq<u32>>(&self, _filter: T) -> &Vec<u32> {
            &self.items
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_items_ref::<u32>(0): {eq(1), eq(2), eq(3)} })
    )
}

#[test]
fn matches_pattern_supports_deref_turbofish_method_with_ordered_container_shorthand()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_items_ref<T: PartialEq<u32>>(&self, _filter: T) -> &Vec<u32> {
            &self.items
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    verify_that!(
        actual,
        matches_pattern!(AStruct { *get_items_ref::<u32>(0): [eq(1), eq(2), eq(3)] })
    )
}

#[test]
fn includes_correct_definition_in_explanation_for_set_shorthand_method() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_a_vec(&self) -> Vec<u32> {
            self.items.clone()
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    let result = verify_that!(actual, matches_pattern!(AStruct { get_a_vec(): {eq(999)} }));

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `get_a_vec ()` results in")))
    )
}

#[test]
fn includes_correct_definition_in_description_for_set_shorthand_method() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_a_vec(&self) -> Vec<u32> {
            self.items.clone()
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    let result = verify_that!(actual, matches_pattern!(AStruct { get_a_vec(): {eq(999)} }));

    verify_that!(result, err(displays_as(contains_substring("result of applying `get_a_vec ()`"))))
}

#[test]
fn includes_correct_definition_in_explanation_for_array_shorthand_method() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_a_vec(&self) -> Vec<u32> {
            self.items.clone()
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    let result = verify_that!(actual, matches_pattern!(AStruct { get_a_vec(): [eq(999)] }));

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `get_a_vec ()` results in")))
    )
}

#[test]
fn includes_correct_definition_in_description_for_array_shorthand_method() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_a_vec(&self) -> Vec<u32> {
            self.items.clone()
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    let result = verify_that!(actual, matches_pattern!(AStruct { get_a_vec(): [eq(999)] }));

    verify_that!(result, err(displays_as(contains_substring("result of applying `get_a_vec ()`"))))
}

#[test]
fn includes_correct_definition_in_explanation_for_deref_method_with_set_shorthand() -> TestResult<()>
{
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_a_slice(&self) -> &[u32] {
            &self.items
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    let result = verify_that!(actual, matches_pattern!(AStruct { *get_a_slice(): {eq(999)} }));

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `get_a_slice ()` results in")))
    )
}

#[test]
fn includes_correct_definition_in_explanation_for_deref_method_with_array_shorthand()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_a_slice(&self) -> &[u32] {
            &self.items
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    let result = verify_that!(actual, matches_pattern!(AStruct { *get_a_slice(): [eq(999)] }));

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `get_a_slice ()` results in")))
    )
}

#[test]
fn includes_correct_definition_in_description_for_deref_method() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        field: u32,
    }
    impl AStruct {
        fn get_field(&self) -> &u32 {
            &self.field
        }
    }
    let actual = AStruct { field: 123 };

    let result = verify_that!(actual, matches_pattern!(AStruct { *get_field(): eq(999) }));

    verify_that!(result, err(displays_as(contains_substring("result of applying `get_field ()`"))))
}

#[test]
fn includes_correct_definition_in_explanation_for_method_with_arguments() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        field: u32,
    }
    impl AStruct {
        fn add_to_field(&self, a: u32) -> u32 {
            self.field + a
        }
    }
    let actual = AStruct { field: 1 };

    let result = verify_that!(actual, matches_pattern!(AStruct { add_to_field(5): eq(999) }));

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `add_to_field (5)` results in")))
    )
}

#[test]
fn includes_correct_definition_in_explanation_for_turbofish_method() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        value: u32,
    }
    impl AStruct {
        fn a_generic_method<T: PartialEq<u32>>(&self, other: T) -> bool {
            other == self.value
        }
    }
    let actual = AStruct { value: 123 };

    let result =
        verify_that!(actual, matches_pattern!(AStruct { a_generic_method::<u32>(5): eq(true) }));

    verify_that!(
        result,
        err(displays_as(contains_substring(
            "which after applying `a_generic_method (5)` results in"
        )))
    )
}

#[test]
fn includes_correct_definition_in_explanation_for_turbofish_method_with_set_shorthand()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_items<T: PartialEq<u32>>(&self, _filter: T) -> Vec<u32> {
            self.items.clone()
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    let result = verify_that!(actual, matches_pattern!(AStruct { get_items::<u32>(0): {eq(999)} }));

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `get_items (0)` results in")))
    )
}

#[test]
fn includes_correct_definition_in_explanation_for_turbofish_method_with_array_shorthand()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_items<T: PartialEq<u32>>(&self, _filter: T) -> Vec<u32> {
            self.items.clone()
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    let result = verify_that!(actual, matches_pattern!(AStruct { get_items::<u32>(0): [eq(999)] }));

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `get_items (0)` results in")))
    )
}

#[test]
fn includes_correct_definition_in_explanation_for_deref_turbofish_method() -> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        value: u32,
    }
    impl AStruct {
        fn get_value_ref<T: PartialEq<u32>>(&self, _other: T) -> &u32 {
            &self.value
        }
    }
    let actual = AStruct { value: 123 };

    let result =
        verify_that!(actual, matches_pattern!(AStruct { *get_value_ref::<u32>(5): eq(999) }));

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `get_value_ref (5)` results in")))
    )
}

#[test]
fn includes_correct_definition_in_explanation_for_deref_turbofish_method_with_set_shorthand()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_items_ref<T: PartialEq<u32>>(&self, _filter: T) -> &Vec<u32> {
            &self.items
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    let result =
        verify_that!(actual, matches_pattern!(AStruct { *get_items_ref::<u32>(0): {eq(999)} }));

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `get_items_ref (0)` results in")))
    )
}

#[test]
fn includes_correct_definition_in_explanation_for_deref_turbofish_method_with_array_shorthand()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
    }
    impl AStruct {
        fn get_items_ref<T: PartialEq<u32>>(&self, _filter: T) -> &Vec<u32> {
            &self.items
        }
    }
    let actual = AStruct { items: vec![1, 2, 3] };

    let result =
        verify_that!(actual, matches_pattern!(AStruct { *get_items_ref::<u32>(0): [eq(999)] }));

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `get_items_ref (0)` results in")))
    )
}

#[test]
fn includes_correct_definition_in_explanation_for_set_shorthand_method_not_in_last_position()
-> TestResult<()> {
    #[derive(Debug)]
    struct AStruct {
        items: Vec<u32>,
        another_field: u32,
    }
    impl AStruct {
        fn get_a_vec(&self) -> Vec<u32> {
            self.items.clone()
        }
    }
    let actual = AStruct { items: vec![1, 2, 3], another_field: 42 };

    let result = verify_that!(
        actual,
        matches_pattern!(AStruct { get_a_vec(): {eq(999)}, another_field: eq(42) })
    );

    verify_that!(
        result,
        err(displays_as(all!(
            contains_substring("result of applying `get_a_vec ()`"),
            contains_substring("which after applying `get_a_vec ()` results in"),
        )))
    )
}

#[test]
#[cfg(feature = "anyhow")]
fn matches_pattern_matches_source_of_source_of_error() -> TestResult<()> {
    use anyhow::anyhow;

    let inner = anyhow!("Inner error");
    let outer = inner.context("Outer error");
    let outermost = outer.context("Outermost error");
    verify_that!(
        outermost,
        matches_pattern!(anyhow::Error {
            source(): some(points_to(matches_pattern!(dyn std::error::Error {
                source(): some(displays_as(eq("Inner error")))
            })))
        })
    )
}

// Tests for dyn Trait arms added to matches_pattern_internal!.
//
// The property $expr / last arm is already covered by
// matches_pattern_matches_source_of_source_of_error above. The remaining 11
// tests below cover the other 11 new arms.

#[test]
fn matches_pattern_on_dyn_trait_with_multiple_expr_method_matchers() -> TestResult<()> {
    use std::fmt::Debug;
    trait MyTrait: Debug {
        fn a(&self) -> i32;
        fn b(&self) -> i32;
    }
    #[derive(Debug)]
    struct MyImpl;
    impl MyTrait for MyImpl {
        fn a(&self) -> i32 {
            1
        }
        fn b(&self) -> i32 {
            2
        }
    }
    let impl_val = MyImpl;
    let value: &dyn MyTrait = &impl_val;
    verify_that!(
        value,
        points_to(matches_pattern!(dyn MyTrait {
            a(): eq(1),
            b(): eq(2),
        }))
    )
}

#[test]
fn matches_pattern_on_dyn_trait_with_single_brace_method_matcher() -> TestResult<()> {
    use std::fmt::Debug;
    trait MyTrait: Debug {
        fn a(&self) -> Vec<i32>;
    }
    #[derive(Debug)]
    struct MyImpl;
    impl MyTrait for MyImpl {
        fn a(&self) -> Vec<i32> {
            vec![1, 2, 3]
        }
    }
    let impl_val = MyImpl;
    let value: &dyn MyTrait = &impl_val;
    verify_that!(
        value,
        points_to(matches_pattern!(dyn MyTrait {
            a(): {eq(3), eq(1), eq(2)}
        }))
    )
}

#[test]
fn matches_pattern_on_dyn_trait_with_multiple_brace_method_matchers() -> TestResult<()> {
    use std::fmt::Debug;
    trait MyTrait: Debug {
        fn a(&self) -> Vec<i32>;
        fn b(&self) -> Vec<i32>;
    }
    #[derive(Debug)]
    struct MyImpl;
    impl MyTrait for MyImpl {
        fn a(&self) -> Vec<i32> {
            vec![1, 2]
        }
        fn b(&self) -> Vec<i32> {
            vec![3, 4]
        }
    }
    let impl_val = MyImpl;
    let value: &dyn MyTrait = &impl_val;
    verify_that!(
        value,
        points_to(matches_pattern!(dyn MyTrait {
            a(): {eq(2), eq(1)},
            b(): {eq(4), eq(3)},
        }))
    )
}

#[test]
fn matches_pattern_on_dyn_trait_with_single_list_method_matcher() -> TestResult<()> {
    use std::fmt::Debug;
    trait MyTrait: Debug {
        fn a(&self) -> Vec<i32>;
    }
    #[derive(Debug)]
    struct MyImpl;
    impl MyTrait for MyImpl {
        fn a(&self) -> Vec<i32> {
            vec![1, 2, 3]
        }
    }
    let impl_val = MyImpl;
    let value: &dyn MyTrait = &impl_val;
    verify_that!(
        value,
        points_to(matches_pattern!(dyn MyTrait {
            a(): [eq(1), eq(2), eq(3)]
        }))
    )
}

#[test]
fn matches_pattern_on_dyn_trait_with_multiple_list_method_matchers() -> TestResult<()> {
    use std::fmt::Debug;
    trait MyTrait: Debug {
        fn a(&self) -> Vec<i32>;
        fn b(&self) -> Vec<i32>;
    }
    #[derive(Debug)]
    struct MyImpl;
    impl MyTrait for MyImpl {
        fn a(&self) -> Vec<i32> {
            vec![1, 2]
        }
        fn b(&self) -> Vec<i32> {
            vec![3, 4]
        }
    }
    let impl_val = MyImpl;
    let value: &dyn MyTrait = &impl_val;
    verify_that!(
        value,
        points_to(matches_pattern!(dyn MyTrait {
            a(): [eq(1), eq(2)],
            b(): [eq(3), eq(4)],
        }))
    )
}

#[test]
fn matches_pattern_on_dyn_trait_with_single_deref_expr_method_matcher() -> TestResult<()> {
    use std::fmt::Debug;
    trait MyTrait: Debug {
        fn a(&self) -> &i32;
    }
    #[derive(Debug)]
    struct MyImpl {
        val: i32,
    }
    impl MyTrait for MyImpl {
        fn a(&self) -> &i32 {
            &self.val
        }
    }
    let impl_val = MyImpl { val: 42 };
    let value: &dyn MyTrait = &impl_val;
    verify_that!(
        value,
        points_to(matches_pattern!(dyn MyTrait {
            *a(): eq(42)
        }))
    )
}

#[test]
fn matches_pattern_on_dyn_trait_with_multiple_deref_expr_method_matchers() -> TestResult<()> {
    use std::fmt::Debug;
    trait MyTrait: Debug {
        fn a(&self) -> &i32;
        fn b(&self) -> &i32;
    }
    #[derive(Debug)]
    struct MyImpl {
        a: i32,
        b: i32,
    }
    impl MyTrait for MyImpl {
        fn a(&self) -> &i32 {
            &self.a
        }
        fn b(&self) -> &i32 {
            &self.b
        }
    }
    let impl_val = MyImpl { a: 1, b: 2 };
    let value: &dyn MyTrait = &impl_val;
    verify_that!(
        value,
        points_to(matches_pattern!(dyn MyTrait {
            *a(): eq(1),
            *b(): eq(2),
        }))
    )
}

#[test]
fn matches_pattern_on_dyn_trait_with_single_deref_brace_method_matcher() -> TestResult<()> {
    use std::fmt::Debug;
    trait MyTrait: Debug {
        fn a(&self) -> &[i32];
    }
    #[derive(Debug)]
    struct MyImpl {
        vals: Vec<i32>,
    }
    impl MyTrait for MyImpl {
        fn a(&self) -> &[i32] {
            &self.vals
        }
    }
    let impl_val = MyImpl { vals: vec![1, 2, 3] };
    let value: &dyn MyTrait = &impl_val;
    verify_that!(
        value,
        points_to(matches_pattern!(dyn MyTrait {
            *a(): {eq(3), eq(1), eq(2)}
        }))
    )
}

#[test]
fn matches_pattern_on_dyn_trait_with_multiple_deref_brace_method_matchers() -> TestResult<()> {
    use std::fmt::Debug;
    trait MyTrait: Debug {
        fn a(&self) -> &[i32];
        fn b(&self) -> &[i32];
    }
    #[derive(Debug)]
    struct MyImpl {
        a: Vec<i32>,
        b: Vec<i32>,
    }
    impl MyTrait for MyImpl {
        fn a(&self) -> &[i32] {
            &self.a
        }
        fn b(&self) -> &[i32] {
            &self.b
        }
    }
    let impl_val = MyImpl { a: vec![1, 2], b: vec![3, 4] };
    let value: &dyn MyTrait = &impl_val;
    verify_that!(
        value,
        points_to(matches_pattern!(dyn MyTrait {
            *a(): {eq(2), eq(1)},
            *b(): {eq(4), eq(3)},
        }))
    )
}

#[test]
fn matches_pattern_on_dyn_trait_with_single_deref_list_method_matcher() -> TestResult<()> {
    use std::fmt::Debug;
    trait MyTrait: Debug {
        fn a(&self) -> &[i32];
    }
    #[derive(Debug)]
    struct MyImpl {
        vals: Vec<i32>,
    }
    impl MyTrait for MyImpl {
        fn a(&self) -> &[i32] {
            &self.vals
        }
    }
    let impl_val = MyImpl { vals: vec![1, 2, 3] };
    let value: &dyn MyTrait = &impl_val;
    verify_that!(
        value,
        points_to(matches_pattern!(dyn MyTrait {
            *a(): [eq(1), eq(2), eq(3)]
        }))
    )
}

#[test]
fn matches_pattern_on_dyn_trait_with_multiple_deref_list_method_matchers() -> TestResult<()> {
    use std::fmt::Debug;
    trait MyTrait: Debug {
        fn a(&self) -> &[i32];
        fn b(&self) -> &[i32];
    }
    #[derive(Debug)]
    struct MyImpl {
        a: Vec<i32>,
        b: Vec<i32>,
    }
    impl MyTrait for MyImpl {
        fn a(&self) -> &[i32] {
            &self.a
        }
        fn b(&self) -> &[i32] {
            &self.b
        }
    }
    let impl_val = MyImpl { a: vec![1, 2], b: vec![3, 4] };
    let value: &dyn MyTrait = &impl_val;
    verify_that!(
        value,
        points_to(matches_pattern!(dyn MyTrait {
            *a(): [eq(1), eq(2)],
            *b(): [eq(3), eq(4)],
        }))
    )
}
