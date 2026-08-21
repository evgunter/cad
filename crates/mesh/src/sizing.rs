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
//! prove every test file is registered — and, since the ε ledger,
//! walks `crates/mesh/src` through `test_utils::source::code_only`
//! (`the_eps_inventory_is_pinned`). **The shape exists; nobody has
//! written this rule's row in it.** Until someone does, the rule is
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
/// and the run's kernel ε — fetched once by [`fn@crate::tessellate`] and
/// carried here, never re-read from `Tolerance`.
///
/// # What ε may and may not do here
///
/// **No sizing function takes it as an argument**: `eps` appears
/// nowhere in this module but on the field below, and no step or grid
/// rule in the crate has it in its signature. **A count can still
/// move**, which is why that sentence is about arguments —
/// `curved::pole_columns(nu, has_pole)` returns 3 where it would
/// return 2, and `has_pole` is a bit an ε comparison in
/// [`crate::walk`] sets. ε is upstream of one argument of one rule.
///
/// **No consumer SNAPS a value** (the loop-closure snap that did is
/// gone, S22). What a read does instead — refuse a face, report into a
/// `debug_assert`, classify a vertex, scale a measurement — **is
/// stated at the read**. This doc holds neither a roster of the reads
/// nor a partition of their kinds: the roster is computed (below), and
/// a partition is a count by another name, which is the thing that
/// went stale here.
///
/// A CLASSIFYING read makes *"ε cannot move an emitted coordinate"*
/// FALSE as stated — pole identification substitutes the pole's exact
/// `v` and emits two entries instead of one; `walk::iso_side_starts`
/// picks which of two analytically-equal columns a side carries. Each
/// argues its own reachability where it is; what belongs here is the
/// summary: **nothing in the tree flips one, and whether anything
/// COULD is not established** (a STEP import is the plausible route
/// in). The dependence is structural and UNEXERCISED, which is not the
/// same as absent, and it is why *"mesh structure is a function of
/// (body, δ)"* is a statement about the tree rather than a theorem.
///
/// A read can also be no bar at all: [`crate::trimmed`]'s deviation
/// probe computes `d / (bound + eps)`, so ε moves the `worst_ratio` it
/// publishes at **every** call, monotonically. No mesh coordinate
/// moves; a reported number does.
///
/// **Where the reads are is not written down here** — it is pinned by
/// `mesh/tests/all.rs`'s `the_eps_inventory_is_pinned`, which counts ε
/// identifiers per file and reds when one lands. A list here could
/// not, and one here was short by a read for two milestones. That walk
/// is textual and cannot see a read spelled another way; the mechanism
/// that would (ε as a type whose operations are named, D2 addendum
/// row 0) is **issue #881**.
pub(crate) struct Tol {
    /// The chordal tolerance δ.
    pub delta: f64,
    /// The sizing target δ_s = δ/2.
    pub delta_s: f64,
    /// The kernel ε. No sizing rule takes it; see above for what its
    /// reads may do, and `the_eps_inventory_is_pinned` for where they
    /// are.
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
