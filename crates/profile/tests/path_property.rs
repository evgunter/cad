//! LIB-U2 PR-1 property + refusal suite:
//!
//! - every authored point lies on the final path (targets are
//!   vertices bit-for-bit; anchors lie on their trimmed side);
//! - the verify layer's two tangency refusals
//!   (`UndeclaredTangency`/`TangencyContradicted`) are unreachable
//!   from the typed surface: tangency enters only by construction
//!   (declared flags are never claims about independently-typed
//!   numbers), and the sign-domain doors that could fake a
//!   construction (negative leg length, non-positive fillet radius)
//!   refuse typed at authoring. Other verify doors (simplicity,
//!   degeneracy) still judge the SHAPE — the lattice guarantees the
//!   authoring, never the geometry;
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
fn turn_pi_refuses_as_cusp_naming_131() {
    let leg = Open.at(p2(0.0, 0.0)).line_to(p2(2.0, 0.0)).unwrap();
    let err = leg.turn(std::f64::consts::PI).unwrap_err();
    assert!(matches!(err, PathError::JunctionCusp { .. }));
    // §4 item 1: the refusal names #131 as the (absent) front door —
    // pinned here, not just carried in prose.
    assert!(
        err.to_string().contains("#131"),
        "cusp refusal must name #131: {err}"
    );
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

/// **G2**: `ArcArrivalFillet` is RETIRED — an arc arrival is no longer
/// "out of scope in v1", it is spelled by the carrier binder. The door
/// that used to refuse it now names the door that does the job.
#[test]
fn an_arc_leg_from_a_bound_arrival_names_the_carrier_binder() {
    let arrival = bent_tip().fillet(0.5).unwrap().at(p2(4.0, 2.0)).unwrap();
    let err = arrival.arc_to(p2(5.0, 3.0), 0.4).unwrap_err();
    assert!(matches!(err, PathError::ArcCarrierSpelling { .. }));
    assert!(
        err.to_string().contains(".at_on(p, centre, winding)"),
        "the refusal must name the door that binds an arc arrival: {err}"
    );
}

/// **G2**: `SeamFilletOntoArc` is RETIRED into the same spelling
/// refusal. `.to(Start)` still needs a straight first side, because it
/// RETRIMS the entry vertex (LB5: that vertex is authored topology);
/// the case that wants an arc carrier at the seam has its own door,
/// `.to_on(Start, centre, winding)`, which keeps the vertex.
#[test]
fn a_seam_fillet_onto_an_arc_first_side_names_the_closing_door() {
    // Side 1 is an arc leg — retrimming the entry would slide it off
    // its own carrier.
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
    let err = arrival.to(Start).unwrap_err();
    assert!(matches!(err, PathError::ArcCarrierSpelling { .. }));
    assert!(
        err.to_string().contains(".to_on(Start, centre, winding)"),
        "the refusal must name the close that KEEPS the entry vertex: {err}"
    );
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

// ------------------------------------------------------------------
// Sign-domain gates (review MAJOR-1 / MINOR-1, PR #233): the reviewer's
// R6/R7 attack shapes, adopted verbatim — every row that once built a
// lying loop now refuses TYPED at the offending verb.
// ------------------------------------------------------------------

/// MAJOR-1 regression (reviewer probe R6, verbatim shape): a negative
/// continuation length after a fillet used to build a loop that PASSED
/// the verify layer with the authored anchor ~0.5 m off the final path
/// (§4 item 3 broken silently). The length now classifies through the
/// funnel and refuses at `line(-0.5)` itself.
#[test]
fn negative_leg_length_refuses_typed_r6() {
    let north = std::f64::consts::FRAC_PI_2;
    let refused = Open
        .at(p2(0.0, -1.0))
        .angle(0.0)
        .unwrap()
        .fillet(0.25)
        .unwrap()
        .at(p2(1.0, 0.0))
        .unwrap()
        .angle(north)
        .unwrap()
        .line(-0.5);
    assert!(matches!(refused, Err(PathError::NonpositiveLeg { .. })));
}

/// MINOR-1 regression: a zero-length leg is a degenerate segment —
/// refused at authoring, not left for the verify layer.
#[test]
fn zero_leg_length_refuses_typed() {
    let leg = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0))
        .unwrap()
        .angle(std::f64::consts::FRAC_PI_2)
        .unwrap();
    assert!(matches!(
        leg.line(0.0),
        Err(PathError::NonpositiveLeg { .. })
    ));
}

/// MINOR-1 regression (reviewer probe R7, verbatim shape): r = 0 and
/// r < 0 used to author successfully and be caught only downstream;
/// the radius now classifies through the funnel at `.fillet(r)`.
#[test]
fn nonpositive_fillet_radius_refuses_typed_r7() {
    for r in [-0.5, 0.0] {
        let refused = Open.at(p2(0.0, 0.0)).angle(0.0).unwrap().fillet(r);
        assert!(
            matches!(refused, Err(PathError::NonpositiveFilletRadius { .. })),
            "r = {r} must refuse at the fillet verb"
        );
    }
}

// ------------------------------------------------------------------
// LIB-G1 vocabulary growth: sign gates, refusal rows, and the two
// invariants the new constructors have to keep.
// ------------------------------------------------------------------

/// G1-1 — a circle validates as a one-step complete loop, and its
/// radius classifies through the funnel like every other sign gate.
#[test]
fn circle_validates_and_refuses_nonpositive_radius() {
    let c = profile::circle(p2(1.0, 2.0), 0.75).unwrap();
    validate_ok(&c);
    assert_eq!(c.vertices.len(), 2);
    assert!(
        c.tangent_joints.is_empty(),
        "a circle's two joints are same-carrier identities, not declared tangencies"
    );
    for r in [-1.0, 0.0] {
        assert!(
            matches!(
                profile::circle(p2(0.0, 0.0), r),
                Err(PathError::NonpositiveCircleRadius { .. })
            ),
            "r = {r} must refuse at the circle primitive"
        );
    }
}

/// G1-1, the PQ4 pin: the circle primitive authors NO seam, so it
/// changes nothing about what a CHAIN may do. A chain that tries to
/// close on the carrier it is already riding still refuses — the
/// mid-carrier seam decision (PATHS-DESIGN §6) is untouched.
#[test]
fn circle_primitive_leaves_pq4_refusing_for_chains() {
    // A tangent-arc close onto the same carrier the tip arrived on is
    // the closed-carrier split said as a chain: carrier identity, and
    // still refused.
    let refused = Open
        .at(p2(1.0, 0.0))
        .arc_to(p2(-1.0, 0.0), 1.0)
        .unwrap()
        .tangent()
        .tangent_arc_to(Start);
    assert!(
        matches!(refused, Err(PathError::SameCarrierJunction { .. })),
        "a chain closing on its own carrier still refuses: {refused:?}"
    );
}

/// G1-2 — the collinear class refuses as ONE refusal, whether the
/// through-point sits on the chord, beyond its far end, or on an
/// endpoint (all three are "on the chord line").
#[test]
fn arc_via_refuses_the_whole_collinear_class() {
    let (a, b) = (p2(0.0, 0.0), p2(2.0, 0.0));
    for via in [p2(1.0, 0.0), p2(3.0, 0.0), p2(-1.0, 0.0), p2(0.0, 0.0)] {
        let refused = Open.at(a).arc_via(via, b);
        assert!(
            matches!(refused, Err(PathError::ArcViaCollinear { .. })),
            "via {via:?} on the chord line must refuse"
        );
    }
}

/// G1-2/G1-3 — a leg spans a chord: coincident endpoints refuse in both
/// new arc modes (a closed carrier is the circle primitive's business).
#[test]
fn arc_modes_refuse_a_degenerate_chord() {
    let a = p2(1.0, 0.0);
    assert!(matches!(
        Open.at(a).arc_via(p2(0.0, 1.0), a),
        Err(PathError::DegenerateArcChord { .. })
    ));
    assert!(matches!(
        Open.at(a)
            .arc_center(p2(0.0, 0.0), a, profile::ArcSweep::Ccw),
        Err(PathError::DegenerateArcChord { .. })
    ));
}

/// G1-3 — equidistance is CHECKED, and a definite mismatch refuses
/// typed. Nothing is re-projected: the refusal reports both radii and
/// leaves all three authored points where the author put them.
#[test]
fn arc_center_refuses_a_definite_equidistance_mismatch() {
    let refused =
        Open.at(p2(1.0, 0.0))
            .arc_center(p2(0.0, 0.0), p2(0.0, 2.0), profile::ArcSweep::Ccw);
    match refused {
        Err(PathError::ArcCenterNotEquidistant {
            tip_radius,
            end_radius,
        }) => {
            assert!((tip_radius - 1.0).abs() < 1e-12);
            assert!((end_radius - 2.0).abs() < 1e-12);
        }
        other => panic!("expected an equidistance refusal, got {other:?}"),
    }
    // A centre on an endpoint has no radius for the winding to select.
    assert!(matches!(
        Open.at(p2(1.0, 0.0))
            .arc_center(p2(1.0, 0.0), p2(0.0, 1.0), profile::ArcSweep::Ccw),
        Err(PathError::DegenerateArcCenter { .. })
    ));
}

/// G1-3 — an exactly-equidistant centre proceeds, and the authored
/// endpoints land on the path verbatim (the §4 item 3 invariant, on the
/// mode that takes a point which is NOT on the path).
#[test]
fn arc_center_stores_its_authored_endpoints_verbatim() {
    let (a, c, b) = (p2(3.0, 0.0), p2(0.0, 0.0), p2(0.0, 3.0));
    let lowered = Open
        .at(a)
        .arc_center(c, b, profile::ArcSweep::Ccw)
        .unwrap()
        .line_to(c)
        .unwrap()
        .line_to(Start)
        .unwrap();
    validate_ok(&lowered);
    assert_eq!(lowered.vertices[0].pos.x.to_bits(), a.x.to_bits());
    assert_eq!(lowered.vertices[1].pos.x.to_bits(), b.x.to_bits());
    assert_eq!(lowered.vertices[1].pos.y.to_bits(), b.y.to_bits());
}

/// G1-5 — a director spelled as components must name a direction.
#[test]
fn toward_refuses_a_zero_direction() {
    assert!(matches!(
        Open.toward(0.0, 0.0_f64),
        Err(PathError::ZeroDirection { .. })
    ));
    assert!(matches!(
        Open.at(p2(0.0, 0.0)).toward(0.0, 0.0_f64),
        Err(PathError::ZeroDirection { .. })
    ));
}

/// G1-5 — `toward` binds the SAME slot as `angle`, so it runs the same
/// §4 item 1 junction check on a directed point: a components-spelled
/// tangent continuation refuses exactly as an angle-spelled one does.
#[test]
fn toward_runs_the_same_junction_check_as_angle() {
    let leg = Open.at(p2(0.0, 0.0)).line_to(p2(2.0, 0.0)).unwrap();
    assert!(matches!(
        leg.clone().toward(1.0, 0.0),
        Err(PathError::JunctionTangent { .. })
    ));
    assert!(matches!(
        leg.toward(-1.0, 0.0),
        Err(PathError::JunctionCusp { .. })
    ));
}

/// G1-4 — the far-end anchor ends the arrival side AT its authored
/// point, which is therefore a vertex, bit-for-bit (§4 item 3), and
/// needs neither a synthetic mid-side anchor nor a measured length.
#[test]
fn far_end_anchor_makes_its_authored_point_a_vertex() {
    let far = p2(1.0, 3.0);
    let lowered = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .unwrap()
        .line_to(p2(3.0, 1.0))
        .unwrap()
        .toward(-1.0, 0.0)
        .unwrap()
        .fillet(0.5)
        .unwrap()
        .toward(0.0, 1.0)
        .unwrap()
        .to(far)
        .unwrap()
        .line_to(p2(0.0, 3.0))
        .unwrap()
        .line_to(Start)
        .unwrap();
    validate_ok(&lowered);
    assert!(
        lowered
            .vertices
            .iter()
            .any(|v| v.pos.x.to_bits() == far.x.to_bits() && v.pos.y.to_bits() == far.y.to_bits()),
        "the authored far vertex must be on the path verbatim"
    );
}

/// G1-4 — at the ENTRY there is no arrival side to end, so the far-end
/// form refuses typed rather than silently meaning something else
/// (PATHS-DESIGN §2's entry rule).
#[test]
fn far_end_anchor_refuses_at_the_entry() {
    assert!(matches!(
        Open.angle(0.0_f64).to(p2(1.0, 0.0)),
        Err(PathError::FarEndAnchorWithoutFillet)
    ));
    assert!(matches!(
        Open.toward(1.0, 0.0_f64).unwrap().to(p2(1.0, 0.0)),
        Err(PathError::FarEndAnchorWithoutFillet)
    ));
}

// ------------------------------------------------------------------
// LIB-G1 fix pass: the far-end anchor's EXACT-FIT branch (both
// reviewers' MAJOR-1). When the fillet trim reaches the authored
// anchor exactly, the side IS the arc — so the arc's outgoing joint
// must be a FREE junction, not a declared tangency.
// ------------------------------------------------------------------

/// A right-angle corner at (1, 1) with r = 0.5: the setback is exactly
/// r, so an anchor at (1, 1.5) is the outgoing tangent point and the
/// fit classifies Zero. Returns the path sitting on that exact fit.
fn exact_fit_far_end() -> PartialPath<f64, HasPos<WithIncoming>, profile::path::NoAng> {
    Open.at(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .unwrap()
        .line_to(p2(3.0, 1.0))
        .unwrap()
        .toward(-1.0, 0.0)
        .unwrap()
        .fillet(0.5)
        .unwrap()
        .toward(0.0, 1.0)
        .unwrap()
        .to(p2(1.0, 1.5))
        .unwrap()
}

/// A SHARP continuation off an exact-fit far-end side validates. Before
/// the fix the arc's outgoing joint carried an inherited declaration,
/// and this legitimate authoring was refused `TangencyContradicted` by
/// the verify layer — a declaration nobody constructed (§4 item 2).
#[test]
fn exact_fit_far_end_allows_a_sharp_continuation() {
    let lowered = exact_fit_far_end()
        .line_to(p2(0.0, 3.0))
        .unwrap()
        .line_to(Start)
        .unwrap();
    validate_ok(&lowered);
}

/// … and a TANGENT continuation off the same tip still declares exactly
/// once and still validates: suppressing the inherited flag removed a
/// false claim, it did not remove the real construction.
#[test]
fn exact_fit_far_end_allows_a_tangent_continuation() {
    let lowered = exact_fit_far_end()
        .tangent()
        .line(0.75)
        .unwrap()
        .line_to(p2(0.0, 3.0))
        .unwrap()
        .line_to(Start)
        .unwrap();
    validate_ok(&lowered);
}

/// The exact-fit branch's vertex rule, pinned (both reviewers' MINOR-1):
/// the side's last vertex is the fillet's tangent point, and the
/// authored anchor is ABSORBED into it — coincident to within the fit
/// gate's own classification, not emitted as a second vertex. The
/// POSITIVE-fit branch, by contrast, emits the anchor verbatim.
#[test]
fn exact_fit_far_end_absorbs_its_anchor_into_the_tangent_point() {
    let anchor = p2(1.0, 1.5);
    let lowered = exact_fit_far_end()
        .line_to(p2(0.0, 3.0))
        .unwrap()
        .line_to(Start)
        .unwrap();
    validate_ok(&lowered);
    let nearest = lowered
        .vertices
        .iter()
        .map(|v| (v.pos - anchor).norm_squared().sqrt())
        .fold(f64::INFINITY, f64::min);
    assert!(
        nearest <= Tolerance::get().eps,
        "the authored anchor must coincide with an emitted vertex (got {nearest} m)"
    );
    // No zero-length segment was minted for the absorbed anchor.
    for w in lowered.vertices.windows(2) {
        let d = (w[1].pos - w[0].pos).norm_squared().sqrt();
        assert!(d > Tolerance::get().eps, "degenerate segment of {d} m");
    }
}

/// LIB-G1 fix pass (R2 NOTE-2): the new funnel gates' IN-BAND rows. A
/// margin between ε_input and K·ε_input is undecidable at this scalar,
/// and every one of the three new gates must ESCALATE typed rather than
/// guess — the reified-predicate contract, on the paths that previously
/// had only decided-Yes and decided-No coverage.
#[test]
fn the_new_funnel_gates_escalate_in_band() {
    let tol = Tolerance::get();
    // Squarely inside (ε, K·ε): undecidable at this scalar.
    let band = tol.eps * ((1.0 + tol.k) / 2.0);

    // toward: a norm in the band names no decidable direction.
    assert!(
        matches!(Open.toward(band, 0.0), Err(PathError::Escalated { .. })),
        "sub-band director norm must escalate"
    );

    // arc_center: an equidistance mismatch in the band.
    let refused =
        Open.at(p2(1.0, 0.0))
            .arc_center(p2(0.0, 0.0), p2(0.0, 1.0 + band), profile::ArcSweep::Ccw);
    assert!(
        matches!(refused, Err(PathError::Escalated { .. })),
        "in-band equidistance mismatch must escalate, got {refused:?}"
    );

    // arc_via: a through-point in the band off the chord line.
    let refused = Open.at(p2(0.0, 0.0)).arc_via(p2(1.0, band), p2(2.0, 0.0));
    assert!(
        matches!(refused, Err(PathError::Escalated { .. })),
        "in-band via offset must escalate, got {refused:?}"
    );
}

/// The decided sides of the same three gates, for contrast: a margin
/// comfortably ABOVE K·ε proceeds, one comfortably below ε refuses with
/// the gate's own typed error (never an escalation).
#[test]
fn the_new_funnel_gates_decide_outside_the_band() {
    let tol = Tolerance::get();
    let big = tol.eps * tol.k * 1000.0;
    let tiny = tol.eps / 1000.0;

    assert!(Open.toward(big, 0.0_f64).is_ok());
    assert!(matches!(
        Open.toward(tiny, 0.0_f64),
        Err(PathError::ZeroDirection { .. })
    ));

    assert!(matches!(
        Open.at(p2(1.0, 0.0))
            .arc_center(p2(0.0, 0.0), p2(0.0, 1.0 + big), profile::ArcSweep::Ccw),
        Err(PathError::ArcCenterNotEquidistant { .. })
    ));
    assert!(
        Open.at(p2(1.0, 0.0))
            .arc_center(p2(0.0, 0.0), p2(0.0, 1.0 + tiny), profile::ArcSweep::Ccw)
            .is_ok(),
        "a sub-epsilon radius difference is definitely equidistant"
    );

    assert!(
        Open.at(p2(0.0, 0.0))
            .arc_via(p2(1.0, big), p2(2.0, 0.0))
            .is_ok()
    );
    assert!(matches!(
        Open.at(p2(0.0, 0.0)).arc_via(p2(1.0, tiny), p2(2.0, 0.0)),
        Err(PathError::ArcViaCollinear { .. })
    ));
}
