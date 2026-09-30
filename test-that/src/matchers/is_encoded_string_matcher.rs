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

use crate::matcher::Matcher;
use alloc::string::String;

/// Matches a byte sequence which is a UTF-8 encoded string matched by `inner`.
///
/// The matcher reports no match if either the string is not UTF-8 encoded or if
/// `inner` does not match on the decoded string.
///
/// The input may be a slice `&[u8]` or a `Vec` of bytes.
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// let bytes: &[u8] = "A string".as_bytes();
/// verify_that!(bytes, is_utf8_string(eq("A string")))?; // Passes
/// let bytes: Vec<u8> = "A string".as_bytes().to_vec();
/// verify_that!(bytes, is_utf8_string(eq("A string")))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> TestResult<()> {
/// # let bytes: &[u8] = "A string".as_bytes();
/// verify_that!(bytes, is_utf8_string(eq("Another string")))?; // Fails (inner matcher does not match)
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> TestResult<()> {
/// let bytes: Vec<u8> = vec![255, 64, 128, 32];
/// verify_that!(bytes, is_utf8_string(anything()))?; // Fails (not UTF-8 encoded)
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// ```
pub fn is_utf8_string<InnerMatcherT>(
    inner: InnerMatcherT,
) -> __internal::IsEncodedStringMatcher<InnerMatcherT, __internal::Utf8>
where
    InnerMatcherT: Matcher<String>,
{
    __internal::IsEncodedStringMatcher { inner, decoder: __internal::Utf8 }
}

/// Matches a byte sequence which is a string in the given `encoding` matched by
/// `inner`.
///
/// The matcher reports no match if either the bytes are malformed in the given
/// encoding or if `inner` does not match on the decoded string. No byte order
/// mark is interpreted: the bytes are always decoded with `encoding`.
///
/// The input may be a slice `&[u8]` or a `Vec` of bytes.
///
/// This matcher requires the `encoding_rs` feature. The `encoding` argument is
/// an [`encoding_rs::Encoding`], such as `encoding_rs::WINDOWS_1252`,
/// `encoding_rs::SHIFT_JIS`, or `encoding_rs::UTF_16LE`.
///
/// ```
/// # use test_that::prelude::*;
/// # fn should_pass() -> TestResult<()> {
/// let bytes: &[u8] = &[0xE4, 0xF6, 0xFC]; // "äöü" in Windows-1252
/// verify_that!(bytes, is_encoded_string(encoding_rs::WINDOWS_1252, eq("äöü")))?; // Passes
/// let bytes: Vec<u8> = vec![b'h', 0, b'i', 0]; // "hi" in UTF-16LE
/// verify_that!(bytes, is_encoded_string(encoding_rs::UTF_16LE, eq("hi")))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> TestResult<()> {
/// # let bytes: &[u8] = &[0xE4, 0xF6, 0xFC];
/// verify_that!(bytes, is_encoded_string(encoding_rs::WINDOWS_1252, eq("abc")))?; // Fails (inner matcher does not match)
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> TestResult<()> {
/// let bytes: Vec<u8> = vec![0xFF, 0xFF];
/// verify_that!(bytes, is_encoded_string(encoding_rs::SHIFT_JIS, anything()))?; // Fails (malformed Shift_JIS)
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// ```
#[cfg(feature = "encoding_rs")]
pub fn is_encoded_string<InnerMatcherT>(
    encoding: &'static encoding_rs::Encoding,
    inner: InnerMatcherT,
) -> __internal::IsEncodedStringMatcher<InnerMatcherT, __internal::EncodingRs>
where
    InnerMatcherT: Matcher<String>,
{
    __internal::IsEncodedStringMatcher { inner, decoder: __internal::EncodingRs(encoding) }
}

pub mod __internal {
    use crate::{
        description::Description,
        matcher::{Describable, Matcher, MatcherResult},
    };
    use alloc::string::{String, ToString as _};
    use core::fmt::Debug;

    /// Decodes a byte sequence in a specific character encoding.
    #[doc(hidden)]
    pub trait Decoder {
        /// Name of the encoding as shown in descriptions.
        fn name(&self) -> &str;

        /// Decodes `bytes`, or returns a description of why that is not
        /// possible.
        fn decode(&self, bytes: &[u8]) -> Result<String, String>;
    }

    #[doc(hidden)]
    pub struct Utf8;

    impl Decoder for Utf8 {
        fn name(&self) -> &str {
            "UTF-8"
        }

        fn decode(&self, bytes: &[u8]) -> Result<String, String> {
            core::str::from_utf8(bytes).map(String::from).map_err(|e| e.to_string())
        }
    }

    #[cfg(feature = "encoding_rs")]
    #[doc(hidden)]
    pub struct EncodingRs(pub(super) &'static encoding_rs::Encoding);

    #[cfg(feature = "encoding_rs")]
    impl Decoder for EncodingRs {
        fn name(&self) -> &str {
            self.0.name()
        }

        fn decode(&self, bytes: &[u8]) -> Result<String, String> {
            self.0
                .decode_without_bom_handling_and_without_replacement(bytes)
                .map(String::from)
                .ok_or_else(|| "malformed byte sequence".to_string())
        }
    }

    #[doc(hidden)]
    pub struct IsEncodedStringMatcher<InnerMatcherT, DecoderT> {
        pub(super) inner: InnerMatcherT,
        pub(super) decoder: DecoderT,
    }

    impl<ActualT, InnerMatcherT, DecoderT> Matcher<ActualT>
        for IsEncodedStringMatcher<InnerMatcherT, DecoderT>
    where
        ActualT: AsRef<[u8]> + Debug,
        InnerMatcherT: Matcher<String>,
        DecoderT: Decoder,
    {
        fn matches(&self, actual: &ActualT) -> MatcherResult {
            self.decoder
                .decode(actual.as_ref())
                .map(|s| self.inner.matches(&s))
                .unwrap_or(MatcherResult::NoMatch)
        }

        fn explain_match(&self, actual: &ActualT) -> Description {
            let name = self.decoder.name();
            match self.decoder.decode(actual.as_ref()) {
                Ok(s) => {
                    format!("which is a {name} encoded string {}", self.inner.explain_match(&s))
                        .into()
                }
                Err(e) => format!("which is not a {name} encoded string: {e}").into(),
            }
        }
    }

    impl<InnerMatcherT: Describable, DecoderT: Decoder> Describable
        for IsEncodedStringMatcher<InnerMatcherT, DecoderT>
    {
        fn describe(&self, matcher_result: MatcherResult) -> Description {
            let name = self.decoder.name();
            match matcher_result {
                MatcherResult::Match => format!(
                    "is a {name} encoded string which {}",
                    self.inner.describe(MatcherResult::Match)
                )
                .into(),
                MatcherResult::NoMatch => format!(
                    "is not a {name} encoded string which {}",
                    self.inner.describe(MatcherResult::Match)
                )
                .into(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matcher::{Describable as _, MatcherResult};
    use crate::prelude::*;

    #[test]
    fn matches_string_as_byte_slice() -> TestResult<()> {
        verify_that!("A string".as_bytes(), is_utf8_string(eq("A string")))
    }

    #[test]
    fn matches_string_as_byte_vec() -> TestResult<()> {
        verify_that!("A string".as_bytes().to_vec(), is_utf8_string(eq("A string")))
    }

    #[test]
    fn matches_string_with_utf_8_encoded_sequences() -> TestResult<()> {
        verify_that!("äöüÄÖÜ".as_bytes().to_vec(), is_utf8_string(eq("äöüÄÖÜ")))
    }

    #[test]
    fn does_not_match_non_equal_string() -> TestResult<()> {
        verify_that!("äöüÄÖÜ".as_bytes().to_vec(), not(is_utf8_string(eq("A string"))))
    }

    #[test]
    fn does_not_match_non_utf_8_encoded_byte_sequence() -> TestResult<()> {
        verify_that!(&[192, 64, 255, 32], not(is_utf8_string(eq("A string"))))
    }

    #[test]
    fn has_correct_description_in_matched_case() -> TestResult<()> {
        let matcher = is_utf8_string(eq("A string"));

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq("is a UTF-8 encoded string which is equal to \"A string\""))
        )
    }

    #[test]
    fn has_correct_description_in_not_matched_case() -> TestResult<()> {
        let matcher = is_utf8_string(eq("A string"));

        verify_that!(
            matcher.describe(MatcherResult::NoMatch),
            displays_as(eq("is not a UTF-8 encoded string which is equal to \"A string\""))
        )
    }

    #[test]
    fn has_correct_explanation_in_matched_case() -> TestResult<()> {
        let explanation = is_utf8_string(eq("A string")).explain_match(&"A string".as_bytes());

        verify_that!(
            explanation,
            displays_as(eq("which is a UTF-8 encoded string which is equal to \"A string\""))
        )
    }

    #[test]
    fn has_correct_explanation_when_byte_array_is_not_utf8_encoded() -> TestResult<()> {
        let explanation = is_utf8_string(eq("A string")).explain_match(&&[192, 128, 0, 64]);

        verify_that!(explanation, displays_as(starts_with("which is not a UTF-8 encoded string: ")))
    }

    #[test]
    fn has_correct_explanation_when_inner_matcher_does_not_match() -> TestResult<()> {
        let explanation =
            is_utf8_string(eq("A string")).explain_match(&"Another string".as_bytes());

        verify_that!(
            explanation,
            displays_as(eq("which is a UTF-8 encoded string which isn't equal to \"A string\""))
        )
    }
}

#[cfg(all(test, feature = "encoding_rs"))]
mod encoding_rs_tests {
    use crate::matcher::{Describable as _, MatcherResult};
    use crate::prelude::*;

    #[test]
    fn matches_windows_1252_string() -> TestResult<()> {
        verify_that!(&[0xE4, 0xF6, 0xFC], is_encoded_string(encoding_rs::WINDOWS_1252, eq("äöü")))
    }

    #[test]
    fn matches_shift_jis_string_in_byte_vec() -> TestResult<()> {
        verify_that!(
            vec![0x93u8, 0xFA, 0x96, 0x7B],
            is_encoded_string(encoding_rs::SHIFT_JIS, eq("日本"))
        )
    }

    #[test]
    fn matches_utf_16le_string() -> TestResult<()> {
        verify_that!(vec![b'h', 0u8, b'i', 0], is_encoded_string(encoding_rs::UTF_16LE, eq("hi")))
    }

    #[test]
    fn does_not_strip_byte_order_mark() -> TestResult<()> {
        verify_that!(
            vec![0xEFu8, 0xBB, 0xBF, b'a'],
            is_encoded_string(encoding_rs::UTF_8, eq("\u{FEFF}a"))
        )
    }

    #[test]
    fn does_not_match_when_inner_matcher_does_not_match() -> TestResult<()> {
        verify_that!(
            &[0xE4, 0xF6, 0xFC],
            not(is_encoded_string(encoding_rs::WINDOWS_1252, eq("A string")))
        )
    }

    #[test]
    fn does_not_match_malformed_byte_sequence() -> TestResult<()> {
        verify_that!(&[0xFF, 0xFF], not(is_encoded_string(encoding_rs::SHIFT_JIS, anything())))
    }

    #[test]
    fn does_not_match_odd_length_utf_16_sequence() -> TestResult<()> {
        verify_that!(&[b'h', 0, b'i'], not(is_encoded_string(encoding_rs::UTF_16LE, anything())))
    }

    #[test]
    fn description_contains_encoding_name() -> TestResult<()> {
        let matcher = is_encoded_string(encoding_rs::SHIFT_JIS, eq("A string"));

        verify_that!(
            matcher.describe(MatcherResult::Match),
            displays_as(eq("is a Shift_JIS encoded string which is equal to \"A string\""))
        )
    }

    #[test]
    fn explanation_contains_encoding_name_when_malformed() -> TestResult<()> {
        let explanation =
            is_encoded_string(encoding_rs::SHIFT_JIS, eq("A string")).explain_match(&&[0xFF, 0xFF]);

        verify_that!(
            explanation,
            displays_as(eq("which is not a Shift_JIS encoded string: malformed byte sequence"))
        )
    }
}
