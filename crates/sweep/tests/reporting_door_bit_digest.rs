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

use crate::common::{arc_section, quad, stacked};
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::loft_body;
use topo::Body;
use topo::splitting::{SplitPart, SplitPlane, split};

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

/// The square prism: planar caps and four degree-1 walls, which the
/// patch engine answers on its exact per-span arm.
fn square_prism() -> Body<f64> {
    loft_body::<f64>(
        &[
            quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
            quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        ],
        &stacked(&[0.0, 2.0], 1.0),
        1,
        Tol::witness(),
    )
    .expect("the square prism lofts")
    .body
}

/// The quintic prism: six identical square sections at v-degree 5,
/// which is outside the exact per-span rule window (> 4 per
/// direction), so its walls take the patch engine's COMPOSITE rounds
/// and the round they stop at is an ε question.
fn quintic_prism() -> Body<f64> {
    // The scale is chosen so the committed rows DIFFER across the
    // matrix, which is the whole point of this member: the composite's
    // starting width lands between the 1e-12 target and the 1e-9 one,
    // so this body refuses on budget at the tight ε and certifies at
    // the other two. A metre-scale twin certifies at round 0 at every
    // ε and would pin the lane's answer without pinning its schedule;
    // a kilometre-scale one refuses at two ε and takes 225 s of debug
    // quadrature at the third.
    let s = 0.1;
    // The sections must DIFFER, or the walls are flat: a flat patch's
    // second derivatives are zero, the composite's remainder vanishes
    // and round 0 certifies at ring-rounding width whatever ε is.
    let sq = |k: f64| {
        quad([
            (-k * s, -k * s),
            (k * s, -k * s),
            (k * s, k * s),
            (-k * s, k * s),
        ])
    };
    loft_body::<f64>(
        &[sq(1.0), sq(1.05), sq(1.15), sq(1.3), sq(1.5), sq(1.75)],
        &stacked(&[0.0, 0.4, 0.8, 1.2, 1.6, 2.0], s),
        5,
        Tol::witness(),
    )
    .expect("the quintic prism lofts")
    .body
}

/// The tilted cylinder cut, upper part: a cylinder split by a plane at
/// `φ = 0.3`, whose wall pieces are bounded by exact `Ellipse`
/// carriers — the CYLINDER chart's Green form, which no loft or sweep
/// verb can produce (their walls carry iso boundaries and take the
/// closed forms).
fn tilted_cut_upper() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-0.5, 0.0), 1.0),
        ProfileVertex::new(Point2::new(0.5, 0.0), 1.0),
    ]);
    let disc = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the disc profile validates");
    let cylinder = sweep::extrude::<f64>(&disc, sweep::Extrusion::Distance(1.0), Tol::witness())
        .expect("the cylinder extrudes")
        .body;
    let phi = 0.3f64;
    let result = split(
        &cylinder,
        &SplitPlane {
            origin: Point3::new(0.0, 0.0, 0.5),
            normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
        },
        Tol::witness(),
    )
    .expect("the tilted cut splits");
    let SplitPart::Body(above) = result.above else {
        panic!("both sides of the tilted cut carry material");
    };
    above
}

/// The arc prism: three identical bulged sections, so every wall is a
/// RATIONAL patch and the quotient composite answers.
fn arc_prism() -> Body<f64> {
    loft_body::<f64>(
        &[arc_section(1.0), arc_section(1.0), arc_section(1.0)],
        &stacked(&[0.0, 1.0, 2.0], 1.0),
        2,
        Tol::witness(),
    )
    .expect("the arc prism lofts")
    .body
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

/// The bulged extrusion: an analytic cylinder wall with a CURVED trim
/// loop — the cylinder chart's Green form.
fn bulged_extrusion() -> Body<f64> {
    let prof = Profile::new(SketchPlane::xy(), arc_section(1.0))
        .validate(Tol::witness())
        .expect("the profile validates");
    sweep::extrude::<f64>(&prof, sweep::Extrusion::Distance(2.0), Tol::witness())
        .expect("extrude")
        .body
}

/// The box: every face closed-form, both pads exactly zero.
fn box_extrusion() -> Body<f64> {
    let prof = Profile::new(
        SketchPlane::xy(),
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
    )
    .validate(Tol::witness())
    .expect("the profile validates");
    sweep::extrude::<f64>(&prof, sweep::Extrusion::Distance(2.0), Tol::witness())
        .expect("extrude")
        .body
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
/// Cut on the MERGE BASE — never on this branch — at the three ε the
/// CI matrix runs, and then run here. That order is the whole
/// instrument: a row cut on the branch would record what the branch
/// does, which is the thing under test. An ε with no entry here prints
/// its block and fails, which is how a new row gets cut.
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
