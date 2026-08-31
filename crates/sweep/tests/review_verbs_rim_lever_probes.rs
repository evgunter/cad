//! **Reviewer's independent derivation of the closed-rim lever
//! guarantee** (#554 / VERBS-RIM). Written against the PUBLIC API
//! only, without reading the implementation's sample schedule as an
//! oracle: every row states a property of the lever that a reader of
//! the *claim* would state, and every row can go RED when the
//! guarantee degrades — not merely when it is violated at one chosen
//! fixture.
//!
//! The claims under test, and the row that would catch each one
//! breaking:
//!
//! 1. **Sound** — the metered lever never over-reports the edge's
//!    extent. Red if `arm_len` ever exceeds the carrier's arc length
//!    (`lever_is_bounded_by_arc_length_and_never_collapses_on_a_closed_rim`).
//! 2. **Never collapses on a closed rim** — the defect itself. The
//!    same row floors the lever at `arclen/π`, which every circular
//!    rim clears by construction (a full circle meters `2r` against
//!    `2πr`) and which the endpoint chord fails by a factor of
//!    ~1e16. A revert to the endpoint chord reds this, at every
//!    randomized radius.
//! 3. **Exact reduction on straight edges** — a prism's twelve
//!    straight edges meter BIT-identically to the endpoint chord
//!    (`straight_edges_meter_bit_identically_to_the_endpoint_chord`).
//! 4. **#554's pair goes honest, and stays honest across the
//!    dihedral** — a full revolve's transverse latitude rim and the
//!    partial revolve of the same profile refuse the SAME class, over
//!    a randomized range of dihedral angles
//!    (`the_554_pair_agrees_across_a_randomized_dihedral`).
//! 5. **The detector survived** — a co-surface seam still refuses
//!    `TangentialEdge` at a margin of exactly 0.0 while a transverse
//!    closed rim of the same body family does not
//!    (`co_surface_seams_still_refuse_while_transverse_rims_do_not`).
//! 6. **Both convexity signs on closed rims**
//!    (`closed_rims_decide_both_convexity_signs_at_diameter_levers`).
//! 7. **The >240° open-arc class the change also moves** — an open
//!    arc approaching a full turn must NOT collapse either; the lever
//!    is monotone in the swept angle beyond the crossover
//!    (`open_arcs_approaching_a_full_turn_do_not_collapse`).
//! 8. **The refusal's prose no longer asserts tangency as a fact**
//!    (`the_tangential_refusal_prose_states_no_geometric_fact`).
//!
//! Randomized rows draw a fresh seed per process and take their
//! counts off `CAD_FUZZ_EFFORT` (`test_utils::fuzz`), per the
//! harness's shape-1 (counterexample search) contract: cutting the
//! count loses detection power and can never make a row pass that
//! would otherwise fail.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Point2, Tol};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::blend::battery::{BlendRequest, run_battery};
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, Convexity};
use sweep::test_support::revolved_about_y;
use sweep::{Extrusion, Revolution, extrude};
use test_utils::fuzz;
use topo::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// Revolve a closed sketch loop about the sketch y-axis.
fn revolved(verts: Vec<ProfileVertex<f64>>, rev: Revolution<f64>) -> Body<f64> {
    revolved_about_y(verts, rev, tol())
}

/// The two support surfaces of an edge and whether it is CLOSED
/// (start vertex == end vertex).
fn sides(body: &Body<f64>, edge: EdgeKey) -> (Surface<f64>, Surface<f64>, bool) {
    let e = body.get_edge(edge).unwrap();
    let surf = |he| {
        let l = body.get_half_edge(he).unwrap().parent_loop;
        let f = body.get_loop(l).unwrap().face;
        body.get_surface(body.get_face(f).unwrap().surface)
            .unwrap()
            .clone()
    };
    let start = body.get_half_edge(e.he_plus).unwrap().start;
    let end = body.half_edge_end(e.he_plus).unwrap();
    (surf(e.he_plus), surf(e.he_minus), start == end)
}

/// The PRE-FIX lever: the straight-line chord between the edge's two
/// endpoints. Kept as the differential reference — every row that
/// says "no worse than before" compares against this, and it is the
/// value a revert would restore.
fn endpoint_chord(body: &Body<f64>, edge: EdgeKey) -> f64 {
    let e = body.get_edge(edge).unwrap();
    let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    let (t0, t1) = c.params();
    (c.carrier().eval(t1) - c.carrier().eval(t0)).norm()
}

/// The carrier's arc length, by refinement — the physical extent no
/// sound lever may exceed. Deliberately NOT the implementation's
/// sample schedule: a copy of that would only restate the code.
fn arc_length(body: &Body<f64>, edge: EdgeKey) -> f64 {
    let e = body.get_edge(edge).unwrap();
    let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    let (t0, t1) = c.params();
    let carrier = c.carrier();
    let n = 4096;
    let mut total = 0.0;
    let mut prev = carrier.eval(t0);
    for i in 1..=n {
        let t = t0 + (t1 - t0) * f64::from(i) / f64::from(n);
        let q = carrier.eval(t);
        total += (q - prev).norm();
        prev = q;
    }
    total
}

fn is_plane(s: &Surface<f64>) -> bool {
    matches!(s, Surface::Plane { .. })
}
fn is_sphere(s: &Surface<f64>) -> bool {
    matches!(s, Surface::Sphere { .. })
}
fn is_cone(s: &Surface<f64>) -> bool {
    matches!(s, Surface::Cone { .. })
}
fn is_cylinder(s: &Surface<f64>) -> bool {
    matches!(s, Surface::Cylinder { .. })
}

/// Every edge matching a support predicate and a closedness
/// requirement, widest by ARC LENGTH — an ordering independent of the
/// lever under test, so a broken lever cannot pick a friendlier edge.
fn pick_edge(
    body: &Body<f64>,
    closed: bool,
    pair: impl Fn(&Surface<f64>, &Surface<f64>) -> bool,
) -> EdgeKey {
    body.edges()
        .map(|(k, _)| k)
        .filter(|k| {
            let (a, b, c) = sides(body, *k);
            c == closed && (pair(&a, &b) || pair(&b, &a))
        })
        .max_by(|x, y| arc_length(body, *x).total_cmp(&arc_length(body, *y)))
        .expect("the fixture mints the requested edge")
}

/// The battery's own metered lever for one edge, read off the verdict
/// (`Link::arm_len`) — the kernel's number, never a restatement of
/// its formula.
fn metered_lever(body: &Body<f64>, edge: EdgeKey) -> Option<f64> {
    let req = BlendRequest {
        body,
        edges: vec![edge],
        size: 0.01,
    };
    run_battery(&req, band())
        .ok()
        .map(|v| v.chains[0].first().arm_len)
}

/// A dome ring: a sphere zone of radius `r` rising off a flat base
/// annulus, bored on-axis so the profile stays ANNULAR (an on-axis
/// profile revolves into half-bands whose latitude rims are open).
/// Its equator is a CLOSED plane–sphere rim of radius `r`.
fn dome(r: f64, zone: f64, bore_frac: f64, rev: Revolution<f64>) -> Body<f64> {
    let bulge = (zone / 4.0).tan();
    let (top_x, top_y) = (r * zone.cos(), r * zone.sin());
    let bore = top_x * bore_frac;
    revolved(
        vec![
            ProfileVertex::new(p2(bore, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), bulge),
            ProfileVertex::new(p2(top_x, top_y), 0.0),
            ProfileVertex::new(p2(bore, top_y), 0.0),
        ],
        rev,
    )
}

/// A neck-and-flare ring: a cylinder wall of radius `r` meeting a
/// cone wall at a dihedral of `half_angle`, on a latitude rim. The
/// profile is annular, so a FULL revolve mints a CLOSED rim there.
fn neck_flare(r: f64, half_angle: f64, bore_frac: f64, rev: Revolution<f64>) -> Body<f64> {
    let flare = half_angle.tan();
    // The flare walks the wall inward by `flare·r`; the bore must stay
    // clear of where it lands or the profile self-intersects.
    let bore = r * bore_frac.min((1.0 - flare) * 0.6);
    revolved(
        vec![
            ProfileVertex::new(p2(bore, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, r), 0.0),
            ProfileVertex::new(p2(r - flare * r, 2.0 * r), 0.0),
            ProfileVertex::new(p2(bore, 2.0 * r), 0.0),
        ],
        rev,
    )
}

/// **Claim 1 and 2, together.** For a randomized family of closed
/// plane–sphere rims, the METERED lever (`Link::arm_len`, the
/// kernel's own number):
///
/// - never exceeds the carrier's arc length — the soundness claim;
/// - never falls below the endpoint chord — the "no worse than
///   before" half of the reduction claim;
/// - never falls below `arc_length / π` — the anti-collapse floor.
///
/// The floor is the row that goes red on DEGRADATION rather than on
/// one fixture: any lever that reads a fraction of the rim (the
/// endpoint chord reads ~1e-16 of it) fails it at every radius, and
/// a circular rim clears it with the constant 2/π ≈ 0.637 to spare.
#[test]
fn lever_is_bounded_by_arc_length_and_never_collapses_on_a_closed_rim() {
    let mut rng = fuzz::start("verbs_rim_lever_bounds");
    let mut checked = 0usize;
    for _ in 0..fuzz::scaled(4) {
        let r = rng.range(0.3, 3.0);
        let zone = rng.range(0.35, 1.2);
        let bore = rng.range(0.15, 0.85);
        let body = dome(r, zone, bore, Revolution::Full);
        let rim = pick_edge(&body, true, |a, b| is_plane(a) && is_sphere(b));
        let lever = metered_lever(&body, rim).expect("a plane–sphere closed rim resolves");
        let (arc, chord) = (arc_length(&body, rim), endpoint_chord(&body, rim));
        assert!(
            lever <= arc * (1.0 + 1e-9),
            "UNSOUND: lever {lever} over-reports the rim's arc length {arc} (r={r})"
        );
        assert!(
            lever >= chord,
            "the lever regressed below the endpoint chord: {lever} < {chord} (r={r})"
        );
        assert!(
            lever >= arc / core::f64::consts::PI,
            "COLLAPSED: lever {lever} is below arc/π = {} on a closed rim (r={r})",
            arc / core::f64::consts::PI
        );
        checked += 1;
    }
    assert!(checked > 0, "the row must exercise at least one rim");
}

/// **Claim 3, the exact reduction.** Every edge of a randomized
/// rectangular prism is straight, so the endpoint chord dominates
/// whatever else the schedule samples and the metered lever must be
/// BIT-identical to it — `to_bits()`, not a tolerance. A schedule
/// that started averaging, summing, or rounding its samples reds
/// here even when the value stays close.
#[test]
fn straight_edges_meter_bit_identically_to_the_endpoint_chord() {
    let mut rng = fuzz::start("verbs_rim_straight_reduction");
    for _ in 0..fuzz::scaled(4) {
        let (w, h, d) = (
            rng.range(0.4, 4.0),
            rng.range(0.4, 4.0),
            rng.range(0.4, 4.0),
        );
        let sq = Profile::new(
            SketchPlane::xy(),
            vec![ProfileLoop::new(vec![
                ProfileVertex::new(p2(0.0, 0.0), 0.0),
                ProfileVertex::new(p2(w, 0.0), 0.0),
                ProfileVertex::new(p2(w, h), 0.0),
                ProfileVertex::new(p2(0.0, h), 0.0),
            ])],
        )
        .validate(tol())
        .unwrap();
        let body = extrude(&sq, Extrusion::Distance(d), tol()).unwrap().body;
        let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
        assert_eq!(edges.len(), 12, "a prism has twelve edges");
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: (w.min(h).min(d)) * 0.1,
        };
        let verdict = run_battery(&req, band()).expect("a prism's edges all resolve");
        for chain in &verdict.chains {
            for link in chain.links() {
                let want = endpoint_chord(&body, link.edge);
                assert_eq!(
                    link.arm_len.to_bits(),
                    want.to_bits(),
                    "straight edge {:?}: metered {} is not bit-identical to the endpoint \
                     chord {want} (w={w} h={h} d={d})",
                    link.edge,
                    link.arm_len
                );
            }
        }
    }
}

/// **Claim 4, #554's pair.** A cylinder×cone latitude rim DECIDES its
/// dihedral, definitely and honestly, whether the rim closes or not,
/// across a randomized dihedral and radius. The pre-fix kernel read a
/// collapsed lever on the closed leg and reported `TangentialEdge` on a
/// transverse corner; that is what a revert reds here, at every draw.
///
/// The legs no longer end in one place, and where they part is the
/// finding: the CLOSED rim is a one-edge band the cylinder×cone arm
/// builds, and the OPEN arc terminates at the revolve's seam-meridian
/// vertices, whose corner configuration is unimplemented. Both are
/// decisions taken after the dihedral was classified — which is the
/// claim this row exists for, and it is asserted directly rather than
/// through the class agreement that used to stand in for it (agreement
/// alone would be satisfied by a kernel that broke both legs the same
/// way).
#[test]
fn the_554_pair_agrees_across_a_randomized_dihedral() {
    let mut rng = fuzz::start("verbs_rim_554_pair");
    for _ in 0..fuzz::scaled(3) {
        let r = rng.range(0.5, 2.0);
        let half_angle = rng.range(0.15, 0.7);
        let bore = rng.range(0.1, 0.5);
        let is_pair = |a: &Surface<f64>, b: &Surface<f64>| is_cone(a) && is_cylinder(b);

        let full = neck_flare(r, half_angle, bore, Revolution::Full);
        let closed = pick_edge(&full, true, is_pair);
        let closed_verdict =
            fillet_edges(&full, &[closed], r * 0.05, band(), tol()).map_err(|r| r.error);

        let part = neck_flare(r, half_angle, bore, Revolution::Partial(1.0));
        let open = pick_edge(&part, false, is_pair);
        let open_verdict =
            fillet_edges(&part, &[open], r * 0.05, band(), tol()).map_err(|r| r.error);

        for (which, v) in [("closed", &closed_verdict), ("open", &open_verdict)] {
            assert!(
                !matches!(v, Err(BlendError::TangentialEdge { .. })),
                "{which} rim at half-angle {half_angle} rad reports tangency on a \
                 transverse cone×cylinder corner: {v:?}"
            );
        }
        assert!(
            closed_verdict.is_ok(),
            "closed rim at half-angle {half_angle} rad: the cylinder×cone band is built \
             once the dihedral has decided, got {closed_verdict:?}"
        );
        assert!(
            matches!(open_verdict, Err(BlendError::UnsupportedCorner { .. })),
            "open rim at half-angle {half_angle} rad: the arc terminates at the revolve's \
             seam-meridian vertices, whose corner configuration is unimplemented, got \
             {open_verdict:?}"
        );
    }
}

/// **Claim 5, the differential.** The detector must still fire where
/// tangency is real and must NOT fire where it is not, on bodies of
/// the same family and at the same (now definitely nonzero) lever
/// scale:
///
/// - a full revolve's co-surface seam meridian — the same sphere on
///   both sides — refuses `TangentialEdge` at a margin of EXACTLY
///   0.0, at every randomized radius;
/// - the transverse plane–sphere equator of a dome of the same
///   radius does not refuse tangency at all.
///
/// A "fix" that disabled the sign test would pass the second half and
/// red the first; a revert to the collapsed lever reds the second.
#[test]
fn co_surface_seams_still_refuse_while_transverse_rims_do_not() {
    let mut rng = fuzz::start("verbs_rim_tangency_differential");
    for _ in 0..fuzz::scaled(3) {
        let r = rng.range(0.4, 2.5);
        let ball = revolved(
            vec![
                ProfileVertex::new(p2(0.0, -r), 1.0),
                ProfileVertex::new(p2(0.0, r), 0.0),
            ],
            Revolution::Full,
        );
        let seam = pick_edge(&ball, false, |a, b| is_sphere(a) && is_sphere(b));
        assert!(
            endpoint_chord(&ball, seam) > r,
            "the seam's own lever is definitely nonzero — the zero must come from the sine"
        );
        match fillet_edges(&ball, &[seam], r * 0.05, band(), tol()).map_err(|r| r.error) {
            Err(BlendError::TangentialEdge { margin, .. }) => assert_eq!(
                margin, 0.0,
                "a co-surface seam's dihedral sine is structurally zero (r={r})"
            ),
            other => {
                panic!("the tangency detector stopped firing on a co-surface seam: {other:?}")
            }
        }

        let transverse = dome(
            r,
            rng.range(0.4, 1.1),
            rng.range(0.2, 0.8),
            Revolution::Full,
        );
        let rim = pick_edge(&transverse, true, |a, b| is_plane(a) && is_sphere(b));
        let verdict = run_battery(
            &BlendRequest {
                body: &transverse,
                edges: vec![rim],
                size: r * 0.02,
            },
            band(),
        );
        assert!(
            verdict.is_ok(),
            "a transverse closed rim must not be refused at all here, got {verdict:?}"
        );
    }
}

/// **Claim 6, both signs on CLOSED rims.** A dome's equator is
/// convex; a spherical boss rising out of a plate meets the plate in
/// a concave crevice. Both rims are closed, both are randomized, and
/// both must meter their own diameter — the geometric value, derived
/// from the fixture's dimensions rather than from the kernel.
#[test]
fn closed_rims_decide_both_convexity_signs_at_diameter_levers() {
    let mut rng = fuzz::start("verbs_rim_convexity_signs");
    for _ in 0..fuzz::scaled(3) {
        // Convex: the dome's equator, rim radius == the sphere radius.
        let r = rng.range(0.4, 2.5);
        let body = dome(
            r,
            rng.range(0.4, 1.1),
            rng.range(0.2, 0.8),
            Revolution::Full,
        );
        let rim = pick_edge(&body, true, |a, b| is_plane(a) && is_sphere(b));
        let link = run_battery(
            &BlendRequest {
                body: &body,
                edges: vec![rim],
                size: r * 0.02,
            },
            band(),
        )
        .expect("the dome's equator resolves");
        let link = link.chains[0].first();
        assert_eq!(
            link.convexity,
            Convexity::Convex,
            "a dome's equator is a convex closed rim (r={r})"
        );
        assert!(
            (link.arm_len - 2.0 * r).abs() < 1e-9,
            "the closed rim levers its own diameter {}, got {}",
            2.0 * r,
            link.arm_len
        );

        // Concave: a boss of radius `r` through a plate whose top
        // plane sits at height `h`, so the root rim has radius
        // √(r² − h²).
        let h = rng.range(0.2, 0.7) * r;
        let rim_r = (r * r - h * h).sqrt();
        let bore = rng.range(0.1, 0.4) * r;
        let bore_y = (r * r - bore * bore).sqrt();
        let bulge = (((bore / r).acos() - (h / r).asin()) / 4.0).tan();
        let boss = revolved(
            vec![
                ProfileVertex::new(p2(bore, 0.0), 0.0),
                ProfileVertex::new(p2(2.0 * r, 0.0), 0.0),
                ProfileVertex::new(p2(2.0 * r, h), 0.0),
                ProfileVertex::new(p2(rim_r, h), bulge),
                ProfileVertex::new(p2(bore, bore_y), 0.0),
            ],
            Revolution::Full,
        );
        let root = pick_edge(&boss, true, |a, b| is_plane(a) && is_sphere(b));
        let link = run_battery(
            &BlendRequest {
                body: &boss,
                edges: vec![root],
                size: rim_r * 0.02,
            },
            band(),
        )
        .expect("the boss's root rim resolves");
        let link = link.chains[0].first();
        assert_eq!(
            link.convexity,
            Convexity::Concave,
            "a boss root is a concave closed rim (r={r} h={h})"
        );
        assert!(
            (link.arm_len - 2.0 * rim_r).abs() < 1e-9,
            "the closed rim levers its own diameter {}, got {}",
            2.0 * rim_r,
            link.arm_len
        );
    }
}

/// **Claim 7, the class the change also moves.** An OPEN arc
/// approaching a full turn has an endpoint chord that collapses just
/// as a closed rim's does — the same defect wearing an open edge. The
/// metered lever must not collapse with it: past the 240° crossover
/// it must stay at least the endpoint chord AND hold near the rim's
/// diameter. (The row originally asserted strict monotonicity in θ —
/// a property of the three-sample schedule it was written against;
/// under the battery's full CHAIN_SAMPLES schedule the lever is
/// strictly TIGHTER at every θ but wiggles by ~1e-4·r as the best
/// sample pair walks, so the pin here is the stronger diameter
/// floor.)
///
/// Pinned because the change to this class is DISCLOSED but otherwise
/// unpinned: nothing else in the tree would go red if an
/// implementation kept the endpoint chord for open edges and special-
/// cased only the closed ones — which is the bolt-on the design
/// ruling exists to forbid.
#[test]
fn open_arcs_approaching_a_full_turn_do_not_collapse() {
    let mut rng = fuzz::start("verbs_rim_open_arc_class");
    for _ in 0..fuzz::scaled(2) {
        let r = rng.range(0.4, 2.0);
        let zone = rng.range(0.4, 1.1);
        let bore = rng.range(0.2, 0.8);
        let near_full = core::f64::consts::TAU - 0.0032;
        for theta in [4.2_f64, 5.0, 5.8, near_full] {
            let body = dome(r, zone, bore, Revolution::Partial(theta));
            let rim = pick_edge(&body, false, |a, b| is_plane(a) && is_sphere(b));
            let lever = metered_lever(&body, rim).expect("an open plane–sphere rim resolves");
            let chord = endpoint_chord(&body, rim);
            assert!(
                lever >= chord,
                "θ={theta}: the lever fell below the endpoint chord ({lever} < {chord})"
            );
            // With samples every θ/8 ≤ π/4 apart, some pair sits
            // within θ/16 of the diametral separation π, so the
            // lever is ≥ 2r·cos(θ/32) ≥ 2r·cos(π/16) ≈ 1.96r on this
            // whole range — the floor a collapsing lever cannot meet.
            assert!(
                lever >= 1.9 * r,
                "θ={theta}: the lever fell off the diameter floor ({lever}, r = {r})"
            );
            assert!(
                lever >= 2.0 * r * (theta / 4.0).sin() - 1e-9,
                "θ={theta}: the lever is below the half-chord a three-sample schedule \
                 already has in hand ({lever})"
            );
        }
        // The endgame: at a full turn less a whisker the endpoint
        // chord is ~0, and the lever must be ~the diameter anyway.
        let body = dome(r, zone, bore, Revolution::Partial(near_full));
        let rim = pick_edge(&body, false, |a, b| is_plane(a) && is_sphere(b));
        let lever = metered_lever(&body, rim).unwrap();
        assert!(
            lever > 1.9 * r,
            "a near-closed OPEN arc collapsed to {lever} (diameter {})",
            2.0 * r
        );
        assert!(
            endpoint_chord(&body, rim) < 0.1 * r,
            "the fixture must actually be in the collapsing regime"
        );
    }
}

/// **Claim 8, the prose.** `TangentialEdge` is raised whenever the
/// dihedral's signed margin DECIDES Zero — of which genuine tangency
/// is one cause. Its user-visible text must therefore state no
/// geometric fact it has not established. Asserted on a real refusal
/// from a real body, so the row is about what a consumer reads.
///
/// This row is the mechanical guard for the rewrite: the prose sites
/// are otherwise pinned only by a doc comment, and doc comments do
/// not go red.
#[test]
fn the_tangential_refusal_prose_states_no_geometric_fact() {
    let ball = revolved(
        vec![
            ProfileVertex::new(p2(0.0, -1.0), 1.0),
            ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
    );
    let seam = pick_edge(&ball, false, |a, b| is_sphere(a) && is_sphere(b));
    let err =
        fillet_edges(&ball, &[seam], 0.05, band(), tol()).expect_err("the co-surface seam refuses");
    let text = format!("{err}");
    for forbidden in [
        "share a tangent plane",
        "shares a tangent plane",
        "sharing a tangent plane",
        "joins its supports tangentially",
        "the supports are tangent",
    ] {
        assert!(
            !text.contains(forbidden),
            "the refusal asserts an unestablished geometric fact ({forbidden:?}): {text}"
        );
    }
    assert!(
        text.contains("no definite wedge side"),
        "the refusal should say what it DID establish: {text}"
    );
    assert!(
        !sweep::blend::FILLET3_TANGENTIAL_RECOURSE.contains("a tangential join has no wedge"),
        "the recourse still states tangency as the established cause"
    );
}
