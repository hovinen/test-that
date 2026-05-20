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

use googletest::prelude::*;

#[derive(Debug)]
struct StructWithField {
    field: u32,
}

impl StructWithField {
    fn returns_field_in_result(&self) -> std::result::Result<u32, ()> {
        Ok(self.field)
    }
}

#[test]
fn can_use_displays_as_inside_okay() -> Result<()> {
    let subject = StructWithField { field: 123 };

    verify_that!(
        subject,
        matches_pattern!(StructWithField {
            returns_field_in_result(): ok(displays_as(eq("123")))
        })
    )
}
