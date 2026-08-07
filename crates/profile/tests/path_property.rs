//! LIB-U2 PR-1 property + refusal suite:
//!
//! - every authored point lies on the final path (targets are
//!   vertices bit-for-bit; anchors lie on their trimmed side);
//! - lowered loops always pass the junction verifier —
//!   tangency-by-construction means the declared flags can never be
//!   caught lying (`UndeclaredTangency`/`TangencyContradicted` are
//!   unreachable from the algebra);
//! - refusals are typed, never panics (`clippy::panic` is denied in
//!   the lib; proptest exercises the near-tangent bands and the
//!   corner gates for totality);
//! - one deterministic row per typed refusal class.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tolerance};
use profile::path::{HasAng, HasPos, WithIncoming};
use profile::{
    LoopBuilder, Open, PartialPath, PathError, Profile, ProfileLoop, SketchPlane, Start,
};
use proptest::prelude::*;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn validate_ok(l: &ProfileLoop<f64>) {
    Profile::new(SketchPlane::xy(), vec![l.clone()])
        .validate(Tolerance::get())
        .expect("algebra-lowered loop passes the junction verifier");
}

/// Distance from `q` to the straight segment [a, b] (for the
/// anchor-on-path property; anchors in these cases sit on line sides).
fn seg_distance(q: Point2<f64>, a: Point2<f64>, b: Point2<f64>) -> f64 {
    let ab = b - a;
    let l2 = ab.norm_squared();
    if l2 == 0.0 {
        return (q - a).norm_squared().sqrt();
    }
    let t = (ab.dot(q - a) / l2).clamp(0.0, 1.0);
    let foot = a + ab * t;
    (q - foot).norm_squared().sqrt()
}

/// A convex polygon's vertices: distinct sorted angles about the
/// origin with per-vertex radii — junctions definitely sharp by
/// construction (angle gaps ≥ 0.15 rad, radii in [1, 3]).
fn convex_polygon() -> impl Strategy<Value = Vec<Point2<f64>>> {
    (3usize..8)
        .prop_flat_map(|n| {
            (
                proptest::collection::vec(0.15f64..1.0, n),
                proptest::collection::vec(1.0f64..3.0, n),
            )
        })
        .prop_map(|(gaps, radii)| {
            let total: f64 = gaps.iter().sum();
            let scale = std::f64::consts::TAU / total;
            let mut phi = 0.0;
            gaps.iter()
                .zip(&radii)
                .map(|(g, r)| {
                    phi += g * scale;
                    p2(r * phi.cos(), r * phi.sin())
                })
                .collect()
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// P1 — sharp polygons: the algebra's line_to chain is bit-
    /// identical to the LoopBuilder chain (every coordinate is
    /// authored), every authored point is a vertex, and the lowered
    /// loop validates — the junction verifier never fires on it.
    #[test]
    fn sharp_polygons_differential_and_verified(pts in convex_polygon()) {
        let mut path = Open.at(pts[0]).line_to(pts[1]).unwrap();
        for q in &pts[2..] {
            path = path.line_to(*q).unwrap();
        }
        let algebra = path.line_to(Start).unwrap();
        let mut hand = LoopBuilder::start(pts[0]);
        for q in &pts[1..] {
            hand = hand.line_to(*q);
        }
        let hand = hand.close();
        prop_assert_eq!(algebra.vertices.len(), hand.vertices.len());
        for (a, h) in algebra.vertices.iter().zip(&hand.vertices) {
            prop_assert_eq!(a.pos.x.to_bits(), h.pos.x.to_bits());
            prop_assert_eq!(a.pos.y.to_bits(), h.pos.y.to_bits());
            prop_assert_eq!(a.bulge.to_bits(), h.bulge.to_bits());
        }
        prop_assert!(algebra.tangent_joints.is_empty());
        for (k, q) in pts.iter().enumerate() {
            prop_assert_eq!(algebra.vertices[k].pos.x.to_bits(), q.x.to_bits());
            prop_assert_eq!(algebra.vertices[k].pos.y.to_bits(), q.y.to_bits());
        }
        validate_ok(&algebra);
    }

    /// P2 — a filleted axis-aligned box: the loop always validates
    /// (flags never caught lying), the authored anchor lies on its
    /// trimmed side, and the entry/targets are vertices.
    #[test]
    fn filleted_box_verified_and_anchor_on_path(
        w in 2.0f64..5.0,
        h in 2.0f64..5.0,
        anchor_frac in 0.3f64..0.9,
        r in 0.1f64..0.9,
    ) {
        // Entry ray east from the origin (side 1); arrival side is the
        // vertical line x = w heading north, anchored mid-side; the
        // radius keeps both trims strictly inside their anchored
        // extents (setback = r < min(w, anchor_y)).
        let anchor_y = h * anchor_frac;
        prop_assume!(r < anchor_y - 0.05 && r < w - 0.05);
        let anchor = p2(w, anchor_y);
        let north = std::f64::consts::FRAC_PI_2;
        let top_len = h - anchor_y;
        let algebra = Open
            .at(p2(0.0, 0.0))
            .angle(0.0).unwrap()
            .fillet(r).unwrap()
            .at(anchor).unwrap()
            .angle(north).unwrap()
            .line(top_len).unwrap()
            .line_to(p2(0.0, h)).unwrap()
            .line_to(Start).unwrap();
        validate_ok(&algebra);
        // The anchor lies on the trimmed arrival side: the segment
        // from the fillet arc's end (vertex 2) to the side's end
        // (vertex 3).
        let v = &algebra.vertices;
        prop_assert_eq!(v.len(), 5);
        let d = seg_distance(anchor, v[2].pos, v[3].pos);
        prop_assert!(d < 1e-9, "anchor off its side by {d:e}");
        // Authored entry/targets are vertices, bit-for-bit.
        prop_assert_eq!(v[0].pos.x.to_bits(), 0.0f64.to_bits());
        prop_assert_eq!(v[4].pos.x.to_bits(), 0.0f64.to_bits());
        prop_assert_eq!(v[4].pos.y.to_bits(), h.to_bits());
    }

    /// P3 — §4 item 1 totality across the tangent band: a departure
    /// within/near/beyond ε_input of the incoming tangent either
    /// proceeds or refuses TYPED — never panics, and the definite
    /// bands land where D4's two-tolerance principle says.
    #[test]
    fn junction_band_refusals_are_typed(
        dtheta in -1e-7f64..1e-7,
        flip in proptest::bool::ANY,
    ) {
        let leg = Open.at(p2(0.0, 0.0)).line_to(p2(2.0, 0.0)).unwrap();
        let dep = if flip { std::f64::consts::PI + dtheta } else { dtheta };
        let tol = Tolerance::get();
        // margin = sin(dtheta)·arm, arm = 2 (the leg length).
        let margin = (dtheta.sin() * 2.0).abs();
        match leg.angle(dep) {
            Ok(_) => prop_assert!(margin > tol.eps, "accepted inside the ε band"),
            Err(PathError::JunctionTangent { .. }) => {
                prop_assert!(!flip && margin <= tol.eps * tol.k);
            }
            Err(PathError::JunctionCusp { .. }) => {
                prop_assert!(flip && margin <= tol.eps * tol.k);
            }
            Err(PathError::Escalated { .. }) => {
                prop_assert!(margin >= tol.eps && margin <= tol.eps * tol.k);
            }
            Err(e) => prop_assert!(false, "unexpected refusal {e:?}"),
        }
    }
}

// ------------------------------------------------------------------
// Deterministic refusal rows: one per typed class.
// ------------------------------------------------------------------

/// A directed tip two legs in (east then north-east), for refusal rows.
fn bent_tip() -> PartialPath<f64, HasPos<WithIncoming>, HasAng> {
    Open.at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0))
        .unwrap()
        .angle(std::f64::consts::FRAC_PI_2)
        .unwrap()
}

#[test]
fn turn_zero_refuses_toward_tangent() {
    let leg = Open.at(p2(0.0, 0.0)).line_to(p2(2.0, 0.0)).unwrap();
    assert!(matches!(
        leg.turn(0.0),
        Err(PathError::JunctionTangent { .. })
    ));
}

#[test]
fn turn_pi_refuses_as_cusp() {
    let leg = Open.at(p2(0.0, 0.0)).line_to(p2(2.0, 0.0)).unwrap();
    assert!(matches!(
        leg.turn(std::f64::consts::PI),
        Err(PathError::JunctionCusp { .. })
    ));
}

#[test]
fn declared_straight_continuation_of_a_line_is_same_carrier() {
    let leg = Open.at(p2(0.0, 0.0)).line_to(p2(2.0, 0.0)).unwrap();
    assert!(matches!(
        leg.tangent().line(1.0),
        Err(PathError::SameCarrierJunction { .. })
    ));
}

#[test]
fn cocircular_tangent_arc_is_same_carrier() {
    // Lower unit semicircle from (−1, 0) to (1, 0) (bulge +1), then a
    // tangent arc to (0, 1) — a point on the SAME unit circle: the
    // constructed arc is the incoming carrier itself.
    let arc_end = Open
        .at(p2(-1.0, 0.0))
        .arc_to(p2(1.0, 0.0), 1.0)
        .unwrap()
        .tangent();
    assert!(matches!(
        arc_end.tangent_arc_to(p2(0.0, 1.0)),
        Err(PathError::SameCarrierJunction { .. })
    ));
}

#[test]
fn parallel_carriers_refuse_no_corner() {
    // Arrival side parallel to the departure ray.
    let arrival = bent_tip().fillet(0.5).unwrap().at(p2(4.0, 3.0)).unwrap();
    assert!(matches!(
        arrival.angle(std::f64::consts::FRAC_PI_2),
        Err(PathError::NoCornerForFillet { .. })
    ));
}

#[test]
fn corner_behind_ray_refuses_no_corner() {
    // Ray heads north from (2, 0); the arrival carrier crosses it at
    // y = −1 — behind the ray start.
    let arrival = bent_tip().fillet(0.5).unwrap().at(p2(4.0, -1.0)).unwrap();
    assert!(matches!(
        arrival.angle(0.0),
        Err(PathError::NoCornerForFillet { .. })
    ));
}

#[test]
fn trim_eating_an_anchor_refuses_typed() {
    // Corner (2, 2); the arrival anchor sits 0.5 past it but the
    // radius wants a 0.9 setback: the trim would eat the anchor.
    let arrival = bent_tip().fillet(0.9).unwrap().at(p2(2.5, 2.0)).unwrap();
    assert!(matches!(
        arrival.angle(0.0),
        Err(PathError::AnchorOutsideTrimmedExtent { .. })
    ));
}

#[test]
fn arc_arrival_refuses_typed() {
    let arrival = bent_tip().fillet(0.5).unwrap().at(p2(4.0, 2.0)).unwrap();
    assert!(matches!(
        arrival.arc_to(p2(5.0, 3.0), 0.4),
        Err(PathError::ArcArrivalFillet)
    ));
}

#[test]
fn seam_fillet_onto_an_arc_first_side_refuses_typed() {
    // Side 1 is an arc leg — a seam fillet cannot land on it in v1.
    let tip = Open
        .at(p2(0.0, 0.0))
        .arc_to(p2(4.0, 0.0), 0.3)
        .unwrap()
        .angle(2.0)
        .unwrap()
        .line(2.0)
        .unwrap()
        .angle(3.5)
        .unwrap();
    let arrival = tip.fillet(0.3).unwrap();
    assert!(matches!(
        arrival.to(Start),
        Err(PathError::SeamFilletOntoArc)
    ));
}

#[test]
fn tangent_line_close_refuses_always() {
    // The seam junction of a straight closer within the tangent band:
    // the closing line arrives at Start along the entry departure —
    // the PQ4 mid-side seam, refused with the two structural
    // spellings named.
    let refused = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0))
        .unwrap()
        .line_to(p2(2.0, 2.0))
        .unwrap()
        .line_to(p2(-2.0, 2.0))
        .unwrap()
        .line_to(p2(-2.0, 0.0))
        .unwrap()
        .line_to(Start);
    assert!(matches!(refused, Err(PathError::TangentLineClose { .. })));
}

#[test]
fn tangent_seam_closes_via_tangent_arc() {
    // The tangent seam (`.tangent().tangent_arc_to(Start)`): the
    // closing arc inherits the tip direction (declared) and meets the
    // entry SHARPLY at Start — one-sided tangency, exactly the doc's
    // seam. (A loop tangent on BOTH sides of its closer — the stadium
    // cap — is the seam fillet's territory, and with parallel side
    // carriers it refuses NoCornerForFillet: a reported finding.)
    let loop_ = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .unwrap()
        .tangent()
        .tangent_arc_to(p2(4.0, 1.0))
        .unwrap()
        .line_to(p2(0.5, 2.0))
        .unwrap()
        .tangent()
        .tangent_arc_to(Start)
        .unwrap();
    validate_ok(&loop_);
    // The two `.tangent()` joints are declared; the junctions at
    // (4, 1) and at Start are definitely sharp; the verifier confirms
    // every flag.
    assert_eq!(loop_.tangent_joints.len(), 2);
}
