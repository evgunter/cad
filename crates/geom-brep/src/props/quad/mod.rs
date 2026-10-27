//! **Certified quadrature for curved-cut faces** (M5 PR 11, C12.7) —
//! the kernel's first quadrature: divergence-theorem contributions for
//! analytic charts whose trim loops are *curved* (conic cuts), computed
//! as certified enclosures with an interval/hull-bounded remainder.
//! Certified bounds or typed refusal; never a silent Gaussian trust
//! (D4).
//!
//! # Formulation (cylinder chart)
//!
//! With chart `S(u, v) = o + û·r·cos u + v̂·r·sin u + â·v` the chart
//! normal is radial and `dA = r du dv`, so for a face whose trim region
//! is Ω (bounded by its loops, the outer one and its rings, each
//! traversed with the interior on its left about the outward normal):
//!
//! ```text
//! A_s   = −Σ_loops ∮ (v − v_a) du   (signed UV area; A_s = s_f·|Ω|)
//! area  = r·|A_s|
//! flux  = ∮ p·n dA = r²·A_s + o·A⃗    (A⃗ the exact vector area, summed over the loops)
//! ```
//!
//! — `v du` because it is a global 1-form on the cylinder (the azimuth
//! is periodic, its differential is not), so the form reads a loop
//! that winds the cylinder — a seamless band's rim — and a loop whose
//! images sit in different branches of the chart alike; `u dv` reads
//! neither. The loops' own traversal signs carry `s_f`, a ring's
//! opposite to its face's, so no separate orientation classification
//! is needed; what is decided is that the stored loops say so (every
//! junction closes, a whole number of turns apart in the azimuth; each
//! loop winds at most once and the face's loops none together, which
//! makes the form independent of the anchor level `v_a`; every loop
//! that does not wind, other than a non-band face's own, is wound
//! against its face).
//! The boundary is the face's **pcurves** (PR 6 caches; C4's first hot
//! consumer): per edge the contribution is
//! `σ·∫ (v_a − v(t))·u'(t) dt` over the stored carrier interval.
//!
//! # The integrand substrate is certification arithmetic
//!
//! Every enclosure here is [`Interval`](geom_core::interval::Interval) arithmetic — no
//! transcendental is ever *evaluated* on the certified path. The two
//! integrand families:
//!
//! - **Harmonic** (every minted cylinder-chart pcurve):
//!   `u, v(t) = c₀ + c_a·cos t + c_b·sin t + c_l·t`. Trigonometric
//!   values enter only as **enclosures propagated from the edge
//!   endpoints**: the caller supplies `(cos t₀, sin t₀)` brackets at the
//!   limits the lane integrates between ([`trig_at_limit`], rotated from
//!   the exact `(1, 0)`), and the engine advances them by exact rotation
//!   with polynomial alternating-series bounds on the step's
//!   `(cos, sin)` (valid for |step| ≤ 1.5; every step here is ≤ 1).
//!   Sound and transcendental-free.
//! - **B-spline** ([`bspline_green_integral`], the general machinery):
//!   channel hulls and derivative hulls from the control-coefficient
//!   convexity facts (`geom_core::spline::hull`). This family has **no
//!   at-rest consumer**: the NURBS-patch flux lanes are separate
//!   engines in this module — `patch_flux_exact` with its composite
//!   rounds for a polynomial patch, and `rational_patch_face`'s
//!   quotient composite (with the `w`-uniform-in-v exact arm) for a
//!   patch whose weights are not one constant — not this Green-form boundary
//!   integral. The consumer that would use it is
//!   the fitted-boundary Green lane, blocked on a construction that
//!   mints a fitted pcurve on an analytic chart at rest — the blocker
//!   `topo::props`' `QuadratureUnsupported` refusal names. That is a
//!   CONDITION, not a date: the lane is a deliberate frontier whose
//!   consumer lands with whichever banked construction (the marched
//!   join windows, the edge×NURBS-face boolean layer) first produces
//!   one, and its callers until then are tests.
//!
//! # The rule and its remainder
//!
//! Per piece `[p, q]`, `h = q − p`, `m` the midpoint (Taylor with
//! integral remainder; `∫(t−m)² dt = h³/12`):
//!
//! ```text
//! ∫_p^q f  ∈  h·f(m) + F₂·h³/24,   F₂ ⊇ f'' over [p, q]
//! ```
//!
//! `f(m)` is a *thin* enclosure (rotation to the midpoint), `F₂` a hull
//! enclosure — the remainder is the enclosure width itself, so the
//! bracket is sound at every refinement level and only *tightens* as
//! pieces shrink. Refinement doubles the piece count.
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
//! rounds. A call that ends unaccepted answers with its narrowest round
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
//!
//! # The lanes, and where they live
//!
//! The formulation above is the CYLINDER chart's, and it is one of
//! several engines here, one file each:
//!
//! - [`cylinder`]: [`cylinder_cut_face_rounds`], on the
//!   transcendental-free trig enclosures of [`trig`];
//! - [`green`]: the B-spline Green integral, [`bspline_green_integral`];
//! - [`patch`]: [`nurbs_patch_face_rounds`], the polynomial patch over a
//!   whole UV rectangle — [`patch_flux_exact`](patch::patch_flux_exact)'s
//!   per-span Newton–Cotes, with composite rounds behind it;
//! - [`rational`]: `rational_patch_face`, the same rectangle through the
//!   quotient statement;
//! - [`trimmed`]: [`trimmed_patch_face_rounds`], whose region is what a
//!   face's chart IMAGE bounds rather than a rectangle of its chart.
//!
//! What they share has its own homes: [`run`] (the rounds, the stop and
//! the answer, [`Run`]), [`net`] (the patch substrate, [`PatchNet`]),
//! [`cells`] (the composite's cells and the rational lane's hull
//! blocks) and [`area`] ([`area_midpoint_taylor`](area::area_midpoint_taylor),
//! the area rule both rectangle lanes share).
//!
//! The trimmed lane is a **second Green-form boundary integral** here,
//! so the sentence above about [`bspline_green_integral`] having no
//! at-rest consumer is a statement about THAT function, not about the
//! form: the trimmed lane integrates `−∮ G_f du` on the image's chord
//! polygon with its own rules — the exact Newton–Cotes rule inside its
//! window, the composite midpoint rule with a certified remainder past
//! it — and its own pads.
//!
//! # Honesty pads (both directions accounted)
//!
//! The list below is the cylinder lane's two. The trimmed lane adds
//! its own, each documented at its site: the LUNE between an arc and
//! its chord, the VERTEX connectors from a bracket's midpoint to the
//! point it stands for, the exact rule's `v`-knot crossing SLIVER, the
//! composite rule's REMAINDER and its crossing-bracket COLUMN bound —
//! plus the closure gap and the refinement's `λ` rounding.
//!
//! - **Map residual**: each pcurve tracks its carrier within its
//!   certificate envelope `e` (metres through the map). The cylinder
//!   metric is `ds² = r²du² + dv²`, so the true trim boundary lies
//!   within metric distance `e` of the pcurve and the UV-area defect is
//!   ≤ Σ (metric edge length)·e / r — added to `A_s` symmetrically.
//! - **Endpoint parameter brackets**: a non-thin `t₀/t₁` bracket
//!   contributes `mag(f)·width` at each end.
//!
//! The closed-form `o·A⃗` term rides at the caller's scalar exactly as
//! the closed-form lanes do (f64 rounding of closed forms is the
//! documented slack there, unchanged).

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
    classify_len, mean_boundary_displacement, narrower,
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

    /// The resolution these rows stand their targets at, as a multiple
    /// of ε: 1024·ε, the one `topo`'s suites read their pinned numbers at.
    pub(in crate::props) const QUAD_TARGET_LEN_FACTOR: f64 = 1024.0;

    /// Accepts a round whose mean boundary displacement is at most
    /// `QUAD_TARGET_LEN_FACTOR·eps`: the reporting target, read as a
    /// plain comparison (the funnel's reading of it is `topo`'s).
    pub(in crate::props) fn to_target(
        eps: f64,
    ) -> impl FnMut(&FaceCutBounds, LaneStop) -> Result<bool, PropsError> {
        move |b, _| Ok(mean_boundary_displacement(b.flux, b.area)? <= QUAD_TARGET_LEN_FACTOR * eps)
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
