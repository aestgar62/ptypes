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

//! # JSON Types
//!
//! This crate module a set of types to work with JSON based specifications, like Javascript
//! Object Signing and Encryption (JOSE) and JSON Linked Data (JSON-LD).
//!

#![deny(missing_docs)]

use crate::Error;

use uriparse::URI;
use serde::{Deserialize, Serialize};

use std::str::FromStr;

/// It is an enum to support properties with a single value or an array of values.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    /// It is a single value.
    One(T),
    /// It is an array of values.
    Many(Vec<T>),
}

/// Implementation of `OneOrMany`.
impl<T> OneOrMany<T> {
    /// Returns `true` if funtion `f` returns `true` for any element in the `OneOrMany`.
    pub fn any<F>(&self, f: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        match self {
            Self::One(value) => f(value),
            Self::Many(values) => values.iter().any(f),
        }
    }

    /// Returns the number of elements in the `OneOrMany`.
    pub fn len(&self) -> usize {
        match self {
            Self::One(_) => 1,
            Self::Many(values) => values.len(),
        }
    }

    /// Returns `true` if the `OneOrMany` is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::One(_) => false,
            Self::Many(values) => values.is_empty(),
        }
    }

    /// Returns `true` if the `OneOrMany` contains the given value.
    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq<T>,
    {
        match self {
            Self::One(value) => x == value,
            Self::Many(values) => values.contains(x),
        }
    }

    /// Returns the first element of the `OneOrMany`.
    pub fn first(&self) -> Option<&T> {
        match self {
            Self::One(value) => Some(value),
            Self::Many(values) => {
                if !values.is_empty() {
                    Some(&values[0])
                } else {
                    None
                }
            }
        }
    }

    /// Returns single element if the `OneOrMany` contains only one element.
    pub fn to_single(&self) -> Option<&T> {
        match self {
            Self::One(value) => Some(value),
            Self::Many(values) => {
                if values.len() == 1 {
                    Some(&values[0])
                } else {
                    None
                }
            }
        }
    }

    /// Returns single element as mutables if the `OneOrMany` contains only one element.
    pub fn to_single_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::One(value) => Some(value),
            Self::Many(values) => {
                if values.len() == 1 {
                    Some(&mut values[0])
                } else {
                    None
                }
            }
        }
    }
}

/// Consuming iterator
impl<T> IntoIterator for OneOrMany<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::One(value) => vec![value].into_iter(),
            Self::Many(values) => values.into_iter(),
        }
    }
}

/// Non-consuming iterator
impl<'a, T> IntoIterator for &'a OneOrMany<T> {
    type Item = &'a T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            OneOrMany::One(value) => vec![value].into_iter(),
            OneOrMany::Many(values) => values.iter().collect::<Vec<Self::Item>>().into_iter(),
        }
    }
}

/// Wrapper for `URI` to support serialization and deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Uri(String);

/// Implementation of `Uri`.
impl Uri {
    /// Creates a new `Uri` from a string.
    pub fn new(value: &str) -> Result<Self, Error> {
        if let Ok(_) = URI::try_from(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(Error::InvalidUri)
        }
    }

    /// Returns the `URI` representation.
    pub fn as_uri(&self) -> URI {
        URI::try_from(self.0.as_str()).expect("The URI must be valid")
    }
}

impl From<Uri> for String {
    fn from(uri: Uri) -> String {
        uri.0
    }
}

impl std::convert::TryFrom<String> for Uri {
    type Error = Error;
    fn try_from(uri: String) -> Result<Self, Self::Error> {
        Uri::new(uri.as_str())
    }
}

impl FromStr for Uri {
    type Err = Error;
    fn from_str(uri: &str) -> Result<Self, Self::Err> {
        Uri::new(uri)
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


/// It is an enum to support properties that can be a `String` or a `URI`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrUri {
    /// It is a `String`.
    String(String),
    /// It is a `URI`.
    Uri(Uri),
}

impl StringOrUri {
    /// Returns the string representation of the `StringOrURI`.
    pub fn as_str(&self) -> &str {
        match self {
            StringOrUri::Uri(uri) => uri.0.as_str(),
            StringOrUri::String(string) => string.as_str(),
        }
    }
}

impl TryFrom<String> for StringOrUri {
    type Error = Error;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.contains(':') {
            let uri = Uri::new(string.as_str())?;
            Ok(Self::Uri(uri))
        } else {
            Ok(Self::String(string))
        }
    }
}
impl TryFrom<&str> for StringOrUri {
    type Error = Error;
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        string.to_string().try_into()
    }
}

impl From<Uri> for StringOrUri {
    fn from(uri: Uri) -> Self {
        StringOrUri::Uri(uri)
    }
}

impl From<StringOrUri> for String {
    fn from(id: StringOrUri) -> Self {
        match id {
            StringOrUri::Uri(uri) => uri.into(),
            StringOrUri::String(s) => s,
        }
    }
}

impl FromStr for StringOrUri {
    type Err = Error;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string.to_string().try_into()
    }
}

