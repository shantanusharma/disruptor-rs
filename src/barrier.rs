use crate::Sequence;

/// Indicates no sequence number has been claimed (yet).
pub const NONE: Sequence = -1;

#[doc(hidden)]
pub trait Barrier: Send + Sync {
	/// Gets the sequence number of the barrier with acquire memory ordering.
	/// `prev` must be the last sequence returned from this barrier.
	fn get_after(&self, prev: Sequence) -> Sequence;
}
