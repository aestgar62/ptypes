// Copyright 2023 Antonio Estevez <aestevez@opencanarias.es>

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing
// permissions and limitations under the License.

//! # Miscellany of types
//!

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

use std::convert::TryFrom;

/// Base64 encoding using the URL- and filename-safe character set defined by Section 5
/// of RFC 4648 [RFC4648](https://tools.ietf.org/html/rfc4648#section-5).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, Eq, Zeroize)]
#[serde(try_from = "String")]
#[serde(into = "Base64urlUIntString")]
pub struct Base64urlUInt(pub Vec<u8>);
type Base64urlUIntString = String;

impl TryFrom<String> for Base64urlUInt {
    type Error = base64::DecodeError;
    fn try_from(data: String) -> Result<Self, Self::Error> {
        Ok(Base64urlUInt(
            general_purpose::STANDARD_NO_PAD.decode(data)?,
        ))
    }
}

impl From<&Base64urlUInt> for String {
    fn from(data: &Base64urlUInt) -> String {
        general_purpose::STANDARD_NO_PAD.encode(&data.0)
    }
}

impl From<Base64urlUInt> for Base64urlUIntString {
    fn from(data: Base64urlUInt) -> Base64urlUIntString {
        String::from(&data)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_base64url_uint() {
        let data = Base64urlUInt(vec![1, 2, 3]);
        assert_eq!(data, Base64urlUInt::try_from("AQID".to_string()).unwrap());
        let string = String::from(&data);
        assert_eq!(string, "AQID");
        let string: Base64urlUIntString = data.into();
        assert_eq!(string, "AQID");
    }
}