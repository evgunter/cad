//! **The reporting door's bits** — `topo::mass_properties` over a
//! roster that reaches every certified-quadrature lane, pinned as raw
//! `f64` bit patterns.
//!
//! The measurement door's result is a NUMBER a consumer stores,
//! exports and compares; a quadrature refactor that leaves it "close
//! enough" has changed it. So this row asserts the four fields of
//! [`topo::MassProperties`] bit for bit, per ε row, for one body of
//! each lane the door can take:
//!
//! | body | lane |
//! |---|---|
//! | square prism loft | planar caps + the patch engine's EXACT per-span arm |
//! | arc prism loft (identical sections) | the RATIONAL patch quotient |
//! | arc loft (sections of differing scale) | the rational quotient, varying in `v` |
//! | bulged extrusion | the cylinder chart's Green form |
//! | box extrusion | closed forms only (both pads exactly `0`) |
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
use geom_core::Tol;
use profile::{Profile, SketchPlane};
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
        ("arc_prism", arc_prism()),
        ("arc_taper", arc_taper()),
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
/// Cut on `main` before the sign-certified +V work, at the three ε the
/// CI matrix runs. An ε with no entry here prints its block and fails,
/// which is how a new row gets cut.
fn expected(eps: f64) -> Option<&'static str> {
    match eps {
        e if e == 1e-6 => Some(include_str!("reporting-door-digest/eps-1e-6.txt")),
        e if e == 1e-9 => Some(include_str!("reporting-door-digest/eps-1e-9.txt")),
        e if e == 1e-12 => Some(include_str!("reporting-door-digest/eps-1e-12.txt")),
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
