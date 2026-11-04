//! **A fillet at a knife edge builds where its ball is defined** (BAND,
//! `work/band/a-knife-edge-blend-is-unbuilt-though-its-rolling-ball-is-defined.md`).
//!
//! A cusp (wedge 0) or slit (wedge 2π) is two supports tangent along
//! the edge with their material facing each other. Its rolling ball
//! rests in the crescent they open into — in the material at a cusp,
//! in the void at a slit — centred where the supports' `r`-offsets
//! cross away from the edge, on the side both faces leave it by. Each
//! row below builds one such body through a public door the user can
//! author it with and checks the fillet's volume change against a
//! closed form: the crescent's horn cut off by the ball, by Green's
//! theorem over the three curves that bound it, times the prism's
//! depth, or carried round the axis by Pappus for a revolved rim. A
//! cusp loses that horn, a slit gains it.
//!
//! The refusals are pinned beside them: a radius past the room the
//! crescent opens to (`KnifeRefusal::NoRoom`), a ball reaching past
//! its supports (the reach and clearance meters), and the chamfer,
//! whose arms are plane–plane (`KnifeRefusal::Chamfer`). The mirrored
//! and rotated crescents guard the side selector: the closed form the
//! ball's centre was picked by before read an exact zero at a knife and
//! chose by the sign of rounding noise.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, PI};

use geom_core::{Point2, Point3, Tol, Vec2};
use profile::test_support::bulge_loop;
use profile::{Open, Profile, ProfileLoop, SketchPlane, Start};
use sweep::blend::{BlendError, KnifeRefusal, chamfer_edges, fillet_edges};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{AtRestBody, Body, BooleanResult, ContactMark, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

fn validated(loops: Vec<ProfileLoop<f64>>) -> profile::ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(tol())
        .expect("the row's profile validates")
}

/// `loops` extruded over `z ∈ [0, 1]`.
fn prism(loops: Vec<ProfileLoop<f64>>) -> Body<f64> {
    extrude(
        &validated(loops),
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .expect("the row extrudes")
    .body
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(x0, y0), 0.0),
        (Point2::new(x1, y0), 0.0),
        (Point2::new(x1, y1), 0.0),
        (Point2::new(x0, y1), 0.0),
    ])
}

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
            let he = body.get_edge(e).unwrap().he_plus;
            at(&point(body.get_half_edge(he).unwrap().start))
                && at(&point(body.half_edge_end(he).unwrap()))
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

/// The fillet of `edge` at `r`, built, and the volume it changed.
fn filleted_dv(what: &str, operand: &AtRestBody<f64>, edge: EdgeKey, r: f64) -> f64 {
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
    let after = finished(what, out.body, tol());
    volume(what, &after) - volume(what, operand)
}

fn assert_close(what: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-9 * want.abs().max(1.0),
        "{what}: ΔV {got}, closed form {want}"
    );
}

// ---------------------------------------------------------------------
// The closed forms: Green's theorem over a region bounded by circular
// arcs and segments, its area and its first moment about the y axis.
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

/// The area of the horn a ball about `ball` of radius `r` cuts off a
/// crescent whose knife is at `rim`, between the circles `a` and `b`
/// (each `(centre, radius)`).
fn two_circle_horn(
    rim: [f64; 2],
    (c_a, r_a): ([f64; 2], f64),
    (c_b, r_b): ([f64; 2], f64),
    ball: [f64; 2],
    r: f64,
) -> f64 {
    let (f_a, f_b) = (
        foot_on_circle(c_a, r_a, ball),
        foot_on_circle(c_b, r_b, ball),
    );
    green(&[
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
    .abs()
}

// ---------------------------------------------------------------------
// The lune: a cusp, and the same lune as a hole, a slit.
// ---------------------------------------------------------------------

/// The lune between `(0,1) r 1` and `(0,2) r 2`
/// ([`sweep::test_support::lune`]): the ball touching the small circle
/// from outside and the big one from inside sits at
/// `(√(8r(1 − r)), 3r)`, and exists while `r < 2 − 1`.
fn lune_horn(r: f64) -> f64 {
    let ball = [(8.0 * r * (1.0 - r)).sqrt(), 3.0 * r];
    two_circle_horn([0.0, 0.0], ([0.0, 1.0], 1.0), ([0.0, 2.0], 2.0), ball, r)
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

#[test]
fn a_lune_cusp_loses_its_horn_and_a_lune_slit_gains_it() {
    for (what, body, sign) in lune_rows() {
        let edge = knife_at(what, &body, on_the_z_axis);
        let operand = finished(what, body, tol());
        for r in [0.05, 0.3, 0.6] {
            let dv = filleted_dv(what, &operand, edge, r);
            assert_close(&format!("{what} at r = {r}"), dv, sign * lune_horn(r));
        }
    }
}

/// **Past `r = 8/9` the ball's own circle crosses the lune's `x = 0`
/// wall**, but only its arc facing the knife is the band, and that arc
/// stays clear of the wall. The cusp builds there. The slit, which adds
/// material, refuses at the ring meter with a BOUND: that conservative
/// screen cannot certify the wall's edge clear of the face the band
/// buries, and refuses rather than guess (`crates/geom-brep/README.md`,
/// C8), so it is not the knife's refusal either.
#[test]
fn past_eight_ninths_the_lune_cusp_builds_and_the_slit_meets_the_ring_screen() {
    let r = 0.95;
    let [(cusp_what, cusp, sign), (slit_what, slit, _)] = lune_rows();
    let edge = knife_at(cusp_what, &cusp, on_the_z_axis);
    let operand = finished(cusp_what, cusp, tol());
    let dv = filleted_dv(cusp_what, &operand, edge, r);
    assert_close(&format!("{cusp_what} at r = {r}"), dv, sign * lune_horn(r));

    let edge = knife_at(slit_what, &slit, on_the_z_axis);
    let operand = finished(slit_what, slit, tol());
    let refusal = fillet_edges(&operand, &[edge], r, tol())
        .err()
        .expect("the ring screen cannot certify the wall");
    assert!(
        matches!(
            refusal.error,
            BlendError::RingClearance { bounded: true, .. }
        ),
        "{slit_what}: {:?}",
        refusal.error
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

// ---------------------------------------------------------------------
// The valley between two touching tubes on a plate: a slit, authored
// as a profile and through the Boolean in either order.
// ---------------------------------------------------------------------

/// Two unit tubes on a plate, the valley between them a slit: the
/// profile's top runs over the right tube's arc into the kiss at the
/// origin and out over the left tube's, the plate filling the lower
/// cusp. Authored raw; the profile door decides the kiss a cusp joint.
fn valley() -> ProfileLoop<f64> {
    let sweep_angle = PI - 0.6_f64.atan2(0.8);
    let b = (sweep_angle / 4.0).tan();
    bulge_loop(vec![
        (Point2::new(-1.8, -1.5), 0.0),
        (Point2::new(1.8, -1.5), 0.0),
        (Point2::new(1.8, 0.6), b),
        (Point2::new(0.0, 0.0), b),
        (Point2::new(-1.8, 0.6), 0.0),
    ])
}

/// The valley the ball fills between the unit tubes about `(±1, 0)`:
/// its centre stands at `(0, √(r² + 2r))`, `1 + r` from both axes.
fn valley_fill(r: f64) -> f64 {
    let ball = [0.0, (r * r + 2.0 * r).sqrt()];
    two_circle_horn([0.0, 0.0], ([1.0, 0.0], 1.0), ([-1.0, 0.0], 1.0), ball, r)
}

#[test]
fn the_valley_between_two_touching_tubes_fills() {
    let profile = validated(vec![valley()]);
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
        assert_close(&format!("the valley at r = {r}"), dv, valley_fill(r));
    }
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
        assert_close(&format!("{what} at r = {r}"), dv, valley_fill(r));
    }
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

/// The horn of the crescent at `phi = 0`, in its own frame: tangent
/// point `n̂ = (h, h)`, line direction `t̂ = (h, −h)` towards the
/// crescent; the ball outside the circle and on the circle's side of
/// the line sits at `(1 − r)n̂ + 2√r t̂`, its feet `ball + r n̂` on the
/// line and on the circle along the line of centres.
fn crescent_horn_pieces(r: f64) -> [Piece; 3] {
    let h = FRAC_1_SQRT_2;
    let (n, t) = ([h, h], [h, -h]);
    let ball = add([n[0] * (1.0 - r), n[1] * (1.0 - r)], t, 2.0 * r.sqrt());
    let on_line = add(ball, n, r);
    let on_circle = foot_on_circle([0.0, 0.0], 1.0, ball);
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
    ]
}

fn crescent_horn(r: f64) -> f64 {
    green(&crescent_horn_pieces(r)).0.abs()
}

/// The ring the horn sweeps about the y axis (Pappus: `2π·x̄·A`, and
/// `x̄·A` is the first moment).
fn crescent_ring(r: f64) -> f64 {
    2.0 * PI * green(&crescent_horn_pieces(r)).1.abs()
}

fn on_the_kiss(phi: f64, mirror: bool) -> impl Fn(&Point3<f64>) -> bool {
    let a = phi + if mirror { -PI / 4.0 } else { PI / 4.0 };
    move |p| (p.x - a.cos()).abs() < 1e-9 && (p.y - a.sin()).abs() < 1e-9
}

/// **The crescent prism loses its horn, whichever way it is placed.**
/// The rotations and the mirror guard the side selector: the ball's
/// crossing is picked by the side both faces leave the knife by, not
/// by a quantity that is zero there.
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

/// **A foot past the end of its support refuses**: the ball's foot on
/// the crescent's line stands `2√r` from the knife, past the line's one
/// unit for `r > 1/4`, so at `r = 0.3` the band would leave its face.
/// The face-clearance screen refuses it, with the line face's one unit
/// less the foot's setback `2√r` as its margin; the ball exists, so it
/// is not the knife's refusal.
#[test]
fn a_crescent_whose_line_is_shorter_than_its_foot_refuses() {
    let body = prism(vec![crescent(0.0, false)]);
    let edge = knife_at("the crescent prism", &body, on_the_kiss(0.0, false));
    let operand = finished("the crescent prism", body, tol());
    let refusal = fillet_edges(&operand, &[edge], 0.3, tol())
        .err()
        .expect("the foot leaves the line");
    let BlendError::FaceClearanceUncertified { margin, .. } = refusal.error else {
        panic!("expected the clearance screen, got {:?}", refusal.error);
    };
    assert_eq!(margin.predicate, "fillet3_face_clearance");
    assert_eq!(margin.sign, geom_core::Sign::Negative);
    let got = margin
        .reading
        .diagnostic_f64_for_error_text()
        .value()
        .expect("a value reading");
    assert!(
        (got - (1.0 - 2.0 * 0.3_f64.sqrt())).abs() < 1e-9,
        "margin {got}"
    );
}

#[test]
fn a_sphere_cone_cusp_rim_loses_its_ring() {
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ring = revolve(
        &validated(vec![crescent(0.0, false)]),
        axis,
        Revolution::Full,
        tol(),
    )
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

/// **The chamfer refuses every knife**: its arms are plane–plane, and a
/// knife's supports are curved, and the refusal says so.
#[test]
fn the_chamfer_refuses_every_knife_with_its_own_recourse() {
    let [(_, cusp, _), (_, slit, _)] = lune_rows();
    let rows: [(&str, Body<f64>, Box<dyn Fn(&Point3<f64>) -> bool>); 4] = [
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
            matches!(
                refusal.error,
                BlendError::KnifeEdge { edge: at, refusal: KnifeRefusal::Chamfer, .. } if at == edge
            ),
            "{what}: {:?}",
            refusal.error
        );
        let text = refusal.render(geom_brep::recourse::Reading::Build);
        assert!(
            text.contains(sweep::blend::CHAMFER_KNIFE_RECOURSE),
            "{what}: {text}"
        );
    }
}

/// **Two tangent tubes unioned are two solids whose kiss edges are
/// seams**: each keeps its own edge on the kiss, read
/// `SmoothUnderdetermined`, and both verbs leave each as it is.
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
            let he = body.get_edge(e).unwrap().he_plus;
            on_kiss(&point(body.get_half_edge(he).unwrap().start))
                && on_kiss(&point(body.half_edge_end(he).unwrap()))
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
