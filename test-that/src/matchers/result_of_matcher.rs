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

/// Matches an item which, upon applying the given closure to it, produces a
/// value matched by the given inner matcher.
///
/// The closure must accept a single parameter: a shared reference to the
/// item. The type must be explicitly stated in the closure. For example:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct {
///     a_field: u32,
/// }
///
/// let value = MyStruct { a_field: 100 };
/// verify_that!(value, result_of!(|s: &MyStruct| s.a_field, eq(100)))
/// #    .unwrap();
/// ```
///
/// Closures taking exclusive references to or ownership of the item are
/// not permitted.
///
/// The closure may also invoke methods on the item. For example:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct {
///     a_field: u32,
/// }
///
/// impl MyStruct {
///     fn get_a_field(&self) -> u32 {
///         self.a_field
///     }
/// }
///
/// let value = MyStruct { a_field: 100 };
/// verify_that!(value, result_of!(|s: &MyStruct| s.get_a_field(), eq(100)))
/// #    .unwrap();
/// ```
///
/// **Important**: The closure should be a pure function with a deterministic
/// output and no side effects. In particular, in the event of an assertion
/// failure, it will be invoked a second time, with the assertion failure output
/// reflecting the *second* invocation.
///
/// ## Closures returning references
///
/// The closure may also return a reference whose lifetime is bound by the
/// self parameter. For example:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct {
///     a_field: u32,
/// }
///
/// impl MyStruct {
///     fn get_ref_to_a_field(&self) -> &u32 {
///         &self.a_field
///     }
/// }
///
/// let value = MyStruct { a_field: 100 };
/// verify_that!(value, result_of!(|s: &MyStruct| s.get_ref_to_a_field(), points_to(eq(100))))
/// #    .unwrap();
/// ```
///
/// The closure may also return a reference whose lifetime is given by the
/// type itself. For example:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct<'a> {
///     a_field: &'a u32,
/// }
///
/// let content = 100;
/// let value = MyStruct { a_field: &content };
/// verify_that!(value, result_of!(|s: &MyStruct| s.a_field, points_to(eq(100))))
/// #    .unwrap();
/// ```
///
/// In both cases, since the closure is returning a reference, one must
/// "dereference" the output using the matcher
/// [`points_to`][crate::matchers::points_to].
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
/// verify_that!(value, result_of!(|s: &MyStruct| s.get_a_string(), eq("A string")))
/// #    .unwrap();
/// ```
///
/// This is because the value against which one is matching is _already_ a
/// `&str`, so the types match.
///
/// ## Closures moving data out of the struct are allowed
///
/// Normally one cannot create a closure which takes a shared reference to a
/// struct and moves data out of that struct.
///
/// ```compile_fail
/// pub struct MyStruct {
///     a_string: String,
/// }
/// let closure: |s: &MyStruct| -> String = |s| s.a_string; // Error: Moving data from behind a shared reference!
/// ```
///
/// However, with `result_of!`, this is allowed:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct {
///     a_string: String,
/// }
///
/// let value = MyStruct { a_string: "A string".into() };
/// verify_that!(value, result_of!(|s: &MyStruct| s.a_string, eq("A string")))
/// #    .unwrap();
/// ```
///
/// This is because the closure is rewritten by the macro so that it no longer
/// moves data.
///
/// Any methods the closure invokes must, however, still take a shared reference
/// as their `self` parameter. So the following does not compile:
///
/// ```compile_fail
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// pub struct MyStruct {
///     a_string: String,
/// }
///
/// impl MyStruct {
///     fn get_a_string(self) -> String {
///         self.a_string
///     }
/// }
///
/// let value = MyStruct { a_string: "A string".into() };
/// verify_that!(value, result_of!(|s: &MyStruct| s.get_a_string(), eq("A string")))
/// #    .unwrap();
/// ```
///
/// ## Shorthand for matching against containers
///
/// One can use the same `[...]` and `{...}` as in [`verify_that!`] and fields
/// to match against containers.
///
/// Use `[...]` to enforce order. This is equivalent to [`contains_exactly!`]
/// with [`in_order()`].
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct MyStruct {
///     a_vec: Vec<u32>,
/// }
///
/// let my_struct = MyStruct { a_vec: vec![1, 2, 3] };
/// verify_that!(my_struct, result_of!(|s: &MyStruct| s.a_vec, [eq(1), gt(1), le(4)]))
/// #    .unwrap();
/// ```
///
/// Use `{...}` to match elements in any order. This is equivalent to
/// [`contains_exactly!`].
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct MyStruct {
///     a_vec: Vec<u32>,
/// }
///
/// let my_struct = MyStruct { a_vec: vec![1, 2, 3] };
/// verify_that!(my_struct, result_of!(|s: &MyStruct| s.a_vec, {eq(3), gt(1), eq(1)}))
/// #    .unwrap();
/// ```
///
/// This shorthand notation works _only_ for direct arguments in the macro. If
/// the container matcher is nested inside another matcher, one must use
/// `contains_exactly!`.
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct MyStruct {
///     maybe_a_vec: Option<Vec<u32>>,
/// }
///
/// let my_struct = MyStruct { maybe_a_vec: Some(vec![1, 2, 3]) };
/// verify_that!(my_struct, result_of!(
///     |s: &MyStruct| s.maybe_a_vec,
///     some(contains_exactly![eq(1), gt(1), eq(3)].in_order())
/// ))
/// #    .unwrap();
/// ```
///
/// [`contains_exactly!`]: crate::matchers::containers::contains_exactly
/// [`in_order()`]: crate::matchers::containers::ContainerContainsUnorderedMatcher::in_order
#[macro_export]
#[doc(hidden)]
macro_rules! __result_of {
    ($($t:tt)*) => { $crate::__result_of_internal!($($t)*) }
}

// Internal-only macro created so that the macro definition does not appear in
// generated documentation.
#[doc(hidden)]
#[macro_export]
macro_rules! __result_of_internal {
    (|$param:ident: $type:ty| $body:expr, {$($matcher:tt)*} $(,)?) => {
        $crate::__result_of_internal!(|$param: $type| $body, $crate::__matcher_expr!({$($matcher)*}),)
    };
    (|$param:ident: $type:ty| $body:expr, [$($matcher:tt)*] $(,)?) => {
        $crate::__result_of_internal!(|$param: $type| $body, $crate::__matcher_expr!([$($matcher)*]),)
    };
    (|$param:ident: $type:ty| $body:expr, $matcher:expr $(,)?) => {{
        $crate::matchers::__internal::ResultOfMatcher::new(
            concat!("|", stringify!($param), ": ", stringify!($type), "| ", stringify!($body)),
            $matcher,
            |$param: $type, matcher| $crate::matcher::Matcher::matches(matcher, &$body),
            |result, matcher| $crate::matcher::Describable::describe(matcher, result),
            |$param: $type, definition, matcher| {
                ::core::convert::Into::into($crate::__alloc::format!(
                    "which after applying `{}` results in `{:#?}`, {}",
                    definition,
                    &$body,
                    $crate::matcher::Matcher::explain_match(matcher, &$body)
                ))
            },
        )
    }};
}

/// Items for use only by the declarative macros in this module.
///
/// **For internal use only. API stability is not guaranteed!**
#[doc(hidden)]
pub mod __internal {
    use crate::{
        description::Description,
        matcher::{Describable, Matcher, MatcherResult},
    };
    use core::{fmt::Debug, marker::PhantomData};

    pub struct ResultOfMatcher<Input: ?Sized, InnerMatcher, ApplyFn, DescribeFn, ExplainFn>
    where
        ApplyFn: Fn(&Input, &InnerMatcher) -> MatcherResult,
        DescribeFn: Fn(MatcherResult, &InnerMatcher) -> Description,
        ExplainFn: Fn(&Input, &'static str, &InnerMatcher) -> Description,
    {
        definition: &'static str,
        matcher: InnerMatcher,
        apply: ApplyFn,
        describe: DescribeFn,
        explain: ExplainFn,
        phantom_1: PhantomData<Input>,
    }

    impl<Input, InnerMatcher, ApplyFn, DescribeFn, ExplainFn>
        ResultOfMatcher<Input, InnerMatcher, ApplyFn, DescribeFn, ExplainFn>
    where
        Input: ?Sized + Debug,
        ApplyFn: Fn(&Input, &InnerMatcher) -> MatcherResult,
        DescribeFn: Fn(MatcherResult, &InnerMatcher) -> Description,
        ExplainFn: Fn(&Input, &'static str, &InnerMatcher) -> Description,
    {
        pub fn new(
            definition: &'static str,
            matcher: InnerMatcher,
            apply: ApplyFn,
            describe: DescribeFn,
            explain: ExplainFn,
        ) -> Self {
            ResultOfMatcher {
                definition,
                matcher,
                apply,
                describe,
                explain,
                phantom_1: PhantomData,
            }
        }

        /// Overrides the default depiction of the closure in the description
        /// and explanation with the given string.
        pub fn with_custom_definition(self, definition: &'static str) -> Self {
            Self { definition, ..self }
        }
    }

    impl<Input, InnerMatcher, ApplyFn, DescribeFn, ExplainFn> Matcher<Input>
        for ResultOfMatcher<Input, InnerMatcher, ApplyFn, DescribeFn, ExplainFn>
    where
        Input: Debug + ?Sized,
        ApplyFn: Fn(&Input, &InnerMatcher) -> MatcherResult,
        DescribeFn: Fn(MatcherResult, &InnerMatcher) -> Description,
        ExplainFn: Fn(&Input, &'static str, &InnerMatcher) -> Description,
    {
        fn matches(&self, actual: &Input) -> MatcherResult {
            (self.apply)(actual, &self.matcher)
        }

        fn explain_match(&self, actual: &Input) -> Description {
            (self.explain)(actual, self.definition, &self.matcher)
        }
    }

    impl<Input: ?Sized, InnerMatcher, ApplyFn, DescribeFn, ExplainFn> Describable
        for ResultOfMatcher<Input, InnerMatcher, ApplyFn, DescribeFn, ExplainFn>
    where
        ApplyFn: Fn(&Input, &InnerMatcher) -> MatcherResult,
        DescribeFn: Fn(MatcherResult, &InnerMatcher) -> Description,
        ExplainFn: Fn(&Input, &'static str, &InnerMatcher) -> Description,
    {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            format!(
                "result of applying `{}` {}",
                self.definition,
                (self.describe)(matcher_result, &self.matcher)
            )
            .into()
        }
    }
}
