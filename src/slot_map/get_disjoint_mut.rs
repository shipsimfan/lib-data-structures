use crate::{Id, SlotMap, slot_map::Slot};
use std::{mem::MaybeUninit, slice::GetDisjointMutError};

impl<T> SlotMap<T> {
    /// Returns multiple mutable references to elements in the slot map with disjoint keys
    pub fn get_disjoint_mut<const N: usize>(
        &mut self,
        indices: [Id<T>; N],
    ) -> Result<[&mut T; N], GetDisjointMutError> {
        self.check_disjoint(&indices)?;

        unsafe { Ok(self.get_disjoint_unchecked_mut(indices)) }
    }

    /// Returns multiple mutable references to elements in the slot map with disjoint keys without
    /// checking for overlap
    pub unsafe fn get_disjoint_unchecked_mut<const N: usize>(
        &mut self,
        indices: [Id<T>; N],
    ) -> [&mut T; N] {
        let slice: *mut [Slot<T>] = self.slots.as_mut_slice();
        let mut arr: MaybeUninit<[&mut T; N]> = MaybeUninit::uninit();
        let arr_ptr = arr.as_mut_ptr();

        unsafe {
            for i in 0..N {
                let id = indices[i];
                arr_ptr.cast::<&mut T>().add(i).write(
                    &mut slice
                        .get_unchecked_mut(id.index())
                        .as_mut()
                        .unwrap()
                        .get_mut(id)
                        .unwrap(),
                );
            }
            arr.assume_init()
        }
    }

    /// Checks if the given indices are disjoint
    fn check_disjoint<const N: usize>(
        &self,
        indices: &[Id<T>; N],
    ) -> Result<(), GetDisjointMutError> {
        for (i, id) in indices.iter().enumerate() {
            if id.index() >= self.slots.len()
                || self.slots[id.index()].generation() != Some(id.generation())
            {
                return Err(GetDisjointMutError::IndexOutOfBounds);
            }

            for id2 in &indices[..i] {
                if id.index() == id2.index() {
                    return Err(GetDisjointMutError::OverlappingIndices);
                }
            }
        }

        Ok(())
    }
}
