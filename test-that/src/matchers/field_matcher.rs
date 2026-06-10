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

// There are no visible documentation elements in this module; the declarative
// macro is documented in the matchers module.
#![doc(hidden)]

/// Matches a structure or enum with a given field which is matched by a given
/// matcher.
///
/// This takes two arguments:
///
///  * a specification of the field against which to match, and
///  * an inner [`Matcher`][crate::matcher::Matcher] to apply to that field.
///
/// For example:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct IntField {
///   int: i32
/// }
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(IntField{int: 32}, field!(IntField.int, eq(32)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Tuple structs are also supported via the index syntax:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct IntField(i32);
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(IntField(32), field!(IntField.0, eq(32)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Enums are also supported, in which case only the specified variant is
/// matched:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// enum MyEnum {
///     A(i32),
///     B,
/// }
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(MyEnum::A(32), field!(MyEnum::A.0, eq(32)))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> TestResult<()> {
/// verify_that!(MyEnum::B, field!(MyEnum::A.0, eq(32)))?; // Fails: wrong enum variant
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// The structure or enum may also be referenced from a separate module:
///
/// ```
/// # use test_that::prelude::*;
/// mod a_module {
///     #[derive(Debug)]
///     pub struct AStruct(pub i32);
/// }
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(a_module::AStruct(32), field!(a_module::AStruct.0, eq(32)))?;
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Nested structures are *not supported*, however:
///
/// ```compile_fail
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct InnerStruct(i32);
/// #[derive(Debug)]
/// struct OuterStruct {
///     inner: InnerStruct,
/// }
/// # fn should_not_compile() -> TestResult<()> {
/// verify_that!(value, field!(OuterStruct.inner.0, eq(32)))?; // Does not compile
/// #     Ok(())
/// # }
/// ```
///
/// See also the macro [`result_of`][crate::matchers::result_of] for a matcher
/// which works for arbitrary closures.
#[macro_export]
#[doc(hidden)]
macro_rules! __field {
    ($($t:tt)*) => { $crate::field_internal!($($t)*) }
}

// Internal-only macro created so that the macro definition does not appear in
// generated documentation.
#[doc(hidden)]
#[macro_export]
macro_rules! field_internal {
    ($($t:ident)::+.$field:tt, $m:expr) => {{
        use $crate::matchers::__internal_unstable_do_not_depend_on_these::field_matcher;
        field_matcher(
            |o| {
                match o {
                    $($t)::* { $field: value, .. } => Some(value),
                    // The pattern below is unreachable if the type is a struct (as opposed to an
                    // enum). Since the macro can't know which it is, we always include it and just
                    // tell the compiler not to complain.
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            },
            &stringify!($field),
            $m)
    }};
}

/// Functions for use only by the declarative macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::{
        description::Description,
        matcher::{Describable, Matcher, MatcherResult},
    };
    use std::fmt::Debug;

    /// Creates a matcher to verify a specific field of the actual struct using
    /// the provided inner matcher.
    ///
    /// **For internal use only. API stablility is not guaranteed!**
    #[doc(hidden)]
    pub fn field_matcher<OuterT: Debug, InnerT: Debug, InnerMatcher: Matcher<InnerT>>(
        field_accessor: fn(&OuterT) -> Option<&InnerT>,
        field_path: &'static str,
        inner: InnerMatcher,
    ) -> impl Matcher<OuterT> {
        FieldMatcher { field_accessor, field_path, inner }
    }

    struct FieldMatcher<OuterT, InnerT, InnerMatcher> {
        field_accessor: fn(&OuterT) -> Option<&InnerT>,
        field_path: &'static str,
        inner: InnerMatcher,
    }

    impl<OuterT: Debug, InnerT: Debug, InnerMatcher: Matcher<InnerT>> Matcher<OuterT>
        for FieldMatcher<OuterT, InnerT, InnerMatcher>
    {
        fn matches(&self, actual: &OuterT) -> MatcherResult {
            if let Some(value) = (self.field_accessor)(actual) {
                self.inner.matches(value)
            } else {
                MatcherResult::NoMatch
            }
        }

        fn explain_match(&self, actual: &OuterT) -> Description {
            if let Some(actual) = (self.field_accessor)(actual) {
                format!(
                    "which has field `{}`, {}",
                    self.field_path,
                    self.inner.explain_match(actual)
                )
                .into()
            } else {
                let formatted_actual_value = format!("{actual:?}");
                let without_fields = formatted_actual_value.split('(').next().unwrap_or("");
                let without_fields = without_fields.split('{').next().unwrap_or("").trim_end();
                format!("which has the wrong enum variant `{without_fields}`").into()
            }
        }
    }

    impl<OuterT, InnerT, InnerMatcher: Describable> Describable
        for FieldMatcher<OuterT, InnerT, InnerMatcher>
    {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "has field `{}`, which {}",
                self.field_path,
                self.inner.describe(matcher_result)
            )
            .into()
        }
    }
}
