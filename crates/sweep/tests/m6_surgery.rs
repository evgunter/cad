//! **The composition surgery** (M6 unit 1): the in-place edge-blend
//! op and THE COMPOSED DIE — filleted blank + 21 pips + filleted pip
//! rims in ONE body, walking every verb of shape (v)'s criterion on a
//! single solid.
//!
//! Geometry constants are `m5_pr12_die.rs`'s; the fixtures reuse its
//! builders byte-for-byte so the two suites meter the same die.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::PI;

use crate::common::approx::band;
use geom::SurfaceKind;
use geom_core::Tol;
use geom_core::Vec3;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{ball_poled, cube, finished};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::query::{self, SurfaceKindSet};
use topo::readback::euler_counts;
use topo::{AtRestBody, Body, EdgeKey};

/// The die's side, meters.
const DIE_L: f64 = 1.0;
/// The blend radius, meters.
const DIE_R: f64 = 0.12;
/// The pip balls' radius, meters.
const PIP_R: f64 = 0.09;
/// How deep each pip ball dips into its face, meters.
const PIP_H: f64 = 0.05;
/// Pip spacing from the face centre, meters.
const PIP_D: f64 = 0.22;
/// The pip-rim blend radius, meters.
const RIM_R: f64 = 0.02;

fn layout(n: u32) -> Vec<(f64, f64)> {
    let c = vec![(0.0, 0.0)];
    let diag = vec![(-1.0, -1.0), (1.0, 1.0)];
    let anti = vec![(-1.0, 1.0), (1.0, -1.0)];
    let sides = vec![(-1.0, 0.0), (1.0, 0.0)];
    match n {
        1 => c,
        2 => diag,
        3 => [diag.clone(), c].concat(),
        4 => [diag.clone(), anti.clone()].concat(),
        5 => [diag.clone(), anti.clone(), c].concat(),
        _ => [diag, anti, sides].concat(),
    }
}

fn pip_placements() -> Vec<(Vec3<f64>, Vec3<f64>)> {
    let h = DIE_L / 2.0;
    type Face = (u32, Vec3<f64>, Vec3<f64>, Vec3<f64>);
    let faces: [Face; 6] = [
        (
            1,
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            6,
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            2,
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        (
            5,
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        (
            3,
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            4,
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
    ];
    let mut out = Vec::new();
    for (n, normal, ex, ey) in faces {
        let base = Vec3::new(h, h, h) + normal * (h + (PIP_R - PIP_H));
        for (u, v) in layout(n) {
            out.push((base + ex * (u * PIP_D) + ey * (v * PIP_D), normal));
        }
    }
    out
}

fn pip_tool() -> AtRestBody<f64> {
    let tol = Tol::witness();
    let pip =
        |c: Vec3<f64>, n: Vec3<f64>| finished("a pip ball", ball_poled(PIP_R, c, n, tol), tol);
    let places = pip_placements();
    let mut tool = pip(places[0].0, places[0].1);
    for (c, n) in &places[1..] {
        tool = boolean_op_with(
            BooleanOp::Union,
            &tool,
            &pip(*c, *n),
            &topo::BooleanInputs::default(),
            SweepStrategy::Realized,
            Tol::witness(),
        )
        .unwrap_or_else(|e| {
            panic!(
                "assembling the pip tool: {e_at}",
                e_at = e.render(geom_brep::recourse::Reading::Build)
            )
        })
        .body()
        .expect("a body")
        .body
        .clone();
    }
    tool
}

fn subtract(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Body<f64> {
    let out = boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &topo::BooleanInputs::default(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .unwrap_or_else(|e| {
        panic!(
            "pip subtraction: {e_at}",
            e_at = e.render(geom_brep::recourse::Reading::Build)
        )
    });
    out.body().expect("a body").body.clone().into_body()
}

/// The pipped cube and its twelve surviving box edges.
fn pipped_and_box_edges() -> (Body<f64>, Vec<EdgeKey>) {
    let cube0 = finished(
        "the cube",
        cube::<f64>(DIE_L, Tol::witness()),
        Tol::witness(),
    );
    let box_edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    let pipped = subtract(&cube0, &pip_tool());
    let surviving: Vec<_> = box_edges
        .into_iter()
        .filter(|k| pipped.get_edge(*k).is_some())
        .collect();
    assert_eq!(surviving.len(), 12, "every box edge survives the pips");
    (pipped, surviving)
}

/// The rim edges of a body: every edge whose two adjacent faces are a
/// plane and a sphere.
fn rim_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|&k| {
            query::edge_adjacent_matches(
                body,
                k,
                SurfaceKindSet::just(SurfaceKind::Plane),
                SurfaceKindSet::just(SurfaceKind::Sphere),
            )
        })
        .collect()
}

/// Deliberately NOT `common::oracles::rounded_box_volume`: the die
/// family spells the twelve quarter-cylinders as `12·(πr²/4)·core`
/// where that form sums them as `3πlr²`. At `(1.0, 0.12)` the two are
/// bit-identical, so this is conservatism, not a bit-level necessity —
/// see `common::oracles`' module doc for the measurement.
///
/// The blank's closed-form volume (Steiner: core + 6 slabs + 12
/// quarter-cylinders + 8 octants).
fn blank_volume() -> f64 {
    let core = DIE_L - 2.0 * DIE_R;
    core.powi(3)
        + 6.0 * DIE_R * core.powi(2)
        + 12.0 * (PI * DIE_R * DIE_R / 4.0) * core
        + (4.0 / 3.0) * PI * DIE_R.powi(3)
}

/// A spherical cap of height `h` off a radius-`r` ball.
fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

/// **The extra material one rim-torus fillet removes**, beyond the
/// pip cap itself — closed form, derived by Pappus over the removed
/// cross-section in the meridian half-plane.
///
/// Frame: the face plane is `z = 0` with material below; the pip
/// ball (radius `R`) is centred on the axis at `z = d`, `d = R − H`.
/// The rolling ball (radius `r`) is tangent to the plane from below
/// and externally tangent to the pip ball, so its centre traces the
/// spine circle of radius `s = √((R+r)² − (d+r)²)` at `z = −r`. The
/// removed region's cross-section is the curvilinear triangle with
/// vertices
///
/// - `P_rim = (ρ_rim, 0)`, `ρ_rim = √(R² − d²)` (the old rim),
/// - `T_p = (s, 0)` (the tangency on the plane),
/// - `T_s = (s·k, d − (d+r)·k)`, `k = R/(R+r)` (on the pip ball),
///
/// bounded by the plane segment, the fillet-ball arc (excluded disk:
/// centre `(s, −r)` radius `r`) and the pip-ball arc (excluded disk:
/// centre `(0, d)` radius `R`). The two disks are tangent at `T_s`
/// and each meets the triangle only in its own circular segment, so
///
/// `V = 2π·(M(triangle) − M(seg_fillet) − M(seg_pip))`,
///
/// with `M = ∬ ρ dA` (first moment about the axis): the triangle's
/// by centroid, each segment's as (sector − centre-triangle) in
/// closed form.
fn rim_fillet_extra(big_r: f64, h: f64, r: f64) -> f64 {
    let d = big_r - h;
    let s = ((big_r + r).powi(2) - (d + r).powi(2)).sqrt();
    let rho_rim = (big_r * big_r - d * d).sqrt();
    let k = big_r / (big_r + r);
    let (tsx, tsz) = (s * k, d - (d + r) * k);

    // First moment of a triangle: area × centroid-ρ.
    let tri_m = |a: (f64, f64), b: (f64, f64), c: (f64, f64)| -> f64 {
        let area2 = (b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1);
        (area2.abs() / 2.0) * ((a.0 + b.0 + c.0) / 3.0)
    };
    // First moment of the circular segment of circle (c, rad) between
    // boundary points p and q, on the minor-arc side: sector minus the
    // centre triangle. Angles from atan2; the sector spans the short
    // way from p to q.
    let seg_m = |c: (f64, f64), rad: f64, p: (f64, f64), q: (f64, f64)| -> f64 {
        let a0 = (p.1 - c.1).atan2(p.0 - c.0);
        let mut a1 = (q.1 - c.1).atan2(q.0 - c.0);
        // Take the short way round.
        if a1 - a0 > PI {
            a1 -= 2.0 * PI;
        } else if a0 - a1 > PI {
            a1 += 2.0 * PI;
        }
        let (lo, hi) = if a0 <= a1 { (a0, a1) } else { (a1, a0) };
        let sector = c.0 * rad * rad * (hi - lo) / 2.0 + rad.powi(3) * (hi.sin() - lo.sin()) / 3.0;
        sector - tri_m(c, p, q)
    };

    let p_rim = (rho_rim, 0.0);
    let t_p = (s, 0.0);
    let t_s = (tsx, tsz);
    let m =
        tri_m(p_rim, t_p, t_s) - seg_m((s, -r), r, t_p, t_s) - seg_m((0.0, d), big_r, p_rim, t_s);
    2.0 * PI * m
}

/// **Door B, open**: the surgery fillets the twelve box edges of the
/// PIPPED cube in place — every ring carried through, tiers 1–3
/// green, closed-form volume, watertight.
#[test]
fn the_pipped_cube_fillets_in_place_with_rings_carried() {
    let (pipped, box_edges) = pipped_and_box_edges();
    let out = fillet_edges(
        &sweep::test_support::at_rest(&pipped, Tol::witness()),
        &box_edges,
        DIE_R,
        Tol::witness(),
    )
    .expect("the surgery fillets the pipped cube");
    let body = topo::test_support::finished("body", out.body, Tol::witness());
    assert_eq!(topo::validate(&body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
    assert_eq!(out.blend_faces.len(), 12);
    assert_eq!(out.corner_faces.len(), 8);
    assert_eq!(out.band_faces.len(), 0);
    // Counts: the blank's 24/48/26 plus 21 untouched pips — each a
    // closed rim, its cap's one seam meridian (a wrap edge from the rim
    // to its tip), the rim vertex and the pole, and one cap face, so per
    // pip V 2 / E 2 / F 1.
    assert_eq!(body.vertices().count(), 24 + 21 * 2);
    assert_eq!(body.edges().count(), 48 + 21 * 2);
    assert_eq!(body.faces().count(), 26 + 21);
    let want = blank_volume() - 21.0 * cap(PIP_R, PIP_H);
    let props = topo::mass_properties(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap();
    assert!(
        (props.enclosure.volume - want).abs() <= 1e-9 * want,
        "volume {} vs closed form {want}",
        props.enclosure.volume
    );
    assert_eq!(props.enclosure.volume_pad, 0.0, "closed forms need no pad");
    let mesh = mesh::tessellate(&body, 5e-3, Tol::witness()).expect("tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// **THE COMPOSED DIE**: blank fillets + 21 pips + 21 rim tori in ONE
/// body — builds, certifies at tier 3, meters its certified volume
/// against the closed form (Steiner blank − 21·(cap + rim-torus
/// extra)) at zero enclosure pad, and tessellates watertight.
///
/// **K-funnel coverage rides along**: the surgery's one new predicate,
/// `fillet3_ring_clearance`, must reach the funnel BY NAME during the
/// composed-die construction (the ring carry-through is decided, not
/// assumed). The verdict log is a `Vec` push per definite outcome
/// inside `decide`, so installing it changes nothing the certification
/// battery measures.
#[test]
fn the_composed_die_certifies_and_tessellates_watertight() {
    use geom_core::k_stats::Bracket;
    // The log must be installed BEFORE the build (it records through
    // `decide` as the surgery runs) and taken straight after, so the
    // certification battery below runs unlogged exactly as it did.
    let bracket = Bracket::open();
    let built = composed_die();
    let log = bracket.finish().verdicts;
    let die = topo::test_support::finished("die", built, Tol::witness());
    assert!(
        log.iter().any(|v| v.predicate == "fillet3_ring_clearance"),
        "K-FUNNEL: fillet3_ring_clearance never reached the funnel"
    );
    assert_eq!(topo::validate(&die), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&die), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&die, Tol::witness()),
        Ok(()),
        "tier 3"
    );
    // Counts: blank 24/48/26 + per pip, the cap one band and its rim one
    // closed edge: V 3 / E 4 / F 2 (the shrunk cap and the band) and one
    // ring (the widened plane hole), so 3 − 4 + 2 − 1 = 0 keeps χ.
    assert_eq!(die.vertices().count(), 24 + 21 * 3);
    assert_eq!(die.edges().count(), 48 + 21 * 4);
    assert_eq!(die.faces().count(), 26 + 21 * 2);
    let want = blank_volume() - 21.0 * (cap(PIP_R, PIP_H) + rim_fillet_extra(PIP_R, PIP_H, RIM_R));
    let props = topo::mass_properties(
        &die,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap();
    assert!(
        (props.enclosure.volume - want).abs() <= 1e-9 * want,
        "composed volume {} vs closed form {want} (diff {})",
        props.enclosure.volume,
        props.enclosure.volume - want
    );
    assert_eq!(props.enclosure.volume_pad, 0.0, "closed forms need no pad");
    let mesh = mesh::tessellate(&die, 5e-3, Tol::witness()).expect("the composed die tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
    let v_mesh = mesh::validate::signed_volume(&mesh);
    assert!(
        v_mesh > 0.0 && ((v_mesh - props.enclosure.volume) / props.enclosure.volume).abs() < 5e-3
    );
}

/// The composed die: pips first (one certified group op), then the
/// twelve box edges in place, then all 21 pip rims as closed chains
/// in ONE further call.
fn composed_die() -> Body<f64> {
    let (pipped, box_edges) = pipped_and_box_edges();
    let blanked = fillet_edges(
        &sweep::test_support::at_rest(&pipped, Tol::witness()),
        &box_edges,
        DIE_R,
        Tol::witness(),
    )
    .expect("the box edges fillet in place")
    .body;
    let rims = rim_edges(&blanked);
    assert_eq!(rims.len(), 21, "21 rims of one closed edge each");
    let out = fillet_edges(
        &sweep::test_support::at_rest(&blanked, Tol::witness()),
        &rims,
        RIM_R,
        Tol::witness(),
    )
    .expect("the rims fillet to tori");
    assert_eq!(out.band_faces.len(), 21, "one torus band per rim");
    out.body.into_body()
}

/// **Bit replay (D9)**: the composed die twice in one process, same
/// volume bits, same entity counts.
#[test]
fn the_composed_die_replays_bit_identically() {
    let a = topo::test_support::finished("a", composed_die(), Tol::witness());
    let b = topo::test_support::finished("b", composed_die(), Tol::witness());
    let (va, vb) = (
        topo::test_support::pinned_volume("the measured body", &a, Tol::witness()).volume,
        topo::test_support::pinned_volume("the measured body", &b, Tol::witness()).volume,
    );
    assert_eq!(va.to_bits(), vb.to_bits(), "volume bits");
    assert_eq!(
        (a.vertices().count(), a.edges().count(), a.faces().count()),
        (b.vertices().count(), b.edges().count(), b.faces().count()),
    );
}

/// **The two-tolerance trio for `fillet3_ring_clearance`** (D4 ¶1
/// addendum: every arm, definite ones included). The margins are the
/// exact closed forms the surgery derives; here the decision function
/// is pinned at its three outcomes, the S2/S9 trio idiom.
#[test]
fn ring_clearance_trio_definite_pass_definite_refuse_in_band_escalate() {
    use sweep::test_support::ring_clearance;
    let (pipped, _) = pipped_and_box_edges();
    let face = pipped.faces().next().unwrap().0;
    let tol = Tol::witness().get();
    // Definite pass.
    ring_clearance(face, sweep::blend::Convexity::Convex, 0.05, band())
        .expect("a definite clearance carries the ring");
    // Definite refuse, typed with the margin as payload.
    let err = ring_clearance(face, sweep::blend::Convexity::Convex, -0.05, band())
        .expect_err("a consumed ring refuses");
    match err {
        sweep::blend::BlendError::RingClearance { margin, .. } => {
            assert_eq!(margin.predicate, "fillet3_ring_clearance");
            assert!(
                margin
                    .reading
                    .diagnostic_f64_for_error_text()
                    .value()
                    .is_some_and(|m| (m - -0.05).abs() < 1e-15)
            );
        }
        other => panic!(
            "expected RingClearance, got {other_at}",
            other_at = other.render(geom_brep::recourse::Reading::Build)
        ),
    }
    let text = ring_clearance(face, sweep::blend::Convexity::Convex, -0.05, band())
        .unwrap_err()
        .render(geom_brep::recourse::Reading::Build);
    assert!(
        text.contains("ring") && text.contains("reduce the blend size"),
        "refusal names the situation and the recourse: {text}"
    );
    // In band: escalates through the funnel with the SAME recourse.
    let err = ring_clearance(face, sweep::blend::Convexity::Convex, 5.0 * tol.eps, band())
        .expect_err("in-band escalates");
    match &err {
        sweep::blend::BlendError::Escalated { source, .. } => {
            assert_eq!(source.predicate, Some("fillet3_ring_clearance"));
        }
        other => panic!(
            "expected Escalated, got {other_at}",
            other_at = other.render(geom_brep::recourse::Reading::Build)
        ),
    }
    assert!(
        err.render(geom_brep::recourse::Reading::Build)
            .contains("reduce the blend size"),
        "the escalated arm shares the definite arm's recourse: {err_at}",
        err_at = err.render(geom_brep::recourse::Reading::Build)
    );
}

/// **The surgery front door refuses typed** at its named gap: an end
/// the cut-off does not build (a curved end face, a run-out). One box
/// edge of the pipped die, which used to be the run-out witness, is cut
/// off at its end faces and carves, and so does one pip's rim alone, a
/// closed edge.
#[test]
fn the_surgery_front_door_refuses_its_named_gaps() {
    let (pipped, box_edges) = pipped_and_box_edges();
    // (a) An edge ending at a curved end face.
    let (round, edge) = crate::common::operands::half_round_end();
    let err = fillet_edges(
        &sweep::test_support::at_rest(&round, Tol::witness()),
        &[edge],
        DIE_R,
        Tol::witness(),
    )
    .expect_err("a curved end face is an unbuilt end");
    assert!(
        matches!(
            err.error,
            sweep::blend::BlendError::UnbuiltEnd(sweep::blend::UnbuiltEnd::CurvedEndFace { .. })
        ),
        "the refusal names the curved end face: {}",
        err.render(geom_brep::recourse::Reading::Build)
    );
    fillet_edges(
        &sweep::test_support::at_rest(&pipped, Tol::witness()),
        &box_edges[..1],
        DIE_R,
        Tol::witness(),
    )
    .expect("one box edge of the pipped die is cut off at its end faces");
    // (b) One rim: a pip's rim is one closed edge (its cap one band), so
    // asking for it alone asks for the rim whole, and it carves. The
    // seam-split arm (an open plane–sphere chain stopping at a chart-seam
    // vertex) has no native fixture here
    // (`work/band/the-seam-split-rim-arms-have-no-native-fixture-once-a-full-revolve-is-one-band.md`).
    let rims = rim_edges(&pipped);
    fillet_edges(
        &sweep::test_support::at_rest(&pipped, Tol::witness()),
        &rims[..1],
        RIM_R,
        Tol::witness(),
    )
    .expect("one pip's whole rim carves alone");
}

/// **Ring carry-through, positively**: the shrunk top face of the
/// box-filleted pipped cube still carries its pip rims as rings — the
/// same FaceKey, the same ring loops, the S12 sense bit intact.
#[test]
fn the_shrunk_faces_keep_their_rings_and_senses() {
    let (pipped, box_edges) = pipped_and_box_edges();
    let rings_before: Vec<(topo::FaceKey, usize, bool)> = pipped
        .faces()
        .filter(|(_, f)| !f.rings.is_empty())
        .map(|(k, f)| (k, f.rings.len(), f.sense))
        .collect();
    assert_eq!(rings_before.len(), 6, "six pipped faces");
    let out = fillet_edges(
        &sweep::test_support::at_rest(&pipped, Tol::witness()),
        &box_edges,
        DIE_R,
        Tol::witness(),
    )
    .expect("the surgery");
    for (k, n, sense) in rings_before {
        let f = out.body.get_face(k).expect("the shrunk face keeps its key");
        assert_eq!(f.rings.len(), n, "ring count carried");
        assert_eq!(f.sense, sense, "sense bit carried (S12)");
    }
    assert_eq!(
        euler_counts(&out.body).r,
        21,
        "all 21 pip rims survive as rings"
    );
}
