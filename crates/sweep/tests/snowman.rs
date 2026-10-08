//! **The snowman**: two balls of revolution on distinct centres along
//! one axis, meeting in a real circle, under every boolean.
//!
//! Each ball is the canonical full revolve of a semicircle: two
//! half-bands on one sphere key, their seam meridians crossing the
//! partner sphere. The section is the radical-plane circle, and the
//! rows reach it through the crossing layer's circle × sphere roots and
//! the sphere pair's join. Every body is held to all three validation
//! tiers and to its volume against the two spherical caps the radical
//! plane cuts, computed here from the radii alone.
//!
//! **The seams need not be coplanar.** Both balls are revolved from the
//! same seam, so each seam meridian pierces the other sphere on the
//! other's seam meridian and every chord runs seam to seam. Spin one
//! ball about the shared axis and the pierce lands inside a half-band
//! instead, a pierce ring, which joins: that pose builds too.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use std::collections::BTreeMap;

use super::common::certificates::assert_certificates_fresh;

use geom_core::{Band, Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, Body, BooleanOp, EdgeKey, ShellKey};

use crate::common::oracles::{ball_volume, cap_volume, lens_volume};

/// A ball of radius `r` centred on the y axis at height `y`.
fn ball(r: f64, y: f64) -> AtRestBody<f64> {
    let ball = revolved_about_y(
        vec![
            (Point2::new(0.0, y - r), 1.0),
            (Point2::new(0.0, y + r), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    finished("the ball", ball, Tol::witness())
}

/// Every tier of validation, a closed tessellation, then the volume
/// against `expected` through the kernel's mass properties — exact on
/// closed-form faces, so the slack is rounding's and a wrong body (a
/// lens counted twice, a cap dropped) misses by orders of magnitude.
fn assert_body(label: &str, body: &Body<f64>, expected: f64) {
    let m = mesh::tessellate(body, 1e-3, Tol::witness())
        .unwrap_or_else(|e| panic!("{label}: tessellates, got {e:?}"));
    assert_eq!(
        mesh::validate::check_mesh(&m),
        Ok(()),
        "{label}: a closed manifold mesh"
    );
    assert_solid(label, body, expected);
}

/// [`assert_body`] short of the mesh, for a body carrying a sphere face
/// a tilted circle bounds, which has no tessellation lane
/// (`work/tess/sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane.md`).
fn assert_solid(label: &str, body: &Body<f64>, expected: f64) {
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
        // Relative to the result, floored at the operands' unit scale:
        // the divergence sum rounds against the whole spheres' terms.
        (p.volume - expected).abs() <= 1e-9 * expected.max(1.0),
        "{label}: volume {} against the cap closed form {expected}",
        p.volume
    );
}

/// The boolean's body; an empty result or a refusal fails with the
/// payload.
fn run(op: BooleanOp, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> AtRestBody<f64> {
    let out = match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
    .unwrap_or_else(|e| panic!("{op:?} refused: {e:?}"));
    out.body()
        .unwrap_or_else(|| panic!("{op:?} came back empty"))
        .body
        .clone()
}

const R1: f64 = 1.0;
const R2: f64 = 0.8;
const D: f64 = 1.4;

/// The snowman under ∪, ∩ and ∖ in both orders, each against its
/// closed form.
#[test]
fn the_snowman_builds_under_every_boolean() {
    let (a, b) = (ball(R1, 0.0), ball(R2, D));
    let lens = lens_volume(R1, R2, D);
    let (va, vb) = (ball_volume(R1), ball_volume(R2));
    for (label, op, x, y, expected) in [
        ("A ∪ B", BooleanOp::Union, &a, &b, va + vb - lens),
        ("B ∪ A", BooleanOp::Union, &b, &a, va + vb - lens),
        ("A ∩ B", BooleanOp::Intersect, &a, &b, lens),
        ("B ∩ A", BooleanOp::Intersect, &b, &a, lens),
        ("A ∖ B", BooleanOp::Subtract, &a, &b, va - lens),
        ("B ∖ A", BooleanOp::Subtract, &b, &a, vb - lens),
    ] {
        assert_body(label, &run(op, x, y), expected);
    }
}

/// The boolean's refusal; a body fails with the op named.
fn refusal(op: BooleanOp, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> topo::BooleanError {
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
    .err()
    .unwrap_or_else(|| panic!("{op:?} built a body where the frontier was pinned"))
}

const OPS: [BooleanOp; 3] = [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];

/// **The near-tangent pair**: the same two balls pulled apart until
/// their centres are `r1 + r2 − 1e-4` apart, so the section circle is
/// about two centimetres across and the lens is a sliver. Every op still
/// builds and still meets its closed form.
#[test]
fn a_near_tangent_snowman_builds() {
    let d = R1 + R2 - 1e-4;
    let (a, b) = (ball(R1, 0.0), ball(R2, d));
    let lens = lens_volume(R1, R2, d);
    let (va, vb) = (ball_volume(R1), ball_volume(R2));
    assert_body(
        "near-tangent A ∪ B",
        &run(BooleanOp::Union, &a, &b),
        va + vb - lens,
    );
    assert_body(
        "near-tangent A ∖ B",
        &run(BooleanOp::Subtract, &a, &b),
        va - lens,
    );
    assert_body(
        "near-tangent B ∖ A",
        &run(BooleanOp::Subtract, &b, &a),
        vb - lens,
    );
    assert_body(
        "near-tangent A ∩ B",
        &run(BooleanOp::Intersect, &a, &b),
        lens,
    );
}

/// **Where the near-tangent family stops, at the default band.** The
/// gap `δ = r1 + r2 − d` is the depth of each pole inside the other
/// ball: at `δ = 1e-6` every op still builds to its closed form; at
/// `δ = 1e-8` the pole's side of the other sphere is a residual of
/// `−δ`, inside the band's escalation gap, and the op escalates on that
/// decision by name. Both are band-relative facts, so the other ε rows
/// stand down rather than re-read them at a different scale.
#[test]
fn the_near_tangent_family_builds_to_1e_6_and_escalates_at_1e_8() {
    if Tol::witness().get().eps != 1e-9 {
        test_utils::vacuity::stood_down(
            "non-default eps",
            "the near-tangent edges are measured at the default band only",
        );
        return;
    }
    let a = ball(R1, 0.0);
    let d = R1 + R2 - 1e-6;
    assert_body(
        "δ = 1e-6, A ∪ B",
        &run(BooleanOp::Union, &a, &ball(R2, d)),
        ball_volume(R1) + ball_volume(R2) - lens_volume(R1, R2, d),
    );
    let b = ball(R2, R1 + R2 - 1e-8);
    for op in OPS {
        let e = refusal(op, &a, &b);
        assert!(
            matches!(
                &e,
                topo::BooleanError::Escalated { diag, .. }
                    if diag.predicate == Some("bool_vertex_face_side")
            ),
            "δ = 1e-8 under {op:?}: expected the vertex-side escalation, got {e:?}"
        );
    }
}

/// **The near-tangent family at the finer bands.** At `δ = 1e-5` and
/// `1e-6` the section circle is a few millimetres across and the
/// meridians cross the partner sphere at a slope of `√(4.5δ)`, so the
/// pierce points are placed only as well as the extremes of the circle
/// × sphere residual are evaluated. Each δ builds under every op to its
/// cap closed form wherever it is a definite depth with a decade to
/// spare (`δ ≥ 10·Kε`): at ε = 1e-9 and 1e-12, not at 1e-6, where the
/// pole's side of the other sphere is inside the band.
#[test]
fn the_near_tangent_family_builds_to_1e_6_at_the_finer_bands() {
    let escalate = Band::linear(Tol::witness()).unwrap().escalate();
    let deltas: Vec<f64> = [1e-5, 1e-6]
        .into_iter()
        .filter(|&delta| delta >= 10.0 * escalate)
        .collect();
    if deltas.is_empty() {
        test_utils::vacuity::stood_down(
            "coarse eps",
            "at this band the near-tangent depths are inside the escalation gap",
        );
        return;
    }
    let a = ball(R1, 0.0);
    let (va, vb) = (ball_volume(R1), ball_volume(R2));
    for delta in deltas {
        let d = R1 + R2 - delta;
        let b = ball(R2, d);
        let lens = lens_volume(R1, R2, d);
        for (label, op, x, y, expected) in [
            ("A ∪ B", BooleanOp::Union, &a, &b, va + vb - lens),
            ("A ∩ B", BooleanOp::Intersect, &a, &b, lens),
            ("A ∖ B", BooleanOp::Subtract, &a, &b, va - lens),
            ("B ∖ A", BooleanOp::Subtract, &b, &a, vb - lens),
        ] {
            let label = format!("δ = {delta:e}, {label}");
            let body = run(op, x, y);
            assert_body(&label, &body, expected);
            assert_pierces_on_the_section(&label, &body, d);
        }
    }
}

/// Every vertex off the axis lies within ε of the section circle,
/// read off the radii alone: the radical plane at `y* = r1 − h` with
/// `h = δ(2r2 − δ)/2d`, the circle's radius `√(h(2r1 − h))`, every
/// factor free of cancellation, and `δ = (r1 − d) + r2` exact. The
/// volume cannot see a misplaced pierce on a lens this thin; this can.
fn assert_pierces_on_the_section(label: &str, body: &Body<f64>, d: f64) {
    let eps = Tol::witness().get().eps;
    let delta = (R1 - d) + R2;
    let h = delta * (2.0 * R2 - delta) / (2.0 * d);
    let (y_star, a) = (R1 - h, (h * (2.0 * R1 - h)).sqrt());
    let mut pierces = 0;
    for (_, p) in body.vertex_points() {
        let off_axis = p.x.hypot(p.z);
        if off_axis <= eps {
            continue;
        }
        pierces += 1;
        let off = (p.y - y_star).hypot(off_axis - a);
        assert!(
            off <= eps,
            "{label}: vertex {p:?} is {off:e} off the section circle (y {y_star}, radius {a})"
        );
    }
    assert!(pierces >= 2, "{label}: the section's pierce vertices exist");
}

/// **Where the near-tangent family stops at ε = 1e-12.** At `δ = 1e-7`
/// the slope at the pierce is `≈ 6.7e-4`, and the `f64` evaluation of
/// the residual's near extreme cannot place the pierce point to within
/// `1e-12`: the circle × sphere roots answer uncertain and every op
/// refuses at the pierce door
/// (`work/reach/f64-cannot-place-a-shallow-crossing-within-the-finest-band.md`).
#[test]
fn the_near_tangent_family_stops_at_1e_7_at_eps_1e_12() {
    if Tol::witness().get().eps != 1e-12 {
        test_utils::vacuity::stood_down(
            "eps other than 1e-12",
            "the f64 placement frontier is measured at the finest band only",
        );
        return;
    }
    let a = ball(R1, 0.0);
    let b = ball(R2, R1 + R2 - 1e-7);
    for op in OPS {
        let e = refusal(op, &a, &b);
        assert!(
            matches!(e, topo::BooleanError::CurvedPierceUnsupported { .. }),
            "δ = 1e-7 under {op:?}: expected the pierce door, got {e:?}"
        );
    }
}

/// **The tangent pairs are the honest frontier.** Touching externally
/// (`d = r1 + r2`) or internally (`d = r1 − r2`), the two balls meet at
/// one pole, where a meridian of each touches the other sphere without
/// crossing it. The circle × sphere roots read that as a tangency, which
/// is not a crossing at any order the crossing layer sees, so every op
/// refuses typed at the pierce door — naming a MERIDIAN (a circle edge)
/// of one ball against a SPHERE face of the other — rather than
/// guessing a contact.
#[test]
fn a_pole_tangent_pair_refuses_at_the_pierce_door() {
    let a = ball(R1, 0.0);
    for (label, b) in [
        ("external", ball(R2, R1 + R2)),
        ("internal", ball(0.3, R1 - 0.3)),
    ] {
        for op in OPS {
            let e = refusal(op, &a, &b);
            let topo::BooleanError::CurvedPierceUnsupported {
                operand,
                edge,
                face,
                ..
            } = e
            else {
                panic!("{label} tangency under {op:?}: expected the pierce door, got {e:?}");
            };
            let (edge_body, face_body) = match operand {
                topo::Operand::A => (&a, &b),
                topo::Operand::B => (&b, &a),
            };
            assert_eq!(
                topo::query::edge_carrier_kind(edge_body, edge),
                Some(topo::CurveKind::Circle),
                "{label} under {op:?}: the edge is a meridian"
            );
            assert_eq!(
                topo::query::face_surface_kind(face_body, face),
                Some(geom::SurfaceKind::Sphere),
                "{label} under {op:?}: the face is the other ball's sphere"
            );
        }
    }
}

/// **The nested pair**: a small ball strictly inside the big one, off
/// centre, so no edge crosses either sphere. Every op builds — the
/// subtract as a ball with a spherical cavity, the reversed subtract
/// as nothing.
#[test]
fn a_nested_pair_builds_under_every_boolean() {
    let (r, y) = (0.3, 0.2);
    let (a, b) = (ball(R1, 0.0), ball(r, y));
    let (va, vb) = (ball_volume(R1), ball_volume(r));
    for (label, op, x, z, expected) in [
        ("nested A ∪ B", BooleanOp::Union, &a, &b, va),
        ("nested B ∪ A", BooleanOp::Union, &b, &a, va),
        ("nested A ∩ B", BooleanOp::Intersect, &a, &b, vb),
        ("nested B ∩ A", BooleanOp::Intersect, &b, &a, vb),
        ("nested A ∖ B", BooleanOp::Subtract, &a, &b, va - vb),
    ] {
        assert_body(label, &run(op, x, z), expected);
    }
    let out = topo::boolean::subtract(&b, &a, Tol::witness())
        .unwrap_or_else(|e| panic!("nested B ∖ A refused: {e:?}"));
    assert!(out.body().is_none(), "nested B ∖ A is empty");
}

/// **The snowman at the `Interval` scalar**: the same two balls,
/// enclosures throughout, under every op — the circle × sphere roots'
/// `atan2`/`acos` and their rounding meter run on enclosures here — and
/// every body certifies with a volume bracket around the closed form.
#[test]
fn the_snowman_builds_at_the_interval_scalar() {
    use crate::common::interval::iv;
    use geom_core::{Bounds, Interval};
    let ball_iv = |r: f64, y: f64| -> AtRestBody<Interval> {
        let ball = sweep::test_support::revolved_about_y_at::<Interval>(
            vec![
                (Point2::new(iv(0.0), iv(y - r)), iv(1.0)),
                (Point2::new(iv(0.0), iv(y + r)), iv(0.0)),
            ],
            Revolution::Full,
            Tol::witness(),
        );
        finished("the ball", ball, Tol::witness())
    };
    let (a, b) = (ball_iv(R1, 0.0), ball_iv(R2, D));
    let lens = lens_volume(R1, R2, D);
    let (va, vb) = (ball_volume(R1), ball_volume(R2));
    for (op, expected) in [
        (BooleanOp::Union, va + vb - lens),
        (BooleanOp::Intersect, lens),
        (BooleanOp::Subtract, va - lens),
    ] {
        let out = match op {
            BooleanOp::Union => topo::boolean::union(&a, &b, Tol::witness()),
            BooleanOp::Intersect => topo::boolean::intersect(&a, &b, Tol::witness()),
            BooleanOp::Subtract => topo::boolean::subtract(&a, &b, Tol::witness()),
        }
        .unwrap_or_else(|e| panic!("Interval {op:?} refused: {e:?}"));
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
            "Interval {op:?}: volume [{}, {}] against the cap closed form {expected}",
            v.lo(),
            v.hi()
        );
    }
}

// ------------------------------------------------------------------
// An exact oracle for axisymmetric stacks.
// ------------------------------------------------------------------

/// A solid of revolution about `y` whose every slice is a disc: its
/// squared slice radius as a function of height, and the heights where
/// that function stops being one quadratic.
struct Axi {
    r2: Box<dyn Fn(f64) -> f64>,
    breaks: Vec<f64>,
}

/// The ball of radius `r` centred at height `c`, clipped to `y ≤ top`.
fn axi_ball(r: f64, c: f64, top: f64) -> Axi {
    Axi {
        r2: Box::new(move |y| {
            if y > top {
                0.0
            } else {
                (r * r - (y - c).powi(2)).max(0.0)
            }
        }),
        breaks: [c - r, c + r, top]
            .into_iter()
            .filter(|y| y.is_finite())
            .collect(),
    }
}

/// Every height where two spheres' slice radii cross, so a min or max
/// of them is one quadratic between consecutive breaks.
fn radical_heights(spheres: &[(f64, f64)]) -> Vec<f64> {
    let mut out = Vec::new();
    for (i, &(r1, c1)) in spheres.iter().enumerate() {
        for &(r2, c2) in &spheres[i + 1..] {
            if c1 != c2 {
                out.push((r2 * r2 - r1 * r1 + c1 * c1 - c2 * c2) / (2.0 * (c1 - c2)));
            }
        }
    }
    out
}

/// `π∫ f(y) dy` for an `f` that is one quadratic between consecutive
/// `breaks`: two-point Gauss–Legendre on each piece, which is exact for
/// a quadratic and samples only the piece's INTERIOR — a clip plane
/// makes `f` jump at a break, so an endpoint rule would read the wrong
/// side of it.
fn axi_volume(f: impl Fn(f64) -> f64, mut breaks: Vec<f64>) -> f64 {
    breaks.sort_by(f64::total_cmp);
    breaks.dedup();
    breaks
        .windows(2)
        .map(|w| {
            let (mid, half) = ((w[0] + w[1]) / 2.0, (w[1] - w[0]) / 2.0);
            let node = half / 3.0_f64.sqrt();
            half * (f(mid - node) + f(mid + node))
        })
        .sum::<f64>()
        * PI
}

/// The volume of `x op y` for two axisymmetric solids sharing the
/// sphere list `spheres` (for the radical heights).
fn axi_op(op: BooleanOp, x: &Axi, y: &Axi, spheres: &[(f64, f64)]) -> f64 {
    let mut breaks: Vec<f64> = x.breaks.iter().chain(&y.breaks).copied().collect();
    breaks.extend(radical_heights(spheres));
    match op {
        BooleanOp::Union => axi_volume(|t| (x.r2)(t).max((y.r2)(t)), breaks),
        BooleanOp::Intersect => axi_volume(|t| (x.r2)(t).min((y.r2)(t)), breaks),
        BooleanOp::Subtract => axi_volume(|t| ((x.r2)(t) - (y.r2)(t)).max(0.0), breaks),
    }
}

/// **A sphere pair meeting a partner face from two of this body's
/// sphere faces.** The lens `ball(1, 0) ∩ ball(0.8, 1.4)` has two sphere
/// SURFACES, and a ball of radius 0.5 at `y = 0.75` crosses both, so one
/// partner face of the ball meets two germ faces of the lens on two
/// different spheres — two different radical planes against one partner.
/// Each body's aux copy of a radical plane is keyed by both spheres it
/// depends on, so neither chord can cite the other's plane.
#[test]
fn a_lens_against_a_ball_crossing_both_its_caps_builds() {
    let lens = run(BooleanOp::Intersect, &ball(R1, 0.0), &ball(R2, D));
    let c = ball(0.5, 0.75);
    let spheres = [(R1, 0.0), (R2, D), (0.5, 0.75)];
    let lens_axi = Axi {
        r2: Box::new(|y| {
            (R1 * R1 - y * y)
                .max(0.0)
                .min((R2 * R2 - (y - D).powi(2)).max(0.0))
        }),
        breaks: vec![-R1, R1, D - R2, D + R2],
    };
    let c_axi = axi_ball(0.5, 0.75, f64::INFINITY);
    for op in OPS {
        let want = axi_op(op, &lens_axi, &c_axi, &spheres);
        assert_body(&format!("lens {op:?} ball"), &run(op, &lens, &c), want);
    }
}

/// **A plane × sphere germ and a sphere × sphere germ against one
/// partner face.** A hemisphere (its flat cap at `y = 0`, its dome
/// below) against a ball of radius 0.9 at `y = −0.2`: the ball's sphere
/// crosses the hemisphere's flat cap AND its dome, and at this pose one
/// partner face of the ball is both a plane×sphere germ's partner (its
/// aux is a copy of the ball's sphere) and a sphere pair's (its aux is
/// a radical plane). Keyed by the partner face alone, the two auxes
/// collide and every op refuses `ResidualExceeded { Surface2Residual }`;
/// keyed by the datum each one is, every op builds to the oracle — in
/// both operand orders.
#[test]
fn a_hemisphere_against_a_ball_crossing_its_cap_and_dome_builds() {
    let mut hemi = revolved_about_y(
        vec![
            (Point2::new(0.0, -1.0), (PI / 8.0).tan()),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 0.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    // The full revolve leaves the flat cap as two half-discs on one
    // plane; the boolean takes maximal faces, so the cap is repaired
    // into one first.
    hemi.merge_coplanar_faces(Tol::witness())
        .expect("the hemisphere's cap halves merge");
    let hemi = finished("the hemisphere", hemi, Tol::witness());
    let (r, c) = (0.9, -0.2);
    let b = ball(r, c);
    let spheres = [(1.0, 0.0), (r, c)];
    let (h_axi, b_axi) = (axi_ball(1.0, 0.0, 0.0), axi_ball(r, c, f64::INFINITY));
    for op in OPS {
        let want = axi_op(op, &h_axi, &b_axi, &spheres);
        assert_body(
            &format!("hemisphere {op:?} ball"),
            &run(op, &hemi, &b),
            want,
        );
        let want = axi_op(op, &b_axi, &h_axi, &spheres);
        assert_body(
            &format!("ball {op:?} hemisphere"),
            &run(op, &b, &hemi),
            want,
        );
    }
}

/// **The snowman builds spun, too.** Spin B about the shared axis by any
/// angle off `0` and `π` and A's seam meridian pierces B's sphere at a
/// point INSIDE B's half-band rather than on B's seam: that pierced face
/// carries a pierce ring, and its chords take the arc the pierce germs'
/// directions name. The spin moves no volume, so every op meets the
/// coplanar pose's closed form.
#[test]
fn a_spun_snowman_builds_under_every_boolean() {
    let a = ball(R1, 0.0);
    let lens = lens_volume(R1, R2, D);
    let (va, vb) = (ball_volume(R1), ball_volume(R2));
    for angle in [1e-3, 0.9, core::f64::consts::FRAC_PI_2] {
        let spin = geom_core::Affine3::rotation_about_axis(
            geom_core::Point3::origin(),
            geom_core::Vec3::new(0.0, 1.0, 0.0),
            angle,
        );
        let b = finished(
            "the spun ball",
            topo::transform_rigid(&ball(R2, D), &spin, Tol::witness()).unwrap(),
            Tol::witness(),
        );
        for (label, op, x, y, expected) in [
            ("A ∪ B", BooleanOp::Union, &a, &b, va + vb - lens),
            ("A ∩ B", BooleanOp::Intersect, &a, &b, lens),
            ("A ∖ B", BooleanOp::Subtract, &a, &b, va - lens),
            ("B ∖ A", BooleanOp::Subtract, &b, &a, vb - lens),
        ] {
            assert_body(
                &format!("spun by {angle}: {label}"),
                &run(op, x, y),
                expected,
            );
        }
    }
}

/// **A straight edge through a ball** reaches the line × sphere roots
/// through a public op: a square bar poking out of a ball, its long
/// edges straddling the sphere. They pierce, the pierce points' sector
/// sides certify, and the bar's faces cut the sphere in circles tilted
/// against its polar axis, each chord ending at a corner of the bar
/// where the face it divides turns reflex. Every op builds. The shared
/// volume is `∬ (√(1 − y² − z²) − ½) dy dz` over the bar's square
/// section, integrated here in `z` in closed form and in `y` by
/// Simpson's rule. The sphere faces left are bounded by circles tilted
/// against the ball's chart, which the tessellator does not draw yet
/// (`work/tess/sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane.md`),
/// so the rows stop at the exact volume and the three tiers.
#[test]
fn a_bar_through_a_ball_builds_under_every_boolean() {
    let a = ball(R1, 0.0);
    let half = 0.3;
    let bar = finished(
        "the bar",
        topo::test_support::brick::<f64>((0.5, 2.0), (-half, half), (-half, half), Tol::witness()),
        Tol::witness(),
    );
    // ∫_{−h}^{h} √(c² − z²) dz = h·√(c² − h²) + c²·asin(h/c), c² = 1 − y².
    let slice = |y: f64| {
        let c2 = 1.0 - y * y;
        half * (c2 - half * half).sqrt() + c2 * (half / c2.sqrt()).asin() - half
    };
    let n = 2000;
    let step = 2.0 * half / f64::from(n);
    let simpson: f64 = (0..=n)
        .map(|i| {
            let w = if i == 0 || i == n {
                1.0
            } else if i % 2 == 1 {
                4.0
            } else {
                2.0
            };
            w * slice(-half + f64::from(i) * step)
        })
        .sum();
    let shared = simpson * step / 3.0;
    let (va, vb) = (ball_volume(R1), 1.5 * (2.0 * half).powi(2));
    for (label, op, x, y, expected) in [
        ("ball ∪ bar", BooleanOp::Union, &a, &bar, va + vb - shared),
        ("ball ∩ bar", BooleanOp::Intersect, &a, &bar, shared),
        ("ball ∖ bar", BooleanOp::Subtract, &a, &bar, va - shared),
        ("bar ∖ ball", BooleanOp::Subtract, &bar, &a, vb - shared),
    ] {
        let body = run(op, x, y);
        assert_eq!(topo::validate(&body), Ok(()), "{label}: validate");
        assert_eq!(
            topo::validate_closed(&body),
            Ok(()),
            "{label}: validate_closed"
        );
        assert_eq!(
            topo::validate_geometric(&body, Tol::witness()),
            Ok(()),
            "{label}: validate_geometric"
        );
        let p = topo::mass_properties(&body, Tol::witness())
            .unwrap_or_else(|e| panic!("{label}: mass properties, got {e:?}"));
        assert_eq!(p.volume_pad, 0.0, "{label}: closed-form faces only");
        assert!(
            (p.volume - expected).abs() <= 1e-9 * expected.max(1.0),
            "{label}: volume {} against the slice integral {expected}",
            p.volume
        );
    }
}

/// **The waist fillets through the sphere × sphere blend arm.** The
/// union's waist is a concave crease between two spheres on distinct
/// centres — the configuration `BlendArm::SphereSphereTorus` names —
/// and the ball rolling in it stays outside both, at `Rᵢ + r` from each
/// centre. Its spine is level at `y = ((R1+r)² − (R2+r)² + D²)/2D` and
/// has radius `√((R1+r)² − y²)`, derived here from the fixture.
#[test]
fn the_snowman_waist_fillets() {
    let r = 0.1;
    let body = run(BooleanOp::Union, &ball(R1, 0.0), &ball(R2, D));
    let x = (D.powi(2) + R1.powi(2) - R2.powi(2)) / (2.0 * D);
    let waist = sweep::test_support::rim_arcs_at(&body, (R1.powi(2) - x.powi(2)).sqrt(), x);
    assert!(!waist.is_empty(), "the union has a waist rim");
    let req = sweep::blend::battery::BlendRequest {
        body: &body,
        edges: waist.clone(),
        size: r,
    };
    let verdict = sweep::blend::battery::run_battery(&req, Band::linear(Tol::witness()).unwrap())
        .unwrap_or_else(|e| panic!("the battery passes the waist: {e:?}"));
    let arms: Vec<_> = verdict
        .chains
        .iter()
        .flat_map(|c| c.links().map(|l| l.arm))
        .collect();
    assert!(
        !arms.is_empty()
            && arms
                .iter()
                .all(|a| *a == sweep::blend::BlendArm::SphereSphereTorus),
        "every waist link takes the sphere × sphere arm, got {arms:?}"
    );
    let out = sweep::blend::build::fillet_edges(
        &sweep::test_support::at_rest(&body, Tol::witness()),
        &waist,
        r,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the waist fillets, got {e:?}"));
    assert_eq!(
        topo::validate_geometric(&out.body, Tol::witness()),
        Ok(()),
        "the filleted snowman: validate_geometric"
    );
    let y = ((R1 + r).powi(2) - (R2 + r).powi(2) + D.powi(2)) / (2.0 * D);
    let rho = ((R1 + r).powi(2) - y.powi(2)).sqrt();
    assert!(!out.band_faces.is_empty(), "the fillet mints a band");
    for &f in &out.band_faces {
        match *out
            .body
            .get_surface(out.body.get_face(f).unwrap().surface)
            .unwrap()
        {
            geom::Surface::Torus {
                center,
                major_radius,
                minor_radius,
                ..
            } => {
                assert!(
                    (center.y - y).abs() < 1e-12,
                    "spine height {y}, got {}",
                    center.y
                );
                assert!(
                    (major_radius - rho).abs() < 1e-12,
                    "spine radius {rho}, got {major_radius}"
                );
                assert!(
                    (minor_radius - r).abs() < 1e-15,
                    "tube radius {r}, got {minor_radius}"
                );
            }
            ref other => panic!("the band is a torus, got {other:?}"),
        }
    }
}

/// The boolean's body, `None` for an empty result; a refusal fails
/// with the payload.
fn run_or_empty(
    op: BooleanOp,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) -> Option<AtRestBody<f64>> {
    let out = match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
    .unwrap_or_else(|e| panic!("{op:?} refused: {e:?}"));
    out.body().map(|b| b.body.clone())
}

/// The union (`max`) or intersection (`min`) of the balls `(r, c)` on
/// the y axis, as an [`Axi`].
fn axi_balls(balls: &'static [(f64, f64)], union: bool) -> Axi {
    Axi {
        r2: Box::new(move |y| {
            let slices = balls
                .iter()
                .map(|&(r, c)| (r * r - (y - c).powi(2)).max(0.0));
            if union {
                slices.fold(0.0, f64::max)
            } else {
                slices.fold(f64::INFINITY, f64::min)
            }
        }),
        breaks: balls.iter().flat_map(|&(r, c)| [c - r, c + r]).collect(),
    }
}

/// **A ball strictly inside a body of two or three spheres**, or
/// holding one, with no boundary crossing. Each small ball's sphere
/// crosses the CARRIER of a big sphere face in a circle on the part
/// that face's trim has cut away, so the carriers meet while the faces
/// do not; the extent scan asks the faces, through the section
/// certificate's witness on that circle, and every op builds — in both
/// operand orders, against the slice integral.
#[test]
fn a_ball_inside_a_two_sphere_body_builds() {
    const SNOWMAN: &[(f64, f64)] = &[(R1, 0.0), (R2, D)];
    const CHAIN: &[(f64, f64)] = &[(R1, 0.0), (R2, D), (0.6, 2.3)];
    let snowman = run(BooleanOp::Union, &ball(R1, 0.0), &ball(R2, D));
    let lens = run(BooleanOp::Intersect, &ball(R1, 0.0), &ball(R2, D));
    let chain = run(BooleanOp::Union, &snowman, &ball(0.6, 2.3));
    for (label, body, balls, union, (r, c)) in [
        ("snowman", &snowman, SNOWMAN, true, (0.5, 0.75)),
        ("lens", &lens, SNOWMAN, false, (0.6, 0.8)),
        ("lens in", &lens, SNOWMAN, false, (0.9, 1.0)),
        ("chain", &chain, CHAIN, true, (0.5, 1.9)),
    ] {
        let small = ball(r, c);
        let mut spheres = balls.to_vec();
        spheres.push((r, c));
        let (big_axi, small_axi) = (axi_balls(balls, union), axi_ball(r, c, f64::INFINITY));
        for op in OPS {
            for (name, x, y, x_axi, y_axi) in [
                (
                    format!("{label} {op:?} ball({r}, {c})"),
                    body,
                    &small,
                    &big_axi,
                    &small_axi,
                ),
                (
                    format!("ball({r}, {c}) {op:?} {label}"),
                    &small,
                    body,
                    &small_axi,
                    &big_axi,
                ),
            ] {
                let want = axi_op(op, x_axi, y_axi, &spheres);
                match run_or_empty(op, x, y) {
                    Some(out) => assert_body(&name, &out, want),
                    // Nested operands leave nothing only where the
                    // oracle does: the inner one less the outer.
                    None => assert!(want.abs() <= 1e-12, "{name}: empty against {want}"),
                }
            }
        }
    }
}

/// **A lens beside a slab whose plane cuts only the lens's trimmed-away
/// sphere.** The slab's facing plane crosses the CARRIER of one of the
/// lens's sphere faces, in a circle wholly inside the plane face, on
/// the part of that sphere the lens does not keep: the circle is not an
/// escape of the trimmed group, the faces never meet, and every op
/// builds — the two bodies are disjoint, so the oracle is the lens's
/// two caps and the slab's box, in both operand orders.
#[test]
fn a_lens_beside_a_slab_its_trimmed_sphere_crosses_builds() {
    let lens = run(BooleanOp::Intersect, &ball(R1, 0.0), &ball(R2, D));
    let v_lens = lens_volume(R1, R2, D);
    for (label, (y0, y1)) in [("below", (-3.0, -0.5)), ("above", (1.6, 3.0))] {
        let slab: AtRestBody<f64> = finished(
            "the slab",
            sweep::test_support::brick((-2.0, 2.0), (y0, y1), (-2.0, 2.0), Tol::witness()),
            Tol::witness(),
        );
        let v_slab = 16.0 * (y1 - y0);
        for (op, x, y, want) in [
            (BooleanOp::Union, &lens, &slab, v_lens + v_slab),
            (BooleanOp::Union, &slab, &lens, v_lens + v_slab),
            (BooleanOp::Intersect, &lens, &slab, 0.0),
            (BooleanOp::Intersect, &slab, &lens, 0.0),
            (BooleanOp::Subtract, &lens, &slab, v_lens),
            (BooleanOp::Subtract, &slab, &lens, v_slab),
        ] {
            let name = format!("slab {label}: {op:?}");
            match run_or_empty(op, x, y) {
                Some(out) => assert_body(&name, &out, want),
                None => assert!(want == 0.0, "{name}: empty against {want}"),
            }
        }
    }
}

/// A 6 × 1 × 6 slab whose near face lies in the plane at distance `s`
/// from the origin along the y axis turned by `tilt` degrees about
/// `axis`. Wide enough that its side faces clear every sphere.
fn tilted_slab_about(axis: geom_core::Vec3<f64>, tilt: f64, s: f64) -> AtRestBody<f64> {
    use geom_core::{Affine3, Point3, Vec3};
    let rot = Affine3::rotation_about_axis(Point3::origin(), axis, tilt.to_radians());
    let n = rot.transform_vec(Vec3::new(0.0, 1.0, 0.0));
    let slab: Body<f64> =
        sweep::test_support::brick((-3.0, 3.0), (0.0, 1.0), (-3.0, 3.0), Tol::witness());
    let slab =
        topo::transform_rigid(&slab, &(Affine3::translation(n * s) * rot), Tol::witness()).unwrap();
    finished("the tilted slab", slab, Tol::witness())
}

/// [`tilted_slab_about`] the x axis: toward azimuth π/2 of the lens's
/// chart for a positive tilt, 3π/2 for a negative one, so the plane's
/// circle on a lens sphere stays clear of the seam meridians in `z = 0`.
fn tilted_slab(tilt: f64, s: f64) -> AtRestBody<f64> {
    tilted_slab_about(geom_core::Vec3::new(1.0, 0.0, 0.0), tilt, s)
}

/// Every op in both orders of `x` (volume `vx`) and `y` (`vy`), whose
/// common part is `meet`, against the closed forms. Each result carries
/// a tilted circle ([`assert_solid`]).
fn assert_every_op(
    label: &str,
    (x, vx): (&AtRestBody<f64>, f64),
    (y, vy): (&AtRestBody<f64>, f64),
    meet: f64,
) {
    for (op, p, q, want) in [
        (BooleanOp::Union, x, y, vx + vy - meet),
        (BooleanOp::Union, y, x, vx + vy - meet),
        (BooleanOp::Intersect, x, y, meet),
        (BooleanOp::Intersect, y, x, meet),
        (BooleanOp::Subtract, x, y, vx - meet),
        (BooleanOp::Subtract, y, x, vy - meet),
    ] {
        let name = format!("{label}, ε {}: {op:?}", Tol::witness().eps());
        assert_solid(&name, &run(op, p, q), want);
    }
}

/// [`assert_every_op`] for `body` against a 6 × 1 × 6 slab whose plane
/// cuts a cap of `cap` off it and nothing else.
fn assert_cap_cut(label: &str, body: &AtRestBody<f64>, v: f64, slab: &AtRestBody<f64>, cap: f64) {
    assert_every_op(label, (body, v), (slab, 36.0), cap);
}

/// The unit direction at latitude `lat` (from the xz plane toward +y)
/// and azimuth `az` (from +x toward +z), in degrees.
fn dir(lat: f64, az: f64) -> geom_core::Vec3<f64> {
    let (lat, az) = (lat.to_radians(), az.to_radians());
    geom_core::Vec3::new(lat.cos() * az.cos(), lat.sin(), lat.cos() * az.sin())
}

/// A `2w × t × 2w` brick whose near face lies in the plane `n·p = s`,
/// centred on its foot, its material beyond the plane.
fn brick_toward(n: geom_core::Vec3<f64>, s: f64, w: f64, t: f64) -> AtRestBody<f64> {
    use geom_core::{Affine3, Point3, Vec3};
    let tol = Tol::witness();
    let y = Vec3::new(0.0, 1.0, 0.0);
    let n = n / n.norm();
    let axis = y.cross(n);
    let rot = Affine3::rotation_about_axis(
        Point3::origin(),
        axis / axis.norm(),
        axis.norm().atan2(y.dot(n)),
    );
    let brick: Body<f64> = sweep::test_support::brick((-w, w), (0.0, t), (-w, w), tol);
    let brick = topo::transform_rigid(&brick, &(Affine3::translation(n * s) * rot), tol).unwrap();
    finished("the brick", brick, tol)
}

/// **A tilted slab against the lens, away from its seam.** The slab's
/// near plane, at 0.985 from the unit sphere's centre, cuts that
/// sphere's carrier in a circle wholly inside the plane face, and no
/// edge of either body crosses a face.
///
/// - Tilted 60° (toward azimuth π/2 and 3π/2), the circle lies on the
///   part of the unit sphere the lens trims away: the lens's faces are
///   certified apart from the plane face, so it is no escape, and every
///   op builds against the lens's caps and the slab's 36.
/// - Tilted 20°, the circle lies inside the lens's top face, a TRIMMED
///   group, and the plane cuts a cap of height 0.015 off the lens. The
///   face takes a cut along its chart's meridian through the circle, and
///   every op builds against the cap `πh²(3 − h)/3`. Skipping the
///   trimmed group without asking whether its faces meet the plane face
///   would instead build tier-3-valid bodies short or long by that cap.
#[test]
fn a_tilted_slab_against_the_lens_builds_off_the_seam() {
    let lens = run(BooleanOp::Intersect, &ball(R1, 0.0), &ball(R2, D));
    let v_lens = lens_volume(R1, R2, D);
    let v_slab = 36.0;
    for tilt in [60.0, -60.0] {
        let slab = tilted_slab(tilt, 0.985);
        for (op, x, y, want) in [
            (BooleanOp::Union, &lens, &slab, v_lens + v_slab),
            (BooleanOp::Union, &slab, &lens, v_lens + v_slab),
            (BooleanOp::Intersect, &lens, &slab, 0.0),
            (BooleanOp::Intersect, &slab, &lens, 0.0),
            (BooleanOp::Subtract, &lens, &slab, v_lens),
            (BooleanOp::Subtract, &slab, &lens, v_slab),
        ] {
            let name = format!("slab tilted {tilt}°: {op:?}");
            match run_or_empty(op, x, y) {
                Some(out) => assert_body(&name, &out, want),
                None => assert!(want == 0.0, "{name}: empty against {want}"),
            }
        }
    }
    for tilt in [20.0, -20.0] {
        let label = format!("slab tilted {tilt}° about x");
        assert_cap_cut(
            &label,
            &lens,
            v_lens,
            &tilted_slab(tilt, 0.985),
            cap_volume(R1, 0.015),
        );
    }
}

/// **The same cap, with the circle across the lens's seam meridians**:
/// the slab turned 20° about z instead, so the plane's circle crosses
/// the seams in `z = 0` and the crossing layer sees it without a cut.
/// Every op builds against the cap of height 0.015.
#[test]
fn a_slab_tilted_across_the_lens_seams_builds() {
    let lens = run(BooleanOp::Intersect, &ball(R1, 0.0), &ball(R2, D));
    let z = geom_core::Vec3::new(0.0, 0.0, 1.0);
    for tilt in [20.0, -20.0] {
        let label = format!("slab tilted {tilt}° about z");
        let slab = tilted_slab_about(z, tilt, 0.985);
        assert_cap_cut(
            &label,
            &lens,
            lens_volume(R1, R2, D),
            &slab,
            cap_volume(R1, 0.015),
        );
    }
}

/// **A cap off a banded ball**: the unit ball kept between the planes
/// `y = ±0.6`, a trimmed group whose two half-bands run rim to rim, cut
/// by a slab whose plane lies 0.985 from the centre toward latitude 10°
/// at azimuth π/2. The circle lies inside one half-band, and the
/// meridian cut through it runs from the lower rim to the upper, so both
/// of its ends split a rim. Every op builds against the band's
/// `π(2·0.6 − 2·0.6³/3)` and the cap of height 0.015.
#[test]
fn a_slab_cutting_a_cap_off_a_banded_ball_builds() {
    let band = finished(
        "the band's box",
        sweep::test_support::brick((-2.0, 2.0), (-0.6, 0.6), (-2.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let banded = run(BooleanOp::Intersect, &ball(1.0, 0.0), &band);
    let v_banded = PI * (1.2 - 2.0 * 0.6f64.powi(3) / 3.0);
    let slab = tilted_slab(80.0, 0.985);
    assert_cap_cut(
        "banded ball",
        &banded,
        v_banded,
        &slab,
        cap_volume(1.0, 0.015),
    );
}

/// **A cap off a pole-strut carve.** The ball of radius 0.5 less the box
/// `[−1, 0.25] × [−1, 1] × [−1, 0]` (`join1_r1_rows`'s pole-strut pose),
/// cut by a slab 0.4925 from the centre: toward latitude 10° at azimuth
/// π/2, and toward `(0.866, 0.1, −0.5)` inside the face the box's
/// `x = 0.25` circle trims. The second cut's meridian meets that one
/// tilted arc at both its ends. Every op builds against the carve's
/// ball less the `z ≤ 0` half-ball's `x ≤ 0.25` part, and the cap of
/// height 0.0075.
#[test]
fn a_slab_cutting_a_cap_off_a_pole_strut_carve_builds() {
    use geom_core::Vec3;
    let tol = Tol::witness();
    let ball = finished(
        "the ball",
        sweep::test_support::ball_poled_y(0.5, Vec3::new(0.0, 0.0, 0.0), tol),
        tol,
    );
    let cutter = finished(
        "the box",
        sweep::test_support::brick((-1.0, 0.25), (-1.0, 1.0), (-1.0, 0.0), tol),
        tol,
    );
    let carve = run(BooleanOp::Subtract, &ball, &cutter);
    let v_carve = ball_volume(0.5) - (ball_volume(0.5) - cap_volume(0.5, 0.25)) / 2.0;
    let t = 10f64.to_radians();
    for n in [
        Vec3::new(0.0, t.sin(), t.cos()),
        Vec3::new(0.866, 0.1, -0.5),
    ] {
        assert_cap_cut(
            &format!("pole strut, slab toward {n:?}"),
            &carve,
            v_carve,
            &brick_toward(n, 0.4925, 3.0, 1.0),
            cap_volume(0.5, 0.0075),
        );
    }
}

/// **A pole-to-pole cut**: the unit ball's half-disc revolved by 4
/// radians about y, whose one sphere face runs pole to pole between two
/// meridian arcs, cut by a slab 0.985 from the centre toward latitude 0
/// at azimuth −114.6° (the face's middle) and toward latitude −20° at
/// −160.4°. The cut's two ends are the poles, a half-turn apart, whatever
/// sign the rounding gives the cross term between them. Every op builds
/// against the wedge's `2θ/3` and the cap of height 0.015.
#[test]
fn a_pole_to_pole_cut_of_a_ball_wedge_builds() {
    let theta: f64 = 4.0;
    let wedge = finished(
        "the wedge",
        revolved_about_y(
            vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
            Revolution::Partial(theta),
            Tol::witness(),
        ),
        Tol::witness(),
    );
    for (lat, share) in [(0.0, 0.5), (-20.0, 0.7)] {
        let az = -(theta * share).to_degrees();
        assert_cap_cut(
            &format!("wedge, slab toward lat {lat} az {az:.1}"),
            &wedge,
            2.0 * theta / 3.0,
            &brick_toward(dir(lat, az), 0.985, 3.0, 1.0),
            cap_volume(1.0, 0.015),
        );
    }
}

/// **Two cut-ins on one face, in either order.** Each later cut lies on
/// whichever piece of the face an earlier cut left its circle on.
///
/// - The lens against two disjoint bricks whose near planes lie 0.985
///   from the unit sphere's centre toward latitude 68° at azimuths 130°
///   and 50°, both caps inside the lens's top face, united as one
///   operand in both orders.
/// - The banded ball against a cube of half-side 0.985 turned 45° and
///   135° about y: the same point set, whose four side faces cut four
///   caps, two on each half-band, met in different orders.
///
/// Every op builds against the closed forms.
#[test]
fn two_cut_ins_on_one_face_build_in_either_order() {
    let lens = run(BooleanOp::Intersect, &ball(R1, 0.0), &ball(R2, D));
    let cap = cap_volume(1.0, 0.015);
    for (az1, az2) in [(130.0, 50.0), (50.0, 130.0)] {
        let bricks = run(
            BooleanOp::Union,
            &brick_toward(dir(68.0, az1), 0.985, 0.2, 0.3),
            &brick_toward(dir(68.0, az2), 0.985, 0.2, 0.3),
        );
        assert_every_op(
            &format!("lens, bricks at az {az1}/{az2}"),
            (&lens, lens_volume(R1, R2, D)),
            (&bricks, 2.0 * 0.4 * 0.4 * 0.3),
            2.0 * cap,
        );
    }
    let band = finished(
        "the band's box",
        sweep::test_support::brick((-2.0, 2.0), (-0.6, 0.6), (-2.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let banded = run(BooleanOp::Intersect, &ball(1.0, 0.0), &band);
    let v_banded = PI * (1.2 - 2.0 * 0.6f64.powi(3) / 3.0);
    let a: f64 = 0.985;
    for turn in [45.0f64, 135.0] {
        use geom_core::{Affine3, Point3, Vec3};
        let cube: Body<f64> = sweep::test_support::brick((-a, a), (-a, a), (-a, a), Tol::witness());
        let spin = Affine3::rotation_about_axis(
            Point3::origin(),
            Vec3::new(0.0, 1.0, 0.0),
            turn.to_radians(),
        );
        let cube = finished(
            "the cube",
            topo::transform_rigid(&cube, &spin, Tol::witness()).unwrap(),
            Tol::witness(),
        );
        assert_every_op(
            &format!("banded ball, cube turned {turn}°"),
            (&banded, v_banded),
            (&cube, 8.0 * a.powi(3)),
            v_banded - 4.0 * cap,
        );
    }
}

/// **A second circle across an earlier cut.** The banded ball against
/// two disjoint bricks whose near planes lie 0.985 from the centre toward
/// latitudes 20° and −20°: at azimuth 90° for both, the second circle's
/// crossings lie on the first cut's meridian, which the crossing layer
/// meets as a seam, so it takes no cut of its own; at 90° and 92°, on
/// the piece of the half-band the first cut left it. Every op builds
/// against the closed forms.
#[test]
fn a_circle_across_an_earlier_cut_takes_none_of_its_own() {
    let band = finished(
        "the band's box",
        sweep::test_support::brick((-2.0, 2.0), (-0.6, 0.6), (-2.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let banded = run(BooleanOp::Intersect, &ball(1.0, 0.0), &band);
    let v_banded = PI * (1.2 - 2.0 * 0.6f64.powi(3) / 3.0);
    for az in [90.0, 92.0] {
        let bricks = run(
            BooleanOp::Union,
            &brick_toward(dir(20.0, 90.0), 0.985, 0.3, 0.3),
            &brick_toward(dir(-20.0, az), 0.985, 0.25, 0.25),
        );
        assert_every_op(
            &format!("banded ball, bricks at az 90/{az}"),
            (&banded, v_banded),
            (&bricks, 0.6 * 0.6 * 0.3 + 0.5 * 0.5 * 0.25),
            2.0 * cap_volume(1.0, 0.015),
        );
    }
}

/// **A cut whose half-meridian meets the face's boundary more than once
/// below the circle.** The unit ball less the box `x ≥ 0.5`, cut by a
/// slab 0.999 from the centre toward latitude ±60° at azimuth 30°. Along
/// that half-meridian, from the circle toward the far pole, the face's
/// boundary is the box's `x = 0.5` arc at latitude ∓54.7°, that arc again
/// at ±54.7°, then the pole: the cut ends at the nearest. Every op
/// builds against the ball less the cap of height 0.5, and the cap of
/// height 0.001.
#[test]
fn a_cut_ends_at_the_nearest_of_several_boundary_hits() {
    let box_ = finished(
        "the box",
        sweep::test_support::brick((0.5, 2.0), (-2.0, 2.0), (-2.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let carve = run(BooleanOp::Subtract, &ball(1.0, 0.0), &box_);
    for lat in [60.0, -60.0] {
        assert_cap_cut(
            &format!("ball less a box, slab toward lat {lat} az 30"),
            &carve,
            ball_volume(1.0) - cap_volume(1.0, 0.5),
            &brick_toward(dir(lat, 30.0), 0.999, 3.0, 1.0),
            cap_volume(1.0, 0.001),
        );
    }
}

/// **A millimetre lens inside a ball builds at every tolerance.** The
/// lens of [`R1`], [`R2`], [`D`] scaled by `k = 1e-3`, inside `ball(0.9k)`
/// centred at `(0.1, 1, −0.1)·k`: the ball's sphere crosses the carriers
/// of both lens spheres off the lens's faces. At ε 1e-6 the band is a
/// thousandth of the bodies, and the section certificate's witness on
/// one crossing circle lands in the band of a lens face's boundary
/// arc's carrier circle, past the arc: that used to place no point, and
/// the pair refused `FallbackExtentUnsupported` (R-undec). The face's
/// boundary is read as the distance to the arc
/// (`topo`'s `ConicArc::hit`), so the witness is placed off the face, the
/// circles are certified off the faces at every ε, and every op builds
/// against the caps, to 1e-9 of the result (the volumes are of order k³).
#[test]
fn a_millimetre_lens_inside_a_ball_builds_at_every_tolerance() {
    let k = 1e-3;
    let tol = Tol::witness();
    let lens = run(
        BooleanOp::Intersect,
        &ball(R1 * k, 0.0),
        &ball(R2 * k, D * k),
    );
    let shift = geom_core::Affine3::translation(geom_core::Vec3::new(0.1 * k, k, -0.1 * k));
    let b = finished(
        "the shifted ball",
        topo::transform_rigid(&ball(0.9 * k, 0.0), &shift, tol).unwrap(),
        tol,
    );
    let (v_lens, v_ball) = (lens_volume(R1 * k, R2 * k, D * k), ball_volume(0.9 * k));
    for (op, x, y, want) in [
        (BooleanOp::Union, &lens, &b, v_ball),
        (BooleanOp::Union, &b, &lens, v_ball),
        (BooleanOp::Intersect, &lens, &b, v_lens),
        (BooleanOp::Intersect, &b, &lens, v_lens),
        (BooleanOp::Subtract, &lens, &b, 0.0),
        (BooleanOp::Subtract, &b, &lens, v_ball - v_lens),
    ] {
        let label = format!("ε {} {op:?}", tol.eps());
        match run_or_empty(op, x, y) {
            None => assert!(want == 0.0, "{label}: empty against {want}"),
            Some(out) => {
                assert_eq!(topo::validate(&out), Ok(()), "{label}: tier 1");
                assert_eq!(topo::validate_closed(&out), Ok(()), "{label}: tier 2");
                assert_eq!(
                    topo::validate_geometric(&out, tol),
                    Ok(()),
                    "{label}: tier 3"
                );
                let v = topo::mass_properties(&out, tol).unwrap().volume;
                assert!(
                    (v - want).abs() <= 1e-9 * want,
                    "{label}: volume {v} against {want}"
                );
            }
        }
    }
}

/// Each certified edge's certificate (`Debug`, its D9 identity), keyed
/// by its carrier and parameter interval, which a graft copies bit for
/// bit while it rewrites the surface handles; a key two edges share
/// fails, since it could not tell them apart. `shell` keeps the edges
/// of that shell alone.
fn certificates(
    label: &str,
    body: &Body<f64>,
    shell: Option<ShellKey>,
) -> BTreeMap<String, (EdgeKey, String)> {
    let shell_of = |e: &topo::Edge| {
        let lp = body.get_half_edge(e.he_plus).unwrap().parent_loop;
        body.get_face(body.get_loop(lp).unwrap().face)
            .unwrap()
            .shell
    };
    let mut out = BTreeMap::new();
    for (k, e) in body.edges() {
        if shell.is_some_and(|s| shell_of(e) != s) {
            continue;
        }
        let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(e.curve) else {
            continue;
        };
        let key = format!("{:?} {:?}", c.carrier(), c.params());
        let cert = format!("{:?}", c.certificate());
        if let Some((other, _)) = out.insert(key.clone(), (k, cert)) {
            panic!("{label}: edges {other:?} and {k:?} share the key {key}");
        }
    }
    out
}

/// The `result` edges that carry the certified edges of `operand`'s
/// shell number `kept` (its solid's order; every shell when `None`),
/// one per operand edge, each asserted to store that edge's
/// certificate verbatim.
fn carried_edges(
    label: &str,
    operand: &Body<f64>,
    kept: Option<usize>,
    result: &Body<f64>,
) -> Vec<EdgeKey> {
    let shell = kept.map(|i| {
        let (solid, _) = operand.solids().next().unwrap();
        operand.shells_of_solid(solid).unwrap()[i]
    });
    let mine = certificates(label, operand, shell);
    assert!(!mine.is_empty(), "{label}: the operand has certified edges");
    let theirs = certificates(label, result, None);
    mine.iter()
        .map(|(key, (_, cert))| {
            let Some((dk, carried)) = theirs.get(key) else {
                panic!("{label}: no result edge on {key}")
            };
            assert_eq!(
                carried, cert,
                "{label}: the edge on {key} carries the operand's certificate"
            );
            *dk
        })
        .collect()
}

/// **The containment fallback's assembly carries its kept B operand's
/// certificates**, as the void door carries a cavity's, under ∪ and ∩.
/// The record of the graft's bridge says so whatever the certificates
/// are. Each B edge arrives with its operand's certificate verbatim,
/// and on the operands whose own certificates are fresh, a fresh
/// re-certification mints that certificate again.
///
/// The lens is a boolean's own result, and the seam meridian it took
/// from its B ball carries a certificate a fresh run does not
/// reproduce
/// (`work/cleave/a-boolean-result-carries-a-seam-meridian-certificate-a-fresh-run-does-not-reproduce.md`):
/// the assembly carries that one too.
#[test]
fn the_fallback_assembly_carries_the_kept_operands_certificates() {
    let tol = Tol::witness();
    let slab: AtRestBody<f64> = finished(
        "the slab",
        sweep::test_support::brick((-2.0, 2.0), (-3.0, -2.0), (-2.0, 2.0), tol),
        tol,
    );
    let lens = run(BooleanOp::Intersect, &ball(R1, 0.0), &ball(R2, D));
    // ∩ keeps shells of both operands: A's cavity sphere lies in B's
    // material and B's outer sphere in A's.
    let block: AtRestBody<f64> = finished(
        "the block",
        sweep::test_support::brick((-3.0, 3.0), (-3.0, 3.0), (-3.0, 3.0), tol),
        tol,
    );
    let holed = run(BooleanOp::Subtract, &block, &ball(1.0, 0.0));
    let shell = run(BooleanOp::Subtract, &ball(2.0, 0.0), &ball(0.5, 0.0));
    // (label, op, A, B, B's kept shell, B's certificates fresh)
    for (label, op, a, b, kept, fresh) in [
        (
            "slab ∪ ball",
            BooleanOp::Union,
            &slab,
            &ball(R1, 0.0),
            None,
            true,
        ),
        ("slab ∪ lens", BooleanOp::Union, &slab, &lens, None, false),
        // ∩ keeps B's outer sphere and drops its cavity.
        (
            "holed block ∩ shell",
            BooleanOp::Intersect,
            &holed,
            &shell,
            Some(0),
            true,
        ),
    ] {
        let _ = topo::test_support::take_graft_bridges();
        let out = match op {
            BooleanOp::Union => topo::boolean::union(a, b, tol),
            _ => topo::boolean::intersect(a, b, tol),
        }
        .unwrap_or_else(|e| panic!("{label}: refused: {e:?}"));
        assert_eq!(
            topo::test_support::take_graft_bridges(),
            [topo::test_support::GraftBridge::RemapKeys],
            "{label}: the assembly graft carries"
        );
        let out = out.body().unwrap_or_else(|| panic!("{label}: empty"));
        assert!(
            matches!(out.kind, topo::BooleanResultKind::Assembly),
            "{label}: the containment fallback's assembly, got {:?}",
            out.kind
        );
        let carried = carried_edges(label, b, kept, &out.body);
        if fresh {
            assert_eq!(
                assert_certificates_fresh(label, &out.body, carried.iter().copied(), tol),
                carried.len(),
                "{label}: every carried edge compared"
            );
        }
    }
}

/// **A corner at a cap circle's conventional vertex reads the circle's
/// interior** (`docs/DESIGN.md`, maximal edges). Two cut-ins on the
/// lens's top face leave, in the lens ∩ bricks caps, each cap circle a
/// closed edge whose one vertex is conventional. A cube whose corner
/// rests on the circle, its body diagonal pointing away from the cap,
/// touches the cap at that one point. Its corner exactly at the
/// conventional vertex, and turned 40° along the circle from it, in
/// both member orders: every union records the touch as the corner on
/// the circle's interior, `(u, E)`, never `(u, v)`, and tier 3′ answers
/// alike, its curved cross-solid reach the only finding. The census
/// still sweeps the vertex's point: stripped of its record, the corner
/// at the vertex is an undeclared contact, so the record is what backs
/// it.
#[test]
fn a_corner_at_a_caps_conventional_vertex_reads_the_circles_interior() {
    use geom_core::{Affine3, Point3, Vec3};
    let tol = Tol::witness();
    let lens = run(BooleanOp::Intersect, &ball(R1, 0.0), &ball(R2, D));
    let bricks = run(
        BooleanOp::Union,
        &brick_toward(dir(68.0, 130.0), 0.985, 0.2, 0.3),
        &brick_toward(dir(68.0, 50.0), 0.985, 0.2, 0.3),
    );
    let caps = run(BooleanOp::Intersect, &lens, &bricks);
    let (x, circle) = caps
        .vertices()
        .find(|&(v, _)| topo::is_conventional_vertex(&caps, v))
        .map(|(v, d)| (v, caps.get_half_edge(d.emanating.unwrap()).unwrap().edge))
        .expect("a cap circle's conventional vertex");
    let at_vertex = *caps.get_point(caps.get_vertex(x).unwrap().point).unwrap();
    let geom::Curve3::Circle { center, axis, .. } = *caps
        .get_curve_geom(caps.get_edge(circle).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .carrier()
    else {
        panic!("the cap's edge is a circle")
    };
    // The cap lies on the side of its plane away from the unit sphere's
    // centre.
    let n = if axis.dot(center - Point3::origin()) > 0.0 {
        axis
    } else {
        -axis
    };
    let diagonal = Vec3::new(1.0, 1.0, 1.0) / 3.0_f64.sqrt();
    let turn = diagonal.cross(-n);
    let aim = Affine3::rotation_about_axis(
        Point3::origin(),
        turn / turn.norm(),
        turn.norm().atan2(diagonal.dot(-n)),
    );
    let caps = finished("the caps", caps.into_body(), tol);
    let mut seen = Vec::new();
    for along in [0.0_f64, 40.0] {
        let p =
            Affine3::rotation_about_axis(center, n, along.to_radians()).transform_point(at_vertex);
        let cube: Body<f64> = sweep::test_support::brick((0.0, 0.1), (0.0, 0.1), (0.0, 0.1), tol);
        let place = Affine3::translation(p - Point3::origin()) * aim;
        let cube = finished(
            "the cube",
            topo::transform_rigid(&cube, &place, tol).unwrap(),
            tol,
        );
        for (order, r) in [
            topo::union(&caps, &cube, tol),
            topo::union(&cube, &caps, tol),
        ]
        .into_iter()
        .enumerate()
        {
            let label = format!("{along}° along the circle, order {order}");
            let Ok(topo::BooleanResult::Body(bb)) = r else {
                panic!("{label}: builds: {r:?}")
            };
            let verdict: Vec<String> = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
                .err()
                .unwrap_or_default()
                .iter()
                .map(|e| {
                    format!("{e:?}")
                        .split([' ', '{', '('])
                        .next()
                        .unwrap()
                        .to_owned()
                })
                .collect();
            assert!(
                verdict.iter().all(|k| k == "CensusUndecidable"),
                "{label}: tier 3′ finds no record at fault: {verdict:?}"
            );
            let c = &bb.contacts;
            let records = [
                c.vv.len(),
                c.a_on_b.len() + c.b_on_a.len(),
                c.ve.len(),
                c.ee.len(),
            ];
            assert_eq!(records, [0, 0, 1, 0], "{label}: one (u, E) record");
            let edge = bb.body.get_edge(c.ve[0].edge).unwrap();
            assert!(
                matches!(
                    bb.body
                        .get_curve_geom(edge.curve)
                        .unwrap()
                        .certified()
                        .unwrap()
                        .carrier(),
                    geom::Curve3::Circle { .. }
                ),
                "{label}: the record's edge is the cap circle"
            );
            // Stripped of the record, the touch at the vertex is found:
            // the sweeps still read a conventional vertex's point.
            let mut bare = bb.contacts.clone();
            bare.ve.clear();
            let stripped: Vec<String> = topo::validate_pseudomanifold(&bb.body, &bare, tol)
                .err()
                .unwrap_or_default()
                .iter()
                .map(|e| format!("{e:?}"))
                .filter(|e| !e.starts_with("CensusUndecidable"))
                .collect();
            if along == 0.0 {
                assert!(
                    matches!(stripped.as_slice(), [e] if e.starts_with(
                        "UndeclaredContact { contact: VertexVertex"
                    )),
                    "{label}: without its record the corner at the vertex is undeclared: \
                     {stripped:?}"
                );
            } else {
                // The gap, pinned: `work/fuse/a-corner-on-a-circles-interior-is-unseen-at-tier-three-prime`.
                assert!(
                    stripped.is_empty(),
                    "{label}: off the vertex the touch is on a curved edge's interior, \
                     outside the vertex sweeps: {stripped:?}"
                );
            }
            seen.push((label, records, verdict.len()));
        }
    }
    for (label, records, verdict) in &seen[1..] {
        assert_eq!(
            (records, verdict),
            (&seen[0].1, &seen[0].2),
            "{label}: reads as the corner at the vertex in order 0 does"
        );
    }
}

/// **A plane through a cap circle at its conventional vertex cuts it as
/// it cuts it anywhere else** (`docs/DESIGN.md`, maximal edges). The
/// lens ∩ brick cap's circle is one closed edge with a conventional
/// vertex. A plane through the cap's axis halves it; through the vertex
/// and turned 40° about the axis, each op's half is the same body: the
/// same census, no records, valid at tiers 3 and 3′, half the cap's
/// closed form. Before the split carried a sphere general circle's
/// fitted row, the cut through the vertex left the parent half's row
/// spanning the whole circle.
#[test]
fn a_plane_through_a_caps_conventional_vertex_cuts_as_elsewhere() {
    use geom_core::{Affine3, Point3};
    let tol = Tol::witness();
    let lens = run(BooleanOp::Intersect, &ball(R1, 0.0), &ball(R2, D));
    let cap = run(
        BooleanOp::Intersect,
        &lens,
        &brick_toward(dir(68.0, 50.0), 0.985, 0.2, 0.3),
    );
    let (x, circle) = cap
        .vertices()
        .find(|&(v, _)| topo::is_conventional_vertex(&cap, v))
        .map(|(v, d)| (v, cap.get_half_edge(d.emanating.unwrap()).unwrap().edge))
        .expect("the cap circle's conventional vertex");
    let at_vertex = *cap.get_point(cap.get_vertex(x).unwrap().point).unwrap();
    let geom::Curve3::Circle { center, axis, .. } = *cap
        .get_curve_geom(cap.get_edge(circle).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .carrier()
    else {
        panic!("the cap's edge is a circle")
    };
    let half = cap_volume(1.0, 0.015) / 2.0;
    let cap = finished("the cap", cap.into_body(), tol);
    let mut seen = Vec::new();
    for along in [0.0_f64, 40.0] {
        let p = Affine3::rotation_about_axis(center, axis, along.to_radians())
            .transform_point(at_vertex);
        let normal = axis.cross((p - center) / (p - center).norm());
        let brick = brick_toward(normal, normal.dot(center - Point3::origin()), 3.0, 3.0);
        for (op, r) in [
            ("∖", topo::subtract(&cap, &brick, tol)),
            ("∩", topo::intersect(&cap, &brick, tol)),
        ] {
            let label = format!("{along}° along the circle, cap {op} brick");
            let Ok(topo::BooleanResult::Body(bb)) = r else {
                panic!("{label}: builds: {r:?}")
            };
            let b = &bb.body;
            assert_solid(&label, b, half);
            topo::validate_pseudomanifold(b, &bb.contacts, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3′: {e:?}"));
            let c = &bb.contacts;
            let got = (
                (b.faces().count(), b.edges().count(), b.vertices().count()),
                [
                    c.vv.len(),
                    c.a_on_b.len() + c.b_on_a.len(),
                    c.ve.len(),
                    c.ee.len(),
                ],
            );
            assert_eq!(
                got,
                ((3, 3, 2), [0; 4]),
                "{label}: the half cap, no records"
            );
            seen.push(got);
        }
    }
    assert!(seen.windows(2).all(|w| w[0] == w[1]), "one body: {seen:?}");
}

/// **A closed join through the door leaves a conventional vertex**
/// (`Body::join_edges`, `docs/DESIGN.md`, maximal edges). The lens ∩
/// bricks cap's circle is one closed edge whose one vertex is
/// conventional; split at its middle, it is two arcs between two
/// vertices of valence 2. The join door makes it one closed edge
/// again: one join, whose survivor it reports as conventional, and the
/// body at tier 3 with no joinable vertex left.
#[test]
fn a_closed_join_through_the_door_leaves_a_conventional_vertex() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let lens = run(BooleanOp::Intersect, &ball(R1, 0.0), &ball(R2, D));
    let bricks = run(
        BooleanOp::Union,
        &brick_toward(dir(68.0, 130.0), 0.985, 0.2, 0.3),
        &brick_toward(dir(68.0, 50.0), 0.985, 0.2, 0.3),
    );
    let mut caps = run(BooleanOp::Intersect, &lens, &bricks).into_body();
    let circle = caps
        .vertices()
        .find(|&(v, _)| topo::is_conventional_vertex(&caps, v))
        .map(|(_, d)| caps.get_half_edge(d.emanating.unwrap()).unwrap().edge)
        .expect("a cap circle's conventional vertex");
    let (t0, t1) = caps
        .get_curve_geom(caps.get_edge(circle).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params();
    let counts = (caps.vertices().count(), caps.edges().count());
    caps.split_edge(circle, 0.5 * (t0 + t1), tol).unwrap();
    assert_eq!(topo::joinable_vertices(&caps, band).unwrap().len(), 2);
    let joins = caps.join_edges(band, tol).unwrap();
    assert_eq!(joins.len(), 1, "one join closes the circle");
    let survivor = joins[0].conventional.expect("the survivor is conventional");
    assert!(topo::is_conventional_vertex(&caps, survivor));
    assert_eq!((caps.vertices().count(), caps.edges().count()), counts);
    assert!(topo::joinable_vertices(&caps, band).unwrap().is_empty());
    topo::validate_geometric(&caps, tol).unwrap();
}
