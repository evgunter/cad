//! **The stop rule of a certify-and-refine loop.** A loop that refines
//! until a certified bound clears its target stops, short of that, on
//! one of two questions: is refinement still paying, and is the work
//! bounded? Every such loop in this crate answers both here, and keeps
//! its own body, its own units and its own action on an idle round:
//! the offset fit ([`crate::offset_fit`]) and limb 2's hull subdivision
//! (`ssi::certify::subdivide`).
//!
//! - **Paying** is measured against the LOWEST certified bound any
//!   round reached ([`Descent`]): a round that does not lower it is
//!   idle, and the loop stalls on its [`PATIENCE`]th idle round in a
//!   row. Before any round's bound is finite no round is idle, since a
//!   bound falling from `+∞` is converging.
//! - **Work** has two bounds, each in the loop's own units: a cap on
//!   the object one round builds (a grid's sample count, a composite's
//!   cuts), and a cumulative budget on all rounds together
//!   ([`Work`]), which bounds the wait where the cap bounds only the
//!   size.
//! - **Precedence** ([`first_stop`]): where a stall and a wall both end
//!   one round, the stall speaks. It is the finding about the request;
//!   a wall says only that the loop ran out before learning more.

/// How many idle rounds in a row stop a loop: it stops on the second,
/// so one is let pass.
///
/// One is let pass because a single round that does not lower the
/// lowest bound says nothing. Near the arithmetic's floor every round
/// rebuilds its object afresh and the rounding moves the bound either
/// way, and a refinement whose prediction was wrong for one round (the
/// offset fit's direction rule) is corrected by the round after it,
/// which is each loop's own idle-1 action. Two in a row is the
/// evidence that refinement has stopped paying: stopping on one would
/// refuse loops that are still converging, and waiting for more pays a
/// whole round each time for what the lowest bound already reports.
pub(crate) const PATIENCE: usize = 2;

/// The lowest certified bound a loop's rounds reached, what was
/// reached on it, and how many rounds in a row have not lowered it.
#[derive(Debug)]
pub(crate) struct Descent<T> {
    lowest: Option<(f64, T)>,
    idle: usize,
}

impl<T> Descent<T> {
    /// A loop before its first round.
    pub(crate) const fn new() -> Self {
        Self {
            lowest: None,
            idle: 0,
        }
    }

    /// Records one round's `bound`, with `at` what it was reached on,
    /// and returns how many rounds in a row have now not lowered the
    /// lowest finite bound: 0 on a round that lowered it, and on every
    /// round while no finite bound has been reached.
    ///
    /// A round lowers the lowest bound when it is strictly smaller, so
    /// an equal bound is idle and the older `at` is kept (the first
    /// round to reach the minimum). A NaN or `+∞` bound never lowers a
    /// finite one; a finite bound always lowers a non-finite one.
    pub(crate) fn observe(&mut self, bound: f64, at: T) -> usize {
        match &self.lowest {
            Some((low, _)) if low.is_finite() => {
                if bound < *low {
                    self.lowest = Some((bound, at));
                    self.idle = 0;
                } else {
                    self.idle += 1;
                }
            }
            Some((_, _)) if !bound.is_finite() => {}
            _ => self.lowest = Some((bound, at)),
        }
        self.idle
    }

    /// Whether the last [`PATIENCE`] rounds have all been idle.
    pub(crate) fn stalled(&self) -> bool {
        self.idle >= PATIENCE
    }

    /// The lowest bound reached and what it was reached on: the first
    /// round's where no round's bound was finite.
    pub(crate) fn lowest(&self) -> Option<(f64, &T)> {
        self.lowest.as_ref().map(|(b, at)| (*b, at))
    }

    /// [`Self::lowest`], by value.
    pub(crate) fn into_lowest(self) -> Option<(f64, T)> {
        self.lowest
    }
}

/// A loop's cumulative work budget, in the loop's own units, spent
/// round by round. A round is admitted only if its cost fits in what is
/// left, so the budget is never passed.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Work {
    budget: usize,
    spent: usize,
}

impl Work {
    /// A budget of `budget` units, none spent.
    pub(crate) const fn new(budget: usize) -> Self {
        Self { budget, spent: 0 }
    }

    /// Whether a round costing `cost` fits in what is left.
    pub(crate) fn admits(&self, cost: usize) -> bool {
        self.spent.saturating_add(cost) <= self.budget
    }

    /// Spends `cost`.
    pub(crate) fn spend(&mut self, cost: usize) {
        self.spent = self.spent.saturating_add(cost);
    }

    /// What has been spent.
    pub(crate) fn spent(&self) -> usize {
        self.spent
    }
}

/// Why a loop stopped short of its target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Stop<W> {
    /// Refinement stopped paying.
    Stall,
    /// A work bound, the loop's own `W` naming which.
    Wall(W),
}

/// The stop one round ends on, if any: the stall where `stalled`, over
/// any wall that fired on the same round.
pub(crate) fn first_stop<W>(stalled: bool, wall: Option<W>) -> Option<Stop<W>> {
    if stalled {
        Some(Stop::Stall)
    } else {
        wall.map(Stop::Wall)
    }
}

#[cfg(test)]
mod tests {
    use super::{Descent, PATIENCE, Stop, Work, first_stop};

    fn idles(bounds: &[f64]) -> Vec<usize> {
        let mut d = Descent::new();
        bounds
            .iter()
            .enumerate()
            .map(|(i, &b)| d.observe(b, i))
            .collect()
    }

    #[test]
    fn no_round_is_idle_before_a_bound_is_finite() {
        let inf = f64::INFINITY;
        assert_eq!(idles(&[inf, inf, f64::NAN, inf]), [0, 0, 0, 0]);
        assert_eq!(idles(&[inf, inf, 1e-3, inf, inf]), [0, 0, 0, 1, 2]);
    }

    #[test]
    fn idle_is_counted_against_the_lowest_bound_not_the_last() {
        // A rise, then a recovery that stays above the lowest: the
        // previous-round reading calls the recovery progress.
        assert_eq!(idles(&[1e-3, 2e-3, 1.5e-3]), [0, 1, 2]);
        assert_eq!(idles(&[1e-3, 2e-3, 0.9e-3, 0.9e-3]), [0, 1, 0, 1]);
    }

    #[test]
    fn an_equal_or_nan_bound_is_idle_and_keeps_the_first_round_to_reach_the_lowest() {
        let mut d = Descent::new();
        for (i, b) in [1e-3, 1e-3, f64::NAN].into_iter().enumerate() {
            d.observe(b, i);
        }
        assert!(
            d.stalled(),
            "two idle rounds stall at PATIENCE = {PATIENCE}"
        );
        assert_eq!(d.lowest(), Some((1e-3, &0)));
    }

    #[test]
    fn a_stall_speaks_over_a_wall_on_one_round() {
        assert_eq!(first_stop(true, Some('w')), Some(Stop::Stall));
        assert_eq!(first_stop(false, Some('w')), Some(Stop::Wall('w')));
        assert_eq!(first_stop::<char>(false, None), None);
    }

    #[test]
    fn the_budget_admits_a_round_that_spends_it_exactly() {
        let mut w = Work::new(10);
        assert!(w.admits(10) && !w.admits(11));
        w.spend(4);
        assert!(w.admits(6) && !w.admits(7));
        assert_eq!(w.spent(), 4);
    }
}
