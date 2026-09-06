//! # enuminfo
//!
//! A comprehensive collection of procedural derive macros and traits for Rust `enum` types,
//! providing boolean variant checkers, string conversions, name lookup constructors,
//! variant slices and counting, casing transformations, and typed property queries.
//!
//! ## Highlights
//!
//! - **[`EnumIs`]**: Generates boolean predicate methods (`is_<variant>()`) for variants.
//! - **[`EnumName`]**: Returns formatted (`name()`) and original (`raw_name()`) variant identifiers.
//! - **[`EnumFromName`]**: Constructs unit variants from string names (`from_name()` and `from_raw_name()`).
//!   Also implements [`std::str::FromStr`] by default.
//! - **[`EnumVariants`]**: Provides a static slice (`VARIANTS`), accessor (`variants()`), and count (`variant_count()`).
//! - **[`EnumProperty`]**: Trait providing default implementations for custom typed property lookups on variants.
//! - **Serde-compatible casing**: Fully configurable casing transformations via `#[enuminfo(rename_all = "...")]`.
//!
//! ---
//!
//! ## Quick Start
//!
//! ```rust
//! use enuminfo::{EnumIs, EnumName, EnumFromName, EnumVariants};
//!
//! #[derive(Debug, PartialEq, EnumIs, EnumName, EnumFromName, EnumVariants)]
//! #[enuminfo(rename_all = "snake_case")]
//! enum TaskStatus {
//!     Pending,
//!     #[enuminfo(rename = "in_progress_custom")]
//!     InProgress,
//!     Completed,
//! }
//!
//! let status = TaskStatus::InProgress;
//!
//! // Boolean checkers (EnumIs)
//! assert!(status.is_in_progress());
//! assert!(!status.is_pending());
//!
//! // String conversion (EnumName)
//! assert_eq!(status.name(), "in_progress_custom");
//! assert_eq!(status.raw_name(), "InProgress");
//!
//! // Name lookup (EnumFromName)
//! assert_eq!(TaskStatus::from_name("pending"), Some(TaskStatus::Pending));
//! assert_eq!(TaskStatus::from_raw_name("InProgress"), Some(TaskStatus::InProgress));
//! assert_eq!(TaskStatus::from_name("in_progress_custom"), Some(TaskStatus::InProgress));
//!
//! // FromStr support (from-str feature, enabled by default)
//! use std::str::FromStr;
//! assert_eq!(TaskStatus::from_str("pending").unwrap(), TaskStatus::Pending);
//!
//! // Variant slices and count (EnumVariants)
//! assert_eq!(
//!     TaskStatus::variants(),
//!     &[TaskStatus::Pending, TaskStatus::InProgress, TaskStatus::Completed]
//! );
//! assert_eq!(TaskStatus::variant_count(), 3);
//! ```
//!
//! ---
//!
//! ## Derive Macros & Traits
//!
//! `enuminfo` exports paired procedural derive macros and traits with the same names.
//! Deriving any macro automatically implements its associated trait for the enum.
//!
//! ### `EnumIs`
//!
//! Generates `pub const fn is_<variant_name>(&self) -> bool` methods in `snake_case` for every variant,
//! and implements the [`EnumIs`] marker trait.
//!
//! ```rust
//! use enuminfo::EnumIs;
//!
//! #[derive(EnumIs)]
//! enum Role {
//!     Admin,
//!     StandardUser,
//! }
//!
//! let role = Role::Admin;
//! assert!(role.is_admin());
//! assert!(!role.is_standard_user());
//! ```
//!
//! ### `EnumName`
//!
//! Implements [`EnumName`] and provides:
//! - `name(&self) -> &'static str`: Returns the formatted variant name (respecting `rename` and `rename_all`).
//! - `raw_name(&self) -> &'static str`: Returns the exact identifier name as written in Rust code.
//!
//! ```rust
//! use enuminfo::EnumName;
//!
//! #[derive(EnumName)]
//! #[enuminfo(rename_all = "kebab-case")]
//! enum Environment {
//!     LocalDevelopment,
//!     #[enuminfo(rename = "prod")]
//!     Production,
//! }
//!
//! let env = Environment::LocalDevelopment;
//! assert_eq!(env.name(), "local-development");
//! assert_eq!(env.raw_name(), "LocalDevelopment");
//!
//! let prod = Environment::Production;
//! assert_eq!(prod.name(), "prod");
//! assert_eq!(prod.raw_name(), "Production");
//! ```
//!
//! ### `EnumFromName`
//!
//! Implements [`EnumFromName`] for **unit enums** (variants without fields):
//! - `from_name(name: &str) -> Option<Self>`: Matches formatted names (respecting casing and renames).
//! - `from_raw_name(name: &str) -> Option<Self>`: Matches exact identifier names.
//! - When the `from-str` feature is enabled (default), also implements [`std::str::FromStr`].
//!
//! ```rust
//! use enuminfo::EnumFromName;
//! use std::str::FromStr;
//!
//! #[derive(Debug, PartialEq, EnumFromName)]
//! #[enuminfo(rename_all = "snake_case")]
//! enum Level {
//!     Debug,
//!     Info,
//!     #[enuminfo(ignore_from_name)]
//!     InternalHidden,
//! }
//!
//! assert_eq!(Level::from_name("debug"), Some(Level::Debug));
//! assert_eq!(Level::from_raw_name("Debug"), Some(Level::Debug));
//! assert_eq!(Level::from_name("internal_hidden"), None);
//!
//! // std::str::FromStr
//! assert_eq!(Level::from_str("info").unwrap(), Level::Info);
//! assert!(Level::from_str("unknown").is_err());
//! ```
//!
//! ### `EnumVariants`
//!
//! Implements [`EnumVariants`] for **unit enums**:
//! - `pub const VARIANTS: &'static [Self]`: Static slice of all non-ignored variants.
//! - `variants() -> &'static [Self]`: Accessor method for the slice.
//! - `variant_count() -> usize`: Number of variants in `VARIANTS`.
//!
//! ```rust
//! use enuminfo::EnumVariants;
//!
//! #[derive(Debug, PartialEq, EnumVariants)]
//! enum Direction {
//!     North,
//!     South,
//!     East,
//!     West,
//!     #[enuminfo(ignore_variants)]
//!     Unknown,
//! }
//!
//! assert_eq!(
//!     Direction::VARIANTS,
//!     &[Direction::North, Direction::South, Direction::East, Direction::West]
//! );
//! assert_eq!(Direction::variants(), Direction::VARIANTS);
//! assert_eq!(Direction::variant_count(), 4);
//! ```
//!
//! ### `EnumProperty`
//!
//! The [`EnumProperty`] trait provides convenient default methods to query custom typed properties
//! associated with an enum variant, returning `None` by default. Implementors can override specific
//! getters to return variant-specific metadata.
//!
//! ```rust
//! use enuminfo::EnumProperty;
//!
//! enum HttpMethod {
//!     Get,
//!     Post,
//! }
//!
//! impl EnumProperty for HttpMethod {
//!     fn get_string_property(&self, key: &str) -> Option<String> {
//!         match (self, key) {
//!             (HttpMethod::Get, "description") => Some("Retrieve data".into()),
//!             (HttpMethod::Post, "description") => Some("Submit data".into()),
//!             _ => None,
//!         }
//!     }
//! }
//!
//! let method = HttpMethod::Get;
//! assert_eq!(method.get_string_property("description"), Some("Retrieve data".into()));
//! assert_eq!(method.get_string_property("unknown"), None);
//! ```
//!
//! ---
//!
//! ## Attribute Reference
//!
//! | Attribute | Target | Description | Example |
//! |---|---|---|---|
//! | `#[enuminfo(rename_all = "...")]` | Enum | Applies a casing convention to all variants | `#[enuminfo(rename_all = "snake_case")]` |
//! | `#[enuminfo(rename = "...")]` | Variant | Overrides the formatted name for a variant | `#[enuminfo(rename = "custom_name")]` |
//! | `#[enuminfo(ignore_from_name)]` | Variant | Excludes a variant from `from_name()` lookup | `#[enuminfo(ignore_from_name)]` |
//! | `#[enuminfo(ignore_variants)]` | Variant | Excludes a variant from `VARIANTS` and `variants()` | `#[enuminfo(ignore_variants)]` |
//!
//! ---
//!
//! ## Casing Conventions (`rename_all`)
//!
//! The `rename_all` attribute supports standard `serde`-compatible casing strategies:
//!
//! | Value | Transformation | Example (`MyVariant`) |
//! |---|---|---|
//! | `"lowercase"` | All lower case | `"myvariant"` |
//! | `"UPPERCASE"` | All upper case | `"MYVARIANT"` |
//! | `"camelCase"` / `"lowerCamelCase"` | Lower camel case | `"myVariant"` |
//! | `"snake_case"` | Snake case | `"my_variant"` |
//! | `"SCREAMING-SNAKE-CASE"` | Screaming snake case | `"MY_SNAKE_CASE"` |
//! | `"kebab-case"` | Kebab case | `"my-variant"` |
//! | `"SCREAMING-KEBAB-CASE"` | Screaming kebab case | `"MY-KEBAB-CASE"` |
//! | `"PascalCase"` / `"UpperCamelCase"` | Pascal / Upper camel case | `"MyVariant"` |
//!
//! ---
//!
//! ## Feature Flags
//!
//! - **`from-str`** *(default)*: Generates `impl std::str::FromStr` for enums deriving [`EnumFromName`].
//!   The error type is [`error::EnumFromNameError`].
//! - **`skip-inherent`**: Omits inherent method definitions (`is_<variant>()`, `name()`, `from_name()`,
//!   `variants()`, etc.) on the enum itself, exposing functionality strictly through the traits. Useful
//!   when inherent methods would conflict with other methods defined on the enum.

mod enum_from_name;
mod enum_is;
mod enum_name;
mod enum_property;
mod enum_variants;

// Derive macros
pub use enuminfo_macros::EnumFromName;
pub use enuminfo_macros::EnumIs;
pub use enuminfo_macros::EnumName;
pub use enuminfo_macros::EnumVariants;

// Traits
/// Trait for constructing unit enum variants from string names.
pub use enum_from_name::EnumFromName;
/// Marker trait for enums supporting boolean variant predicate checks.
pub use enum_is::EnumIs;
/// Trait for inspecting formatted and raw string names of enum variants.
pub use enum_name::EnumName;
/// Trait providing dynamic typed property getters for enum variants.
pub use enum_property::EnumProperty;
/// Trait for accessing all variants of a unit enum as a static slice.
pub use enum_variants::EnumVariants;

/// Errors produced by `enuminfo` operations and trait implementations.
pub mod error {
    /// Error returned when parsing an enum from a string fails via [`std::str::FromStr`].
    pub use crate::enum_from_name::EnumFromNameError;
}
