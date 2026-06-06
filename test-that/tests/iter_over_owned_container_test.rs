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
struct IntContainer(Vec<i32>);

impl<'a> IntoIterator for &'a IntContainer {
    type Item = i32;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, i32>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

#[test]
fn contains_exactly_supports_containers_which_iterate_over_owned_values() -> TestResult<()> {
    let container = IntContainer(vec![1, 2, 3]);

    verify_that!(container, contains_exactly![eq(1), eq(2), eq(3)])
}

#[test]
fn contains_exactly_in_order_supports_containers_which_iterate_over_owned_values() -> TestResult<()> {
    let container = IntContainer(vec![1, 2, 3]);

    verify_that!(container, contains_exactly![eq(1), eq(2), eq(3)].in_order())
}

#[derive(Debug)]
struct IntMap(Vec<(i32, i32)>);

impl<'a> IntoIterator for &'a IntMap {
    type Item = (i32, i32);
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, (i32, i32)>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

#[test]
fn contains_exactly_with_map_supports_containers_which_iterate_over_owned_values() -> TestResult<()> {
    let container = IntMap(vec![(1, 1), (2, 2), (3, 3)]);

    verify_that!(container, contains_exactly![(eq(1) => eq(1)), (eq(2) => eq(2)), (eq(3) => eq(3))])
}
