# ptypes

[![Rust](https://img.shields.io/badge/Rust-v1.66.0-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-Apache--2.0-green)](https://github.com/aestgar62/ptypes/blob/v0.1.1/LICENSE)
[![Build & Test]((https://github.com/aestgar62/ptypes/actions/workflows/rust.yml/badge.svg))](https://github.com/aestgar62/ptypes/blob/main/.github/workflows/rust.yml?branch=main)

Particular types for different specifications like based on the JSON format, such as JOSE (Javascript Object Signing and Encryption) and JSON-LD (JSON Linked Data).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ptypes = "0.1.4"
```

## Examples

### `OneOrMany`

It is used for properties of a JSON that can be an element of a type or a collection of elements of the same kind—for example, the [Type](https://www.w3.org/TR/vc-data-model/#types) property in the verifiable credentials specification.

```rust
use ptypes::OneOrMany;

let one = OneOrMany::One(1);
let many = OneOrMany::Many(vec![1, 2, 3]);

assert_eq!(one.is_one(), true);
assert_eq!(many.is_many(), true);
```

### `Uri`

A Uniform Resource Identifier, as defined by [RFC3986](https://www.rfc-editor.org/rfc/rfc3986).

```rust
use ptypes::Uri;

let uri = Uri::try_from("https://example.com".to_string()).unwrap();
assert_eq!(uri, Uri::try_from("https://example.com".to_string()).unwrap());
```

### `StringOrUri`

Used when a property can be either a string or a URI.

```rust
use ptypes::StringOrUri;

let string = StringOrUri::String("string".to_string());
let uri = StringOrUri::Uri(Uri::try_from("https://example.com".to_string()).unwrap());

assert_eq!(string.is_string(), true);
assert_eq!(uri.is_uri(), true);
```

### `Base64urlUInt`

Base64 encoding using the URL- and filename-safe character set defined in [Section 5 of RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-5).

```rust
use ptypes::Base64urlUInt;

let data = Base64urlUInt(vec![1, 2, 3]);
assert_eq!(data, Base64urlUInt::try_from("AQID".to_string()).unwrap());
```
