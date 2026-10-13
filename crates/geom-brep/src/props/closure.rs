//! **Whether a face's loops close on a periodic chart, and wind it as a
//! face's boundary does**: one decision for every lane that reads a face
//! on a cylinder or a torus chart — the cylinder quadrature
//! ([`super::quad`]), the cylinder and torus closed forms
//! ([`super::curved`]) and the spiric wall ([`super::spiric`]).
//!
//! At each junction the edge before ends where the edge after starts:
//! their chart points differ by a whole number of periods in each
//! periodic coordinate and by nothing else. The gap left once those
//! periods are taken off, metered in metres, is decided against the
//! junction's stated slack ([`require_junction_closed`]). With every
//! junction closed, a loop's turns about each periodic direction are
//! whole numbers. A face's boundary is simple closed curves that bound
//! it, so each loop turns at most once about each direction and the
//! face's loops turn zero times together ([`require_boundary_windings`]),
//! integer facts that decide nothing in the band.
//!
//! A lane whose closed form cannot fold a legal winding configuration —
//! the torus's, for a seamless band — refuses it as its own inventory
//! after this decision has passed it.

use geom_core::{Band, Decide, Margin, Sign};

use super::{PropsCheck, PropsError};
use crate::dihedral::decide;

/// Each carrier's end lies within the run's ε of the vertex it ends
/// at, so two meet there within `2ε`: in the chart metric, `3ε`. The
/// corpus needs it: on the geom-brep, topo and sweep suites at
/// ε = 1e-12, 3428 junctions of real faces read up to `1.99ε` beyond
/// their end slack alone.
pub(super) const VERTEX_SLACK: f64 = 3.0;

/// How far an interval lane's `gap` exceeds what `slack` allows: `0`
/// within it, so a closed junction reads zero rather than a negative
/// margin the band would hold as near; `NaN` stays `NaN`.
pub(super) fn excess(gap: f64, slack: f64) -> f64 {
    let over = gap - slack;
    if over > 0.0 || over.is_nan() {
        over
    } else {
        0.0
    }
}

/// A junction's chart gap, less whole periods, in excess of its stated
/// slack (metres; `0` within it): read high, for the closed decision,
/// and low, for the open one. A lane whose gap is one value hands the
/// same margin twice.
pub(super) struct JunctionExcess<T> {
    pub(super) hi: Margin<T>,
    pub(super) lo: Margin<T>,
}

/// **One junction closes** (`props_chart_junction_closed`,
/// `props_chart_junction_open`), with the band `(ε, Kε)`:
///
/// - **closed** when the high excess is under `ε` — every gap the
///   reading admits is within the slack and `ε`;
/// - **open** ([`PropsError::ChartLoopOpen`]) when the high excess is
///   beyond `Kε` and so is the low one, through the funnel's positive
///   gate — no gap the reading admits is within the slack and `Kε`;
/// - **escalated** otherwise. A junction the reading cannot decide is
///   never measured.
pub(super) fn require_junction_closed<T: Decide>(
    gap: JunctionExcess<T>,
    band: Band,
) -> Result<(), PropsError> {
    let escalated = |cause| PropsError::Escalated {
        cause,
        check: PropsCheck::Inventory,
    };
    if decide("props_chart_junction_closed", gap.hi, band).map_err(escalated)? == Sign::Zero {
        return Ok(());
    }
    Err(
        match geom_core::k_stats::decide_positive("props_chart_junction_open", gap.lo, band) {
            Ok(()) => PropsError::ChartLoopOpen,
            Err(cause) => escalated(cause),
        },
    )
}

/// **A face's loops wind its chart as a boundary does**: each loop at
/// most once about each periodic direction, and all of them zero times
/// together ([`PropsError::LoopsWindChart`]). `windings[i][c]` is loop
/// `i`'s whole turns about direction `c`.
pub(super) fn require_boundary_windings<const N: usize>(
    windings: &[[i64; N]],
) -> Result<(), PropsError> {
    for c in 0..N {
        let mut total = 0i64;
        for k in windings {
            if k[c].abs() > 1 {
                return Err(PropsError::LoopsWindChart);
            }
            total = total.saturating_add(k[c]);
        }
        if total != 0 {
            return Err(PropsError::LoopsWindChart);
        }
    }
    Ok(())
}

/// One junction of a closed-form loop: per periodic coordinate, the
/// signed angle from the edge before's end to the edge after's start,
/// the short way round (rad), and the radius that meters it (m); and
/// the gap in the chart's other coordinate, if it is not periodic (m).
pub(super) struct Joint<T, const N: usize> {
    pub(super) turn: [(T, T); N],
    pub(super) flat: T,
}

/// One closed-form loop: per periodic coordinate, the signed sweep of
/// its edges summed (rad), and its joints in loop order.
pub(super) struct ChartLoop<T, const N: usize> {
    pub(super) sweep: [T; N],
    pub(super) joints: Vec<Joint<T, N>>,
}

/// **A closed form's loops close and wind as a face's boundary does**,
/// returning each loop's whole turns per periodic coordinate.
///
/// A closed form's carriers end exactly where they are evaluated, so a
/// junction's slack is [`VERTEX_SLACK`]`·ε` and its gap is one value,
/// `Σ lever·|turn| + |flat|`. Each joint's turn is the short way round,
/// so once every junction is closed a loop's turns are
/// `W/τ = (sweep + Σ turn)/τ`, a whole number in exact arithmetic;
/// [`loop_turns`] reads it.
pub(super) fn closed_form_windings<T: Decide, const N: usize>(
    loops: &[ChartLoop<T, N>],
    band: Band,
) -> Result<Vec<[i64; N]>, PropsError> {
    let slack = T::from_f64(VERTEX_SLACK * band.zero());
    let mut windings = Vec::with_capacity(loops.len());
    for l in loops {
        let mut w = l.sweep;
        for j in &l.joints {
            let gap = j
                .turn
                .iter()
                .fold(j.flat.abs(), |g, &(turn, lever)| g + lever * turn.abs());
            let excess = Margin::of((gap - slack).max(T::zero()));
            require_junction_closed(
                JunctionExcess {
                    hi: excess,
                    lo: excess,
                },
                band,
            )?;
            for (wc, &(turn, _)) in w.iter_mut().zip(&j.turn) {
                *wc = *wc + turn;
            }
        }
        let mut k = [0i64; N];
        for c in 0..N {
            let lever = l.joints.first().map_or(T::one(), |j| j.turn[c].1);
            k[c] = loop_turns(w[c], lever, band)?;
        }
        windings.push(k);
    }
    require_boundary_windings(&windings)?;
    Ok(windings)
}

/// **A closed loop's whole turns** from its total angle `W` (rad),
/// metered at `lever` (`props_chart_loop_turns`, `props_chart_loop_sense`):
/// `0` when `|W|` is short of a half turn, `±1` when it is past one and
/// short of three, `±2` for any more — the half-turn thresholds a whole
/// number of turns is clear of by `π·lever`. A `W` within the band of a
/// half turn is no whole number, so the loop does not close.
fn loop_turns<T: Decide>(w: T, lever: T, band: Band) -> Result<i64, PropsError> {
    let classify = |name, margin| {
        decide(name, margin, band).map_err(|cause| PropsError::Escalated {
            cause,
            check: PropsCheck::Inventory,
        })
    };
    let half = |halves: T| Margin::levered(T::pi() * halves - w.abs(), lever);
    match classify("props_chart_loop_turns", half(T::one()))? {
        Sign::Positive => return Ok(0),
        Sign::Zero => return Err(PropsError::ChartLoopOpen),
        Sign::Negative => {}
    }
    let k = match classify("props_chart_loop_turns", half(T::from_f64(3.0)))? {
        Sign::Positive => 1,
        Sign::Negative => 2,
        Sign::Zero => return Err(PropsError::ChartLoopOpen),
    };
    match classify("props_chart_loop_sense", Margin::levered(w, lever))? {
        Sign::Positive => Ok(k),
        Sign::Negative => Ok(-k),
        Sign::Zero => Err(PropsError::ChartLoopOpen),
    }
}
