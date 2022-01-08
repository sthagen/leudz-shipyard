use crate::not::Not;
use crate::{Component, EntityId, View, ViewMut};

/// Wrapper type allowing iterating over *inserted* flagged components.
#[derive(Clone)]
pub struct Inserted<Storage>(pub(crate) Storage);

impl<Storage> core::ops::Not for Inserted<Storage> {
    type Output = Not<Inserted<Storage>>;

    fn not(self) -> Self::Output {
        Not(self)
    }
}

impl<'a, T: Component> Inserted<View<'a, T>> {
    pub fn inserted(&self) -> Inserted<&View<'a, T>> {
        Inserted(&self.0)
    }
}

impl<'a, T: Component> core::ops::Deref for Inserted<View<'a, T>> {
    type Target = View<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Component> Inserted<ViewMut<'a, T>> {
    pub fn inserted(&self) -> Inserted<&ViewMut<'a, T>> {
        Inserted(&self.0)
    }
    pub fn inserted_mut(&mut self) -> Inserted<&mut ViewMut<'a, T>> {
        Inserted(&mut self.0)
    }
    #[inline]
    pub fn clear_all_inserted(self) {
        self.0.sparse_set.private_clear_all_inserted(self.current);
    }
}

impl<'a, T: Component> core::ops::Deref for Inserted<ViewMut<'a, T>> {
    type Target = ViewMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Component> core::ops::DerefMut for Inserted<ViewMut<'a, T>> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Wrapper type allowing iterating over *modified* flagged components.
#[derive(Clone)]
pub struct Modified<Storage>(pub(crate) Storage);

impl<Storage> core::ops::Not for Modified<Storage> {
    type Output = Not<Modified<Storage>>;

    fn not(self) -> Self::Output {
        Not(self)
    }
}

impl<'a, T: Component> Modified<View<'a, T>> {
    pub fn modified(&self) -> Modified<&View<'a, T>> {
        Modified(&self.0)
    }
}

impl<'a, T: Component> core::ops::Deref for Modified<View<'a, T>> {
    type Target = View<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Component> Modified<ViewMut<'a, T>> {
    pub fn modified(&self) -> Modified<&ViewMut<'a, T>> {
        Modified(&self.0)
    }
    pub fn modified_mut(&mut self) -> Modified<&mut ViewMut<'a, T>> {
        Modified(&mut self.0)
    }
    #[inline]
    pub fn clear_all_modified(self) {
        self.0.sparse_set.private_clear_all_modified(self.current);
    }
}

impl<'a, T: Component> core::ops::Deref for Modified<ViewMut<'a, T>> {
    type Target = ViewMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Component> core::ops::DerefMut for Modified<ViewMut<'a, T>> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Wrapper type allowing iterating over *inserted* and *modified* flagged components.
#[derive(Clone)]
pub struct InsertedOrModified<Storage>(pub(crate) Storage);

impl<Storage> core::ops::Not for InsertedOrModified<Storage> {
    type Output = Not<InsertedOrModified<Storage>>;

    fn not(self) -> Self::Output {
        Not(self)
    }
}

impl<'a, T: Component> InsertedOrModified<View<'a, T>> {
    pub fn inserted(&self) -> Inserted<&View<'a, T>> {
        Inserted(&self.0)
    }
    pub fn modified(&self) -> Modified<&View<'a, T>> {
        Modified(&self.0)
    }
    pub fn inserted_or_modified(&self) -> InsertedOrModified<&View<'a, T>> {
        InsertedOrModified(&self.0)
    }
}

impl<'a, T: Component> core::ops::Deref for InsertedOrModified<View<'a, T>> {
    type Target = View<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Component> InsertedOrModified<ViewMut<'a, T>> {
    pub fn inserted(&self) -> Inserted<&ViewMut<'a, T>> {
        Inserted(&self.0)
    }
    pub fn inserted_mut(&mut self) -> Inserted<&mut ViewMut<'a, T>> {
        Inserted(&mut self.0)
    }
    pub fn modified(&self) -> Modified<&ViewMut<'a, T>> {
        Modified(&self.0)
    }
    pub fn modified_mut(&mut self) -> Modified<&mut ViewMut<'a, T>> {
        Modified(&mut self.0)
    }
    pub fn inserted_or_modified(&self) -> InsertedOrModified<&ViewMut<'a, T>> {
        InsertedOrModified(&self.0)
    }
    pub fn inserted_or_modified_mut(&mut self) -> InsertedOrModified<&mut ViewMut<'a, T>> {
        InsertedOrModified(&mut self.0)
    }
    #[inline]
    pub fn clear_all_inserted(self) {
        self.0.sparse_set.private_clear_all_inserted(self.current);
    }
    #[inline]
    pub fn clear_all_modified(self) {
        self.0.sparse_set.private_clear_all_modified(self.current);
    }
    #[inline]
    pub fn clear_all_inserted_and_modified(self) {
        self.0
            .sparse_set
            .private_clear_all_inserted_and_modified(self.current);
    }
}

impl<'a, T: Component> core::ops::Deref for InsertedOrModified<ViewMut<'a, T>> {
    type Target = ViewMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Component> core::ops::DerefMut for InsertedOrModified<ViewMut<'a, T>> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone)]
pub struct All<Storage>(pub(crate) Storage);

impl<'a, T: Component> All<View<'a, T>> {
    pub fn inserted(&self) -> Inserted<&View<'a, T>> {
        Inserted(&self.0)
    }
    pub fn modified(&self) -> Modified<&View<'a, T>> {
        Modified(&self.0)
    }
    pub fn inserted_or_modified(&self) -> InsertedOrModified<&View<'a, T>> {
        InsertedOrModified(&self.0)
    }
    /// Returns the *deleted* components of a storage tracking deletion.
    pub fn deleted(&self) -> impl Iterator<Item = (EntityId, &T)> + '_ {
        self.sparse_set
            .deletion_data
            .iter()
            .filter_map(move |(entity, timestamp, component)| {
                if is_track_within_bounds(*timestamp, self.last_removal_or_deletion, self.current) {
                    Some((*entity, component))
                } else {
                    None
                }
            })
    }
    /// Returns the ids of *removed* components of a storage tracking removal.
    pub fn removed(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.sparse_set
            .removal_data
            .iter()
            .filter_map(move |(entity, timestamp)| {
                if is_track_within_bounds(*timestamp, self.last_removal_or_deletion, self.current) {
                    Some(*entity)
                } else {
                    None
                }
            })
    }
    /// Returns the ids of *removed* or *deleted* components of a storage tracking removal and/or deletion.
    pub fn removed_or_deleted(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.sparse_set
            .deletion_data
            .iter()
            .filter_map(move |(entity, timestamp, _)| {
                if is_track_within_bounds(*timestamp, self.last_removal_or_deletion, self.current) {
                    Some(*entity)
                } else {
                    None
                }
            })
            .chain(
                self.sparse_set
                    .removal_data
                    .iter()
                    .filter_map(move |(entity, timestamp)| {
                        if is_track_within_bounds(*timestamp, self.last_insert, self.current) {
                            Some(*entity)
                        } else {
                            None
                        }
                    }),
            )
    }
}

impl<'a, T: Component> core::ops::Deref for All<View<'a, T>> {
    type Target = View<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Component> All<ViewMut<'a, T>> {
    pub fn inserted(&self) -> Inserted<&ViewMut<'a, T>> {
        Inserted(&self.0)
    }
    pub fn inserted_mut(&mut self) -> Inserted<&mut ViewMut<'a, T>> {
        Inserted(&mut self.0)
    }
    pub fn modified(&self) -> Modified<&ViewMut<'a, T>> {
        Modified(&self.0)
    }
    pub fn modified_mut(&mut self) -> Modified<&mut ViewMut<'a, T>> {
        Modified(&mut self.0)
    }
    pub fn inserted_or_modified(&self) -> InsertedOrModified<&ViewMut<'a, T>> {
        InsertedOrModified(&self.0)
    }
    pub fn inserted_or_modified_mut(&mut self) -> InsertedOrModified<&mut ViewMut<'a, T>> {
        InsertedOrModified(&mut self.0)
    }
    #[inline]
    pub fn clear_all_inserted(self) {
        self.0.sparse_set.private_clear_all_inserted(self.current);
    }
    #[inline]
    pub fn clear_all_modified(self) {
        self.0.sparse_set.private_clear_all_modified(self.current);
    }
    #[inline]
    pub fn clear_all_inserted_and_modified(self) {
        self.0
            .sparse_set
            .private_clear_all_inserted_and_modified(self.current);
    }
    /// Returns the *deleted* components of a storage tracking deletion.
    pub fn deleted(&self) -> impl Iterator<Item = (EntityId, &T)> + '_ {
        self.sparse_set
            .deletion_data
            .iter()
            .filter_map(move |(entity, timestamp, component)| {
                if is_track_within_bounds(*timestamp, self.last_removal_or_deletion, self.current) {
                    Some((*entity, component))
                } else {
                    None
                }
            })
    }
    /// Returns the ids of *removed* components of a storage tracking removal.
    pub fn removed(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.sparse_set
            .removal_data
            .iter()
            .filter_map(move |(entity, timestamp)| {
                if is_track_within_bounds(*timestamp, self.last_removal_or_deletion, self.current) {
                    Some(*entity)
                } else {
                    None
                }
            })
    }
    /// Returns the ids of *removed* or *deleted* components of a storage tracking removal and/or deletion.
    pub fn removed_or_deleted(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.sparse_set
            .deletion_data
            .iter()
            .filter_map(move |(entity, timestamp, _)| {
                if is_track_within_bounds(*timestamp, self.last_removal_or_deletion, self.current) {
                    Some(*entity)
                } else {
                    None
                }
            })
            .chain(
                self.sparse_set
                    .removal_data
                    .iter()
                    .filter_map(move |(entity, timestamp)| {
                        if is_track_within_bounds(*timestamp, self.last_insert, self.current) {
                            Some(*entity)
                        } else {
                            None
                        }
                    }),
            )
    }
}

impl<'a, T: Component> core::ops::Deref for All<ViewMut<'a, T>> {
    type Target = ViewMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Component> core::ops::DerefMut for All<ViewMut<'a, T>> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[inline]
pub(crate) fn is_track_within_bounds(timestamp: u32, last: u32, current: u32) -> bool {
    let more_than_last = if timestamp < last {
        u32::MAX - last + timestamp
    } else {
        timestamp - last
    };
    let less_than_current = if current < timestamp {
        u32::MAX - timestamp + current
    } else {
        current - timestamp
    };

    more_than_last < u32::MAX / 2 && more_than_last > 0 && less_than_current < u32::MAX / 2
}
