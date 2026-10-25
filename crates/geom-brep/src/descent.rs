//! **The stop rule of a certify-and-refine loop.** A loop that refines
//! until a certified bound clears its target stops, short of that, on
//! one of two questions: is refinement still paying, and is the work
//! bounded? Two loops answer both here, each keeping its own body, its
//! own units and its own action on an idle round: the offset fit
//! ([`crate::offset_fit`]) and limb 2's hull subdivision
//! (`ssi::certify::subdivide`). The quadrature lanes in `props::quad`
//! take the cumulative budget ([`Work`]) as their work wall. None of
//! them takes the stall today: each splits a round's width into the
//! part a later round removes and the part none does, so refinement
//! stopping paying is a reading of the round itself
//! (`props::quad::Run`'s floor). The stall stays the backstop the
//! measure door's R2 names, for a lane that cannot split the two. Not every such loop
//! in the crate reads this home: `ssi::refine::refine_by_certificate`
//! compares each round with the previous one and is bounded by the
//! march's step wall.
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
//! - **Precedence** ([`Descent::stop`], which every stop of both loops
//!   goes through): where a stall and a wall both end one round, the
//!   stall speaks. It is the finding about the request; a wall says
//!   only that the loop ran out before learning more.

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
/// It exists from the first round on, so it always has a lowest bound.
#[derive(Debug)]
pub(crate) struct Descent<T> {
    lowest: (f64, T),
    idle: usize,
}

impl<T> Descent<T> {
    /// The tracker after a round that reported `bound` on `at`, from
    /// `before`, the tracker after the round before it (`None` before
    /// the first round).
    ///
    /// A round lowers the lowest bound when it is strictly smaller than
    /// a finite lowest bound, or finite where the lowest is not; it is
    /// idle when it does not lower a finite lowest bound, so an equal
    /// bound, a NaN or `+∞` is idle and the older `at` is kept (the
    /// first round to reach the minimum). While the lowest bound is not
    /// finite no round is idle, and a round that is not finite either
    /// keeps the older `at`.
    pub(crate) fn after(before: Option<Self>, bound: f64, at: T) -> Self {
        let Some(mut d) = before else {
            return Self {
                lowest: (bound, at),
                idle: 0,
            };
        };
        let low = d.lowest.0;
        if low.is_finite() {
            if bound < low {
                d.lowest = (bound, at);
                d.idle = 0;
            } else {
                d.idle += 1;
            }
        } else if bound.is_finite() {
            d.lowest = (bound, at);
        }
        d
    }

    /// How many rounds in a row, the last included, have not lowered
    /// the lowest finite bound.
    pub(crate) fn idle(&self) -> usize {
        self.idle
    }

    /// The stop the last round ends on, if any. `exhausted` is the
    /// loop's own evidence that refinement cannot go on (nothing left
    /// to refine, or nothing to steer by), which stops it as the stall
    /// does; `wall` is a work bound the next round would pass. A stall
    /// speaks over a wall.
    pub(crate) fn stop<W>(&self, exhausted: bool, wall: Option<W>) -> Option<Stop<W>> {
        if exhausted || self.idle >= PATIENCE {
            Some(Stop::Stall)
        } else {
            wall.map(Stop::Wall)
        }
    }

    /// The lowest bound reached and what it was first reached on: the
    /// first round's where no round's bound was finite.
    pub(crate) fn lowest(&self) -> (f64, &T) {
        (self.lowest.0, &self.lowest.1)
    }

    /// [`Self::lowest`], by value.
    pub(crate) fn into_lowest(self) -> (f64, T) {
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
    /// Refinement stopped paying: [`PATIENCE`] idle rounds in a row,
    /// or the loop's own exhaustion ([`Descent::stop`]).
    Stall,
    /// A work bound, the loop's own `W` naming which.
    Wall(W),
}

#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use super::{Descent, PATIENCE, Stop, Work};

    fn run(bounds: &[f64]) -> (Vec<usize>, Descent<usize>) {
        let mut d = None;
        let mut idles = Vec::new();
        for (i, &b) in bounds.iter().enumerate() {
            let next = Descent::after(d, b, i);
            idles.push(next.idle());
            d = Some(next);
        }
        (idles, d.unwrap_or_else(|| panic!("no round")))
    }

    #[test]
    fn no_round_is_idle_before_a_bound_is_finite() {
        let inf = f64::INFINITY;
        assert_eq!(run(&[inf, inf, f64::NAN, inf]).0, [0, 0, 0, 0]);
        assert_eq!(run(&[inf, inf, f64::NAN]).1.lowest(), (inf, &0));
        assert_eq!(run(&[inf, inf, 1e-3, inf, inf]).0, [0, 0, 0, 1, 2]);
        assert_eq!(run(&[f64::NAN, 1e-3]).1.lowest(), (1e-3, &1));
    }

    #[test]
    fn idle_is_counted_against_the_lowest_bound_not_the_last() {
        // A rise, then a recovery that stays above the lowest: the
        // previous-round reading calls the recovery progress.
        assert_eq!(run(&[1e-3, 2e-3, 1.5e-3]).0, [0, 1, 2]);
        assert_eq!(run(&[1e-3, 2e-3, 0.9e-3, 0.9e-3]).0, [0, 1, 0, 1]);
    }

    #[test]
    fn an_equal_or_nan_bound_is_idle_and_keeps_the_first_round_to_reach_the_lowest() {
        let (idles, d) = run(&[1e-3, 1e-3, f64::NAN]);
        assert_eq!(idles, [0, 1, 2]);
        assert_eq!(
            d.stop::<()>(false, None),
            Some(Stop::Stall),
            "PATIENCE = {PATIENCE}"
        );
        assert_eq!(d.into_lowest(), (1e-3, 0));
    }

    #[test]
    fn a_stall_speaks_over_a_wall_on_one_round() {
        let (_, idle_once) = run(&[1.0, 1.0]);
        assert_eq!(idle_once.stop::<char>(false, None), None);
        assert_eq!(idle_once.stop(false, Some('w')), Some(Stop::Wall('w')));
        assert_eq!(idle_once.stop(true, Some('w')), Some(Stop::Stall));
        let (_, stalled) = run(&[1.0, 1.0, 1.0]);
        assert_eq!(stalled.stop(false, Some('w')), Some(Stop::Stall));
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
