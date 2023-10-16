/// Interpret 1d vector/array as a 2d matrix
pub trait MatrixSlice2d<T> {
    /// Interpret slice as a 2d coordinate system, returning a specific pixel
    fn interpret_position(&self, x: u32, y: u32, w: u32, h: u32) -> T;

    /// Interpret slice as 2d matrix, then interpret that matrix geometrically in 2d
    /// space. Rotate matrix geometrically, then superimpose both matrices on each
    /// other, taking the weighted average of their intersections.
    fn rotate(&self, theta: f64) -> Self;
}
#[macro_export]
macro_rules! matrix_slice_2d_impl {
    ($typ:ident) => {
        impl<T> $crate::misc::linear_algebra::MatrixSlice2d<$typ<T>> for Vec<$typ<T>>
        where
            T: Copy,
        {
            fn interpret_position(&self, x: u32, y: u32, w: u32, h: u32) -> $typ<T> {
                let total = w * h;
                let des_array_member = y * w + x;

                if des_array_member < total {
                    return self[des_array_member as usize];
                }

                self[0]
            }
            fn rotate(&self, _theta: f64) -> Self {
                todo!()
            }
        }
    };
}
