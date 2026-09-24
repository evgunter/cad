//! **The reporting door's bits** — `topo::mass_properties` over a
//! roster that reaches every certified-quadrature lane, pinned as raw
//! `f64` bit patterns.
//!
//! The measurement door's result is a NUMBER a consumer stores,
//! exports and compares; a quadrature refactor that leaves it "close
//! enough" has changed it. So this row asserts the four fields of
//! [`topo::MassProperties`] bit for bit, per ε row, over a roster
//! chosen so that **every lane the certified quadrature has is reached
//! by a body whose answer DEPENDS on ε** — a lane covered only by
//! ε-invariant rows is a lane this digest does not pin:
//!
//! | body | lane it reaches | ε-coupled? |
//! |---|---|---|
//! | square prism loft | the patch engine's EXACT per-span arm (v-degree 1, inside the rule window) | no — the exact arm's enclosure is ring rounding only, so the same bits at every ε; it pins the arm's answer and the accept decision |
//! | quintic prism loft | the patch engine's COMPOSITE rounds (v-degree 5, outside the rule window) | yes — the round it stops at is the round that clears `1024·ε` |
//! | arc prism loft (identical sections) | the RATIONAL patch quotient, `w` uniform in `v` | yes — refuses on budget at 1e-12, certifies at 1e-9 and 1e-6 at different rounds |
//! | arc loft (sections of differing scale) | the rational quotient, varying in `v` (no exact-v arm) | yes — same |
//! | tilted cylinder cut (upper part) | the CYLINDER chart's Green form over ellipse trim carriers | yes — the harmonic composite's round is the round that clears `1024·ε` |
//! | bulged extrusion | closed forms — an analytic cylinder wall with an ISO boundary is not a cut face | no |
//! | box extrusion | closed forms only (both pads exactly `0`) | no |
//!
//! **What no body here reaches** is stated rather than left to be
//! discovered: `geom-brep`'s B-spline Green lane
//! (`bspline_green_integral`) has no at-rest consumer at all, and the
//! cone/sphere/torus charts refuse the quadrature typed. Nothing in
//! this tree can build a body that takes either, so their absence is
//! the kernel's frontier and not this roster's gap.
//!
//! **The tour's bodies** are the loft and cut classes above: the
//! tour's skinned stop is `step-export`'s `loft_prism`, a unit-weight
//! NURBS loft on the exact per-span arm, and its cut scenes are the
//! cylinder Green form. Both classes are here. The tour's OWN scenes
//! are not, and cannot be: `demos/tour` is a separate cargo root that
//! this crate may not depend on, and a digest that lived there would
//! not run in the test matrix at all.
//!
//! **A refusal is a row too.** At a tight ε the fixed schedule
//! honestly runs out on a rational wall, and the refusal's own
//! `width_len`/`target_len` are as much the door's output as a volume
//! is — so a refusing body records its typed refusal rather than
//! being skipped, and a body that starts computing where it used to
//! refuse reds this row. The refusal is recorded in its DEBUG form,
//! which carries the payload fields (`width_len`, `target_len`,
//! `rounds`) rather than the prose the `Display` wraps them in.
//!
//! **Not a claim that these numbers are RIGHT.** Accuracy is
//! `m8_3_rational_volume`'s and `m7_skin_integral`'s job, against
//! independent oracles. This row's only claim is that they do not
//! MOVE — and when one legitimately does, the repair is to re-cut the
//! table here and say in the PR what moved and why, never to loosen
//! the comparison.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{
    arc_prism, arc_section, bulged_extrusion, quintic_prism, square_prism, stacked,
    tilted_cut_upper,
};
use geom_core::Tol;
use sweep::loft_body;
use topo::Body;

/// One roster row's digest line: the body's name, then either the four
/// `MassProperties` fields as hex bit patterns or the door's typed
/// refusal.
fn digest_line(name: &str, body: &Body<f64>) -> String {
    match topo::mass_properties(body, Tol::witness()) {
        Ok(m) => format!(
            "{name} v={:016x} a={:016x} vpad={:016x} apad={:016x}",
            m.volume.to_bits(),
            m.surface_area.to_bits(),
            m.volume_pad.to_bits(),
            m.area_pad.to_bits(),
        ),
        Err(e) => format!("{name} REFUSED {e:?}"),
    }
}

/// The tapered arc loft: sections of DIFFERING scale, so the rational
/// wall genuinely varies in `v` (the `w`-uniform-in-v exact arm does
/// not apply).
fn arc_taper() -> Body<f64> {
    loft_body::<f64>(
        &[arc_section(1.0), arc_section(1.25), arc_section(1.0)],
        &stacked(&[0.0, 1.0, 2.0], 1.0),
        2,
        Tol::witness(),
    )
    .expect("the tapered arc loft lofts")
    .body
}

/// The box: every face closed-form, both pads exactly zero.
fn box_extrusion() -> Body<f64> {
    sweep::test_support::prism::<f64>(
        sweep::test_support::corners(&[(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        2.0,
        Tol::witness(),
    )
}

/// The roster, in a fixed order (the digest is compared as one block,
/// so the order is part of the pin).
fn roster() -> String {
    let rows = [
        ("square_prism", square_prism()),
        ("quintic_prism", quintic_prism()),
        ("arc_prism", arc_prism()),
        ("arc_taper", arc_taper()),
        ("tilted_cut_upper", tilted_cut_upper()),
        ("bulged_extrusion", bulged_extrusion()),
        ("box_extrusion", box_extrusion()),
    ];
    rows.iter()
        .map(|(name, body)| digest_line(name, body))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The committed digest for each ε row the matrix gates.
///
/// Cut on the MERGE BASE, not on the branch under test, at the three ε
/// the CI matrix runs, and then run here. That order is the whole
/// instrument: a row cut on the branch would record what the branch
/// does, which is the thing under test. An ε with no entry here prints
/// its block and fails, which is how a new row gets cut.
///
/// **When a lane may re-cut here instead, and what licenses it.** The
/// merge-base rule exists to stop a branch recording its own
/// regression as the baseline — not to make a pinned number a
/// contract. A branch re-cuts on itself exactly when its own change is
/// what moved the table AND the new reading is the right answer, with
/// the cause named at the cut: `work/scalar/H5.md` ruling 2 (a
/// certified bound that gets tighter re-baselines like any other move)
/// and `memories/output-stability-as-justification.md`. Anything else
/// — a move the branch cannot explain, or one in the wrong direction —
/// is a finding, and the table stays where it is.
///
/// **Re-cut at all three ε when the C9 ring became a newtype over
/// `interval-transcendentals`' `DInterval`.** That is the other repair
/// the assertion below names: the ring padded one representable step
/// outward on every operation and the backend pads only where the
/// operation is inexact, so every `vpad`/`apad` in the table shrank
/// and none grew, and the five ε-coupled rows' `v`/`a` midpoints moved
/// with them. Every verdict hash in the block is unchanged — nothing
/// certified that refused, or refused that certified — and the pads
/// are the whole of what moved, downward.
fn expected(eps: f64) -> Option<&'static str> {
    match eps {
        1e-6 => Some(include_str!("reporting-door-digest/eps-1e-6.txt")),
        1e-9 => Some(include_str!("reporting-door-digest/eps-1e-9.txt")),
        1e-12 => Some(include_str!("reporting-door-digest/eps-1e-12.txt")),
        _ => None,
    }
}

#[test]
fn the_reporting_doors_bits_are_unchanged() {
    let eps = Tol::witness().get().eps;
    let got = roster();
    let Some(want) = expected(eps) else {
        panic!("no committed digest for eps={eps:e}; this run produced\n{got}");
    };
    assert_eq!(
        got.trim(),
        want.trim(),
        "the reporting door's bits moved at eps={eps:e}. Decide whether the new \
         numbers are RIGHT (they are not a baseline to preserve): if they are, \
         re-cut this table and say in the PR what moved and why."
    );
}
