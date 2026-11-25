//! **A circle lying in a face, closed or on a torus**
//! (`crates/topo/README.md`, tier 3): where both of an edge's faces
//! leave its line into the material of a face of the other operand, the
//! material about the line is two wedges, and the Boolean parts the
//! edge into two coincident arcs, recorded as an edge–edge pair. A
//! closed circle inside a face divides it: the disc inside the circle is
//! a face of its own, bounded by one of the two circles.
//!
//! - **A closed rim in a plane face divides it.** The block
//!   `[−2, 2] × [0, 1] × [−2, 2]` less the full revolve about `y` of the
//!   quadrilateral `(.5, .5), (1, .5), (1, 1), (.6, .6)`: its top vertex
//!   sweeps a closed circle of radius 1 in the block's top face `y = 1`,
//!   and the revolve's two faces at it both leave it into the block's
//!   material. Likewise the slab `[−3, 3] × [−1, 0] × [−3, 3]` less the
//!   full revolve of the triangle `(1, 0), (.5, −.5), (1.5, −.5)`, its
//!   apex sweeping a circle in the slab's top face.
//! - **A rim on a sphere divides its face.** The ball of radius `√2`
//!   about the origin, its seam meridians in the `xz` plane, less the
//!   full revolve about `y` of a triangle whose apex sweeps the circle of
//!   angular radius 0.4 about `y` on the sphere, both its cones going
//!   into the ball: the keel as built, turned 0.3 about `x` and 1 about
//!   `z`, the rim inside the `y > 0` lune, a closed circle there; turned
//!   1.2 about `x`, the rim across both seam meridians, three arcs over
//!   two lunes (the keel's own vertex one of their ends), each lune
//!   divided. The same keel turned 1.2 about `x` in the ball revolved
//!   about `y`, whose one sphere face wraps its seam meridian, holds the
//!   rim off the seam; turned 0.3, across it.
//! - **A rim whose vertex lies on a wrap edge refuses `Shared`.** The
//!   tube of radii 1 and 2 about `y` less the full revolve of the
//!   triangle `(1, .5), (1.2, .7), (1.2, .3)`, its apex sweeping a closed
//!   circle in the bore's cylinder; the solid torus of radii 3 and 1
//!   about `y` less the full revolve of a triangle whose apex `(3, 1)`
//!   sweeps the torus's top parallel; and the torus of radii 2 and 1 less
//!   the puck `ρ ≤ 2.5`, `y ∈ [0, √3/2]`, whose top rim is the parallel
//!   at tube angle `π/3`, the puck as built and turned 1 rad; and a
//!   quarter turn of the top-parallel keel, an open arc. The rim's vertex
//!   lies on the face's wrap edge, so the end joins a vertex the face's
//!   loop passes twice.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, SQRT_2};

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{ball_poled_z, brick, extruded, finished, revolved_about_y, sketch_at};
use topo::{AtRestBody, BooleanError, BooleanResult, EdgeInFaceGap, subtract};

fn tol() -> Tol {
    Tol::witness()
}

fn revolved(what: &str, pts: &[(f64, f64)], rev: Revolution<f64>) -> AtRestBody<f64> {
    let verts = pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
    finished(what, revolved_about_y(verts, rev, tol()), tol())
}

/// `body` turned `angle` about `axis` through the origin.
fn turned(what: &str, body: &AtRestBody<f64>, axis: Vec3<f64>, angle: f64) -> AtRestBody<f64> {
    let m = Affine3::rotation_about_axis(Point3::origin(), axis, angle);
    let moved = topo::transform_rigid(body, &geom_core::test_support::rigid(&m, tol())).unwrap();
    finished(what, moved, tol())
}

/// `a ∖ b` refuses `EdgeInFace` with `gap`.
fn refuses(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, gap: &EdgeInFaceGap) {
    match subtract(a, b, tol()) {
        Err(BooleanError::EdgeInFace { gap: got, .. }) if got == *gap => {}
        other => panic!(
            "{what}: {:?}, not the rider's `{gap:?}` refusal",
            other.map(drop)
        ),
    }
}

/// `a ∖ b` builds one body at `volume` that passes tiers 1, 3 and 3′ on
/// its records, among them `pairs` edge–edge records of the rim's
/// coincident arcs, the two of each pair bounding two faces of the
/// surface they lie on: the face the rim divides. *Breaks if* the
/// parting ships them unrecorded (3′ refuses an `EdgeEdgeOverlap`), or
/// as a zero-area ring of one face.
fn divides(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, volume: f64, pairs: usize) {
    holds(what, subtract(a, b, tol()), volume, pairs, true);
}

/// `r` is one body at `volume` that passes tiers 1, 3 and 3′ on its
/// records, among them `pairs` edge–edge records, the two edges of each
/// bounding two faces of their surface where `divided`, one face (a
/// zero-area ring of it) otherwise.
fn holds(
    what: &str,
    r: Result<BooleanResult<f64>, BooleanError>,
    volume: f64,
    pairs: usize,
    divided: bool,
) {
    let r = match r {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{what}: {:?}, not a body", other.map(drop)),
    };
    let body = &r.body;
    assert_eq!(topo::validate(body), Ok(()), "{what}: tier 1");
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(body, r.contacts(), tol()),
        Ok(()),
        "{what}: tier 3′ on {:?}",
        r.contacts()
    );
    assert_eq!(body.solids().count(), 1, "{what}: one solid");
    let faces = |e| {
        let d = body.get_edge(e).unwrap();
        [d.he_plus, d.he_minus].map(|he| body.face_of_half_edge(he).unwrap())
    };
    let surface = |f| body.get_face(f).unwrap().surface;
    let recorded: Vec<_> = r.contacts().edge_edge().collect();
    assert_eq!(
        recorded.len(),
        pairs,
        "{what}: the rim's arcs are {pairs} edge–edge records: {:?}",
        r.contacts()
    );
    for (x, y) in recorded {
        let floor = |e| {
            faces(e)
                .into_iter()
                .find(|&f| {
                    faces(if e == x { y } else { x })
                        .map(surface)
                        .contains(&surface(f))
                })
                .unwrap_or_else(|| panic!("{what}: {x:?} and {y:?} bound no faces of one surface"))
        };
        assert_eq!(
            floor(x) != floor(y),
            divided,
            "{what}: {x:?} and {y:?} bound two faces of their surface"
        );
    }
    let got =
        topo::test_support::pinned_volume(what, &finished(what, body.clone(), tol()), tol()).volume;
    assert!(
        (got - volume).abs() < 1e-9 * volume,
        "{what}: volume {got}, closed form {volume}"
    );
}

/// **The closed rim in the block's top face divides it**, the block less
/// the ring's Pappus volume `2π · 5/48`, and the slab less the groove's
/// `2π · 1/4`.
#[test]
fn a_closed_rim_in_a_plane_face_divides_it() {
    let block = finished(
        "the block",
        brick::<f64>((-2.0, 2.0), (0.0, 1.0), (-2.0, 2.0), tol()),
        tol(),
    );
    let ring = revolved(
        "the ring",
        &[(0.5, 0.5), (1.0, 0.5), (1.0, 1.0), (0.6, 0.6)],
        Revolution::Full,
    );
    divides(
        "the block less the ring",
        &block,
        &ring,
        16.0 - 2.0 * PI * 5.0 / 48.0,
        1,
    );
    let slab = finished(
        "the slab",
        brick::<f64>((-3.0, 3.0), (-1.0, 0.0), (-3.0, 3.0), tol()),
        tol(),
    );
    let groove = revolved(
        "the groove",
        &[(1.0, 0.0), (0.5, -0.5), (1.5, -0.5)],
        Revolution::Full,
    );
    divides(
        "the slab less the groove",
        &slab,
        &groove,
        36.0 - PI / 2.0,
        1,
    );
}

/// **A rim on the sphere divides the faces it lies in** (module docs),
/// the ball less the keel's Pappus volume.
#[test]
fn a_rim_on_a_sphere_divides_its_face() {
    let apex = (SQRT_2 * 0.4f64.sin(), SQRT_2 * 0.4f64.cos());
    let (b, c) = ((0.45, 1.15), (0.7, 1.05));
    let keel = revolved("the keel", &[apex, b, c], Revolution::Full);
    let area = ((b.0 - apex.0) * (c.1 - apex.1) - (c.0 - apex.0) * (b.1 - apex.1)).abs() / 2.0;
    let volume = 4.0 / 3.0 * PI * SQRT_2.powi(3) - area * 2.0 * PI * (apex.0 + b.0 + c.0) / 3.0;
    let poled = finished("the ball", ball_poled_z(SQRT_2, Vec3::zero(), tol()), tol());
    let spun = revolved_about_y(
        vec![
            (Point2::new(0.0, -SQRT_2), 1.0),
            (Point2::new(0.0, SQRT_2), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let spun = finished("the spun ball", spun, tol());
    for (what, ball, axis, angle, pairs) in [
        ("the ball", &poled, Vec3::unit_x(), 0.0, 1),
        (
            "the ball, the keel turned 0.3 about x",
            &poled,
            Vec3::unit_x(),
            0.3,
            1,
        ),
        (
            "the ball, the keel turned 1 about z",
            &poled,
            Vec3::unit_z(),
            1.0,
            1,
        ),
        (
            "the ball, the keel turned 1.2 about x",
            &poled,
            Vec3::unit_x(),
            1.2,
            3,
        ),
        (
            "the spun ball, the keel turned 1.2 about x",
            &spun,
            Vec3::unit_x(),
            1.2,
            1,
        ),
        (
            "the spun ball, the keel turned 0.3 about x",
            &spun,
            Vec3::unit_x(),
            0.3,
            3,
        ),
    ] {
        let keel = turned("the keel", &keel, axis, angle);
        divides(what, ball, &keel, volume, pairs);
    }
}

/// **A rim whose vertex lies on a wrap edge refuses `Shared`** (module
/// docs): the bore, the torus's top parallel closed and a quarter turn
/// of it, and the torus less a puck on a parallel, as built and turned.
#[test]
fn a_rim_whose_vertex_lies_on_a_wrap_edge_refuses_shared() {
    let tube = revolved(
        "the tube",
        &[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)],
        Revolution::Full,
    );
    let keel = revolved(
        "the keel",
        &[(1.0, 0.5), (1.2, 0.7), (1.2, 0.3)],
        Revolution::Full,
    );
    refuses(
        "the tube less the keel",
        &tube,
        &keel,
        &EdgeInFaceGap::Shared,
    );
    let torus = finished(
        "the torus",
        revolved_about_y(
            vec![(Point2::new(2.0, 0.0), 1.0), (Point2::new(4.0, 0.0), 1.0)],
            Revolution::Full,
            tol(),
        ),
        tol(),
    );
    let keel = [(3.0, 1.0), (2.8, 0.6), (3.2, 0.6)];
    for (what, rev) in [
        ("the full turn", Revolution::Full),
        ("the quarter turn", Revolution::Partial(FRAC_PI_2)),
    ] {
        let ring = revolved(what, &keel, rev);
        refuses(
            &format!("the torus less {what}"),
            &torus,
            &ring,
            &EdgeInFaceGap::Shared,
        );
    }
    let small = finished(
        "the small torus",
        revolved_about_y(
            vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(3.0, 0.0), 1.0)],
            Revolution::Full,
            tol(),
        ),
        tol(),
    );
    let puck = puck(2.5, 3f64.sqrt() / 2.0);
    for turn in [0.0, 1.0] {
        let puck = turned("the puck", &puck, Vec3::unit_y(), turn);
        refuses(
            &format!("the torus less the puck, turned {turn}"),
            &small,
            &puck,
            &EdgeInFaceGap::Shared,
        );
    }
}

/// **A small closed rim divides its plane face at every scale clear of
/// the tolerance**: the block's ring and the slab's groove scaled by `s`
/// about the revolve's axis, each profile in both windings. The side the
/// disc lies on is read as a length, the way to the rim's centre, so it
/// stays clear of the band down to rims a few hundred ε across.
#[test]
fn a_small_closed_rim_in_a_plane_face_divides_it() {
    let block = finished(
        "the block",
        brick::<f64>((-2.0, 2.0), (0.0, 1.0), (-2.0, 2.0), tol()),
        tol(),
    );
    let slab = finished(
        "the slab",
        brick::<f64>((-3.0, 3.0), (-1.0, 0.0), (-3.0, 3.0), tol()),
        tol(),
    );
    for s in [1e-1, 3e-3, 1e-3, 5e-4] {
        let ring = [
            (0.5 * s, 1.0 - 0.5 * s),
            (s, 1.0 - 0.5 * s),
            (s, 1.0),
            (0.6 * s, 1.0 - 0.4 * s),
        ];
        let groove = [(s, 0.0), (0.5 * s, -0.5 * s), (1.5 * s, -0.5 * s)];
        for reversed in [false, true] {
            let wound = |pts: &[(f64, f64)]| {
                let mut pts = pts.to_vec();
                if reversed {
                    pts.reverse();
                }
                revolved("the keel", &pts, Revolution::Full)
            };
            divides(
                &format!("the block less the ring at s = {s:e}, reversed {reversed}"),
                &block,
                &wound(&ring),
                16.0 - 2.0 * PI * 5.0 / 48.0 * s.powi(3),
                1,
            );
            divides(
                &format!("the slab less the groove at s = {s:e}, reversed {reversed}"),
                &slab,
                &wound(&groove),
                36.0 - PI / 2.0 * s.powi(3),
                1,
            );
        }
    }
}

/// **A small closed rim on the sphere divides its face** where the
/// face's mass properties read the disc: the ball less a keel whose
/// rim has radius `ρ`, the keel scaled with it. Where the disc's solid
/// angle levered by the radius, `π ρ² / R`, is within ten ε, the result
/// gate's sphere lane reads the hole's area in band and refuses
/// `VolumeUncomputable`
/// (`work/nearpoint/a-small-hole-in-a-sphere-face-reads-its-solid-angle-levered-by-the-radius-against-the-band.md`):
/// a correct body it does not measure, never a wrong one.
#[test]
fn a_small_closed_rim_on_a_sphere_divides_its_face() {
    let eps = tol().eps();
    let ball = finished("the ball", ball_poled_z(SQRT_2, Vec3::zero(), tol()), tol());
    for rho in [0.55, 1e-1, 3e-2, 1e-2, 3e-3, 1e-3, 5e-4] {
        let k = rho / 0.55;
        let polar = (rho / SQRT_2).asin();
        let apex = (SQRT_2 * polar.sin(), SQRT_2 * polar.cos());
        let b = (apex.0 - 0.1 * k, apex.1 - 0.15 * k);
        let c = (apex.0 + 0.15 * k, apex.1 - 0.25 * k);
        let keel = revolved("the keel", &[apex, b, c], Revolution::Full);
        let area = ((b.0 - apex.0) * (c.1 - apex.1) - (c.0 - apex.0) * (b.1 - apex.1)).abs() / 2.0;
        let volume = 4.0 / 3.0 * PI * SQRT_2.powi(3) - area * 2.0 * PI * (apex.0 + b.0 + c.0) / 3.0;
        let what = format!("the ball less the keel at ρ = {rho:e}");
        let r = subtract(&ball, &keel, tol());
        if PI * rho * rho / SQRT_2 < 10.0 * eps {
            assert!(
                matches!(
                    &r,
                    Err(BooleanError::ResultInvalid { findings })
                        if matches!(
                            findings.as_slice(),
                            [topo::ValidationError::VolumeUncomputable { .. }]
                        )
                ),
                "{what}: {:?}, not the sphere lane's limit",
                r.map(drop)
            );
            continue;
        }
        holds(&what, r, volume, 1, true);
    }
}

/// **A closed rim in a face with a hole keeps its ring form**: the
/// block bored by a pin of radius 0.2 through its top face, outside the
/// rim's disc or at its centre, less the ring. The divided disc would
/// take the face's rings inside it, which the parting does not move
/// (`work/heldrecords/a-closed-rider-in-a-face-with-rings-keeps-its-zero-area-ring.md`),
/// so the rim's two circles bound the one face, a zero-area ring of it.
#[test]
fn a_closed_rim_in_a_holed_plane_face_keeps_its_ring() {
    let block = finished(
        "the block",
        brick::<f64>((-2.0, 2.0), (0.0, 1.0), (-2.0, 2.0), tol()),
        tol(),
    );
    let ring = revolved(
        "the ring",
        &[(0.5, 0.5), (1.0, 0.5), (1.0, 1.0), (0.6, 0.6)],
        Revolution::Full,
    );
    for (what, x) in [("outside the disc", 1.6), ("inside the disc", 0.0)] {
        let pin = revolved(
            "the pin",
            &[(0.0, -1.0), (0.2, -1.0), (0.2, 2.0), (0.0, 2.0)],
            Revolution::Full,
        );
        let pin = topo::transform_rigid(
            &pin,
            &geom_core::test_support::rigid(&Affine3::translation(Vec3::new(x, 0.0, 0.0)), tol()),
        )
        .unwrap();
        let holed = match subtract(&block, &finished("the pin", pin, tol()), tol()) {
            Ok(BooleanResult::Body(b)) => finished("the bored block", b.body, tol()),
            other => panic!("the block bored {what}: {:?}", other.map(drop)),
        };
        holds(
            &format!("the block bored {what}, less the ring"),
            subtract(&holed, &ring, tol()),
            16.0 - PI * 0.04 - 2.0 * PI * 5.0 / 48.0,
            1,
            false,
        );
    }
}

/// The puck `ρ ≤ r`, `y ∈ [0, h]` about `y`, extruded from a
/// one-segment circle whose vertex lies on `+x`.
fn puck(r: f64, h: f64) -> AtRestBody<f64> {
    let circle = profile::circle(Point2::new(0.0, 0.0), r, tol()).unwrap();
    let flat = finished(
        "the puck",
        extruded(sketch_at(0.0), vec![circle.into()], h, tol()),
        tol(),
    );
    turned("the puck", &flat, Vec3::unit_x(), -FRAC_PI_2)
}
