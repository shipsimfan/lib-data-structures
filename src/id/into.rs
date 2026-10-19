use crate::Id;

const impl<T> Into<u64> for Id<T> {
    fn into(self) -> u64 {
        ((self.index as u64) << 32) | (self.generation as u64)
    }
}
