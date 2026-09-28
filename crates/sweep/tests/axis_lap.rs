//! **Box cuts of a cylinder whose cutter face meets a cap along a chord
//! with ONE rim arc between its ends.** The chord's two ends are then
//! adjacent on the cap's loop, and the plane×plane join lane asks the
//! rim arc between them whether it IS the section segment — a conic
//! edge on a planar face, answered against the partner germ plane.
//! It never is (a circle or ellipse arc meets that plane only at its
//! ends), so the chord is minted.
//!
//! The rod is `r = 0.5` about `z` over `z ∈ [0, 4]`, an extruded
//! circle: two semicircles meeting at `(±0.5, 0)`, so its wall carries
//! two ruling edges at `x = ±0.5, y = 0`. Every cutter spans `x ∈ [−1,
//! 1]` unless it says otherwise. What each pose does:
//!
//! - a FULL-LENGTH flat (the cutter past both caps) builds, certifies,
//!   and has the analytic volume, at every depth — through the axis,
//!   off it, and from either side;
//! - a LAP (the cutter from `z = 3` past the far cap, so one end wall
//!   sits inside the rod) off the axis, or through the axis across the
//!   rulings (`x = 0`), refuses `CurvedSectorSideUnsupported` — the
//!   first-order sector-side frontier;
//! - a lap in the plane `y = 0`, which holds both ruling edges, refuses
//!   `Join(UnpairedLooseEnds)` — and so does the all-planar diamond
//!   prism whose side edges sit in that same plane, which is what says
//!   the refusal is the edge-in-face class
//!   (`work/zip/an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`),
//!   not anything conic;
//! - OBLIQUE caps (ellipse rims, from the plane split) flatted the same
//!   way take the same arm with an ellipse arc, mint their chords, and
//!   refuse one door later, where the containment door cannot measure
//!   an obliquely trimmed wall in closed form.
//! - a flat cutter with a thin half-rod on the axis leaves role
//!   resolution only the rim's CHORD midpoint to probe, which is on
//!   neither flanking region, and the join refuses `SectionLoopMixed`
//!   (`work/zip/role-resolution-interior-tiers-certify-only-planar-region-faces`);
//! - a blind D pocket in a block builds from the bottom face (its floor's
//!   chord has the D's arc between its ends) and refuses `JoinDesync`
//!   from the top
//!   (`work/zip/blind-d-pocket-subtract-refuses-with-join-internal-words`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::brick;
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError, SplitJoinError};

const R: f64 = 0.5;
const LEN: f64 = 4.0;

fn tol() -> Tol {
    Tol::witness()
}

fn extruded(plane: SketchPlane<f64>, lp: profile::ProfileLoop<f64>, h: f64) -> Body<f64> {
    let profile = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    extrude(&profile, Extrusion::Distance(h), tol())
        .unwrap()
        .body
}

fn polygon(pts: &[(f64, f64)]) -> profile::ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect())
}

/// The rod: an extruded circle.
fn rod() -> Body<f64> {
    let disc = profile::circle(Point2::new(0.0, 0.0), R, tol()).unwrap();
    extruded(SketchPlane::xy(), disc.into(), LEN)
}

/// The rod's all-planar twin: a square turned 45°, its corners where
/// the rod's semicircles meet, so its side edges `x = ±0.5, y = 0` are
/// the rod's rulings.
fn diamond() -> Body<f64> {
    extruded(
        SketchPlane::xy(),
        polygon(&[(R, 0.0), (0.0, R), (-R, 0.0), (0.0, -R)]),
        LEN,
    )
}

fn cut(
    a: &Body<f64>,
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
) -> Result<Body<f64>, BooleanError> {
    topo::subtract(a, &brick(x, y, z, tol()), tol())
        .map(|r| r.body().expect("a body remains").body.clone())
}

/// The lap: the cutter starts inside the rod at `z = 3` and runs past
/// the far cap.
const LAP: (f64, f64) = (3.0, 4.5);
/// The full-length flat: the cutter runs past both caps.
const FLAT: (f64, f64) = (-1.0, 5.0);
const ACROSS: (f64, f64) = (-1.0, 1.0);

/// The area of the disc segment `y ≥ d` of radius `R`.
fn segment(d: f64) -> f64 {
    R * R * (d / R).acos() - d * (R * R - d * d).sqrt()
}

/// The body certifies at rest and has `expect` for its volume.
fn assert_sound(body: &Body<f64>, expect: f64, what: &str) {
    topo::validate_geometric_certificate(body, tol())
        .unwrap_or_else(|e| panic!("{what}: the result does not certify at rest: {e:?}"));
    let v = topo::mass_properties(body, tol()).unwrap().volume;
    assert!(
        (v - expect).abs() < 1e-9,
        "{what}: volume {v} against the analytic {expect}"
    );
}

/// **The row's own pose, and its mirror.** The cap chord lies on the
/// diameter between the semicircles' shared vertices, so ONE
/// semicircle lies between its ends; the join lane answers that it
/// bellies off the cutter's plane and mints the chord. What refuses
/// next is the edge-in-face class, which the planar diamond in the same
/// pose refuses identically.
#[test]
fn axis_lap_refuses_where_its_planar_twin_does() {
    for y in [(0.0, 1.0), (-1.0, 0.0)] {
        for (name, body) in [("rod", rod()), ("diamond", diamond())] {
            let err = cut(&body, ACROSS, y, LAP).expect_err("the edge-in-face lap refuses");
            assert!(
                matches!(
                    err,
                    BooleanError::Join(SplitJoinError::UnpairedLooseEnds { count: 4 })
                ),
                "{name} lap at y ∈ {y:?}: {err:?}"
            );
        }
    }
}

/// Laps off the rulings: the cutter's end wall crosses the rod's wall
/// inside a face, and that crossing's sector side is the frontier — the
/// plane through the axis at `x = 0` included, so the axis alone is not
/// what the lap above refuses on.
#[test]
fn laps_off_the_rulings_refuse_sector_side() {
    for (x, y) in [
        (ACROSS, (0.2, 1.0)),
        (ACROSS, (0.35, 1.0)),
        ((0.0, 1.0), ACROSS),
        ((-1.0, 0.0), ACROSS),
    ] {
        let err = cut(&rod(), x, y, LAP).expect_err("the lap refuses");
        assert!(
            matches!(err, BooleanError::CurvedSectorSideUnsupported { .. }),
            "lap at x ∈ {x:?}, y ∈ {y:?}: {err:?}"
        );
    }
}

/// Full-length flats at every depth: the cap chord has one rim arc
/// between its ends at each cap, and each is minted. The result is the
/// rod less the segment prism.
#[test]
fn full_length_flats_build_at_the_analytic_volume() {
    let rod_v = PI * R * R * LEN;
    for (x, y, d) in [
        (ACROSS, (0.0, 1.0), 0.0),
        (ACROSS, (-1.0, 0.0), 0.0),
        (ACROSS, (0.2, 1.0), 0.2),
        (ACROSS, (0.35, 1.0), 0.35),
        ((0.0, 1.0), ACROSS, 0.0),
    ] {
        let body =
            cut(&rod(), x, y, FLAT).unwrap_or_else(|e| panic!("x ∈ {x:?}, y ∈ {y:?}: {e:?}"));
        assert_sound(
            &body,
            rod_v - segment(d) * LEN,
            &format!("flat at x ∈ {x:?}, y ∈ {y:?}"),
        );
    }
}

/// **The ellipse carrier on the same arm.** The rod is split by the
/// planes `z = 0.5 + tan θ · y` and `z = 3.5 + tan θ · y` (tilted `θ`
/// about `x`) and the part between them kept: both caps are planar
/// faces bounded by ellipse arcs meeting at the rulings. The flat
/// `y ≥ 0.2` cuts each cap along a chord with one ellipse arc between
/// its ends, so every conic that arm meets is an ellipse.
///
/// The arm mints both chords; the result then refuses
/// `Containment(VolumeUncertified)`, because the containment door's
/// orientation probe measures the body in closed form and an
/// obliquely trimmed wall has none — the capability the volume
/// backstop lacks on the same shapes
/// (`work/reach/union-with-a-tilted-cylinder-boss-refuses-as-classification-invariant`).
#[test]
fn an_oblique_cap_flats_through_its_ellipse_arc() {
    let theta = 20f64.to_radians();
    let normal = Vec3::new(0.0, -theta.sin(), theta.cos());
    let part = |body: &Body<f64>, z0: f64, above: bool| -> Body<f64> {
        let split = topo::split(
            body,
            &topo::SplitPlane {
                origin: Point3::new(0.0, 0.0, z0),
                normal,
            },
            tol(),
        )
        .expect("the oblique split runs");
        let topo::SplitPart::Body(kept) = (if above { split.above } else { split.below }) else {
            panic!("the rod has material on the kept side of z0 = {z0}");
        };
        kept
    };
    let capped = part(&part(&rod(), 3.5, false), 0.5, true);
    // Between two parallel planes 3 apart along z, over the disc.
    assert_sound(&capped, 3.0 * PI * R * R, "the oblique-capped rod");
    let err = cut(&capped, ACROSS, (0.2, 1.0), FLAT).expect_err("the flat refuses");
    assert!(
        matches!(
            err,
            BooleanError::Containment(topo::PointInSolidError::VolumeUncertified)
        ),
        "{err:?}"
    );
}

/// **Role resolution's chord-midpoint probe is unsound on a curved
/// edge.** The cutter is the half-space `y ≥ 0` over the rod's whole
/// length, with a thin half-rod (`r = 0.1`) on the axis, bulging either
/// way. Every rod vertex sits on the cutter's boundary, so role
/// resolution falls to its `Anchor::EdgeMidpoint` chord midpoints, and
/// a rim semicircle's is the circle's centre — a point on neither
/// flanking region, whose one verdict both loops then take. The join
/// refuses `SectionLoopMixed` rather than resolving. Pinned at that
/// outcome; the fix is
/// `work/zip/role-resolution-interior-tiers-certify-only-planar-region-faces`.
#[test]
fn a_chord_midpoint_probe_reads_both_loops_alike() {
    for bulge in [1.0, -1.0] {
        let cutter = extruded(
            SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -1.0))),
            bulge_loop(vec![
                (Point2::new(-1.0, 0.0), 0.0),
                (Point2::new(-0.1, 0.0), bulge),
                (Point2::new(0.1, 0.0), 0.0),
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(1.0, 1.0), 0.0),
                (Point2::new(-1.0, 1.0), 0.0),
            ]),
            6.0,
        );
        let r = topo::subtract(&rod(), &cutter, tol());
        assert!(
            matches!(
                r,
                Err(BooleanError::Join(SplitJoinError::SectionLoopMixed { .. }))
            ),
            "bulge {bulge}: {:?}",
            r.err()
        );
    }
}

/// The block `[−1, 1]² × [0, 1]` minus a D-profile rod (chord `x = 0.3`,
/// major arc `r = 0.5` about the origin) extruded `1.0` from `z = z0`.
fn d_pocket(z0: f64) -> Result<Body<f64>, BooleanError> {
    let block = extruded(
        SketchPlane::xy(),
        polygon(&[(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        1.0,
    );
    let c = sweep::test_support::rod_chord_at(0.3);
    let d = extruded(
        SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0))),
        bulge_loop(vec![
            (Point2::new(0.3, c.half), c.wall_bulge),
            (Point2::new(0.3, -c.half), 0.0),
        ]),
        1.0,
    );
    topo::subtract(&block, &d, tol()).map(|r| r.body().expect("a body remains").body.clone())
}

/// **A blind D pocket, from either face.** Entering through the BOTTOM
/// face, the block's floor meets the D's flat wall along a chord whose
/// ends are adjacent on the floor's new ring with the major arc between
/// them — the plane×plane arm's conic question — and the pocket builds:
/// it certifies at rest and its volume is the block less the D's area
/// over the pocket's depth `0.5`. Entering through the TOP face it
/// refuses `JoinDesync` in the join's internal words (the ring-run
/// winding decides `Zero`), which
/// `work/zip/blind-d-pocket-subtract-refuses-with-join-internal-words`
/// carries.
#[test]
fn a_blind_d_pocket_builds_from_below_and_refuses_from_above() {
    let bottom = d_pocket(-0.5).expect("the bottom-entry pocket builds");
    assert_sound(
        &bottom,
        4.0 - (PI * R * R - segment(0.3)) * 0.5,
        "the bottom-entry D pocket",
    );
    let err = d_pocket(0.5).expect_err("the top-entry pocket refuses");
    assert!(
        matches!(
            err,
            BooleanError::JoinDesync {
                what: "ring-run winding is degenerate (zero enclosed area)"
            }
        ),
        "{err:?}"
    );
}
