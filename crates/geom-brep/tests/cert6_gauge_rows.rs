//! **CERT-6 direct rows** for the certified perimeter primitive and
//! the gauge arithmetic (issue 870).
//!
//! The delivered unit shipped a kernel assert with a comments-only
//! test diff: the panic itself is exercised by every fixture CI walks,
//! but the PRIMITIVE's arithmetic and the gauge's arm selection were
//! never asserted anywhere. These are those rows.
//!
//! Several are promoted from the review probe suites — the aliasing
//! row from R1's `r1_perimeter_probes` (which measured the defect the
//! fix pass repaired) and the collapsed-boundary balloon from R2's
//! `r2_cert6_probes`. Those files stay as the reviewers wrote them;
//! what is promoted here is the ASSERTION each of them motivated.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::props::quad::{RVec3, boundary_chord_perimeter_lo};
use geom_core::RingInterval;

fn p(x: f64, y: f64, z: f64) -> RVec3 {
    [
        RingInterval::point(x),
        RingInterval::point(y),
        RingInterval::point(z),
    ]
}

const UNIT: (f64, f64, f64, f64) = (0.0, 1.0, 0.0, 1.0);

/// **Two-sided anchor.** A flat unit square's boundary is four
/// straight edges, so every chord lies ON the curve and the bound is
/// the truth up to ring rounding. It must not OVERSTATE (that would
/// break the one-directional claim the gauge rests on) and must not
/// sag (that would mean the primitive is losing length on the easiest
/// possible input).
#[test]
fn cert6_flat_square_is_sound_and_tight() {
    let lo = boundary_chord_perimeter_lo(UNIT, 16, (&[], &[]), |u, v| p(u, v, 0.0));
    assert!(lo <= 4.0, "must never overstate the truth: {lo}");
    assert!(lo > 4.0 - 1e-9, "must not sag on straight edges: {lo}");
}

/// **Refinement is monotone**, which is what makes the schedule's
/// design a free choice: adding vertices to an inscribed polygon can
/// only lengthen it, so a richer schedule is never worse.
#[test]
fn cert6_bound_is_monotone_in_the_schedule() {
    let f = |u: f64, v: f64| {
        let t = std::f64::consts::TAU * u;
        p(t.cos(), t.sin(), v)
    };
    let mut last = f64::NEG_INFINITY;
    for n in [1usize, 2, 4, 8, 16, 64, 256] {
        let lo = boundary_chord_perimeter_lo(UNIT, n, (&[], &[]), f);
        assert!(
            lo >= last - 1e-12,
            "refining {n} must not shorten the polygon: {lo} < {last}"
        );
        last = lo;
    }
    // And it never overstates the closed traversal, which on this
    // wrapped patch crosses the seam twice.
    let truth = 2.0 * std::f64::consts::TAU + 2.0;
    assert!(last <= truth + 1e-9, "overstated the traversal: {last}");
}

/// **RED-FIRST, the fix pass's headline** (promoted from R1's
/// `r1_oscillatory_boundary_aliases_the_sixteen_chord_bound`).
///
/// The delivered schedule was a single uniform grid of 16 per side. A
/// `sin(2πku)` edge is sampled only at its zero crossings whenever
/// `k ≡ 0 mod 8`, and the polygon reads a straight line: at k = 8 it
/// returned **4.000** against a true 66.07, a 16.5x understatement.
/// The gauge divides by this number, so the ceiling shrinks as its
/// SQUARE — 273x, more than the entire calibrated margin — and an
/// understated denominator makes the tripwire fire on HONEST
/// geometry, in a release build.
///
/// The shipped schedule unions two coprime grids (and the knot lines,
/// and the block edges), so no single commensurate frequency can
/// collapse it. This row fails on the old schedule and passes on the
/// new one.
#[test]
fn cert6_coprime_schedule_does_not_collapse_at_commensurate_frequencies() {
    for k in [8.0f64, 16.0, 24.0, 32.0] {
        let f = move |u: f64, v: f64| {
            let bump = if v <= 0.0 || v >= 1.0 {
                (std::f64::consts::TAU * k * u).sin()
            } else {
                0.0
            };
            p(u, v, bump)
        };
        let lo = boundary_chord_perimeter_lo(UNIT, 16, (&[], &[]), f);
        // The flat reading is 4.0 — the aliased answer. Any honest
        // reading of an oscillating edge is far above it.
        assert!(
            lo > 20.0,
            "k={k}: the schedule collapsed to the flat reading ({lo}) — a commensurate \
             frequency is aliasing the chord polygon, which understates the gauge's \
             denominator and fires the tripwire on honest geometry"
        );
    }
}

/// **The bound stays one-directional under oscillation.** It may sit
/// well under the truth at frequencies past what any fixed schedule
/// resolves (Nyquist, not a schedule defect) — but it must never sit
/// ABOVE it, because that is the direction the gauge's claim depends
/// on.
#[test]
fn cert6_bound_never_overstates_an_oscillating_edge() {
    for k in [3.0f64, 8.0, 17.0, 34.0, 64.0] {
        let f = move |u: f64, v: f64| {
            let bump = if v <= 0.0 || v >= 1.0 {
                (std::f64::consts::TAU * k * u).sin()
            } else {
                0.0
            };
            p(u, v, bump)
        };
        let coarse = boundary_chord_perimeter_lo(UNIT, 16, (&[], &[]), f);
        let fine = boundary_chord_perimeter_lo(UNIT, 4096, (&[], &[]), f);
        assert!(
            coarse <= fine + 1e-9,
            "k={k}: the coarse bound {coarse} exceeded the refined one {fine}"
        );
    }
}

/// **A sub-ulp face returns zero, not a negative length.** The ring
/// sum's lower endpoint rounds outward, and on a 1e-12 face at offset
/// 1e6 the delivered primitive returned −3.16e-322. A negative
/// denominator is not a length; the primitive clamps, and the gauge
/// then routes to its relative arm rather than dividing by it.
#[test]
fn cert6_sub_ulp_face_clamps_to_zero() {
    for (size, offset) in [(1e-12, 1e6f64), (1e-12, 1e9), (1e-15, 1e9)] {
        let f = move |u: f64, v: f64| p(offset + size * u, offset + size * v, 0.0);
        let lo = boundary_chord_perimeter_lo(UNIT, 16, (&[], &[]), f);
        assert!(
            lo >= 0.0,
            "size={size:e} offset={offset:e}: a length bound went negative ({lo:e})"
        );
    }
    // And a face the arithmetic CAN resolve still reports a real one.
    let f = |u: f64, v: f64| p(1e-3 * u, 1e-3 * v, 0.0);
    let lo = boundary_chord_perimeter_lo(UNIT, 16, (&[], &[]), f);
    assert!(
        lo > 3.99e-3,
        "a resolvable small face must still measure: {lo:e}"
    );
}
