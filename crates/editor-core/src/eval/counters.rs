//! **What the evaluator built, counted on this thread** ([`EvalCounters`]).
//!
//! `cfg(debug_assertions)`-gated, as `crate::product::gathers_on_this_thread`
//! is and for the same reason: this workspace's release profile keeps
//! debug assertions, so every build this repo produces carries the
//! counter. A caller reads a DIFFERENCE across the evaluation it asks
//! about; the absolute values mean nothing.

#[cfg(debug_assertions)]
thread_local! {
    static RIGID_MAPS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static JUDGEMENTS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// One body the evaluator mapped by a rigid motion into another frame
/// ([`super::Framed`]'s reads through `relative`).
pub(crate) fn rigid_map() {
    #[cfg(debug_assertions)]
    RIGID_MAPS.with(|n| n.set(n.get().saturating_add(1)));
}

/// `n` pairwise judgements a union or an intersect ran over its
/// members.
pub(crate) fn judgements(n: usize) {
    #[cfg(debug_assertions)]
    JUDGEMENTS.with(|c| c.set(c.get().saturating_add(u64::try_from(n).unwrap_or(u64::MAX))));
    #[cfg(not(debug_assertions))]
    let _ = n;
}

/// **The evaluator's builds on this thread so far.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EvalCounters {
    /// Bodies mapped by a rigid motion: an operand read into another's
    /// frame. A copy maps none: it shares its source's body.
    pub rigid_maps: u64,
    /// Pairwise judgements of a union's or an intersect's members. A
    /// pair certified apart is not judged.
    pub judgements: u64,
}

impl EvalCounters {
    /// The counts on this thread now.
    #[cfg(debug_assertions)]
    #[must_use]
    pub fn on_this_thread() -> Self {
        Self {
            rigid_maps: RIGID_MAPS.with(std::cell::Cell::get),
            judgements: JUDGEMENTS.with(std::cell::Cell::get),
        }
    }

    /// What `self` counted since `before`.
    #[must_use]
    pub fn since(self, before: Self) -> Self {
        Self {
            rigid_maps: self.rigid_maps - before.rigid_maps,
            judgements: self.judgements - before.judgements,
        }
    }
}
