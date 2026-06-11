# Test That!

[![Apache licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/test_that.svg
[crates-url]: https://crates.io/crates/test_that
[docs-badge]: https://img.shields.io/badge/docs.rs-test_that-66c2a5
[docs-url]: https://docs.rs/test_that/*/test_that/
[license-badge]: https://img.shields.io/badge/license-Apache-blue.svg
[license-url]: https://github.com/hovinen/test_that/blob/main/LICENSE
[actions-badge]: https://github.com/hovinen/test-that/workflows/CI/badge.svg
[actions-url]: https://github.com/hovinen/test-that/actions?query=workflow%3ACI+branch%3Amain

Test That! is a powerful test assertion library for Rust.

> [!NOTE]
> This is not yet released on crates.io. There is still some work to be done on
> it.

## Background

The existing assertions which come with Rust are fairly primitive:

* `assert!`
* `assert_eq!`
* `assert_matches!` (as of Rust 1.96)

These work fine for simple cases. But suppose you want to express something
like "every element of this `Vec` is positive". You could assert on the exact
values:

```rust
assert_eq!(vec, vec![1, 2, 3]);
```

But then the test may be brittle. The specific values might change, causing the
test to fail. But the _intent_ of the test is just that the values are positive,
so nothing is really wrong.

You could also express the intent with a computation:

```rust
assert!(vec.into_iter().all(|i| i > 0));
```

What happens, then, when the test fails?

```
assertion failed: vec.into_iter().all(|i| i > 0) 
```

There's no _context_ in the assertion failure. You look at the test logs of
your CI system and only see this one message. No indication what the contents
of `vec` were or which element caused the failure. To get that information, you
already have to rerun the test and investigate. Possibly adding more debugging
information. This costs time and nerves.

Test That! lets you write test assertions which precisely capture your _intent_
while then producing test failure messages which tell you _exactly_ what was
wrong.

```rust
assert_that!(vec, each(gt(0)));
```

```
Value of: vec
Expected: only contains elements that is greater than 0
Actual: [-1],
  whose element #0 is -1, which is less than or equal to 0
```

Writing test assertions is a finger exercise. You can churn out tests without
having to think much about how they show up when they fail.

## History of this crate

I started Test That! as a fork of the crate
[GoogleTest Rust](https://crates.io/crates/googletest), which I had spearheaded
while working at Google. The goal of the fork is to provide a more ergonomic
developer experience than GoogleTest Rust does.

The GoogleTest Rust was in turned inspired by Google's C++ testing library
[GoogleTest](https://github.com/google/googletest). Its goal was to bring the
powerful assertions and matchers of GoogleTest to the Rust world.

## A brief tour of Test That!

The core of Test That! is its *matchers*. Matchers indicate what aspect of an
actual value one is asserting: (in-)equality, containment, regular expression
matching, and so on.

To make an assertion using a matcher, Test That! offers three macros:

 * [`assert_that!`] panics if the assertion fails, aborting the test.
 * [`expect_that!`] logs an assertion failure, marking the test as having
   failed, but allows the test to continue running (called a _non-fatal
   assertion_). It requires the use of the [`test_that::test`] attribute macro
   on the test itself.
 * [`verify_that!`] has no side effects and evaluates to a [`Result<()>`] whose
   `Err` variant describes the assertion failure, if there is one. In
   combination with the
   [`?` operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator),
   this can be used to abort the test on assertion failure without panicking.
   (It is also the building block for the other two macros above.)

For example:

```rust
use test_that::prelude::*;

#[test]
fn single_panicking_failure() {
    let value = 2;
    assert_that!(value, eq(4)); // Panics, failing the test.
}

#[test_that::test]
fn two_logged_failures() {
    let value = 2;
    expect_that!(value, eq(4)); // Test now failed, but continues executing.
    expect_that!(value, eq(5)); // Second failure is also logged.
}

#[test]
fn immediate_failure_without_panic() -> Result<()> {
    let value = 2;
    verify_that!(value, eq(4))?; // Test fails and aborts.
    verify_that!(value, eq(2))?; // Never executes.
    Ok(())
}

#[test]
fn simple_assertion() -> Result<()> {
    let value = 2;
    verify_that!(value, eq(4)) // One can also just return the last assertion.
}
```

This library includes a rich set of matchers, covering:

 * Equality, numeric inequality, and approximate equality;
 * Strings and regular expressions;
 * Containers and set-theoretic matching.

Matchers are composable:

```rust
use test_that::prelude::*;

#[test_that::test]
fn contains_at_least_one_item_at_least_3() {
    let value = vec![1, 2, 3];
    expect_that!(value, contains(ge(3)));
}
```

They can also be logically combined:

```rust
use test_that::prelude::*;

#[test_that::test]
fn strictly_between_9_and_11() {
    let value = 10;
    expect_that!(value, gt(9).and(not(ge(11))));
}
```

### Pattern-matching

One can use the macro [`matches_pattern!`] to create a composite matcher for a
struct or enum that matches fields with other matchers:

```rust
use test_that::prelude::*;

struct AStruct {
    a_field: i32,
    another_field: i32,
    a_third_field: &'static str,
}

#[test]
fn struct_has_expected_values() {
    let value = AStruct {
        a_field: 10,
        another_field: 100,
        a_third_field: "A correct value",
    };
    expect_that!(value, matches_pattern!(AStruct {
        a_field: eq(10),
        another_field: gt(50),
        a_third_field: contains_substring("correct"),
    }));
}
```

### Writing matchers

One can write one's own matchers. To do so, create a struct holding the
matcher's data and have it implement the traits [`Matcher`] and
['Describable']:

```rust
pub struct MyEqMatcher<T> {
    expected: T,
}

impl<T: PartialEq + Debug> Matcher<T> for MyEqMatcher<T> {
    fn matches(&self, actual: &T) -> MatcherResult {
         (self.expected == *actual).into()
    }
}

impl<T: Debug> Describable for MyEqMatcher<T> {
    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Match => {
                format!("is equal to {:?} the way I define it", self.expected)
            }
            MatcherResult::NoMatch => {
                format!("isn't equal to {:?} the way I define it", self.expected)
            }
        }
    }
}
```

One should also expose a function which constructs the matcher:

```rust
pub fn eq_my_way<T>(expected: T) -> MyEqMatcher<T> {
    MyEqMatcher { expected }
}
```

The new matcher can then be used in the assertion macros:

```rust
#[test_that::test]
fn should_be_equal_by_my_definition() {
    expect_that!(10, eq_my_way(10));
}
```

### Non-fatal assertions

Using non-fatal assertions, a single test is able to log multiple assertion
failures. Any single assertion failure causes the test to be considered having
failed, but execution continues until the test completes or otherwise aborts.

This is analogous to the `EXPECT_*` family of macros in GoogleTest.

To make a non-fatal assertion, use the macro [`expect_that!`]. The test must
also be marked with [`test_that::test`] instead of the Rust-standard `#[test]`.

```rust
use test_that::prelude::*;

#[test_that::test]
fn three_non_fatal_assertions() {
    let value = 2;
    expect_that!(value, eq(2));  // Passes; test still considered passing.
    expect_that!(value, eq(3));  // Fails; logs failure and marks the test failed.
    expect_that!(value, eq(4));  // A second failure, also logged.
}
```

This can be used in the same tests as `verify_that!`, in which case the test
function must also return [`Result<()>`]:

```rust
use test_that::prelude::*;

#[test_that::test]
fn failing_non_fatal_assertion() -> Result<()> {
    let value = 2;
    expect_that!(value, eq(3));  // Just marks the test as having failed.
    verify_that!(value, eq(2))?;  // Passes, so does not abort the test.
    Ok(())        // Because of the failing expect_that! call above, the
                  // test fails despite returning Ok(())
}
```

```rust
use test_that::prelude::*;

#[test_that::test]
fn failing_fatal_assertion_after_non_fatal_assertion() -> Result<()> {
    let value = 2;
    verify_that!(value, eq(3))?; // Fails and aborts the test.
    expect_that!(value, eq(3));  // Never executes, since the test already aborted.
    Ok(())
}
```

### Interoperability

You can use the `#[test_that::test]` macro together with many other libraries
such as [rstest](https://crates.io/crates/rstest) and
[tokio](https://crates.io/crates/tokio). Just apply both attribute macros to the
test:

```rust
#[test_that::test]
#[rstest]
#[case(1)]
#[case(2)]
#[case(3)]
fn rstest_works_with_test_that(#[case] value: u32) -> Result<()> {
    verify_that!(value, gt(0))
}
```

```rust
#[test_that::test]
#[tokio::test]
async fn tokio_works_with_test_that() -> Result<()> {
    verify_that!(get_some_value_async().await, gt(0))
}
```

> [!NOTE]
> In the case of rstest, make sure to put `#[test_that::test]` *before*
> `#[rstest]`. Otherwise the annotated test will run twice, since both macros will
> attempt to register a test with the Rust test harness.

### Predicate assertions

The macro [`verify_pred!`] provides predicate assertions analogous to
GoogleTest's `EXPECT_PRED` family of macros. Wrap an invocation of a predicate
in a `verify_pred!` invocation to turn that into a test assertion which passes
precisely when the predicate returns `true`:

```rust
fn stuff_is_correct(x: i32, y: i32) -> bool {
    x == y
}

let x = 3;
let y = 4;
verify_pred!(stuff_is_correct(x, y))?;
```

The assertion failure message shows the arguments and the values to which they
evaluate:

```
stuff_is_correct(x, y) was false with
  x = 3,
  y = 4
```

The `verify_pred!` invocation evaluates to a [`Result<()>`] just like
[`verify_that!`]. There is also a macro [`expect_pred!`] to make a non-fatal
predicaticate assertion.

### Unconditionally generating a test failure

The macro [`fail!`] unconditionally evaluates to a `Result` indicating a test
failure. It can be used analogously to [`verify_that!`] and [`verify_pred!`] to
cause a test to fail, with an optional formatted message:

```rust
#[test]
fn always_fails() -> Result<()> {
    fail!("This test must fail with {}", "today")
}
```

### Configuration

This library is configurable through environment variables. Since the
configuration does not impact whether a test fails or not but how a failure is
displayed, we recommend setting those variables in the personal
`~/.cargo/config.toml` instead of in the project-scoped `Cargo.toml`.

The following environment variables are supported:

| Variable name | Description                                             |
| ------------- | ------------------------------------------------------- |
| NO_COLOR      | Disables colored output. See <https://no-color.org/>.   |
| FORCE_COLOR   | Forces colors even when the output is piped to a file.  |

## Porting from GoogleTest Rust

GoogleTest Rust made major changes between versions 0.11 and 0.12, so there are
separate instructions.

### GoogleTest Rust 0.11

Key differences are:

* `Matcher` now takes its actual value as a type parameter and not an associated
  type.
* The `describe` method is now in a separate trait `Describable`, which must be
  implemented to implement `Matcher`.
* The alias `Result` has been renamed to `TestResult`.
* The macro `elements_are!` has been renamed `contains_exactly!`.
* The macro `unordered_elements_are!` has been removed. Instead, a new method
  `in_order()` is available on the matcher produced by the macros
  `contains_exactly!`, `contains_each!`, and `is_contained_in!` to provide the
  same functionality.
* The syntax for matching `HashMap` with `contains_exactly!` has changed from
  pairs to `key => value` expressions. So what was previously:

  ```rust
  let map = HashMap::from([(1, "a")]);
  assert_that!(map, contains_each![(eq(1), eq("a"))]);
  ```

  becomes:

  ```rust
  let map = HashMap::from([(1, "a")]);
  assert_that!(map, contains_each![(eq(1) => eq("a"))]);
  ```

* The macro `property!` has been removed in favour of a new macro `result_of!`.

To facilitate porting existing code, Test That! has two features:

* `googletest-compat`: Reintroduces symbol names used in GoogleTest Rust as much
  as feasible. This minimizes the actual code changes you must make. In
  particular, it adds:
  * `test_that::Result` as an alias for `test_that::TestResult`,
  * `elements_are!` as an alias for `contains_exactly![...].in_order()`,
  * `unordered_elements_are!` as an alias for `contains_exactly!`,
  * `into_test_result()` as an alias for `or_fail()`, and
  * the pair-based syntax for matching against `HashMap` in
    `unordered_elements_are!` (not, however, for `contains_each!` or
    `is_contained_in!`).
* `googletest-migrate`: Marks, as much as possible, each of the aliases
  introduced by `googletest-compat` as deprecated, so you can easily find and
  update them. This feature implies `googletest-compat`.

I believe that this covers most cases. You will have to manually port any other
changes.

### GoogleTest Rust 0.12 and later

The key differences include:

* In GoogleTest, `Matcher::matches` takes an owned value which must be `Copy`.
  In Test That!, it takes a reference, just as in GoogleTest prior to 0.11. This
  means that nearly all cases where one must add `&` or `ref` in GoogleTest are
  not required (or allowed) in Test That!. For example, in GoogleTest:

  ```rust
  #[derive(Debug)]
  enum MyEnum {
      A(u32),
  }
  assert_that!(MyEnum::A(123), matches_pattern!(&MyEnum::A(eq(123))));
  ```

  is in Test That!:

  ```rust
  assert_that!(MyEnum::A(123), matches_pattern!(MyEnum::A(eq(123))));
  ```

* The `matches_pattern!` matcher in Test That! does not support exhaustive
  matching. The `..` syntax used in GoogleTest is neither necessary nor allowed.

* Test That! has no `verify_eq!` or similar. Use `verify_that!(.., eq())`.

* Test That! also has no matcher `ne`. Use `not(eq(..))`.

* There is no support for fixtures. Such functionality is available in the crate
  [rstest](https://crates.io/crates/rstest).

* As with GoogleTest 0.11, the equivalent alias to `Result` is `TestResult`.

* As with GoogleTest 0.11, there is no macro `property!`.

* The same notes as above apply to `elements_are!` and
  `unordered_elements_are!`.

## Contributing Changes

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to contribute
to this project.

[`and_log_failure()`]: https://docs.rs/test_that/*/test_that/trait.TestResultExt.html#tymethod.and_log_failure
[`assert_that!`]: https://docs.rs/test_that/*/test_that/macro.assert_that.html
[`expect_pred!`]: https://docs.rs/test_that/*/test_that/macro.expect_pred.html
[`expect_that!`]: https://docs.rs/test_that/*/test_that/macro.expect_that.html
[`fail!`]: https://docs.rs/test_that/*/test_that/macro.fail.html
[`test_that::test`]: https://docs.rs/test_that/*/test_that/attr.test.html
[`matches_pattern!`]: https://docs.rs/test_that/*/test_that/macro.matches_pattern.html
[`verify_pred!`]: https://docs.rs/test_that/*/test_that/macro.verify_pred.html
[`verify_that!`]: https://docs.rs/test_that/*/test_that/macro.verify_that.html
[`Describable`]: https://docs.rs/test_that/*/test_that/matcher/trait.Describable.html
[`Describe`]: https://docs.rs/test_that/*/test_that/matcher/trait.Describe.html
[`Matcher`]: https://docs.rs/test_that/*/test_that/matcher/trait.Matcher.html
[`Result<()>`]: https://docs.rs/test_that/*/test_that/type.Result.html
