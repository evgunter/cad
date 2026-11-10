//! **A fillet at a knife edge builds where its ball is defined** (BAND,
//! `work/band/a-knife-edge-blend-is-unbuilt-though-its-rolling-ball-is-defined.md`).
//!
//! A cusp (wedge 0) or slit (wedge 2π) is two supports tangent along
//! the edge with their material facing each other. Its rolling ball
//! rests in the crescent they open into — in the material at a cusp,
//! in the void at a slit — centred where the supports' `r`-offsets
//! cross away from the edge, on the side both faces leave it by. Each
//! row below builds one such body through a public door the user can
//! author it with and checks the fillet against closed forms: its
//! volume change, the crescent's horn cut off by the ball (Green's
//! theorem over the three curves that bound it, times the prism's
//! depth, or carried round the axis by Pappus for a revolved rim), and,
//! on one row of each prism body, point membership at a lattice against
//! the result's region (`common::membership::holds`), which sees a
//! move the volume alone would cancel. A cusp loses that horn, a slit
//! gains it.
//!
//! The refusals are pinned beside them: a radius past the room the
//! crescent opens to (`KnifeRefusal::NoRoom`), a foot past the end of
//! its support (the clearance screen), the lune slit from `r = 2/3`
//! (the cap-sliver screen reading an exact zero,
//! `work/band/the-cap-sliver-screen-reads-an-exact-zero-where-a-lune-slits-wall-ends-at-its-arcs-centre.md`),
//! and the chamfer, whose one arm is plane–plane
//! (`ChamferArmUnsupported`). The mirrored and rotated crescents guard
//! the side selector: the closed form the ball's centre was picked by
//! before read an exact zero at a knife and chose by the sign of
//! rounding noise.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, PI};

use geom_core::{Point2, Point3, Tol, Vec2};
use profile::test_support::bulge_loop;
use profile::{Open, ProfileLoop, SketchPlane, Start};
use sweep::blend::{BlendError, KnifeRefusal, chamfer_edges, fillet_edges};
use sweep::test_support::{corners, extruded, finished};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{AtRestBody, Body, BooleanResult, ContactMark, EdgeKey};

use crate::common::membership;

fn tol() -> Tol {
    Tol::witness()
}

/// `loops` extruded over `z ∈ [0, 1]`.
fn prism(loops: Vec<ProfileLoop<f64>>) -> Body<f64> {
    extruded(SketchPlane::xy(), loops, 1.0, tol())
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> ProfileLoop<f64> {
    bulge_loop(corners(&[(x0, y0), (x1, y0), (x1, y1), (x0, y1)]))
}

/// The circle about `(cx, cy)` of radius `r`, as two half-turn bulges
/// (no `test_support` fixture spells a circle loop).
fn circle(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(cx + r, cy), 1.0),
        (Point2::new(cx - r, cy), 1.0),
    ])
}

/// The one edge check 4 marks `Tangent` with both ends satisfying `at`.
fn knife_at(what: &str, body: &Body<f64>, at: impl Fn(&Point3<f64>) -> bool) -> EdgeKey {
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let found: Vec<EdgeKey> = topo::contact_marks(body, tol())
        .expect("the body is tier-3 valid")
        .iter()
        .filter(|(_, m)| **m == ContactMark::Tangent)
        .map(|(e, _)| e)
        .filter(|&e| {
            let (v0, v1) = body.edge_vertices(e).unwrap();
            at(&point(v0)) && at(&point(v1))
        })
        .collect();
    let [edge] = found[..] else {
        panic!("{what}: one knife edge, found {found:?}");
    };
    edge
}

fn on_the_z_axis(p: &Point3<f64>) -> bool {
    p.x.abs() < 1e-9 && p.y.abs() < 1e-9
}

fn volume(what: &str, body: &AtRestBody<f64>) -> f64 {
    topo::test_support::pinned_volume(what, body, tol()).volume
}

/// The fillet of `edge` at `r`, built.
fn filleted(what: &str, operand: &AtRestBody<f64>, edge: EdgeKey, r: f64) -> AtRestBody<f64> {
    let out = fillet_edges(operand, &[edge], r, tol()).unwrap_or_else(|e| {
        panic!(
            "{what} at r = {r}: refused {:?}: {}",
            e.error,
            e.render(geom_brep::recourse::Reading::Build)
        )
    });
    assert!(
        !out.blend_faces.is_empty() || !out.band_faces.is_empty(),
        "{what} at r = {r}: no band built"
    );
    finished(what, out.body, tol())
}

/// The volume the fillet of `edge` at `r` changed.
fn filleted_dv(what: &str, operand: &AtRestBody<f64>, edge: EdgeKey, r: f64) -> f64 {
    volume(what, &filleted(what, operand, edge, r)) - volume(what, operand)
}

fn assert_close(what: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-9 * want.abs().max(1.0),
        "{what}: ΔV {got}, closed form {want}"
    );
}

// ---------------------------------------------------------------------
// The closed forms: Green's theorem over a region bounded by circular
// arcs and segments, its area and its first moment about the y axis;
// and each result's region, as a signed margin for the membership
// lattice (positive inside).
// ---------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Piece {
    /// The short arc of the circle about `c` of radius `r` from `from`
    /// to `to`.
    Arc {
        c: [f64; 2],
        r: f64,
        from: [f64; 2],
        to: [f64; 2],
    },
    /// The segment from `from` to `to`.
    Line { from: [f64; 2], to: [f64; 2] },
}

/// `(∮ ½(x dy − y dx), ∮ ½x² dy)` over the closed boundary `pieces`:
/// the signed area and the signed first moment `∫∫ x dA`.
fn green(pieces: &[Piece]) -> (f64, f64) {
    pieces
        .iter()
        .fold((0.0, 0.0), |(a, m), piece| match *piece {
            Piece::Line { from: p, to: q } => {
                let (dx, dy) = (q[0] - p[0], q[1] - p[1]);
                (
                    a + 0.5 * (p[0] * q[1] - q[0] * p[1]),
                    m + 0.5 * dy * (p[0] * p[0] + p[0] * dx + dx * dx / 3.0),
                )
            }
            Piece::Arc { c, r, from, to } => {
                let t0 = (from[1] - c[1]).atan2(from[0] - c[0]);
                let sweep =
                    ((to[1] - c[1]).atan2(to[0] - c[0]) - t0 + PI).rem_euclid(2.0 * PI) - PI;
                let t1 = t0 + sweep;
                let area = |t: f64| 0.5 * (r * r * t + r * c[0] * t.sin() - r * c[1] * t.cos());
                let moment = |t: f64| {
                    0.5 * r
                        * (c[0] * c[0] * t.sin()
                            + 2.0 * c[0] * r * (t / 2.0 + (2.0 * t).sin() / 4.0)
                            + r * r * (t.sin() - t.sin().powi(3) / 3.0))
                };
                (a + area(t1) - area(t0), m + moment(t1) - moment(t0))
            }
        })
}

fn add(p: [f64; 2], v: [f64; 2], s: f64) -> [f64; 2] {
    [p[0] + v[0] * s, p[1] + v[1] * s]
}

/// Where a ball about `ball` touches the circle about `c` of radius
/// `rr`: on the line of centres, `rr` from `c`.
fn foot_on_circle(c: [f64; 2], rr: f64, ball: [f64; 2]) -> [f64; 2] {
    let d = [ball[0] - c[0], ball[1] - c[1]];
    add(c, d, rr / d[0].hypot(d[1]))
}

/// A planar region in the sketch plane, a signed margin at `(x, y)`.
type Margin2 = Box<dyn Fn(f64, f64) -> f64>;

/// `m2` over the prism's depth `z ∈ [0, 1]`, for the lattice.
fn over_depth(m2: Margin2) -> impl Fn(Point3<f64>) -> f64 {
    move |p| m2(p.x, p.y).min(p.z).min(1.0 - p.z)
}

/// Inside the circle about `c` of radius `r`.
fn inside(c: [f64; 2], r: f64) -> impl Fn(f64, f64) -> f64 {
    move |x, y| r - (x - c[0]).hypot(y - c[1])
}

/// The side of the line through `a` and `b` that `tip` lies on.
fn tip_side(a: [f64; 2], b: [f64; 2], tip: [f64; 2]) -> impl Fn(f64, f64) -> f64 {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let len = dx.hypot(dy);
    let (nx, ny) = (-dy / len, dx / len);
    let sign = ((tip[0] - a[0]) * nx + (tip[1] - a[1]) * ny).signum();
    move |x, y| sign * ((x - a[0]) * nx + (y - a[1]) * ny)
}

/// A crescent's horn, cut off at the ball: between the two circles
/// (`between`), outside the ball, and on the knife's side of the chord
/// through the ball's two feet.
struct Horn {
    area: f64,
    region: Margin2,
}

/// The horn a ball about `ball` of radius `r` cuts off a crescent whose
/// knife is at `rim`, between the circles `a` and `b` (each
/// `(centre, radius)`), the crescent read by `between`.
fn two_circle_horn(
    rim: [f64; 2],
    (c_a, r_a): ([f64; 2], f64),
    (c_b, r_b): ([f64; 2], f64),
    ball: [f64; 2],
    r: f64,
    between: Margin2,
) -> Horn {
    let (f_a, f_b) = (
        foot_on_circle(c_a, r_a, ball),
        foot_on_circle(c_b, r_b, ball),
    );
    let area = green(&[
        Piece::Arc {
            c: c_a,
            r: r_a,
            from: rim,
            to: f_a,
        },
        Piece::Arc {
            c: ball,
            r,
            from: f_a,
            to: f_b,
        },
        Piece::Arc {
            c: c_b,
            r: r_b,
            from: f_b,
            to: rim,
        },
    ])
    .0
    .abs();
    let (out_ball, side) = (inside(ball, r), tip_side(f_a, f_b, rim));
    Horn {
        area,
        region: Box::new(move |x, y| between(x, y).min(-out_ball(x, y)).min(side(x, y))),
    }
}

// ---------------------------------------------------------------------
// The lune: a cusp, and the same lune as a hole, a slit.
// ---------------------------------------------------------------------

/// The lune between `(0,1) r 1` and `(0,2) r 2` (inside the big circle,
/// outside the small one, `x > 0`), its area `3π/2`.
fn lune_region() -> Margin2 {
    let (big, small) = (inside([0.0, 2.0], 2.0), inside([0.0, 1.0], 1.0));
    Box::new(move |x, y| big(x, y).min(-small(x, y)).min(x))
}

/// The lune's horn ([`sweep::test_support::lune`]): the ball touching
/// the small circle from outside and the big one from inside sits at
/// `(√(8r(1 − r)), 3r)`, and exists while `r < 2 − 1`.
fn lune_horn(r: f64) -> Horn {
    let ball = [(8.0 * r * (1.0 - r)).sqrt(), 3.0 * r];
    let (big, small) = (inside([0.0, 2.0], 2.0), inside([0.0, 1.0], 1.0));
    two_circle_horn(
        [0.0, 0.0],
        ([0.0, 1.0], 1.0),
        ([0.0, 2.0], 2.0),
        ball,
        r,
        Box::new(move |x, y| big(x, y).min(-small(x, y))),
    )
}

fn lune_rows() -> [(&'static str, Body<f64>, f64); 2] {
    let lune = sweep::test_support::lune(tol());
    [
        ("the lune cusp", prism(vec![lune.clone()]), -1.0),
        (
            "the lune slit",
            prism(vec![rect(-1.0, -1.0, 3.0, 5.0), lune]),
            1.0,
        ),
    ]
}

/// The lattice over the lune, the plate's hole and a margin past both.
fn lune_lattice() -> (Point3<f64>, Point3<f64>) {
    (Point3::new(-0.2, -0.2, -0.1), Point3::new(2.2, 4.2, 1.1))
}

#[test]
fn a_lune_cusp_loses_its_horn_and_a_lune_slit_gains_it() {
    for (what, body, sign) in lune_rows() {
        let edge = knife_at(what, &body, on_the_z_axis);
        let operand = finished(what, body, tol());
        for r in [0.05, 0.3, 0.6] {
            let dv = filleted_dv(what, &operand, edge, r);
            assert_close(&format!("{what} at r = {r}"), dv, sign * lune_horn(r).area);
        }
    }
}

/// **Membership, not only volume**: the cusp at `0.3` is the lune less
/// its horn, the slit the plate less the lune, with the horn filled.
#[test]
fn the_lune_fillets_place_every_lattice_point() {
    let r = 0.3;
    let [(cusp_what, cusp, _), (slit_what, slit, _)] = lune_rows();
    let area = lune_horn(r).area;
    let edge = knife_at(cusp_what, &cusp, on_the_z_axis);
    let built = filleted(cusp_what, &finished(cusp_what, cusp, tol()), edge, r);
    let (lune, cut) = (lune_region(), lune_horn(r).region);
    membership::holds(
        cusp_what,
        &built,
        1.5 * PI - area,
        &over_depth(Box::new(move |x, y| lune(x, y).min(-cut(x, y)))),
        lune_lattice(),
        10,
    );

    let edge = knife_at(slit_what, &slit, on_the_z_axis);
    let built = filleted(slit_what, &finished(slit_what, slit, tol()), edge, r);
    let (lune, fill) = (lune_region(), lune_horn(r).region);
    let plate = |x: f64, y: f64| (x + 1.0).min(3.0 - x).min(y + 1.0).min(5.0 - y);
    membership::holds(
        slit_what,
        &built,
        24.0 - 1.5 * PI + area,
        &over_depth(Box::new(move |x, y| {
            plate(x, y).min((-lune(x, y)).max(fill(x, y)))
        })),
        lune_lattice(),
        10,
    );
}

/// **Past the room the crescent opens to, the knife refuses**: the
/// lune's offsets cross only while `r < R_big − R_small = 1`, so at
/// `r = 1.2` there is no ball, and `fillet3_knife_room` refuses it
/// with that margin (the offsets' internal clearance,
/// `1 − |1 − 2r| = −0.4`), on the cusp and the slit alike.
#[test]
fn a_lune_refuses_a_ball_wider_than_its_crescent() {
    for (what, body, _) in lune_rows() {
        let edge = knife_at(what, &body, on_the_z_axis);
        let operand = finished(what, body, tol());
        let refusal = fillet_edges(&operand, &[edge], 1.2, tol())
            .err()
            .unwrap_or_else(|| panic!("{what}: built a ball wider than its crescent"));
        let BlendError::KnifeEdge {
            edge: at,
            margin,
            refusal: KnifeRefusal::NoRoom { radius },
        } = refusal.error
        else {
            panic!(
                "{what}: expected a knife with no room, got {:?}",
                refusal.error
            );
        };
        assert_eq!(at, edge, "{what}: the refusal names the knife");
        assert_eq!(radius, 1.2, "{what}");
        assert_eq!(margin.predicate, "fillet3_knife_room", "{what}");
        let room = margin
            .reading
            .diagnostic_f64_for_error_text()
            .value()
            .expect("a value reading");
        assert!((room + 0.4).abs() < 1e-9, "{what}: room {room}");
        let text = refusal.render(geom_brep::recourse::Reading::Build);
        assert!(
            text.contains(sweep::blend::FILLET3_KNIFE_RECOURSE),
            "{what}: {text}"
        );
    }
}

/// **The cusp builds up to its room; the slit stops at `r = 2/3`.**
///
/// The cusp builds at `0.95`, where the ball's own circle crosses the
/// lune's `x = 0` wall: only its arc facing the knife is the band, and
/// that arc stays clear of the wall.
///
/// The slit builds at `0.66` and refuses from `2/3`, though its band is
/// as clear of the wall as the cusp's: the cut-off's cap-sliver screen
/// (`surgery.rs`'s end-face cycle pass) reads the plate's `x = 0` wall
/// edge at an EXACT zero there. That edge runs from `(0, 4)` down to
/// `(0, 2)`, the big arc's centre, which lies at every radius on the
/// line through the ball's centre and its big foot — the boundary of
/// the screen's half-plane along the section's second axis — so that
/// term is zero on the edge at every `r`, and from `2/3` on no other term
/// of the enclosing region clears it. The reading is a bound, never a
/// crossing (`RingClearance { bounded: true }`): a correct shape refused
/// by a screen reading an exact zero at a reachable pose
/// (`work/band/the-cap-sliver-screen-reads-an-exact-zero-where-a-lune-slits-wall-ends-at-its-arcs-centre.md`).
/// This row pins today's boundary.
#[test]
fn the_lune_cusp_builds_to_its_room_and_the_slit_stops_at_two_thirds() {
    let [(cusp_what, cusp, cusp_sign), (slit_what, slit, slit_sign)] = lune_rows();
    let edge = knife_at(cusp_what, &cusp, on_the_z_axis);
    let operand = finished(cusp_what, cusp, tol());
    let dv = filleted_dv(cusp_what, &operand, edge, 0.95);
    assert_close(
        &format!("{cusp_what} at r = 0.95"),
        dv,
        cusp_sign * lune_horn(0.95).area,
    );

    let edge = knife_at(slit_what, &slit, on_the_z_axis);
    let operand = finished(slit_what, slit, tol());
    let dv = filleted_dv(slit_what, &operand, edge, 0.66);
    assert_close(
        &format!("{slit_what} at r = 0.66"),
        dv,
        slit_sign * lune_horn(0.66).area,
    );
    for r in [0.67, 0.8, 0.95] {
        let refusal = fillet_edges(&operand, &[edge], r, tol())
            .expect_err("the cap-sliver screen reads the wall at zero");
        assert!(
            matches!(
                refusal.error,
                BlendError::RingClearance { bounded: true, margin, .. }
                    if margin.sign == geom_core::Sign::Zero
            ),
            "{slit_what} at r = {r}: {:?}",
            refusal.error
        );
    }
}

// ---------------------------------------------------------------------
// The valley between two touching tubes on a plate: a slit, authored
// as a profile, through the Boolean in either order, and at Interval.
// ---------------------------------------------------------------------

/// The valley profile's vertices: two unit tubes on a plate, its top
/// running over the right tube's arc into the kiss at the origin and
/// out over the left tube's, the plate filling the lower cusp. The
/// profile door decides the kiss a cusp joint.
fn valley_vertices() -> Vec<((f64, f64), f64)> {
    let sweep_angle = PI - 0.6_f64.atan2(0.8);
    let b = (sweep_angle / 4.0).tan();
    vec![
        ((-1.8, -1.5), 0.0),
        ((1.8, -1.5), 0.0),
        ((1.8, 0.6), b),
        ((0.0, 0.0), b),
        ((-1.8, 0.6), 0.0),
    ]
}

fn valley() -> ProfileLoop<f64> {
    bulge_loop(
        valley_vertices()
            .into_iter()
            .map(|((x, y), b)| (Point2::new(x, y), b))
            .collect(),
    )
}

/// The valley profile's area, by Green's theorem over its pieces.
fn valley_area() -> f64 {
    green(&[
        Piece::Line {
            from: [-1.8, -1.5],
            to: [1.8, -1.5],
        },
        Piece::Line {
            from: [1.8, -1.5],
            to: [1.8, 0.6],
        },
        Piece::Arc {
            c: [1.0, 0.0],
            r: 1.0,
            from: [1.8, 0.6],
            to: [0.0, 0.0],
        },
        Piece::Arc {
            c: [-1.0, 0.0],
            r: 1.0,
            from: [0.0, 0.0],
            to: [-1.8, 0.6],
        },
        Piece::Line {
            from: [-1.8, 0.6],
            to: [-1.8, -1.5],
        },
    ])
    .0
    .abs()
}

/// The valley the ball fills between the unit tubes about `(±1, 0)`:
/// its centre stands at `(0, √(r² + 2r))`, `1 + r` from both axes.
fn valley_fill(r: f64) -> Horn {
    let ball = [0.0, (r * r + 2.0 * r).sqrt()];
    let (right, left) = (inside([1.0, 0.0], 1.0), inside([-1.0, 0.0], 1.0));
    two_circle_horn(
        [0.0, 0.0],
        ([1.0, 0.0], 1.0),
        ([-1.0, 0.0], 1.0),
        ball,
        r,
        Box::new(move |x, y| (-right(x, y)).min(-left(x, y)).min(y).min(1.0 - x.abs())),
    )
}

#[test]
fn the_valley_between_two_touching_tubes_fills() {
    let profile = profile::Profile::new(SketchPlane::xy(), vec![valley()])
        .validate(tol())
        .expect("the valley validates");
    assert_eq!(
        profile.loops()[0].cusp_joints(),
        &[3],
        "the kiss is a cusp joint"
    );
    let body = prism(vec![valley()]);
    let edge = knife_at("the valley", &body, on_the_z_axis);
    let operand = finished("the valley", body, tol());
    for r in [0.05, 0.3] {
        let dv = filleted_dv("the valley", &operand, edge, r);
        assert_close(&format!("the valley at r = {r}"), dv, valley_fill(r).area);
    }
    let r = 0.3;
    let built = filleted("the valley", &operand, edge, r);
    let (right, left) = (inside([1.0, 0.0], 1.0), inside([-1.0, 0.0], 1.0));
    let fill = valley_fill(r).region;
    membership::holds(
        "the valley",
        &built,
        valley_area() + valley_fill(r).area,
        &over_depth(Box::new(move |x, y| {
            let plate = (1.8 - x.abs())
                .min(y + 1.5)
                .min((-y).max(right(x, y)).max(left(x, y)));
            plate.max(fill(x, y))
        })),
        (Point3::new(-2.0, -1.7, -0.1), Point3::new(2.0, 1.2, 1.1)),
        10,
    );
}

/// **The valley through the Boolean**: a plate whose top face passes
/// through both tube axes and the kiss, unioned with the two tubes in
/// either order, is one solid with one knife on the kiss, and the
/// fillet fills it as it fills the authored valley.
#[test]
fn the_valley_through_the_boolean_fills_in_either_order() {
    let slab = |loops| finished("a slab", prism(loops), tol());
    let plate = slab(vec![rect(-1.8, -1.5, 1.8, 0.0)]);
    let left = slab(vec![circle(-1.0, 0.0, 1.0)]);
    let right = slab(vec![circle(1.0, 0.0, 1.0)]);
    let union =
        |what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>| match topo::union(a, b, tol()) {
            Ok(BooleanResult::Body(r)) => r.body,
            other => panic!("{what}: {other:?}"),
        };
    let plate_first = union(
        "(plate ∪ left) ∪ right",
        &union("plate ∪ left", &plate, &left),
        &right,
    );
    let tubes_first = union(
        "(left ∪ right) ∪ plate",
        &union("left ∪ right", &left, &right),
        &plate,
    );
    for (what, body) in [
        ("(plate ∪ left) ∪ right", plate_first),
        ("(left ∪ right) ∪ plate", tubes_first),
    ] {
        assert_eq!(body.solids().count(), 1, "{what}: one solid");
        let edge = knife_at(what, &body, on_the_z_axis);
        let r = 0.1;
        let dv = filleted_dv(what, &body, edge, r);
        assert_close(&format!("{what} at r = {r}"), dv, valley_fill(r).area);
    }
}

/// **The valley at Interval**: the knife's end, its room and its
/// crossing decide definitely over enclosures, the band builds tier-3
/// valid, and the volume's enclosure holds the closed form.
#[test]
fn the_valley_fills_at_interval() {
    use crate::common::interval::{iv, p2};
    use geom_core::{Bounds, Interval};
    let r = 0.1;
    let loop_iv: ProfileLoop<Interval> = bulge_loop(
        valley_vertices()
            .into_iter()
            .map(|((x, y), b)| (p2(x, y), iv(b)))
            .collect(),
    );
    let body = extruded(SketchPlane::xy(), vec![loop_iv], iv(1.0), tol());
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let on_axis = |p: &Point3<Interval>| p.x.hi().abs() < 1e-9 && p.y.hi().abs() < 1e-9;
    let knives: Vec<EdgeKey> = topo::contact_marks(&body, tol())
        .expect("the valley is tier-3 valid at Interval")
        .iter()
        .filter(|(_, m)| **m == ContactMark::Tangent)
        .map(|(e, _)| e)
        .filter(|&e| {
            let (v0, v1) = body.edge_vertices(e).unwrap();
            on_axis(&point(v0)) && on_axis(&point(v1))
        })
        .collect();
    let [edge] = knives[..] else {
        panic!("one knife on the kiss at Interval, found {knives:?}");
    };
    let out = fillet_edges(
        &sweep::test_support::at_rest(&body, tol()),
        &[edge],
        iv(r),
        tol(),
    )
    .unwrap_or_else(|e| panic!("the valley fills at Interval, got {:?}", e.error));
    let filled = finished("the valley at Interval", out.body, tol());
    assert_eq!(topo::validate_geometric(&filled, tol()), Ok(()), "tier 3");
    let vol = topo::mass_properties(&filled, tol(), topo::test_support::pinned_resolution(tol()))
        .expect("the measured body")
        .met()
        .expect("the measured body")
        .volume;
    let want = valley_area() + valley_fill(r).area;
    assert!(
        vol.lo() <= want && want <= vol.hi(),
        "enclosure [{}, {}] must contain {want}",
        vol.lo(),
        vol.hi()
    );
}

// ---------------------------------------------------------------------
// The crescent between a circle and its tangent line: a cylinder–plane
// cusp extruded, a slit as a hole, a sphere–cone cusp rim revolved.
// ---------------------------------------------------------------------

/// The crescent outside the unit circle and inside its tangent at angle
/// `phi + π/4`, from the circle's point at `phi`: `.cusp()` at the
/// tangency, the line running one unit on to `√2` along `phi`, and
/// back to the start radially. Mirrored in the line at `phi` when
/// `mirror`, which runs the loop the other way round.
fn crescent(phi: f64, mirror: bool) -> ProfileLoop<f64> {
    let turn = if mirror { -1.0 } else { 1.0 };
    let at = |a: f64| Point2::new(a.cos(), a.sin());
    Open.at(at(phi))
        .angle(phi + turn * FRAC_PI_2, tol())
        .unwrap()
        .tangent_arc_to(at(phi + turn * PI / 4.0), tol())
        .unwrap()
        .cusp()
        .line(1.0, tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap()
        .into()
}

/// The crescent at `phi = 0`: outside the unit circle, inside its
/// tangent at `n̂ = (h, h)`, above `y = 0`.
fn crescent_region() -> Margin2 {
    let h = FRAC_1_SQRT_2;
    Box::new(move |x, y| (x.hypot(y) - 1.0).min(1.0 - h * (x + y)).min(y))
}

/// The horn of the crescent at `phi = 0`, in its own frame: tangent
/// point `n̂ = (h, h)`, line direction `t̂ = (h, −h)` towards the
/// crescent; the ball outside the circle and on the circle's side of
/// the line sits at `(1 − r)n̂ + 2√r t̂`, its feet `ball + r n̂` on the
/// line and on the circle along the line of centres. Returns the
/// horn's boundary, the ball's centre and its two feet.
fn crescent_horn_pieces(r: f64) -> ([Piece; 3], [f64; 2], [f64; 2], [f64; 2]) {
    let h = FRAC_1_SQRT_2;
    let (n, t) = ([h, h], [h, -h]);
    let ball = add([n[0] * (1.0 - r), n[1] * (1.0 - r)], t, 2.0 * r.sqrt());
    let on_line = add(ball, n, r);
    let on_circle = foot_on_circle([0.0, 0.0], 1.0, ball);
    (
        [
            Piece::Line {
                from: n,
                to: on_line,
            },
            Piece::Arc {
                c: ball,
                r,
                from: on_line,
                to: on_circle,
            },
            Piece::Arc {
                c: [0.0, 0.0],
                r: 1.0,
                from: on_circle,
                to: n,
            },
        ],
        ball,
        on_line,
        on_circle,
    )
}

fn crescent_horn(r: f64) -> f64 {
    green(&crescent_horn_pieces(r).0).0.abs()
}

/// The horn's region: the crescent, outside the ball, on the knife's
/// side of the chord through the feet.
fn crescent_horn_region(r: f64) -> Margin2 {
    let (_, ball, on_line, on_circle) = crescent_horn_pieces(r);
    let h = FRAC_1_SQRT_2;
    let (crescent, out_ball, side) = (
        crescent_region(),
        inside(ball, r),
        tip_side(on_line, on_circle, [h, h]),
    );
    Box::new(move |x, y| crescent(x, y).min(-out_ball(x, y)).min(side(x, y)))
}

/// The ring the horn sweeps about the y axis (Pappus: `2π·x̄·A`, and
/// `x̄·A` is the first moment).
fn crescent_ring(r: f64) -> f64 {
    2.0 * PI * green(&crescent_horn_pieces(r).0).1.abs()
}

fn on_the_kiss(phi: f64, mirror: bool) -> impl Fn(&Point3<f64>) -> bool {
    let a = phi + if mirror { -PI / 4.0 } else { PI / 4.0 };
    move |p| (p.x - a.cos()).abs() < 1e-9 && (p.y - a.sin()).abs() < 1e-9
}

/// **The crescent prism loses its horn, whichever way it is placed.**
/// The rotations and the mirror guard the side selector: the ball's
/// crossing is picked by the side both faces leave the knife by, not
/// by a quantity that is zero there. At `phi = 0` the lattice places
/// every point against the crescent less its horn.
#[test]
fn a_cylinder_plane_cusp_loses_its_horn_in_every_pose() {
    for (phi, mirror) in [
        (0.0, false),
        (0.0, true),
        (0.3, false),
        (1.9, true),
        (4.0, false),
    ] {
        let what = format!("the crescent prism at {phi} (mirrored: {mirror})");
        let body = prism(vec![crescent(phi, mirror)]);
        let edge = knife_at(&what, &body, on_the_kiss(phi, mirror));
        let operand = finished(&what, body, tol());
        for r in [0.02, 0.1] {
            let dv = filleted_dv(&what, &operand, edge, r);
            assert_close(&format!("{what} at r = {r}"), dv, -crescent_horn(r));
        }
    }
    let r = 0.1;
    let body = prism(vec![crescent(0.0, false)]);
    let edge = knife_at("the crescent prism", &body, on_the_kiss(0.0, false));
    let operand = finished("the crescent prism", body, tol());
    let before = volume("the crescent prism", &operand);
    let built = filleted("the crescent prism", &operand, edge, r);
    let (crescent, horn) = (crescent_region(), crescent_horn_region(r));
    membership::holds(
        "the crescent prism",
        &built,
        before - crescent_horn(r),
        &over_depth(Box::new(move |x, y| crescent(x, y).min(-horn(x, y)))),
        (Point3::new(0.6, -0.1, -0.1), Point3::new(1.5, 0.8, 1.1)),
        10,
    );
}

#[test]
fn a_crescent_hole_is_a_slit_that_gains_its_horn() {
    let body = prism(vec![rect(-1.0, -1.0, 3.0, 2.0), crescent(0.0, false)]);
    let edge = knife_at("the crescent hole", &body, on_the_kiss(0.0, false));
    let operand = finished("the crescent hole", body, tol());
    for r in [0.02, 0.1] {
        let dv = filleted_dv("the crescent hole", &operand, edge, r);
        assert_close(
            &format!("the crescent hole at r = {r}"),
            dv,
            crescent_horn(r),
        );
    }
}

/// **A foot past the end of its support refuses**, and the arc's foot
/// leaves first. The ball's foot on the circle stands at angle
/// `π/4 − atan(2√r/(1 − r))`, below the arc's start at `0` once
/// `2√r > 1 − r`, that is from `r = (√2 − 1)² ≈ 0.1716`; its foot on the
/// line stands `2√r` from the knife, past the line's one unit only from
/// `r = 1/4`. So the crescent builds at `0.17`, and from `0.172` the
/// face-clearance screen refuses on the ARC's face (a cylinder), at
/// `0.2` too. At `0.3` both feet are past their faces, and the screen
/// reports the line's, the worse: one unit less the foot's `2√r`. The
/// ball exists throughout, so none of these is the knife's refusal.
#[test]
fn a_crescent_whose_arc_is_shorter_than_its_foot_refuses() {
    let body = prism(vec![crescent(0.0, false)]);
    let edge = knife_at("the crescent prism", &body, on_the_kiss(0.0, false));
    let operand = finished("the crescent prism", body.clone(), tol());
    let dv = filleted_dv("the crescent prism", &operand, edge, 0.17);
    assert_close("the crescent prism at r = 0.17", dv, -crescent_horn(0.17));
    for (r, arc) in [(0.172, true), (0.2, true), (0.3, false)] {
        let refusal =
            fillet_edges(&operand, &[edge], r, tol()).expect_err("the arc's foot leaves the arc");
        let BlendError::FaceClearanceUncertified { face, margin, .. } = refusal.error else {
            panic!(
                "r = {r}: expected the clearance screen, got {:?}",
                refusal.error
            );
        };
        assert_eq!(margin.predicate, "fillet3_face_clearance");
        assert_eq!(margin.sign, geom_core::Sign::Negative, "r = {r}");
        let surface = body.face_surface(face).unwrap();
        if arc {
            assert!(
                matches!(surface, geom::Surface::Cylinder { .. }),
                "r = {r}: the arc's face refuses first, got {surface:?}"
            );
        } else {
            assert!(
                matches!(surface, geom::Surface::Plane { .. }),
                "r = {r}: the line's face is the worse, got {surface:?}"
            );
            let got = margin
                .reading
                .diagnostic_f64_for_error_text()
                .value()
                .expect("a value reading");
            assert!((got - (1.0 - 2.0 * r.sqrt())).abs() < 1e-9, "margin {got}");
        }
    }
}

#[test]
fn a_sphere_cone_cusp_rim_loses_its_ring() {
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let profile = profile::Profile::new(SketchPlane::xy(), vec![crescent(0.0, false)])
        .validate(tol())
        .expect("the crescent validates");
    let ring = revolve(&profile, axis, Revolution::Full, tol())
        .unwrap()
        .body;
    let h = FRAC_1_SQRT_2;
    let rim = knife_at("the crescent ring", &ring, |p| {
        (p.y - h).abs() < 1e-9 && (p.x.hypot(p.z) - h).abs() < 1e-9
    });
    let operand = finished("the crescent ring", ring, tol());
    for r in [0.02, 0.1] {
        let dv = filleted_dv("the crescent ring", &operand, rim, r);
        assert_close(
            &format!("the crescent ring at r = {r}"),
            dv,
            -crescent_ring(r),
        );
    }
}

// ---------------------------------------------------------------------
// The chamfer, and the touch between two solids.
// ---------------------------------------------------------------------

/// A predicate picking a knife's ends out of a body's points.
type OnKnife = Box<dyn Fn(&Point3<f64>) -> bool>;

/// **The chamfer meets its one arm's refusal at every knife**: its arm
/// is plane–plane, a knife's supports are curved, and the kiss refuses
/// `ChamferArmUnsupported` as any curved pair does.
#[test]
fn the_chamfer_refuses_every_knife_at_its_one_arm() {
    let [(_, cusp, _), (_, slit, _)] = lune_rows();
    let rows: [(&str, Body<f64>, OnKnife); 4] = [
        ("the lune cusp", cusp, Box::new(on_the_z_axis)),
        ("the lune slit", slit, Box::new(on_the_z_axis)),
        ("the valley", prism(vec![valley()]), Box::new(on_the_z_axis)),
        (
            "the crescent prism",
            prism(vec![crescent(0.0, false)]),
            Box::new(on_the_kiss(0.0, false)),
        ),
    ];
    for (what, body, at) in rows {
        let edge = knife_at(what, &body, at);
        let operand = finished(what, body, tol());
        let refusal = chamfer_edges(&operand, &[edge], 0.02, tol())
            .err()
            .unwrap_or_else(|| panic!("{what}: chamfered a knife"));
        assert!(
            matches!(refusal.error, BlendError::ChamferArmUnsupported { edge: at, .. } if at == edge),
            "{what}: {:?}",
            refusal.error
        );
        let text = refusal.render(geom_brep::recourse::Reading::Build);
        assert!(
            text.contains(sweep::blend::CHAMFER_ARM_RECOURSE),
            "{what}: {text}"
        );
    }
}

/// **Two tangent tubes unioned are two solids whose kiss edges are
/// seams**: each keeps its own edge on the kiss, read
/// `SmoothUnderdetermined`, and both verbs leave each as it is
/// (`work/band/a-fillet-leaves-the-kiss-of-two-touching-solids-as-it-is-with-no-refusal.md`).
#[test]
fn two_tangent_tubes_unioned_are_two_solids_whose_kiss_edges_are_left_as_they_are() {
    let slab = |loops| finished("a slab", prism(loops), tol());
    let a = slab(vec![circle(0.0, 0.0, 1.0)]);
    let b = slab(vec![circle(2.0, 0.0, 1.0)]);
    let Ok(BooleanResult::Body(u)) = topo::union(&a, &b, tol()) else {
        panic!("the kiss unions");
    };
    let body = &u.body;
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let on_kiss = |p: &Point3<f64>| (p.x - 1.0).abs() < 1e-9 && p.y.abs() < 1e-9;
    let marks = topo::contact_marks(body, tol()).expect("the union is tier-3 valid");
    let kiss: Vec<EdgeKey> = body
        .edges()
        .map(|(e, _)| e)
        .filter(|&e| {
            let (v0, v1) = body.edge_vertices(e).unwrap();
            on_kiss(&point(v0)) && on_kiss(&point(v1))
        })
        .collect();
    assert_eq!(body.solids().count(), 2, "two solids");
    assert_eq!(kiss.len(), 2, "one kiss edge per solid");
    for e in kiss {
        assert_eq!(marks.get(e), Some(&ContactMark::SmoothUnderdetermined));
        for (verb, got) in [
            ("fillet", fillet_edges(body, &[e], 0.1, tol())),
            ("chamfer", chamfer_edges(body, &[e], 0.1, tol())),
        ] {
            let out = got.unwrap_or_else(|r| panic!("{verb}: refused {:?}", r.error));
            assert!(
                out.blend_faces.is_empty() && out.band_faces.is_empty(),
                "{verb}: the kiss is left as it is"
            );
        }
    }
}
