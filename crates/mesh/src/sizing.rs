//! The sizing vocabulary: one home for "how fine should this be".
//!
//! # Three quantities, one word each
//!
//! Every sizing question in this crate is one of three things, and
//! this module fixes the word for each so a reader never has to open a
//! function to learn which kind it answers:
//!
//! - a **target** — the metres of deviation a schedule aims at. There
//!   is exactly one, δ_s = [`sizing_target`]`(δ)`, and it is carried
//!   with the run's other tolerances by [`Tol`].
//! - a **step** — an increment in a *parameter* (a curve parameter, a
//!   chart angle, a UV coordinate), always `f64`. Steps come from a
//!   closed-form deviation bound: [`sagitta_step`] and [`ellipse_step`]
//!   here, [`curvature_step`] for a second-derivative bound,
//!   [`torus_grid_step`] for the torus chart, and
//!   [`crate::nurbs_cert::NurbsFaceBound::grid_steps`] for a certified
//!   NURBS patch. The cap on an angular one is [`MAX_ANGULAR_STEP`];
//!   this module is the only place it is applied ([`cap_angular`], and
//!   [`sagitta_step`]'s vacuous branch, which returns it directly).
//! - a **count** — how many chords or grid divisions a span is cut
//!   into, always `usize`. Every count in the crate is
//!   [`ceil_count`]`(span, step)` or a rule applied to one
//!   (`curved::pole_columns` lifts the single singular value
//!   `nu == 2` — an equality, not a floor, and its own doc says why;
//!   `NurbsCellGrid::band_schedule` snaps a malign band's).
//!
//! **The rule, in one sentence:** *"step" names an `f64` increment and
//! nothing else; a `usize` count is never called a step, and neither is
//! a sample count.* A new rule that PRODUCES one of the three states
//! which by its name, or it is misnamed. It binds the PROSE as well as
//! the identifiers — a rename that leaves a doc sentence calling a
//! count a step has not landed.
//!
//! **Scope, and what enforces it.** The rule is this crate's; the
//! nearest violation outside it is `step-import`'s `STEPS: usize`
//! sample count. Nothing mechanical enforces either half today — the
//! guard this wants is a source-scraping row in the shape of this
//! crate's own `tests/all.rs`, which `include_str!`s its own source to
//! prove every test file is registered. Until that exists the rule is
//! held by review, which is a weaker thing than the sentence above
//! sounds.
//!
//! The one deliberate second spelling is `tess_meter::divisions`, in
//! the consumer half of the budget meter: a different cargo root, so
//! it cannot share this import, and it answers in `f64` and refuses
//! nothing because it sizes nothing. Its own docs state the
//! divergences from [`ceil_count`]; the different word is the tell
//! that it is a different function.
//!
//! # What is NOT here
//!
//! The *policy* — which target a schedule aims at, how much margin a
//! chart buys itself, when a schedule may be coarsened — is not stated
//! here, and this module does not state it. Each rule is argued at its
//! own site (`curved`'s per-chart grid, `nurbs_cert`'s per-band
//! schedule, `trimmed`'s refinement ladder), and this module unifies
//! only the *vocabulary* those arguments are written in.
//!
//! **One sizing question does have a ratified answer, and it is worth
//! knowing it is the only one**: `docs/TESS-BUDGET.md`'s *split
//! schedule's aspect policy* (2026-08-16, PR #568) rules the NURBS
//! split schedule's 3-D aspect cap at A = 16, and
//! `docs/TESS-SPLIT-SPEC.md` binds its execution. Both are scoped
//! entirely to `nurbs_cert`'s per-cell step derivation. Nothing covers
//! the analytic charts, the sizing target itself, or the retry and
//! refinement budgets.

use crate::types::TessellateError;

/// The call's tolerance bundle: δ (the promise), δ_s = δ/2 (sizing),
/// and the run's kernel ε.
///
/// # ε is never SIZING, and that is the checkable half
///
/// No step, count or schedule in this crate takes ε as an input.
/// [`sagitta_step`], [`ellipse_step`], [`curvature_step`],
/// [`torus_grid_step`], [`cap_angular`] and [`ceil_count`] are
/// functions of δ_s and geometry; `eps` appears nowhere in this module
/// but on the field below. That is the claim D9 and the memo-key
/// contract are read through, and it is checked by reading the
/// signatures rather than by trusting a sentence.
///
/// # What ε IS here is a bar, and the bars come in two kinds
///
/// **Neither kind SNAPS a value** — nothing in this crate replaces a
/// coordinate with a nearby one because ε says they are close.
///
/// - **Bars that only REFUSE or REPORT**, and cannot move a
///   coordinate whichever way they answer: [`crate::curved`]'s banded
///   swept-rectangle domain guard (refuses the face), the
///   [`crate::walk::gap_is_noise`] detectors (report and gate
///   nothing), and [`crate::trimmed`]'s per-triangle certificate
///   probe (asserts, and is absent from a default build).
/// - **Bars that CLASSIFY**, whose answer selects which `f64` an
///   emitted entry carries: pole/apex vertex identification in
///   [`crate::walk`], and that module's `iso_side_starts` run
///   grouping, which decides whether a traversal opens an iso side or
///   repeats its predecessor's coordinate bitwise.
///
/// So *"ε cannot move an emitted coordinate"* is FALSE as stated, and
/// the second kind is why. Pole identification substitutes the pole's
/// exact `v` for `Chart::v_of(p)` and emits TWO `pole: true` polygon
/// entries instead of one; both reach the UV polygon, hence the
/// bounding box, hence the interior grid and the pole fan's triangles.
/// `iso_side_starts` decides which of two analytically-equal columns a
/// side's entries carry.
///
/// **What is true is that nothing in the tree flips either
/// classification**: no in-tree body puts a non-pole vertex within any
/// of the suite's ε rows of a pole, and no swept junction has landed
/// at `0 < radial <= eps` (the sweep is recorded on
/// `walk::iso_side_starts`). Whether one is REACHABLE is not
/// established — `revolve` would very likely refuse such a sliver, and
/// a STEP import is the plausible route in — so the ε-dependence here
/// is structural and UNEXERCISED, which is not the same as absent or
/// unreachable. Mesh structure is a function of (body, δ) alone **for
/// every body this build can mint**, and that qualifier is the whole
/// difference between the memo-key contract and a theorem.
///
/// # Deliberately not a roster of read sites
///
/// A list of *where* ε is read would be the natural shape for this doc
/// and it is not used, because nothing can keep one true. Source
/// locations are guardable only by a source-text walk; the shared one
/// (`topo`'s `fixtures::code_only`) is `pub(crate)` and does not reach
/// this crate, and a private copy here would be the thirteenth
/// unshared such walk in the tree (`SMELL-SCAN-2026-08.md` S117). A
/// count also answers the wrong question: what a reader of D9 needs is
/// what a read may DO, which is the two kinds above, each argued at
/// its own site. `rg eps crates/mesh/src` is the
/// enumeration; this doc is the invariant.
pub(crate) struct Tol {
    /// The chordal tolerance δ.
    pub delta: f64,
    /// The sizing target δ_s = δ/2.
    pub delta_s: f64,
    /// The kernel ε. Never sizing; a bar of one of the two kinds
    /// above, at every site that reads it.
    pub eps: f64,
}

/// The sizing target δ_s for a call's chordal tolerance δ.
///
/// Every schedule in the crate sizes against this rather than against
/// δ itself; the halving is the documented safety factor that keeps
/// the two additive slacks outside the certificates (boundary vertices
/// on certified carriers, f64 rounding of the evaluations) from
/// deciding in practice — crate docs.
pub(crate) fn sizing_target(chordal: f64) -> f64 {
    chordal * 0.5
}

/// The cap on any *angular* step (π/4).
///
/// Two things rest on it, both structural rather than tuning: it keeps
/// the unwrapping-by-continuity of a periodic chart along the boundary
/// walk branch-unambiguous, and it makes a full-period rim
/// polygonalize with at least 8 chords rather than degenerate. It
/// therefore binds only where a step is an angle in a periodic
/// coordinate; a NURBS parameter step is uncapped.
pub(crate) const MAX_ANGULAR_STEP: f64 = core::f64::consts::FRAC_PI_4;

/// Sanity cap on any single count (δ small enough to exceed this would
/// allocate gigabytes before failing anywhere else).
const MAX_COUNT: f64 = 16_777_216.0; // 2^24

/// A parameter step capped at [`MAX_ANGULAR_STEP`]. A non-finite or
/// vacuous step takes the cap.
pub(crate) fn cap_angular(step: f64) -> f64 {
    if step < MAX_ANGULAR_STEP {
        step
    } else {
        MAX_ANGULAR_STEP
    }
}

/// The per-chord angular step for sagitta ≤ `delta_s` on a circle of
/// radius `rho`, capped at [`MAX_ANGULAR_STEP`]. Total (poison-free
/// for positive inputs): if δ_s ≥ ρ the sagitta constraint is vacuous
/// and the cap rules.
pub fn sagitta_step(delta_s: f64, rho: f64) -> f64 {
    if delta_s < rho {
        cap_angular(2.0 * (1.0 - delta_s / rho).acos())
    } else {
        MAX_ANGULAR_STEP
    }
}

/// The parameter step for chord deviation ≤ `delta_s` against a
/// second-derivative bound `m = sup‖C″‖` over the step: the standard
/// C² secant bound `deviation ≤ h²·m/8` inverted.
///
/// Uncapped, because the parameter it steps need not be an angle —
/// callers whose parameter is periodic apply [`cap_angular`].
pub(crate) fn curvature_step(delta_s: f64, m: f64) -> f64 {
    (8.0 * delta_s / m).sqrt()
}

/// The per-chord parameter step for chord deviation ≤ `delta_s` on an
/// ellipse with semi-axes `major > minor` (M5 PR 5), capped at
/// [`MAX_ANGULAR_STEP`].
///
/// Certified-conservative from [`curvature_step`]: over a parameter
/// span φ the arc length is `L ≤ major·φ` (`|dP/dθ| ≤ major`) and the
/// ellipse's maximum curvature is `κ_max = major/minor²` (at the major
/// vertices), so the effective second-derivative bound is
/// `R_eff = major·(major/minor)²`. Coarser than the circle's exact
/// sagitta near `major = minor` — conservative is the promised
/// direction.
pub fn ellipse_step(delta_s: f64, major: f64, minor: f64) -> f64 {
    let r_eff = major * (major / minor) * (major / minor);
    cap_angular(curvature_step(delta_s, r_eff))
}

/// The torus UV grid step `h = √(δ_s/(3(R+2r)))` — shared by the
/// curved-face grid sizing and the chord pass's adjacent-torus
/// tightening so boundary and interior steps agree.
///
/// Uncapped here, and its two consumers differ on that. The curved
/// lane steps a periodic chart coordinate with it directly and applies
/// [`cap_angular`] itself; the chord pass takes it only as a *lower
/// bound on a count* it has already sized from the circle sagitta, and
/// that sagitta step is capped over the same span. `h` exceeds the cap
/// only when `δ_s > (3π²/16)·(R + 2r) ≈ 1.85·(R + 2r)`, which is above
/// every circle radius a torus carries (`R + r` at most), so the
/// sagitta step is then exactly the cap and the capped and uncapped
/// requirements coincide. No in-tree body reaches that regime, so the
/// claim is pinned directly rather than by the mesh oracles:
/// `torus_cap_regime_is_sagitta_capped` below goes red if either
/// formula or [`MAX_ANGULAR_STEP`] moves.
///
/// The chord pass must NOT simply cap here to sidestep the argument:
/// [`ceil_count`] refuses a non-finite step typed, while
/// [`cap_angular`] turns one into the cap, so capping a poisoned torus
/// step there would swallow a refusal.
pub(crate) fn torus_grid_step(delta_s: f64, major: f64, minor: f64) -> f64 {
    (delta_s / (3.0 * (major + 2.0 * minor))).sqrt()
}

/// The torus boundary-step requirement `h` (crate docs) for a face's
/// surface, if that surface is a torus.
pub(crate) fn torus_step(surface: &geom::Surface<f64>, delta_s: f64) -> Option<f64> {
    match *surface {
        geom::Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => Some(torus_grid_step(delta_s, major_radius, minor_radius)),
        _ => None,
    }
}

/// `ceil(span/step)` as a chord/grid count, with the `MAX_COUNT` (2^24)
/// sanity cap surfaced as a typed error and a floor of 1.
///
/// # Errors
///
/// [`TessellateError::ResolutionOverflow`] when the count is
/// non-finite or at/above the cap.
pub fn ceil_count(span: f64, step: f64) -> Result<usize, TessellateError> {
    let raw = (span / step).ceil();
    if !(raw.is_finite() && raw < MAX_COUNT) {
        return Err(TessellateError::ResolutionOverflow { count: raw });
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Ok(if raw < 1.0 { 1 } else { raw as usize })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// [`torus_grid_step`]'s doc claim, which no meshing oracle
    /// reaches: above `delta_s = (3*pi^2/16)*(R+2r)` the torus step
    /// passes the angular cap, and there the sagitta step over the
    /// widest circle a torus carries (`R + r`) is EXACTLY the cap — so
    /// the chord pass's uncapped `max` and the curved lane's capped one
    /// agree. Red if either formula or the cap moves.
    #[test]
    fn torus_cap_regime_is_sagitta_capped() {
        for &(major, minor) in &[(1.0, 0.25), (5.0, 3.0), (0.5, 0.4), (100.0, 1.0)] {
            let threshold = 3.0 * core::f64::consts::PI.powi(2) / 16.0 * (major + 2.0 * minor);
            let delta_s = threshold * 1.0001;
            assert!(
                torus_grid_step(delta_s, major, minor) > MAX_ANGULAR_STEP,
                "the threshold no longer predicts the uncapped regime"
            );
            assert!(
                sagitta_step(delta_s, major + minor) == MAX_ANGULAR_STEP,
                "the sagitta step is not exactly the cap, so the two requirements diverge"
            );
            assert!(
                torus_grid_step(threshold * 0.9999, major, minor) <= MAX_ANGULAR_STEP,
                "the threshold no longer predicts the capped regime"
            );
        }
    }

    /// [`cap_angular`]'s documented total behaviour. The obvious-looking
    /// rewrite `if step > MAX { MAX } else { step }` is NOT equivalent —
    /// it returns NaN — and nothing else in the tree would notice.
    #[test]
    fn cap_angular_is_total() {
        assert!(cap_angular(f64::NAN) == MAX_ANGULAR_STEP);
        assert!(cap_angular(f64::INFINITY) == MAX_ANGULAR_STEP);
        assert!(cap_angular(MAX_ANGULAR_STEP) == MAX_ANGULAR_STEP);
        assert!(cap_angular(0.1) == 0.1);
        assert!(cap_angular(f64::NEG_INFINITY) == f64::NEG_INFINITY);
    }

    /// [`ceil_count`]'s documented edges at the boundary rather than
    /// through a whole tessellation (`tests/errors.rs` reaches
    /// `ResolutionOverflow` only end-to-end): the floor of 1, and the
    /// AT-the-cap refusal — `MAX_COUNT` itself refuses, it is not the
    /// last accepted value.
    #[test]
    fn ceil_count_floors_at_one_and_refuses_at_the_cap() {
        assert!(ceil_count(1.0, 10.0) == Ok(1));
        assert!(ceil_count(MAX_COUNT - 1.0, 1.0) == Ok(16_777_215));
        assert!(matches!(
            ceil_count(MAX_COUNT, 1.0),
            Err(TessellateError::ResolutionOverflow { .. })
        ));
        assert!(matches!(
            ceil_count(1.0, f64::NAN),
            Err(TessellateError::ResolutionOverflow { .. })
        ));
    }
}
