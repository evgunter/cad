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
//! **What builds is the COPLANAR-seam pose.** Both balls are revolved
//! from the same seam, so each seam meridian pierces the other sphere
//! on the other's seam meridian and every chord runs seam to seam. Spin
//! one ball about the shared axis and the pierce lands inside a
//! half-band instead, which is the pierce-ring door — pinned below as
//! the frontier, not as a body.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Band, Point2, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

/// A ball of radius `r` centred on the y axis at height `y`.
fn ball(r: f64, y: f64) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, y - r), 1.0),
            (Point2::new(0.0, y + r), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
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

/// Every tier of validation, a closed tessellation, then the volume
/// against `expected` through the kernel's mass properties — exact on
/// closed-form faces, so the slack is rounding's and a wrong body (a
/// lens counted twice, a cap dropped) misses by orders of magnitude.
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
    let m = mesh::tessellate(body, 1e-3, Tol::witness())
        .unwrap_or_else(|e| panic!("{label}: tessellates, got {e:?}"));
    assert_eq!(
        mesh::validate::check_mesh(&m),
        Ok(()),
        "{label}: a closed manifold mesh"
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
fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
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
fn refusal(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> topo::BooleanError {
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
    let ball_iv = |r: f64, y: f64| -> Body<Interval> {
        sweep::test_support::revolved_about_y_at::<Interval>(
            vec![
                (Point2::new(iv(0.0), iv(y - r)), iv(1.0)),
                (Point2::new(iv(0.0), iv(y + r)), iv(0.0)),
            ],
            Revolution::Full,
            Tol::witness(),
        )
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
/// carries a pierce ring, and its chords take their arc from the face's
/// own azimuth window. The spin moves no volume, so every op meets the
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
        let b = topo::transform_rigid(&ball(R2, D), &spin, Tol::witness()).unwrap();
        for (label, op, x, y, expected) in [
            ("A ∪ B", BooleanOp::Union, &a, &b, va + vb - lens),
            ("A ∩ B", BooleanOp::Intersect, &a, &b, lens),
            ("A ∖ B", BooleanOp::Subtract, &a, &b, va - lens),
            ("B ∖ A", BooleanOp::Subtract, &b, &a, vb - lens),
        ] {
            assert_body(&format!("spun by {angle}: {label}"), &run(op, x, y), expected);
        }
    }
}

/// **A straight edge through a ball** reaches the line × sphere roots
/// through a public op: a square bar poking out of a ball, its long
/// edges straddling the sphere. They pierce, the pierce points' sector
/// sides certify, and the op goes on to the join, where the bar's faces
/// cut the sphere in circles tilted against its polar axis: the
/// arc-side rule's polar gate (`SectionNotPolar`), typed, for every op.
/// A refusal at the pierce door would mean the root lane went dark.
#[test]
fn a_bar_through_a_ball_crosses_the_sphere() {
    let a = ball(R1, 0.0);
    let bar =
        topo::test_support::brick::<f64>((0.5, 2.0), (-0.3, 0.3), (-0.3, 0.3), Tol::witness());
    for op in OPS {
        let e = refusal(op, &a, &bar);
        assert!(
            matches!(
                e,
                topo::BooleanError::Join(topo::SplitJoinError::SectionNotPolar { .. })
            ),
            "bar through a ball under {op:?}: expected to cross the sphere and stop at the \
             polar gate, got {e:?}"
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
    let out = sweep::blend::build::fillet_edges(&body, &waist, r, Tol::witness())
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
fn run_or_empty(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Option<Body<f64>> {
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
        let slab: Body<f64> =
            sweep::test_support::brick((-2.0, 2.0), (y0, y1), (-2.0, 2.0), Tol::witness());
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
/// from the origin along the y axis tilted by `tilt` degrees about x —
/// toward azimuth π/2 of the lens's chart for a positive tilt, 3π/2 for
/// a negative one, so the plane's circle on a lens sphere stays clear of
/// the seam meridians in `z = 0`. Wide enough that its side faces clear
/// every sphere.
fn tilted_slab(tilt: f64, s: f64) -> Body<f64> {
    use geom_core::{Affine3, Point3, Vec3};
    let rot = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(1.0, 0.0, 0.0),
        tilt.to_radians(),
    );
    let n = rot.transform_vec(Vec3::new(0.0, 1.0, 0.0));
    let slab: Body<f64> =
        sweep::test_support::brick((-3.0, 3.0), (0.0, 1.0), (-3.0, 3.0), Tol::witness());
    topo::transform_rigid(&slab, &(Affine3::translation(n * s) * rot), Tol::witness()).unwrap()
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
/// - Tilted 20°, the circle lies inside the lens's top face: the plane
///   cuts a cap of height 0.015 off the lens, a real escape of a
///   TRIMMED group, which the re-chart cannot serve. Every op refuses
///   it typed. Skipping the trimmed group without asking whether its
///   faces meet the plane face would instead build tier-3-valid bodies
///   short or long by that cap.
#[test]
fn a_tilted_slab_against_the_lens_builds_or_refuses_the_trimmed_escape() {
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
        let slab = tilted_slab(tilt, 0.985);
        for (x, y) in [(&lens, &slab), (&slab, &lens)] {
            for op in OPS {
                let e = refusal(op, x, y);
                assert!(
                    matches!(
                        e,
                        topo::BooleanError::FallbackExtentUnsupported { what, .. }
                            if what.contains("TRIMMED sphere face group escapes")
                    ),
                    "slab tilted {tilt}° under {op:?}: expected the trimmed escape, got {e:?}"
                );
            }
        }
    }
}

/// **A millimetre lens inside a ball, at the tolerance that cannot
/// place the section circle.** The lens of [`R1`], [`R2`], [`D`] scaled
/// by `k = 1e-3`, inside `ball(0.9k)` centred at `(0.1, 1, −0.1)·k`: the
/// ball's sphere crosses the carriers of both lens spheres off the
/// lens's faces. At ε 1e-6 the band is a thousandth of the bodies, and
/// the section certificate's witness on one crossing circle lands in
/// it: no point placed, so the pair refuses with the certificate's own
/// reason (R-undec) as `FallbackExtentUnsupported`, never as spheres
/// that meet, and never as apart. At every other ε the circles are
/// certified off the faces and every op builds against the caps,
/// to 1e-9 of the result (the volumes are of order k³).
#[test]
fn a_millimetre_lens_inside_a_ball_refuses_its_unplaced_circle_at_1e_6() {
    let k = 1e-3;
    let tol = Tol::witness();
    let lens = run(
        BooleanOp::Intersect,
        &ball(R1 * k, 0.0),
        &ball(R2 * k, D * k),
    );
    let shift = geom_core::Affine3::translation(geom_core::Vec3::new(0.1 * k, k, -0.1 * k));
    let b = topo::transform_rigid(&ball(0.9 * k, 0.0), &shift, tol).unwrap();
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
        if tol.eps() == 1e-6 {
            let e = refusal(op, x, y);
            assert!(
                matches!(
                    e,
                    topo::BooleanError::FallbackExtentUnsupported { what, .. }
                        if what.contains("no witness could place")
                ),
                "{label}: expected the certificate's undecided refusal, got {e:?}"
            );
            continue;
        }
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
