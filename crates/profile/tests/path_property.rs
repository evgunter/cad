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

mod common;

use common::pinned;
use geom_core::Point2;
use profile::path::{HasAng, HasPos, WithIncoming};
use profile::{Open, PartialPath, PathError, Profile, ProfileLoop, SketchPlane, Start};
use proptest::prelude::*;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// One home for "this lowered loop passes the data gate" — the suite
/// had grown a second copy of it (`validate_lp`), which is now this.
fn validate_ok(l: &ProfileLoop<f64>) {
    Profile::new(SketchPlane::xy(), vec![l.clone()])
        .validate(Tol::witness())
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

/// Per-vertex radii vary within `base * [1 - RADIUS_JITTER, 1 +
/// RADIUS_JITTER]`. The bound is not free: see `convex_polygon`.
const RADIUS_JITTER: f64 = 0.08;

/// A STRICTLY CONVEX polygon's vertices, at increasing angles about the
/// origin with per-vertex radii. Both halves of that are guaranteed by
/// construction, and both are what `sharp_polygons_differential_and_
/// verified` needs of its input: a simple closed loop whose every
/// junction turns, so no `line_to` meets a cusp or a straight
/// continuation and the junction verifier has nothing to refuse.
///
/// ANGLES. `n` weights drawn from `[1, 1.5]` are normalised to sum to
/// `TAU`, so each angular gap lies in `[TAU / ((n-1) * 1.5 + w), TAU * w
/// / ((n-1) + w)]` for its own `w` — over `n` in `3..8` that is contained
/// in `[TAU/10, TAU * 1.5/3.5]`, hence strictly inside `(0, PI)`. Gaps
/// under `PI` are the load-bearing part: they put the origin strictly
/// inside the fan, so consecutive edges live in disjoint angular wedges
/// and increasing-angle order implies a SIMPLE loop. Drawing gaps freely
/// and rescaling them to a full turn does NOT give this — the rescale
/// multiplies gaps, and one gap past `PI` puts the origin outside, where
/// increasing angles imply nothing.
///
/// RADII. Left-turning at a vertex with neighbouring gaps `g1, g2` and
/// radii `a, b, c` is `sin(g2)/a + sin(g1)/c > sin(g1+g2)/b`. Over
/// `g1, g2 >= g_min` the right side is worst at `g1 = g2 = g_min`, and
/// with radii confined to `base * [1-J, 1+J]` the worst assignment is
/// `a = c = base(1+J)`, `b = base(1-J)`; the condition then reduces to
/// `(1-J)/(1+J) > cos(g_min)`, i.e. `J < tan^2(g_min/2)` — with
/// `g_min = TAU/10`, `J < tan^2(TAU/20)`. That is the bound
/// `RADIUS_JITTER` is chosen inside, and it is the whole reason the
/// radii are a jitter about one base rather than free in `[1, 3]`: free
/// radii let a middle vertex land on the chord of its neighbours, which
/// is a straight junction, not a sharp one.
fn convex_polygon() -> impl Strategy<Value = Vec<Point2<f64>>> {
    (3usize..8)
        .prop_flat_map(|n| {
            (
                proptest::collection::vec(1.0f64..1.5, n),
                1.0f64..3.0,
                proptest::collection::vec(-RADIUS_JITTER..RADIUS_JITTER, n),
            )
        })
        .prop_map(|(weights, base, jitter)| {
            let total: f64 = weights.iter().sum();
            let mut phi = 0.0;
            weights
                .iter()
                .zip(&jitter)
                .map(|(w, j)| {
                    phi += std::f64::consts::TAU * w / total;
                    let r = base * (1.0 + j);
                    p2(r * phi.cos(), r * phi.sin())
                })
                .collect()
        })
}

/// Twice the signed area of `(a, b, c)` — positive exactly when the
/// junction at `b` turns left.
fn turn(a: Point2<f64>, b: Point2<f64>, c: Point2<f64>) -> f64 {
    (b.x - a.x) * (c.y - b.y) - (b.y - a.y) * (c.x - b.x)
}

/// ANTI-VACUITY for the convexity contract asserted on every draw of
/// `sharp_polygons_differential_and_verified`: a witness that predicate
/// must reject, written down rather than searched for. These four points
/// are at increasing angles about the origin with one angular gap past
/// `PI`, which is the shape a gaps-then-rescale strategy emits and the
/// shape whose edges 1 and 3 cross. The predicate has to see it, or it is
/// guarding nothing.
///
/// PINNED WRITTEN OUT, deliberately not as a committed
/// `.proptest-regressions` seed: a `cc` seed re-derives its input only
/// through the strategy that drew it, and the strategy that drew this
/// shape is exactly what the rewrite replaced — a seed file here would
/// re-run some other input in silence, where these coordinates cannot
/// drift. (Regression files proptest itself writes against a LIVE
/// strategy stay committed, as elsewhere in this directory; a witness
/// you can write down is a fixture, not a seed.)
#[test]
fn the_convexity_contract_rejects_an_increasing_angle_loop_that_crosses() {
    let pts = [
        p2(1.477_867_658_492_595, 1.132_126_545_585_816_3),
        p2(0.675_863_001_545_653_5, 2.506_274_403_424_949),
        p2(2.269_885_464_407_144_7, -1.738_855_015_147_889_9),
        p2(1.0, -2.449_293_598_294_706_4e-16),
    ];
    let n = pts.len();
    let turns: Vec<f64> = (0..n)
        .map(|k| turn(pts[k], pts[(k + 1) % n], pts[(k + 2) % n]))
        .collect();
    assert!(
        turns.iter().any(|t| *t <= 0.0),
        "the convexity contract accepted a self-crossing loop: turns {turns:?}"
    );
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// P1 — sharp polygons: the algebra's `line_to` chain lowers to
    /// the authored table verbatim (every coordinate is authored, so
    /// identity is exact everywhere), every authored point is a vertex
    /// in order, and the lowered loop validates — the junction verifier
    /// never fires on it.
    #[test]
    fn sharp_polygons_differential_and_verified(pts in convex_polygon()) {
        // The strategy's own contract, asserted against the INPUT rather
        // than assumed. A left turn at every vertex is what makes the
        // loop simple and every junction sharp, and it is exactly the
        // precondition `validate_ok` below is entitled to; a generator
        // that stops supplying it turns this row into a test of
        // `validate` against inputs it is right to refuse.
        let n = pts.len();
        for k in 0..n {
            let t = turn(pts[k], pts[(k + 1) % n], pts[(k + 2) % n]);
            prop_assert!(
                t > 0.0,
                "convex_polygon() is contracted to be strictly convex, but the \
                 junction at vertex {} turns by {t} (<= 0): {pts:?}",
                (k + 1) % n
            );
        }

        let mut path = Open.at(pts[0]).line_to(pts[1], Tol::witness()).unwrap();
        for q in &pts[2..] {
            path = path.line_to(*q, Tol::witness()).unwrap();
        }
        let algebra = path.line_to(Start, Tol::witness()).unwrap();
        let algebra = pinned(algebra);
        // The property SAID DIRECTLY (LIB-RETTAIL): a sharp chain's
        // lowering is the authored table verbatim — one vertex per
        // authored point, in order, bit-for-bit, every bulge +0.0, no
        // declared joints. This used to be stated as "identical to the
        // LoopBuilder chain"; against a random input a recorded fixture
        // is impossible, and the twin was only ever a second way of
        // saying this. Said against the INPUT it is strictly stronger:
        // a lowering that dropped, duplicated or reordered a point now
        // fails even if a twin would have made the same mistake.
        prop_assert_eq!(algebra.vertices().len(), pts.len());
        prop_assert!(algebra.tangent_joints().is_empty());
        for (k, q) in pts.iter().enumerate() {
            prop_assert_eq!(algebra.vertices()[k].pos().x.to_bits(), q.x.to_bits());
            prop_assert_eq!(algebra.vertices()[k].pos().y.to_bits(), q.y.to_bits());
            prop_assert_eq!(algebra.vertices()[k].bulge().to_bits(), 0.0f64.to_bits());
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
            .angle(0.0, Tol::witness()).unwrap()
            .fillet(r, Tol::witness()).unwrap()
            .at(anchor, Tol::witness()).unwrap()
            .angle(north, Tol::witness()).unwrap()
            .line(top_len, Tol::witness()).unwrap()
            .line_to(p2(0.0, h), Tol::witness()).unwrap()
            .line_to(Start, Tol::witness()).unwrap();
        let algebra = pinned(algebra);
        validate_ok(&algebra);
        // The anchor lies on the trimmed arrival side: the segment
        // from the fillet arc's end (vertex 2) to the side's end
        // (vertex 3).
        let v = &algebra.vertices();
        prop_assert_eq!(v.len(), 5);
        let d = seg_distance(anchor, v[2].pos(), v[3].pos());
        prop_assert!(d < 1e-9, "anchor off its side by {d:e}");
        // Authored entry/targets are vertices, bit-for-bit.
        prop_assert_eq!(v[0].pos().x.to_bits(), 0.0f64.to_bits());
        prop_assert_eq!(v[4].pos().x.to_bits(), 0.0f64.to_bits());
        prop_assert_eq!(v[4].pos().y.to_bits(), h.to_bits());
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
        let leg = Open.at(p2(0.0, 0.0)).line_to(p2(2.0, 0.0), Tol::witness()).unwrap();
        let dep = if flip { std::f64::consts::PI + dtheta } else { dtheta };
        let tol = Tol::witness().get();
        // margin = sin(dtheta)·arm, arm = 2 (the leg length).
        let margin = (dtheta.sin() * 2.0).abs();
        match leg.angle(dep, Tol::witness()) {
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
        .line_to(p2(2.0, 0.0), Tol::witness())
        .unwrap()
        .angle(std::f64::consts::FRAC_PI_2, Tol::witness())
        .unwrap()
}

#[test]
fn turn_zero_refuses_toward_tangent() {
    let leg = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0), Tol::witness())
        .unwrap();
    assert!(matches!(
        leg.turn(0.0, Tol::witness()),
        Err(PathError::JunctionTangent { .. })
    ));
}

/// The reverse class still refuses when it is AUTHORED rather than
/// declared — a value within ε of the reverse is a coincidence, and
/// the ladder never reads intent off a margin — but the refusal now
/// names the door: `.cusp()`, which reverses the incoming ray exactly
/// and emits the declaration the kernel's wedge arm asks for.
#[test]
fn turn_pi_refuses_as_cusp_naming_the_declaration_door() {
    let leg = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0), Tol::witness())
        .unwrap();
    let err = leg.turn(std::f64::consts::PI, Tol::witness()).unwrap_err();
    assert!(matches!(err, PathError::JunctionCusp { .. }));
    // The message names the verb, and no longer says the door is
    // absent: a caller who means the cusp has somewhere to go.
    let text = err.to_string();
    assert!(
        text.contains(".cusp()"),
        "cusp refusal must name the verb: {text}"
    );
    assert!(
        !text.contains("no declaration door"),
        "the door exists now: {text}"
    );
    // Declaring it is a different spelling, not a looser tolerance:
    // the same junction authored through the verb is exact.
    let declared = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0), Tol::witness())
        .unwrap()
        .cusp()
        .tangent_arc_to(p2(1.0, 1.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    assert_eq!(
        declared.loop_.tangent_joints(),
        &[1],
        "the cusp joint is DECLARED, like a tangent one"
    );
}

#[test]
fn declared_straight_continuation_of_a_line_is_same_carrier() {
    let leg = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0), Tol::witness())
        .unwrap();
    assert!(matches!(
        leg.tangent().line(1.0, Tol::witness()),
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
        .arc_to(
            Bulge {
                p: p2(1.0, 0.0),
                b: 1.0,
            },
            Tol::witness(),
        )
        .unwrap()
        .tangent();
    assert!(matches!(
        arc_end.tangent_arc_to(p2(0.0, 1.0), Tol::witness()),
        Err(PathError::SameCarrierJunction { .. })
    ));
}

#[test]
fn parallel_carriers_refuse_no_corner() {
    // Arrival side parallel to the departure ray.
    let arrival = bent_tip()
        .fillet(0.5, Tol::witness())
        .unwrap()
        .at(p2(4.0, 3.0), Tol::witness())
        .unwrap();
    assert!(matches!(
        arrival.angle(std::f64::consts::FRAC_PI_2, Tol::witness()),
        Err(PathError::NoCornerForFillet { .. })
    ));
}

#[test]
fn corner_behind_ray_refuses_no_corner() {
    // Ray heads north from (2, 0); the arrival carrier crosses it at
    // y = −1 — behind the ray start.
    let arrival = bent_tip()
        .fillet(0.5, Tol::witness())
        .unwrap()
        .at(p2(4.0, -1.0), Tol::witness())
        .unwrap();
    assert!(matches!(
        arrival.angle(0.0, Tol::witness()),
        Err(PathError::NoCornerForFillet { .. })
    ));
}

#[test]
fn trim_eating_an_anchor_refuses_typed() {
    // Corner (2, 2); the arrival anchor sits 0.5 past it but the
    // radius wants a 0.9 setback: the trim would eat the anchor.
    let arrival = bent_tip()
        .fillet(0.9, Tol::witness())
        .unwrap()
        .at(p2(2.5, 2.0), Tol::witness())
        .unwrap();
    assert!(matches!(
        arrival.angle(0.0, Tol::witness()),
        Err(PathError::AnchorOutsideTrimmedExtent { .. })
    ));
}

/// **§2c**: an arc ARRIVAL is authored with the fillet that trims it,
/// so a sharp arc LEG off an arrival anchor while a fillet is still open
/// is refused, and the refusal names the fused verbs that do the job.
#[test]
fn an_arc_leg_on_an_open_fillet_names_the_fused_verbs() {
    let arrival = bent_tip()
        .fillet(0.5, Tol::witness())
        .unwrap()
        .at(p2(4.0, 2.0), Tol::witness())
        .unwrap();
    let err = arrival
        .arc_to(
            Bulge {
                p: p2(5.0, 3.0),
                b: 0.4,
            },
            Tol::witness(),
        )
        .unwrap_err();
    assert!(matches!(err, PathError::ArcLegOnOpenFillet { .. }));
    assert!(
        err.to_string().contains("fillet_arc(r, spec)"),
        "the refusal must name the verbs that author an arc arrival: {err}"
    );
}

/// **§2c**: `.to(Start)` still needs a straight first side, because it
/// RETRIMS the entry vertex (LB5: that vertex is authored topology);
/// the case that wants an arc carrier at the seam is the arc-arrival
/// close, `fillet_arc(r, Center { c, winding, p: Start })`, which keeps
/// the vertex. The refusal is its own variant now — under the axiom no
/// carrier-keyed spelling refusal exists, and this one is about the
/// SEAM's retrim, not about any carrier the state secretly knew.
#[test]
fn a_seam_fillet_onto_an_arc_first_side_names_the_closing_door() {
    // Side 1 is an arc leg — retrimming the entry would slide it off
    // its own carrier.
    let tip = Open
        .at(p2(0.0, 0.0))
        .arc_to(
            Bulge {
                p: p2(4.0, 0.0),
                b: 0.3,
            },
            Tol::witness(),
        )
        .unwrap()
        .angle(2.0, Tol::witness())
        .unwrap()
        .line(2.0, Tol::witness())
        .unwrap()
        .angle(3.5, Tol::witness())
        .unwrap();
    let arrival = tip.fillet(0.3, Tol::witness()).unwrap();
    let err = arrival.to(Start, Tol::witness()).unwrap_err();
    assert!(matches!(err, PathError::SeamRetrimsArcFirstSide));
    assert!(
        err.to_string().contains("Center { c, winding, p: Start }"),
        "the refusal must name the close that KEEPS the entry vertex: {err}"
    );
}

#[test]
fn the_seam_tangent_close_refuses_always() {
    // The seam junction of a straight closer within the tangent band:
    // the closing line arrives at Start along the entry departure —
    // the PQ4 mid-side seam, refused with the two structural
    // spellings named.
    let refused = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0), Tol::witness())
        .unwrap()
        .line_to(p2(2.0, 2.0), Tol::witness())
        .unwrap()
        .line_to(p2(-2.0, 2.0), Tol::witness())
        .unwrap()
        .line_to(p2(-2.0, 0.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness());
    assert!(matches!(refused, Err(PathError::SeamTangent { .. })));
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
        .line_to(p2(3.0, 0.0), Tol::witness())
        .unwrap()
        .tangent()
        .tangent_arc_to(p2(4.0, 1.0), Tol::witness())
        .unwrap()
        .line_to(p2(0.5, 2.0), Tol::witness())
        .unwrap()
        .tangent()
        .tangent_arc_to(Start, Tol::witness())
        .unwrap();
    let loop_ = pinned(loop_);
    validate_ok(&loop_);
    // The two `.tangent()` joints are declared; the junctions at
    // (4, 1) and at Start are definitely sharp; the verifier confirms
    // every flag.
    assert_eq!(loop_.tangent_joints().len(), 2);
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
        .angle(0.0, Tol::witness())
        .unwrap()
        .fillet(0.25, Tol::witness())
        .unwrap()
        .at(p2(1.0, 0.0), Tol::witness())
        .unwrap()
        .angle(north, Tol::witness())
        .unwrap()
        .line(-0.5, Tol::witness());
    assert!(matches!(refused, Err(PathError::NonpositiveLeg { .. })));
}

/// MINOR-1 regression: a zero-length leg is a degenerate segment —
/// refused at authoring, not left for the verify layer.
#[test]
fn zero_leg_length_refuses_typed() {
    let leg = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0), Tol::witness())
        .unwrap()
        .angle(std::f64::consts::FRAC_PI_2, Tol::witness())
        .unwrap();
    assert!(matches!(
        leg.line(0.0, Tol::witness()),
        Err(PathError::NonpositiveLeg { .. })
    ));
}

/// MINOR-1 regression (reviewer probe R7, verbatim shape): r = 0 and
/// r < 0 used to author successfully and be caught only downstream;
/// the radius now classifies through the funnel at `.fillet(r)`.
#[test]
fn nonpositive_fillet_radius_refuses_typed_r7() {
    for r in [-0.5, 0.0] {
        let refused = Open
            .at(p2(0.0, 0.0))
            .angle(0.0, Tol::witness())
            .unwrap()
            .fillet(r, Tol::witness());
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
    let c = profile::circle(p2(1.0, 2.0), 0.75, Tol::witness()).unwrap();
    let c = pinned(c);
    validate_ok(&c);
    assert_eq!(c.vertices().len(), 2);
    assert!(
        c.tangent_joints().is_empty(),
        "a circle's two joints are same-carrier identities, not declared tangencies"
    );
    for r in [-1.0, 0.0] {
        assert!(
            matches!(
                profile::circle(p2(0.0, 0.0), r, Tol::witness()),
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
        .arc_to(
            Bulge {
                p: p2(-1.0, 0.0),
                b: 1.0,
            },
            Tol::witness(),
        )
        .unwrap()
        .tangent()
        .tangent_arc_to(Start, Tol::witness());
    assert!(
        matches!(refused, Err(PathError::SameCarrierJunction { .. })),
        "a chain closing on its own carrier still refuses: {refused:?}"
    );
}

/// G1-2 — the collinear class refuses as ONE refusal, whether the
/// through-point sits on the chord, beyond its far end, or on an
/// endpoint (all three are "on the chord line").
#[test]
fn the_via_mode_refuses_the_whole_collinear_class() {
    let (a, b) = (p2(0.0, 0.0), p2(2.0, 0.0));
    for via in [p2(1.0, 0.0), p2(3.0, 0.0), p2(-1.0, 0.0), p2(0.0, 0.0)] {
        let refused = Open.at(a).arc_to(Via { q: via, p: b }, Tol::witness());
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
        Open.at(a).arc_to(
            Via {
                q: p2(0.0, 1.0),
                p: a
            },
            Tol::witness()
        ),
        Err(PathError::DegenerateArcChord { .. })
    ));
    assert!(matches!(
        Open.at(a).arc_to(
            Center {
                c: p2(0.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: a
            },
            Tol::witness()
        ),
        Err(PathError::DegenerateArcChord { .. })
    ));
}

/// G1-3 — equidistance is CHECKED, and a definite mismatch refuses
/// typed. Nothing is re-projected: the refusal reports both radii and
/// leaves all three authored points where the author put them.
#[test]
fn the_center_mode_refuses_a_definite_equidistance_mismatch() {
    let refused = Open.at(p2(1.0, 0.0)).arc_to(
        Center {
            c: p2(0.0, 0.0),
            winding: profile::ArcSweep::Ccw,
            p: p2(0.0, 2.0),
        },
        Tol::witness(),
    );
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
        Open.at(p2(1.0, 0.0)).arc_to(
            Center {
                c: p2(1.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: p2(0.0, 1.0),
            },
            Tol::witness()
        ),
        Err(PathError::DegenerateArcCenter { .. })
    ));
}

/// G1-3 — an exactly-equidistant centre proceeds, and the authored
/// endpoints land on the path verbatim (the §4 item 3 invariant, on the
/// mode that takes a point which is NOT on the path).
#[test]
fn the_center_mode_stores_its_authored_endpoints_verbatim() {
    let (a, c, b) = (p2(3.0, 0.0), p2(0.0, 0.0), p2(0.0, 3.0));
    let lowered = Open
        .at(a)
        .arc_to(
            Center {
                c,
                winding: profile::ArcSweep::Ccw,
                p: b,
            },
            Tol::witness(),
        )
        .unwrap()
        .line_to(c, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let lowered = pinned(lowered);
    validate_ok(&lowered);
    assert_eq!(lowered.vertices()[0].pos().x.to_bits(), a.x.to_bits());
    assert_eq!(lowered.vertices()[1].pos().x.to_bits(), b.x.to_bits());
    assert_eq!(lowered.vertices()[1].pos().y.to_bits(), b.y.to_bits());
}

/// G1-5 — a director spelled as components must name a direction.
#[test]
fn toward_refuses_a_zero_direction() {
    assert!(matches!(
        Open.toward(0.0, 0.0_f64, Tol::witness()),
        Err(PathError::ZeroDirection { .. })
    ));
    assert!(matches!(
        Open.at(p2(0.0, 0.0)).toward(0.0, 0.0_f64, Tol::witness()),
        Err(PathError::ZeroDirection { .. })
    ));
}

/// G1-5 — `toward` binds the SAME slot as `angle`, so it runs the same
/// §4 item 1 junction check on a directed point: a components-spelled
/// tangent continuation refuses exactly as an angle-spelled one does.
#[test]
fn toward_runs_the_same_junction_check_as_angle() {
    let leg = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0), Tol::witness())
        .unwrap();
    assert!(matches!(
        leg.clone().toward(1.0, 0.0, Tol::witness()),
        Err(PathError::JunctionTangent { .. })
    ));
    assert!(matches!(
        leg.toward(-1.0, 0.0, Tol::witness()),
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
        .line_to(p2(3.0, 0.0), Tol::witness())
        .unwrap()
        .line_to(p2(3.0, 1.0), Tol::witness())
        .unwrap()
        .toward(-1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet(0.5, Tol::witness())
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .to(far, Tol::witness())
        .unwrap()
        .line_to(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let lowered = pinned(lowered);
    validate_ok(&lowered);
    assert!(
        lowered
            .vertices()
            .iter()
            .any(|v| v.pos().x.to_bits() == far.x.to_bits()
                && v.pos().y.to_bits() == far.y.to_bits()),
        "the authored far vertex must be on the path verbatim"
    );
}

/// G1-4 — at the ENTRY there is no arrival side to end, so the far-end
/// form refuses typed rather than silently meaning something else
/// (PATHS-DESIGN §2's entry rule).
#[test]
fn far_end_anchor_refuses_at_the_entry() {
    assert!(matches!(
        Open.angle(0.0_f64).to(p2(1.0, 0.0), Tol::witness()),
        Err(PathError::FarEndAnchorWithoutFillet)
    ));
    assert!(matches!(
        Open.toward(1.0, 0.0_f64, Tol::witness())
            .unwrap()
            .to(p2(1.0, 0.0), Tol::witness()),
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
        .line_to(p2(3.0, 0.0), Tol::witness())
        .unwrap()
        .line_to(p2(3.0, 1.0), Tol::witness())
        .unwrap()
        .toward(-1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet(0.5, Tol::witness())
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .to(p2(1.0, 1.5), Tol::witness())
        .unwrap()
}

/// A SHARP continuation off an exact-fit far-end side validates. Before
/// the fix the arc's outgoing joint carried an inherited declaration,
/// and this legitimate authoring was refused `TangencyContradicted` by
/// the verify layer — a declaration nobody constructed (§4 item 2).
#[test]
fn exact_fit_far_end_allows_a_sharp_continuation() {
    let lowered = exact_fit_far_end()
        .line_to(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let lowered = pinned(lowered);
    validate_ok(&lowered);
}

/// … and a TANGENT continuation off the same tip still declares exactly
/// once and still validates: suppressing the inherited flag removed a
/// false claim, it did not remove the real construction.
#[test]
fn exact_fit_far_end_allows_a_tangent_continuation() {
    let lowered = exact_fit_far_end()
        .tangent()
        .line(0.75, Tol::witness())
        .unwrap()
        .line_to(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let lowered = pinned(lowered);
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
        .line_to(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();
    let lowered = pinned(lowered);
    validate_ok(&lowered);
    let nearest = lowered
        .vertices()
        .iter()
        .map(|v| (v.pos() - anchor).norm_squared().sqrt())
        .fold(f64::INFINITY, f64::min);
    assert!(
        nearest <= Tol::witness().get().eps,
        "the authored anchor must coincide with an emitted vertex (got {nearest} m)"
    );
    // No zero-length segment was minted for the absorbed anchor.
    for w in lowered.vertices().windows(2) {
        let d = (w[1].pos() - w[0].pos()).norm_squared().sqrt();
        assert!(d > Tol::witness().get().eps, "degenerate segment of {d} m");
    }
}

/// LIB-G1 fix pass (R2 NOTE-2): the new funnel gates' IN-BAND rows. A
/// margin between ε_input and K·ε_input is undecidable at this scalar,
/// and every one of the three new gates must ESCALATE typed rather than
/// guess — the reified-predicate contract, on the paths that previously
/// had only decided-Yes and decided-No coverage.
#[test]
fn the_new_funnel_gates_escalate_in_band() {
    let tol = Tol::witness().get();
    // Squarely inside (ε, K·ε): undecidable at this scalar.
    let band = tol.eps * ((1.0 + tol.k) / 2.0);

    // toward: a norm in the band names no decidable direction.
    assert!(
        matches!(
            Open.toward(band, 0.0, Tol::witness()),
            Err(PathError::Escalated { .. })
        ),
        "sub-band director norm must escalate"
    );

    // arc_to(Center { .. }): an equidistance mismatch in the band.
    let refused = Open.at(p2(1.0, 0.0)).arc_to(
        Center {
            c: p2(0.0, 0.0),
            winding: profile::ArcSweep::Ccw,
            p: p2(0.0, 1.0 + band),
        },
        Tol::witness(),
    );
    assert!(
        matches!(refused, Err(PathError::Escalated { .. })),
        "in-band equidistance mismatch must escalate, got {refused:?}"
    );

    // arc_to(Via { .. }): a through-point in the band off the chord line.
    let refused = Open.at(p2(0.0, 0.0)).arc_to(
        Via {
            q: p2(1.0, band),
            p: p2(2.0, 0.0),
        },
        Tol::witness(),
    );
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
    let tol = Tol::witness().get();
    let big = tol.eps * tol.k * 1000.0;
    let tiny = tol.eps / 1000.0;

    assert!(Open.toward(big, 0.0_f64, Tol::witness()).is_ok());
    assert!(matches!(
        Open.toward(tiny, 0.0_f64, Tol::witness()),
        Err(PathError::ZeroDirection { .. })
    ));

    assert!(matches!(
        Open.at(p2(1.0, 0.0)).arc_to(
            Center {
                c: p2(0.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                p: p2(0.0, 1.0 + big),
            },
            Tol::witness()
        ),
        Err(PathError::ArcCenterNotEquidistant { .. })
    ));
    assert!(
        Open.at(p2(1.0, 0.0))
            .arc_to(
                Center {
                    c: p2(0.0, 0.0),
                    winding: profile::ArcSweep::Ccw,
                    p: p2(0.0, 1.0 + tiny),
                },
                Tol::witness()
            )
            .is_ok(),
        "a sub-epsilon radius difference is definitely equidistant"
    );

    assert!(
        Open.at(p2(0.0, 0.0))
            .arc_to(
                Via {
                    q: p2(1.0, big),
                    p: p2(2.0, 0.0),
                },
                Tol::witness()
            )
            .is_ok()
    );
    assert!(matches!(
        Open.at(p2(0.0, 0.0)).arc_to(
            Via {
                q: p2(1.0, tiny),
                p: p2(2.0, 0.0),
            },
            Tol::witness()
        ),
        Err(PathError::ArcViaCollinear { .. })
    ));
}

// ------------------------------------------------------------------
// LIB-G2 §3: the arc-carrier fillet family.
// ------------------------------------------------------------------

/// A unit lens like the rocker eye's, parameterised by the fillet
/// radius: entry on the right lobe, one fillet, close on the left.
fn lens(r: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let tip = 0.75f64.sqrt();
    Open.arc_fillet_arc(
        Center {
            c: p2(-0.5, 0.0),
            winding: profile::ArcSweep::Ccw,
            p: p2(0.0, -tip),
        },
        r,
        Center {
            c: p2(0.5, 0.0),
            winding: profile::ArcSweep::Ccw,
            p: Start,
        },
        Tol::witness(),
    )
    .map(pinned)
}

/// The eye program lowers, validates, and puts every AUTHORED point on
/// the final path: the entry anchor is vertex 0, bit for bit, and the
/// two derived tangent points are the only other vertices — the corner
/// itself is never a vertex, because it is never authored.
#[test]
fn the_carrier_bound_lens_lowers_and_keeps_its_authored_point() {
    let tip = 0.75f64.sqrt();
    let lowered = lens(0.25).unwrap();
    assert_eq!(lowered.vertices().len(), 3, "entry + two tangent points");
    assert_eq!(lowered.vertices()[0].pos().x.to_bits(), 0.0f64.to_bits());
    assert_eq!(lowered.vertices()[0].pos().y.to_bits(), (-tip).to_bits());
    // No vertex sits at the DERIVED corner (0, +tip): it is filleted
    // away, and the algebra never had a chance to author it.
    for v in lowered.vertices() {
        assert!(
            (v.pos().y - tip).abs() > 1e-9 || v.pos().x.abs() > 1e-9,
            "the derived corner must not appear as a vertex: {:?}",
            v.pos()
        );
    }
    Profile::new(SketchPlane::xy(), vec![lowered])
        .validate(Tol::witness())
        .expect("the carrier-bound lens validates");
}

/// A radius far too large for the lens refuses typed. This row carries
/// TWO pins, both of which have been mutation-checked:
///
/// 1. the **enclosing class**: r = 5 against unit lobes puts both offset
///    radii at ρ = 1 − 5 = −4, so the requested circle would contain
///    each lobe whole and the corner with it. No fillet of this corner
///    exists at this radius and none ever will
///    (`docs/ENCLOSING-TANGENCY-DESIGN.md`), so the refusal is the
///    enclosing one and it names the lobe radius as the bound. The
///    §3c CARRIER-KIND payload this row used to pin — an arc side's
///    arc-length setback and ANGULAR margin — has its own home at
///    `arc_fillet::oversized_radius_on_an_arc_side_names_the_carrier_and_angular_margin`,
///    on an ordinary corner that actually overruns its anchor; this
///    lens never reached that gate honestly, because on this geometry
///    every radius past the waist (1/2) is either offset-disjoint or
///    enclosing;
/// 2. the boundary's **refusal precedence**: `resolve` keeps gate
///    refusals and construction refusals in two channels and lets the
///    CONSTRUCTION one win. Here the discarded root's advance gate also
///    refuses, so a single-channel "first refusal wins" reports
///    `NoCornerForFillet{BehindIncomingRay}` — a claim that a corner
///    which is in fact ahead of the anchor is behind it — and this
///    assertion fails. Do not weaken it to `matches!(.., PathError::_)`.
#[test]
fn an_oversized_carrier_fillet_refuses_as_the_enclosing_class() {
    let err = lens(5.0).unwrap_err();
    let PathError::FilletEnclosesLegCarrier {
        carrier_radius,
        offset_radius,
        radius,
        largest_tangent_radius,
        ..
    } = err
    else {
        panic!("expected the enclosing-class refusal, got {err:?}");
    };
    assert!(
        (carrier_radius - 1.0).abs() < 1e-12,
        "the lens's lobes are R = 1, got {carrier_radius}"
    );
    assert!(
        (offset_radius + 4.0).abs() < 1e-12,
        "rho = R - sigma*tau*r = 1 - 5 = -4, got {offset_radius}"
    );
    assert_eq!(radius, 5.0);
    // **The endorsed radius BUILDS.** The message names the largest
    // circle tangent to both lobes at this corner — (R1 + R2 - d)/2 with
    // unit lobes 1 m apart, so 1/2 — and a radius below it rounds the
    // lens. The lobe radius alone would have endorsed 1.0, where nothing
    // above 1/2 builds; that gap is what the payload's second bound
    // exists to close.
    let bound = largest_tangent_radius.expect("an arc x arc corner defines the bound");
    assert!(
        (bound - 0.5).abs() < 1e-12,
        "the lens's largest tangent circle is R = 1/2, got {bound}"
    );
    assert!(
        bound < carrier_radius,
        "the endorsed bound must sit below the class bound"
    );
    lens(0.99 * bound).expect("the endorsed radius must build");
    assert!(
        lens(1.01 * bound).is_err(),
        "above the endorsed bound the lens must not round"
    );
}

/// Carriers that never meet name their own reason — distinct from the
/// tangency knife edge, which still reports `CarriersParallel`.
///
/// Note this needs an INTERIOR arrival anchor, not `p: Start`: a
/// closing arrival anchors BOTH carriers at the entry point, so they
/// always share it and can never be disjoint. Independent anchors are
/// what make the case reachable.
#[test]
fn carriers_that_do_not_meet_refuse_typed() {
    // Two unit circles 10 m apart: disjoint, no corner anywhere.
    let refused = Open.arc_fillet_arc(
        Center {
            c: p2(0.0, 0.0),
            winding: profile::ArcSweep::Ccw,
            p: p2(1.0, 0.0),
        },
        0.25,
        Center {
            c: p2(10.0, 0.0),
            winding: profile::ArcSweep::Ccw,
            p: p2(11.0, 0.0),
        },
        Tol::witness(),
    );
    assert!(
        matches!(
            refused,
            Err(PathError::NoCornerForFillet {
                reason: profile::path::PathNoCornerReason::CarriersDoNotMeet,
                ..
            })
        ),
        "expected CarriersDoNotMeet, got {refused:?}"
    );
}

/// An anchor at its own carrier's centre names no tangent, so the fused
/// entry verb refuses before any fillet is opened against it.
#[test]
fn a_carrier_anchor_at_the_centre_refuses_typed() {
    let refused = Open.arc_fillet(
        Center {
            c: p2(0.0, 0.0),
            winding: profile::ArcSweep::Ccw,
            p: p2(0.0, 0.0),
        },
        0.25,
        Tol::witness(),
    );
    assert!(
        matches!(refused, Err(PathError::DegenerateArcCenter { .. })),
        "expected DegenerateArcCenter, got {refused:?}"
    );
}

/// **G1 NOTE-2's lesson, applied to G2's new gates**: an undecidable
/// margin escalates typed rather than guessing — on the carrier-meet
/// gate AND on the angular advance/reach gates, the paths that would
/// otherwise have only decided-Yes and decided-No coverage.
#[test]
fn the_new_arc_carrier_gates_escalate_in_band() {
    let tol = Tol::witness().get();
    // Squarely inside (ε, K·ε): undecidable at this scalar.
    let band = tol.eps * ((1.0 + tol.k) / 2.0);

    // path_carrier_meet: two unit circles whose centres sit `band`
    // FURTHER apart than 2 — externally tangent to within the band, so
    // whether they cross at all cannot be classified here.
    let refused = Open.arc_fillet_arc(
        Center {
            c: p2(0.0, 0.0),
            winding: profile::ArcSweep::Ccw,
            p: p2(1.0, 0.0),
        },
        0.25,
        Center {
            c: p2(2.0 + band, 0.0),
            winding: profile::ArcSweep::Ccw,
            p: p2(1.0 + band, 0.0),
        },
        Tol::witness(),
    );
    let Err(PathError::Escalated { source }) = refused else {
        panic!("in-band carrier separation must escalate, got {refused:?}");
    };
    assert_eq!(source.predicate, Some("path_carrier_meet"));

    // path_corner_reach_arc: the ray y = 0 meets the circle about
    // (2, −2) through the origin at (0,0) and (4,0). Put the ARRIVAL
    // anchor an in-band ARC LENGTH ahead of the surviving corner (4,0):
    // whether the corner really lies behind its anchor is then
    // undecidable, and the angular gate must say so.
    let centre = p2(2.0, -2.0);
    let r = 8.0f64.sqrt();
    let theta = std::f64::consts::FRAC_PI_4 + band / r;
    let anchor = p2(centre.x + r * theta.cos(), centre.y + r * theta.sin());
    let refused = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.3,
            Center {
                c: centre,
                winding: profile::ArcSweep::Ccw,
                p: anchor,
            },
            Tol::witness(),
        );
    let Err(PathError::Escalated { source }) = refused else {
        panic!("an in-band angular reach must escalate, got {refused:?}");
    };
    assert_eq!(source.predicate, Some("path_corner_reach_arc"));
}

/// The DECIDED sides of the same gates, for contrast: a margin
/// comfortably above K·ε proceeds, and one comfortably below refuses
/// with the gate's own typed refusal — never an escalation.
#[test]
fn the_new_arc_carrier_gates_decide_outside_the_band() {
    let centre = p2(2.0, -2.0);
    let r = 8.0f64.sqrt();
    // Well ahead of the corner: decided, and the fillet resolves.
    let theta = std::f64::consts::FRAC_PI_4 + 0.9;
    let anchor = p2(centre.x + r * theta.cos(), centre.y + r * theta.sin());
    assert!(
        Open.at(p2(0.0, 0.0))
            .toward(1.0, 0.0, Tol::witness())
            .unwrap()
            .fillet_arc(
                0.3,
                Center {
                    c: centre,
                    winding: profile::ArcSweep::Ccw,
                    p: anchor,
                },
                Tol::witness(),
            )
            .is_ok(),
        "a decided angular reach must resolve"
    );
    // Well BEHIND the corner: decided the other way, typed refusal.
    let theta = std::f64::consts::FRAC_PI_4 - 0.4;
    let anchor = p2(centre.x + r * theta.cos(), centre.y + r * theta.sin());
    let refused = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.3,
            Center {
                c: centre,
                winding: profile::ArcSweep::Ccw,
                p: anchor,
            },
            Tol::witness(),
        );
    assert!(
        matches!(refused, Err(PathError::NoCornerForFillet { .. })),
        "a decided-behind arrival anchor must refuse typed, got {refused:?}"
    );
}

/// **MINOR-2 (review)**: the third new gate's escalation path.
/// `path_carrier_meet` and `path_corner_reach_arc` already have in-band
/// rows; `path_corner_advance_arc` had only decided coverage, so the
/// G1 NOTE-2 standard was met for two predicates out of three.
///
/// Construction: the departure runs CCW on the unit circle from
/// `(1, 0)`, and the arrival carrier is built as the circle on the
/// diameter `P–Q`, where `P` sits an in-band ARC LENGTH ahead of the
/// departure anchor (R = 1, so the arc length IS the angle). `P` is
/// then an intersection of the two carriers to within an ulp, so the
/// derived corner lands there and "is it ahead of its anchor?" is
/// exactly the undecidable question. The carrier-meet gates are
/// comfortably decided first, so the escalation can only come from the
/// angular advance gate.
#[test]
fn the_angular_advance_gate_escalates_in_band() {
    let tol = Tol::witness().get();
    let band = tol.eps * ((1.0 + tol.k) / 2.0);
    let near = p2(band.cos(), band.sin());
    let far = p2(-0.5, 0.75f64.sqrt());
    // The circle on the P–Q diameter passes through both by
    // construction, so `far` is an exact anchor for it.
    let centre = p2((near.x + far.x) / 2.0, (near.y + far.y) / 2.0);
    let refused = Open.arc_fillet_arc(
        Center {
            c: p2(0.0, 0.0),
            winding: profile::ArcSweep::Ccw,
            p: p2(1.0, 0.0),
        },
        0.1,
        Center {
            c: centre,
            winding: profile::ArcSweep::Ccw,
            p: far,
        },
        Tol::witness(),
    );
    // Assert WHICH gate escalated: an escalation from `path_carrier_meet`
    // would satisfy a bare `Escalated` match while leaving the angular
    // advance gate as untested as before.
    let Err(PathError::Escalated { source }) = refused else {
        panic!("an in-band angular ADVANCE must escalate, got {refused:?}");
    };
    assert_eq!(
        source.predicate,
        Some("path_corner_advance_arc"),
        "the escalation must come from the angular advance gate"
    );
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// **LB10 route 3**: whatever the arrival's height and the fillet's
    /// radius, a STRAIGHT arrival off an ARC departure puts the arrival
    /// side EXACTLY on the ray `.at(p).toward(dx, dy)` names — the trim
    /// point rides `y = h`, the anchor lies on the trimmed side
    /// (strictly between the trim point and the leg's far end), and the
    /// lowered loop validates.
    ///
    /// The anchor is never emitted as a vertex: it anchors the side, and
    /// the side's run is emitted by the leg that ends it. That is the
    /// straight-arrival rule, unchanged by the carrier on the departure.
    #[test]
    fn a_straight_arrival_off_an_arc_departure_rides_the_authored_ray(
        h in 1.0f64..4.0,
        r in 0.1f64..0.6,
    ) {
        let lowered = Open
            .arc_fillet(
                Center {
                    c: p2(0.0, 0.0),
                    winding: profile::ArcSweep::Ccw,
                    p: p2(5.0, 0.0),
                },
                r,
                Tol::witness(),
            )
            .unwrap()
            .at(p2(0.0, h), Tol::witness())
            .unwrap()
            .toward(-1.0, 0.0, Tol::witness())
            .unwrap()
            .line(3.0, Tol::witness())
            .unwrap()
            .line_to(Start, Tol::witness())
            .unwrap();
        let lowered = pinned(lowered);
        // (5,0) the entry, t1 on the circle, t2 on the ray, (-3,h).
        prop_assert_eq!(lowered.vertices().len(), 4);
        let t2 = lowered.vertices()[2].pos();
        prop_assert!((t2.y - h).abs() < 1e-12, "t2 rides y = h: {:?}", t2);
        prop_assert!(t2.x > 0.0, "the trim point is short of the anchor: {:?}", t2);
        let far = lowered.vertices()[3].pos();
        prop_assert_eq!(far.x.to_bits(), (-3.0f64).to_bits());
        prop_assert_eq!(far.y.to_bits(), h.to_bits());
        validate_ok(&lowered);
    }
}

// ------------------------------------------------------------------
// LIB-RESPELL §2c: the fused family's admissibility matrix, row by row.
// Every ADMISSIBLE (site, state, mode) pair is exercised; inadmissible
// pairs are missing impls (compile-time) and, at the wire, the
// Transition class (path_program.rs).
// ------------------------------------------------------------------

use geom_core::Tol;
use profile::{ArcLen, ArcSide, Bulge, Center, Radius, Sweep, Via};

/// LEG rows `Sweep@Directed` / `ArcLen@Directed`: the endpoint-free
/// pair are the arc analogs of `line(len)` — and `ArcLen { len }` IS
/// `Sweep { angle: len / r }`, bit for bit.
#[test]
fn sweep_and_arclen_legs_agree_bitwise() {
    let by_sweep = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .arc_to(
            Sweep {
                r: 2.0,
                side: ArcSide::Left,
                angle: 0.8,
            },
            Tol::witness(),
        )
        .unwrap()
        .line_to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    let by_len = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .arc_to(
            ArcLen {
                r: 2.0,
                side: ArcSide::Left,
                len: 1.6,
            },
            Tol::witness(),
        )
        .unwrap()
        .line_to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    assert_eq!(by_sweep.vertices().len(), by_len.vertices().len());
    for (a, b) in by_sweep.vertices().iter().zip(by_len.vertices().iter()) {
        assert_eq!(a.pos().x.to_bits(), b.pos().x.to_bits());
        assert_eq!(a.pos().y.to_bits(), b.pos().y.to_bits());
        assert_eq!(a.bulge().to_bits(), b.bulge().to_bits());
    }
    validate_ok(&by_sweep);
}

/// FUSED-INCOMING rows `Bulge@Point`, `Via@Point`, `Center@Point`: the
/// endpoint-full modes author the incoming side and its anchor in one
/// act; the anchor is a real on-path point INSIDE the emitted run (the
/// trim extends past it into the corner), never a vertex.
#[test]
fn fused_point_incomings_author_their_anchor_on_path() {
    // Bulge: the sagging arc (0,0)→(4,0), b = +0.25 (Ccw), filleted
    // onto the northbound line x = 6 anchored at (6,3).
    let bulge = Open
        .at(p2(0.0, 0.0))
        .arc_fillet(
            Bulge {
                p: p2(4.0, 0.0),
                b: 0.25,
            },
            0.3,
            Tol::witness(),
        )
        .unwrap()
        .at(p2(6.0, 3.0), Tol::witness())
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .line(2.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    validate_ok(&bulge);
    // The authored anchor (4,0) lies ON the first emitted arc's carrier
    // (it is interior to the run, not a vertex).
    assert!(
        !bulge
            .vertices()
            .iter()
            .any(|v| v.pos().x == 4.0 && v.pos().y == 0.0)
    );

    // Via and Center naming the SAME carrier (centre (2, 1.5) exact):
    // the circle through (0,0), (2,−1), (4,0).
    let via = Open
        .at(p2(0.0, 0.0))
        .arc_fillet(
            Via {
                q: p2(2.0, -1.0),
                p: p2(4.0, 0.0),
            },
            0.3,
            Tol::witness(),
        )
        .unwrap()
        .at(p2(4.0, 4.0), Tol::witness())
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    validate_ok(&via);
    let center = Open
        .at(p2(0.0, 0.0))
        .arc_fillet(
            Center {
                c: p2(2.0, 1.5),
                winding: profile::ArcSweep::Ccw,
                p: p2(4.0, 0.0),
            },
            0.3,
            Tol::witness(),
        )
        .unwrap()
        .at(p2(4.0, 4.0), Tol::witness())
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    validate_ok(&center);
}

/// FUSED-INCOMING rows `Sweep@Directed` / `ArcLen@Directed` (the
/// tangent-departing incomings), bit-identical to each other, plus the
/// far-end anchor ENDING an arc-incoming fillet's straight arrival.
#[test]
fn fused_tangent_incomings_and_the_far_end_arrival() {
    let chain = |by_len: bool| {
        let dir = Open.at(p2(0.0, 0.0)).angle(0.0, Tol::witness()).unwrap();
        let opened = if by_len {
            dir.arc_fillet(
                ArcLen {
                    r: 2.0,
                    side: ArcSide::Left,
                    len: 1.6,
                },
                0.25,
                Tol::witness(),
            )
        } else {
            dir.arc_fillet(
                Sweep {
                    r: 2.0,
                    side: ArcSide::Left,
                    angle: 0.8,
                },
                0.25,
                Tol::witness(),
            )
        };
        opened
            .unwrap()
            .toward(-1.0, 0.0, Tol::witness())
            .unwrap()
            .to(p2(0.0, 3.0), Tol::witness())
            .unwrap()
            .line_to(Start, Tol::witness())
            .map(pinned)
            .unwrap()
    };
    let sweep = chain(false);
    let len = chain(true);
    for (a, b) in sweep.vertices().iter().zip(len.vertices().iter()) {
        assert_eq!(a.pos().x.to_bits(), b.pos().x.to_bits());
        assert_eq!(a.pos().y.to_bits(), b.pos().y.to_bits());
    }
    // The far-end anchor is a vertex, exactly as on straight chains.
    assert!(
        sweep
            .vertices()
            .iter()
            .any(|v| v.pos().x == 0.0 && v.pos().y == 3.0)
    );
    validate_ok(&sweep);
}

/// ARRIVAL rows `Radius` (both binder orders) and `Via` (interior):
/// the Radius arrival derives its centre from the directed anchor; the
/// binder ORDER cannot move a bit.
#[test]
fn radius_and_via_arrivals_complete_via_their_binders() {
    type ArrivalTip =
        PartialPath<f64, profile::path::HasPos<profile::path::WithIncoming>, profile::path::NoAng>;
    let close_from = |arrival: ArrivalTip| {
        arrival
            .arc_fillet(
                Radius {
                    r: 1.0,
                    side: ArcSide::Left,
                },
                0.25,
                Tol::witness(),
            )
            .unwrap()
            .at(p2(1.2, 0.2), Tol::witness())
            .unwrap()
            .toward(0.0, -1.0, Tol::witness())
            .unwrap()
            .line(0.1, Tol::witness())
            .unwrap()
            .line_to(Start, Tol::witness())
            .map(pinned)
            .unwrap()
    };
    let entry = || {
        Open.at(p2(0.0, 0.0))
            .toward(1.0, 0.0, Tol::witness())
            .unwrap()
    };
    // Radius arrival, anchor-first and director-first.
    let a = close_from(
        entry()
            .fillet_arc(
                0.25,
                Radius {
                    r: 1.0,
                    side: ArcSide::Left,
                },
                Tol::witness(),
            )
            .unwrap()
            .at(p2(2.0, 1.5))
            .toward(-1.0, 0.0, Tol::witness())
            .unwrap(),
    );
    let b = close_from(
        entry()
            .fillet_arc(
                0.25,
                Radius {
                    r: 1.0,
                    side: ArcSide::Left,
                },
                Tol::witness(),
            )
            .unwrap()
            .toward(-1.0, 0.0, Tol::witness())
            .unwrap()
            .at(p2(2.0, 1.5), Tol::witness())
            .unwrap(),
    );
    for (va, vb) in a.vertices().iter().zip(b.vertices().iter()) {
        assert_eq!(va.pos().x.to_bits(), vb.pos().x.to_bits());
        assert_eq!(va.pos().y.to_bits(), vb.pos().y.to_bits());
        assert_eq!(va.bulge().to_bits(), vb.bulge().to_bits());
    }
    validate_ok(&a);
    // Via arrival: the SAME carrier named through a point on it.
    let v = close_from(
        entry()
            .fillet_arc(
                0.25,
                Via {
                    q: p2(3.0, 0.5),
                    p: p2(2.0, 1.5),
                },
                Tol::witness(),
            )
            .unwrap()
            .toward(-1.0, 0.0, Tol::witness())
            .unwrap(),
    );
    validate_ok(&v);
}

/// ARRIVAL row `Via { q, p: Start }`: the via-completed CLOSE, and the
/// SEAM fillet on a fused arc incoming (`.to(Start)` with a straight
/// first side) — the two Start-targeting completions the family adds.
#[test]
fn via_start_close_and_the_arc_incoming_seam() {
    let via_close = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .line(3.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.25,
            Via {
                q: p2(2.5, 2.5),
                p: Start,
            },
            Tol::witness(),
        )
        .unwrap()
        .toward(-0.2, -1.0, Tol::witness())
        .unwrap();
    let via_close = pinned(via_close);
    validate_ok(&via_close);

    let seam = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .line(3.0, Tol::witness())
        .unwrap()
        .angle(2.0, Tol::witness())
        .unwrap()
        .arc_fillet(
            Sweep {
                r: 3.5,
                side: ArcSide::Left,
                angle: 1.9,
            },
            0.3,
            Tol::witness(),
        )
        .unwrap()
        .to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    validate_ok(&seam);
    // The seam arc closes the loop: joint 0 is declared.
    assert!(seam.tangent_joints().contains(&0));
}

/// RAY EXTENSION (§2c round 10): bare `fillet(r)` directly on a leg
/// end is `.tangent().fillet(r)`, bit for bit — the surviving ray
/// piece is a genuine line leg, whatever leg came before.
#[test]
fn ray_extension_is_tangent_fillet_bitwise() {
    let chain = |extend: bool| {
        let leg = Open
            .at(p2(0.0, 0.0))
            .line_to(p2(3.0, 0.0), Tol::witness())
            .unwrap();
        let opened = if extend {
            leg.fillet(0.5, Tol::witness())
        } else {
            leg.tangent().fillet(0.5, Tol::witness())
        };
        opened
            .unwrap()
            .at(p2(5.0, 3.0), Tol::witness())
            .unwrap()
            .toward(0.0, 1.0, Tol::witness())
            .unwrap()
            .line(1.0, Tol::witness())
            .unwrap()
            .line_to(Start, Tol::witness())
            .map(pinned)
            .unwrap()
    };
    let extended = chain(true);
    let spelled = chain(false);
    for (a, b) in extended.vertices().iter().zip(spelled.vertices().iter()) {
        assert_eq!(a.pos().x.to_bits(), b.pos().x.to_bits());
        assert_eq!(a.pos().y.to_bits(), b.pos().y.to_bits());
        assert_eq!(a.bulge().to_bits(), b.bulge().to_bits());
    }
    assert_eq!(extended.tangent_joints(), spelled.tangent_joints());
    validate_ok(&extended);
}

// ------------------------------------------------------------------
// The straight continuation: `line(len)` off a DIRECTED POINT (issue
// 433 half (i)). A straight run subdivided at an interior vertex is
// one carrier said on extra vertices — carrier IDENTITY, which the
// data gate already accepts undeclared. These rows pin where the
// authoring door now agrees with it, and where it still refuses.
// ------------------------------------------------------------------

/// The continuation chain, end to end: a straight run subdivided at an
/// interior vertex, authored through the public surface and closed.
/// Nothing is declared (the subdivision is structural, not a tangency
/// claim), and the lowered loop passes the data gate — the two doors
/// that issue 433 found disagreeing now agree on this shape.
#[test]
fn straight_continuation_subdivides_a_run_and_validates() {
    let lp = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .line(2.0, Tol::witness())
        .unwrap()
        // No director bound here: the leg departs along the directed
        // point's own tangent. This is the row under test.
        .line(2.0, Tol::witness())
        .unwrap()
        .line_to(p2(4.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    let v: Vec<_> = lp
        .vertices()
        .iter()
        .map(|x| (x.pos().x, x.pos().y))
        .collect();
    assert_eq!(v, vec![(0.0, 0.0), (2.0, 0.0), (4.0, 0.0), (4.0, 3.0)]);
    assert!(
        lp.tangent_joints().is_empty(),
        "the subdivision declares nothing: {:?}",
        lp.tangent_joints()
    );
    validate_ok(&lp);
}

/// The inherited tangent is the incoming RAY itself, bit for bit. What
/// that buys is the direction, not the arithmetic: here the two equal
/// legs also lay down bit-identical DISPLACEMENTS, but that is this
/// fixture's property — the entry sits at the origin and the sums
/// `0 + d` and `d + d` are exact. A third equal leg's endpoint rounds
/// and its realized displacement differs in the last bit
/// (`r2_probe_bitwise_inheritance_is_transitive` pins exactly that
/// boundary). The comparison direction is what makes the ray claim
/// non-vacuous: re-deriving the ray through the leg's angle (the
/// `atan2`/`sin_cos` round trip any re-authored spelling would take)
/// moves the endpoint.
#[test]
fn straight_continuation_inherits_the_tangent_bitwise() {
    let lp = Open
        .at(p2(0.0, 0.0))
        .toward(3.0, 7.0, Tol::witness())
        .unwrap()
        .line(2.0, Tol::witness())
        .unwrap()
        .line(2.0, Tol::witness())
        .unwrap()
        .line_to(p2(-4.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    let v = lp.vertices();
    let first = (v[1].pos().x - v[0].pos().x, v[1].pos().y - v[0].pos().y);
    let second = (v[2].pos().x - v[1].pos().x, v[2].pos().y - v[1].pos().y);
    assert_eq!(first.0.to_bits(), second.0.to_bits());
    assert_eq!(first.1.to_bits(), second.1.to_bits());
    let theta = first.1.atan2(first.0);
    let round_tripped = (theta.cos() * 2.0, theta.sin() * 2.0);
    assert!(
        round_tripped.0.to_bits() != second.0.to_bits()
            || round_tripped.1.to_bits() != second.1.to_bits(),
        "the round trip must MOVE the ray, or this row proves nothing"
    );
}

/// The length gate is the continuation's one band-sensitive read: a
/// non-positive length would run the side backward, exactly as on the
/// directed row.
#[test]
fn straight_continuation_gates_its_length() {
    let tip = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .line(2.0, Tol::witness())
        .unwrap();
    assert!(matches!(
        tip.line(-1.0, Tol::witness()),
        Err(PathError::NonpositiveLeg { .. })
    ));
}

/// An AUTHORED direction landing in the tangent band still refuses —
/// a target that happens to be collinear is a value coincidence, and
/// the ladder never reads intent off a margin. The recourse now names
/// the structural spelling that exists for the case it was missing.
#[test]
fn authored_collinear_target_refuses_naming_the_structural_spelling() {
    let err = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0), Tol::witness())
        .unwrap()
        .line_to(p2(4.0, 0.0), Tol::witness())
        .unwrap_err();
    assert!(matches!(err, PathError::JunctionTangent { .. }));
    let text = err.to_string();
    assert!(
        text.contains("line(len)"),
        "the tangent refusal must name the straight continuation: {text}"
    );
    assert!(
        text.contains(".tangent()"),
        "and still name the declaration door: {text}"
    );
}

/// A CURVED zero-turn junction keeps refusing: the departure is
/// authored, and off an arc the same direction is tangency onto a
/// DISTINCT carrier — the undeclared-tangency doctrine, untouched.
#[test]
fn curved_zero_turn_still_refuses() {
    let arc_end = Open
        .at(p2(-1.0, 0.0))
        .arc_to(
            Bulge {
                p: p2(1.0, 0.0),
                b: 1.0,
            },
            Tol::witness(),
        )
        .unwrap();
    assert!(matches!(
        arc_end.turn(0.0, Tol::witness()),
        Err(PathError::JunctionTangent { .. })
    ));
}

/// The continuation row is CARRIER-BLIND, as the §2c axiom requires:
/// it reads the tangent bit and nothing about the leg that produced
/// it. Off an ARC that spelling authors a line tangent to the arc and
/// declares nothing — a tangency between distinct carriers, which the
/// DATA gate refuses. The declared spelling of the same geometry
/// (`.tangent().line(len)`) passes, which is the whole difference.
#[test]
fn continuation_off_an_arc_is_undeclared_tangency_at_the_data_gate() {
    let semicircle = || {
        Open.at(p2(-1.0, 0.0))
            .arc_to(
                Bulge {
                    p: p2(1.0, 0.0),
                    b: 1.0,
                },
                Tol::witness(),
            )
            .unwrap()
    };
    let undeclared = semicircle()
        .line(1.0, Tol::witness())
        .expect("the door cannot see the carrier, so it cannot refuse here")
        .line_to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    assert!(undeclared.tangent_joints().is_empty());
    let refused = Profile::new(SketchPlane::xy(), vec![undeclared])
        .validate(Tol::witness())
        .unwrap_err();
    assert!(
        matches!(refused, profile::ProfileError::UndeclaredTangency { .. }),
        "the data gate is where this lands: {refused:?}"
    );
    let declared = semicircle()
        .tangent()
        .line(1.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    assert_eq!(declared.tangent_joints(), &[1]);
    validate_ok(&declared);
}

/// **Where the continuation stops — and where it no longer does.**
/// (BOOL-8 pinned this as a two-rotation WALL; the ruling's declared
/// closer landed, and the row records what actually moved.)
///
/// The fixture is lily's section: four corners, every side subdivided
/// at one interior vertex, eight vertices on a four-corner outline (the
/// loft's vertex budget). Interior subdivisions authored fine already;
/// the side that CROSSES the seam is what this row is about, and the
/// two rotations were never the same refusal wearing one name.
///
/// - **Rotation 1 — seam at a CORNER.** The closer departs the run's
///   subdivision vertex, so the junction in band is the CLOSER'S OWN
///   departure. Undeclared, that still refuses
///   (now an ordinary `JunctionTangent`): `line_to(Start)` computed
///   a direction and found it collinear, and reading intent off that is
///   the inference the ladder refuses. DECLARED, it closes —
///   `continue_to(Start)` takes the departing point's own ray and
///   checks `Start` against it. This half of the wall is over.
/// - **Rotation 2 — seam at the SUBDIVISION vertex.** The closer
///   departs the corner asserted below, and the junction in band is the
///   SEAM'S. That is a mid-carrier seam, PQ4, and no spelling of the
///   closing leg moves it: `line_to(Start)` refuses
///   `SeamTangent`, and the declared closer does
///   not apply at all — the leg departs a CORNER here, so `Start` is
///   off its ray and the verb refuses that first.
///
/// The premise both rotations rest on — that the tip `right` is a
/// DEFINITE corner — is measured here rather than argued, because it is
/// what makes each rotation have exactly ONE in-band junction. The site
/// The two mechanisms are separable at the REFUSAL rather than only
/// through the fixture that provoked each — which was BOOL-8's ask —
/// and they are separable by TYPE: `JunctionTangent` for a departure,
/// `SeamTangent` for a seam. That is strictly better than the payload
/// tag an earlier draft used. A tag has to be read and can be ignored
/// by a `{ .. }` pattern; two types cannot be confused by a caller,
/// cannot be matched by accident, and let each refusal carry only the
/// payload its own recourse needs.
///
/// The departure half is an ORDINARY refusal now, and deliberately so:
/// a tangent departure on a closing leg is geometrically identical to
/// one mid-chain, and since the declared closer landed the recourse is
/// identical too — so a close-only second name for it was uniformity
/// debt, against PATHS' rule that `Start` goes through ordinary verbs.
#[test]
fn the_seam_wall_ends_at_the_departure_and_stands_at_the_seam() {
    let right = p2(1.0, 0.0);
    let ridge = p2(0.0, 1.5);
    let left = p2(-1.0, 0.0);
    let keel = p2(0.0, -1.0);
    let mid = |a: Point2<f64>, b: Point2<f64>| p2(0.5 * (a.x + b.x), 0.5 * (a.y + b.y));
    let half = |a: Point2<f64>, b: Point2<f64>| 0.5 * (b - a).norm_squared().sqrt();
    let t = Tol::witness();
    let m3 = mid(keel, right);
    let into_right = (right.x - m3.x, right.y - m3.y);
    let out_of_right = (ridge.x - right.x, ridge.y - right.y);
    let turn_at_right = into_right.0 * out_of_right.1 - into_right.1 * out_of_right.0;
    assert!(
        turn_at_right.abs() > 0.5,
        "the seam corner must be definitely sharp, not near-tangent: {turn_at_right}"
    );
    let side = |chain: PartialPath<f64, HasPos<WithIncoming>, profile::path::NoAng>,
                from: Point2<f64>,
                to: Point2<f64>| {
        let d = to - from;
        chain
            .toward(d.x, d.y, t)
            .unwrap()
            .line(half(from, to), t)
            .unwrap()
            .line(half(from, to), t)
            .unwrap()
    };
    // Rotation 1 — seam at the tip `right`: three sides subdivide, and
    // the closer departs the fourth side's subdivision vertex.
    let at_m3 = || {
        let d0 = ridge - right;
        let first = Open
            .at(right)
            .toward(d0.x, d0.y, t)
            .unwrap()
            .line(half(right, ridge), t)
            .unwrap()
            .line(half(right, ridge), t)
            .unwrap();
        side(side(first, ridge, left), left, keel)
            .toward(right.x - keel.x, right.y - keel.y, t)
            .unwrap()
            .line(half(keel, right), t)
            .unwrap()
    };
    assert!(matches!(
        at_m3().line_to(Start, t),
        Err(PathError::JunctionTangent { .. })
    ));
    let closed = pinned(
        at_m3()
            .continue_to(Start, t)
            .expect("the declared closer ends the run that crosses the seam"),
    );
    assert_eq!(closed.vertices().len(), 8);
    assert!(closed.tangent_joints().is_empty());
    validate_ok(&closed);
    // Rotation 2 — seam at the subdivision vertex `mid(keel, right)`:
    // the closer departs the corner asserted above, and the SEAM
    // junction is the straight one. PQ4, unmoved by either spelling.
    let d = right - m3;
    let round = || {
        Open.at(m3)
            .toward(d.x, d.y, t)
            .unwrap()
            .line(half(m3, right), t)
            .unwrap()
    };
    let back_at_keel = || side(side(side(round(), right, ridge), ridge, left), left, keel);
    assert!(matches!(
        back_at_keel().line_to(Start, t),
        Err(PathError::SeamTangent { .. })
    ));
    // The declared closer does not even APPLY here, and that is the
    // sharper half: in this rotation the closing leg departs a corner,
    // so it is not a continuation of anything — `Start` is off the ray
    // the departing point defines, and the verb says so before the seam
    // is ever classified. The seam refusal above is what the spelling
    // that IS applicable gets. (A fixture where the closer is a
    // continuation AND the seam is straight needs two consecutive
    // subdivisions on one side, which strict alternation forbids;
    // `bool11_probes` builds one and pins `site: Seam` from this verb.)
    assert!(matches!(
        back_at_keel().continue_to(Start, t),
        Err(PathError::ContinuationTargetOffRay { .. })
    ));
}
// ==================================================================
// R2 BOOL-8 probes (PR #1508, frozen head 6aa2684f2). APPENDED to
// crates/profile/tests/path_property.rs for the probe runs and
// REVERTED after; kept here as the record. Each probe attacks one
// PR-body claim; results in review/r2-bool8/NOTES.md.
// ==================================================================

/// PROBE 1 (claim 3): no authored spelling sneaks a tangency through
/// as a "continuation". Every director that could re-author the
/// incoming direction — `.toward` with the exact same displacement,
/// `.turn(0)`, `.angle(exact incoming angle)` — still refuses
/// `JunctionTangent`; the declared spelling refuses
/// `SameCarrierJunction`. The only accepting spelling is the one with
/// NO authored direction at all.
#[test]
fn r2_probe_authored_spellings_cannot_sneak_the_continuation() {
    let t = Tol::witness();
    let tip = || {
        Open.at(p2(0.0, 0.0))
            .toward(3.0, 7.0, t)
            .unwrap()
            .line(2.0, t)
            .unwrap()
    };
    // .toward with the exact incoming displacement: authored, refuses.
    assert!(matches!(
        tip().toward(3.0, 7.0, t),
        Err(PathError::JunctionTangent { .. })
    ));
    // .turn(0) off a LINE end (the arc case has its own row): refuses.
    assert!(matches!(
        tip().turn(0.0, t),
        Err(PathError::JunctionTangent { .. })
    ));
    // .angle at the exact incoming angle: authored, refuses.
    let theta = 7.0f64.atan2(3.0);
    assert!(matches!(
        tip().angle(theta, t),
        Err(PathError::JunctionTangent { .. })
    ));
    // declared identity: refuses (the #101 rule, untouched).
    assert!(matches!(
        tip().tangent().line(2.0, t),
        Err(PathError::SameCarrierJunction { .. })
    ));
}

/// PROBE 2 (claim 4): the carrier-blindness seam cannot be laundered
/// past `validate` by chaining — TWO continuations off the arc still
/// land at the data gate, and so does a continuation off a fillet's
/// ARC arrival end (a second arc-carrier directed point in the tree).
#[test]
fn r2_probe_arc_continuations_never_pass_validate() {
    let t = Tol::witness();
    let undeclared = Open
        .at(p2(-1.0, 0.0))
        .arc_to(
            Bulge {
                p: p2(1.0, 0.0),
                b: 1.0,
            },
            t,
        )
        .unwrap()
        .line(0.5, t)
        .unwrap()
        .line(0.5, t)
        .unwrap()
        .line_to(Start, t)
        .map(pinned)
        .unwrap();
    assert!(undeclared.tangent_joints().is_empty());
    let refused = Profile::new(SketchPlane::xy(), vec![undeclared])
        .validate(t)
        .unwrap_err();
    assert!(
        matches!(refused, profile::ProfileError::UndeclaredTangency { .. }),
        "chained continuations off an arc must still land at the data gate: {refused:?}"
    );
}

/// PROBE 3 (claim 5): third-spelling search for the lily seam wall,
/// rotation 1 fixture (seam at the corner `right`). Every UNDECLARED
/// candidate closer refuses from the run's subdivision vertex, and the
/// continuation dead-ends structurally. (The declared closer is the one
/// that gets through, and the row above pins that; what this row keeps
/// is that nothing in the undeclared alphabet does, which is what makes
/// the declaration load-bearing rather than decorative.)
///  (a) `.tangent()` + tangent arc to Start — degenerates onto the
///      carrier (SameCarrierJunction);
///  (b) the REVERSED traversal — same alternation, same wall;
///  (c) continuing `line(half)` to land exactly ON Start's
///      coordinates — a directed point, not a closure; the zero-length
///      `line_to(Start)` left over refuses.
#[test]
fn r2_probe_lily_seam_third_spellings_all_refuse() {
    let right = p2(1.0, 0.0);
    let ridge = p2(0.0, 1.5);
    let left = p2(-1.0, 0.0);
    let keel = p2(0.0, -1.0);
    let half = |a: Point2<f64>, b: Point2<f64>| 0.5 * (b - a).norm_squared().sqrt();
    let t = Tol::witness();
    let side = |chain: PartialPath<f64, HasPos<WithIncoming>, profile::path::NoAng>,
                from: Point2<f64>,
                to: Point2<f64>| {
        let d = to - from;
        chain
            .toward(d.x, d.y, t)
            .unwrap()
            .line(half(from, to), t)
            .unwrap()
            .line(half(from, to), t)
            .unwrap()
    };
    let at_m3 = || {
        let d0 = ridge - right;
        let first = Open
            .at(right)
            .toward(d0.x, d0.y, t)
            .unwrap()
            .line(half(right, ridge), t)
            .unwrap()
            .line(half(right, ridge), t)
            .unwrap();
        side(side(first, ridge, left), left, keel)
            .toward(right.x - keel.x, right.y - keel.y, t)
            .unwrap()
            .line(half(keel, right), t)
            .unwrap()
    };
    // (a) declared + tangent arc to Start: degenerate onto the carrier.
    assert!(matches!(
        at_m3().tangent().tangent_arc_to(Start, t),
        Err(PathError::SameCarrierJunction { .. })
    ));
    // (b) reversed traversal (right -> keel -> left -> ridge -> right):
    // the closer still departs a subdivision vertex.
    let db = keel - right;
    let rev_first = Open
        .at(right)
        .toward(db.x, db.y, t)
        .unwrap()
        .line(half(right, keel), t)
        .unwrap()
        .line(half(right, keel), t)
        .unwrap();
    let rev_at_last_mid = side(side(rev_first, keel, left), left, ridge)
        .toward(right.x - ridge.x, right.y - ridge.y, t)
        .unwrap()
        .line(half(ridge, right), t)
        .unwrap();
    assert!(matches!(
        rev_at_last_mid.line_to(Start, t),
        Err(PathError::JunctionTangent { .. })
    ));
    // (c) the continuation lands ON Start's coordinates but mints a
    // directed point, not a closure; the leftover closer is
    // zero-length and refuses. (NonpositiveLeg via line_to's sugar, or
    // whatever typed refusal the door gives — the point is Err.)
    let parked_on_start = at_m3().line(half(keel, right), t).unwrap();
    assert!(parked_on_start.line_to(Start, t).is_err());
}

/// PROBE 4 (claim 1), REVISED after a first run: the bit-identical
/// DISPLACEMENT property is a fixture artifact, not the inherited
/// thing. From the origin, `0 + d` and `d + d` are exact, so the first
/// two displacements match bitwise — but the THIRD leg's endpoint
/// rounds (`2d + d` is inexact) and its realized displacement differs
/// in the last bit. What is inherited bitwise is the `Dir`; the vertex
/// table only shows it exactly while the additions are exact. This
/// probe pins the boundary: d(0) == d(1), d(1) != d(2).
#[test]
fn r2_probe_bitwise_inheritance_is_transitive() {
    let t = Tol::witness();
    let lp = Open
        .at(p2(0.0, 0.0))
        .toward(0.1, 0.3, t)
        .unwrap()
        .line(0.7, t)
        .unwrap()
        .line(0.7, t)
        .unwrap()
        .line(0.7, t)
        .unwrap()
        .line_to(p2(-5.0, 1.0), t)
        .unwrap()
        .line_to(Start, t)
        .map(pinned)
        .unwrap();
    let v = lp.vertices();
    let d = |i: usize| {
        (
            (v[i + 1].pos().x - v[i].pos().x).to_bits(),
            (v[i + 1].pos().y - v[i].pos().y).to_bits(),
        )
    };
    assert_eq!(d(0), d(1), "doubling from the origin is exact");
    assert_ne!(
        d(1),
        d(2),
        "the third endpoint rounds: bit-identical displacements are the \
         fixture's property, not the inheritance's"
    );
    assert!(lp.tangent_joints().is_empty());
    validate_ok(&lp);
}
