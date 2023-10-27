use arrayvec::ArrayVec;

#[derive(Debug, Clone)]
pub struct SortedArrayVec<V, const CAP: usize>(ArrayVec<V, CAP>);

pub trait SortedItemKey: Sized {
    type K: PartialOrd + PartialEq + Eq + Ord;
    fn key(&self) -> Self::K;
}

impl<V, const CAP: usize> Default for SortedArrayVec<V, CAP>
where
    V: SortedItemKey,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<V, const CAP: usize> SortedArrayVec<V, CAP>
where
    V: SortedItemKey,
{
    /// Binary searches the vec for the given Item Key
    /// If an item with the Key is found It returns Ok
    /// containing the first position of the item with the given Key
    /// Otherwise It returns the position where the item is supposed to be inserted
    fn binary_search_by_key(&self, key: &V::K) -> Result<usize, usize> {
        self.0.binary_search_by(|item| item.key().cmp(key))
    }

    ///  Finds a position by an Key
    fn position_by_key(&self, key: &V::K) -> Option<usize> {
        self.binary_search_by_key(key).ok()
    }

    /// Finds the first position by an Key
    fn first_position_by_key(&self, key: &V::K) -> Option<usize> {
        // Since the array is always sorted the partition point item.key < key
        // is the first index
        let ix = self.0.partition_point(|item| item.key() < *key);
        (ix < CAP).then_some(ix)
    }

    /// Finds an insert position for an Key
    fn find_insert_position_by_key(&self, key: &V::K) -> usize {
        self.binary_search_by_key(key).unwrap_or_else(|ix| ix)
    }

    /// Lenght of the Inventory
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks whether the inventory is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Checks full
    pub fn is_full(&self) -> bool {
        self.0.is_full()
    }

    /// Gets a ref Item by its index
    pub fn get(&self, ix: usize) -> &V {
        &self.0[ix]
    }

    /// Gets a mut ref Item by its index
    pub fn get_mut(&mut self, ix: usize) -> &mut V {
        &mut self.0[ix]
    }

    /// checks whether an Item with the given Key exists
    pub fn contains_key(&self, key: V::K) -> bool {
        self.position_by_key(&key).is_some()
    }

    /// Clears
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Adds an item to the vec
    pub fn add(&mut self, item: V) -> usize {
        self.try_add(item).map_err(|_| ()).expect("inventory add")
    }

    /// Attempts to add an item and return the inserted index, otherwise return the item back
    pub fn try_add(&mut self, item: V) -> Result<usize, V> {
        if self.is_full() {
            return Err(item);
        }

        let insert_ix = self.find_insert_position_by_key(&item.key());
        self.0.insert(insert_ix, item);
        Ok(insert_ix)
    }

    /// Removes an item by its index
    pub fn remove(&mut self, ix: usize) -> V {
        self.0.remove(ix)
    }

    /// Gets all items by key
    pub fn items_by_key(&self, key: V::K) -> impl Iterator<Item = &V> + '_ {
        let ix = self.first_position_by_key(&key).unwrap_or(0);
        self.0.iter().skip(ix).take_while(move |i| i.key() == key)
    }

    /// Gets all items mutable by key
    pub fn items_by_key_mut(&mut self, key: V::K) -> impl Iterator<Item = &mut V> + '_ {
        let ix = self.first_position_by_key(&key).unwrap_or(0);
        self.0
            .iter_mut()
            .skip(ix)
            .take_while(move |i| i.key() == key)
    }

    /// Gets all items by key
    pub fn enumerate_items_by_key(&self, key: V::K) -> impl Iterator<Item = (usize, &V)> + '_ {
        let ix = self.first_position_by_key(&key).unwrap_or(0);
        self.0
            .iter()
            .enumerate()
            .skip(ix)
            .take_while(move |i| i.1.key() == key)
    }

    /// Gets all items mutable by key
    pub fn enumerate_items_by_key_mut(
        &mut self,
        key: V::K,
    ) -> impl Iterator<Item = (usize, &mut V)> + '_ {
        let ix = self.first_position_by_key(&key).unwrap_or(0);
        self.0
            .iter_mut()
            .enumerate()
            .skip(ix)
            .take_while(move |i| i.1.key() == key)
    }

    /// Iterator over all items
    pub fn iter(&self) -> impl Iterator<Item = &V> + '_ {
        self.0.iter()
    }

    /// Mutable iterator over all items
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut V> + '_ {
        self.0.iter_mut()
    }

    /// Returns the index of the reference
    pub fn index_of(&self, item: &V) -> usize {
        let item_ptr = item as *const V as usize;
        let slice_ptr = self.0.as_ptr() as usize;
        (slice_ptr - item_ptr) / std::mem::size_of::<V>()
    }
}

impl<const CAP: usize, T> AsRef<[T]> for SortedArrayVec<T, CAP> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<V, const CAP: usize> FromIterator<V> for SortedArrayVec<V, CAP>
where
    V: SortedItemKey,
{
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        let mut items = Self::default();
        for item in iter {
            items.add(item);
        }
        items
    }
}
#[cfg(test)]
mod tests {
    use super::SortedArrayVec;

    // Test sorted item list
    const CAP: usize = 32;

    #[test]
    fn item_vec_insert_delete() {
        let range = 1..=CAP as u32;

        // Ascending order insert
        let mut items = SortedArrayVec::<u32, CAP>::from_iter(range.clone());
        //Check that items are in ascending sorted order
        itertools::assert_equal(items.iter().cloned(), range.clone());
        assert!(items.is_full());

        // Delete from start
        for i in range.clone() {
            assert_eq!(items.remove(0), i);
        }
        assert_eq!(items.len(), 0); 

        //Insert in reverse order
        let items = SortedArrayVec::<u32, CAP>::from_iter(range.clone().rev());
        //Check that items are in ascending sorted order
        itertools::assert_equal(items.iter().cloned(), range.clone());
    }

    #[test]
    fn item_get_by_id() {
        let mut items = SortedArrayVec::<u32, CAP>::default();
        // Add 2 ids for 1 to 15 and one 0 + one 16
        for i in 1..=CAP {
            items.try_add((i % 17) as u32).unwrap();
        }

        assert_eq!(items.items_by_key(17).count(), 0);
        assert_eq!(items.items_by_key(0).count(), 1);
        assert_eq!(items.items_by_key(16).count(), 1);

        for i in 1..=15 {
            items.contains_key(i);
            assert_eq!(items.items_by_key(i).count(), 2);
        }
    }
}
