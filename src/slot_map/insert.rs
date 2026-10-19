use crate::{Id, SlotMap, slot_map::Slot};

impl<T> SlotMap<T> {
    /// Insert a new element into this map
    pub fn insert(&mut self, value: T) -> Id<T> {
        self.len += 1;
        match self.first_free {
            Some(index) => {
                let (generation, next_free) = self.slots[index].set(value);
                self.first_free = next_free;
                Id::new(index, generation)
            }
            None => {
                self.slots.push(Slot::Used {
                    generation: 0,
                    value,
                });
                Id::new(self.slots.len() - 1, 0)
            }
        }
    }

    /// Insert a new element into the map with its ID before insertion
    pub fn insert_cyclic<F: FnOnce(Id<T>) -> T>(&mut self, f: F) -> Id<T> {
        self.len += 1;
        match self.first_free {
            Some(index) => {
                let generation = match &self.slots[index] {
                    Slot::Free {
                        next_generation, ..
                    } => *next_generation,
                    Slot::Used { .. } => panic!("cannot set a used slot"),
                };

                let id = Id::new(index, generation);
                let value = f(id);

                let (_, next_free) = self.slots[index].set(value);
                self.first_free = next_free;
                id
            }
            None => {
                let id = Id::new(self.slots.len(), 0);
                let value = f(id);
                self.slots.push(Slot::Used {
                    generation: 0,
                    value,
                });
                id
            }
        }
    }
}
