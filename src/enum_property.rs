macro_rules! get_x_property {
    // Acepta una lista de pares: (nombre_funcion, tipo)
    ($( ($fn_name:ident, $type:ty) ),* $(,)?) => {
        $(
            fn $fn_name(&self, key: &str) -> Option<$type> {
                let _ = key;
                None
            }
        )*
    };
}

pub trait EnumProperty {
    // Invocación única para todos los enteros, unsigned y flotantes
    get_x_property!(
        // String
        (get_string_property, String),
        // Signed integers
        (get_i8_property, i8),
        (get_i16_property, i16),
        (get_i32_property, i32),
        (get_i64_property, i64),
        (get_i128_property, i128),
        (get_isize_property, isize),
        // Unsigned integers
        (get_u8_property, u8),
        (get_u16_property, u16),
        (get_u32_property, u32),
        (get_u64_property, u64),
        (get_u128_property, u128),
        (get_usize_property, usize),
        // Floats
        (get_f32_property, f32),
        (get_f64_property, f64),
        // Systen Time
        (get_system_time_property, std::time::SystemTime),
    );
}
