//! **A corner pinned between two loft sections is classified at the
//! loft door, and the sections are apart beyond it.** A pinned corner is
//! a profile vertex at one world point in two adjacent sections; a hinge
//! is two adjacent ones. Pinned throughout the span its wall covers, it
//! is a legal body (`pinned_corners_build`), refused as unsupported only
//! at a full turn's one vertex; pinned in some slabs and moving in others
//! under one wall, it
//! is a wall that retraces itself; in band, it escalates. Beyond the
//! pinned corners, two sections that meet refuse naming the two
//! segments. The coincident-section and strip-0 hinge rows are
//! `one_door_for_coincident_sections`'s.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

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

/// **A full turn's pinned vertex builds.** Two one-segment circles, one
/// turned about the tangent at its vertex, pin that vertex. The loft cuts
/// the full turn at its sub-arc joints into four quarter walls, so the
/// vertex is a corner between two of them: one vertex, no strut, and the
/// walls beside it each a collapsed side there, as every other pinned
/// corner builds. The body is closed (tier 2), at rest (tier 3), a
/// pseudomanifold (tier 3′), and its volume holds the ruled solid's,
/// read off the sections alone.
#[test]
fn a_full_turns_pinned_vertex_builds() {
    let tol = Tol::witness();
    let circle = || {
        vec![<ProfileLoop<f64> as RawLoop<f64>>::new([(
            Point2::new(1.0, 0.0),
            profile::Segment::Arc(geom_core::Arc2 {
                centre: Point2::new(0.0, 0.0),
                radius: 1.0,
                sweep: core::f64::consts::TAU,
            }),
        )])]
    };
    let ms = [
        Affine3::identity(),
        turn([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.5),
    ];
    let ps = places(&ms, tol);
    let lofted = loft_body::<f64>(&vec![circle(); 2], &ps, 1, tol)
        .unwrap_or_else(|e| panic!("the full turn with a pinned vertex lofts: {e:?}"));
    assert!(
        lofted.side_faces[0][0]
            .iter()
            .all(|w| matches!(w[..], [Some(_)])),
        "four quarter walls: {:?}",
        lofted.side_faces
    );
    assert_eq!(lofted.side_faces[0][0].len(), 4, "four quarter walls");
    assert_eq!(
        lofted.seam_edges[0][0],
        [None],
        "no strut at the pinned vertex"
    );
    assert_eq!(topo::validate_closed(&lofted.body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&lofted.body, tol),
        Ok(()),
        "the pinned full turn is at rest"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&lofted.body, &topo::ContactRecords::default(), tol),
        Ok(()),
        "tier 3′"
    );
    let tau = core::f64::consts::TAU;
    let want = crate::pinned_corners_build::ruled_curve_volume(
        |u| ((tau * u).cos(), (tau * u).sin()),
        |u| (-tau * (tau * u).sin(), tau * (tau * u).cos()),
        core::f64::consts::PI,
        (&ms[0], &ms[1]),
        256,
    );
    let m = topo::mass_properties(
        &lofted.body,
        tol,
        topo::test_support::pinned_resolution(tol),
    )
    .unwrap()
    .enclosure;
    assert!(
        (m.volume - want).abs() <= m.volume_pad + 1e-9 * want,
        "volume {} ± {}, ruled {want}",
        m.volume,
        m.volume_pad
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

/// **Class 2 at a sub-arc joint: a joint pinned in one slab and moving
/// in the next.** The half disc's semicircle (segment 1) is two quarter
/// walls joined at its apex `(0, 1)`. Turned about the line through the
/// apex along `x`, the apex is one point in the first slab; lifted
/// after, it moves in the second. Under a v-degree-2 wall through both
/// the door refuses naming the joint, with the pinning slab and the
/// moving one; at v-degree 1 each slab is its own wall, and it builds.
#[test]
fn a_joint_pinned_in_one_slab_only_refuses_at_degree_two_and_builds_at_one() {
    let tol = Tol::witness();
    let dee = || {
        vec![bulge_loop(vec![
            (Point2::new(-1.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 1.0),
        ])]
    };
    let ps = places(
        &[
            Affine3::identity(),
            turn([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], -0.4),
            lift(2.0),
        ],
        tol,
    );
    match loft_body::<f64>(&vec![dee(); 3], &ps, 2, tol) {
        Err(LoftError::PinnedCorner {
            loop_index: 0,
            at:
                PinnedSite::Joint {
                    segment: 1,
                    joint: 0,
                },
            slab: 0,
            moving: 1,
        }) => {}
        other => panic!("expected PinnedCorner naming the semicircle's joint, got {other:?}"),
    }
    let lofted = loft_body::<f64>(&vec![dee(); 3], &ps, 1, tol)
        .unwrap_or_else(|e| panic!("at v-degree 1 each slab is its own wall: {e:?}"));
    assert_eq!(
        lofted.joint_edges[0][1][0],
        [None, lofted.joint_edges[0][1][0][1]],
        "no strut at the joint in the pinning slab"
    );
    assert!(
        lofted.joint_edges[0][1][0][1].is_some(),
        "and a strut in the moving one"
    );
    assert_eq!(topo::validate_closed(&lofted.body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&lofted.body, tol),
        Ok(()),
        "tier 3"
    );
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
            let msg = e.render(geom_brep::recourse::Reading::Build);
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
            Err(
                e @ LoftError::SectionsTouch {
                    slab: 0,
                    segments,
                    shared: 0,
                },
            ) => {
                assert_eq!(segments, want, "{what}: the first pair that meets is named");
                let msg = e.render(geom_brep::recourse::Reading::Build);
                assert!(
                    msg.contains(&format!(
                        "loft section 0's {} meets section 1's {}, so the body's boundary",
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
/// the arcs apart elsewhere, the curve-pair reader finding them meet
/// only at those corners. No corner moves, so each arc steps on its own,
/// and the two step to opposite sides of the lens's plane: the slab
/// stacks against itself, symmetric or not, and turned either way.
#[test]
fn a_lens_turned_about_its_chord_stacks_against_itself() {
    let tol = Tol::witness();
    for sense in [1.0, -1.0] {
        let about = |a: f64| turn([0.0; 3], [-sense, 0.0, 0.0], a);
        let three = places(&[Affine3::identity(), about(0.4), about(0.8)], tol);
        let two = places(&[Affine3::identity(), about(0.8)], tol);
        for (what, ps, v) in [("three sections", &three, 2), ("a wedge", &two, 1)] {
            for (ridge, keel) in [(0.8, 0.3), (0.5, 0.5)] {
                match loft_body::<f64>(&vec![lens(ridge, keel); ps.len()], ps, v, tol) {
                    Err(LoftError::ReversedStacking { slab: 0 }) => {}
                    other => panic!(
                        "lens ({ridge}, {keel}) turned {sense}, {what}: expected \
                         ReversedStacking, got {other:?}"
                    ),
                }
            }
        }
    }
}

/// A circle of radius `r`, two semicircles, its vertices at angle `a`
/// and `a + π`.
fn circle(r: f64, a: f64) -> Vec<ProfileLoop<f64>> {
    let p = |t: f64| Point2::new(r * t.cos(), r * t.sin());
    vec![bulge_loop(vec![
        (p(a), 1.0),
        (p(a + core::f64::consts::PI), 1.0),
    ])]
}

/// **An elbow builds whatever vertex its circle is authored from.** A
/// unit circle turned 0.4 rad about an axis in its plane at x = 1.2:
/// the two planes meet outside the circle, so the loft is a torus
/// wedge, embedded. The arc pairs are read by the curve-pair reader's
/// closed forms, which do not see the start vertex.
#[test]
fn an_elbow_whose_planes_meet_outside_the_circle_builds_from_any_start() {
    let tol = Tol::witness();
    for start in [0.0f64, 45.0, 90.0] {
        for (n, v) in [(2usize, 1usize), (3, 2)] {
            let ms: Vec<Affine3<f64>> = (0..n)
                .map(|i| {
                    #[allow(clippy::cast_precision_loss)]
                    let a = 0.4 * i as f64;
                    turn([1.2, 0.0, 0.0], [0.0, 1.0, 0.0], a)
                })
                .collect();
            let out = loft_body::<f64>(
                &vec![circle(1.0, start.to_radians()); n],
                &places(&ms, tol),
                v,
                tol,
            );
            assert!(out.is_ok(), "start {start}, {n} sections: {:?}", out.err());
        }
    }
}

/// **A tilted D whose arc rows step back builds.** A D (the line x = 0
/// from (0, −1) to (0, 1), a semicircle through (±1, 0)) tilted about
/// y with its arc side down, then moved by (1.2, 0, h). Every ruling
/// stays at its own y, and each y-slice is a simple quad, so the body
/// is embedded; the arc's mid control points step back along the base
/// normal, which the step does not read.
#[test]
fn a_tilted_d_whose_arc_rows_step_back_builds() {
    let tol = Tol::witness();
    for bulge in [1.0, -1.0] {
        let d = vec![bulge_loop(vec![
            (Point2::new(0.0, -1.0), 0.0),
            (Point2::new(0.0, 1.0), -bulge),
        ])];
        for th in [0.3, 0.5, 0.7] {
            for h in [0.1, 0.2, 0.3] {
                let ps = places(
                    &[
                        Affine3::identity(),
                        Affine3::translation(Vec3::new(1.2, 0.0, h))
                            * turn([0.0; 3], [0.0, 1.0, 0.0], th),
                    ],
                    tol,
                );
                let out = loft_body::<f64>(&vec![d.clone(); 2], &ps, 1, tol);
                assert!(out.is_ok(), "bulge {bulge}, θ {th}, h {h}: {:?}", out.err());
            }
        }
    }
}

/// **A tight-bend tube sweeps whatever vertex its circle starts at.** A
/// circle of radius up to 0.9 swept along a radius-1 quarter circle,
/// its vertices toward the bend's axis or 45° off it.
#[test]
fn a_tight_bend_tube_sweeps_from_any_start() {
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
    let place = crate::common::normal_start_place(&path);
    let frame = place.placement();
    let d = Point3::new(0.0, 0.0, 0.0) - place.origin();
    let toward = d.dot(frame.linear.c1).atan2(d.dot(frame.linear.c0));
    for r in [0.3, 0.6, 0.75, 0.8, 0.9] {
        for shift in [0.0f64, 45.0] {
            let profile = circle(r, toward - shift.to_radians());
            for (stations, v) in [(3usize, 2usize), (5, 2), (3, 1)] {
                let out = sweep_body::<f64>(&profile, place, &path, stations, v, tol);
                assert!(
                    out.is_ok(),
                    "r {r}, shift {shift}, {stations} stations, v {v}: {:?}",
                    out.err()
                );
            }
        }
    }
}

/// **The step's own escalation.** A square moved five units along its
/// plane and by the band's midpoint along its normal: every corner
/// moves, the two are apart, and only the mean step along the base
/// normal is in band.
#[test]
fn a_step_in_band_escalates_on_the_base_normal() {
    let tol = Tol::witness();
    let mid = 0.5 * (1.0 + tol.k()) * tol.eps();
    let ps = places(
        &[
            Affine3::identity(),
            Affine3::translation(Vec3::new(5.0, 0.0, mid)),
        ],
        tol,
    );
    match loft_body::<f64>(&vec![square(); 2], &ps, 1, tol) {
        Err(LoftError::StackingEscalated { slab: 0, source }) => {
            assert_eq!(source.predicate, Some("loft_stacking"));
        }
        other => panic!("expected StackingEscalated on slab 0, got {other:?}"),
    }
}

/// **Sections that hold their outer loop and move a hole.** The outer
/// square is pinned at every corner and the hole slides in the plane:
/// the outer loop gives no step, so the step reads every loop's moving
/// corners, whose mean along the normal is zero — no direction.
#[test]
fn an_outer_loop_held_with_a_hole_moving_in_its_plane_has_no_step() {
    let tol = Tol::witness();
    let ring = |cx: f64| -> Vec<ProfileLoop<f64>> {
        vec![
            ProfileLoop::polygon(
                [(-5.0, -5.0), (5.0, -5.0), (5.0, 5.0), (-5.0, 5.0)]
                    .iter()
                    .map(|&(x, y)| Point2::new(x, y)),
            ),
            ProfileLoop::polygon(
                [(-0.5, -0.5), (-0.5, 0.5), (0.5, 0.5), (0.5, -0.5)]
                    .iter()
                    .map(|&(x, y)| Point2::new(x + cx, y)),
            ),
        ]
    };
    let ps = places(&[Affine3::identity(), Affine3::identity()], tol);
    match loft_body::<f64>(&[ring(-2.0), ring(2.0)], &ps, 1, tol) {
        Err(LoftError::DegenerateStacking { slab: 0 }) => {}
        other => panic!("expected DegenerateStacking, got {other:?}"),
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
fn a_sweep_turning_about_a_profile_edge_builds_its_wedge() {
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
    let place = crate::common::normal_start_place(&path);
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
    let swept = sweep_body::<f64>(&profile, place, &path, 3, 2, tol)
        .unwrap_or_else(|e| panic!("the swept wedge builds: {e}"));
    assert_eq!(
        swept.side_faces[0]
            .iter()
            .filter(|w| w[0][0].is_none())
            .count(),
        1,
        "the segment on the axis has no wall: the two caps hold it"
    );
}

/// A D on the line x = 0 from (0, 1) to (0, −1), its semicircle through
/// (1, 0).
fn d_east() -> Vec<ProfileLoop<f64>> {
    vec![bulge_loop(vec![
        (Point2::new(0.0, 1.0), 0.0),
        (Point2::new(0.0, -1.0), 1.0),
    ])]
}

/// **An arc pair the reader cannot settle decides nothing at the door.**
/// The D turned about an axis through (−1 − d, 0) in its plane, or
/// tilted out of it: the axis is tangent to (or just off) the arc's
/// circle on the half no section holds, so the sections are apart by
/// about the turn, yet the curve-pair reader, which poses the whole
/// carriers, reads the near-tangency off both spans in band or
/// unsettled. Only a meeting it verifies refuses, so each builds, as on
/// main.
#[test]
fn a_d_turned_about_its_carriers_tangent_off_the_arc_builds() {
    let tol = Tol::witness();
    for tilt in [0.0f64, 0.3, 0.6, 1.0] {
        for angle in [0.1, 0.4, 1.2] {
            for d in [0.0, 1e-8, 1e-7] {
                let axis = [0.0, tilt.cos(), tilt.sin()];
                let ps = places(
                    &[
                        Affine3::identity(),
                        turn([-1.0 - d, 0.0, 0.0], axis, -angle),
                    ],
                    tol,
                );
                let out = loft_body::<f64>(&vec![d_east(); 2], &ps, 1, tol);
                assert!(
                    out.is_ok(),
                    "tilt {tilt}, turn {angle}, d {d}: {:?}",
                    out.err()
                );
            }
        }
    }
}

/// **A circle elbow just clear of its own tangent builds**, monotone in
/// the clearance. The unit circle turned about an axis through
/// (1 + d, 0), tilted out of the plane, for d of 300, 1000 and 3000 ε:
/// the rims are apart by at least 16 ε (at the default ε, 1.6e-8 to
/// 4e-6, the review's triple), past the band, and the caps are apart.
/// The reader's in-band or unsettled readings near the touch decide
/// nothing, so no clearance in this range refuses.
#[test]
fn a_tilted_circle_elbow_just_clear_of_its_tangent_builds() {
    let tol = Tol::witness();
    for tilt in [0.0f64, 0.3, 0.6, 1.0] {
        for angle in [0.1, 0.4, 1.2] {
            for d in [300.0, 1000.0, 3000.0].map(|m| m * tol.eps()) {
                let axis = [0.0, tilt.cos(), tilt.sin()];
                let ps = places(
                    &[Affine3::identity(), turn([1.0 + d, 0.0, 0.0], axis, angle)],
                    tol,
                );
                let out = loft_body::<f64>(&vec![circle(1.0, 45f64.to_radians()); 2], &ps, 1, tol);
                assert!(
                    out.is_ok(),
                    "tilt {tilt}, turn {angle}, d {d}: {:?}",
                    out.err()
                );
            }
        }
    }
}

/// **An elbow's verdict does not depend on its spelling.** The unit
/// circle turned about an axis outside it, in its plane or tilted
/// 0.3 rad out of it, authored from every 45° and in both senses:
/// every spelling builds.
#[test]
fn an_elbow_builds_from_every_spelling() {
    let tol = Tol::witness();
    for tilt in [0.0f64, 0.3] {
        for off in [1.001, 1.02, 1.05, 1.2] {
            for angle in [0.1, 0.4, 1.2] {
                let axis = [0.0, tilt.cos(), tilt.sin()];
                let ps = places(
                    &[Affine3::identity(), turn([off, 0.0, 0.0], axis, angle)],
                    tol,
                );
                for start in (0..360).step_by(45) {
                    for bulge in [1.0, -1.0] {
                        let p = |t: f64| Point2::new(t.cos(), t.sin());
                        let a = f64::from(start).to_radians();
                        let c = vec![bulge_loop(vec![
                            (p(a), bulge),
                            (p(a + core::f64::consts::PI), bulge),
                        ])];
                        let out = loft_body::<f64>(&vec![c; 2], &ps, 1, tol);
                        assert!(
                            out.is_ok(),
                            "tilt {tilt}, axis at {off}, turn {angle}, start {start}, \
                             bulge {bulge}: {:?}",
                            out.err()
                        );
                    }
                }
            }
        }
    }
}

/// **Two lines leaving a shared pinned corner along one ray meet beyond
/// it.** A right triangle with legs 2 and 1, then one with legs 1 and 1
/// turned 0.5 rad about its first leg's line: the corner at the origin
/// is pinned, and the two first legs share the run from it to (1, 0, 0).
/// The census's collinear overlap reads that run, so the pair refuses,
/// naming the one corner they share.
#[test]
fn two_lines_leaving_a_shared_corner_along_one_ray_touch() {
    let tol = Tol::witness();
    let tri = |a: f64, b: f64| {
        vec![ProfileLoop::polygon(
            [(0.0, 0.0), (a, 0.0), (0.0, b)]
                .iter()
                .map(|&(x, y)| Point2::new(x, y)),
        )]
    };
    let ps = places(
        &[Affine3::identity(), turn([0.0; 3], [1.0, 0.0, 0.0], 0.5)],
        tol,
    );
    match loft_body::<f64>(&[tri(2.0, 1.0), tri(1.0, 1.0)], &ps, 1, tol) {
        Err(
            e @ LoftError::SectionsTouch {
                slab: 0,
                segments,
                shared: 1,
            },
        ) => {
            let seg = LoopSegment {
                loop_index: 0,
                segment: 0,
            };
            assert_eq!(segments, [seg, seg]);
            assert!(
                e.render(geom_brep::recourse::Reading::Build)
                    .contains("off the pinned corner they share"),
                "{e_at}",
                e_at = e.render(geom_brep::recourse::Reading::Build)
            );
        }
        other => panic!("expected SectionsTouch sharing one corner, got {other:?}"),
    }
}

/// **The first refusal is the earliest slab's.** Three squares: the
/// second stacked against the first's normal, the third turned through
/// the second. Slab 0 reverses and slab 1's sections cross; the door
/// reads each slab whole before the next, so slab 0's reversal is the
/// one named.
#[test]
fn the_earliest_slab_names_the_refusal() {
    let tol = Tol::witness();
    let ps = places(
        &[
            Affine3::identity(),
            lift(-1.0),
            turn([0.0, 0.0, -1.0], [0.0, 1.0, 0.0], 0.5),
        ],
        tol,
    );
    for v in [1, 2] {
        match loft_body::<f64>(&vec![square(); 3], &ps, v, tol) {
            Err(LoftError::ReversedStacking { slab: 0 }) => {}
            other => panic!("v {v}: expected ReversedStacking at slab 0, got {other:?}"),
        }
    }
}
