//! **The trace one sweep keeps**: the pairs it examined and the ones it
//! accepted, each in the sweep's own order — the currency of the
//! idealized/realized differential suites (PERF-PLAN §4.4), and the
//! home of the comparators they share. The census
//! ([`crate::CensusTrace`], one per pass, pairs `(EntityId, EntityId)`)
//! and the boolean reduction sweep (one per direction, pairs
//! `(edge of x, face of y)`) both write one; neither hosts it.

/// One sweep's examined and accepted pairs. A census pass accepts a
/// pair it pushed a finding for; the boolean reduction sweep accepts a
/// pair where its exact predicates accepted an event (a crossing inside
/// the face, or an endpoint contact). Realized `examined` must hold
/// every idealized `accepted` pair (the conservative-superset pin).
///
/// The writers keep `accepted` a subsequence of `examined` by
/// construction: a sweep accepts only the pair it examined last, and
/// that examination at most once.
#[derive(Clone, Debug)]
pub struct SweepPairs<P> {
    examined: Vec<P>,
    accepted: Vec<P>,
    /// Whether the last examination is already accepted.
    last_accepted: bool,
}

impl<P> Default for SweepPairs<P> {
    fn default() -> Self {
        Self {
            examined: Vec::new(),
            accepted: Vec::new(),
            last_accepted: false,
        }
    }
}

impl<P> SweepPairs<P> {
    /// Every pair the sweep examined, in decision order.
    pub fn examined(&self) -> &[P] {
        &self.examined
    }

    /// The examined pairs the sweep accepted, in decision order.
    pub fn accepted(&self) -> &[P] {
        &self.accepted
    }
}

impl<P: Copy> SweepPairs<P> {
    /// Records `pair` as examined: its exact classification runs.
    pub(crate) fn examine(&mut self, pair: P) {
        self.examined.push(pair);
        self.last_accepted = false;
    }

    /// Accepts the pair examined last; a second accept of one
    /// examination records nothing more.
    #[track_caller]
    pub(crate) fn accept_last(&mut self) {
        let Some(&pair) = self.examined.last() else {
            unreachable!(
                "a sweep trace accepted a pair before examining any: every writer accepts \
                 the pair it is examining"
            )
        };
        if !self.last_accepted {
            self.accepted.push(pair);
            self.last_accepted = true;
        }
    }

    /// [`SweepPairs::examine`], then [`SweepPairs::accept_last`] where
    /// `accepted`: a sweep that decides a pair in one step.
    pub(crate) fn note(&mut self, pair: P, accepted: bool) {
        self.examine(pair);
        if accepted {
            self.accept_last();
        }
    }
}

#[cfg(feature = "sweep-testing")]
impl<P: Copy + Ord> SweepPairs<P> {
    /// Whether this sweep's examined sequence is `exact`'s with pairs
    /// removed and nothing reordered — the order pin of the
    /// differential suites.
    pub fn is_restriction_of(&self, exact: &Self) -> bool {
        let mut it = exact.examined.iter();
        self.examined.iter().all(|p| it.any(|q| q == p))
    }

    /// `exact`'s accepted pairs this sweep never examined — any entry
    /// is a finding the filter lost (the superset pin).
    pub fn lost_accepted(&self, exact: &Self) -> Vec<P> {
        let examined: std::collections::BTreeSet<P> = self.examined.iter().copied().collect();
        exact
            .accepted
            .iter()
            .filter(|p| !examined.contains(p))
            .copied()
            .collect()
    }
}
