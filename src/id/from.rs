use crate::Id;
use std::marker::PhantomData;

impl<T> Id<T> {
    /// Creates an [`Id<T>`] from a `u64` value.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it assumes the `u64` value was created using the
    /// corresponding [`Into<u64>`] implementation for [`Id<T>`]. Using an arbitrary `u64` value
    /// may result in unpredictable behavior.
    pub unsafe fn from_u64(value: u64) -> Id<T> {
        Id {
            index: (value >> 32) as u32,
            generation: value as u32,
            _type: PhantomData,
        }
    }
}
