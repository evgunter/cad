//! **BLEND-14 review probes (lane r1)** — rows the unit's suite
//! (`contact_edge_must_carry.rs`) does not hold, each an ordinary test
//! the fix pass may adopt.
//!
//! What they draw, beyond the unit's rows:
//!
//! - the in-band verdict reached through the OTHER door the surgery
//!   has — the closed rim's annulus band (`ContactCarrier::Exact`, a
//!   full-period circle whose extent is the carrier diameter), on a
//!   SPHERE support of the band's own convexity: a sphere zone's
//!   equator at `R/r = 10`, scaled so the sphere side's
//!   `(1/r − 1/R)·r²/2` sits at `0.945·Kε` while the plane side's `r/2`
//!   and the headroom are definite — and BOTH clauses of the recourse
//!   followed on that support (enlarge toward `R/2`; scale the feature);
//! - the screened ratio (`R/r = 1.1`) scaled until the clearance
//!   screen's own margin would sit in the band: the mill refuses the
//!   body first (the flat's belly graze in band), so "a definite
//!   negative whatever the scale" is a claim about the screen at
//!   ordinary scale, and the family is unreachable either way;
//! - the filed D-bore residue reproduced through the extrude door, with
//!   the CAUSE asserted beside the refusal: the crease's end vertices
//!   sit on a RING of each cap, which is the loop `chord_site` never
//!   walks;
//! - the corner ball's own arcs on a SLIM WEDGE (the skewed cavity of
//!   `blend4_r1_probes`): the arc's extent `r·θ` is the folded lever
//!   arm, so its margin `θ²·r/2` reaches the in-band verdict at an
//!   ordinary radius with no scaling into the octave, and one step
//!   slimmer reaches the UNDER-DETERMINED verdict — four contact edges
//!   stored as chart images on a tier-3-valid body — which the unit's
//!   algebra (`margin ≥ headroom/2`, an `r_band` lever) calls
//!   unreachable through the door;
//! - the K cost on the die (48 contact edges, corner arcs included):
//!   `48 × 2 × 7` samples of `tangent_second_order`, under `probe`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{EdgeDescription, SurfaceKind};
use geom_core::{Band, MarginDiag, Point2, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::Revolution;
use sweep::blend::{
    BlendError, BlendRefusal, BlendSite, FILLET3_CONTACT_RECOURSE, Filleted, fillet_edges,
};
use sweep::test_support::{revolved_about_y, rod_upper_crease, rod_with_flat_at};
use sweep::{Extrusion, extrude};
use topo::query::{self, SurfaceKindSet};
use topo::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the run's linear band")
}

/// How many edges of a body store the intrinsic tangency.
fn intrinsic_edges(body: &Body<f64>) -> usize {
    body.edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(|c| c.description()),
                Some(EdgeDescription::TangentIntersection { .. })
            )
        })
        .count()
}

/// The typed in-band refusal of the must-carry rule, or a panic naming
/// what came instead; returns the deciding station's margin.
fn contact_in_band_margin(result: Result<Filleted<f64>, BlendRefusal>, what: &str) -> f64 {
    match result {
        Ok(out) => panic!(
            "{what}: built, storing {} intrinsic tangencies, where the rule reads in band",
            intrinsic_edges(&out.body)
        ),
        Err(BlendRefusal { error, .. }) => {
            let BlendError::Escalated {
                site: BlendSite::Link { .. },
                source,
            } = &error
            else {
                panic!("{what}: refused, but not as the rule's in-band escalation: {error}");
            };
            assert_eq!(source.predicate, Some("tangent_second_order"), "{what}");
            let MarginDiag::Value(m) = source.margin else {
                panic!("{what}: the deciding station's margin is a value, got {source:?}");
            };
            let b = band();
            assert!(
                m > b.zero() && m < b.escalate(),
                "{what}: the escalated margin {m:e} lies strictly inside ({:e}, {:e})",
                b.zero(),
                b.escalate()
            );
            assert!(
                error.to_string().contains(FILLET3_CONTACT_RECOURSE),
                "{what}: the refusal carries the contact recourse.\n  got: {error}"
            );
            m
        }
    }
}

// ---------------------------------------------------------------
// The annulus door: a sphere support of the band's own convexity.
// ---------------------------------------------------------------

/// **A sphere zone on a flat base annulus**, of sphere radius `big_r`:
/// the base `(0.2R, 0) → (R, 0)`, a 60° arc of the sphere up from the
/// equator, and a top annulus back to the on-axis bore at `0.2R` —
/// the dome's shape with an arc whose sagitta (`0.13·R`) and whose
/// top segment (`0.3·R`) stay definite at a few `K·ε`, where the
/// dome fixture's own 45° arc escalates `segment_straightness`.
fn sphere_zone_on_base(big_r: f64) -> Body<f64> {
    let (c, s) = (60f64.to_radians().cos(), 60f64.to_radians().sin());
    let bulge = (60f64.to_radians() / 4.0).tan();
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.2 * big_r, 0.0), 0.0),
            ProfileVertex::new(Point2::new(big_r, 0.0), bulge),
            ProfileVertex::new(Point2::new(big_r * c, big_r * s), 0.0),
            ProfileVertex::new(Point2::new(0.2 * big_r, big_r * s), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The zone's equator — its ONE plane–sphere rim of the larger radius
/// (the top rim is the other plane–sphere circle).
fn equator(body: &Body<f64>) -> EdgeKey {
    let mut rims: Vec<(f64, EdgeKey)> = query::all_edges(body)
        .into_iter()
        .filter(|&k| {
            query::edge_adjacent_matches(
                body,
                k,
                SurfaceKindSet::just(SurfaceKind::Plane),
                SurfaceKindSet::just(SurfaceKind::Sphere),
            )
        })
        .map(|k| {
            let c = body
                .get_curve_geom(body.get_edge(k).unwrap().curve)
                .and_then(|g| g.certified())
                .expect("a certified rim");
            let geom::Curve3::Circle { radius, .. } = *c.carrier() else {
                panic!("a rim is a circle");
            };
            (radius, k)
        })
        .collect();
    assert_eq!(rims.len(), 2, "the zone's two plane–sphere rims");
    rims.sort_by(|a, b| a.0.total_cmp(&b.0));
    rims[1].1
}

/// The difference branch's closed form on a sphere support.
fn sphere_side_margin(r: f64, big_r: f64) -> f64 {
    (1.0 / r - 1.0 / big_r) * r * r * 0.5
}

/// **A sphere-supported rim scaled into the band's octave refuses
/// typed at the annulus door, and both clauses of the recourse are
/// followable on it.** `R/r = 10`, `r = 2.1·Kε`: the sphere-side contact
/// edge's margin `(1 − 1/10)·r/2 = 0.945·Kε` is in band while the plane
/// side's `r/2 = 1.05·Kε` and the headroom `(1 − 1/10)·r = 1.9·Kε` are
/// definite. Then: "enlarge the radius toward `R/2`" — `r' = 0.3·R`
/// reads `0.105·R = 2.2·Kε`, definite, and builds with both contact
/// edges intrinsic (the peak itself is NOT reachable on a plane–sphere
/// rim: at `r = 0.4·R` the spine's own regularity margin
/// `(1 − r/ρ_spine)·r` is already in band, and at `R/2` the ball's
/// centre sits on the axis); and "blend a larger feature" — the zone
/// and its blend scaled by two build the same way.
#[test]
fn r1_a_sphere_supported_rim_in_the_octave_refuses_typed_at_the_annulus_door() {
    let b = band();
    let r = 2.1 * b.escalate();
    let big_r = 10.0 * r;
    let body = sphere_zone_on_base(big_r);
    let rim = equator(&body);
    let m = contact_in_band_margin(
        fillet_edges(&body, &[rim], r, tol()),
        "the zone's equator at R/r = 10, r = 2.1·Kε",
    );
    let predicted = sphere_side_margin(r, big_r);
    assert!(
        ((m - predicted) / predicted).abs() < 1e-6,
        "the deciding station reads the closed form: {m:e} vs {predicted:e}"
    );
    // Clause one, on the sphere support: enlarge toward R/2.
    let r1 = 0.3 * big_r;
    assert!(sphere_side_margin(r1, big_r) > b.escalate());
    let out = fillet_edges(&body, &[rim], r1, tol())
        .unwrap_or_else(|e| panic!("the zone at r = 0.3·R builds: {e}"));
    assert_eq!(
        intrinsic_edges(&out.body),
        2,
        "both contact edges intrinsic"
    );
    // Clause two: twice the feature.
    let body = sphere_zone_on_base(2.0 * big_r);
    let rim = equator(&body);
    let out = fillet_edges(&body, &[rim], 2.0 * r, tol())
        .unwrap_or_else(|e| panic!("the zone at twice the scale builds: {e}"));
    assert_eq!(
        intrinsic_edges(&out.body),
        2,
        "both contact edges intrinsic"
    );
}

// ---------------------------------------------------------------
// The ruled door: the screened ratio, scaled.
// ---------------------------------------------------------------

/// **The screened ratio cannot be scaled into the band at all.** At
/// `R/r = 1.1` the clearance screen's margin is `≈ −0.27·r` (the unit
/// measured `−2.717e-2` at `r = 0.1`), so a member whose screen reading
/// would sit in the band has `r ≈ 1.8·Kε` — and there the mill refuses
/// first: the flat grazes the cylinder at `R − r = 0.1·r`, in band
/// (`split_conic_belly_graze`). The family is out of the rule's reach
/// at every scale, but the unit's "definite negative whatever the
/// scale" describes the screen at ordinary scale only: at this one the
/// body is not buildable.
#[test]
fn r1_the_screened_ratio_scaled_into_the_band_is_refused_at_the_mill() {
    let b = band();
    let r = 0.5 * b.escalate() / 0.2717;
    match rod_with_flat_at(1.1 * r, r, 10.0 * r, 2.2 * r, tol()) {
        Err(text) => assert!(
            text.contains("split_conic_belly_graze"),
            "the mill escalates the belly graze: {text}"
        ),
        Ok(body) => {
            let crease = rod_upper_crease(&body);
            panic!(
                "the rod mills at this scale; the door then says {:?}",
                fillet_edges(&body, &[crease], r, tol()).err()
            );
        }
    }
}

// ---------------------------------------------------------------
// The filed residue: a D-bore's crease at the ruled door.
// ---------------------------------------------------------------

/// A block `[−1, 1]² × [0, len]` with a D-shaped through-hole of
/// radius `big_r` and flat at `x = flat`, as one profile with a ring.
fn block_with_d_bore(big_r: f64, flat: f64, len: f64) -> Body<f64> {
    let block = ProfileLoop::new(
        [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
            .collect(),
    );
    let y = (big_r * big_r - flat * flat).sqrt();
    let theta = 2.0 * (core::f64::consts::PI - y.atan2(flat));
    let bulge = (theta / 4.0).tan();
    let hole = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(flat, y), bulge),
        ProfileVertex::new(Point2::new(flat, -y), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![block, hole])
        .validate(tol())
        .expect("the D-bore profile validates");
    extrude(&profile, Extrusion::Distance(len), tol())
        .expect("the D-bore extrudes")
        .body
}

/// **The D-bore's concave crease refuses `BodyNotIntact` at the ruled
/// door, and the cause is the cap's RING.** The crease's two end
/// vertices each lie on a loop that is a ring of a face that is not
/// one of the crease's supports — the cap — which is the loop
/// `chord_site` (keyed on the cap's `outer`) never walks. The body
/// itself is tier-3 valid.
#[test]
fn r1_the_d_bore_crease_refuses_body_not_intact_because_its_rim_is_a_ring_of_the_cap() {
    let body = block_with_d_bore(0.2, 0.1, 1.0);
    topo::validate(&body).expect("the D-bored block holds together");
    let crease = rod_upper_crease(&body);
    let e = body.get_edge(crease).unwrap();
    let supports = [
        body.get_loop(body.get_half_edge(e.he_plus).unwrap().parent_loop)
            .unwrap()
            .face,
        body.get_loop(body.get_half_edge(e.he_minus).unwrap().parent_loop)
            .unwrap()
            .face,
    ];
    let start = body.get_half_edge(e.he_plus).unwrap().start;
    let end = body.half_edge_end(e.he_plus).unwrap();
    for v in [start, end] {
        let on_a_cap_ring = body.half_edges().any(|(_, h)| {
            if h.start != v {
                return false;
            }
            let l = h.parent_loop;
            let f = body.get_loop(l).unwrap().face;
            !supports.contains(&f) && body.get_face(f).unwrap().rings.contains(&l)
        });
        assert!(
            on_a_cap_ring,
            "the crease's end {v:?} sits on a ring of its cap"
        );
    }
    match fillet_edges(&body, &[crease], 0.1, tol()) {
        Err(BlendRefusal {
            error: BlendError::BodyNotIntact { detail, .. },
            ..
        }) => assert!(
            detail.contains("outer cycle"),
            "the refusal names the outer-cycle walk: {detail}"
        ),
        other => panic!("expected the residue's BodyNotIntact, got {other:?}"),
    }
}

// ---------------------------------------------------------------
// The K cost on the die.
// ---------------------------------------------------------------

/// **The die's 48 contact edges — 24 trimlines and 24 corner arcs —
/// each spend the schedule's interior once in the rule and once in
/// the certificate**: `48 × 2 × 7` samples of `tangent_second_order`,
/// and nothing else on the carve samples that predicate.
#[cfg(feature = "probe")]
#[test]
fn r1_the_die_spends_the_rules_stations_once_per_contact_edge_beside_the_certificates() {
    use geom_brep::CERT_SAMPLES;
    use geom_core::k_stats::{self, Probe};
    let interior = usize::try_from(CERT_SAMPLES - 2).expect("a small count");
    // The unit die through the extrude door at the `Probe` scalar.
    let p = |x: f64, y: f64| ProfileVertex::new(Point2::new(Probe(x), Probe(y)), Probe(0.0));
    let square = ProfileLoop::new(vec![p(0.0, 0.0), p(1.0, 0.0), p(1.0, 1.0), p(0.0, 1.0)]);
    let profile = Profile::new(SketchPlane::<Probe>::xy(), vec![square])
        .validate(tol())
        .expect("the die's profile validates");
    let die: Body<Probe> = extrude(&profile, Extrusion::Distance(Probe(1.0)), tol())
        .expect("the die extrudes")
        .body;
    let edges = query::all_edges(&die);
    k_stats::start_recording();
    let out = fillet_edges(&die, &edges, Probe(0.15), tol())
        .unwrap_or_else(|e| panic!("the die carves: {e}"));
    let spent = k_stats::take_samples()
        .iter()
        .filter(|s| s.predicate == "tangent_second_order")
        .count();
    let contact = out
        .body
        .edges()
        .filter(|(_, e)| {
            matches!(
                out.body
                    .get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(|c| c.description()),
                Some(EdgeDescription::TangentIntersection { .. })
            )
        })
        .count();
    assert_eq!(contact, 48, "the die's 24 trimlines and 24 corner arcs");
    assert_eq!(spent, contact * 2 * interior);
}

// ---------------------------------------------------------------
// The corner ball's own arcs on a slim wedge: the EXTENT lever.
// ---------------------------------------------------------------

/// The skewed vented cavity of `blend4_r1_probes` — block `[0,4]³`, a
/// parallelogram cavity prism of side `1.2` and skew `theta` over
/// `z ∈ [1, 3]`, a round vent from its centroid — with its twelve
/// concave edges, the whole body scaled by `scale`. Re-posed here
/// because the pose is the point: at a slim skew the corner ball's
/// arc against the band subtends the wedge angle, so the arc's EXTENT
/// `r·θ` — not `r_band` — is the folded lever arm, and the
/// second-order margin is `θ²·r/2`.
pub(crate) fn skewed_cavity_edges(theta: f64, scale: f64) -> (Body<f64>, Vec<EdgeKey>) {
    use crate::common::cavity::{cut, edges_with_corners, prism, rod};
    let p = |x: f64, y: f64| Point2::new(x * scale, y * scale);
    let s = 1.2;
    let (ax, ay) = (1.0, 1.0);
    let (dx, dy) = (s * theta.cos(), s * theta.sin());
    let quad = [
        p(ax, ay),
        p(ax + s, ay),
        p(ax + s + dx, ay + dy),
        p(ax + dx, ay + dy),
    ];
    let block = prism(
        &[p(0.0, 0.0), p(4.0, 0.0), p(4.0, 4.0), p(0.0, 4.0)],
        0.0,
        4.0 * scale,
    );
    let centroid = Point2::new(
        (quad[0].x + quad[1].x + quad[2].x + quad[3].x) / 4.0,
        (quad[0].y + quad[1].y + quad[2].y + quad[3].y) / 4.0,
    );
    // A narrower vent than `blend4_r1_probes`' (`0.12·sin θ`, not
    // `0.45·sin θ`): the slimmer poses below leave the cavity's own
    // walls `0.48·sin θ` clear of it, so the clearance screen answers
    // for the corner arcs and not for the vent.
    let vent = rod(
        centroid,
        (theta.sin() * 0.12).min(0.25) * scale,
        2.5 * scale,
        5.0 * scale,
    );
    let cavity = prism(&quad, 1.0 * scale, 3.0 * scale);
    let vented = cut("vent", &block, &vent);
    let body = cut("cavity", &vented, &cavity);
    let near = 1e-9 * scale;
    let edges = edges_with_corners(&body, |q: geom_core::Point3<f64>| {
        ((q.z - 1.0 * scale).abs() < near || (q.z - 3.0 * scale).abs() < near)
            && quad
                .iter()
                .any(|c| (q.x - c.x).abs() < near && (q.y - c.y).abs() < near)
    });
    assert_eq!(edges.len(), 12, "the cavity's twelve concave edges");
    (body, edges)
}

/// Contact edges of a carved body stored as a non-seam chart image —
/// the rule's UNDER-DETERMINED description.
pub(crate) fn chart_contact_edges(body: &Body<f64>) -> usize {
    body.edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(|c| c.description()),
                Some(EdgeDescription::Chart(c)) if !c.seam
            )
        })
        .count()
}

/// **The corner ball's arcs reach BOTH non-determinate verdicts at an
/// ordinary radius, through the public door, with no scaling into
/// the band's octave.** At a wedge of skew `θ` the corner arc between
/// the band and the ball is `r·θ` long, and that extent is the folded
/// lever arm, so the arc's margin is `(2r·sin(θ/2))²/(2r) = 2r·sin²(θ/2)
/// ≈ θ²·r/2` — independent of the support's curvature and far below
/// `r/2`. The radius is `8.7e-5` on the 4 m block at the default ε
/// (the pose `blend4_r1_probes::p2_slim_skews…` already carves at
/// `0.2°`); the WHOLE fixture scales with `ε` across the rows so every
/// margin, the band and the f64 floor keep their ratios (at `1e-12`
/// the unscaled block refuses at the attachment gate on a first-order
/// residual, which is the floor and not this rule); and the skew is
/// chosen from the band: at `θ² r/2 = 0.75·Kε` the request refuses
/// typed with the contact recourse and the deciding station reads the
/// closed form; at `θ² r/2 = 0.5·ε` the request BUILDS, tier-3 valid,
/// with four contact edges stored as chart images — the
/// `UnderDetermined → Chart` arm — this row is that arm's pin.
#[test]
fn r1_a_slim_wedges_corner_arcs_reach_the_in_band_and_under_determined_verdicts() {
    let b = band();
    let scale = b.zero() / 1e-9;
    let r = 8.7e-5 * scale;
    // In band.
    let theta = (1.5 * b.escalate() / r).sqrt();
    let (body, edges) = skewed_cavity_edges(theta, scale);
    let m = contact_in_band_margin(
        fillet_edges(&body, &edges, r, tol()),
        &format!("the skewed cavity at θ = {theta:e}, r = {r:e}"),
    );
    let predicted = 2.0 * r * (0.5 * theta).sin().powi(2);
    assert!(
        ((m - predicted) / predicted).abs() < 1e-6,
        "the deciding station reads the corner arc's closed form: {m:e} vs {predicted:e}"
    );
    // Under-determined: the same pose, slimmer.
    let theta = (b.zero() / r).sqrt();
    let (body, edges) = skewed_cavity_edges(theta, scale);
    let out = fillet_edges(&body, &edges, r, tol())
        .unwrap_or_else(|e| panic!("the slimmer skew at θ = {theta:e} builds: {e}"));
    assert_eq!(
        topo::validate_geometric(&out.body, tol()),
        Ok(()),
        "the carve with chart-described corner arcs is tier-3 valid"
    );
    assert_eq!(
        (intrinsic_edges(&out.body), chart_contact_edges(&out.body)),
        (44, 6),
        "four of the 48 contact edges store the conventional chart image (the vent's two \
         chart edges are the boolean's)"
    );
}
