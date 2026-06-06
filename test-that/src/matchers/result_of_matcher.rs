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
/// The same holds if the method returns a slice:
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
/// Alternatively (though more verbosely), one can use the [`points_to`][crate::matchers::points_to]
/// matcher:
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
/// verify_that!(value, property!(MyStruct.get_a_slice(), points_to(contains(eq(1)))))
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
macro_rules! __result_of {
    ($($t:tt)*) => { $crate::result_of_internal!($($t)*) }
}

// Internal-only macro created so that the macro definition does not appear in
// generated documentation.
#[doc(hidden)]
#[macro_export]
macro_rules! result_of_internal {
    (|$param:ident: $type:ty| $body:expr, $matcher:expr) => {{
        $crate::matchers::__internal_unstable_do_not_depend_on_these::result_of(
            stringify!($closure),
            $matcher,
            |$param: $type, matcher| $crate::matcher::Matcher::matches(matcher, &$body),
            |result, matcher| $crate::matcher::Describable::describe(matcher, result),
            |$param: $type, matcher| {
                ::std::convert::Into::into(format!(
                    concat!(
                        "which after applying `|",
                        stringify!($param),
                        ": ",
                        stringify!($type),
                        "| ",
                        stringify!($body),
                        "` results in `{:#?}`, {}"
                    ),
                    &$body,
                    $crate::matcher::Matcher::explain_match(matcher, &$body)
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

    pub fn result_of<Input, InnerMatcher, ApplyFn, DescribeFn, ExplainFn>(
        definition: &'static str,
        matcher: InnerMatcher,
        apply: ApplyFn,
        describe: DescribeFn,
        explain: ExplainFn,
    ) -> ResultOfMatcher<Input, InnerMatcher, ApplyFn, DescribeFn, ExplainFn>
    where
        Input: Debug + ?Sized,
        ApplyFn: Fn(&Input, &InnerMatcher) -> MatcherResult,
        DescribeFn: Fn(MatcherResult, &InnerMatcher) -> Description,
        ExplainFn: Fn(&Input, &InnerMatcher) -> Description,
    {
        ResultOfMatcher { definition, matcher, apply, describe, explain, phantom_1: PhantomData }
    }

    pub struct ResultOfMatcher<Input: ?Sized, InnerMatcher, ApplyFn, DescribeFn, ExplainFn>
    where
        ApplyFn: Fn(&Input, &InnerMatcher) -> MatcherResult,
        DescribeFn: Fn(MatcherResult, &InnerMatcher) -> Description,
        ExplainFn: Fn(&Input, &InnerMatcher) -> Description,
    {
        definition: &'static str,
        matcher: InnerMatcher,
        apply: ApplyFn,
        describe: DescribeFn,
        explain: ExplainFn,
        phantom_1: PhantomData<Input>,
    }

    impl<Input, InnerMatcher, ApplyFn, DescribeFn, ExplainFn> Matcher<Input>
        for ResultOfMatcher<Input, InnerMatcher, ApplyFn, DescribeFn, ExplainFn>
    where
        Input: Debug + ?Sized,
        ApplyFn: Fn(&Input, &InnerMatcher) -> MatcherResult,
        DescribeFn: Fn(MatcherResult, &InnerMatcher) -> Description,
        ExplainFn: Fn(&Input, &InnerMatcher) -> Description,
    {
        fn matches(&self, actual: &Input) -> MatcherResult {
            (self.apply)(actual, &self.matcher)
        }

        fn explain_match(&self, actual: &Input) -> Description {
            (self.explain)(actual, &self.matcher)
        }
    }

    impl<Input: ?Sized, InnerMatcher, ApplyFn, DescribeFn, ExplainFn> Describable
        for ResultOfMatcher<Input, InnerMatcher, ApplyFn, DescribeFn, ExplainFn>
    where
        ApplyFn: Fn(&Input, &InnerMatcher) -> MatcherResult,
        DescribeFn: Fn(MatcherResult, &InnerMatcher) -> Description,
        ExplainFn: Fn(&Input, &InnerMatcher) -> Description,
    {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "result of `{}`, which {}",
                self.definition,
                (self.describe)(matcher_result, &self.matcher)
            )
            .into()
        }
    }
}
