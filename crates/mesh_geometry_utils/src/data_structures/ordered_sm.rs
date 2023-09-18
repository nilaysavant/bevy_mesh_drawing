use slotmap::{Key, SlotMap};

/// Ordered SlotMap.
///
/// SlotMap with items stored in order of insertion/push.
///
/// - Maintains a vec of item IDs stored separately.
/// - Allows intermediate insertions.
/// - Allows for changing of order if needed. (Yet to be implemented).
#[derive(Debug, Clone)]
pub struct OrderedSlotMap<Item, ItemId>
where
    Item: Default + Clone,
    ItemId: Key,
{
    ids: Vec<ItemId>,
    sm: SlotMap<ItemId, Item>,
}

impl<Item, ItemId> OrderedSlotMap<Item, ItemId>
where
    Item: Default + Clone,
    ItemId: Key,
{
    /// Create a new empty OrderedSlotMap.
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            sm: SlotMap::with_key(),
        }
    }

    /// Get ordered list of ids.
    pub fn ids(&self) -> &Vec<ItemId> {
        &self.ids
    }

    /// Reverse the order of items.
    pub fn reverse(&mut self) {
        self.ids.reverse();
    }

    /// Get the total number items stored.
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Push new item at the end.
    ///
    /// Returns the pushed item's id.
    pub fn push(&mut self, item: Item) -> ItemId {
        let id = self.sm.insert(item);
        self.ids.push(id);
        id
    }

    /// Push new item(s) at the end.
    ///
    /// Returns the pushed items' ids.
    pub fn push_many(&mut self, items_vec: Vec<Item>) -> Vec<ItemId> {
        let mut ids = vec![];
        for item in items_vec {
            let id = self.push(item);
            ids.push(id);
        }
        ids
    }

    /// Prepend new item at the beginning.
    ///
    /// Returns the prepended item's id.
    pub fn prepend(&mut self, item: Item) -> ItemId {
        let id = self.sm.insert(item);
        self.ids.splice(..0, [id]);
        id
    }

    /// Prepend new item(s) at the beginning.
    ///
    /// Returns the prepended items' ids.
    pub fn prepend_many(&mut self, items_vec: Vec<Item>) -> Vec<ItemId> {
        let mut ids = vec![];
        for item in items_vec {
            let id = self.sm.insert(item);
            ids.push(id);
        }
        self.ids.splice(..0, ids.clone());
        ids
    }

    /// Insert new item at given index.
    ///
    /// Returns the inserted item's id.
    pub fn insert(&mut self, index: usize, item: Item) -> ItemId {
        let id = self.sm.insert(item);
        self.ids.splice(index..index, [id]);
        id
    }

    /// Insert new item(s) at given index.
    ///
    /// Returns the inserted items' ids.
    pub fn insert_many(&mut self, index: usize, items_vec: Vec<Item>) -> Vec<ItemId> {
        let mut ids = vec![];
        for item in items_vec {
            let id = self.sm.insert(item);
            ids.push(id);
        }
        self.ids.splice(index..index, ids.clone());
        ids
    }

    /// Remove item by given id.
    ///
    /// Returns removed item.
    pub fn remove(&mut self, id: ItemId) -> Option<Item> {
        let Some(item) = self.sm.remove(id) else {
            return None;
        };
        if let Some(index) = self.ids.iter().position(|tmp_id| *tmp_id == id) {
            self.ids.remove(index);
        }
        Some(item)
    }

    /// Remove item(s) by given ids.
    ///
    /// Returns removed item(s).
    pub fn remove_many(&mut self, ids: Vec<ItemId>) -> Vec<Item> {
        let mut removed = vec![];
        for id in ids {
            if let Some(item) = self.remove(id) {
                removed.push(item);
            }
        }
        removed
    }

    /// Clear all item(s).
    ///
    /// Returns clear count.
    ///
    /// PS: Does not reset the internal id generator! Use [clear_with_reset] instead.
    pub fn clear(&mut self) -> usize {
        self.sm.clear(); // clear hashmap
        let remove_count = self.ids.len();
        self.ids.clear(); // clear ids
        remove_count
    }

    /// Clears all item(s) and resets the internal slot-map.
    pub fn clear_with_reset(&mut self) -> usize {
        let remove_count = self.clear();
        self.sm = SlotMap::with_key();
        remove_count
    }

    /// Get item by ID.
    pub fn get(&self, id: ItemId) -> Option<&Item> {
        self.sm.get(id)
    }

    /// Get mutable item by ID.
    pub fn get_mut(&mut self, id: ItemId) -> Option<&mut Item> {
        self.sm.get_mut(id)
    }

    /// Get item by ID (owned).
    pub fn get_owned(&self, id: ItemId) -> Option<Item> {
        if let Some(item) = self.sm.get(id) {
            return Some(item.clone());
        }
        None
    }

    /// Get all item(s) in order.
    pub fn get_all(&self) -> Vec<&Item> {
        let mut items_vec = vec![];
        for id in self.ids.iter() {
            if let Some(item) = self.get(*id) {
                items_vec.push(item);
            }
        }
        items_vec
    }

    /// Get all (owned) items(s) in order.
    pub fn get_all_owned(&self) -> Vec<Item> {
        let mut items_vec = vec![];
        for id in self.ids.iter() {
            if let Some(item) = self.get_owned(*id) {
                items_vec.push(item);
            }
        }
        items_vec
    }

    /// Get first item.
    pub fn first(&self) -> Option<&Item> {
        let Some(id) = self.ids.first() else {
            return None;
        };
        self.sm.get(*id)
    }

    /// Get first item mutably.
    pub fn first_mut(&mut self) -> Option<&mut Item> {
        let Some(id) = self.ids.first() else {
            return None;
        };
        self.sm.get_mut(*id)
    }

    /// Get last item.
    pub fn last(&self) -> Option<&Item> {
        let Some(id) = self.ids.last() else {
            return None;
        };
        self.sm.get(*id)
    }

    /// Get last item mutably.
    pub fn last_mut(&mut self) -> Option<&mut Item> {
        let Some(id) = self.ids.last() else {
            return None;
        };
        self.sm.get_mut(*id)
    }

    /// Returns iterator to enumerate over
    /// all items as (key: ItemId, Item) pairs.
    pub fn enumerate(&self) -> OrderedSlotMapEnumerateIter<'_, Item, ItemId> {
        OrderedSlotMapEnumerateIter {
            ordered_sm: self,
            current_index: 0,
        }
    }

    /// Returns an iterator over item(s).
    pub fn iter(&self) -> OrderedSlotMapIterator<'_, Item, ItemId> {
        OrderedSlotMapIterator {
            ordered_sm: self,
            current_index: 0,
        }
    }
}

impl<Item, ItemId> Default for OrderedSlotMap<Item, ItemId>
where
    Item: Default + Clone,
    ItemId: Key,
{
    /// Ordered SlotMap with defaults.
    fn default() -> Self {
        Self::new()
    }
}

impl<Item, ItemId> From<Vec<Item>> for OrderedSlotMap<Item, ItemId>
where
    Item: Default + Clone,
    ItemId: Key,
{
    /// Convert from vector to Ordered SlotMap.
    fn from(items_vec: Vec<Item>) -> Self {
        let mut ordered_sm = Self::new();
        // Add items
        ordered_sm.push_many(items_vec);
        ordered_sm
    }
}

/// Iterator over Ordered SlotMap.
#[derive(Debug)]
pub struct OrderedSlotMapIterator<'a, Item, ItemId>
where
    Item: Default + Clone,
    ItemId: Key,
{
    ordered_sm: &'a OrderedSlotMap<Item, ItemId>,
    current_index: usize,
}

impl<'a, Item, ItemId> Iterator for OrderedSlotMapIterator<'a, Item, ItemId>
where
    Item: Default + Clone,
    ItemId: Key,
{
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(id) = self.ordered_sm.ids.get(self.current_index) else {
            return None;
        };
        self.current_index += 1;
        self.ordered_sm.get(*id)
    }
}

impl<'a, Item, ItemId> IntoIterator for &'a OrderedSlotMap<Item, ItemId>
where
    Item: Default + Clone,
    ItemId: Key,
{
    type Item = &'a Item;

    type IntoIter = OrderedSlotMapIterator<'a, Item, ItemId>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over Ordered SM enumerated with ids.
#[derive(Debug)]
pub struct OrderedSlotMapEnumerateIter<'a, Item, ItemId>
where
    Item: Default + Clone,
    ItemId: Key,
{
    ordered_sm: &'a OrderedSlotMap<Item, ItemId>,
    current_index: usize,
}

impl<'a, Item, ItemId> Iterator for OrderedSlotMapEnumerateIter<'a, Item, ItemId>
where
    Item: Default + Clone,
    ItemId: Key,
{
    type Item = (ItemId, &'a Item);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(id) = self.ordered_sm.ids.get(self.current_index) else {
            return None;
        };
        self.current_index += 1;
        let Some(item) = self.ordered_sm.get(*id) else {
            return None;
        };
        Some((*id, item))
    }
}

/// Test Ordered SM borrow iterator.
#[test]
fn test_ordered_sm_borrow_iter() {
    use bevy::prelude::*;
    use slotmap::new_key_type;

    let vec_of_vec2 = vec![
        Vec2::splat(0.),
        Vec2::splat(1.),
        Vec2::splat(2.),
        Vec2::splat(3.),
    ];
    new_key_type! {
        /// ItemId used to designate items for this OrderedSlotMap.
        pub struct ItemId;
    }
    let ordered_sm = OrderedSlotMap::<_, ItemId>::from(vec_of_vec2.clone());
    let collected = ordered_sm.iter().copied().collect::<Vec<_>>();

    assert_eq!(vec_of_vec2, collected);
}

/// Test Ordered SM into iterator.
#[test]
fn test_ordered_sm_into_iter() {
    use bevy::prelude::*;
    use slotmap::new_key_type;

    let vec_of_vec2 = vec![
        Vec2::splat(0.),
        Vec2::splat(1.),
        Vec2::splat(2.),
        Vec2::splat(3.),
    ];
    new_key_type! {
        /// ItemId used to designate items for this OrderedSlotMap.
        pub struct ItemId;
    }
    let ordered_sm = OrderedSlotMap::<_, ItemId>::from(vec_of_vec2.clone());
    let mut collected = vec![];
    for item in &ordered_sm {
        collected.push(*item);
    }
    assert_eq!(vec_of_vec2, collected);
}

/// Test Ordered SM enumerate iterator.
#[test]
fn test_ordered_sm_enumerate_iter() {
    use bevy::prelude::*;
    use slotmap::new_key_type;

    let vec_of_vec2 = vec![
        Vec2::splat(0.),
        Vec2::splat(1.),
        Vec2::splat(2.),
        Vec2::splat(3.),
    ];
    new_key_type! {
        /// ItemId used to designate items for this OrderedSlotMap.
        pub struct ItemId;
    }
    let ordered_sm = OrderedSlotMap::<_, ItemId>::from(vec_of_vec2.clone());
    let mut collected = vec![];
    for (_, item) in ordered_sm.enumerate() {
        collected.push(*item);
    }
    assert_eq!(vec_of_vec2, collected);
}

/// Test Ordered SM insert.
#[test]
fn test_ordered_sm_insert_many() {
    use bevy::prelude::*;
    use slotmap::new_key_type;

    let vec_of_vec2 = vec![
        Vec2::splat(0.),
        Vec2::splat(1.),
        Vec2::splat(2.),
        Vec2::splat(3.),
    ];
    new_key_type! {
        /// ItemId used to designate items for this OrderedSlotMap.
        pub struct ItemId;
    }
    let mut ordered_sm = OrderedSlotMap::<_, ItemId>::from(vec_of_vec2.clone());
    ordered_sm.insert_many(2, vec![Vec2::splat(10.), Vec2::splat(11.)]);
    let mut collected = vec![];
    for (_, item) in ordered_sm.enumerate() {
        collected.push(*item);
    }
    assert_eq!(
        vec![
            Vec2::splat(0.),
            Vec2::splat(1.),
            Vec2::splat(10.),
            Vec2::splat(11.),
            Vec2::splat(2.),
            Vec2::splat(3.),
        ],
        collected
    );
}
