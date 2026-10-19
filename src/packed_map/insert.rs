use crate::{Id, PackedMap};

impl<T> PackedMap<T> {
    /// Insert a new element into this map
    pub fn insert(&mut self, value: T) -> Id<T> {
        let id = if self.ids.len() > self.values.len() {
            self.ids[self.values.len()].bump_generation();
            self.ids[self.values.len()]
        } else {
            let new_id = Id::new(self.values.len(), 0);
            self.ids.push(new_id);
            self.indices.push(new_id.index());
            new_id
        };

        self.indices[id.index()] = self.values.len();

        self.values.push(value);
        id
    }

    /// Insert a new element into the map with its ID before insertion
    pub fn insert_cyclic<F: FnOnce(Id<T>) -> T>(&mut self, f: F) -> Id<T> {
        let id = if self.ids.len() > self.values.len() {
            self.ids[self.values.len()].bump_generation();
            self.ids[self.values.len()]
        } else {
            let new_id = Id::new(self.values.len(), 0);
            self.ids.push(new_id);
            self.indices.push(new_id.index());
            new_id
        };

        self.indices[id.index()] = self.values.len();

        let value = f(id);
        self.values.push(value);
        id
    }
}
