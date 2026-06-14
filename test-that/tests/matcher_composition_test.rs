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

#[test]
fn vec_of_options_each_some_and_positive() -> TestResult<()> {
    let value: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3)];
    verify_that!(value, each(some(gt(0))))
}

#[test]
fn vec_of_options_of_enum_contains_specific_variant() -> TestResult<()> {
    #[derive(Debug)]
    enum MyEnum {
        Foo(i32),
        Bar,
    }
    let value: Vec<Option<MyEnum>> = vec![Some(MyEnum::Foo(42)), Some(MyEnum::Bar)];
    verify_that!(value, contains(some(matches_pattern!(MyEnum::Foo(eq(42))))))
}

#[test]
fn vec_of_options_of_enum_ref_contains_specific_variant() -> TestResult<()> {
    #[derive(Debug)]
    enum MyEnum {
        Foo(i32),
        Bar,
    }
    let a = MyEnum::Foo(42);
    let b = MyEnum::Bar;
    let value: Vec<Option<&MyEnum>> = vec![Some(&a), Some(&b)];
    verify_that!(value, contains(some(points_to(matches_pattern!(MyEnum::Foo(eq(42)))))))
}

#[test]
fn struct_method_returning_vec_ref_contains_single_ok_string() -> TestResult<()> {
    #[derive(Debug)]
    struct MyStruct {
        items: Vec<std::result::Result<String, i32>>,
    }
    impl MyStruct {
        fn get_items(&self) -> &Vec<std::result::Result<String, i32>> {
            &self.items
        }
    }
    let value = MyStruct { items: vec![Ok("hello".to_string())] };
    verify_that!(
        value,
        result_of!(
            |s: &MyStruct| s.get_items(),
            points_to(contains_exactly![ok(eq("hello"))].in_order())
        )
    )
}

#[test]
fn nested_vec_each_inner_vec_contains_only_positive() -> TestResult<()> {
    let value: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4, 5]];
    verify_that!(value, each(each(gt(0))))
}

#[test]
fn option_of_vec_of_strings_some_with_exact_elements() -> TestResult<()> {
    let value: Option<Vec<String>> = Some(vec!["alpha".to_string(), "beta".to_string()]);
    verify_that!(value, some(contains_exactly![eq("alpha"), eq("beta")].in_order()))
}

#[test]
fn result_ok_of_vec_of_options_each_some_positive() -> TestResult<()> {
    let value: std::result::Result<Vec<Option<i32>>, String> = Ok(vec![Some(10), Some(20)]);
    verify_that!(value, ok(each(some(gt(0)))))
}

#[test]
fn matches_pattern_on_struct_with_vec_field() -> TestResult<()> {
    #[derive(Debug)]
    struct Config {
        name: String,
        tags: Vec<String>,
    }
    let value = Config { name: "foo".to_string(), tags: vec!["a".to_string(), "b".to_string()] };
    verify_that!(
        value,
        matches_pattern!(Config { name: eq("foo"), tags: contains_each![eq("a"), eq("b")] })
    )
}

#[test]
fn all_matcher_on_option_with_some_and_range() -> TestResult<()> {
    let value: Option<i32> = Some(5);
    verify_that!(value, all!(not(none()), some(all!(gt(0), lt(10)))))
}

#[test]
fn some_of_vec_matched_with_pointwise() -> TestResult<()> {
    let value: Option<Vec<i32>> = Some(vec![1, 4, 9]);
    verify_that!(value, some(pointwise!(eq, [1, 4, 9])))
}

#[test]
fn matches_pattern_on_doubly_nested_struct_with_option_and_vec() -> TestResult<()> {
    #[derive(Debug)]
    struct Inner {
        values: Vec<i32>,
    }
    #[derive(Debug)]
    struct Outer {
        inner: Option<Inner>,
    }
    let value = Outer { inner: Some(Inner { values: vec![10, 20] }) };
    verify_that!(
        value,
        matches_pattern!(Outer {
            inner: some(matches_pattern!(Inner { values: contains_each![eq(10), eq(20)] })),
        })
    )
}

#[cfg(feature = "std")]
#[test]
fn has_entry_with_vec_of_options_value() -> TestResult<()> {
    use std::collections::HashMap;
    let mut map: HashMap<&str, Vec<Option<i32>>> = HashMap::new();
    map.insert("key", vec![Some(1), None, Some(3)]);
    verify_that!(map, has_entry("key", contains(some(gt(0)))))
}

#[test]
fn contains_each_with_ok_starts_with_matchers() -> TestResult<()> {
    let value: Vec<core::result::Result<String, i32>> =
        vec![Ok("hello world".to_string()), Ok("goodbye".to_string())];
    verify_that!(value, contains_each![ok(starts_with("hello")), ok(starts_with("good"))])
}

#[test]
fn result_of_slice_of_options_each_some_positive() -> TestResult<()> {
    #[derive(Debug)]
    struct Holder {
        data: Vec<Option<i32>>,
    }
    impl Holder {
        fn as_slice(&self) -> &[Option<i32>] {
            &self.data
        }
    }
    let value = Holder { data: vec![Some(5), Some(10)] };
    verify_that!(value, result_of!(|h: &Holder| h.as_slice(), points_to(each(some(gt(0))))))
}

#[test]
fn vec_of_options_each_none_or_negative() -> TestResult<()> {
    let value: Vec<Option<i32>> = vec![None, Some(-1), None, Some(-5)];
    verify_that!(value, each(any!(none(), some(lt(0)))))
}

#[test]
fn contains_each_with_tuple_and_option_string() -> TestResult<()> {
    let value: Vec<(i32, Option<String>)> =
        vec![(1, Some("apple".to_string())), (2, Some("banana".to_string()))];
    verify_that!(
        value,
        contains_each![(eq(1), some(starts_with("app"))), (eq(2), some(starts_with("ban"))),]
    )
}

#[test]
fn ok_with_displays_as_matcher() -> TestResult<()> {
    let value: std::result::Result<i32, String> = Ok(42);
    verify_that!(value, ok(displays_as(eq("42"))))
}

#[test]
fn some_of_vec_with_len_and_each_constraints() -> TestResult<()> {
    let value: Option<Vec<i32>> = Some(vec![7, 8, 9]);
    verify_that!(value, some(all!(len(eq(3)), each(gt(0)))))
}

#[test]
fn ok_of_vec_is_subset_of_set() -> TestResult<()> {
    let value: std::result::Result<Vec<i32>, String> = Ok(vec![1, 3]);
    verify_that!(value, ok(subset_of([1, 2, 3, 4, 5])))
}

#[test]
fn matches_pattern_on_enum_with_result_option_payload() -> TestResult<()> {
    #[derive(Debug)]
    enum Event {
        Data(std::result::Result<Option<String>, i32>),
    }
    let value = Event::Data(Ok(Some("payload".to_string())));
    verify_that!(value, matches_pattern!(Event::Data(ok(some(eq("payload"))))))
}

#[test]
fn result_of_returning_option_str_ref_with_some_contains_substring() -> TestResult<()> {
    #[derive(Debug)]
    struct Record {
        description: Option<String>,
    }
    impl Record {
        fn description_str(&self) -> Option<&str> {
            self.description.as_deref()
        }
    }
    let value = Record { description: Some("hello world".to_string()) };
    verify_that!(
        value,
        result_of!(|r: &Record| r.description_str(), some(contains_substring("world")))
    )
}

#[test]
fn each_ok_not_contains_substring() -> TestResult<()> {
    let value: Vec<core::result::Result<String, i32>> =
        vec![Ok("hello".to_string()), Ok("world".to_string())];
    verify_that!(value, each(ok(not(contains_substring("bad")))))
}

#[test]
fn points_to_with_all_range_matchers() -> TestResult<()> {
    let n = 42i32;
    verify_that!(&n, points_to(all!(gt(0), lt(100))))
}

#[test]
fn all_contains_and_not_contains_on_vec() -> TestResult<()> {
    let value = vec![1, 2, 3];
    verify_that!(value, all!(contains(eq(2)), not(contains(eq(99)))))
}

#[test]
fn any_ok_positive_or_err_negative_on_result() -> TestResult<()> {
    let ok_value: std::result::Result<i32, i32> = Ok(5);
    let err_value: std::result::Result<i32, i32> = Err(-3);
    verify_that!(ok_value, any!(ok(gt(0)), err(lt(0))))?;
    verify_that!(err_value, any!(ok(gt(0)), err(lt(0))))
}

#[test]
fn not_some_gt_on_option() -> TestResult<()> {
    let none_val: Option<i32> = None;
    let small_val: Option<i32> = Some(5);
    verify_that!(none_val, not(some(gt(10))))?;
    verify_that!(small_val, not(some(gt(10))))
}

#[test]
fn points_to_vec_each_any_of_allowed_values() -> TestResult<()> {
    let v = vec![1, 2, 1, 3, 2];
    verify_that!(&v, points_to(each(any!(eq(1), eq(2), eq(3)))))
}

#[test]
fn all_len_gt_zero_and_not_empty() -> TestResult<()> {
    let value = vec![42];
    verify_that!(value, all!(len(gt(0)), not(empty())))
}

#[test]
fn not_matches_pattern_on_struct() -> TestResult<()> {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let value = Point { x: 3, y: 4 };
    verify_that!(value, not(matches_pattern!(Point { x: eq(0), y: eq(0) })))
}

#[test]
fn ok_all_starts_with_and_ends_with() -> TestResult<()> {
    let value: std::result::Result<String, i32> = Ok("foobar".to_string());
    verify_that!(value, ok(all!(starts_with("foo"), ends_with("bar"))))
}

#[test]
fn points_to_not_empty_vec() -> TestResult<()> {
    let v = vec![1, 2, 3];
    verify_that!(&v, points_to(not(empty())))
}

#[test]
fn any_of_multiple_enum_patterns() -> TestResult<()> {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    let value = Color::Green;
    verify_that!(value, any!(matches_pattern!(Color::Red), matches_pattern!(Color::Green)))
}

#[test]
fn all_not_err_and_ok_in_range() -> TestResult<()> {
    let value: std::result::Result<i32, String> = Ok(7);
    verify_that!(value, all!(not(err(anything())), ok(all!(gt(0), lt(10)))))
}

#[test]
fn points_to_some_not_zero() -> TestResult<()> {
    let opt: Option<i32> = Some(5);
    verify_that!(&opt, points_to(some(not(eq(0)))))
}

#[test]
fn result_of_option_vec_points_to_some_not_empty() -> TestResult<()> {
    #[derive(Debug)]
    struct Wrapper {
        data: Option<Vec<i32>>,
    }
    impl Wrapper {
        fn data_ref(&self) -> &Option<Vec<i32>> {
            &self.data
        }
    }
    let value = Wrapper { data: Some(vec![1, 2]) };
    verify_that!(value, result_of!(|w: &Wrapper| w.data_ref(), points_to(some(not(empty())))))
}

#[test]
fn some_eq_on_option_of_non_static_str_ref() -> TestResult<()> {
    let s = String::from("hello");
    let value: Option<&str> = Some(&s);
    verify_that!(value, some(eq("hello")))
}

#[test]
fn ok_eq_on_result_of_non_static_str_ref() -> TestResult<()> {
    let s = String::from("hello");
    let value: std::result::Result<&str, i32> = Ok(&s);
    verify_that!(value, ok(eq("hello")))
}

#[test]
fn each_points_to_on_vec_of_non_static_i32_refs() -> TestResult<()> {
    let a = 1i32;
    let b = 2i32;
    let c = 3i32;
    let value: Vec<&i32> = vec![&a, &b, &c];
    verify_that!(value, each(points_to(gt(0))))
}

#[test]
fn contains_each_eq_on_vec_of_non_static_str_refs() -> TestResult<()> {
    let a = String::from("foo");
    let b = String::from("bar");
    let value: Vec<&str> = vec![&a, &b];
    verify_that!(value, contains_each![eq("foo"), eq("bar")])
}

#[test]
fn matches_pattern_on_struct_with_lifetime_parameter_and_ref_field() -> TestResult<()> {
    #[derive(Debug)]
    struct Wrapper<'a> {
        label: &'a str,
        value: &'a i32,
    }
    let s = String::from("answer");
    let n = 42i32;
    let w = Wrapper { label: &s, value: &n };
    verify_that!(w, matches_pattern!(Wrapper { label: eq("answer"), value: points_to(eq(42)) }))
}

#[test]
fn all_starts_with_not_eq_on_non_static_str_ref() -> TestResult<()> {
    let s = String::from("hello world");
    let value: &str = &s;
    verify_that!(value, all!(starts_with("hello"), not(eq("hello"))))
}

#[test]
fn result_of_method_narrowing_lifetime_from_field_to_self() -> TestResult<()> {
    #[derive(Debug)]
    struct Titled<'a> {
        title: &'a str,
    }
    impl<'a> Titled<'a> {
        // Returns &'b str (lifetime of &'b self), narrower than 'a.
        fn title<'b>(&'b self) -> &'b str {
            self.title
        }
    }
    let s = String::from("hello world");
    let value = Titled { title: &s };
    verify_that!(value, result_of!(|t: &Titled| t.title(), contains_substring("world")))
}

#[test]
fn any_some_or_none_on_option_of_non_static_i32_ref() -> TestResult<()> {
    let n = 5i32;
    let some_val: Option<&i32> = Some(&n);
    let none_val: Option<&i32> = None;
    verify_that!(some_val, any!(some(points_to(gt(0))), none()))?;
    verify_that!(none_val, any!(some(points_to(gt(0))), none()))
}

#[test]
fn points_to_some_eq_on_ref_to_option_of_non_static_str_ref() -> TestResult<()> {
    let s = String::from("hello");
    let opt: Option<&str> = Some(&s);
    verify_that!(&opt, points_to(some(eq("hello"))))
}

#[test]
fn each_ok_eq_on_vec_of_results_with_non_static_str_refs() -> TestResult<()> {
    let a = String::from("foo");
    let b = String::from("bar");
    let value: Vec<std::result::Result<&str, i32>> = vec![Ok(&a), Ok(&b)];
    verify_that!(value, each(ok(any!(eq("foo"), eq("bar")))))
}

#[test]
fn each_matches_pattern_on_vec_of_structs_with_lifetime_parameter() -> TestResult<()> {
    #[derive(Debug)]
    struct Item<'a> {
        name: &'a str,
        score: i32,
    }
    let s1 = String::from("alice");
    let s2 = String::from("bob");
    let value = vec![Item { name: &s1, score: 10 }, Item { name: &s2, score: 20 }];
    verify_that!(value, each(matches_pattern!(Item { name: anything(), score: gt(0) })))
}

#[test]
fn matches_pattern_on_struct_with_two_distinct_lifetime_parameters() -> TestResult<()> {
    #[derive(Debug)]
    struct Pair<'a, 'b> {
        first: &'a str,
        second: &'b i32,
    }
    let s = String::from("key");
    let n = 99i32;
    let value = Pair { first: &s, second: &n };
    verify_that!(value, matches_pattern!(Pair { first: eq("key"), second: points_to(eq(99)) }))
}

#[test]
fn result_of_method_returning_vec_of_refs_with_field_lifetime() -> TestResult<()> {
    #[derive(Debug)]
    struct Registry<'a> {
        names: Vec<&'a str>,
    }
    impl<'a> Registry<'a> {
        fn names(&self) -> &Vec<&'a str> {
            &self.names
        }
    }
    let s1 = String::from("alice");
    let s2 = String::from("bob");
    let value = Registry { names: vec![&s1, &s2] };
    verify_that!(
        value,
        result_of!(|r: &Registry| r.names(), points_to(contains_each![eq("alice"), eq("bob")]))
    )
}

#[test]
fn not_contains_eq_on_vec_of_non_static_str_refs() -> TestResult<()> {
    let a = String::from("foo");
    let b = String::from("bar");
    let value: Vec<&str> = vec![&a, &b];
    verify_that!(value, not(contains(eq("baz"))))
}

#[test]
fn result_of_method_returning_option_with_field_lifetime_not_self() -> TestResult<()> {
    // The method returns Option<&'a str> where 'a is the struct's field lifetime,
    // which outlives &self. This tests that result_of! handles the case where
    // the returned reference lifetime is independent of &self.
    #[derive(Debug)]
    struct MaybeNamed<'a> {
        name: Option<&'a str>,
    }
    impl<'a> MaybeNamed<'a> {
        fn name(&self) -> Option<&'a str> {
            self.name
        }
    }
    let s = String::from("carol");
    let value = MaybeNamed { name: Some(&s) };
    verify_that!(
        value,
        result_of!(|m: &MaybeNamed| m.name(), some(all!(starts_with("car"), not(eq("car")))))
    )
}

#[test]
fn contains_exactly_in_order_on_slice_ref() -> TestResult<()> {
    let data = [1, 2, 3];
    let value: &[i32] = &data;
    verify_that!(value, points_to(contains_exactly![eq(1), eq(2), eq(3)].in_order()))
}

#[test]
fn matches_pattern_on_struct_with_slice_field() -> TestResult<()> {
    #[derive(Debug)]
    struct Window<'a> {
        data: &'a [i32],
    }
    let data = [10, 20, 30];
    let value = Window { data: &data };
    verify_that!(value, matches_pattern!(Window { data: points_to(each(gt(0))) }))
}

#[test]
fn result_of_method_returning_slice_with_each_and_gt() -> TestResult<()> {
    #[derive(Debug)]
    struct Buffer {
        data: Vec<i32>,
    }
    impl Buffer {
        fn as_slice(&self) -> &[i32] {
            &self.data
        }
    }
    let value = Buffer { data: vec![5, 10, 15] };
    verify_that!(value, result_of!(|b: &Buffer| b.as_slice(), points_to(each(gt(0)))))
}

#[test]
fn vec_of_slices_each_slice_contains_only_positive() -> TestResult<()> {
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let value: Vec<&[i32]> = vec![&a, &b];
    verify_that!(value, each(points_to(each(gt(0)))))
}

#[test]
fn each_starts_with_on_vec_of_str_slices() -> TestResult<()> {
    let value: Vec<&str> = vec!["hello world", "hello there", "hello!"];
    verify_that!(value, each(starts_with("hello")))
}

#[test]
fn pointwise_on_slice_ref_via_points_to() -> TestResult<()> {
    let data = [10, 20, 30];
    let value: &[i32] = &data;
    verify_that!(value, points_to(pointwise!(eq, [10, 20, 30])))
}

#[test]
fn ok_points_to_contains_exactly_on_result_of_slice() -> TestResult<()> {
    let data = [1, 2, 3];
    let value: std::result::Result<&[i32], String> = Ok(&data);
    verify_that!(value, ok(points_to(contains_exactly![eq(1), eq(2), eq(3)].in_order())))
}

#[test]
fn matches_pattern_on_struct_with_str_field_using_string_matchers() -> TestResult<()> {
    #[derive(Debug)]
    struct Message<'a> {
        text: &'a str,
    }
    let s = String::from("hello world");
    let value = Message { text: &s };
    verify_that!(
        value,
        matches_pattern!(Message {
            text: all!(starts_with("hello"), ends_with("world"), not(eq("hello"))),
        })
    )
}

#[test]
fn some_points_to_contains_each_on_option_of_str_slice() -> TestResult<()> {
    let tags: &[&str] = &["rust", "testing"];
    let value: Option<&[&str]> = Some(tags);
    verify_that!(value, some(points_to(contains_each![eq("rust"), eq("testing")])))
}

#[test]
fn result_of_method_returning_string_slice_with_each_all_string_matchers() -> TestResult<()> {
    #[derive(Debug)]
    struct Catalog {
        items: Vec<String>,
    }
    impl Catalog {
        fn items(&self) -> &[String] {
            &self.items
        }
    }
    let value = Catalog { items: vec!["apple".to_string(), "apricot".to_string()] };
    verify_that!(value, result_of!(|c: &Catalog| c.items(), points_to(each(starts_with("ap")))))
}

#[test]
fn matches_pattern_property_returning_str_narrowed_to_self_lifetime() -> TestResult<()> {
    #[derive(Debug)]
    struct Article {
        title: String,
        body: String,
    }
    impl Article {
        fn title(&self) -> &str {
            &self.title
        }
        fn body(&self) -> &str {
            &self.body
        }
    }
    let value = Article {
        title: "Rust lifetimes".to_string(),
        body: "An in-depth look at lifetimes.".to_string(),
    };
    verify_that!(
        value,
        matches_pattern!(Article {
            title(): starts_with("Rust"),
            body(): contains_substring("lifetimes"),
        })
    )
}

#[test]
fn each_inner_vec_non_empty_and_no_zeros() -> TestResult<()> {
    let value: Vec<Vec<i32>> = vec![vec![1, 2], vec![3], vec![4, 5, 6]];
    verify_that!(value, each(all!(len(gt(0)), not(contains(eq(0))))))
}

#[test]
fn result_of_method_returning_sub_slice_narrowed_to_self_lifetime() -> TestResult<()> {
    #[derive(Debug)]
    struct View<'a> {
        data: &'a [i32],
    }
    impl<'a> View<'a> {
        // Returns a sub-slice with the narrower lifetime of &'b self.
        fn first_two<'b>(&'b self) -> &'b [i32] {
            &self.data[..2]
        }
    }
    let data = [10, 20, 30, 40];
    let value = View { data: &data };
    verify_that!(
        value,
        result_of!(
            |v: &View| v.first_two(),
            points_to(contains_exactly![eq(10), eq(20)].in_order())
        )
    )
}

#[test]
fn contains_each_with_string_matchers_on_slice_of_str_slices() -> TestResult<()> {
    let a = String::from("foobar");
    let b = String::from("foobaz");
    let data: &[&str] = &[&a, &b];
    verify_that!(
        data,
        points_to(contains_each![
            all!(starts_with("foo"), ends_with("bar")),
            all!(starts_with("foo"), ends_with("baz")),
        ])
    )
}

#[test]
fn result_of_option_str_with_field_lifetime_combined_with_string_matchers() -> TestResult<()> {
    #[derive(Debug)]
    struct Config<'a> {
        prefix: Option<&'a str>,
    }
    impl<'a> Config<'a> {
        fn prefix(&self) -> Option<&'a str> {
            self.prefix
        }
    }
    let s = String::from("/api/v2");
    let value = Config { prefix: Some(&s) };
    verify_that!(
        value,
        result_of!(
            |c: &Config| c.prefix(),
            some(all!(starts_with("/"), contains_substring("v2"), not(eq("/"))))
        )
    )
}

#[test]
fn ok_contains_each_in_order_on_result_of_vec() -> TestResult<()> {
    let value: std::result::Result<Vec<i32>, String> = Ok(vec![1, 2, 3, 4, 5]);
    verify_that!(value, ok(contains_each![eq(1), eq(3), eq(5)].in_order()))
}

#[test]
fn not_contains_each_in_order_when_order_is_wrong() -> TestResult<()> {
    let value = vec![1, 2, 3];
    verify_that!(value, not(contains_each![eq(3), eq(1)].in_order()))
}

#[test]
fn all_contains_each_in_order_and_len_constraint() -> TestResult<()> {
    let value = vec![10, 20, 30, 40];
    verify_that!(value, all!(contains_each![eq(10), eq(30)].in_order(), len(gt(2))))
}

#[test]
fn each_contains_exactly_in_order_on_vec_of_vecs() -> TestResult<()> {
    let value = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    verify_that!(value, each(contains_exactly![anything(), anything()].in_order()))
}

#[test]
fn contains_exactly_in_order_with_some_and_none_inner_matchers() -> TestResult<()> {
    let value: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
    verify_that!(value, contains_exactly![some(eq(1)), none(), some(eq(3))].in_order())
}

#[test]
fn contains_each_in_order_with_ok_inner_matchers() -> TestResult<()> {
    let value: Vec<std::result::Result<i32, String>> = vec![Ok(1), Err("bad".into()), Ok(2), Ok(3)];
    verify_that!(value, contains_each![ok(eq(1)), ok(eq(3))].in_order())
}

#[test]
fn matches_pattern_with_vec_field_using_contains_each_in_order() -> TestResult<()> {
    #[derive(Debug)]
    struct Log {
        entries: Vec<String>,
    }
    let value = Log { entries: vec!["start".into(), "processing".into(), "done".into()] };
    verify_that!(
        value,
        matches_pattern!(Log {
            entries: contains_each![starts_with("start"), starts_with("done")].in_order(),
        })
    )
}

#[test]
fn some_contains_each_in_order_on_option_of_vec() -> TestResult<()> {
    let value: Option<Vec<i32>> = Some(vec![1, 2, 3, 4, 5]);
    verify_that!(value, some(contains_each![eq(2), eq(4)].in_order()))
}

#[test]
fn result_of_method_matched_with_contains_exactly_in_order_with_complex_inner_matchers()
-> TestResult<()> {
    #[derive(Debug)]
    struct Pipeline {
        stages: Vec<String>,
    }
    impl Pipeline {
        fn stages(&self) -> &Vec<String> {
            &self.stages
        }
    }
    let value = Pipeline { stages: vec!["build".into(), "test".into(), "deploy".into()] };
    verify_that!(
        value,
        result_of!(
            |p: &Pipeline| p.stages(),
            points_to(
                contains_exactly![
                    eq("build"),
                    all!(not(eq("build")), not(eq("deploy"))),
                    eq("deploy"),
                ]
                .in_order()
            )
        )
    )
}

#[test]
fn points_to_contains_each_in_order_on_ref_to_vec_of_str_slices() -> TestResult<()> {
    let a = String::from("alpha");
    let b = String::from("beta");
    let c = String::from("gamma");
    let v: Vec<&str> = vec![&a, &b, &c];
    verify_that!(&v, points_to(contains_each![eq("alpha"), eq("gamma")].in_order()))
}

#[test]
fn contains_exactly_in_order_with_string_matchers_on_non_static_str_slice_vec() -> TestResult<()> {
    let a = String::from("foobar");
    let b = String::from("bazqux");
    let value: Vec<&str> = vec![&a, &b];
    verify_that!(
        value,
        contains_exactly![
            all!(starts_with("foo"), ends_with("bar")),
            all!(starts_with("baz"), ends_with("qux")),
        ]
        .in_order()
    )
}

#[test]
fn result_of_method_on_lifetime_struct_returning_slice_matched_with_contains_each_in_order()
-> TestResult<()> {
    #[derive(Debug)]
    struct Series<'a> {
        values: &'a [i32],
    }
    impl<'a> Series<'a> {
        fn values(&self) -> &'a [i32] {
            self.values
        }
    }
    let data = [2, 4, 6, 8, 10];
    let value = Series { values: &data };
    verify_that!(
        value,
        result_of!(
            |s: &Series| s.values(),
            points_to(contains_each![eq(2), eq(6), eq(10)].in_order())
        )
    )
}

#[test]
fn any_contains_each_in_order_or_empty() -> TestResult<()> {
    let full: Vec<i32> = vec![1, 2, 3];
    let no_items: Vec<i32> = vec![];
    verify_that!(full, any!(contains_each![eq(1), eq(3)].in_order(), empty()))?;
    verify_that!(no_items, any!(contains_each![eq(1), eq(3)].in_order(), empty()))
}
