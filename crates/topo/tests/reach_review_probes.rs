//! Review probes for PR 7661 (the reach from the read point): poses the
//! unit's rows do not hold. Not a deliverable of the unit's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Point3, UnitVec3, Vec3};
use topo::boolean::trace::{TraceFault, ray_crossings_to};

fn floor() -> Surface<f64> {
    Surface::Plane {
        frame: geom::Frame::new(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
    }
}

/// A ray tilted `c = 7.5·10⁻⁹` off the plane `z = 0`, crossing it at the
/// middle of a run of 2. The curve × face pose reads the drift `c·run =
/// 1.5·10⁻⁸` definite and roots it; the crossing's sense at the reach
/// from the crossing, 1, is `7.5·10⁻⁹`, in band under `(10⁻⁹, 10⁻⁸)`,
/// while at the run (the old arm) it reads `1.5·10⁻⁸`, a definite
/// passage. The whole run stands within `7.5·10⁻⁹` of the plane.
#[test]
fn probe_trace_ray_crossing_mid_run_reads_the_reach_not_the_run() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let c = 7.5e-9_f64;
    let dir = Vec3::new((1.0 - c * c).sqrt(), 0.0, c);
    let start = Point3::origin() + dir * -1.0;
    let ray = geom::Ray::new(start, UnitVec3::squared(dir));
    let any =
        |cs: &[topo::boolean::trace::Crossing<f64>]| if cs.is_empty() { None } else { Some(0) };
    let got = ray_crossings_to(&floor(), true, &ray, 2.0, band, any);
    eprintln!("PROBE trace ray mid-run: {got:?}");
    assert!(
        matches!(got, Err(TraceFault::Escalated(_))),
        "the sense at the reach 1 is in band: got {got:?}"
    );
}

/// The same pose read by the sibling `ray_crossings` (the unconverted
/// whole-run door editor-core's trace marks read), its sense levered at
/// the caller's scene reach from the ray's start: it claims a passage on
/// a run that stands within `7.5·10⁻⁹` of the plane all along it.
#[test]
fn probe_trace_ray_crossings_sibling_claims_the_passage() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let c = 7.5e-9_f64;
    let dir = Vec3::new((1.0 - c * c).sqrt(), 0.0, c);
    let start = Point3::origin() + dir * -1.0;
    let ray = geom::Ray::new(start, UnitVec3::squared(dir));
    let got = topo::boolean::trace::ray_crossings(&floor(), true, &ray, 2.0, band);
    eprintln!("PROBE trace ray_crossings sibling: {got:?}");
    assert!(got.is_ok(), "the sibling reads a definite passage: {got:?}");
}

fn tilted_circle(psi: f64) -> geom::Curve3<f64> {
    geom::Curve3::Circle(geom::Circle::new(
        geom::Frame::new(
            Point3::origin(),
            Vec3::new(0.0, -psi.sin(), psi.cos()),
            Vec3::unit_x(),
        ),
        1.0,
    ))
}

/// **The graze is read from each Newton iterate, not from the seed.** A
/// unit circle tilted `ψ` about `x̂` meets `z = 0` at `θ = 0` at the sine
/// `sin ψ = 1.2·10⁻⁸`, inside its edge over `[−π/4, π/4]`, the sine at
/// `θ` being `sin ψ · cos θ`. Seeded at the edge's end `π/4`, the seed
/// reads `sin ψ · cos(π/4) · √2 = sin ψ` (Positive); the iterates close on
/// the root, whose reach is `2 sin(π/8) ≈ 0.77` (in band): the honest loop
/// serves a graze. Read from the seed throughout (reach `√2`), every
/// iterate is Positive and the root's own read escalates.
#[test]
fn probe_corner_graze_is_read_from_the_iterate_not_the_seed() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let carrier = tilted_circle(1.2e-8_f64.asin());
    let third = core::f64::consts::FRAC_PI_4;
    let got = topo::root_along(&floor(), &carrier, third, (&carrier, (-third, third)), band);
    eprintln!("PROBE corner graze from the iterate: {got:?}");
    assert!(
        matches!(got, Ok(Err(topo::CornerVerdict::Graze { .. }))),
        "the iterates near the root read the reach 1 in band: got {got:?}"
    );
}

/// **A line's graze is read at its root, not at the seed's point.** The
/// line meets `z = 0` at the sine `0.8·10⁻⁹` at the middle of its edge
/// over `[−1, 1]`; seeded at the edge's end, the root's reach (1) reads a
/// graze where the seed's (2) reads in band.
#[test]
fn probe_line_graze_is_read_at_the_root_not_the_seed() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let s = 0.8e-9_f64;
    let carrier = geom::Curve3::Line(geom::Line {
        origin: Point3::origin(),
        dir: UnitVec3::squared(Vec3::new((1.0 - s * s).sqrt(), 0.0, s)),
    });
    let got = topo::root_along(&floor(), &carrier, 1.0, (&carrier, (-1.0, 1.0)), band);
    eprintln!("PROBE line graze at the root: {got:?}");
    assert!(
        matches!(got, Ok(Err(topo::CornerVerdict::Graze { .. }))),
        "read at the root, a graze: got {got:?}"
    );
}
