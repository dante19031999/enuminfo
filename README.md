

# enuminfo

A comprehensive collection of procedural derive macros and traits for Rust `enum` types providing boolean checkers, string conversions, name lookup constructors, variant slices and counting, casing transformations, and dynamic property queries compatible with `serde`.

[![Crates.io](https://img.shields.io/crates/v/enuminfo_macros.svg)](https://crates.io/crates/enuminfo)
[![Documentation](https://docs.rs/enuminfo_macros/badge.svg)](https://docs.rs/enuminfo)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)]()

---

## Features

- **`#[derive(EnumIs)]`**: Generates boolean helper methods (`is_<variant_name>()`) for all enum variants and implements the `EnumIs` marker trait.
- **`#[derive(EnumName)]`**: Generates `const` methods (`name()` and `raw_name()`) returning variant names as string slices and implements the `EnumName` trait.
- **`#[derive(EnumFromName)]`**: Generates lookup constructors (`from_name()` and `from_raw_name()`) to construct unit enum variants from string names, implements the `EnumFromName` trait, and optionally implements `std::str::FromStr`.
- **`#[derive(EnumVariants)]`**: Generates static slice `VARIANTS`, `variants()`, and `variant_count()` for unit enums and implements the `EnumVariants` trait.
- **`EnumProperty` Trait**: Provides default implementations for custom typed property queries (`get_string_property()`, `get_i32_property()`, etc.) by key name.
- **Serde-compatible Casing**: Supports container-level `rename_all` attributes using standard casing options (`snake_case`, `kebab-case`, `camelCase`, etc.).

---

## Installation

Add `enuminfo` to your `Cargo.toml`:

```toml
[dependencies]
enuminfo = "0.1"
```

### Feature Flags

- **`from-str`** *(enabled by default)*: Automatically implements `std::str::FromStr` for enums deriving `EnumFromName`. Returns `enuminfo::error::EnumFromNameError` on parse failure.
- **`skip-inherent`**: Omits inherent method definitions on the enum, exposing functionality strictly through the traits (`EnumIs`, `EnumName`, `EnumFromName`, `EnumVariants`). Useful to avoid name collisions with your own enum methods.

---

## Usage & Examples

### 1. `EnumIs`

Generates `is_<variant_name>(&self) -> bool` methods in `snake_case` for every variant.

```rust
use enuminfo::EnumIs;

#[derive(EnumIs)]
enum UserRole {
    Admin,
    StandardUser,
    SuperAdmin,
}

fn main() {
    let role = UserRole::StandardUser;

    assert!(role.is_standard_user());
    assert!(!role.is_admin());
    assert!(!role.is_super_admin());
}
```

---

### 2. `EnumName`

Provides string inspection methods:

- `name(&self) -> &'static str`: Returns the formatted variant name (respecting `rename` and `rename_all`).
- `raw_name(&self) -> &'static str`: Returns the exact identifier name as written in Rust code.

```rust
use enuminfo::EnumName;

#[derive(EnumName)]
#[enuminfo(rename_all = "snake_case")]
enum Status {
    Pending,
    #[enuminfo(rename = "custom_in_progress")]
    InProgress,
    Completed,
}

fn main() {
    let status = Status::InProgress;

    // Converted name (respects variant rename override)
    assert_eq!(status.name(), "custom_in_progress");

    // Original identifier name
    assert_eq!(status.raw_name(), "InProgress");
}
```

---

### 3. `EnumFromName`

Constructs unit enum variants from string names:

- `from_name(name: &str) -> Option<Self>`: Matches formatted names (respecting `rename` and `rename_all`).
- `from_raw_name(name: &str) -> Option<Self>`: Matches original identifier names.
- When the `from-str` feature is enabled (default), it also implements `std::str::FromStr`.

> **Note:** `EnumFromName` supports unit enums (variants without data fields).

```rust
use enuminfo::EnumFromName;
use std::str::FromStr;

#[derive(Debug, PartialEq, EnumFromName)]
#[enuminfo(rename_all = "kebab-case")]
enum Priority {
    Low,
    MediumPriority,
    #[enuminfo(ignore_from_name)]
    Critical,
}

fn main() {
    // Lookup by formatted name
    assert_eq!(Priority::from_name("medium-priority"), Some(Priority::MediumPriority));

    // Lookup by raw identifier name
    assert_eq!(Priority::from_raw_name("MediumPriority"), Some(Priority::MediumPriority));

    // Ignored variants will not match via `from_name`
    assert_eq!(Priority::from_name("critical"), None);

    // std::str::FromStr parsing
    assert_eq!(Priority::from_str("medium-priority").unwrap(), Priority::MediumPriority);
    assert!(Priority::from_str("critical").is_err());
}
```

---

### 4. `EnumVariants`

Provides static slice access and variant counting for unit enums:

- `VARIANTS: &'static [Self]`: Static slice of all non-ignored variants.
- `variants() -> &'static [Self]`: Accessor method returning the static slice.
- `variant_count() -> usize`: Total number of variants in the slice.

> **Note:** `EnumVariants` supports unit enums.

```rust
use enuminfo::EnumVariants;

#[derive(Debug, PartialEq, EnumVariants)]
enum Direction {
    North,
    South,
    East,
    West,
    #[enuminfo(ignore_variants)]
    Unknown,
}

fn main() {
    // Access static slice
    assert_eq!(
        Direction::VARIANTS,
        &[Direction::North, Direction::South, Direction::East, Direction::West]
    );

    // Access via method
    assert_eq!(Direction::variants(), Direction::VARIANTS);

    // Get count
    assert_eq!(Direction::variant_count(), 4);
}
```

---

### 5. `EnumProperty`

A trait providing default getter methods for associating typed metadata (strings, integers, floats, `SystemTime`) with enum variants. Implementors can override only the specific properties they need.

```rust
use enuminfo::EnumProperty;

enum HttpMethod {
    Get,
    Post,
}

impl EnumProperty for HttpMethod {
    fn get_string_property(&self, key: &str) -> Option<String> {
        match (self, key) {
            (HttpMethod::Get, "description") => Some("Retrieve data".into()),
            (HttpMethod::Post, "description") => Some("Submit data".into()),
            _ => None,
        }
    }
}

fn main() {
    let method = HttpMethod::Get;
    assert_eq!(method.get_string_property("description"), Some("Retrieve data".into()));
    assert_eq!(method.get_string_property("unknown"), None);
}
```

---

## Attribute Reference

| Attribute | Target | Description | Example |
|---|---|---|---|
| `#[enuminfo(rename_all = "...")]` | Enum | Applies a casing convention to all variants | `#[enuminfo(rename_all = "snake_case")]` |
| `#[enuminfo(rename = "...")]` | Variant | Overrides the formatted name for a specific variant | `#[enuminfo(rename = "custom_val")]` |
| `#[enuminfo(ignore_from_name)]` | Variant | Excludes a variant from `from_name()` lookup | `#[enuminfo(ignore_from_name)]` |
| `#[enuminfo(ignore_variants)]` | Variant | Excludes a variant from `VARIANTS` and `variants()` | `#[enuminfo(ignore_variants)]` |

---

## Casing Conventions (`rename_all`)

The `rename_all` attribute supports standard `serde`-compatible casing strings:

| Value | Transformation | Example (`MyVariant`) |
|---|---|---|
| `"lowercase"` | Lowercase | `"myvariant"` |
| `"UPPERCASE"` | Uppercase | `"MYVARIANT"` |
| `"camelCase"` / `"lowerCamelCase"` | Lower camel case | `"myVariant"` |
| `"snake_case"` | Snake case | `"my_variant"` |
| `"SCREAMING-SNAKE-CASE"` | Screaming snake case | `"MY_SNAKE_CASE"` |
| `"kebab-case"` | Kebab case | `"my-variant"` |
| `"SCREAMING-KEBAB-CASE"` | Screaming kebab case | `"MY-KEBAB-CASE"` |
| `"PascalCase"` / `"UpperCamelCase"` | Upper camel case | `"MyVariant"` |

---

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Acknowledgements: https://dante19031999.github.io/enuminfo-macros/ACKNOWLEDGEMENTS.html

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.