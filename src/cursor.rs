use crossbeam_utils::CachePadded;
use std::sync::atomic::{AtomicI64, Ordering};

use crate::{Sequence, barrier::NONE};

pub(crate) struct Cursor {
	counter: CachePadded<AtomicI64>
}

impl Cursor {
	pub(crate) fn new() -> Self {
		Self {
			counter: CachePadded::new(AtomicI64::new(NONE))
		}
	}

	#[inline]
	pub(crate) fn compare_exchange_weak(&self, current: Sequence, next: Sequence) -> Result<i64, i64> {
		self.counter.compare_exchange_weak(current, next, Ordering::AcqRel, Ordering::Relaxed)
	}

	/// Stores `sequence` to the cursor with `Ordering::Release` semantics.
	#[inline]
	pub(crate) fn store(&self, sequence: Sequence) {
		self.counter.store(sequence, Ordering::Release);
	}

	/// Retrieves the cursor value with `Ordering::Acquire` semantics.
	#[inline]
	pub(crate) fn load(&self) -> Sequence {
		self.counter.load(Ordering::Acquire)
	}
}
