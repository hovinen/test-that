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

/// **Deprecated**. Please use
/// [matches_pattern!][crate::matchers::matches_pattern] instead.
#[cfg(feature = "googletest-compat")]
#[cfg_attr(feature = "googletest-migrate", deprecated(note = "Use matches_pattern! instead"))]
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
    // Strip leading :: (Rust 2021: ::Name means extern crate, not crate root; strip to use the
    // plain path which resolves correctly in the caller's scope).
    (:: $($rest:tt)*) => {
        $crate::field_internal!($($rest)*)
    };
    // Type-parameterized paths: strip the type params for the match pattern — Rust infers the
    // concrete type from the value being matched. Using $(:ty),* stops at > without ambiguity
    // because `ty` cannot begin with `>`.
    ($($t:ident)::+ < $($type_params:ty),* $(,)? > . $field:tt, {$($m:tt)*}) => {
        $crate::field_internal!($($t)::+.$field, $crate::__matcher_expr!({$($m)*}))
    };
    ($($t:ident)::+ < $($type_params:ty),* $(,)? > . $field:tt, [$($m:tt)*]) => {
        $crate::field_internal!($($t)::+.$field, $crate::__matcher_expr!([$($m)*]))
    };
    ($($t:ident)::+ < $($type_params:ty),* $(,)? > . $field:tt, $m:expr) => {
        $crate::field_internal!($($t)::+.$field, $m)
    };
    ($($t:ident)::+.$field:tt, {$($m:tt)*}) => {
        $crate::field_internal!($($t)::+.$field, $crate::__matcher_expr!({$($m)*}))
    };
    ($($t:ident)::+.$field:tt, [$($m:tt)*]) => {
        $crate::field_internal!($($t)::+.$field, $crate::__matcher_expr!([$($m)*]))
    };
    ($($t:ident)::+.$field:tt, $m:expr) => {{
        $crate::matchers::__internal::field_matcher(
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
            &stringify!($field))
        .with($m)
    }};
}

/// Functions for use only by the declarative macros in this module.
///
/// **For internal use only. API stability is not guaranteed!**
#[doc(hidden)]
pub mod __internal {
    use crate::{
        description::Description,
        matcher::{Describable, Matcher, MatcherResult},
    };
    use core::fmt::Debug;

    /// Creates a matcher to verify a specific field of the actual struct.
    ///
    /// The inner matcher to apply is supplied via [`FieldMatcherStage::with`].
    ///
    /// **For internal use only. API stability is not guaranteed!**
    #[doc(hidden)]
    pub fn field_matcher<OuterT: Debug, InnerT: Debug>(
        field_accessor: fn(&OuterT) -> Option<&InnerT>,
        field_path: &'static str,
    ) -> FieldMatcherStage<OuterT, InnerT> {
        FieldMatcherStage { field_accessor, field_path }
    }

    /// **For internal use only. API stability is not guaranteed!**
    #[doc(hidden)]
    pub struct FieldMatcherStage<OuterT, InnerT> {
        field_accessor: fn(&OuterT) -> Option<&InnerT>,
        field_path: &'static str,
    }

    impl<OuterT: Debug, InnerT: Debug> FieldMatcherStage<OuterT, InnerT> {
        /// **For internal use only. API stability is not guaranteed!**
        #[doc(hidden)]
        pub fn with<InnerMatcher: Matcher<InnerT>>(
            self,
            inner: InnerMatcher,
        ) -> impl Matcher<OuterT> {
            FieldMatcher { field_accessor: self.field_accessor, field_path: self.field_path, inner }
        }
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
