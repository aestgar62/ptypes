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

//! # ptypes
//!
//! Particular types for different specifications like based on the JSON format, such as JOSE 
//! (Javascript Object Signing and Encryption) and JSON-LD (JSON Linked Data).
//! 
//! ## Usage
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! ptypes = "0.1.4"
//! ```
//! 
//! ## Examples
//! 
//! ### `OneOrMany`
//! 
//! It is used for properties of a JSON that can be an element of a type or a collection of 
//! elements of the same kind—for example, the [Type](https://www.w3.org/TR/vc-data-model/#types) 
//! property in the verifiable credentials specification.
//! 
//! ```rust
//! use ptypes::OneOrMany;
//! 
//! let one = OneOrMany::One("one".to_string());
//! let many = OneOrMany::Many(vec!["one".to_string(), "two".to_string()]);
//! 
//! assert_eq!(one, OneOrMany::One("one".to_string()));
//! assert_eq!(many, OneOrMany::Many(vec!["one".to_string(), "two".to_string()]));
//! ```
//! TBD: Add more examples

pub mod error;

pub mod misc;

#[cfg(feature = "json")]
pub mod json;


pub use error::Error;

#[cfg(feature = "json")]
pub use json::OneOrMany;

#[cfg(feature = "json")]
pub use json::Uri;

#[cfg(feature = "json")]
pub use json::StringOrUri;

#[cfg(feature = "json")]
pub use json::ObjectWithId;

pub use misc::Base64urlUInt;

#[cfg(test)]
mod tests {}
