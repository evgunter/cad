//! LIB-U2 PR-1 differential suite: for a representative family of
//! loops, the PATHS-algebra-lowered [`ProfileLoop`] is IDENTICAL to
//! the hand-built [`LoopBuilder`] equivalent — **bit-level on every
//! coordinate the authoring determines exactly** (authored points,
//! authored bulges, sharp chains, tangent-arc bulges through the same
//! closed-form expressions) — and both build and validate identically.
//!
//! Where a coordinate is NOT determined exactly by the authoring (a
//! fillet's virtual corner and trim points pass through `sin_cos` /
//! division), the hand chain is fed the same corner value the
//! algebra's documented construction produces (re-derived here from
//! the ratified closed forms, independently of the implementation) and
//! the results are then bit-compared end to end, with additional
//! value-level assertions pinning the derived geometry to its exact
//! intended location within 1e-12.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tolerance, Vec2};
use profile::{LoopBuilder, Open, Profile, ProfileLoop, SketchPlane, Start};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// Bit-level loop identity: vertex count, every coordinate and bulge
/// by `to_bits`, and the declared-joint SET (declaration order is not
/// semantic — `tangent_joints` documents set semantics).
fn assert_loops_identical(algebra: &ProfileLoop<f64>, hand: &ProfileLoop<f64>) {
    assert_eq!(algebra.vertices.len(), hand.vertices.len(), "vertex count");
    for (i, (a, h)) in algebra.vertices.iter().zip(&hand.vertices).enumerate() {
        assert_eq!(
            a.pos.x.to_bits(),
            h.pos.x.to_bits(),
            "vertex {i} x: {} vs {}",
            a.pos.x,
            h.pos.x
        );
        assert_eq!(
            a.pos.y.to_bits(),
            h.pos.y.to_bits(),
            "vertex {i} y: {} vs {}",
            a.pos.y,
            h.pos.y
        );
        assert_eq!(
            a.bulge.to_bits(),
            h.bulge.to_bits(),
            "vertex {i} bulge: {} vs {}",
            a.bulge,
            h.bulge
        );
    }
    let mut ta = algebra.tangent_joints.clone();
    let mut th = hand.tangent_joints.clone();
    ta.sort_unstable();
    ta.dedup();
    th.sort_unstable();
    th.dedup();
    assert_eq!(ta, th, "declared-joint sets");
}

/// Both loops validate Ok, and their canonical forms are identical
/// (bit-identical inputs make this near-trivial; asserting it pins
/// that the lowered form really is the v1 form the verifier expects).
fn assert_validate_identically(algebra: &ProfileLoop<f64>, hand: &ProfileLoop<f64>) {
    let va = Profile::new(SketchPlane::xy(), vec![algebra.clone()])
        .validate(Tolerance::get())
        .expect("algebra-lowered loop validates");
    let vh = Profile::new(SketchPlane::xy(), vec![hand.clone()])
        .validate(Tolerance::get())
        .expect("hand-built loop validates");
    assert_eq!(
        format!("{:?}", va.loops()),
        format!("{:?}", vh.loops()),
        "canonical validated forms"
    );
}

/// D1 — sharp polygon: lines only, closed with `line_to(Start)`.
/// Every coordinate is authored; identity is exact everywhere.
#[test]
fn sharp_triangle_matches_loopbuilder() {
    let (a, b, c) = (p2(0.0, 0.0), p2(4.0, 0.5), p2(1.5, 3.0));
    let algebra = Open
        .at(a)
        .line_to(b)
        .unwrap()
        .line_to(c)
        .unwrap()
        .line_to(Start)
        .unwrap();
    let hand = LoopBuilder::start(a).line_to(b).line_to(c).close();
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// D2 — sharp line/arc chain with an arc seam: authored bulges pass
/// through untouched; junction checks classify definitely sharp.
#[test]
fn sharp_arc_chain_matches_loopbuilder() {
    let (a, b, c) = (p2(0.0, 0.0), p2(2.0, 0.0), p2(2.0, 2.0));
    let (b1, b2) = (0.5, 0.2);
    let algebra = Open
        .at(a)
        .line_to(b)
        .unwrap()
        .arc_to(c, b1)
        .unwrap()
        .arc_to(Start, b2)
        .unwrap();
    let hand = LoopBuilder::start(a)
        .line_to(b)
        .arc_to(c, b1)
        .close_with_bulge(b2);
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// D3 — declared tangent leg: `.tangent().tangent_arc_to(p)` against
/// the hand `.declare_tangent().arc_to(p, tan(Δ/2))` with Δ from the
/// same atan2 — the bulge is bit-identical because both sides evaluate
/// the same closed-form expression on the same inputs.
#[test]
fn tangent_arc_leg_matches_loopbuilder() {
    let (a, b, c) = (p2(0.0, 0.0), p2(2.0, 0.0), p2(3.0, 1.0));
    // The unique tangent arc departing east from b to c: tangent-chord
    // angle Δ = atan2(1, 1), bulge tan(Δ/2) (the documented form).
    let delta = 1.0_f64.atan2(1.0);
    let hand_bulge = (delta / 2.0).tan();
    let algebra = Open
        .at(a)
        .line_to(b)
        .unwrap()
        .tangent()
        .tangent_arc_to(c)
        .unwrap()
        .line_to(Start)
        .unwrap();
    let hand = LoopBuilder::start(a)
        .line_to(b)
        .declare_tangent()
        .arc_to(c, hand_bulge)
        .close();
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// The ratified corner construction, re-derived independently of the
/// implementation (the documented closed forms): the incoming ray
/// (origin, angle θ₁) × the arrival carrier (anchor, angle θ₂), via
/// t = (w × u₂)/(u₁ × u₂), corner = origin + t·u₁.
fn expected_corner(origin: Point2<f64>, th1: f64, anchor: Point2<f64>, th2: f64) -> Point2<f64> {
    let (s1, c1) = th1.sin_cos();
    let (s2, c2) = th2.sin_cos();
    let u1 = Vec2::new(c1, s1);
    let u2 = Vec2::new(c2, s2);
    let w = anchor - origin;
    let t = w.perp_dot(u2) / u1.perp_dot(u2);
    origin + u1 * t
}

/// The ratified line×line trim closed form (the docs on
/// `LoopBuilder::fillet`), re-derived independently: (t1, t2, bulge).
fn expected_trims(
    head: Point2<f64>,
    corner: Point2<f64>,
    next: Point2<f64>,
    r: f64,
) -> (Point2<f64>, Point2<f64>, f64) {
    let v1 = corner - head;
    let v2 = next - corner;
    let m = (v1.norm_squared() * v2.norm_squared()).sqrt();
    let half_tan = v1.perp_dot(v2) / (m + v1.dot(v2));
    let bulge = half_tan / (1.0 + (1.0 + half_tan * half_tan).sqrt());
    let setback = r * half_tan.abs();
    let len1 = v1.norm_squared().sqrt();
    let len2 = v2.norm_squared().sqrt();
    let t1 = corner - v1 * (setback / len1);
    let t2 = corner + v2 * (setback / len2);
    (t1, t2, bulge)
}

/// D4 — one fillet whose incoming ray origin IS the chain head (the
/// common authoring shape): the algebra vs `LoopBuilder::fillet` fed
/// the same virtual corner and the arrival anchor as `next` — the
/// shared closed form makes the trims, bulge, and declarations
/// bit-identical; the continuation leg and sharp seam match exactly.
#[test]
fn single_fillet_after_leg_matches_loopbuilder_fillet() {
    // The entry ray (side 1) departs east from the entry anchor a —
    // the fillet's incoming ray origin IS the chain head here, exactly
    // the shape `LoopBuilder::fillet` computes from. The arrival
    // carrier is the vertical line through the anchor (6, 2) heading
    // north; the virtual corner is (6, 0).
    let a = p2(0.0, 0.0);
    let anchor = p2(6.0, 2.0);
    let north = std::f64::consts::FRAC_PI_2;
    let r = 0.5;
    // The corner the algebra constructs, re-derived from the
    // documented closed form and pinned to its intended location.
    let corner = expected_corner(a, 0.0, anchor, north);
    assert!((corner.x - 6.0).abs() < 1e-12 && corner.y.abs() < 1e-12);
    let algebra = Open
        .at(a)
        .angle(0.0)
        .unwrap()
        .fillet(r)
        .unwrap()
        .at(anchor)
        .unwrap()
        .angle(north)
        .unwrap()
        // End the arrival side one unit past the anchor, then close
        // sharply through the top-left.
        .line(1.0)
        .unwrap()
        .line_to(p2(0.0, 3.0))
        .unwrap()
        .line_to(Start)
        .unwrap();
    // Hand equivalent: same corner value, next = the arrival anchor
    // (the algebra's documented canonical choice), then the same
    // continuation: the side extends from the trim point through the
    // anchor to anchor + 1·unit(north).
    let (sn, cn) = north.sin_cos();
    let side_end = anchor + Vec2::new(cn, sn) * 1.0;
    let hand = LoopBuilder::start(a)
        .fillet(corner, anchor, r)
        .unwrap()
        .line_to(side_end)
        .line_to(p2(0.0, 3.0))
        .close();
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// D5 — the flagship all-rounded square (4 anchors + 4 directions +
/// seam fillet, the PATHS-DESIGN §3 example): the algebra vs the
/// fully explicit hand chain a `LoopBuilder` author writes with the
/// trim points in hand (start at the seam arc's end, declare joint 0,
/// alternate trimmed sides and fillet arcs, close with the seam arc's
/// bulge). The hand numbers are re-derived here from the ratified
/// closed forms (corner = ray×carrier, trims/bulge = the documented
/// line×line form, anchor-based), and every derived vertex is
/// additionally pinned to its exact intended dyadic location within
/// 1e-12.
#[test]
fn rounded_square_with_seam_fillet_matches_explicit_hand_chain() {
    let m = [
        p2(0.0, -1.0), // side 1 midpoint, heading east
        p2(1.0, 0.0),  // side 2, north
        p2(0.0, 1.0),  // side 3, west
        p2(-1.0, 0.0), // side 4, south
    ];
    let north = std::f64::consts::FRAC_PI_2;
    let th = [0.0, north, std::f64::consts::PI, -north];
    let r = 0.25;

    let algebra = Open
        .at(m[0])
        .angle(th[0])
        .unwrap()
        .fillet(r)
        .unwrap()
        .at(m[1])
        .unwrap()
        .angle(th[1])
        .unwrap()
        .fillet(r)
        .unwrap()
        .at(m[2])
        .unwrap()
        .angle(th[2])
        .unwrap()
        .fillet(r)
        .unwrap()
        .at(m[3])
        .unwrap()
        .angle(th[3])
        .unwrap()
        .fillet(r)
        .unwrap()
        .to(Start)
        .unwrap();

    // Hand: corner k sits between side k (anchor m[k], direction
    // th[k]) and side k+1; the algebra's canonical trim inputs are
    // head = the ray origin (the side's anchor) and next = the arrival
    // anchor.
    let mut t1 = [p2(0.0, 0.0); 4];
    let mut t2 = [p2(0.0, 0.0); 4];
    let mut bulge = [0.0; 4];
    for k in 0..4 {
        let next = (k + 1) % 4;
        let corner = expected_corner(m[k], th[k], m[next], th[next]);
        let (a, b, c) = expected_trims(m[k], corner, m[next], r);
        t1[k] = a;
        t2[k] = b;
        bulge[k] = c;
    }
    // The exact intended locations (side 2, r = 1/4 — all dyadic).
    let exact_t1 = [
        p2(0.75, -1.0),
        p2(1.0, 0.75),
        p2(-0.75, 1.0),
        p2(-1.0, -0.75),
    ];
    let exact_t2 = [
        p2(1.0, -0.75),
        p2(0.75, 1.0),
        p2(-1.0, 0.75),
        p2(-0.75, -1.0),
    ];
    let quarter = 1.0 / (1.0 + 2.0_f64.sqrt());
    for k in 0..4 {
        assert!((t1[k].x - exact_t1[k].x).abs() < 1e-12, "t1[{k}].x");
        assert!((t1[k].y - exact_t1[k].y).abs() < 1e-12, "t1[{k}].y");
        assert!((t2[k].x - exact_t2[k].x).abs() < 1e-12, "t2[{k}].x");
        assert!((t2[k].y - exact_t2[k].y).abs() < 1e-12, "t2[{k}].y");
        assert!((bulge[k] - quarter).abs() < 1e-12, "bulge[{k}]");
    }

    let hand = LoopBuilder::start(t2[3])
        .declare_tangent() // joint 0: the seam arc meets side 1 tangentially
        .line_to(t1[0])
        .declare_tangent()
        .arc_to(t2[0], bulge[0])
        .declare_tangent()
        .line_to(t1[1])
        .declare_tangent()
        .arc_to(t2[1], bulge[1])
        .declare_tangent()
        .line_to(t1[2])
        .declare_tangent()
        .arc_to(t2[2], bulge[2])
        .declare_tangent()
        .line_to(t1[3])
        .declare_tangent()
        .close_with_bulge(bulge[3]);

    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
    assert_eq!(algebra.vertices.len(), 8);
    assert_eq!(algebra.tangent_joints.len(), 8);
}

/// D6 — a fillet arrival bound by `line_to` ("also from arrivals"):
/// binds the arrival direction toward the target, resolves the
/// fillet, and ends the side at the target — against the hand
/// `.fillet(corner, anchor, r)` + `line_to(end)` with the same corner
/// value.
#[test]
fn arrival_bound_by_line_to_matches_loopbuilder() {
    let a = p2(0.0, 0.0);
    let anchor = p2(6.0, 1.0);
    let end = p2(6.0, 3.0);
    let r = 0.5;
    let corner = expected_corner(a, 0.0, anchor, std::f64::consts::FRAC_PI_2);
    assert!((corner.x - 6.0).abs() < 1e-12 && corner.y.abs() < 1e-12);
    let algebra = Open
        .at(a)
        .angle(0.0)
        .unwrap()
        .fillet(r)
        .unwrap()
        .at(anchor)
        .unwrap()
        .line_to(end)
        .unwrap()
        .line_to(p2(0.0, 3.0))
        .unwrap()
        .line_to(Start)
        .unwrap();
    let hand = LoopBuilder::start(a)
        .fillet(corner, anchor, r)
        .unwrap()
        .line_to(end)
        .line_to(p2(0.0, 3.0))
        .close();
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}
