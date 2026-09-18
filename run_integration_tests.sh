#!/bin/bash
#
# Copyright 2022 Google LLC
# Copyright 2026 Bradford Hovinen <bradford@hovinen.me>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Shell script to build and run the library integration tests. These will not be
# run with "cargo test" due to limitations in Cargo.
#
# To use this, just run the script in the root directory of Test That!. You
# must have the Rust toolchain available.

set -e

INTEGRATION_TEST_BINARIES=(
  "integration-tests"
  "assert-predicate-with-failure"
  "assertion-failure-in-subroutine"
  "assertion-failures-with-short-structured-actual-values"
  "async-test-with-expect-that"
  "custom-error-message"
  "expect-pred-failure"
  "expect-that-failure"
  "failure-due-to-fail-macro"
  "failure-due-to-fail-macro-with-empty-message"
  "failure-due-to-fail-macro-with-format-arguments"
  "failure-due-to-returned-error"
  "fatal-and-non-fatal-failure"
  "first-failure-aborts"
  "test-that-test-with-rstest"
  "non-fatal-failure-in-subroutine"
  "passing-test-with-should-panic"
  "simple-assertion-failure"
  "simple-assertion-failure-with-assert-that"
  "test-returning-anyhow-error"
  "two-expect-pred-failures"
  "two-expect-that-failures"
  "two-non-fatal-failures"
  "verify-predicate-with-failure"
  "verify-predicate-with-failure-as-method-in-submodule"
)

cargo build
for binary in ${INTEGRATION_TEST_BINARIES[@]}; do
  cargo rustc -p integration-tests --bin $binary -- --test
done
./target/debug/integration-tests
