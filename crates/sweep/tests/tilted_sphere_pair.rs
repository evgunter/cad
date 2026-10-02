//! **Sphere sections tilted against the chart**: a sphere pair whose
//! centre line is off a ball's polar axis, and a plane that is not
//! normal to it, under every boolean.
//!
//! Each ball is the canonical full revolve of a semicircle about `y`
//! (two half-bands on one sphere key), moved off the origin. The pair's
//! section is the radical-plane circle, handed to both sides' wall
//! lanes; offset along `x` (or anywhere in
//! the seam plane `z = c_z`) that circle is tilted against both charts'
//! polar axis, so its azimuth doubles back and the join selects its arcs
//! by the side of the run they leave on
//! (`chord_join::select_arc_by_run_side`). The faces it leaves are
//! bounded by circles that are neither rims nor meridians, and the
//! sphere flux arm measures them by Gauss–Bonnet
//! (`props::curved::sphere_circle_loop`). Every body is held to all
//! three validation tiers and to its volume against the lens the two
//! spheres share, computed here from the radii and the centre distance
//! alone.
//!
//! The rows a tilted section does not yet reach are pinned at the door
//! they stop at: an offset that leaves the seam plane (a pierce ring),
//! and a plane tilted against the ball's chart (the boolean's planar
//! side, which has only the sphere face's azimuth window to select by).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

/// A ball of radius `r` centred at `c`, poles on world `y`.
fn ball(r: f64, c: Vec3<f64>) -> Body<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    topo::transform_rigid(&b, &Affine3::translation(c), Tol::witness()).unwrap()
}

fn ball_volume(r: f64) -> f64 {
    4.0 / 3.0 * PI * r.powi(3)
}

/// The volume of a spherical cap of height `h` on a sphere of radius `r`.
fn cap_volume(r: f64, h: f64) -> f64 {
    PI * h.powi(2) * (3.0 * r - h) / 3.0
}

/// The lens two balls `r1`, `r2` at centre distance `d` share: the cap
/// of each beyond the radical plane, which sits at
/// `x = (d² + r1² − r2²)/2d` from the first centre.
fn lens_volume(r1: f64, r2: f64, d: f64) -> f64 {
    let x = (d.powi(2) + r1.powi(2) - r2.powi(2)) / (2.0 * d);
    cap_volume(r1, r1 - x) + cap_volume(r2, r2 - (d - x))
}

/// Every tier of validation, then the volume against `expected` through
/// the kernel's mass properties — closed-form on every face here, so
/// the slack is rounding's and a wrong arc (the complement of a cap
/// selected) misses by the cap's own volume.
fn assert_body(label: &str, body: &Body<f64>, expected: f64) {
    assert_eq!(topo::validate(body), Ok(()), "{label}: validate");
    assert_eq!(
        topo::validate_closed(body),
        Ok(()),
        "{label}: validate_closed"
    );
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "{label}: validate_geometric"
    );
    let p = topo::mass_properties(body, Tol::witness())
        .unwrap_or_else(|e| panic!("{label}: mass properties, got {e:?}"));
    assert_eq!(p.volume_pad, 0.0, "{label}: closed-form faces only");
    assert!(
        (p.volume - expected).abs() <= 1e-9 * expected.max(1.0),
        "{label}: volume {} against the lens closed form {expected}",
        p.volume
    );
}

fn run(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
) -> Result<topo::BooleanResult<f64>, topo::BooleanError> {
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
}

/// `a ∪ b`, `a ∩ b`, `a ∖ b` and `b ∖ a`, each a body against its
/// closed form from the operands' volumes and the shared volume.
fn assert_every_op(pose: &str, a: &Body<f64>, b: &Body<f64>, va: f64, vb: f64, shared: f64) {
    for (label, op, x, y, expected) in [
        ("A ∪ B", BooleanOp::Union, a, b, va + vb - shared),
        ("A ∩ B", BooleanOp::Intersect, a, b, shared),
        ("A ∖ B", BooleanOp::Subtract, a, b, va - shared),
        ("B ∖ A", BooleanOp::Subtract, b, a, vb - shared),
    ] {
        let label = format!("{pose}, {label}");
        let out = run(op, x, y).unwrap_or_else(|e| panic!("{label} refused: {e:?}"));
        let body = &out
            .body()
            .unwrap_or_else(|| panic!("{label} came back empty"))
            .body;
        assert_body(&label, body, expected);
    }
}

/// The unit ball every pose offsets from.
const BASE: Vec3<f64> = Vec3::new(2.0, 2.0, 0.5);

/// **Sphere pairs offset in the seam plane** build under every op: the
/// radical plane is tilted against both charts, from a pair of equal
/// balls whose section is a great-circle-sized cut to a small ball
/// whose section lies wholly inside one half-band. Each pose is the
/// `y`-poled unit ball at [`BASE`] against a ball of radius `r` at
/// `BASE + offset`.
#[test]
fn sphere_pairs_tilted_against_both_charts_build_under_every_boolean() {
    for (pose, r, offset) in [
        ("equal balls along x", 1.0, Vec3::new(1.4, 0.0, 0.0)),
        ("equal balls along x and y", 1.0, Vec3::new(1.2, 0.6, 0.0)),
        ("a smaller ball along x", 0.6, Vec3::new(0.9, 0.0, 0.0)),
        ("a small ball mostly inside", 0.3, Vec3::new(0.9, 0.0, 0.0)),
        (
            "a smaller ball along x and y",
            0.5,
            Vec3::new(0.8, 0.4, 0.0),
        ),
    ] {
        assert_every_op(
            pose,
            &ball(1.0, BASE),
            &ball(r, BASE + offset),
            ball_volume(1.0),
            ball_volume(r),
            lens_volume(1.0, r, offset.norm()),
        );
    }
}

/// **The equal pair at the `Interval` scalar**: enclosures throughout,
/// the run-side rule's trileans and the Gauss–Bonnet turning angles on
/// enclosures, and every body certifies with a volume bracket around
/// the lens closed form — at the default and the 1e-6 band; at 1e-12
/// the pcurve mint escalates first, pinned below.
#[test]
fn a_tilted_sphere_pair_builds_at_the_interval_scalar() {
    use crate::common::interval::iv;
    use geom_core::{Bounds, Interval};
    let ball_iv = |c: Vec3<f64>| -> Body<Interval> {
        let b = sweep::test_support::revolved_about_y_at::<Interval>(
            vec![
                (Point2::new(iv(0.0), iv(-1.0)), iv(1.0)),
                (Point2::new(iv(0.0), iv(1.0)), iv(0.0)),
            ],
            Revolution::Full,
            Tol::witness(),
        );
        let to = Vec3::new(iv(c.x), iv(c.y), iv(c.z));
        topo::transform_rigid(&b, &Affine3::translation(to), Tol::witness()).unwrap()
    };
    let (a, b) = (ball_iv(BASE), ball_iv(BASE + Vec3::new(1.4, 0.0, 0.0)));
    let lens = lens_volume(1.0, 1.0, 1.4);
    let v1 = ball_volume(1.0);
    for (op, expected) in [
        (BooleanOp::Union, 2.0 * v1 - lens),
        (BooleanOp::Intersect, lens),
        (BooleanOp::Subtract, v1 - lens),
    ] {
        let out = match op {
            BooleanOp::Union => topo::boolean::union(&a, &b, Tol::witness()),
            BooleanOp::Intersect => topo::boolean::intersect(&a, &b, Tol::witness()),
            BooleanOp::Subtract => topo::boolean::subtract(&a, &b, Tol::witness()),
        };
        // At ε 1e-12 the tilted arcs' fitted pcurve rows meet the loop's
        // continuity check with enclosures wider than the band, and the
        // mint escalates by name
        // (`work/pcert/fitted-general-circle-rows-escalate-loop-continuity-at-the-interval-scalar.md`).
        if Tol::witness().get().eps < 1e-10 {
            let Err(topo::BooleanError::Pcurves {
                source: topo::PcurveMintError::Escalated { cause, .. },
            }) = &out
            else {
                panic!("Interval {op:?} at eps 1e-12: expected the mint's escalation, got {out:?}");
            };
            assert_eq!(
                cause.predicate,
                Some("pcurve_loop_continuity"),
                "Interval {op:?}"
            );
            continue;
        }
        let out = out.unwrap_or_else(|e| panic!("Interval {op:?} refused: {e:?}"));
        let body = &out
            .body()
            .unwrap_or_else(|| panic!("Interval {op:?} came back empty"))
            .body;
        assert_eq!(topo::validate(body), Ok(()), "Interval {op:?}: validate");
        assert_eq!(
            topo::validate_geometric(body, Tol::witness()),
            Ok(()),
            "Interval {op:?}: validate_geometric"
        );
        let v = topo::mass_properties(body, Tol::witness())
            .unwrap_or_else(|e| panic!("Interval {op:?}: mass properties, got {e:?}"))
            .volume;
        let slack = 1e-9 * expected.max(1.0);
        assert!(
            v.lo() - slack <= expected && expected <= v.hi() + slack,
            "Interval {op:?}: volume [{}, {}] against the lens closed form {expected}",
            v.lo(),
            v.hi()
        );
    }
}

/// **Where a tilted section stops.** An offset with a component off the
/// seam plane drives the pierce off the seam, into a half-band: the
/// pierce lands as a ring, whose run carries no certified edge for the
/// run-side rule to read (`work/tang/pierce-ring-has-no-join-arm.md`). A PLANE
/// tilted against the ball's chart — a box face across the ball, and a
/// pip whose poles land on the cube's top — refuses on the planar side,
/// which selects its arc by the sphere face's azimuth window and has no
/// window for a tilted section
/// (`work/reach/planar-side-of-a-tilted-plane-sphere-cut-has-no-arc-cue.md`).
#[test]
fn a_tilted_section_stops_at_the_pierce_ring_and_the_planar_side() {
    for (pose, b) in [
        (
            "off the seam plane",
            ball(1.0, BASE + Vec3::new(1.3, 0.0, 0.2)),
        ),
        (
            "a smaller ball off every axis",
            ball(0.7, BASE + Vec3::new(0.9, 0.3, 0.6)),
        ),
    ] {
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let e = run(op, &ball(1.0, BASE), &b)
                .err()
                .unwrap_or_else(|| panic!("{pose}, {op:?}: built where the frontier was pinned"));
            assert!(
                matches!(
                    e,
                    topo::BooleanError::Join(topo::SplitJoinError::SectionArcSide {
                        case: topo::ArcSideCase::NoCertifiedRun,
                        ..
                    })
                ),
                "{pose}, {op:?}: expected the pierce ring's empty run, got {e:?}"
            );
        }
    }
    let box_face = sweep::test_support::brick((0.5, 3.0), (-2.0, 2.0), (0.0, 2.0), Tol::witness());
    let cube = sweep::test_support::cube::<f64>(1.0, Tol::witness());
    for (pose, a, b) in [
        (
            "a box face across the ball",
            box_face,
            ball(1.0, Vec3::new(0.0, 0.0, 0.0)),
        ),
        (
            "a pip on the cube's top",
            cube,
            ball(0.3, Vec3::new(0.5, 0.5, 1.0)),
        ),
    ] {
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let e = run(op, &a, &b)
                .err()
                .unwrap_or_else(|| panic!("{pose}, {op:?}: built where the frontier was pinned"));
            assert!(
                matches!(
                    e,
                    topo::BooleanError::Join(topo::SplitJoinError::SectionNotPolar { .. })
                ),
                "{pose}, {op:?}: expected the planar side's tilted section, got {e:?}"
            );
        }
    }
}

/// **A flipped sense bit on a tilted face is named, not measured.** The
/// faces a tilted cut leaves that hold neither pole — both faces of the
/// equal pair's lens, and the bitten cap `A ∖ B` keeps on B's sphere —
/// encode their side by their loop, so a lone flip of the bit reads as
/// `CurvedSenseInverted` in tier 3 and as `SenseContradicted` in the
/// mass properties, rather than as the complement's volume. (A face
/// with a pole on its loop encodes no side; there the bit stands alone,
/// as on the rimless band —
/// `work/props/a-sphere-face-whose-boundary-encodes-no-side-is-measured-under-its-bit-alone.md`.)
#[test]
fn a_flipped_tilted_face_is_refused_by_name() {
    let a = ball(1.0, BASE);
    let b = ball(1.0, BASE + Vec3::new(1.4, 0.0, 0.0));
    let b_centre = BASE + Vec3::new(1.4, 0.0, 0.0);
    for (label, op, on_b_only) in [
        ("A ∩ B", BooleanOp::Intersect, false),
        ("A ∖ B", BooleanOp::Subtract, true),
    ] {
        let body = run(op, &a, &b).unwrap().body().unwrap().body.clone();
        let mut flipped = 0;
        for (k, f) in body.faces() {
            let on_b = matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Sphere { center, .. })
                    if (*center - geom_core::Point3::new(b_centre.x, b_centre.y, b_centre.z)).norm() < 1e-12
            );
            if on_b_only && !on_b {
                continue;
            }
            let inverted = body.flipped_face_sense_for_tests(k).expect("the face");
            assert_eq!(
                topo::validate_geometric(&inverted, Tol::witness()),
                Err(vec![topo::ValidationError::CurvedSenseInverted { face: k }]),
                "{label}: face {k:?} flipped"
            );
            assert!(
                matches!(
                    topo::mass_properties(&inverted, Tol::witness()),
                    Err(topo::MassPropsError::Face {
                        face,
                        source: geom_brep::props::PropsError::SenseContradicted,
                    }) if face == k
                ),
                "{label}: face {k:?} flipped is refused, not measured"
            );
            flipped += 1;
        }
        assert!(flipped > 0, "{label}: a tilted face to flip");
    }
}

/// **The two arc rules never meet on one face today, and this row says
/// so where it can fail.** CLEAVE's conic pairing
/// (`topo::splitting::join`'s `conic_pairs`, `SectionCrossings`) decides
/// WHICH crossings of a curved face a split chord joins. The run-side
/// rule (`chord_join::select_arc_by_run_side`, `SectionArcSide`) decides
/// WHICH ARC of the section conic that chord takes, and only on a
/// sphere section tilted against its chart. The pairing runs only in the
/// split lane, and the split lane refuses a sphere face at its reduce,
/// so a tilted sphere section reaches the run-side rule only through the
/// boolean lane, which pairs by its own walk. A ball, the tilted pair's
/// union and its lens, each split by planes tilted against their charts,
/// all refuse there, before either rule. When the split lane admits
/// sphere faces this goes red: compose the two then (the pairing first,
/// then the arc; both name the arc inside the face).
#[test]
fn a_tilted_split_of_a_sphere_body_refuses_before_either_arc_rule() {
    let tol = Tol::witness();
    let a = ball(1.0, BASE);
    let b = ball(1.0, BASE + Vec3::new(1.4, 0.0, 0.0));
    let body_of = |out: Result<topo::BooleanResult<f64>, topo::BooleanError>| match out {
        Ok(topo::BooleanResult::Body(bb)) => bb.body,
        other => panic!("the tilted pair builds: {other:?}"),
    };
    let union = body_of(topo::boolean::union(&a, &b, tol));
    let lens = body_of(topo::boolean::intersect(&a, &b, tol));
    for (name, body) in [("ball", &a), ("union", &union), ("lens", &lens)] {
        for (origin, normal) in [
            (Point3::new(2.0, 2.0, 0.5), Vec3::new(0.3, 1.0, 0.4)),
            (Point3::new(2.7, 2.3, 0.5), Vec3::new(0.2, 1.0, 0.7)),
            (Point3::new(2.7, 2.0, 0.5), Vec3::new(1.0, 0.3, 0.2)),
        ] {
            let plane = topo::test_support::split_plane(origin, normal, tol);
            match topo::split(body, &plane, tol) {
                Err(topo::SplitError::Reduce(
                    topo::SplitReduceError::CurvedBooleanUnsupported {
                        kind: geom::SurfaceKind::Sphere,
                        ..
                    },
                )) => {}
                other => panic!(
                    "{name} split through {origin:?} along {normal:?}: expected the split's \
                     reduce to refuse the sphere face, got {other:?}"
                ),
            }
        }
    }
}
