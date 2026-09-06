macro_rules! get_x_property {
    ($( $(#[$doc:meta])* ($fn_name:ident, $type:ty) ),* $(,)?) => {
        $(
            $(#[$doc])*
            fn $fn_name(&self, key: &str) -> Option<$type> {
                let _ = key;
                None
            }
        )*
    };
}

/// Trait providing dynamic typed property getters for enum variants.
///
/// All methods provide default implementations returning `None`. Enums can implement this
/// trait and override specific getters to associate typed metadata with variants.
pub trait EnumProperty {
    get_x_property!(
        /// Retrieves a [`String`] property by key name, or `None` if not present.
        (get_string_property, String),
        /// Retrieves an `i8` property by key name, or `None` if not present.
        (get_i8_property, i8),
        /// Retrieves an `i16` property by key name, or `None` if not present.
        (get_i16_property, i16),
        /// Retrieves an `i32` property by key name, or `None` if not present.
        (get_i32_property, i32),
        /// Retrieves an `i64` property by key name, or `None` if not present.
        (get_i64_property, i64),
        /// Retrieves an `i128` property by key name, or `None` if not present.
        (get_i128_property, i128),
        /// Retrieves an `isize` property by key name, or `None` if not present.
        (get_isize_property, isize),
        /// Retrieves a `u8` property by key name, or `None` if not present.
        (get_u8_property, u8),
        /// Retrieves a `u16` property by key name, or `None` if not present.
        (get_u16_property, u16),
        /// Retrieves a `u32` property by key name, or `None` if not present.
        (get_u32_property, u32),
        /// Retrieves a `u64` property by key name, or `None` if not present.
        (get_u64_property, u64),
        /// Retrieves a `u128` property by key name, or `None` if not present.
        (get_u128_property, u128),
        /// Retrieves a `usize` property by key name, or `None` if not present.
        (get_usize_property, usize),
        /// Retrieves an `f32` property by key name, or `None` if not present.
        (get_f32_property, f32),
        /// Retrieves an `f64` property by key name, or `None` if not present.
        (get_f64_property, f64),
        /// Retrieves a [`std::time::SystemTime`] property by key name, or `None` if not present.
        (get_system_time_property, std::time::SystemTime),
    );
}
