use crate::component::Component;
use crate::entity_id::EntityId;
use crate::seal::Sealed;
use crate::sparse_set::SparseSet;
use crate::track::InsertionAndRemoval;
use crate::tracking::{
    map_deletion_data, InsertionTracking, RemovalOrDeletionTracking, RemovalTracking, Tracking,
    TrackingTimestamp,
};

impl Sealed for InsertionAndRemoval {}

impl Tracking for InsertionAndRemoval {
    const VALUE: u32 = 0b1001;

    fn name() -> &'static str {
        "Insertion and Removal"
    }

    #[inline]
    fn is_inserted<T: Component>(
        sparse_set: &SparseSet<T>,
        entity: EntityId,
        last: TrackingTimestamp,
        current: TrackingTimestamp,
    ) -> bool {
        if let Some(dense) = sparse_set.index_of(entity) {
            sparse_set.insertion_data[dense].is_within(last, current)
        } else {
            false
        }
    }

    fn is_removed<T: Component>(
        sparse_set: &SparseSet<T>,
        entity: EntityId,
        last: TrackingTimestamp,
        current: TrackingTimestamp,
    ) -> bool {
        sparse_set
            .removal_data
            .iter()
            .any(|(id, timestamp)| *id == entity && timestamp.is_within(last, current))
    }
}

impl InsertionTracking for InsertionAndRemoval {}
impl RemovalTracking for InsertionAndRemoval {}
impl RemovalOrDeletionTracking for InsertionAndRemoval {
    #[allow(trivial_casts)]
    fn removed_or_deleted<T: Component>(
        sparse_set: &SparseSet<T>,
    ) -> core::iter::Chain<
        core::iter::Map<
            core::slice::Iter<'_, (EntityId, TrackingTimestamp, T)>,
            for<'r> fn(&'r (EntityId, TrackingTimestamp, T)) -> (EntityId, TrackingTimestamp),
        >,
        core::iter::Copied<core::slice::Iter<'_, (EntityId, TrackingTimestamp)>>,
    > {
        [].iter()
            .map(map_deletion_data as _)
            .chain(sparse_set.removal_data.iter().copied())
    }

    fn clear_all_removed_and_deleted<T: Component>(sparse_set: &mut SparseSet<T>) {
        sparse_set.removal_data.clear();
    }

    fn clear_all_removed_and_deleted_older_than_timestamp<T: Component>(
        sparse_set: &mut SparseSet<T>,
        timestamp: TrackingTimestamp,
    ) {
        sparse_set
            .removal_data
            .retain(|(_, t)| timestamp.is_older_than(*t));
    }
}
