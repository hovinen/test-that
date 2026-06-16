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

/// Matches a value according to a pattern of matchers.
///
/// This takes as an argument a specification similar to a struct or enum
/// initialiser, where each value is a [`Matcher`][crate::matcher::Matcher]
/// which is applied to the corresponding field.
///
/// This can be used to match arbitrary combinations of fields on structures
/// using arbitrary matchers:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct MyStruct {
///     a_field: String,
///     another_field: String,
/// }
///
/// let my_struct = MyStruct {
///     a_field: "Something to believe in".into(),
///     another_field: "Something else".into()
/// };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_field: starts_with("Something"),
///     another_field: ends_with("else"),
/// }))
/// #     .unwrap();
/// ```
///
/// It is not required to include all named fields in the specification. Omitted
/// fields have no effect on the output of the matcher.
///
/// ```
/// # use test_that::prelude::*;
/// # #[derive(Debug)]
/// # struct MyStruct {
/// #     a_field: String,
/// #     another_field: String,
/// # }
/// #
/// # let my_struct = MyStruct {
/// #     a_field: "Something to believe in".into(),
/// #     another_field: "Something else".into()
/// # };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_field: starts_with("Something"),
///     // another_field is missing, so it may be anything.
/// }))
/// #     .unwrap();
/// ```
///
/// One can use it recursively to match nested structures:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct MyStruct {
///     a_nested_struct: MyInnerStruct,
/// }
///
/// #[derive(Debug)]
/// struct MyInnerStruct {
///     a_field: String,
/// }
///
/// let my_struct = MyStruct {
///     a_nested_struct: MyInnerStruct { a_field: "Something to believe in".into() },
/// };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_nested_struct: matches_pattern!(MyInnerStruct {
///         a_field: starts_with("Something"),
///     }),
/// }))
/// #     .unwrap();
/// ```
///
/// One can use the alias [`pat`][crate::matchers::pat] to make this less
/// verbose:
///
/// ```
/// # use test_that::prelude::*;
/// # #[derive(Debug)]
/// # struct MyStruct {
/// #     a_nested_struct: MyInnerStruct,
/// # }
/// #
/// # #[derive(Debug)]
/// # struct MyInnerStruct {
/// #     a_field: String,
/// # }
/// #
/// # let my_struct = MyStruct {
/// #     a_nested_struct: MyInnerStruct { a_field: "Something to believe in".into() },
/// # };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_nested_struct: pat!(MyInnerStruct {
///         a_field: starts_with("Something"),
///     }),
/// }))
/// #     .unwrap();
/// ```
///
/// In addition to fields, one can match on the outputs of methods
/// ("properties"):
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct MyStruct {
///     a_field: String,
/// }
///
/// impl MyStruct {
///     fn get_a_field(&self) -> String { self.a_field.clone() }
/// }
///
/// let my_struct = MyStruct { a_field: "Something to believe in".into() };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     get_a_field(): starts_with("Something"),
/// }))
/// #     .unwrap();
/// ```
///
/// **Important**: The method should be pure function with a deterministic
/// output and no side effects. In particular, in the event of an assertion
/// failure, it will be invoked a second time, with the assertion failure output
/// reflecting the *second* invocation.
///
/// These may also include extra parameters you pass in:
///
/// ```
/// # use test_that::prelude::*;
/// # #[derive(Debug)]
/// # struct MyStruct {
/// #     a_field: String,
/// # }
/// #
/// impl MyStruct {
///     fn append_to_a_field(&self, suffix: &str) -> String { self.a_field.clone() + suffix }
/// }
///
/// # let my_struct = MyStruct { a_field: "Something to believe in".into() };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     append_to_a_field("a suffix"): ends_with("a suffix"),
/// }))
/// #     .unwrap();
/// ```
///
/// If the method returns a reference, you must "dereference" it. Either preceed
/// it with `*`:
///
/// ```
/// # use test_that::prelude::*;
/// # #[derive(Debug)]
/// # struct MyStruct {
/// #     a_field: String,
/// # }
/// #
/// impl MyStruct {
///     fn get_a_field_ref(&self) -> &String { &self.a_field }
/// }
///
/// # let my_struct = MyStruct { a_field: "Something to believe in".into() };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     *get_a_field_ref(): starts_with("Something"),
/// }))
/// #    .unwrap();
/// ```
///
/// Or use [`points_to`](crate::matchers::points_to):
///
/// ```
/// # use test_that::prelude::*;
/// # #[derive(Debug)]
/// # struct MyStruct {
/// #     a_field: String,
/// # }
/// #
/// impl MyStruct {
///     fn get_a_field_ref(&self) -> &String { &self.a_field }
/// }
///
/// # let my_struct = MyStruct { a_field: "Something to believe in".into() };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     get_a_field_ref(): points_to(starts_with("Something")),
/// }))
/// #    .unwrap();
/// ```
///
/// This is also the case if the method returns an array slice:
///
/// ```
/// # use test_that::prelude::*;
/// # #[derive(Debug)]
/// # struct MyStruct {
/// #     a_vec: Vec<u32>,
/// # }
/// #
/// impl MyStruct {
///     fn get_a_slice(&self) -> &[u32] { &self.a_vec }
/// }
///
/// # let my_struct = MyStruct { a_vec: vec![1, 2, 3] };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     *get_a_slice(): contains(eq(1)),
/// }))
/// #    .unwrap();
/// ```
///
/// No preceding `*` should be used when the method returns a _string_ slice:
///
/// ```
/// # use test_that::prelude::*;
/// # #[derive(Debug)]
/// pub struct MyStruct {
///     a_string: String,
/// }
/// impl MyStruct {
///     pub fn get_a_string(&self) -> &str { &self.a_string }
/// }
///
/// let value = MyStruct { a_string: "A string".into() };
/// verify_that!(value, matches_pattern!( MyStruct {
///     get_a_string(): eq("A string"),
/// }))
/// #    .unwrap();
/// ```
///
/// One can also match tuple structs with up to 10 fields. In this case, all
/// fields must have matchers:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct MyTupleStruct(String, String);
///
/// let my_struct = MyTupleStruct("Something".into(), "Some other thing".into());
/// verify_that!(
///     my_struct,
///     matches_pattern!(MyTupleStruct(eq("Something"), eq("Some other thing")))
/// )
/// #    .unwrap();
/// ```
///
/// One can also match enum values, including specific variants and fields:
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// enum MyEnum {
///     A(u32),
///     B,
/// }
///
/// # fn should_pass() -> TestResult<()> {
/// verify_that!(MyEnum::A(123), matches_pattern!(MyEnum::A(eq(123))))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> TestResult<()> {
/// verify_that!(MyEnum::B, matches_pattern!(MyEnum::A(eq(123))))?; // Fails - wrong enum variant
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// This macro does not support plain (non-struct) tuples. Use the macro
/// [`tuple`] for that purpose.
///
/// Trailing commas are allowed (but not required) in both ordinary and tuple
/// structs.
///
/// ## Shorthand for matching against containers
///
/// One can use the same `[...]` and `{...}` as in [`verify_that!`] and fields
/// to match against containers. Use `[...]` to enforce order. This is
/// equivalent to [`contains_exactly!`] with [`in_order()`].
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct MyStruct {
///     a_vec: Vec<u32>,
/// }
///
/// let my_struct = MyStruct { a_vec: vec![1, 2, 3] };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_vec: [eq(1), gt(1), le(4)],
/// }))
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
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     a_vec: {eq(3), gt(1), eq(1)},
/// }))
/// #    .unwrap();
/// ```
///
/// This works for both fields and properties.
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct MyStruct {
///     a_vec: Vec<u32>,
/// }
///
/// impl MyStruct {
///     fn get_a_vec(&self) -> Vec<u32> {
///         self.a_vec.clone()
///     }
/// }
///
/// let my_struct = MyStruct { a_vec: vec![1, 2, 3] };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     get_a_vec(): [eq(1), gt(1), eq(3)],
/// }))
/// #    .unwrap();
/// ```
///
/// It also works with the `*` notation for dereferencing a slice.
///
/// ```
/// # use test_that::prelude::*;
/// #[derive(Debug)]
/// struct MyStruct {
///     a_vec: Vec<u32>,
/// }
///
/// impl MyStruct {
///     fn get_a_slice(&self) -> &[u32] {
///         &self.a_vec
///     }
/// }
///
/// let my_struct = MyStruct { a_vec: vec![1, 2, 3] };
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     *get_a_slice(): [eq(1), gt(1), eq(3)],
/// }))
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
/// verify_that!(my_struct, matches_pattern!(MyStruct {
///     maybe_a_vec: some(contains_exactly![eq(1), gt(1), eq(3)].in_order()),
/// }))
/// #    .unwrap();
/// ```
///
/// [`contains_exactly!`]: crate::matchers::containers::contains_exactly
/// [`in_order()`]: crate::matchers::containers::ContainerContainsUnorderedMatcher::in_order
/// [`verify_that!`]: crate::verify_that
#[macro_export]
#[doc(hidden)]
macro_rules! __matches_pattern {
    ($($t:tt)*) => { $crate::matches_pattern_internal!($($t)*) }
}

// Internal-only macro created so that the macro definition does not appear in
// generated documentation.
#[doc(hidden)]
#[macro_export]
macro_rules! matches_pattern_internal {
    // Named struct fields: dispatch to @fwd accumulation.
    // $first:tt requires at least one token, so empty {} falls through to the accumulator
    // arm below, which handles patterns like AStruct {} (unit struct / enum variant).
    ([$($struct_name:tt)*], { $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(@fwd [], [$($struct_name)*], { $first $($rest)* })
    };

    // @fwd: accumulate named-struct field matchers one at a time.
    // Within each field kind, {}/[] arms precede $expr to prevent $expr committing to
    // {expr, expr} as a block expression (which would be a hard parse error).
    // "Last" arms: field with optional trailing comma and nothing else; produce the result.
    // "Multi" arms: field followed by $first:tt $($rest:tt)* (requires non-empty remainder).

    // Regular field, {} matcher
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { $field_name:ident : {$($matcher:tt)*} $(,)? }) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($($acc)* $crate::matchers::field!($($struct_name)*.$field_name, $crate::__matcher_expr!({$($matcher)*})),)
        )
    };
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { $field_name:ident : {$($matcher:tt)*}, $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(
            @fwd [$($acc)* $crate::matchers::field!($($struct_name)*.$field_name, $crate::__matcher_expr!({$($matcher)*})),],
            [$($struct_name)*], { $first $($rest)* }
        )
    };

    // Regular field, [] matcher
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { $field_name:ident : [$($matcher:tt)*] $(,)? }) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($($acc)* $crate::matchers::field!($($struct_name)*.$field_name, $crate::__matcher_expr!([$($matcher)*])),)
        )
    };
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { $field_name:ident : [$($matcher:tt)*], $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(
            @fwd [$($acc)* $crate::matchers::field!($($struct_name)*.$field_name, $crate::__matcher_expr!([$($matcher)*])),],
            [$($struct_name)*], { $first $($rest)* }
        )
    };

    // Regular field, $expr matcher
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { $field_name:ident : $matcher:expr $(,)? }) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($($acc)* $crate::matchers::field!($($struct_name)*.$field_name, $matcher),)
        )
    };
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { $field_name:ident : $matcher:expr, $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(
            @fwd [$($acc)* $crate::matchers::field!($($struct_name)*.$field_name, $matcher),],
            [$($struct_name)*], { $first $($rest)* }
        )
    };

    // Deref field, {} matcher
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { *$field_name:ident : {$($matcher:tt)*} $(,)? }) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($($acc)* $crate::matchers::field!($($struct_name)*.$field_name, $crate::matchers::points_to($crate::__matcher_expr!({$($matcher)*}))),)
        )
    };
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { *$field_name:ident : {$($matcher:tt)*}, $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(
            @fwd [$($acc)* $crate::matchers::field!($($struct_name)*.$field_name, $crate::matchers::points_to($crate::__matcher_expr!({$($matcher)*}))),],
            [$($struct_name)*], { $first $($rest)* }
        )
    };

    // Deref field, [] matcher
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { *$field_name:ident : [$($matcher:tt)*] $(,)? }) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($($acc)* $crate::matchers::field!($($struct_name)*.$field_name, $crate::matchers::points_to($crate::__matcher_expr!([$($matcher)*]))),)
        )
    };
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { *$field_name:ident : [$($matcher:tt)*], $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(
            @fwd [$($acc)* $crate::matchers::field!($($struct_name)*.$field_name, $crate::matchers::points_to($crate::__matcher_expr!([$($matcher)*]))),],
            [$($struct_name)*], { $first $($rest)* }
        )
    };

    // Deref field, $expr matcher
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { *$field_name:ident : $matcher:expr $(,)? }) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($($acc)* $crate::matchers::field!($($struct_name)*.$field_name, $crate::matchers::points_to($matcher)),)
        )
    };
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { *$field_name:ident : $matcher:expr, $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(
            @fwd [$($acc)* $crate::matchers::field!($($struct_name)*.$field_name, $crate::matchers::points_to($matcher)),],
            [$($struct_name)*], { $first $($rest)* }
        )
    };

    // Property, {} matcher
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { $property_name:ident($($argument:expr),* $(,)?) : {$($matcher:tt)*} $(,)? }) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($($acc)* $crate::matchers::result_of!(|s: &$($struct_name)*| s.$property_name($($argument),*), $crate::__matcher_expr!({$($matcher)*}),),)
        )
    };
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { $property_name:ident($($argument:expr),* $(,)?) : {$($matcher:tt)*}, $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(
            @fwd [$($acc)* $crate::matchers::result_of!(|s: &$($struct_name)*| s.$property_name($($argument),*), $crate::__matcher_expr!({$($matcher)*}),),],
            [$($struct_name)*], { $first $($rest)* }
        )
    };

    // Property, [] matcher
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { $property_name:ident($($argument:expr),* $(,)?) : [$($matcher:tt)*] $(,)? }) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($($acc)* $crate::matchers::result_of!(|s: &$($struct_name)*| s.$property_name($($argument),*), $crate::__matcher_expr!([$($matcher)*]),),)
        )
    };
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { $property_name:ident($($argument:expr),* $(,)?) : [$($matcher:tt)*], $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(
            @fwd [$($acc)* $crate::matchers::result_of!(|s: &$($struct_name)*| s.$property_name($($argument),*), $crate::__matcher_expr!([$($matcher)*]),),],
            [$($struct_name)*], { $first $($rest)* }
        )
    };

    // Property, $expr matcher
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr $(,)? }) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($($acc)* $crate::matchers::result_of!(|s: &$($struct_name)*| s.$property_name($($argument),*), $matcher,),)
        )
    };
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr, $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(
            @fwd [$($acc)* $crate::matchers::result_of!(|s: &$($struct_name)*| s.$property_name($($argument),*), $matcher,),],
            [$($struct_name)*], { $first $($rest)* }
        )
    };

    // Deref property, {} matcher
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { * $property_name:ident($($argument:expr),* $(,)?) : {$($matcher:tt)*} $(,)? }) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($($acc)* $crate::matchers::result_of!(|s: &$($struct_name)*| s.$property_name($($argument),*), $crate::matchers::points_to($crate::__matcher_expr!({$($matcher)*})),),)
        )
    };
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { * $property_name:ident($($argument:expr),* $(,)?) : {$($matcher:tt)*}, $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(
            @fwd [$($acc)* $crate::matchers::result_of!(|s: &$($struct_name)*| s.$property_name($($argument),*), $crate::matchers::points_to($crate::__matcher_expr!({$($matcher)*})),),],
            [$($struct_name)*], { $first $($rest)* }
        )
    };

    // Deref property, [] matcher
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { * $property_name:ident($($argument:expr),* $(,)?) : [$($matcher:tt)*] $(,)? }) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($($acc)* $crate::matchers::result_of!(|s: &$($struct_name)*| s.$property_name($($argument),*), $crate::matchers::points_to($crate::__matcher_expr!([$($matcher)*])),),)
        )
    };
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { * $property_name:ident($($argument:expr),* $(,)?) : [$($matcher:tt)*], $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(
            @fwd [$($acc)* $crate::matchers::result_of!(|s: &$($struct_name)*| s.$property_name($($argument),*), $crate::matchers::points_to($crate::__matcher_expr!([$($matcher)*])),),],
            [$($struct_name)*], { $first $($rest)* }
        )
    };

    // Deref property, $expr matcher
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { * $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr $(,)? }) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($($acc)* $crate::matchers::result_of!(|s: &$($struct_name)*| s.$property_name($($argument),*), $crate::matchers::points_to($matcher),),)
        )
    };
    (@fwd [$($acc:tt)*], [$($struct_name:tt)*], { * $property_name:ident($($argument:expr),* $(,)?) : $matcher:expr, $first:tt $($rest:tt)* }) => {
        $crate::matches_pattern_internal!(
            @fwd [$($acc)* $crate::matchers::result_of!(|s: &$($struct_name)*| s.$property_name($($argument),*), $crate::matchers::points_to($matcher),),],
            [$($struct_name)*], { $first $($rest)* }
        )
    };

    (
        [$($struct_name:tt)*],
    ) => {
        $crate::matchers::predicate(|v| matches!(v, $($struct_name)*))
            .with_description(
                concat!("is ", stringify!($($struct_name)*)),
                concat!("is not ", stringify!($($struct_name)*)),
            )
    };

    (
        [$($struct_name:tt)*],
        ({$($matcher:tt)*} $(,)?)
    ) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($crate::matchers::field!($($struct_name)*.0, $crate::__matcher_expr!({$($matcher)*})))
        )
    };

    (
        [$($struct_name:tt)*],
        ({$($matcher:tt)*}, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $crate::matchers::field!($($struct_name)*.0, $crate::__matcher_expr!({$($matcher)*}))
            ),
            [$($struct_name)*],
            1,
            ($($rest)*)
        )
    };

    (
        [$($struct_name:tt)*],
        ([$($matcher:tt)*] $(,)?)
    ) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($crate::matchers::field!($($struct_name)*.0, $crate::__matcher_expr!([$($matcher)*])))
        )
    };

    (
        [$($struct_name:tt)*],
        ([$($matcher:tt)*], $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $crate::matchers::field!($($struct_name)*.0, $crate::__matcher_expr!([$($matcher)*]))
            ),
            [$($struct_name)*],
            1,
            ($($rest)*)
        )
    };

    (
        [$($struct_name:tt)*],
        ($matcher:expr $(,)?)
    ) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!($crate::matchers::field!($($struct_name)*.0, $matcher))
        )
    };

    (
        [$($struct_name:tt)*],
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $crate::matchers::field!($($struct_name)*.0, $matcher)
            ),
            [$($struct_name)*],
            1,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        $field:tt,
        ({$($matcher:tt)*} $(,)?)
    ) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.$field, $crate::__matcher_expr!({$($matcher)*}))
            ),
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        $field:tt,
        ([$($matcher:tt)*] $(,)?)
    ) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.$field, $crate::__matcher_expr!([$($matcher)*]))
            ),
        )
    };

    // We need to repeat this once for every supported field position, unfortunately. There appears
    // to be no way in declarative macros to compute $field + 1 and have the result evaluated to a
    // token which can be used as a tuple index.
    //
    // The {}/[] multi arms must come before the generic $expr last arm to avoid
    // $expr committing to parsing {expr, expr} as a block expression and failing.
    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        1,
        ({$($matcher:tt)*}, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.1, $crate::__matcher_expr!({$($matcher)*}))
            ),
            [$($struct_name)*],
            2,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        1,
        ([$($matcher:tt)*], $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.1, $crate::__matcher_expr!([$($matcher)*]))
            ),
            [$($struct_name)*],
            2,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        2,
        ({$($matcher:tt)*}, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.2, $crate::__matcher_expr!({$($matcher)*}))
            ),
            [$($struct_name)*],
            3,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        2,
        ([$($matcher:tt)*], $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.2, $crate::__matcher_expr!([$($matcher)*]))
            ),
            [$($struct_name)*],
            3,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        3,
        ({$($matcher:tt)*}, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.3, $crate::__matcher_expr!({$($matcher)*}))
            ),
            [$($struct_name)*],
            4,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        3,
        ([$($matcher:tt)*], $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.3, $crate::__matcher_expr!([$($matcher)*]))
            ),
            [$($struct_name)*],
            4,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        4,
        ({$($matcher:tt)*}, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.4, $crate::__matcher_expr!({$($matcher)*}))
            ),
            [$($struct_name)*],
            5,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        4,
        ([$($matcher:tt)*], $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.4, $crate::__matcher_expr!([$($matcher)*]))
            ),
            [$($struct_name)*],
            5,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        5,
        ({$($matcher:tt)*}, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.5, $crate::__matcher_expr!({$($matcher)*}))
            ),
            [$($struct_name)*],
            6,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        5,
        ([$($matcher:tt)*], $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.5, $crate::__matcher_expr!([$($matcher)*]))
            ),
            [$($struct_name)*],
            6,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        6,
        ({$($matcher:tt)*}, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.6, $crate::__matcher_expr!({$($matcher)*}))
            ),
            [$($struct_name)*],
            7,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        6,
        ([$($matcher:tt)*], $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.6, $crate::__matcher_expr!([$($matcher)*]))
            ),
            [$($struct_name)*],
            7,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        7,
        ({$($matcher:tt)*}, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.7, $crate::__matcher_expr!({$($matcher)*}))
            ),
            [$($struct_name)*],
            8,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        7,
        ([$($matcher:tt)*], $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.7, $crate::__matcher_expr!([$($matcher)*]))
            ),
            [$($struct_name)*],
            8,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        8,
        ({$($matcher:tt)*}, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.8, $crate::__matcher_expr!({$($matcher)*}))
            ),
            [$($struct_name)*],
            9,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        8,
        ([$($matcher:tt)*], $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.8, $crate::__matcher_expr!([$($matcher)*]))
            ),
            [$($struct_name)*],
            9,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        $field:tt,
        ($matcher:expr $(,)?)
    ) => {
        $crate::matchers::__internal::is(
            stringify!($($struct_name)*),
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.$field, $matcher)
            ),
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        1,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.1, $matcher)
            ),
            [$($struct_name)*],
            2,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        2,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.2, $matcher)
            ),
            [$($struct_name)*],
            3,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        3,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.3, $matcher)
            ),
            [$($struct_name)*],
            4,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        4,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.4, $matcher)
            ),
            [$($struct_name)*],
            5,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        5,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.5, $matcher)
            ),
            [$($struct_name)*],
            6,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        6,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.6, $matcher)
            ),
            [$($struct_name)*],
            7,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        7,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.7, $matcher)
            ),
            [$($struct_name)*],
            8,
            ($($rest)*)
        )
    };

    (
        $crate::matchers::all!($($processed:tt)*),
        [$($struct_name:tt)*],
        8,
        ($matcher:expr, $($rest:tt)*)
    ) => {
        $crate::matches_pattern_internal!(
            $crate::matchers::all!(
                $($processed)*,
                $crate::matchers::field!($($struct_name)*.8, $matcher)
            ),
            [$($struct_name)*],
            9,
            ($($rest)*)
        )
    };

    ([$($struct_name:tt)*], $first:tt $($rest:tt)*) => {
        $crate::matches_pattern_internal!([$($struct_name)* $first], $($rest)*)
    };

    ($first:tt $($rest:tt)*) => {{
        $crate::matches_pattern_internal!([$first], $($rest)*)
    }};
}

/// An alias for [`matches_pattern`][crate::matchers::matches_pattern!].
#[macro_export]
#[doc(hidden)]
macro_rules! __pat {
    ($($t:tt)*) => { $crate::matches_pattern_internal!($($t)*) }
}
