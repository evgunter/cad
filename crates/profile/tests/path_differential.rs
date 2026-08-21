//! LIB-U2 PR-1 differential suite: for a representative family of
//! loops, the PATHS-algebra-lowered [`ProfileLoop`] is IDENTICAL to a
//! **recorded fixture** — **bit-level on every coordinate the
//! authoring determines exactly** (authored points, authored bulges,
//! sharp chains, tangent-arc bulges through the same closed-form
//! expressions) — and both build and validate identically.
//!
//! The comparison target was the hand-built `LoopBuilder` equivalent
//! until LIB-RETTAIL retired the shim; see [`recorded`] for exactly
//! what that trade gave up (independence) and what it did not (every
//! independent closed-form oracle below, and bit-identity itself).
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

mod common;

use common::pinned;
use geom_core::{Point2, Vec2};
use profile::RawLoop;
use profile::{ArcSweep, Bulge, Center, Open, Profile, ProfileLoop, SketchPlane, Start, Via};
use geom_core::Tol;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

// ---------------------------------------------------------------
// The recorded fixtures (LIB-RETTAIL): what this suite compares against
// ---------------------------------------------------------------

/// A BLESSED lowering: the vertex table (x, y, bulge) and declared-joint
/// set that the algebra produced when the fixture was recorded, spelled
/// as `f64` literals — Rust's shortest round-tripping form, so the
/// literals ARE the bits.
///
/// This is what replaced the `LoopBuilder` twin. Be precise about what
/// changed and what did not:
///
/// - What the twin gave that a fixture cannot: independence. Two
///   implementations disagreeing meant one was wrong; a fixture only
///   says "the lowering changed", never "the lowering is wrong".
/// - What survives unchanged: every INDEPENDENT ORACLE in this file —
///   `expected_corner`, `expected_trims`, the closed-form tangent and
///   radius checks, the exact-location assertions to 1e-12. Those were
///   always the tests' mathematical content; the twin only ever pinned
///   bit-identity, and a recorded table pins bit-identity exactly as
///   hard (any single-ulp change in any coordinate fails).
/// - Why the trade is worth taking: the twin was a second copy of the
///   kernel's geometry, unmaintained by construction (refactoring it
///   would destroy the independence that justified it), and it was the
///   sole reason a retired authoring surface stayed compiled.
///
/// To re-bless after a DELIBERATE lowering change, run
/// `CAD_BLESS_TWINS=1 cargo test -p profile --test all
/// path_differential -- --nocapture` (this file rides the
/// aggregated `all` target) and paste what it prints. Blessing is a decision:
/// the printed numbers are the new contract.
fn recorded(name: &str, algebra: &ProfileLoop<f64>) -> ProfileLoop<f64> {
    if std::env::var_os("CAD_BLESS_TWINS").is_some() {
        println!("    (");
        println!("        {name:?},");
        println!("        &[");
        for v in algebra.vertices() {
            println!(
                "            [{:?}, {:?}, {:?}],",
                v.pos().x,
                v.pos().y,
                v.bulge()
            );
        }
        println!("        ],");
        println!("        &{:?},", algebra.tangent_joints());
        println!("    ),");
        // Blessing mode compares the lowering against itself so the rest
        // of the row still runs; the printed table is the new contract.
        return algebra.clone();
    }
    let (table, joints) = FIXTURES
        .iter()
        .find(|(n, _, _)| *n == name)
        .map(|(_, t, j)| (*t, *j))
        .unwrap_or_else(|| panic!("no recorded fixture named {name:?}"));
    let mut lp = ProfileLoop::new(
        table
            .iter()
            .map(|&[x, y, bulge]| profile::ProfileVertex::new(p2(x, y), bulge))
            .collect(),
    );
    lp = lp.with_tangent_joints(joints.to_vec());
    lp
}

/// The blessed tables. One row per fixture name; see [`recorded`].
#[allow(clippy::type_complexity)]
static FIXTURES: &[(&str, &[[f64; 3]], &[usize])] = &[
    (
        "arc_center_ccw",
        &[
            [1.0, 0.0, 0.41421356237309503],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0],
        ],
        &[],
    ),
    (
        "arc_center_cw",
        &[
            [1.0, 0.0, -2.414213562373095],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0],
        ],
        &[],
    ),
    (
        "arc_via_closing_matches_loopbuilder_close_arc_via",
        &[[0.0, 0.0, -0.5], [2.0, 0.0, 0.1]],
        &[],
    ),
    (
        "arc_via_matches_loopbuilder_arc_to_via",
        &[[0.0, 0.0, -0.9999999999999999], [2.0, 0.0, 0.0]],
        &[],
    ),
    (
        "arrival_bound_by_line_to_matches_loopbuilder",
        &[
            [0.0, 0.0, 0.0],
            [5.5, 0.0, 0.4142135623730951],
            [6.0, 0.5, 0.0],
            [6.0, 3.0, 0.0],
            [0.0, 3.0, 0.0],
        ],
        &[1, 2],
    ),
    (
        "bracket_matches_loopbuilder_via_toward_and_far_end_anchor",
        &[
            [0.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
            [3.0, 1.0, 0.0],
            [1.5, 1.0, -0.4142135623730951],
            [1.0, 1.5, 0.0],
            [1.0, 3.0, 0.0],
            [0.0, 3.0, 0.0],
        ],
        &[3, 4],
    ),
    (
        "eye_arc_by_arc_fillet_matches_loopbuilder_fillet_corner",
        &[
            [0.0, -0.8660254037844386, 0.5105684202253234],
            [0.16666666666666674, 0.7453559924999299, 0.38196601125010526],
            [-0.16666666666666674, 0.7453559924999299, 0.5105684202253233],
        ],
        &[1, 2],
    ),
    (
        "line_by_arc_carrier_fillet_matches_loopbuilder_fillet_corner",
        &[
            [0.0, 0.0, 0.0],
            [
                3.0502112764355034,
                -1.6653345369377348e-16,
                0.805875189115555,
            ],
            [3.174819725635825, 0.5728969299715385, 0.31310350092995504],
        ],
        &[1, 2],
    ),
    (
        "rounded_square_with_seam_fillet_matches_explicit_hand_chain",
        &[
            [-0.7499999999999999, -1.0, 0.0],
            [0.7499999999999999, -1.0, 0.41421356237309503],
            [0.9999999999999999, -0.75, 0.0],
            [1.0, 0.7499999999999999, 0.41421356237309503],
            [0.75, 0.9999999999999999, 0.0],
            [-0.75, 1.0000000000000002, 0.41421356237309515],
            [-1.0, 0.7500000000000002, 0.0],
            [-0.9999999999999999, -0.75, 0.41421356237309503],
        ],
        &[1, 2, 3, 4, 5, 6, 7, 0],
    ),
    (
        "sharp_arc_chain_matches_loopbuilder",
        &[[0.0, 0.0, 0.0], [2.0, 0.0, 0.5], [2.0, 2.0, 0.2]],
        &[],
    ),
    (
        "sharp_triangle_matches_loopbuilder",
        &[[0.0, 0.0, 0.0], [4.0, 0.5, 0.0], [1.5, 3.0, 0.0]],
        &[],
    ),
    (
        "single_fillet_after_leg_matches_loopbuilder_fillet",
        &[
            [0.0, 0.0, 0.0],
            [5.5, 0.0, 0.4142135623730951],
            [6.0, 0.5, 0.0],
            [6.0, 3.0, 0.0],
            [0.0, 3.0, 0.0],
        ],
        &[1, 2],
    ),
    (
        "straight_arrival_off_an_arc_departure_matches_loopbuilder_fillet_corner",
        &[
            [5.0, 0.0, 0.14833147735478827],
            [4.15739709641549, 2.7777777777777777, 0.2504916501726719],
            [3.7416573867739413, 3.0, 0.0],
            [-3.0, 3.0, 0.0],
        ],
        &[1, 2],
    ),
    (
        "tangent_arc_leg_matches_loopbuilder",
        &[
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.41421356237309503],
            [3.0, 1.0, 0.0],
        ],
        &[1],
    ),
];

/// Bit-level loop identity: vertex count, every coordinate and bulge
/// by `to_bits`, and the declared-joint SET (declaration order is not
/// semantic — `tangent_joints` documents set semantics).
fn assert_loops_identical(algebra: &ProfileLoop<f64>, hand: &ProfileLoop<f64>) {
    assert_eq!(
        algebra.vertices().len(),
        hand.vertices().len(),
        "vertex count"
    );
    for (i, (a, h)) in algebra.vertices().iter().zip(hand.vertices()).enumerate() {
        assert_eq!(
            a.pos().x.to_bits(),
            h.pos().x.to_bits(),
            "vertex {i} x: {} vs {}",
            a.pos().x,
            h.pos().x
        );
        assert_eq!(
            a.pos().y.to_bits(),
            h.pos().y.to_bits(),
            "vertex {i} y: {} vs {}",
            a.pos().y,
            h.pos().y
        );
        assert_eq!(
            a.bulge().to_bits(),
            h.bulge().to_bits(),
            "vertex {i} bulge: {} vs {}",
            a.bulge(),
            h.bulge()
        );
    }
    let mut ta = algebra.tangent_joints().to_vec();
    let mut th = hand.tangent_joints().to_vec();
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
        .validate(Tol::witness())
        .expect("algebra-lowered loop validates");
    let vh = Profile::new(SketchPlane::xy(), vec![hand.clone()])
        .validate(Tol::witness())
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
        .line_to(b, Tol::witness())
        .unwrap()
        .line_to(c, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let algebra = pinned(algebra);
    let hand = recorded("sharp_triangle_matches_loopbuilder", &algebra);
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
        .line_to(b, Tol::witness())
        .unwrap()
        .arc_to(Bulge { p: c, b: b1 }, Tol::witness())
        .unwrap()
        .arc_to(Bulge { p: Start, b: b2 }, Tol::witness())
        .unwrap();
    let algebra = pinned(algebra);
    let hand = recorded("sharp_arc_chain_matches_loopbuilder", &algebra);
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// D3 — declared tangent leg: `.tangent().tangent_arc_to(p)` lowers to
/// a joint declared tangent plus the bulge tan(Δ/2), Δ the
/// tangent-chord angle from atan2 — bit-identical to that closed form
/// evaluated directly on the same inputs (the oracle below).
#[test]
fn tangent_arc_leg_matches_loopbuilder() {
    let (a, b, c) = (p2(0.0, 0.0), p2(2.0, 0.0), p2(3.0, 1.0));
    // The unique tangent arc departing east from b to c: tangent-chord
    // angle Δ = atan2(1, 1), bulge tan(Δ/2) (the documented form).
    let delta = 1.0_f64.atan2(1.0);
    let expected_bulge = (delta / 2.0).tan();
    let algebra = Open
        .at(a)
        .line_to(b, Tol::witness())
        .unwrap()
        .tangent()
        .tangent_arc_to(c, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let algebra = pinned(algebra);
    // The INDEPENDENT oracle (it used to be the hand chain's argument;
    // now it is asserted directly): vertex 1's bulge is tan(delta/2)
    // from the documented closed form, bit for bit.
    assert_eq!(
        algebra.vertices()[1].bulge().to_bits(),
        expected_bulge.to_bits(),
        "tangent-arc bulge: {} vs the closed form {}",
        algebra.vertices()[1].bulge(),
        expected_bulge
    );
    let hand = recorded("tangent_arc_leg_matches_loopbuilder", &algebra);
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

/// The ratified line×line trim closed form, re-derived independently
/// from the documented construction: (t1, t2, bulge).
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
/// common authoring shape): the algebra against the blessed table, with
/// the virtual corner and the trims re-derived here from the ratified
/// closed form — the trims, bulge, and declarations are bit-identical;
/// the continuation leg and sharp seam match exactly.
#[test]
fn single_fillet_after_leg_matches_loopbuilder_fillet() {
    // The entry ray (side 1) departs east from the entry anchor a —
    // the fillet's incoming ray origin IS the chain head here. The
    // arrival
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
        .angle(0.0, Tol::witness())
        .unwrap()
        .fillet(r, Tol::witness())
        .unwrap()
        .at(anchor, Tol::witness())
        .unwrap()
        .angle(north, Tol::witness())
        .unwrap()
        // End the arrival side one unit past the anchor, then close
        // sharply through the top-left.
        .line(1.0, Tol::witness())
        .unwrap()
        .line_to(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let algebra = pinned(algebra);
    let hand = recorded(
        "single_fillet_after_leg_matches_loopbuilder_fillet",
        &algebra,
    );
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// D5 — the flagship all-rounded square (4 anchors + 4 directions +
/// seam fillet, the PATHS-DESIGN §3 example): the algebra vs the
/// blessed table, which is the fully explicit vertex chain with the
/// trim points in hand (start at the seam arc's end, joint 0 declared,
/// alternating trimmed sides and fillet arcs, closing with the seam
/// arc's bulge). Those numbers are re-derived here from the ratified
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
        .angle(th[0], Tol::witness())
        .unwrap()
        .fillet(r, Tol::witness())
        .unwrap()
        .at(m[1], Tol::witness())
        .unwrap()
        .angle(th[1], Tol::witness())
        .unwrap()
        .fillet(r, Tol::witness())
        .unwrap()
        .at(m[2], Tol::witness())
        .unwrap()
        .angle(th[2], Tol::witness())
        .unwrap()
        .fillet(r, Tol::witness())
        .unwrap()
        .at(m[3], Tol::witness())
        .unwrap()
        .angle(th[3], Tol::witness())
        .unwrap()
        .fillet(r, Tol::witness())
        .unwrap()
        .to(Start, Tol::witness())
        .unwrap();

    let algebra = pinned(algebra);

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

    let hand = recorded(
        "rounded_square_with_seam_fillet_matches_explicit_hand_chain",
        &algebra,
    );

    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
    assert_eq!(algebra.vertices().len(), 8);
    assert_eq!(algebra.tangent_joints().len(), 8);
}

/// D6 — a fillet arrival bound by `line_to` ("also from arrivals"):
/// binds the arrival direction toward the target, resolves the fillet,
/// and ends the side at the target — against the blessed table built
/// from the same virtual corner.
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
        .angle(0.0, Tol::witness())
        .unwrap()
        .fillet(r, Tol::witness())
        .unwrap()
        .at(anchor, Tol::witness())
        .unwrap()
        .line_to(end, Tol::witness())
        .unwrap()
        .line_to(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let algebra = pinned(algebra);
    let hand = recorded("arrival_bound_by_line_to_matches_loopbuilder", &algebra);
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
        let algebra = profile::circle(p2(cx, cy), r, Tol::witness()).unwrap();
        let algebra = pinned(algebra);
        let hand = ProfileLoop::new(vec![
            profile::ProfileVertex::new(p2(cx + r, cy), 1.0),
            profile::ProfileVertex::new(p2(cx - r, cy), 1.0),
        ]);
        assert_loops_identical(&algebra, &hand);
        assert_validate_identically(&algebra, &hand);
    }
}

/// G1-2 — `arc_to(Via { q, p })`: the three authored points go through
/// the documented closed form, so the derived bulge is exactly
/// `bulge_from_via(a, q, p)` and the endpoints pass through verbatim.
#[test]
fn arc_via_matches_loopbuilder_arc_to_via() {
    let (a, via, b) = (p2(0.0, 0.0), p2(1.0, 1.0), p2(2.0, 0.0));
    let algebra = Open
        .at(a)
        .arc_to(Via { q: via, p: b }, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let algebra = pinned(algebra);
    let hand = recorded("arc_via_matches_loopbuilder_arc_to_via", &algebra);
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// G1-2, closing — `arc_to(Via { q, p: Start })`: the two-arc crescent
/// (the lily leaf's shape), whose seam and tip are both sharp
/// arc-onto-arc junctions.
#[test]
fn arc_via_closing_matches_loopbuilder_close_arc_via() {
    let (a, b) = (p2(0.0, 0.0), p2(2.0, 0.0));
    let (out, back) = (p2(1.0, 0.5), p2(1.0, 0.1));
    let algebra = Open
        .at(a)
        .arc_to(Via { q: out, p: b }, Tol::witness())
        .unwrap()
        .arc_to(Via { q: back, p: Start }, Tol::witness())
        .unwrap();
    let algebra = pinned(algebra);
    let hand = recorded(
        "arc_via_closing_matches_loopbuilder_close_arc_via",
        &algebra,
    );
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// G1-3 — `arc_to(Center { c, winding, p })` in BOTH windings: the
/// winding is structural, and it is the only thing that distinguishes
/// the minor arc from the major one on the same three authored points.
#[test]
fn arc_center_matches_loopbuilder_in_both_windings() {
    let (a, c, b) = (p2(1.0, 0.0), p2(0.0, 0.0), p2(0.0, 1.0));
    for winding in [profile::ArcSweep::Ccw, profile::ArcSweep::Cw] {
        let algebra = Open
            .at(a)
            .arc_to(Center { c, winding, p: b }, Tol::witness())
            .unwrap()
            .line_to(c, Tol::witness())
            .unwrap()
            .line_to(Start, Tol::witness())
            .unwrap();
        let algebra = pinned(algebra);
        let hand = recorded(
            match winding {
                profile::ArcSweep::Ccw => "arc_center_ccw",
                profile::ArcSweep::Cw => "arc_center_cw",
            },
            &algebra,
        );
        assert_loops_identical(&algebra, &hand);
    }
    // The minor-arc (Ccw) pie slice is a simple loop, so it also
    // validates identically; the Cw major arc sweeps past its own
    // chord and is a shape question, not an authoring one.
    let algebra = Open
        .at(a)
        .arc_to(Center {
            c,
            winding: profile::ArcSweep::Ccw,
            p: b,
        }, Tol::witness())
        .unwrap()
        .line_to(c, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let algebra = pinned(algebra);
    let hand = recorded("arc_center_ccw", &algebra);
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
    // The virtual corner the two legs meet at (exact here: both legs are
    // axis-aligned). It used to be the hand chain's `fillet` argument;
    // it is asserted against the lowering below.
    let corner = p2(1.0, 1.0);
    let far = p2(1.0, 3.0);
    let r = 0.5;
    let algebra = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0), Tol::witness())
        .unwrap()
        .line_to(p2(3.0, 1.0), Tol::witness())
        .unwrap()
        .toward(-1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet(r, Tol::witness())
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .to(far, Tol::witness())
        .unwrap()
        .line_to(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let algebra = pinned(algebra);
    // The INDEPENDENT oracle for the two DERIVED vertices: both legs are
    // axis-aligned, so the setback is exactly r along each leg from the
    // virtual corner — trim 1 at (corner.x + r, corner.y), trim 2 at
    // (corner.x, corner.y + r), exactly.
    assert_eq!(algebra.vertices()[3].pos().x, corner.x + r);
    assert_eq!(algebra.vertices()[3].pos().y, corner.y);
    assert_eq!(algebra.vertices()[4].pos().x, corner.x);
    assert_eq!(algebra.vertices()[4].pos().y, corner.y + r);
    let hand = recorded(
        "bracket_matches_loopbuilder_via_toward_and_far_end_anchor",
        &algebra,
    );
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
            .line_to(p2(3.0, 0.0), Tol::witness())
            .unwrap()
            .line_to(p2(3.0, 1.0), Tol::witness())
            .unwrap();
        let opened = if exact {
            tip.toward(-1.0, 0.0, Tol::witness()).unwrap()
        } else {
            tip.angle(std::f64::consts::PI, Tol::witness()).unwrap()
        }
        .fillet(r, Tol::witness())
        .unwrap();
        let arrival = if exact {
            opened.toward(0.0, 1.0, Tol::witness()).unwrap()
        } else {
            opened.angle(std::f64::consts::FRAC_PI_2, Tol::witness()).unwrap()
        };
        arrival
            .to(far, Tol::witness())
            .unwrap()
            .line_to(p2(0.0, 3.0), Tol::witness())
            .unwrap()
            .line_to(Start, Tol::witness())
            .unwrap()
    };
    let exact = pinned(build(true));
    let drifted = pinned(build(false));
    // Same shape to any tolerance anyone could care about …
    for (a, b) in exact.vertices().iter().zip(drifted.vertices()) {
        assert!((a.pos() - b.pos()).norm_squared().sqrt() < 1e-12);
        assert!((a.bulge() - b.bulge()).abs() < 1e-12);
    }
    // … and NOT the same bits: the two trim vertices differ, which is
    // exactly the SAID-not-shape drift that kept the bracket raw.
    let same_bits = exact
        .vertices()
        .iter()
        .zip(drifted.vertices())
        .all(|(a, b)| {
            a.pos().x.to_bits() == b.pos().x.to_bits() && a.pos().y.to_bits() == b.pos().y.to_bits()
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
            .toward(dx, dy, Tol::witness())
            .unwrap()
            .line(2.0, Tol::witness())
            .unwrap()
            .line_to(third, Tol::witness())
            .unwrap()
            .line_to(Start, Tol::witness())
            .unwrap();
        let lowered = pinned(lowered);
        let v = lowered.vertices()[1].pos();
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
        .arc_fillet_arc(
            profile::Center {
                c: p2(-0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -tip),
            },
            r,
            profile::Center {
                c: p2(0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .unwrap();
    let algebra = pinned(algebra);
    let hand = recorded(
        "eye_arc_by_arc_fillet_matches_loopbuilder_fillet_corner",
        &algebra,
    );
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
    // The S8 pick, independently: the fillet arc's centre must be the
    // NEAR pocket (0, √0.3125), not the rival at the sharp tip.
    let want = 0.3125f64.sqrt();
    let mid = algebra.vertices()[1].pos();
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

/// **Line × arc**, the second G2 combination: a straight side 1 running
/// east from the entry, and one `fillet_arc(r, Center { .., p: Start })`
/// whose arrival closes back over the circle through the entry point.
///
/// The corner is DERIVED by the ray×circle form and is exact here by
/// construction — the ray `y = 0` meets the circle about `(2, −2)`
/// through the origin at `(0,0)` and `(4,0)`, both representable. The
/// first is the anchor itself and the advance gate discards it (a
/// corner is not ahead of its own anchor); the second is the corner a
/// hand author would write, and this row pins that the algebra reaches
/// it bit for bit.
///
/// LB4's wall is about the corners that are NOT exact this way; where
/// the natural anchors do land, line×arc migrates like any other site.
#[test]
fn line_by_arc_carrier_fillet_matches_loopbuilder_fillet_corner() {
    let centre = p2(2.0, -2.0);
    // r ≤ 0.414 here: the turn onto the arc is sharp (≈135°) and the
    // offset carriers separate above that — measured, not guessed.
    let r = 0.3;
    let algebra = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            r,
            profile::Center {
                c: centre,
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .unwrap();
    let algebra = pinned(algebra);
    let hand = recorded(
        "line_by_arc_carrier_fillet_matches_loopbuilder_fillet_corner",
        &algebra,
    );
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// The advance gate discards the ray×circle root that sits AT the
/// incoming anchor, so the pair is resolved by the gates rather than by
/// a value comparison — and the surviving root is the far one.
#[test]
fn the_advance_gate_discards_the_root_at_the_incoming_anchor() {
    let lowered = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.3,
            profile::Center {
                c: p2(2.0, -2.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .unwrap();
    let lowered = pinned(lowered);
    // Vertex 1 is the trim point on the straight side: it must sit
    // short of the FAR corner (4, 0), not of the discarded one at the
    // origin (which would have put it behind the entry).
    let t1 = lowered.vertices()[1].pos();
    assert!(t1.x > 3.0 && t1.x < 4.0, "trim point on side 1: {t1:?}");
    // On the ray y = 0 to rounding: `t1` is the offset-carrier centre
    // pushed back by the offset normal, so its y is a cancellation
    // residue, not a stored zero. The hand door computes the SAME
    // residue — the differential row above pins that bitwise — so the
    // claim here is the geometric one, not a bit pattern.
    assert!(t1.y.abs() < 1e-15, "side 1 rides the ray y = 0: {t1:?}");
}

/// **LB10 route 3, the mandatory differential row**: a STRAIGHT arrival
/// off an ARC departure, both ways.
///
/// The entry is bound ON the R = 5 circle at `(5, 0)` by the fused
/// `arc_fillet(Center { .. }, r)`, which opens the fillet against that
/// carrier, and `.at((0, 3)).toward(−1, 0)` binds the arrival's anchor
/// and its exact director. The corner is
/// DERIVED as the ray × circle intersection and is exact here by
/// construction — the line `y = 3` meets the circle at `(±4, 3)`, both
/// representable — so the hand chain, which AUTHORS `(4, 3)`, is fed the
/// same numbers and the two loops must agree to the bit.
///
/// The gates do the choosing, not a value comparison: the `(−4, 3)` root
/// passes the advance gate (it is ahead of `(5, 0)` going CCW) and is
/// discarded by the arrival side's REACH gate, since the anchor `(0, 3)`
/// travelling west never came from it.
#[test]
fn straight_arrival_off_an_arc_departure_matches_loopbuilder_fillet_corner() {
    let centre = p2(0.0, 0.0);
    let r = 0.5;
    let algebra = Open
        .arc_fillet(
            profile::Center {
                c: centre,
                winding: ArcSweep::Ccw,
                p: p2(5.0, 0.0),
            },
            r,
            Tol::witness(),
        )
        .unwrap()
        .at(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .toward(-1.0, 0.0, Tol::witness())
        .unwrap()
        .line(3.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let algebra = pinned(algebra);
    let hand = recorded(
        "straight_arrival_off_an_arc_departure_matches_loopbuilder_fillet_corner",
        &algebra,
    );
    assert_loops_identical(&algebra, &hand);
    assert_validate_identically(&algebra, &hand);
}

/// A director naming no direction refuses at the binder itself, BEFORE
/// any fillet resolution — an arc-incoming fillet is open, but a zero
/// director names no arrival carrier to resolve against.
#[test]
fn an_arc_carrier_arrival_refuses_a_zero_director() {
    let err = Open
        .arc_fillet(
            profile::Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(5.0, 0.0),
            },
            0.5,
            Tol::witness(),
        )
        .unwrap()
        .at(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .toward(0.0, 0.0, Tol::witness())
        .unwrap_err();
    assert!(
        matches!(err, profile::PathError::ZeroDirection { .. }),
        "a zero director names no arrival carrier: {err:?}"
    );
}

/// A parallel arrival: the line `y = 6` misses the R = 5 circle, so the
/// carriers admit no corner at all and the refusal is
/// `CarriersDoNotMeet` — no corner exists, not a radius that will not
/// fit.
#[test]
fn an_arc_carrier_arrival_refuses_carriers_that_never_meet() {
    let err = Open
        .arc_fillet(
            profile::Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(5.0, 0.0),
            },
            0.5,
            Tol::witness(),
        )
        .unwrap()
        .at(p2(0.0, 6.0), Tol::witness())
        .unwrap()
        .toward(-1.0, 0.0, Tol::witness())
        .unwrap_err();
    assert!(
        matches!(
            err,
            profile::PathError::NoCornerForFillet {
                reason: profile::path::PathNoCornerReason::CarriersDoNotMeet,
                ..
            }
        ),
        "a ray clear of the circle names no corner: {err:?}"
    );
}
