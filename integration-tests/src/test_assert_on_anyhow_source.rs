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

use anyhow::Context as _;

fn main() {}

#[allow(unused)]
fn returns_anyhow_error() -> anyhow::Result<()> {
    anyhow::bail!("Anyhow error")
}

#[allow(unused)]
fn returns_wrapping_anyhow_error() -> anyhow::Result<()> {
    returns_anyhow_error().context("Wrapping context")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use test_that::prelude::*;

    use crate::returns_wrapping_anyhow_error;

    #[test]
    fn allows_asserting_on_error_source() -> Result<()> {
        let result = returns_wrapping_anyhow_error();

        verify_that!(
            result,
            err(matches_pattern!(anyhow::Error {
                source(): some(displays_as(eq("Anyhow error")))
            }))
        )
    }
}
