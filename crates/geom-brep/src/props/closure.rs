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
//! whole numbers, read two ways by what the lane holds:
//!
//! - an interval lane holds each junction's chart ends, and its turns are
//!   the sum of the whole periods each junction's gap is nearest
//!   ([`interval_loop_turns`]);
//! - a closed form holds its carriers, and reads a loop's total angle —
//!   its edges' end-to-end turns and its junctions' short turns — at
//!   half-turn thresholds, then decides it a whole number of turns
//!   within its stated slack ([`closed_form_windings`]).
//!
//! A face's boundary is simple closed curves that bound one connected
//! region, so each loop turns at most once about each direction, the
//! face's loops turn zero times together, and at most two of them turn
//! at all ([`require_boundary_windings`]): integer facts that decide
//! nothing in the band.
//!
//! A lane whose closed form cannot fold a legal winding configuration —
//! the torus's, for a seamless band — refuses it as its own inventory
//! after this decision has passed it.

use geom_core::interval::{Interval, certified_hi};
use geom_core::{Band, Decide, Margin, Sign};

use super::bracket::{AbsEnclosure, lo_or_refuse, mid, pt, tau_bracket};
use super::{PropsCheck, PropsError};
use crate::dihedral::decide;

/// Each carrier's end lies within the run's ε of the vertex it ends
/// at, so two meet there within `2ε`: in the chart metric, `3ε`. The
/// corpus needs it: on the geom-brep, topo and sweep suites at
/// ε = 1e-12, 3428 junctions of real faces read up to `1.99ε` beyond
/// their end slack alone.
pub(super) const VERTEX_SLACK: f64 = 3.0;

/// How far `gap` exceeds what `slack` allows: `0` within it, so a
/// closed junction reads zero rather than a negative margin the band
/// would hold as near. `NaN` stays `NaN` ([`geom_core::Real::max`]).
pub(super) fn excess<T: Decide>(gap: T, slack: T) -> T {
    (gap - slack).max(T::zero())
}

/// A junction's chart gap, less whole periods, in excess of its stated
/// slack (metres; `0` within it): read high, for the closed decision,
/// and low, for the open one. A lane whose gap is one value hands the
/// same margin twice.
pub(super) struct JunctionExcess<T> {
    pub(super) hi: Margin<T>,
    pub(super) lo: Margin<T>,
}

fn escalated(cause: geom_core::Indeterminate) -> PropsError {
    PropsError::Escalated {
        cause,
        check: PropsCheck::Inventory,
    }
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

/// **A face's loops wind its chart as a boundary does**
/// ([`PropsError::LoopsWindChart`]). `windings[i][c]` is loop `i`'s
/// whole turns about direction `c`:
///
/// - each loop turns at most once about each direction;
/// - the face's loops turn zero times together about each;
/// - at most two loops turn at all. A face is one connected region, and
///   disjoint loops that wind a cylinder or a torus are parallel: the
///   region between two consecutive ones is an annulus, so a connected
///   region has at most two of them on its boundary. Four rims of one
///   cylinder, wound `+ + − −` or `+ − − +` up the axis, pass the first
///   two checks and bound no single face.
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
    if windings
        .iter()
        .filter(|k| k.iter().any(|&kc| kc != 0))
        .count()
        > 2
    {
        return Err(PropsError::LoopsWindChart);
    }
    Ok(())
}

/// One junction of an interval lane's loop, read off its chart images,
/// per coordinate `c`: `delta[c]`, the edge before's end less the edge
/// after's start; `lever[c]`, an enclosure of the metres one unit of
/// that coordinate moves a point at the junction; and `slack`, the two
/// ends' own certified slack (metres).
pub(super) struct IntervalJunction<const N: usize> {
    pub(super) delta: [Interval; N],
    pub(super) lever: [Interval; N],
    pub(super) slack: f64,
}

/// **An interval lane's loop closes**, returning its whole turns about
/// each coordinate `periodic` marks.
///
/// At each junction a periodic coordinate's gap is taken less the whole
/// number `m` of periods nearest its midpoint, and the junction's gap is
/// `G = Σ lever·|Δ − m·τ|` over the coordinates, an enclosure. Its upper
/// bound's excess over the slack `S` (the ends' own and
/// [`VERTEX_SLACK`]`·ε`) is the closed reading, its lower bound's the
/// open one ([`require_junction_closed`]). A loop's chart images are
/// continuous along each edge, so once every junction is closed its
/// turns are the sum of its junctions' `m`.
pub(super) fn interval_loop_turns<T: Decide, const N: usize>(
    junctions: impl IntoIterator<Item = IntervalJunction<N>>,
    periodic: [bool; N],
    band: Band,
) -> Result<[i64; N], PropsError> {
    let tau = tau_bracket();
    let mut turns = [0i64; N];
    for j in junctions {
        let mut gap = pt(0.0);
        let mut m = [0.0; N];
        for c in 0..N {
            if periodic[c] {
                m[c] = (mid(j.delta[c]) / core::f64::consts::TAU).round();
            }
            gap = gap + j.lever[c] * (j.delta[c] - pt(m[c]) * tau).abs_enclosure();
        }
        let slack = j.slack + VERTEX_SLACK * band.zero();
        require_junction_closed(
            JunctionExcess {
                hi: Margin::of(excess(certified_hi(gap), slack)).lift::<T>(),
                lo: Margin::of(excess(lo_or_refuse(gap), slack)).lift::<T>(),
            },
            band,
        )?;
        // `m` is finite here: a refused enclosure reads `NaN` in the gap
        // and escalates above. The cast saturates.
        #[allow(clippy::cast_possible_truncation)]
        for c in 0..N {
            turns[c] = turns[c].saturating_add(m[c] as i64);
        }
    }
    Ok(turns)
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

/// **A closed form's circle sits clear of the axis it is carried to**
/// (`props_chart_axis_inside`): `radius`, the circle's radius about the
/// axis (the core circle), exceeds `2ε`. The parse holds the circle's
/// centre within `ε` of the axis (the core circle), so then the axis
/// passes inside the circle: from any point of it the circle's centre
/// and the axis are less than a quarter turn apart, and the turn
/// carrying the circle's stored turn to the axis (`curved::about_axis`
/// and the tube's analogue) is read clear of its branch cut. The
/// cylinder decides it once at its radius, which every rim's is within
/// `ε` of; the torus once at its minor radius, for its meridians, and
/// at each parallel's own. A circle narrower than that is within its own
/// band, and this closed form has no reading of it
/// ([`PropsError::NotIsoRectangle`]).
pub(super) fn require_axis_inside<T: Decide>(radius: T, band: Band) -> Result<(), PropsError> {
    let eps = T::from_f64(band.zero());
    if decide(
        "props_chart_axis_inside",
        Margin::of(radius - eps - eps),
        band,
    )
    .map_err(escalated)?
        == Sign::Positive
    {
        Ok(())
    } else {
        Err(PropsError::NotIsoRectangle {
            what: "props_chart_axis_inside",
        })
    }
}

/// **A closed form's loops close and wind as a face's boundary does**,
/// returning each loop's whole turns per periodic coordinate.
///
/// A closed form's carriers end exactly where they are evaluated, so a
/// junction's slack is [`VERTEX_SLACK`]`·ε` and its gap is one value,
/// `Σ lever·|turn| + |flat|`.
///
/// A loop's total angle about a direction is `W = sweep + Σ turn`: its
/// edges' sweeps and the short turns across its junctions. The caller's
/// sweep is each edge's turn about the surface's own axis (or core
/// circle) between that edge's own two ends, never a nominal:
///
/// - a rim's (a meridian's) stored turn is about its own centre, which
///   the parse admits up to `ε` off the axis (the core circle); its ends
///   then sit up to `≈ 2ε/R` farther or less far round the axis than
///   that turn says, arc by arc, and that does not cancel across
///   carriers. So the caller carries the stored turn to the axis at each
///   end (`curved::about_axis`, and the tube's analogue);
/// - an edge that nominally turns nothing in a coordinate (a ruling, a
///   meridian's azimuth, a parallel's tube angle) is read as the short
///   turn between its ends: a ruling certified axial may still turn by
///   `ε/R`, and a loop may hold any number of them.
///
/// So `W` is exactly the total turn of a closed path — each edge about
/// the axis, then the short way across each junction — and a whole
/// number of turns up to rounding, whatever the junction gaps and
/// however many edges. [`loop_turns`] reads the whole number `k` at
/// half-turn thresholds, and the residual is then decided
/// (`props_chart_loop_whole`): `|W − k·τ|` metered at the loop's largest
/// junction radius, against [`VERTEX_SLACK`]`·ε` for the loop. A residual
/// past it is a sweep that disagrees with its edge's ends
/// ([`PropsError::LoopsWindChart`]), and in the band it escalates.
///
/// The thresholds are metered at that radius too; they are clear by `π`
/// times any radius.
///
/// The interval lanes ([`interval_loop_turns`]) read each edge's own
/// chart ends, so their turns are exact by construction and ask no such
/// question.
pub(super) fn closed_form_windings<T: Decide, const N: usize>(
    loops: &[ChartLoop<T, N>],
    band: Band,
) -> Result<Vec<[i64; N]>, PropsError> {
    let slack = T::from_f64(VERTEX_SLACK * band.zero());
    let mut windings = Vec::with_capacity(loops.len());
    for l in loops {
        let mut w = l.sweep;
        let mut lever = [T::zero(); N];
        for j in &l.joints {
            let gap = j
                .turn
                .iter()
                .fold(j.flat.abs(), |g, &(turn, lever)| g + lever * turn.abs());
            let over = Margin::of(excess(gap, slack));
            require_junction_closed(JunctionExcess { hi: over, lo: over }, band)?;
            for c in 0..N {
                w[c] = w[c] + j.turn[c].0;
                lever[c] = lever[c].max(j.turn[c].1);
            }
        }
        let mut k = [0i64; N];
        for c in 0..N {
            k[c] = loop_turns(w[c], lever[c], band)?;
            let whole = match k[c] {
                0 => T::zero(),
                1 => T::tau(),
                -1 => T::zero() - T::tau(),
                // Past one turn: refused below as no boundary's.
                _ => continue,
            };
            let residual = (w[c] - whole).abs() * lever[c];
            if decide(
                "props_chart_loop_whole",
                Margin::of(excess(residual, slack)),
                band,
            )
            .map_err(escalated)?
                != Sign::Zero
            {
                return Err(PropsError::LoopsWindChart);
            }
        }
        windings.push(k);
    }
    require_boundary_windings(&windings)?;
    Ok(windings)
}

/// **A closed loop's whole turns** from its total angle `W` (rad),
/// metered at `lever` (`props_chart_loop_turns`, `props_chart_loop_sense`):
/// `0` when `|W|` is short of a half turn, `±1` when it is past one and
/// short of three, `±2` for any more. A whole number of turns is clear of
/// each threshold by `π·lever`; a `W` in the band of one is no whole
/// number, so the loop does not close. Whether `W` is that whole number
/// is the caller's decision ([`closed_form_windings`]).
fn loop_turns<T: Decide>(w: T, lever: T, band: Band) -> Result<i64, PropsError> {
    let classify = |name, margin| decide(name, margin, band).map_err(escalated);
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use geom_core::{Interval, Tol};

    /// One loop of one closed junction whose edges' sweep is `excess`
    /// metres (at a radius of `1`) off no turn at all.
    fn off_a_whole_turn<T: Decide>(excess: f64, band: Band) -> Result<Vec<[i64; 1]>, PropsError> {
        let one = ChartLoop {
            sweep: [T::from_f64(excess)],
            joints: vec![Joint {
                turn: [(T::zero(), T::one())],
                flat: T::zero(),
            }],
        };
        closed_form_windings(&[one], band)
    }

    /// **A loop whose total turn is off a whole number of turns refuses**
    /// (`props_chart_loop_whole`), at `f64` and at the interval scalar:
    /// within the loop's `3ε` it reads its turns; past it by `2Kε` it is
    /// no boundary ([`PropsError::LoopsWindChart`]); past it by `2ε`, in
    /// the band, it escalates.
    #[test]
    fn a_loop_off_a_whole_turn_refuses_and_in_the_band_escalates() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (zero, k) = (band.zero(), band.escalate() / band.zero());
        let within = VERTEX_SLACK * zero - 0.5 * zero;
        let past = VERTEX_SLACK * zero + 2.0 * k * zero;
        let in_band = VERTEX_SLACK * zero + 2.0 * zero;
        for (scalar, run) in [
            ("f64", off_a_whole_turn::<f64> as fn(f64, Band) -> _),
            ("interval", off_a_whole_turn::<Interval>),
        ] {
            assert_eq!(
                run(within, band),
                Ok(vec![[0]]),
                "within the slack at {scalar}"
            );
            assert_eq!(
                run(past, band),
                Err(PropsError::LoopsWindChart),
                "past the slack at {scalar}"
            );
            assert!(
                matches!(run(in_band, band), Err(PropsError::Escalated { .. })),
                "in the band at {scalar}"
            );
        }
    }
}
