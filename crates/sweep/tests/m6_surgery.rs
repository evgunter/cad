//! **The composition surgery** (M6 unit 1): the in-place edge-blend
//! op and THE COMPOSED DIE — filleted blank + 21 pips + filleted pip
//! rims in ONE body, walking every verb of shape (v)'s criterion on a
//! single solid.
//!
//! Geometry constants are `m5_pr12_die.rs`'s; the fixtures reuse its
//! builders byte-for-byte so the two suites meter the same die.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::PI;
use profile::RawLoop;

use geom::Surface;
use geom_core::Tol;
use geom_core::{Affine3, Band, Point2, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::blend::build::fillet_edges;
use sweep::test_support::cube;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, EdgeKey};

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

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn ball_poled(r: f64, c: Vec3<f64>, pole: Vec3<f64>) -> Body<f64> {
    let ball = ball_at(r, Vec3::new(0.0, 0.0, 0.0));
    let y = Vec3::new(0.0, 1.0, 0.0);
    let axis = y.cross(pole);
    let placed = if axis.norm() < 1e-12 {
        if y.dot(pole) > 0.0 {
            ball
        } else {
            topo::transform_rigid(
                &ball,
                &Affine3::rotation_about_axis(
                    geom_core::Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    PI,
                ),
                Tol::witness(),
            )
            .unwrap()
        }
    } else {
        topo::transform_rigid(
            &ball,
            &Affine3::rotation_about_axis(
                geom_core::Point3::new(0.0, 0.0, 0.0),
                axis.normalize(),
                y.dot(pole).clamp(-1.0, 1.0).acos(),
            ),
            Tol::witness(),
        )
        .unwrap()
    };
    topo::transform_rigid(&placed, &Affine3::translation(c), Tol::witness()).unwrap()
}

fn ball_at(r: f64, c: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -r), 1.0),
        ProfileVertex::new(p2(0.0, r), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    topo::transform_rigid(&ball, &Affine3::translation(c), Tol::witness()).unwrap()
}

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

fn pip_tool() -> Body<f64> {
    let places = pip_placements();
    let mut tool = ball_poled(PIP_R, places[0].0, places[0].1);
    for (c, n) in &places[1..] {
        tool = boolean_op_with(
            BooleanOp::Union,
            &tool,
            &ball_poled(PIP_R, *c, *n),
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("assembling the pip tool: {e}"))
        .body()
        .expect("a body")
        .body
        .clone();
    }
    tool
}

fn subtract(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let out = boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("pip subtraction: {e}"));
    out.body().expect("a body").body.clone()
}

/// The pipped cube and its twelve surviving box edges.
fn pipped_and_box_edges() -> (Body<f64>, Vec<EdgeKey>) {
    let cube0 = cube(DIE_L, Tol::witness());
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
    let kind_of = |f: topo::FaceKey| -> Option<u8> {
        match body.get_surface(body.get_face(f)?.surface)? {
            Surface::Plane { .. } => Some(0),
            Surface::Sphere { .. } => Some(1),
            _ => Some(2),
        }
    };
    let face_of = |he: topo::HalfEdgeKey| -> Option<topo::FaceKey> {
        Some(body.get_loop(body.get_half_edge(he)?.parent_loop)?.face)
    };
    let mut out = Vec::new();
    for (k, e) in body.edges() {
        let (Some(fa), Some(fb)) = (face_of(e.he_plus), face_of(e.he_minus)) else {
            continue;
        };
        let kinds = (kind_of(fa), kind_of(fb));
        if matches!(kinds, (Some(0), Some(1)) | (Some(1), Some(0))) {
            out.push(k);
        }
    }
    out
}

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
    let out = fillet_edges(&pipped, &box_edges, DIE_R, band(), Tol::witness())
        .expect("the surgery fillets the pipped cube");
    let body = out.body;
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
    // 2-arc rim, 2 meridian seam edges, a pole vertex and 2 half-cap
    // faces (the revolve seam), so per pip V 3 / E 4 / F 2.
    assert_eq!(body.vertices().count(), 24 + 21 * 3);
    assert_eq!(body.edges().count(), 48 + 21 * 4);
    assert_eq!(body.faces().count(), 26 + 21 * 2);
    let want = blank_volume() - 21.0 * cap(PIP_R, PIP_H);
    let props = topo::mass_properties(&body, Tol::witness()).unwrap();
    assert!(
        (props.volume - want).abs() <= 1e-9 * want,
        "volume {} vs closed form {want}",
        props.volume
    );
    assert_eq!(props.volume_pad, 0.0, "closed forms need no pad");
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
    use geom_core::k_stats::{start_verdict_log, take_verdict_log};
    // The log must be installed BEFORE the build (it records through
    // `decide` as the surgery runs) and taken straight after, so the
    // certification battery below runs unlogged exactly as it did.
    start_verdict_log();
    let die = composed_die();
    let log = take_verdict_log();
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
    // Counts: blank 24/48/26 + per pip: the pole and the two lower
    // meridian pieces survive in the shrunk half-caps; 2 plane-trim
    // feet + 2 meridian split vertices; 2 ta + 2 tb arcs + 1 slit
    // meridian (the band is ring-free, donut-style); 2 shrunk
    // half-caps + 1 band; 1 ring (the widened plane hole) —
    // 5 − 7 + 3 − 1 = 0 keeps χ.
    assert_eq!(die.vertices().count(), 24 + 21 * 5);
    assert_eq!(die.edges().count(), 48 + 21 * 7);
    assert_eq!(die.faces().count(), 26 + 21 * 3);
    let want = blank_volume() - 21.0 * (cap(PIP_R, PIP_H) + rim_fillet_extra(PIP_R, PIP_H, RIM_R));
    let props = topo::mass_properties(&die, Tol::witness()).unwrap();
    assert!(
        (props.volume - want).abs() <= 1e-9 * want,
        "composed volume {} vs closed form {want} (diff {})",
        props.volume,
        props.volume - want
    );
    assert_eq!(props.volume_pad, 0.0, "closed forms need no pad");
    let mesh = mesh::tessellate(&die, 5e-3, Tol::witness()).expect("the composed die tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
    let v_mesh = mesh::validate::signed_volume(&mesh);
    assert!(v_mesh > 0.0 && ((v_mesh - props.volume) / props.volume).abs() < 5e-3);
}

/// The composed die: pips first (one certified group op), then the
/// twelve box edges in place, then all 21 pip rims as closed chains
/// in ONE further call.
fn composed_die() -> Body<f64> {
    let (pipped, box_edges) = pipped_and_box_edges();
    let blanked = fillet_edges(&pipped, &box_edges, DIE_R, band(), Tol::witness())
        .expect("the box edges fillet in place")
        .body;
    let rims = rim_edges(&blanked);
    assert_eq!(rims.len(), 42, "21 rims of two arcs each");
    let out = fillet_edges(&blanked, &rims, RIM_R, band(), Tol::witness())
        .expect("the rims fillet to tori");
    assert_eq!(out.band_faces.len(), 21, "one torus band per rim");
    out.body
}

/// **Bit replay (D9)**: the composed die twice in one process, same
/// volume bits, same entity counts.
#[test]
fn the_composed_die_replays_bit_identically() {
    let a = composed_die();
    let b = composed_die();
    let (va, vb) = (
        topo::mass_properties(&a, Tol::witness()).unwrap().volume,
        topo::mass_properties(&b, Tol::witness()).unwrap().volume,
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
    use sweep::blend::surgery::ring_clearance;
    let (pipped, _) = pipped_and_box_edges();
    let face = pipped.faces().next().unwrap().0;
    let tol = Tol::witness().get();
    // Definite pass.
    ring_clearance(face, 0.05, band()).expect("a definite clearance carries the ring");
    // Definite refuse, typed with the margin as payload.
    let err = ring_clearance(face, -0.05, band()).expect_err("a consumed ring refuses");
    match err {
        sweep::blend::BlendError::RingClearance { margin, .. } => {
            assert!((margin - -0.05).abs() < 1e-15)
        }
        other => panic!("expected RingClearance, got {other}"),
    }
    let text = ring_clearance(face, -0.05, band()).unwrap_err().to_string();
    assert!(
        text.contains("ring") && text.contains("reduce the blend size"),
        "refusal names the situation and the recourse: {text}"
    );
    // In band: escalates through the funnel with the SAME recourse.
    let err = ring_clearance(face, 5.0 * tol.eps, band()).expect_err("in-band escalates");
    match &err {
        sweep::blend::BlendError::Escalated { source, .. } => {
            assert_eq!(source.predicate, Some("fillet3_ring_clearance"));
        }
        other => panic!("expected Escalated, got {other}"),
    }
    assert!(
        err.to_string().contains("reduce the blend size"),
        "the escalated arm shares the definite arm's recourse: {err}"
    );
}

/// **The surgery front door refuses typed** at its named gaps: a
/// partially-requested corner (run-outs), and an open plane–sphere
/// chain (a rim arc alone is not a closed rim).
#[test]
fn the_surgery_front_door_refuses_its_named_gaps() {
    let (pipped, box_edges) = pipped_and_box_edges();
    // (a) One box edge: its corners' other edges are not requested.
    let err = fillet_edges(&pipped, &box_edges[..1], DIE_R, band(), Tol::witness())
        .expect_err("a partially-requested corner is a run-out, not implemented");
    let text = format!("{err}");
    assert!(
        text.contains("not implemented") && text.contains("corner"),
        "the refusal names the run-out gap: {text}"
    );
    // (b) One rim arc: an OPEN plane–sphere chain terminates at rim
    // vertices whose third edge is the cap MERIDIAN — a sphere–sphere
    // support pair no arm covers — so the BATTERY's corner classifier
    // refuses first, naming the run-out policy that would handle it.
    // The refusal is one door earlier than the surgery's own front
    // door, and that is the honest order: verdict before assembly.
    let rims = rim_edges(&pipped);
    let err = fillet_edges(&pipped, &rims[..1], RIM_R, band(), Tol::witness())
        .expect_err("an open rim arc has no classifiable termination");
    let text = format!("{err}");
    assert!(
        text.contains("run-out") && text.contains("not implemented"),
        "the refusal names the run-out gap: {text}"
    );
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
    let out =
        fillet_edges(&pipped, &box_edges, DIE_R, band(), Tol::witness()).expect("the surgery");
    for (k, n, sense) in rings_before {
        let f = out.body.get_face(k).expect("the shrunk face keeps its key");
        assert_eq!(f.rings.len(), n, "ring count carried");
        assert_eq!(f.sense, sense, "sense bit carried (S12)");
    }
    let total: usize = out.body.faces().map(|(_, f)| f.rings.len()).sum();
    assert_eq!(total, 21, "all 21 pip rims survive as rings");
}
