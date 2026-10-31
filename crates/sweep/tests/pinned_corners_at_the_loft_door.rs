//! **A corner pinned between two loft sections is classified at the
//! loft door, and the sections are apart beyond it.** A pinned corner is
//! a profile vertex at one world point in two adjacent sections; a hinge
//! is two adjacent ones. Pinned throughout the span its wall covers, it
//! is a legal body the kernel does not build yet (a spline wall's
//! collapsed boundary column), refused as unsupported and never as bad
//! input; pinned in some slabs and moving in others under one wall, it
//! is a wall that retraces itself; in band, it escalates. Beyond the
//! pinned corners, two sections that meet refuse naming the two
//! segments. The coincident-section and strip-0 hinge rows are
//! `one_door_for_coincident_sections`'s.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::linalg::frame::path_start_frame;
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, RawLoop, SketchPlane};
use sweep::{LoftError, LoopSegment, PinnedSite, loft_body, sweep_body};

/// The 2×2 square, corner 0 at (−1, −1), counterclockwise.
fn square() -> Vec<ProfileLoop<f64>> {
    vec![ProfileLoop::polygon(
        [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
            .iter()
            .map(|&(x, y)| Point2::new(x, y)),
    )]
}

/// A lens on the chord from (1, 0) to (−1, 0): its upper arc rising
/// `ridge`, its lower one dropping `keel`, each as a bulge.
fn lens(ridge: f64, keel: f64) -> Vec<ProfileLoop<f64>> {
    vec![bulge_loop(vec![
        (Point2::new(1.0, 0.0), -ridge),
        (Point2::new(-1.0, 0.0), -keel),
    ])]
}

fn places(ms: &[Affine3<f64>], tol: Tol) -> Vec<SketchPlane<f64>> {
    ms.iter()
        .map(|m| SketchPlane::from_frame(geom_core::test_support::rigid(m, tol)))
        .collect()
}

fn turn(at: [f64; 3], about: [f64; 3], angle: f64) -> Affine3<f64> {
    Affine3::rotation_about_axis(
        Point3::new(at[0], at[1], at[2]),
        Vec3::new(about[0], about[1], about[2]),
        angle,
    )
}

fn lift(z: f64) -> Affine3<f64> {
    Affine3::translation(Vec3::new(0.0, 0.0, z))
}

/// Asserts `out` is the class-1 refusal naming `at` of loop 0 in slab
/// `slab`, as a kernel limit and not an input fault.
fn assert_unsupported(
    what: &str,
    out: Result<sweep::Lofted<f64>, LoftError>,
    at: PinnedSite,
    slab: usize,
) {
    match out {
        Err(
            e @ LoftError::PinnedCornerUnsupported {
                loop_index: 0,
                at: got,
                slab: s,
            },
        ) => {
            assert_eq!(
                (got, s),
                (at, slab),
                "{what}: the pinned site and its slab are named"
            );
            let msg = e.to_string();
            assert!(
                msg.contains(&format!(
                    "loft sections {slab} and {} hold {at} of loop 0 at one point; that is a \
                     legal body",
                    slab + 1
                )),
                "{what}: {msg}"
            );
            assert!(
                msg.contains("a kernel limit, not an input fault"),
                "{what}: the refusal is the kernel's, not the input's: {msg}"
            );
        }
        other => panic!("{what}: expected PinnedCornerUnsupported at {at}, got {other:?}"),
    }
}

/// **Class 1: a corner pinned throughout the span its wall covers.** A
/// two-section wedge (a square turned about its edge x = −1, segment 3)
/// and the same hinge at v-degree 1 through three sections, where each
/// slab's wall is its own; a single corner pinned in a wedge; and a
/// single corner pinned through every section at v-degree 2. Each is a
/// legal body the kernel does not build, refused as such.
#[test]
fn a_corner_pinned_throughout_its_walls_span_refuses_as_unsupported() {
    let tol = Tol::witness();
    let hinge = turn([-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], 0.5);
    let wedge = places(&[Affine3::identity(), hinge], tol);
    assert_unsupported(
        "two-section wedge",
        loft_body::<f64>(&vec![square(); 2], &wedge, 1, tol),
        PinnedSite::Segment(3),
        0,
    );
    let degree_one = places(&[Affine3::identity(), hinge, lift(3.0)], tol);
    assert_unsupported(
        "hinge at v-degree 1",
        loft_body::<f64>(&vec![square(); 3], &degree_one, 1, tol),
        PinnedSite::Segment(3),
        0,
    );
    // About the diagonal through corner 0, which no other corner is on.
    let corner = |a: f64| turn([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], a);
    assert_unsupported(
        "one corner pinned in a wedge",
        loft_body::<f64>(
            &vec![square(); 2],
            &places(&[Affine3::identity(), corner(0.5)], tol),
            1,
            tol,
        ),
        PinnedSite::Corner(0),
        0,
    );
    let every = places(&[Affine3::identity(), corner(0.3), corner(0.6)], tol);
    assert_unsupported(
        "one corner pinned through every section",
        loft_body::<f64>(&vec![square(); 3], &every, 2, tol),
        PinnedSite::Corner(0),
        0,
    );
}

/// **Class 2: one corner pinned in one slab and moving in the next**,
/// under a v-degree-2 wall through both. Named as the corner alone (its
/// neighbours move), with the pinning slab and the moving one.
#[test]
fn a_corner_pinned_in_one_slab_only_refuses_naming_it() {
    let tol = Tol::witness();
    let ps = places(
        &[
            Affine3::identity(),
            turn([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], 0.5),
            lift(3.0),
        ],
        tol,
    );
    match loft_body::<f64>(&vec![square(); 3], &ps, 2, tol) {
        Err(LoftError::PinnedCorner {
            loop_index: 0,
            at: PinnedSite::Corner(0),
            slab: 0,
            moving: 1,
        }) => {}
        other => panic!("expected PinnedCorner naming corner 0, got {other:?}"),
    }
}

/// **Class 3: a corner the band cannot call escalates, naming it.**
/// Corner 0 of the middle section is 3ε from its place in the first;
/// the others are far, so only that corner is in band.
#[test]
fn a_corner_in_band_escalates_naming_it() {
    let tol = Tol::witness();
    let step = 3.0 * tol.eps();
    let ps = places(
        &[
            Affine3::identity(),
            turn([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], 0.5)
                * Affine3::translation(Vec3::new(0.0, 0.0, step)),
            lift(3.0),
        ],
        tol,
    );
    match loft_body::<f64>(&vec![square(); 3], &ps, 2, tol) {
        Err(
            e @ LoftError::CornerEscalated {
                loop_index: 0,
                corner: 0,
                slab: 0,
                source,
            },
        ) => {
            assert_eq!(source.predicate, Some("loft_corner_pinned"));
            let msg = e.to_string();
            assert!(
                msg.contains("whether loft sections 0 and 1 hold corner 0 of loop 0 at one point"),
                "{msg}"
            );
        }
        other => panic!("expected CornerEscalated naming corner 0, got {other:?}"),
    }
}

/// **A hinge slid along its own edge pins no corner, and the sections
/// touch along that edge.** The middle square is turned about its edge
/// y = −1 and slid half a unit along it: every corner moves, and segment
/// 0 of both sections overlaps over x ∈ [−0.5, 1]. **Sections that
/// cross** — a square turned about its own x axis through its centre —
/// meet where segment 1 of each passes (1, 0, 0). Neither has a pinned
/// corner, so no corner reading sees either.
#[test]
fn sections_that_meet_beyond_their_pinned_corners_refuse_naming_the_segments() {
    let tol = Tol::witness();
    let seg = |segment| LoopSegment {
        loop_index: 0,
        segment,
    };
    let slid = places(
        &[
            Affine3::identity(),
            Affine3::translation(Vec3::new(0.5, 0.0, 0.0))
                * turn([0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 0.3),
            lift(2.0),
        ],
        tol,
    );
    let crossing = places(
        &[Affine3::identity(), turn([0.0; 3], [1.0, 0.0, 0.0], 0.5)],
        tol,
    );
    for (what, n, ps, v, want) in [
        ("slid hinge", 3, &slid, 2, [seg(0), seg(0)]),
        ("crossing sections", 2, &crossing, 1, [seg(1), seg(1)]),
    ] {
        match loft_body::<f64>(&vec![square(); n], ps, v, tol) {
            Err(e @ LoftError::SectionsTouch { slab: 0, segments }) => {
                assert_eq!(segments, want, "{what}: the first pair that meets is named");
                let msg = e.to_string();
                assert!(
                    msg.contains(&format!(
                        "loft section 0's {} meets section 1's {} beyond the corners they share",
                        want[0], want[1]
                    )),
                    "{what}: {msg}"
                );
            }
            other => panic!("{what}: expected SectionsTouch, got {other:?}"),
        }
    }
}

/// **A lens turned about its chord**: both corners pinned in every slab,
/// the arcs apart elsewhere. Its clearance holds (each arc's control
/// points, the shared corners left out, lie on one side of the other
/// section's plane), so an asymmetric lens reaches the class-1 refusal
/// naming its hinged segment. A symmetric one turns its upper half up
/// and its lower half down by the same amount, so its moving rows have
/// no mean step along the base normal: the frontier of the step arm,
/// refused as `DegenerateStacking`.
#[test]
fn a_lens_turned_about_its_chord_reaches_class_1_or_states_its_frontier() {
    let tol = Tol::witness();
    let about = |a: f64| turn([0.0; 3], [-1.0, 0.0, 0.0], a);
    let three = places(&[Affine3::identity(), about(0.4), about(0.8)], tol);
    let two = places(&[Affine3::identity(), about(0.8)], tol);
    for (what, ps, v) in [("three sections", &three, 2), ("a wedge", &two, 1)] {
        let n = ps.len();
        assert_unsupported(
            &format!("asymmetric lens, {what}"),
            loft_body::<f64>(&vec![lens(0.8, 0.3); n], ps, v, tol),
            PinnedSite::Segment(0),
            0,
        );
        match loft_body::<f64>(&vec![lens(0.5, 0.5); n], ps, v, tol) {
            Err(LoftError::DegenerateStacking { slab: 0 }) => {}
            other => panic!("symmetric lens, {what}: expected DegenerateStacking, got {other:?}"),
        }
    }
}

/// **What a sweep reaches.** A sweep's stations are one profile moved
/// rigidly along a path, so a corner is pinned only where the motion
/// between two stations fixes it. A quarter circle of radius 1 turns
/// every station about the circle's axis, and a rectangle with one edge
/// on that axis (from the centre, along it) is hinged there in every
/// slab: class 1, as a partial revolve's on-axis segment is a legal
/// wedge. (Two stations that coincide are
/// `one_door_for_coincident_sections`'s.)
#[test]
fn a_sweep_turning_about_a_profile_corner_refuses_as_unsupported() {
    let tol = Tol::witness();
    let h = core::f64::consts::FRAC_1_SQRT_2;
    let path = geom::NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ],
        vec![1.0, h, 1.0],
    )
    .unwrap();
    let place =
        SketchPlane::from_frame(path_start_frame(path.eval(0.0), path.deriv(0.0), tol).unwrap());
    // The circle's centre, and the direction from it to the path's start,
    // in the start frame's sketch coordinates.
    let frame = place.placement();
    let to_sketch = |p: Point3<f64>| {
        let d = p - place.origin();
        Point2::new(d.dot(frame.linear.c0), d.dot(frame.linear.c1))
    };
    let c = to_sketch(Point3::new(0.0, 0.0, 0.0));
    let (u, w) = (Point2::new(0.0, 0.0) - c, Point2::new(c.y, -c.x));
    let at = |a: f64, b: f64| Point2::new(c.x + a * u.x + b * w.x, c.y + a * u.y + b * w.y);
    let profile = vec![ProfileLoop::polygon([
        at(0.0, 0.0),
        at(0.5, 0.0),
        at(0.5, 0.3),
        at(0.0, 0.3),
    ])];
    match sweep_body::<f64>(&profile, place, &path, 3, 2, tol) {
        Err(
            e @ LoftError::PinnedCornerUnsupported {
                loop_index: 0,
                at: PinnedSite::Segment(_),
                slab: 0,
            },
        ) => {
            assert!(
                e.to_string().contains("a kernel limit, not an input fault"),
                "{e}"
            );
        }
        other => panic!("expected the class-1 refusal at the swept corner, got {other:?}"),
    }
}
