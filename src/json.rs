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

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uriparse::URI;

use std::str::FromStr;
use std::collections::HashMap;

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
        if URI::try_from(value).is_ok() {
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

/// Object with identifier and properties.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectWithId {
    /// The identifier of the object.
    pub id: Uri,
    /// The properties of the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub property_set: Option<HashMap<String, Value>>,
}

impl ObjectWithId {

    /// Creates a new `ObjectWithId` from an identifier.
    pub fn new(id: Uri) -> Self {
        Self {
            id,
            property_set: None,
        }
    }

    /// Sets a property.
    pub fn set_property(&mut self, name: &str, value: Value) {
        if self.property_set.is_none() {
            self.property_set = Some(HashMap::new());
        }
        self.property_set.as_mut()
            .expect("The properties must exist.")
            .insert(name.to_string(), value);
    }

    /// Returns the value of a property.
    pub fn get_property(&self, name: &str) -> Option<&Value> {
        self.property_set.as_ref()
            .and_then(|properties| properties.get(name))
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_one_or_many() {
        let mut one = OneOrMany::One(1);
        assert_eq!(one.len(), 1);
        assert_eq!(one.is_empty(), false);
        assert_eq!(one.contains(&1), true);
        assert_eq!(one.contains(&2), false);
        assert_eq!(one.first(), Some(&1));
        assert_eq!(one.to_single(), Some(&1));
        assert_eq!(one.to_single_mut(), Some(&mut 1));

        let mut many = OneOrMany::Many(vec![1, 2, 3]);
        assert_eq!(many.len(), 3);
        assert_eq!(many.is_empty(), false);
        assert_eq!(many.contains(&1), true);
        assert_eq!(many.contains(&2), true);
        assert_eq!(many.contains(&3), true);
        assert_eq!(many.contains(&4), false);
        assert_eq!(many.first(), Some(&1));
        assert_eq!(many.to_single(), None);
        assert_eq!(many.to_single_mut(), None);
    }

    #[test]
    fn test_uri() {
        let uri = Uri::new("https://example.com").unwrap();
        assert_eq!(uri.to_string(), "https://example.com");
        assert_eq!(uri, Uri::from_str("https://example.com").unwrap());
        assert_eq!(
            uri,
            Uri::try_from("https://example.com".to_string()).unwrap()
        );
    }

    #[test]
    fn test_string_or_uri() {
        let uri = StringOrUri::try_from("https://example.com").unwrap();
        assert_eq!(uri.as_str(), "https://example.com");
        assert_eq!(uri, StringOrUri::from_str("https://example.com").unwrap());
        assert_eq!(
            uri,
            StringOrUri::try_from("https://example.com".to_string()).unwrap()
        );

        let string = StringOrUri::try_from("example").unwrap();
        assert_eq!(string.as_str(), "example");
        assert_eq!(string, StringOrUri::from_str("example").unwrap());
        assert_eq!(
            string,
            StringOrUri::try_from("example".to_string()).unwrap()
        );
    }

    #[test]
    fn test_object_with_id() {
        let mut object = ObjectWithId::new(Uri::new("https://example.com").unwrap());
        object.set_property("name", Value::String("example".to_string()));
        assert_eq!(object.get_property("name"), Some(&Value::String("example".to_string())));
    }

}
