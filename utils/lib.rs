pub mod maths;
pub mod utils;

#[derive(Debug)]
pub struct NullMutPtr<T>(pub *mut T);

impl<T> Default for NullMutPtr<T> {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

#[derive(Debug)]
pub struct NullPtr<T>(pub *const T);

impl<T> Default for NullPtr<T> {
    fn default() -> Self {
        Self(std::ptr::null())
    }
}
