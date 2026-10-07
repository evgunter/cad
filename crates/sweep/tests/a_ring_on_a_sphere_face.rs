//! **A ring on a sphere face winds its island without a chart.** A box
//! edge that pierces a ball's face lands the section there as a ring,
//! and the join's ring lane winds the island it walls off by the side
//! of the section plane the run lies on, the arc's lean, and an
//! outer-loop point read against the two caps
//! (`chord_join::sphere_island_winding`). A bystander ring of a sphere
//! face is re-homed by the parity of a great-circle path
//! (`chord_join::sphere_ring_side`).
//!
//! Every pose runs every op in both member orders. A body is held to
//! tiers 3 and 3′ and to its volume against a slice integral of the
//! ball and box: each slice is a disc against a rectangle, exact, and
//! only the stack over height is quadrature. A result whose ball face
//! keeps the ring as a hole (the ball's side of ∪ and of ball ∖ box)
//! refuses at the result gate, `VolumeUncomputable { RingOnCurvedFace }`
//! (`work/flux/sphere-face-with-a-hole-has-no-closed-form.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_y, ball_poled_z, brick, finished};
use topo::{AtRestBody, BooleanDeclarations, BooleanError, BooleanResult};

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

/// The area of the disc of radius `rho` at the origin left of `x` and
/// below `y`: `∫ (clamp(y, −s, s) + s) dX` over `X < x`, `s = √(ρ² − X²)`,
/// in closed form on the pieces where the clamp is fixed.
fn quadrant(rho: f64, x: f64, y: f64) -> f64 {
    let s = |t: f64| (rho * rho - t * t).max(0.0).sqrt();
    // `∫_{−ρ}^{t} s`.
    let arc = |t: f64| {
        let t = t.clamp(-rho, rho);
        0.5 * (t * s(t) + rho * rho * (t / rho).asin()) + 0.25 * PI * rho * rho
    };
    let x = x.clamp(-rho, rho);
    let a = s(y);
    // Inside `|X| < a` the clamp is `y`; outside it is `s` (above the
    // chord) or `−s` (below it).
    let inner = |lo: f64, hi: f64| {
        let (lo, hi) = (lo.max(-a), hi.min(a));
        if hi > lo {
            y * (hi - lo) + arc(hi) - arc(lo)
        } else {
            0.0
        }
    };
    let outer = |lo: f64, hi: f64| {
        if y < 0.0 {
            return 0.0;
        }
        let mut total = 0.0;
        for (p, q) in [(-rho, -a), (a, rho)] {
            let (p, q) = (p.max(lo), q.min(hi));
            if q > p {
                total += 2.0 * (arc(q) - arc(p));
            }
        }
        total
    };
    inner(-rho, x) + outer(-rho, x)
}

/// The area of the disc of radius `rho` centred `(cx, cy)` inside the
/// rectangle `[x0, x1] × [y0, y1]`.
fn disc_in_rect(rho: f64, (cx, cy): (f64, f64), (x0, x1): (f64, f64), (y0, y1): (f64, f64)) -> f64 {
    let f = |x: f64, y: f64| quadrant(rho, x - cx, y - cy);
    f(x1, y1) - f(x0, y1) - f(x1, y0) + f(x0, y0)
}

/// Adaptive Simpson on `[a, b]`.
fn simpson(f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    /// One panel `(a, b)`, its end and middle values, and its estimate.
    fn step(
        f: &dyn Fn(f64) -> f64,
        (a, b): (f64, f64),
        (fa, fm, fb): (f64, f64, f64),
        whole: f64,
        depth: u32,
    ) -> f64 {
        let (m, h) = (0.5 * (a + b), b - a);
        let (flm, frm) = (f(0.5 * (a + m)), f(0.5 * (m + b)));
        let left = h / 12.0 * (fa + 4.0 * flm + fm);
        let right = h / 12.0 * (fm + 4.0 * frm + fb);
        if depth == 0 || (left + right - whole).abs() <= 1e-15 {
            return left + right + (left + right - whole) / 15.0;
        }
        step(f, (a, m), (fa, flm, fm), left, depth - 1)
            + step(f, (m, b), (fm, frm, fb), right, depth - 1)
    }
    let (fa, fm, fb) = (f(a), f(0.5 * (a + b)), f(b));
    step(
        f,
        (a, b),
        (fa, fm, fb),
        (b - a) / 6.0 * (fa + 4.0 * fm + fb),
        40,
    )
}

/// The volume the ball `(r, c)` shares with the box `b`: the stack of
/// its slices' disc-in-rectangle areas over height.
fn ball_in_box(r: f64, c: Vec3<f64>, b: Bounds) -> f64 {
    let (lo, hi) = ((c.z - r).max(b[2].0), (c.z + r).min(b[2].1));
    if hi <= lo {
        return 0.0;
    }
    let area = |z: f64| {
        let rho = (r * r - (z - c.z).powi(2)).max(0.0).sqrt();
        disc_in_rect(rho, (c.x, c.y), b[0], b[1])
    };
    simpson(&area, lo, hi)
}

fn ball_volume(r: f64) -> f64 {
    4.0 / 3.0 * PI * r.powi(3)
}

fn box_volume(b: Bounds) -> f64 {
    b.iter().map(|(lo, hi)| hi - lo).product()
}

/// The lens two balls `r1`, `r2` at centre distance `d` share.
fn lens_volume(r1: f64, r2: f64, d: f64) -> f64 {
    let cap = |r: f64, h: f64| PI * h * h * (3.0 * r - h) / 3.0;
    let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    cap(r1, r1 - x) + cap(r2, r2 - (d - x))
}

/// What one op yields: a body at this volume, or the result gate's
/// refusal of the ball face the ring holes.
#[derive(Clone, Copy, Debug)]
enum Want {
    Body(f64),
    RingedSphere,
}

/// `a ∪ b`, `b ∪ a`, `a ∖ b`, `b ∖ a`, `a ∩ b`, `b ∩ a` against `wants`,
/// in that order. A body holds tiers 3 and 3′ and its volume.
fn assert_six(pose: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, wants: [Want; 6]) {
    let (tol, none) = (Tol::witness(), BooleanDeclarations::none());
    let ops = [
        ("a ∪ b", topo::union_with(a, b, &none, tol)),
        ("b ∪ a", topo::union_with(b, a, &none, tol)),
        ("a ∖ b", topo::subtract_with(a, b, &none, tol)),
        ("b ∖ a", topo::subtract_with(b, a, &none, tol)),
        ("a ∩ b", topo::intersect_with(a, b, &none, tol)),
        ("b ∩ a", topo::intersect_with(b, a, &none, tol)),
    ];
    for ((op, out), want) in ops.into_iter().zip(wants) {
        let label = format!("{pose}, {op}");
        match (out, want) {
            (Ok(BooleanResult::Body(bb)), Want::Body(volume)) => {
                topo::validate_geometric(&bb.body, tol)
                    .unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
                    .unwrap_or_else(|e| panic!("{label}: tier 3′: {e:?}"));
                let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
                assert!(
                    (v - volume).abs() <= 1e-9 * volume.max(1.0),
                    "{label}: volume {v} against the slice integral {volume}"
                );
            }
            (Err(BooleanError::ResultInvalid { errors }), Want::RingedSphere)
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
fn box_ring_wants(bounds: Bounds, r: f64, c: Vec3<f64>) -> [Want; 6] {
    let shared = ball_in_box(r, c, bounds);
    let (ring, common) = (Want::RingedSphere, Want::Body(shared));
    [
        ring,
        ring,
        Want::Body(box_volume(bounds) - shared),
        ring,
        common,
        common,
    ]
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
            box_ring_wants(SLAB, 0.5, c),
        );
    }
}

/// **An outer-loop point on the island's side of the plane is read
/// against the caps.** A `z`-poled ball at `(−0.3, 2, 1.2)`, its seam
/// turned to `−y` and its poles tilted off `z`: the lower pole lies on
/// the run's side of the top face's plane, so the great-circle path
/// from it to the far cap's pole says, by its crossings, that it is
/// outside the island.
#[test]
fn an_outer_point_on_the_runs_side_is_read_by_a_path() {
    let c = Vec3::new(-0.3, 2.0, 1.2);
    for tilt in [0.3, -0.3, 0.6] {
        let pose = format!("tilt {tilt}");
        let ball = ball_z(0.5, c, tilt, 3.0 * FRAC_PI_2);
        assert_six(&pose, &boxed(SLAB), &ball, box_ring_wants(SLAB, 0.5, c));
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
            box_ring_wants(bounds, 1.0, o),
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
    let shared = lens_volume(1.0, 0.6, d);
    let ring = Want::RingedSphere;
    assert_six(
        "the unit ball against ball(0.6)",
        &big,
        &small,
        [
            ring,
            ring,
            ring,
            Want::Body(ball_volume(0.6) - shared),
            Want::Body(shared),
            Want::Body(shared),
        ],
    );
}

/// **A section crossing the ball's seam meridian is no ring.** The probe
/// ball lowered to `(0.3, 2, 0.8)`: its seam meridian, in the plane
/// `z = 0.8` on `+x`, runs through the slab, so the section reaches the
/// face's boundary and divides it by the outer loop. Every op builds.
#[test]
fn a_section_across_the_seam_meridian_divides_the_outer_loop() {
    let c = Vec3::new(0.3, 2.0, 0.8);
    let shared = ball_in_box(0.5, c, SLAB);
    let (slab, ball) = (box_volume(SLAB), ball_volume(0.5));
    let common = Want::Body(shared);
    let union = Want::Body(slab + ball - shared);
    assert_six(
        "the seam through the slab",
        &boxed(SLAB),
        &ball_y(0.5, c),
        [
            union,
            union,
            Want::Body(slab - shared),
            Want::Body(ball - shared),
            common,
            common,
        ],
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
    let (u, k) = (Want::Body(va + vb - common), Want::Body(common));
    assert_six(
        "the lens union against a crease ball",
        &lens,
        &small,
        [u, u, Want::Body(va - common), Want::Body(vb - common), k, k],
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
