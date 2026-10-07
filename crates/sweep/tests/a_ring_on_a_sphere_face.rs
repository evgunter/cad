//! **A ring on a sphere face winds its island without a chart.** A box
//! edge that pierces a ball's face lands the section there as a ring,
//! and the join's ring lane winds the island it walls off by a
//! great-circle path from an outer-loop point to the closing arc's
//! midpoint: its crossings and the side it arrives from
//! (`chord_join::path_island_winding`). A bystander ring of a sphere
//! face is re-homed by the parity of a great-circle path
//! (`chord_join::path_ring_side`).
//!
//! Every pose runs every op in both member orders. A body is held to
//! tier 3, to tier 3′ (or, for a result in two lumps, to the census
//! refusal its curved lumps draw), and to its volume against a slice
//! integral of the ball and box: each slice is a disc against a rectangle, exact, and
//! only the stack over height is quadrature. A result whose ball face
//! keeps the ring as a hole (the ball's side of ∪ and of ball ∖ box)
//! refuses at the result gate, `VolumeUncomputable { RingOnCurvedFace }`
//! (`work/flux/sphere-face-with-a-hole-has-no-closed-form.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled, ball_poled_y, ball_poled_z, brick, finished};
use topo::{AtRestBody, BooleanDeclarations, BooleanError, BooleanResult};

use crate::common::oracles::{ball_volume, lens_volume};

use Want::{Body, Gate, Lumps};

/// A box `[x0, x1] × [y0, y1] × [z0, z1]`.
type Bounds = [(f64, f64); 3];

/// The slab every box pose but the item's two starts from.
const SLAB: Bounds = [(0.0, 4.0), (0.0, 4.0), (0.0, 1.0)];

fn boxed(b: Bounds) -> AtRestBody<f64> {
    finished(
        "the box",
        brick(b[0], b[1], b[2], Tol::witness()),
        Tol::witness(),
    )
}

/// `ball` turned by `turn` before it is moved to `c`.
fn placed(ball: topo::Body<f64>, turn: Affine3<f64>, c: Vec3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let turned = topo::transform_rigid(&ball, &turn, tol).unwrap();
    let moved = topo::transform_rigid(&turned, &Affine3::translation(c), tol).unwrap();
    finished("the ball", moved, tol)
}

/// A `y`-poled ball of radius `r` at `c`, its seam meridian on `+x`.
fn ball_y(r: f64, c: Vec3<f64>) -> AtRestBody<f64> {
    let at = ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), Tol::witness());
    placed(at, Affine3::identity(), c)
}

/// A `z`-poled ball of radius `r` at `c`, tilted `tilt` about `x` and
/// then turned `turn` about `z`.
fn ball_z(r: f64, c: Vec3<f64>, tilt: f64, turn: f64) -> AtRestBody<f64> {
    let at = ball_poled_z(r, Vec3::new(0.0, 0.0, 0.0), Tol::witness());
    let o = Point3::origin();
    let tilt = Affine3::rotation_about_axis(o, Vec3::new(1.0, 0.0, 0.0), tilt);
    let turn = Affine3::rotation_about_axis(o, Vec3::new(0.0, 0.0, 1.0), turn);
    placed(at, turn * tilt, c)
}

/// The volume the ball `(r, c)` shares with the box `b`.
fn ball_in_box(r: f64, c: Vec3<f64>, b: Bounds) -> f64 {
    crate::common::oracles::ball_in_box(r, (c.x, c.y, c.z), b)
}

fn box_volume(b: Bounds) -> f64 {
    b.iter().map(|(lo, hi)| hi - lo).product()
}

/// What one op yields.
#[derive(Clone, Copy, Debug)]
enum Want {
    /// A body at tiers 3 and 3′ and at its volume.
    Body,
    /// Two lumps at tier 3 and at their volume, whose tier 3′ the census
    /// cannot decide: curved faces of the two lumps are within reach of
    /// each other
    /// (`work/contact/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
    Lumps,
    /// The result gate's refusal of the ball face the ring holes.
    Gate,
}

/// `a ∪ b`, `b ∪ a`, `a ∖ b`, `b ∖ a`, `a ∩ b`, `b ∩ a` against `wants`,
/// in that order, each body's volume read from the operands' volumes
/// `va`, `vb` and the volume they share.
fn assert_six(
    pose: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    (va, vb, shared): (f64, f64, f64),
    wants: [Want; 6],
) {
    let (tol, none) = (Tol::witness(), BooleanDeclarations::none());
    let ops = [
        (
            "a ∪ b",
            va + vb - shared,
            topo::union_with(a, b, &none, tol),
        ),
        (
            "b ∪ a",
            va + vb - shared,
            topo::union_with(b, a, &none, tol),
        ),
        ("a ∖ b", va - shared, topo::subtract_with(a, b, &none, tol)),
        ("b ∖ a", vb - shared, topo::subtract_with(b, a, &none, tol)),
        ("a ∩ b", shared, topo::intersect_with(a, b, &none, tol)),
        ("b ∩ a", shared, topo::intersect_with(b, a, &none, tol)),
    ];
    for ((op, volume, out), want) in ops.into_iter().zip(wants) {
        let label = format!("{pose}, {op}");
        match (out, want) {
            (Ok(BooleanResult::Body(bb)), Body | Lumps) => {
                topo::validate_geometric(&bb.body, tol)
                    .unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
                match (
                    topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
                    want,
                ) {
                    (Ok(()), Body) => {}
                    (Err(errors), Lumps)
                        if bb.body.shells().count() == 2
                            && errors.iter().all(|e| {
                                matches!(e, topo::ValidationError::CensusUndecidable { .. })
                            }) => {}
                    (got, _) => panic!("{label}: tier 3′ for {want:?}: {got:?}"),
                }
                let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
                assert!(
                    (v - volume).abs() <= 1e-9 * volume.max(1.0),
                    "{label}: volume {v} against the slice integral {volume}"
                );
            }
            (Err(BooleanError::ResultInvalid { errors }), Gate)
                if matches!(
                    errors.as_slice(),
                    [topo::ValidationError::VolumeUncomputable {
                        source: topo::MassPropsError::RingOnCurvedFace { .. },
                        ..
                    }]
                ) => {}
            (out, want) => panic!(
                "{label}: wanted {want:?}, got {:?}",
                out.map(|r| r.body().map(|b| b.body.faces().count()))
            ),
        }
    }
}

/// A box against a ball whose face keeps the section as a hole on the
/// ball's side of ∪ and of ball ∖ box: those two refuse at the result
/// gate in both member orders, and the box's ∖ and ∩ build.
const BOX_RING: [Want; 6] = [Gate, Gate, Body, Gate, Body, Body];

/// The operand and shared volumes of the box `bounds` against the ball
/// `(r, c)`.
fn box_ball(bounds: Bounds, r: f64, c: Vec3<f64>) -> (f64, f64, f64) {
    (
        box_volume(bounds),
        ball_volume(r),
        ball_in_box(r, c, bounds),
    )
}

/// **The review probe's pose builds wherever its result has a closed
/// form.** The slab against `ball_poled_y(0.5)` at `(0.3, 2, 1.2)`: the
/// slab's top edge pierces the ball's face and the section lands as a
/// ring, whose island lies below the top face's plane and the ball's
/// poles above it. Mirrored below the slab, the arc closing the run
/// leans the other way.
#[test]
fn a_slab_edge_through_a_ball_face_winds_its_island() {
    for (pose, c) in [
        ("the probe", Vec3::new(0.3, 2.0, 1.2)),
        ("mirrored below", Vec3::new(0.3, 2.0, -0.2)),
        // The ring passes within 0.1 of the pole at (2, −0.1, 1.1).
        ("near a pole", Vec3::new(2.0, 0.4, 1.1)),
    ] {
        assert_six(
            pose,
            &boxed(SLAB),
            &ball_y(0.5, c),
            box_ball(SLAB, 0.5, c),
            BOX_RING,
        );
    }
}

/// **An outer-loop point on the run's side of the section plane winds
/// the island.** A `z`-poled ball at `(−0.3, 2, 1.2)`, its seam turned to
/// `−y` and its poles tilted off `z`: the lower pole lies on the run's
/// side of the top face's plane, and the great-circle path from it to
/// the closing arc says, by its crossings and its arrival, that it is
/// outside the island.
#[test]
fn an_outer_point_on_the_runs_side_of_the_section_winds_the_island() {
    let c = Vec3::new(-0.3, 2.0, 1.2);
    for tilt in [0.3, -0.3, 0.6] {
        let pose = format!("tilt {tilt}");
        let ball = ball_z(0.5, c, tilt, 3.0 * FRAC_PI_2);
        assert_six(&pose, &boxed(SLAB), &ball, box_ball(SLAB, 0.5, c), BOX_RING);
    }
}

/// **The item's box poses**: the unit ball at the origin against a box
/// with a corner inside it, and against one with an edge through it
/// (PR 4046's dual review).
#[test]
fn a_box_corner_and_a_box_edge_inside_a_ball_wind_their_islands() {
    let o = Vec3::new(0.0, 0.0, 0.0);
    for (pose, bounds) in [
        ("a corner inside", [(0.3, 2.0), (0.2, 2.0), (0.1, 2.0)]),
        ("an edge through", [(-0.2, 2.0), (-2.0, 2.0), (0.4, 2.0)]),
    ] {
        assert_six(
            pose,
            &boxed(bounds),
            &ball_y(1.0, o),
            box_ball(bounds, 1.0, o),
            BOX_RING,
        );
    }
}

/// **A ring that is the whole section circle** winds by the arc alone:
/// the run lies on the circle with the arc, so the island is the cap the
/// arc leans into. The unit ball at the origin against `ball(0.6)` at
/// `1.2·(0.6, 0, 0.8)`, whose seam the section crosses: the circle lands
/// as a ring of the unit ball's face, which ∪ and unit ∖ small keep.
#[test]
fn a_ring_on_the_section_circle_winds_by_its_arc() {
    let (c, d) = (Vec3::new(0.72, 0.0, 0.96), 1.2);
    let (big, small) = (ball_y(1.0, Vec3::new(0.0, 0.0, 0.0)), ball_y(0.6, c));
    assert_six(
        "the unit ball against ball(0.6)",
        &big,
        &small,
        (ball_volume(1.0), ball_volume(0.6), lens_volume(1.0, 0.6, d)),
        [Gate, Gate, Gate, Body, Body, Body],
    );
}

/// **A section crossing the ball's seam meridian is no ring.** The probe
/// ball lowered to `(0.3, 2, 0.8)`: its seam meridian, in the plane
/// `z = 0.8` on `+x`, runs through the slab, so the section reaches the
/// face's boundary and divides it by the outer loop. Every op builds.
#[test]
fn a_section_across_the_seam_meridian_divides_the_outer_loop() {
    let c = Vec3::new(0.3, 2.0, 0.8);
    assert_six(
        "the seam through the slab",
        &boxed(SLAB),
        &ball_y(0.5, c),
        box_ball(SLAB, 0.5, c),
        [Body; 6],
    );
}

/// **A bystander ring on a sphere face is re-homed by a path.** The lens
/// union `ball(1, (2, 2, 0.5)) ∪ ball(1, (3.4, 2, 0.5))` against
/// `ball(0.5, (2.7, 2, 1.3))`: the lens's crease pierces the small ball's
/// face twice, and the pierce ring left when the first chord walls its
/// island off is read outside it. Every op builds; the three-ball
/// shared volume has no closed form here, so the ops are held to one
/// another: each is the operands' volumes and the ∩ volume combined.
#[test]
fn a_pierce_ring_beside_a_sphere_island_stays_outside() {
    let tol = Tol::witness();
    let lens = topo::union(
        &ball_y(1.0, Vec3::new(2.0, 2.0, 0.5)),
        &ball_y(1.0, Vec3::new(3.4, 2.0, 0.5)),
        tol,
    )
    .unwrap()
    .body()
    .unwrap()
    .body
    .clone();
    let small = ball_y(0.5, Vec3::new(2.7, 2.0, 1.3));
    let va = topo::mass_properties(&lens, tol).unwrap().volume;
    let vb = ball_volume(0.5);
    let common = topo::mass_properties(
        &topo::intersect(&lens, &small, tol)
            .unwrap()
            .body()
            .unwrap()
            .body,
        tol,
    )
    .unwrap()
    .volume;
    assert!(common > 0.0 && common < vb, "the shared volume {common}");
    assert_six(
        "the lens union against a crease ball",
        &lens,
        &small,
        (va, vb, common),
        [Body; 6],
    );
}

/// **A ring inside a sphere island is re-homed into it, and the op stops
/// at the role read.** The slab with a well `[0.1, 0.4] × [1.8, 2.2]`
/// down to `z = 0.5` against the probe ball: the well's walls cut the
/// ball's face in a ring inside the island the slab's top edge walls
/// off, which the path reads inside and moves. The section loop about
/// the well then flanks only sphere patches, every witness of which
/// lies on the slab, and the role read refuses
/// (`work/cleave/the-uncut-shell-witness-reads-no-curved-face-interior.md`).
#[test]
fn a_ring_inside_a_sphere_island_moves_and_stops_at_the_role_read() {
    let tol = Tol::witness();
    let well = boxed([(0.1, 0.4), (1.8, 2.2), (0.5, 2.0)]);
    let slab = topo::subtract(&boxed(SLAB), &well, tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    let ball = ball_y(0.5, Vec3::new(0.3, 2.0, 1.2));
    let none = BooleanDeclarations::none();
    for (op, out) in [
        ("a ∪ b", topo::union_with(&slab, &ball, &none, tol)),
        ("b ∪ a", topo::union_with(&ball, &slab, &none, tol)),
        ("a ∖ b", topo::subtract_with(&slab, &ball, &none, tol)),
        ("b ∖ a", topo::subtract_with(&ball, &slab, &none, tol)),
        ("a ∩ b", topo::intersect_with(&slab, &ball, &none, tol)),
        ("b ∩ a", topo::intersect_with(&ball, &slab, &none, tol)),
    ] {
        assert!(
            matches!(
                out,
                Err(BooleanError::Join(
                    topo::SplitJoinError::SectionLoopUndecided { .. }
                ))
            ),
            "the well, {op}: wanted the role read's refusal, got {:?}",
            out.map(|_| "a body")
        );
    }
}

/// The unit ball at the origin, its poles along `pole`, spun `spin`
/// about world `+y` and then tilted `tilt` about `tilt_axis`.
fn unit_ball(pole: Vec3<f64>, spin: f64, (tilt_axis, tilt): (Vec3<f64>, f64)) -> AtRestBody<f64> {
    let o = Point3::origin();
    let at = ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), pole, Tol::witness());
    let spin = Affine3::rotation_about_axis(o, Vec3::new(0.0, 1.0, 0.0), spin);
    let tilt = Affine3::rotation_about_axis(o, tilt_axis, tilt);
    placed(at, tilt * spin, Vec3::new(0.0, 0.0, 0.0))
}

/// **An island that holds the far cap's pole.** A tool covering the
/// unit ball but for the cap `x > 0.6`, widened by a notch, against the
/// ball poled on `y` and spun so its seam meridian lies in `x = 0`, on
/// the run's side of the section plane `x = 0.6`. The island is the
/// region holding the far cap's pole, and the outer-loop point lies
/// beside the run, not across the section from it. Ball ∖ tool is the
/// cap with the notch: it keeps no ring and builds.
#[test]
fn an_island_holding_the_far_caps_pole_winds() {
    let big: Bounds = [(-2.0, 0.6), (-2.0, 2.0), (-2.0, 2.0)];
    let o = Vec3::new(0.0, 0.0, 0.0);
    for (name, notch) in [
        ("notch y > 0.5", [(0.4, 3.0), (0.5, 3.0), (-0.3, 0.3)]),
        ("notch y > 0.3", [(0.3, 3.0), (0.3, 3.0), (-0.25, 0.35)]),
        ("notch y < −0.4", [(0.45, 3.0), (-3.0, -0.4), (-0.2, 0.3)]),
        ("notch on z = 0 above", [(0.4, 3.0), (0.5, 3.0), (0.0, 0.3)]),
        (
            "notch on z = 0 below",
            [(0.4, 3.0), (0.5, 3.0), (-0.3, 0.0)],
        ),
    ] {
        let tol = Tol::witness();
        let tool = topo::subtract(&boxed(big), &boxed(notch), tol)
            .unwrap()
            .body()
            .unwrap()
            .body
            .clone();
        let vt = topo::mass_properties(&tool, tol).unwrap().volume;
        let cut: Bounds = [(notch[0].0, 0.6), notch[1], notch[2]];
        let shared = ball_in_box(1.0, o, big) - ball_in_box(1.0, o, cut);
        for tilt in [
            (Vec3::new(1.0, 0.0, 0.0), 0.0),
            (Vec3::new(1.0, 0.0, 0.0), 0.15),
            (Vec3::new(0.0, 0.0, 1.0), -0.2),
        ] {
            for spin in [FRAC_PI_2, -FRAC_PI_2] {
                assert_six(
                    &format!("{name}, tilt {}, spin {spin:.2}", tilt.1),
                    &tool,
                    &unit_ball(Vec3::new(0.0, 1.0, 0.0), spin, tilt),
                    (vt, ball_volume(1.0), shared),
                    [Body, Body, Gate, Body, Gate, Gate],
                );
            }
        }
    }
}

/// **A bar through the ball** leaves two rings on its face; bar ∖ ball
/// is two lumps.
#[test]
fn a_bar_through_a_ball_winds_both_rings() {
    let o = Vec3::new(0.0, 0.0, 0.0);
    for pole in [
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.3, 0.8, 0.52).normalize(),
    ] {
        for (name, b) in [
            ("bar x", [(-2.0, 2.0), (-0.2, 0.25), (0.1, 0.4)]),
            ("bar x off", [(-2.0, 2.0), (0.3, 0.5), (-0.45, -0.2)]),
            ("bar z", [(-0.3, 0.1), (0.2, 0.45), (-2.0, 2.0)]),
        ] {
            let ball = unit_ball(pole, 0.0, (Vec3::new(1.0, 0.0, 0.0), 0.0));
            assert_six(
                &format!("{name}, pole {pole:?}"),
                &boxed(b),
                &ball,
                box_ball(b, 1.0, o),
                [Gate, Gate, Lumps, Gate, Body, Body],
            );
        }
    }
}

/// **A ring re-homed where every vertex of the old face's outer loop is
/// on the run** reads its side from an edge midpoint of that loop. Two
/// bars through the unit ball and a box over a corner of it, the ball's
/// poles turned off every axis: the old face's outer loop is all copies
/// of run vertices, so a path to any of its vertices ends on the run and
/// says nothing (review finding m1).
#[test]
fn a_ring_beside_an_outer_loop_on_the_run_is_read_from_an_edge_midpoint() {
    let o = Vec3::new(0.0, 0.0, 0.0);
    let pole = Vec3::new(-0.6, 0.2, 0.77).normalize();
    for b in [
        [(-2.0, 2.0), (-0.2, 0.25), (0.1, 0.4)],
        [(-0.3, 0.1), (0.2, 0.45), (-2.0, 2.0)],
    ] {
        let ball = unit_ball(pole, 0.0, (Vec3::new(1.0, 0.0, 0.0), 0.0));
        assert_six(
            &format!("bar {b:?}"),
            &boxed(b),
            &ball,
            box_ball(b, 1.0, o),
            [Gate, Gate, Lumps, Gate, Body, Body],
        );
    }
    let b: Bounds = [
        (-0.6237172865476482, 1.233346104703386),
        (-0.5631024588741076, 0.346881547918797),
        (-0.7919341754260856, -0.23917581093071405),
    ];
    let pole = Vec3::new(0.6355369378990602, -0.7198546525528592, -0.2791094404779036);
    let ball = unit_ball(pole, 0.0, (Vec3::new(1.0, 0.0, 0.0), 0.0));
    assert_six(
        "the corner box",
        &boxed(b),
        &ball,
        box_ball(b, 1.0, o),
        [Gate, Gate, Lumps, Gate, Body, Body],
    );
}

/// **A path that grazes a run arc in the escalation band says nothing,
/// and the next one winds the island.** Two millimetre balls with poles
/// turned off every axis (a review's random pair 82): the first
/// outer-loop point's great circle crosses a run arc's plane in the
/// escalation band, and the reading moves on to the next point rather
/// than refusing the op. The small ball's ∖ and both ∩ build at the
/// lens; the ops whose big ball keeps the ring as a hole refuse at the
/// result gate.
#[test]
fn a_path_escalating_at_a_graze_hands_the_winding_to_the_next() {
    let tol = Tol::witness();
    let (r1, r2) = (6.098088671076322e-4, 1.4613194916300017e-3);
    let c2 = Vec3::new(
        -0.0018727710410726642,
        -5.7762472824686515e-5,
        -3.412205707271761e-5,
    );
    let p1 = Vec3::new(
        -0.20107294329337744,
        -0.891895479861727,
        -0.4050828612488538,
    );
    let p2 = Vec3::new(
        0.01866131486960719,
        -0.9760726504487204,
        -0.21664241591467578,
    );
    let a = finished(
        "the small ball",
        ball_poled(r1, Vec3::new(0.0, 0.0, 0.0), p1, tol),
        tol,
    );
    let b = finished("the big ball", ball_poled(r2, c2, p2, tol), tol);
    assert_six(
        "pair 82",
        &a,
        &b,
        (
            ball_volume(r1),
            ball_volume(r2),
            lens_volume(r1, r2, c2.norm()),
        ),
        [Gate, Gate, Gate, Body, Body, Body],
    );
}
