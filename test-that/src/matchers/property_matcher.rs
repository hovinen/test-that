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

// There are no visible documentation elements in this module; the declarative
// macro is documented in the matcher module.
#![doc(hidden)]

/// Matches an object which, upon calling the given method on it with the given
/// arguments, produces a value matched by the given inner matcher.
///
/// This is particularly useful as a nested matcher when the desired
/// property cannot be accessed through a field and must instead be
/// extracted through a method call. For example:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct {
///     a_field: u32,
/// }
///
/// impl MyStruct {
///     pub fn get_a_field(&self) -> u32 { self.a_field }
/// }
///
/// let value = vec![MyStruct { a_field: 100 }];
/// verify_that!(value, contains(property!(MyStruct.get_a_field(), eq(100))))
/// #    .unwrap();
/// ```
///
/// **Important**: The method should be pure function with a deterministic
/// output and no side effects. In particular, in the event of an assertion
/// failure, it will be invoked a second time, with the assertion failure output
/// reflecting the *second* invocation.
///
/// This macro is analogous to [`field`][crate::matchers::field], except that it
/// extracts the datum to be matched from the given object by invoking a method
/// rather than accessing a field.
///
/// The list of arguments may optionally have a trailing comma.
///
/// ## Methods returning references
///
/// If the method returns a *reference*, then it must be preceded by a `*`:
///
/// ```
/// # use test_that::prelude::*;
/// # #[derive(Debug)]
/// # pub struct MyStruct {
/// #     a_field: u32,
/// # }
/// impl MyStruct {
///     pub fn get_a_field(&self) -> &u32 { &self.a_field }
/// }
///
/// # let value = vec![MyStruct { a_field: 100 }];
/// verify_that!(value, contains(property!(*MyStruct.get_a_field(), eq(100))))
/// #    .unwrap();
/// ```
///
/// This is due to the parallel structure between the matcher and the data being
/// matched. The following, after all, does not compile:
///
/// ```compile_fail
/// let value1 = 123;
/// let value2 = 234;
/// assert!(value1 == &value2);  // Comparing i32 with &i32
/// ```
///
/// The same holds if the method returns an array slice:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct {
///     a_vec: Vec<u32>,
/// }
/// impl MyStruct {
///     pub fn get_a_slice(&self) -> &[u32] { &self.a_vec }
/// }
///
/// let value = MyStruct { a_vec: vec![1, 2, 3] };
/// verify_that!(value, property!(*MyStruct.get_a_slice(), contains(eq(1))))
/// #    .unwrap();
/// ```
///
/// Again, when iterating over an array slice `&[T]`, one gets `&T`, not `T`. So
/// one must "dereference" the slice to an array to match the elements.
///
/// Alternatively (though more verbosely), one can use the [`points_to`] matcher:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct {
///     a_vec: Vec<u32>,
/// }
/// impl MyStruct {
///     pub fn get_a_slice(&self) -> &[u32] { &self.a_vec }
/// }
///
/// let value = MyStruct { a_vec: vec![1, 2, 3] };
/// verify_that!(value, property!(MyStruct.get_a_slice(), contains(points_to(eq(1)))))
/// #    .unwrap();
/// ```
///
/// When the method returns a _string slice_, one does _not_ add `*`:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct {
///     a_string: String,
/// }
/// impl MyStruct {
///     pub fn get_a_string(&self) -> &str { &self.a_string }
/// }
///
/// let value = MyStruct { a_string: "A string".into() };
/// verify_that!(value, property!(MyStruct.get_a_string(), eq("A string")))
/// #    .unwrap();
/// ```
///
/// This is because the value against which one is matching is _already_ a `&str`,
/// so the types match.
///
/// ## Methods taking additional arguments
///
/// The method may also take additional arguments:
///
/// ```
/// # use test_that::prelude::*;
/// # #[derive(Debug)]
/// # pub struct MyStruct {
/// #     a_field: u32,
/// # }
/// impl MyStruct {
///     pub fn add_to_a_field(&self, a: u32) -> u32 { self.a_field + a }
/// }
///
/// # let value = vec![MyStruct { a_field: 100 }];
/// verify_that!(value, contains(property!(MyStruct.add_to_a_field(50), eq(150))))
/// #    .unwrap();
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __property {
    ($($t:tt)*) => { $crate::property_internal!($($t)*) }
}

// Internal-only macro created so that the macro definition does not appear in
// generated documentation.
#[doc(hidden)]
#[macro_export]
macro_rules! property_internal {
    ($($t:ident)::+.$method:tt($($argument:tt),* $(,)?), $m:expr) => {{
        use $crate::matchers::__internal_unstable_do_not_depend_on_these::property_matcher;
        property_matcher::<$($t)::+, _, _, _, _>(
            stringify!($method($($argument),*)),
            $m,
            |o: &$($t)::+, inner| inner.matches(&o.$method($($argument),*)),
            |inner, r| $crate::matcher::Describable::describe(inner, r),
            |o: &$($t)::+, inner| {
                let actual_inner = o.$method($($argument),*);
                ::std::convert::Into::into(format!(
                    "whose property `{}` is `{:#?}`, {}",
                    stringify!($method($($argument),*)),
                    actual_inner,
                    inner.explain_match(&actual_inner)
                ))
            },
        )
    }};

    (* $($t:ident)::+.$method:tt($($argument:tt),* $(,)?), $m:expr) => {{
        use $crate::matchers::__internal_unstable_do_not_depend_on_these::property_matcher;
        let inner = $crate::matchers::points_to($m);
        property_matcher::<$($t)::+, _, _, _, _>(
            stringify!($method($($argument),*)),
            inner,
            |o: &$($t)::+, inner| inner.matches(&o.$method($($argument),*)),
            |inner, r| $crate::matcher::Describable::describe(inner, r),
            |o: &$($t)::+, inner| {
                let actual_inner = o.$method($($argument),*);
                ::std::convert::Into::into(format!(
                    "whose property `{}` is `{:#?}`, {}",
                    stringify!($method($($argument),*)),
                    actual_inner,
                    inner.explain_match(&actual_inner)
                ))
            },
        )
    }};
}

/// Items for use only by the declarative macros in this module.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub mod internal {
    use crate::{
        description::Description,
        matcher::{Describable, Matcher, MatcherResult},
    };
    use std::{fmt::Debug, marker::PhantomData};

    /// **For internal use only. API stablility is not guaranteed!**
    ///
    /// The callbacks let the property's extraction + matching live entirely
    /// inside the closure bodies. That way the closure's return type — which
    /// may bind a lifetime from the input (e.g. `Option<&'a dyn Error>` from
    /// `Error::source`) — never has to appear in `property_matcher`'s type
    /// signature, which would otherwise force a fixed `InnerT` and reject any
    /// HRTB-return extractor.
    ///
    /// The inner matcher is stored in the returned matcher and passed by
    /// reference to each closure, so it is not captured (and not cloned). A
    /// downside is that `describe` cannot delegate to `inner.describe(...)`
    /// generically (Rust cannot disambiguate which `Matcher<T>` impl to pick
    /// when the inner is polymorphic in `T`); instead the macro stringifies
    /// the inner expression for the describe output.
    #[doc(hidden)]
    pub fn property_matcher<OuterT, InnerMatcherT, ApplyFn, DescribeFn, ExplainFn>(
        property_desc: &'static str,
        inner: InnerMatcherT,
        apply: ApplyFn,
        describe: DescribeFn,
        explain: ExplainFn,
    ) -> impl Matcher<OuterT>
    where
        OuterT: Debug,
        InnerMatcherT: Describable,
        ApplyFn: Fn(&OuterT, &InnerMatcherT) -> MatcherResult,
        DescribeFn: Fn(&InnerMatcherT, MatcherResult) -> Description,
        ExplainFn: Fn(&OuterT, &InnerMatcherT) -> Description,
    {
        PropertyMatcher { property_desc, inner, apply, describe, explain, phantom: PhantomData }
    }

    struct PropertyMatcher<OuterT, InnerMatcherT, ApplyFn, DescribeFn, ExplainFn> {
        property_desc: &'static str,
        inner: InnerMatcherT,
        apply: ApplyFn,
        describe: DescribeFn,
        explain: ExplainFn,
        phantom: PhantomData<fn(&OuterT)>,
    }

    impl<OuterT, InnerT, ApplyFn, DescribeFn, ExplainFn> Matcher<OuterT>
        for PropertyMatcher<OuterT, InnerT, ApplyFn, DescribeFn, ExplainFn>
    where
        OuterT: Debug,
        ApplyFn: Fn(&OuterT, &InnerT) -> MatcherResult,
        DescribeFn: Fn(&InnerT, MatcherResult) -> Description,
        ExplainFn: Fn(&OuterT, &InnerT) -> Description,
    {
        fn matches(&self, actual: &OuterT) -> MatcherResult {
            (self.apply)(actual, &self.inner)
        }

        fn explain_match(&self, actual: &OuterT) -> Description {
            (self.explain)(actual, &self.inner)
        }
    }

    impl<OuterT, InnerT, ApplyFn, DescribeFn, ExplainFn> Describable
        for PropertyMatcher<OuterT, InnerT, ApplyFn, DescribeFn, ExplainFn>
    where
        DescribeFn: Fn(&InnerT, MatcherResult) -> Description,
    {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "has property `{}`, which {}",
                self.property_desc,
                (self.describe)(&self.inner, matcher_result)
            )
            .into()
        }
    }
}
