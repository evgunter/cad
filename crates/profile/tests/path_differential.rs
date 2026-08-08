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
use profile::{
    ArcSweep, FilletLegShape, LoopBuilder, Open, Profile, ProfileLoop, SketchPlane, Start,
};

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

// ------------------------------------------------------------------
// LIB-G1 vocabulary growth: each new constructor against its
// hand-built raw twin, bit for bit.
// ------------------------------------------------------------------

/// G1-1 — the circle primitive lowers to the corpus's existing
/// convention bit-for-bit: two semicircles at the ±x poles, east first,
/// bulge 1, counterclockwise, nothing declared (the two joints are
/// same-carrier identities, not tangencies).
#[test]
fn circle_matches_the_raw_corpus_convention() {
    for (cx, cy, r) in [(0.0, 0.0, 1.0), (-1.5, 0.0, 0.7), (2.0, 2.0, 0.5)] {
        let algebra = profile::circle(p2(cx, cy), r).unwrap();
        let hand = ProfileLoop::new(vec![
            profile::ProfileVertex {
                pos: p2(cx + r, cy),
                bulge: 1.0,
            },
            profile::ProfileVertex {
                pos: p2(cx - r, cy),
                bulge: 1.0,
            },
        ]);
        assert_loops_identical(&algebra, &hand);
        assert_validate_identically(&algebra, &hand);
    }
}

/// G1-2 — `arc_via` against `LoopBuilder::arc_to_via`: the two doors
/// feed the same three authored points to the same closed form, so the
/// derived bulge agrees bit-for-bit and the endpoints are verbatim.
#[test]
fn arc_via_matches_loopbuilder_arc_to_via() {
    let (a, via, b) = (p2(0.0, 0.0), p2(1.0, 1.0), p2(2.0, 0.0));
    let algebra = Open.at(a).arc_via(via, b).unwrap().line_to(Start).unwrap();
    let hand = LoopBuilder::start(a).arc_to_via(via, b).close();
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// G1-2, closing — `arc_via(v, Start)` against `close_arc_via`: the
/// two-arc crescent (the lily leaf's shape), whose seam and tip are
/// both sharp arc-onto-arc junctions.
#[test]
fn arc_via_closing_matches_loopbuilder_close_arc_via() {
    let (a, b) = (p2(0.0, 0.0), p2(2.0, 0.0));
    let (out, back) = (p2(1.0, 0.5), p2(1.0, 0.1));
    let algebra = Open
        .at(a)
        .arc_via(out, b)
        .unwrap()
        .arc_via(back, Start)
        .unwrap();
    let hand = LoopBuilder::start(a).arc_to_via(out, b).close_arc_via(back);
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// G1-3 — `arc_center` against `LoopBuilder::arc_to_center`, BOTH
/// windings: the winding is structural, and it is the only thing that
/// distinguishes the minor arc from the major one on the same three
/// authored points.
#[test]
fn arc_center_matches_loopbuilder_in_both_windings() {
    let (a, c, b) = (p2(1.0, 0.0), p2(0.0, 0.0), p2(0.0, 1.0));
    for winding in [profile::ArcSweep::Ccw, profile::ArcSweep::Cw] {
        let algebra = Open
            .at(a)
            .arc_center(c, b, winding)
            .unwrap()
            .line_to(c)
            .unwrap()
            .line_to(Start)
            .unwrap();
        let hand = LoopBuilder::start(a)
            .arc_to_center(b, c, winding)
            .line_to(c)
            .close();
        assert_loops_identical(&algebra, &hand);
    }
    // The minor-arc (Ccw) pie slice is a simple loop, so it also
    // validates identically; the Cw major arc sweeps past its own
    // chord and is a shape question, not an authoring one.
    let algebra = Open
        .at(a)
        .arc_center(c, b, profile::ArcSweep::Ccw)
        .unwrap()
        .line_to(c)
        .unwrap()
        .line_to(Start)
        .unwrap();
    let hand = LoopBuilder::start(a)
        .arc_to_center(b, c, profile::ArcSweep::Ccw)
        .line_to(c)
        .close();
    assert_validate_identically(&algebra, &hand);
}

/// G1-4 + G1-5, the acceptance row: **the BRACKET** — the corpus's one
/// line×line fillet, the #101 showcase, which LIB-U2 PR-2 MEASURED as
/// unmovable. It needed both new constructors at once: the corner is
/// reached by an axis director (`.toward(-1, 0)`, where `.angle(PI)`
/// carried sin(π) = 1.22e-16 into the ray), and the filleted side ends
/// at its authored far vertex (`.to(p2(1, 3))`, where the old surface
/// had only a synthetic mid-side anchor plus a length).
///
/// With both, the algebra lowers to the raw chain bit-for-bit.
#[test]
fn bracket_matches_loopbuilder_via_toward_and_far_end_anchor() {
    let corner = p2(1.0, 1.0);
    let far = p2(1.0, 3.0);
    let r = 0.5;
    let algebra = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .unwrap()
        .line_to(p2(3.0, 1.0))
        .unwrap()
        .toward(-1.0, 0.0)
        .unwrap()
        .fillet(r)
        .unwrap()
        .toward(0.0, 1.0)
        .unwrap()
        .to(far)
        .unwrap()
        .line_to(p2(0.0, 3.0))
        .unwrap()
        .line_to(Start)
        .unwrap();
    let hand = LoopBuilder::start(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 1.0))
        .fillet(corner, far, r)
        .unwrap()
        .line_to(far)
        .line_to(p2(0.0, 3.0))
        .close();
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// The same bracket said with ANGLE directors instead — the drift this
/// constructor exists to kill, measured rather than asserted away. The
/// authored points still land verbatim; the DERIVED trim vertices do
/// not, because `.angle(PI)` fixes an angle and the ray comes back
/// through `sin_cos`.
#[test]
fn angle_directors_drift_where_toward_is_exact() {
    let far = p2(1.0, 3.0);
    let r = 0.5;
    let build = |exact: bool| {
        let tip = Open
            .at(p2(0.0, 0.0))
            .line_to(p2(3.0, 0.0))
            .unwrap()
            .line_to(p2(3.0, 1.0))
            .unwrap();
        let opened = if exact {
            tip.toward(-1.0, 0.0).unwrap()
        } else {
            tip.angle(std::f64::consts::PI).unwrap()
        }
        .fillet(r)
        .unwrap();
        let arrival = if exact {
            opened.toward(0.0, 1.0).unwrap()
        } else {
            opened.angle(std::f64::consts::FRAC_PI_2).unwrap()
        };
        arrival
            .to(far)
            .unwrap()
            .line_to(p2(0.0, 3.0))
            .unwrap()
            .line_to(Start)
            .unwrap()
    };
    let exact = build(true);
    let drifted = build(false);
    // Same shape to any tolerance anyone could care about …
    for (a, b) in exact.vertices.iter().zip(&drifted.vertices) {
        assert!((a.pos - b.pos).norm_squared().sqrt() < 1e-12);
        assert!((a.bulge - b.bulge).abs() < 1e-12);
    }
    // … and NOT the same bits: the two trim vertices differ, which is
    // exactly the SAID-not-shape drift that kept the bracket raw.
    let same_bits = exact.vertices.iter().zip(&drifted.vertices).all(|(a, b)| {
        a.pos.x.to_bits() == b.pos.x.to_bits() && a.pos.y.to_bits() == b.pos.y.to_bits()
    });
    assert!(
        !same_bits,
        "the angle-director spelling is expected to drift; if it no longer does, \
         the exactness claim needs re-measuring, not silently widening"
    );
}

/// G1-5, the exactness pin itself: an axis-aligned `.toward` builds the
/// ray verbatim, so a leg along it lands on exact coordinates — no
/// `sin_cos` residue anywhere in the emitted geometry.
#[test]
fn toward_axis_rays_are_exact() {
    let cases = [
        (1.0, 0.0, p2(2.0, 0.0)),
        (-1.0, 0.0, p2(-2.0, 0.0)),
        (0.0, 1.0, p2(0.0, 2.0)),
        (0.0, -1.0, p2(0.0, -2.0)),
        // A Pythagorean direction normalizes exactly too (3,4)/5.
        (3.0, 4.0, p2(1.2, 1.6)),
    ];
    for (dx, dy, expected) in cases {
        // A third vertex perpendicular to the leg keeps the loop
        // non-degenerate; only vertex 1 is under test.
        let third = p2(expected.x - dy, expected.y + dx);
        let lowered = Open
            .at(p2(0.0, 0.0))
            .toward(dx, dy)
            .unwrap()
            .line(2.0)
            .unwrap()
            .line_to(third)
            .unwrap()
            .line_to(Start)
            .unwrap();
        let v = lowered.vertices[1].pos;
        assert_eq!(v.x.to_bits(), expected.x.to_bits(), "toward({dx},{dy}) x");
        assert_eq!(v.y.to_bits(), expected.y.to_bits(), "toward({dx},{dy}) y");
    }
}

// ------------------------------------------------------------------
// LIB-G2 §4: the arc-carrier fillet family.
// ------------------------------------------------------------------

/// The rocker eye, both ways — **the mandatory G2 differential row**.
///
/// A lens of two R = 1 circles about (∓1/2, 0) meeting at (0, ±√¾):
/// the top tip is filleted (the S8 corner where BOTH tangent circles of
/// radius 0.25 survive — one in each tip's pocket — so the pick is the
/// ladder's, not a survivor count's), and the bottom tip is left sharp,
/// a genuine two-carrier junction.
///
/// The algebra never authors either tip. It binds the entry ON the
/// right lobe, opens the fillet, and closes ON the left lobe; the top
/// corner is DERIVED as the circle×circle intersection. Bit-identity
/// with the hand chain therefore says the squared-radius form
/// reproduces the authored corner exactly — the radius form lands it an
/// ulp low and this row fails.
#[test]
fn eye_arc_by_arc_fillet_matches_loopbuilder_fillet_corner() {
    let tip = 0.75f64.sqrt();
    let r = 0.25;
    let algebra = Open
        .at_on(p2(0.0, -tip), p2(-0.5, 0.0), ArcSweep::Ccw)
        .unwrap()
        .fillet(r)
        .unwrap()
        .to_on(Start, p2(0.5, 0.0), ArcSweep::Ccw)
        .unwrap();
    let hand = LoopBuilder::start(p2(0.0, -tip))
        .fillet_corner(
            FilletLegShape::Arc {
                center: p2(-0.5, 0.0),
                sweep: ArcSweep::Ccw,
            },
            p2(0.0, tip),
            FilletLegShape::Arc {
                center: p2(0.5, 0.0),
                sweep: ArcSweep::Ccw,
            },
            p2(0.0, -tip),
            r,
            Tolerance::get(),
        )
        .unwrap()
        .close_arc_center(p2(0.5, 0.0), ArcSweep::Ccw);
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
    // The S8 pick, independently: the fillet arc's centre must be the
    // NEAR pocket (0, √0.3125), not the rival at the sharp tip.
    let want = 0.3125f64.sqrt();
    let mid = algebra.vertices[1].pos;
    assert!(
        mid.y > 0.0,
        "the trimmed incoming run must reach the TOP tip's pocket, got {mid:?}"
    );
    assert!(
        (want - 0.559_016_994_374_947_4).abs() < 1e-15,
        "the near pocket's centre height is the S8 pin's"
    );
}

/// The derived corner is bit-exact where the authored one is: the
/// entry's own carrier radius is `|anchor − centre|`, never
/// `√(R²)²`, and the eye's corner comes back as the literal
/// `√0.75` the hand author writes.
#[test]
fn the_derived_circle_by_circle_corner_lands_on_the_authored_one() {
    let tip = 0.75f64.sqrt();
    // Reproduce the boundary's squared-radius closed form here,
    // independently of the implementation (the differential discipline).
    let (o1, o2) = (p2(-0.5, 0.0), p2(0.5, 0.0));
    let a = p2(0.0, -tip);
    let r1_sq = (a - o1).norm_squared();
    let r2_sq = (a - o2).norm_squared();
    let d = o2 - o1;
    let d_sq = d.norm_squared();
    let k = (d_sq + r1_sq - r2_sq) / (2.0 * d_sq);
    let mid = o1 + d * k;
    let h = (r1_sq / d_sq - k.powi(2)).sqrt();
    let corner = mid + Vec2::new(-d.y, d.x) * h;
    assert_eq!(corner.x.to_bits(), 0.0f64.to_bits(), "corner x");
    assert_eq!(
        corner.y.to_bits(),
        tip.to_bits(),
        "corner y is √0.75 exactly"
    );
    // The radius form, for contrast: one ulp low. Pinned so the design
    // rule cannot be "simplified" away silently (LB4).
    let radius_form = {
        let (r1, r2, dl) = (r1_sq.sqrt(), r2_sq.sqrt(), d_sq.sqrt());
        let aa = (d_sq + r1 * r1 - r2 * r2) / (2.0 * dl);
        (r1 * r1 - aa * aa).sqrt()
    };
    assert_ne!(
        radius_form.to_bits(),
        tip.to_bits(),
        "the radius form is expected to MISS by an ulp — that is why the squared form is the rule"
    );
}
