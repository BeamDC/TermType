/// macro for converting `inches` to `millimetres`
#[macro_export]
macro_rules! inches {
    ($inches: expr) => {
        $inches as f32 * 25.4
    };
}