//! **Certified quadrature** (C12.7): the divergence-theorem flux
//! `∮ p·n dA` and the area of the faces no closed-form arm covers, as
//! certified enclosures with a bounded remainder. Certified bounds or
//! typed refusal; never a silent Gaussian trust (D4). Every enclosure
//! is [`Interval`](geom_core::interval::Interval) arithmetic, and no
//! transcendental is ever *evaluated* on the certified path.
//!
//! # The lanes
//!
//! Five engines, one file each; each file's docs carry its rule and its
//! honesty pads:
//!
//! - [`cylinder`]: [`cylinder_cut_face_rounds`], a cylinder face with a
//!   curved trim, by the Green form on its harmonic pcurves (on the
//!   transcendental-free enclosures of [`trig`]);
//! - [`green`]: the B-spline Green integral, [`bspline_green_integral`],
//!   the same boundary rule on spline channels, with no at-rest consumer;
//! - [`patch`]: [`nurbs_patch_face_rounds`], the polynomial patch over a
//!   whole UV rectangle — [`patch_flux_exact`](patch::patch_flux_exact)'s
//!   per-span Newton–Cotes, with composite rounds behind it;
//! - [`rational`]: `rational_patch_face`, the same rectangle through the
//!   quotient statement;
//! - [`trimmed`]: [`trimmed_patch_face_rounds`], whose region is what a
//!   face's chart IMAGE bounds rather than a rectangle of its chart: a
//!   second Green-form boundary integral, on the image's chord polygon.
//!
//! What they share has its own homes: [`run`] (the rounds, the stop and
//! the answer, [`Run`]), [`net`] (the patch substrate, [`PatchNet`]),
//! [`cells`] (the composite's cells and the rational lane's hull
//! blocks) and [`area`] ([`area_midpoint_taylor`](area::area_midpoint_taylor),
//! the area rule both rectangle lanes share).
//!
//! # When a lane stops
//!
//! A lane knows no target. Each round it hands its enclosure to the
//! caller's [`Accept`], which answers whether that is enough, and it
//! stops short of an answer on one of two findings of its own
//! ([`LaneStop`], read by the one [`Run`]): its FLOOR, where the part of
//! the width a later round removes (the rule's remainder) is at most
//! [`FLOOR_SHARE`] of it, the rest being the part the rule does not
//! remove (the honesty pads and the arithmetic),
//! and the work WALL, [`FACE_WORK_WALL`], one bound in one unit for every
//! lane, on the weighted cell evaluations a face spends over all its
//! rounds — once per knot-span cell on the rectangle lanes, whose cells
//! are laid per span, so a patch costs the same read whole or split
//! into one face per span. A call that ends unaccepted answers with its narrowest round
//! and says why it stopped ([`LaneRun`]), never silently wide.
//!
//! The rounds are independent recomputations, and a round's stop reads
//! that round alone, so the windows of one face compose: running
//! `0..=k` and then `k+1..=…` ([`RoundWindow`]) runs each round the
//! uninterrupted run runs, once, and ends in the same place. Each entry
//! re-derives the face's setup (its hull blocks, its spans), which the
//! uninterrupted run derives once.
//! A caller deciding a SIGN can stop between rounds and resume.
//!
//! **One arm has no rounds to address and ignores the window's
//! start**: [`nurbs_patch_face`]'s exact per-span rule, which answers
//! a whole patch in one Newton–Cotes pass, at its floor.

mod area;
mod cells;
mod cylinder;
mod green;
mod net;
mod patch;
mod rational;
mod run;
mod trig;
mod trimmed;

pub use area::boundary_chord_perimeter_lo;

pub use cylinder::{HarmChan, TrimEdgeQ, cylinder_cut_face, cylinder_cut_face_rounds};
pub use green::bspline_green_integral;
pub use net::{PatchNet, RVec3};
pub use patch::{nurbs_patch_face, nurbs_patch_face_rounds};
#[cfg(any(test, feature = "test-support"))]
pub use run::rounds_run_by;
pub use run::{
    Accept, FACE_WORK_WALL, FLOOR_SHARE, FaceCutBounds, LaneEnd, LaneRun, LaneStop, RoundWindow,
    mean_boundary_displacement, narrower,
};
pub(super) use run::{EDGE_INIT_PIECES, Run, face_extent};
pub use trig::trig_at_limit;
pub(super) use trig::{rotate, spread_cos, spread_sin, trig_at};
pub use trimmed::{TrimChord, TrimPiece, trimmed_patch_face_rounds};

/// Test vocabulary for the lanes' callers in this crate.
#[cfg(test)]
pub(super) mod test_target {
    use super::{
        FaceCutBounds, LaneEnd, LaneRun, LaneStop, PatchNet, RVec3, mean_boundary_displacement,
    };
    use crate::props::PropsError;
    use geom_core::Bounds;
    use geom_core::interval::Interval;
    use geom_core::spline::KnotVector;

    /// The test-only reporting resolution, as a multiple of ε: 1024·ε,
    /// the resolution `topo`'s suites pin their numbers at. No lane
    /// reads it; the rows here stand their targets at it.
    pub(in crate::props) const REPORTING_RESOLUTION_FACTOR: f64 = 1024.0;

    /// Accepts a round whose mean boundary displacement is at most
    /// `REPORTING_RESOLUTION_FACTOR·eps`: the reporting target, read as a
    /// plain comparison (the funnel's reading of it is `topo`'s).
    pub(in crate::props) fn to_target(
        eps: f64,
    ) -> impl FnMut(&FaceCutBounds, LaneStop) -> Result<bool, PropsError> {
        move |b, _| {
            Ok(mean_boundary_displacement(b.flux, b.area)? <= REPORTING_RESOLUTION_FACTOR * eps)
        }
    }

    /// The enclosure of a run the caller accepted; a run that stopped
    /// short of it panics, naming where it stopped.
    pub(in crate::props) fn met(
        run: Result<LaneRun, PropsError>,
    ) -> Result<FaceCutBounds, PropsError> {
        let run = run?;
        assert!(
            matches!(run.end, LaneEnd::Accepted(_)),
            "the lane stopped short of the target: {run:?}"
        );
        Ok(run.bounds)
    }

    /// The checked net of a fixture's parts.
    #[allow(clippy::unwrap_used)]
    pub(in crate::props) fn patch<'a>(
        ku: &'a KnotVector,
        kv: &'a KnotVector,
        control: &[RVec3],
        weights: &[f64],
    ) -> PatchNet<'a> {
        PatchNet::new(ku, kv, control, weights).unwrap()
    }

    /// The bounds of a run, however it ended.
    pub(in crate::props) fn bounds_of(out: &LaneRun) -> FaceCutBounds {
        out.bounds
    }

    pub(in crate::props) fn encloses(x: Interval, truth: f64, label: &str) {
        assert!(
            x.lo() <= truth && truth <= x.hi(),
            "{label}: the closed form {truth} escapes the certified enclosure {x:?}"
        );
    }
}
