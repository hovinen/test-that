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

// Tests that container matchers infer types without diverging, even when a
// recursive blanket implementation is in scope.
//
// Several matchers are generic over containers: they can match against any
// type `T` such that `&T` implements `IntoIterator`. This file introduces a
// generic struct a reference to which implements `IntoIterator` for _any_
// type `T` as soon as `&T` implements `IntoIterator`.
//
// Suppose we have a matcher implementing the trait `Matcher<T>` for any
// `T` where `&T` implements `IntoIterator` (as is the case with the
// container matchers `contains`, `empty`, `len`, etc.). The trait solver
// will attempt to find a type which it can use for `T`. One example is
// `DivergenceTriggeringStruct` defined below, which the trait solver then
// tries. However, it comes with its own generic parameter `T`, which the
// trait solver now needs to identify. That has the same requirement: `&T`
// must implement `IntoIterator`. So the trait solver finds
// `DivergenceTriggeringStruct` again and tries to use it. This continues
// ad infinitum, or rather until the compiler's recursion limit is reached.
// Compilation fails with E0275. Raising the recursion limit does not help
// since the recursion is infinite.
//
// This bites in three cases:
//
// - The field matcher `test_that::matchers::field_matcher` (used when matching
//   fields in `matches_pattern!`) when the inner matcher is a container
//   matcher,
// - Analogously the `result_of!` matcher (also used when matching properties
//   with `matches_pattern!`), and
// - The `all!` and `any!` matchers, also with container matchers.
//
// In each case, this problem occurs because the trait solver does not yet
// know the concrete type `T` by the time it encounters that type. This does
// not happen, for example, in an ordinary assertion using one of the container
// matchers because the actual type is already clear:
//
// ```
// verify_that!(my_vec, contains(eq(123)))  // The type `T` is that of `my_vec`
// ```
//
// As soon as the matcher appears inside a closure, the trait solver attempts
// to resolve the actual type against which `contains` matches without first
// resolving it by examining the output of that closure:
//
// ```
// verify_that!(my_struct, result_of!(|s: &MyStruct| s.get_vec(), contains(eq(123))))
// ```
//
// This also fails when the matcher uses type erasure -- i.e.
// `Box<dyn Matcher<T>>` -- as was previously the case with `all!` and `any!`:
//
// ```
// verify_that!(my_vec, all![contains(eq(123)), contains(eq(234))])
// ```
//
// A real world example of this is the `Retained` struct in version 0.6.4 of
// the crate [objc2](https://crates.io/crates/objc2). Merely having the type
// in scope when using the container matchers is enough to trigger divergence,
// causing compilation to fail.
//
// Every test here is a compilation regression test. By declaring
// `DivergenceTriggeringStruct` in scope and then exercising the various
// container matchers, this should trigger compiler errors in the absence of
// countermeasures.
//
// This is solved by ensuring the type whose reference implements `IntoIterator`
// is concretely known _before_ the trait solver attempts to resolve the inner
// matcher's trait. This is the reason for the indirection via the functions
// `test_that::matchers::__internal::apply_matcher` and `...::explain_matcher`
// as well as the construction
// `test_that::matchers::field_matcher::__internal::FieldMatcherStage` with its
// `with` method. This is solved for the matchers `any!` and `all!` by removing
// the type erasure and instead having them build a tuple of matchers.

use std::collections::HashSet;
use test_that::prelude::*;

// This type is intended to fool the trait solver into diverging.
#[allow(dead_code)]
#[derive(Debug)]
struct DivergenceTriggeringStruct<T: ?Sized>(core::marker::PhantomData<T>);

impl<T: ?Sized> core::ops::Deref for DivergenceTriggeringStruct<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unimplemented!("never constructed; present only for its trait impls")
    }
}

impl<'a, T: ?Sized> IntoIterator for &'a DivergenceTriggeringStruct<T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!("never constructed; present only for its trait impls")
    }
}

#[derive(Debug)]
struct AStruct {
    a_vec: Vec<u32>,
    a_boxed_slice: Box<[u32]>,
    an_array: [u32; 3],
    a_name: String,
    a_set: HashSet<u32>,
}

impl AStruct {
    fn get_a_slice(&self) -> &[u32] {
        &self.a_vec
    }

    fn get_a_slice_from(&self, from: usize) -> &[u32] {
        &self.a_vec[from..]
    }

    fn get_a_vec_of<T: From<u32>>(&self) -> Vec<T> {
        self.a_vec.iter().map(|v| T::from(*v)).collect()
    }

    fn get_a_boxed_slice(&self) -> Box<[u32]> {
        self.a_vec.clone().into_boxed_slice()
    }

    fn get_a_vec(&self) -> Vec<u32> {
        self.a_vec.clone()
    }

    fn get_a_str(&self) -> &str {
        &self.a_name
    }
}

fn a_struct() -> AStruct {
    AStruct {
        a_vec: vec![1, 2, 3],
        a_boxed_slice: vec![1, 2, 3].into_boxed_slice(),
        an_array: [1, 2, 3],
        a_name: "hello".into(),
        a_set: HashSet::from([1, 2, 3]),
    }
}

#[test]
fn matches_field_with_container_matcher() -> TestResult<()> {
    verify_that!(a_struct(), matches_pattern!(AStruct { a_vec: container_eq([1, 2, 3]) }))
}

#[test]
fn matches_array_field_with_container_matcher() -> TestResult<()> {
    verify_that!(a_struct(), matches_pattern!(AStruct { an_array: container_eq([1, 2, 3]) }))
}

#[test]
fn matches_deref_field_with_container_matcher() -> TestResult<()> {
    verify_that!(a_struct(), matches_pattern!(AStruct { *a_boxed_slice: container_eq([1, 2, 3]) }))
}

#[test]
fn matches_deref_property_with_container_matcher() -> TestResult<()> {
    verify_that!(a_struct(), matches_pattern!(AStruct { *get_a_slice(): container_eq([1, 2, 3]) }))
}

#[test]
fn matches_deref_property_with_empty() -> TestResult<()> {
    let value = AStruct { a_vec: vec![], ..a_struct() };

    verify_that!(value, matches_pattern!(AStruct { *get_a_slice(): empty() }))
}

#[test]
fn matches_deref_property_with_argument() -> TestResult<()> {
    verify_that!(
        a_struct(),
        matches_pattern!(AStruct { *get_a_slice_from(1): container_eq([2, 3]) })
    )
}

#[test]
fn matches_deref_property_with_turbofish() -> TestResult<()> {
    verify_that!(
        a_struct(),
        matches_pattern!(AStruct { *get_a_vec_of::<u64>(): container_eq([1u64, 2, 3]) })
    )
}

/// The property returns an owned value which is only then dereferenced, rather
/// than returning a reference directly.
#[test]
fn matches_deref_property_returning_owned_value() -> TestResult<()> {
    verify_that!(
        a_struct(),
        matches_pattern!(AStruct { *get_a_boxed_slice(): container_eq([1, 2, 3]) })
    )
}

#[test]
fn matches_deref_property_with_ordered_shorthand() -> TestResult<()> {
    verify_that!(a_struct(), matches_pattern!(AStruct { *get_a_slice(): [eq(1), gt(1), le(3)] }))
}

#[test]
fn matches_deref_property_with_unordered_shorthand() -> TestResult<()> {
    verify_that!(a_struct(), matches_pattern!(AStruct { *get_a_slice(): {eq(3), eq(1), eq(2)} }))
}

#[test]
fn matches_property_returning_owned_container() -> TestResult<()> {
    verify_that!(a_struct(), matches_pattern!(AStruct { get_a_vec(): container_eq([1, 2, 3]) }))
}

#[test]
fn matches_property_returning_owned_container_with_turbofish() -> TestResult<()> {
    verify_that!(
        a_struct(),
        matches_pattern!(AStruct { get_a_vec_of::<u64>(): container_eq([1u64, 2, 3]) })
    )
}

#[test]
fn matches_property_returning_reference_with_points_to() -> TestResult<()> {
    verify_that!(
        a_struct(),
        matches_pattern!(AStruct { get_a_slice(): points_to(container_eq([1, 2, 3])) })
    )
}

#[test]
fn matches_property_returning_str_with_non_container_matcher() -> TestResult<()> {
    verify_that!(a_struct(), matches_pattern!(AStruct { get_a_str(): eq("hello") }))
}

#[test]
fn matches_property_returning_owned_container_with_empty() -> TestResult<()> {
    let value = AStruct { a_vec: vec![], ..a_struct() };

    verify_that!(value, matches_pattern!(AStruct { get_a_vec(): empty() }))
}

#[test]
fn matches_hash_set_field_with_container_matcher() -> TestResult<()> {
    verify_that!(a_struct(), matches_pattern!(AStruct { a_set: contains(eq(2)) }))
}

#[test]
fn matches_member_with_all_of_container_matchers() -> TestResult<()> {
    verify_that!(
        a_struct(),
        matches_pattern!(AStruct { a_vec: all!(container_eq([1, 2, 3]), len(eq(3))) })
    )
}

#[test]
fn matches_member_with_any_of_container_matchers() -> TestResult<()> {
    verify_that!(
        a_struct(),
        matches_pattern!(AStruct { a_vec: any!(container_eq([1, 2, 3]), empty()) })
    )
}

#[test]
fn matches_several_members_together() -> TestResult<()> {
    verify_that!(
        a_struct(),
        matches_pattern!(AStruct {
            a_vec: container_eq([1, 2, 3]),
            *a_boxed_slice: len(eq(3)),
            *get_a_slice(): contains(eq(2)),
        })
    )
}

#[test]
fn does_not_match_deref_property_with_wrong_content() -> TestResult<()> {
    let result = verify_that!(
        a_struct(),
        matches_pattern!(AStruct { *get_a_slice(): container_eq([4, 5]) })
    );

    verify_that!(result, err(anything()))
}

#[test]
fn does_not_match_field_with_wrong_content() -> TestResult<()> {
    let result = verify_that!(a_struct(), matches_pattern!(AStruct { a_vec: empty() }));

    verify_that!(result, err(anything()))
}

#[test]
fn describes_deref_property_in_failure_message() -> TestResult<()> {
    let result = verify_that!(
        a_struct(),
        matches_pattern!(AStruct { *get_a_slice(): container_eq([4, 5]) })
    );

    verify_that!(result, err(displays_as(contains_substring("get_a_slice"))))
}

#[test]
fn explains_deref_property_match_in_failure_message() -> TestResult<()> {
    let result = verify_that!(
        a_struct(),
        matches_pattern!(AStruct { *get_a_slice(): container_eq([4, 5]) })
    );

    verify_that!(
        result,
        err(displays_as(contains_substring("which after applying `get_a_slice ()` results in")))
    )
}

mod dyn_trait {
    use super::*;

    trait ATrait: core::fmt::Debug {
        fn items(&self) -> &[u32];
    }

    #[derive(Debug)]
    struct AnImplementation(Vec<u32>);

    impl ATrait for AnImplementation {
        fn items(&self) -> &[u32] {
            &self.0
        }
    }

    #[test]
    fn matches_deref_property_on_trait_object() -> TestResult<()> {
        let value = AnImplementation(vec![1, 2, 3]);
        let reference: &dyn ATrait = &value;

        verify_that!(
            reference,
            points_to(matches_pattern!(dyn ATrait { *items(): container_eq([1, 2, 3]) }))
        )
    }

    #[test]
    fn does_not_match_deref_property_on_trait_object_with_wrong_content() -> TestResult<()> {
        let value = AnImplementation(vec![1, 2, 3]);
        let reference: &dyn ATrait = &value;

        let result = verify_that!(
            reference,
            points_to(matches_pattern!(dyn ATrait { *items(): container_eq([4, 5]) }))
        );

        verify_that!(result, err(anything()))
    }
}
