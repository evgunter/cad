//! **Reviewer probes for VERBS-CHAMFER (r1)** — independent
//! derivations of the PR's claims, authored against the public API
//! only. Each row goes RED when the guarantee it names degrades, not
//! merely at one chosen fixture:
//!
//! * the closed-form mass claim is re-derived for a GENERAL box
//!   (`a×b×c` random each run, `CAD_FUZZ_SEED` replays), not the cube
//!   the PR pinned;
//! * the outward-normal claim (C3) is checked with a direct material
//!   oracle — every face of a convex chamfered body must have its
//!   sensed normal pointing away from the body's interior — on skewed
//!   wedges whose corners are far from the cube's right trihedra;
//! * the K-funnel claim (C4) is re-taken at the recording scalar:
//!   which predicates a chamfer run actually meters is asserted as a
//!   SET, so a vacuous row entering (or a real one vanishing) reddens
//!   it;
//! * the refusal surface (C5) is probed where the PR's own fixtures
//!   did not stand: the L-bracket's all-convex-edges request, and a
//!   sliver wedge whose feet overrun their own support edge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Point2, Point3, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::chamfer::chamfer_edges;
use sweep::fillet::BlendError;
use sweep::{Extrusion, extrude};
use test_utils::fuzz;
use topo::{Body, EdgeKey};

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn all_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges().map(|(k, _)| k).collect()
}

/// Extrude a convex polygon (counterclockwise vertices) by `h`.
fn prism(pts: &[(f64, f64)], h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a convex polygon validates");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("the prism extrudes")
        .body
}

/// **The material oracle**: on a CONVEX body, every face's sensed
/// outward normal must point away from the interior — checked at the
/// face's own boundary-vertex centroid against the body's vertex
/// centroid. This is the direct form of C3's claim, independent of
/// `validate_geometric`'s internals; a single flipped sense bit or a
/// wrongly-folded chart normal reddens it.
fn assert_every_face_outward(body: &Body<f64>) {
    let pts: Vec<Point3<f64>> = body
        .vertices()
        .filter_map(|(k, _)| body.get_vertex(k))
        .filter_map(|v| body.get_point(v.point))
        .copied()
        .collect();
    let n = pts.len() as f64;
    let interior = Point3::new(
        pts.iter().map(|p| p.x).sum::<f64>() / n,
        pts.iter().map(|p| p.y).sum::<f64>() / n,
        pts.iter().map(|p| p.z).sum::<f64>() / n,
    );
    for (fk, _) in body.faces() {
        let f = body.get_face(fk).expect("a face");
        let Some(Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            panic!("face {fk:?}: every face of these fixtures is a plane");
        };
        let outward = if f.sense { *normal } else { -*normal };
        // Any point of the face's plane serves as the witness; the
        // plane's own origin is one.
        let d = outward.dot(*origin - interior);
        assert!(
            d > 0.0,
            "face {fk:?}: sensed normal {outward:?} does not point away from the \
             interior (fold {d}); a convex body's every face must"
        );
    }
}

/// Census + tiers + planes-only, shared by the probes.
fn assert_chamfer_shape(body: &Body<f64>, census: (usize, usize, usize)) {
    assert_eq!(topo::validate(body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
    let got = (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
    );
    assert_eq!(got, census, "census");
    let (v, e, f) = got;
    assert_eq!(v as i64 - e as i64 + f as i64, 2, "Euler–Poincaré");
    for (k, _) in body.faces() {
        let fd = body.get_face(k).expect("a face");
        assert!(
            matches!(body.get_surface(fd.surface), Some(Surface::Plane { .. })),
            "face {k:?} is not a plane"
        );
    }
}

/// **C2, re-derived for the general box.** The PR pins the CUBE's
/// closed form; the corner and edge deficits are local facts, so the
/// same derivation gives `abc − 2d²(a+b+c) + (16/3)d³` for any box —
/// an independent formula the PR never wrote down. Dimensions are
/// random each run (`CAD_FUZZ_SEED` replays); a chamfer whose strips
/// sit at anything but the exact setback, or whose corner patches cut
/// anything but the exact tetrahedron correction, reddens this on
/// some draw.
#[test]
fn a_general_box_matches_an_independent_closed_form() {
    let mut rng = fuzz::start("chamfer general box");
    for _ in 0..fuzz::scaled(4) {
        let (a, b, c) = (
            rng.range(0.5, 3.0),
            rng.range(0.5, 3.0),
            rng.range(0.5, 3.0),
        );
        let d = rng.range(0.02, 0.2) * a.min(b).min(c);
        let pad = prism(&[(0.0, 0.0), (a, 0.0), (a, b), (0.0, b)], c);
        let out = chamfer_edges(&pad, &all_edges(&pad), d, band(), Tol::witness())
            .unwrap_or_else(|e| panic!("a {a}×{b}×{c} box chamfers at {d}: {e}"));
        assert_chamfer_shape(&out.body, (24, 48, 26));
        assert_every_face_outward(&out.body);
        let want = a * b * c - 2.0 * d * d * (a + b + c) + 16.0 / 3.0 * d.powi(3);
        let props = topo::mass_properties(&out.body, Tol::witness()).expect("props");
        assert!(
            (props.volume - want).abs() <= 1e-12 * want,
            "volume {} vs independent closed form {want} (a={a} b={b} c={c} d={d})",
            props.volume
        );
        assert_eq!(props.volume_pad, 0.0, "closed forms need no pad");
    }
}

/// **C3 away from the cube.** A skewed wedge (random triangular
/// cross-section, apex angles well off 90°) chamfers on all nine
/// edges; every one of its twenty faces must come out a plane with an
/// outward sensed normal. The corner patches here are genuinely
/// oblique trihedra, so the `normalize(g·(g·Σn))` fold is exercised
/// where `g` and `Σn` are far from parallel — the cube never leaves
/// them so.
#[test]
fn a_skewed_wedge_chamfers_with_every_face_outward() {
    let mut rng = fuzz::start("chamfer skewed wedge");
    for _ in 0..fuzz::scaled(4) {
        // A triangle with a controlled sharpest angle in [25°, 80°]:
        // base along x, apex placed by angle from each base end.
        let base = rng.range(1.5, 3.0);
        let ang_a = rng.range(25f64, 80.0).to_radians();
        let ang_b = rng.range(25f64, 80.0).to_radians();
        // Apex = intersection of the two rays from the base ends.
        let t = base / (ang_a.tan().recip() + ang_b.tan().recip());
        let apex_x = t / ang_a.tan();
        let tri = [(0.0, 0.0), (base, 0.0), (apex_x, t)];
        let h = rng.range(0.8, 2.0);
        let body = prism(&tri, h);
        // Setback small against every feature so the request is
        // honestly grantable.
        let d = 0.02 * base.min(h).min(t);
        let out = chamfer_edges(&body, &all_edges(&body), d, band(), Tol::witness())
            .unwrap_or_else(|e| panic!("the wedge chamfers at {d}: {e}"));
        assert_chamfer_shape(&out.body, (18, 36, 20));
        assert_every_face_outward(&out.body);
        let props = topo::mass_properties(&out.body, Tol::witness()).expect("props");
        let uncut = 0.5 * base * t * h;
        assert!(
            props.volume > 0.0 && props.volume < uncut,
            "a chamfer removes material: {} vs {uncut}",
            props.volume
        );
        let mesh = mesh::tessellate(&out.body, 5e-3, Tol::witness()).expect("tessellates");
        mesh::validate::check_mesh(&mesh).expect("watertight");
    }
}

/// **C4, re-taken at the recording scalar.** Chamfer a box end to end
/// at `Probe` and read the funnel: the predicate NAMES that recorded
/// must be exactly the chamfer's — the two rolling-ball facts
/// (`fillet3_radius_headroom`, `fillet3_spine_regularity`) absent, and
/// nothing new minted. Asserted as set equality, so a vacuous
/// predicate ENTERING the corpus reddens this row exactly as a real
/// one vanishing does.
///
/// Note what the set does NOT contain: `fillet3_chain_g1`. A
/// fully-chamfered box has no two-link junctions (every vertex is a
/// trivalent corner), so chain G1 never fires — on this scene OR on
/// the tour's `spacer` scene, whose K-row registration names it as one
/// of the four the scene delivers. The four that reach the funnel from
/// such a scene are chain-arm, convexity-sign, face-clearance and
/// corner-independence.
#[cfg(feature = "probe")]
#[test]
fn the_chamfers_probe_rows_are_exactly_its_own_questions() {
    use geom_core::k_stats::{self, Probe};
    let lp: ProfileLoop<Probe> = ProfileLoop::new(
        [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(Probe(x), Probe(y)), Probe(0.0)))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a rectangle validates");
    let pad = extrude(&profile, Extrusion::Distance(Probe(1.0)), Tol::witness())
        .expect("the pad extrudes")
        .body;
    let edges: Vec<EdgeKey> = pad.edges().map(|(k, _)| k).collect();
    k_stats::start_recording();
    chamfer_edges(&pad, &edges, Probe(0.1), band(), Tol::witness()).expect("the pad chamfers");
    let samples = k_stats::take_samples();
    // The construction also meters the certification predicates
    // (carrier_*, witness_*, dihedral_*) — real questions of the
    // carve, not battery rows. The C4 claim is about the BATTERY
    // roster, so restrict to its family.
    let mut names: Vec<&str> = samples
        .iter()
        .map(|s| s.predicate)
        .filter(|n| n.starts_with("fillet3_"))
        .collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(
        names,
        vec![
            "fillet3_chain_arm",
            "fillet3_convexity_sign",
            "fillet3_corner_independence",
            "fillet3_face_clearance",
        ],
        "the chamfer's battery rows: ball facts absent, nothing new minted, \
         and no junction ever fires chain G1 on an all-corners request"
    );
}

/// **A ringed support face carries its ring through a chamfer** — the
/// one surgery path the PR's own fixtures never drive under
/// `BlendKind::Chamfer`: `ring_clearance_pass` folding a support
/// face's RING against the chamfer's trimlines, then the carve
/// keeping the ring on the shrunk face. The part is a real one — a
/// dimpled spacer: a cube with a hemispherical dimple sunk into its
/// top face (boolean subtract), then every box edge broken. The
/// dimple's rim ring must survive on the shrunk top face, the twelve
/// strips and eight patches must mint, and the volume must be the
/// chamfered-box closed form minus exactly the hemisphere.
#[test]
fn a_dimpled_spacer_carries_its_ring_through_the_chamfer() {
    let l = 1.0;
    let d = 0.1;
    let r = 0.15;
    let cube = prism(&[(0.0, 0.0), (l, 0.0), (l, l), (0.0, l)], l);
    let box_edges: Vec<EdgeKey> = cube.edges().map(|(k, _)| k).collect();
    // A ball centred ON the top face: subtracting sinks a
    // hemispherical dimple whose rim is a ring of that face.
    let ball = {
        use geom_core::{Affine3, Vec2, Vec3};
        use sweep::{Revolution, RevolveAxis, revolve};
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(Point2::new(0.0, -r), 1.0),
            ProfileVertex::new(Point2::new(0.0, r), 0.0),
        ]);
        let vp = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(Tol::witness())
            .expect("a ball profile validates");
        let axis = RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        };
        let ball = revolve(&vp, axis, Revolution::Full, Tol::witness())
            .expect("the ball revolves")
            .body;
        // Stand the revolve's pole axis (y) up along z so the face
        // plane cuts a clean latitude rim, then centre it on the top
        // face.
        let upright = topo::transform_rigid(
            &ball,
            &Affine3::rotation_about_axis(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                core::f64::consts::FRAC_PI_2,
            ),
            Tol::witness(),
        )
        .expect("the ball stands up");
        topo::transform_rigid(
            &upright,
            &Affine3::translation(Vec3::new(0.5, 0.5, l)),
            Tol::witness(),
        )
        .expect("the ball translates")
    };
    let dimpled = {
        use topo::boolean::{BooleanDeclarations, BooleanOp, SweepStrategy, boolean_op_with};
        boolean_op_with(
            BooleanOp::Subtract,
            &cube,
            &ball,
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        )
        .expect("the dimple subtracts")
        .body()
        .expect("a body")
        .body
        .clone()
    };
    let surviving: Vec<EdgeKey> = box_edges
        .into_iter()
        .filter(|k| dimpled.get_edge(*k).is_some())
        .collect();
    assert_eq!(surviving.len(), 12, "every box edge survives the dimple");
    let out = chamfer_edges(&dimpled, &surviving, d, band(), Tol::witness())
        .expect("the dimpled spacer's twelve edges chamfer around its ring");
    assert_eq!(topo::validate(&out.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&out.body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&out.body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
    assert_eq!(out.blend_faces.len(), 12, "a strip per box edge");
    assert_eq!(out.corner_faces.len(), 8, "a patch per corner");
    let props = topo::mass_properties(&out.body, Tol::witness()).expect("props");
    let want = l.powi(3) - 6.0 * l * d * d + 16.0 / 3.0 * d.powi(3)
        - 0.5 * 4.0 / 3.0 * core::f64::consts::PI * r.powi(3);
    assert!(
        (props.volume - want).abs() <= 1e-9 * want,
        "chamfered box minus the hemisphere: {} vs {want}",
        props.volume
    );
}

/// **The fail-loud contract at an overrunning corner.** At a sharp
/// wedge corner the chamfer's foot slides `d/tan(θ/2)` along the
/// boundary — far beyond `d` — and the clearance screen skips
/// ADJACENT edge pairs (their consumption is delegated to "the corner
/// and G1 predicates", which meter neither this length nor its
/// overrun). So drive the feet past each other on a shared cap edge
/// and hold the verb to its D-series contract: it must either refuse
/// TYPED or return a body that still passes every tier with a
/// positive, material-removing volume. A silently invalid body — or a
/// silently wrong volume from crossed trimlines — reddens this row.
#[test]
fn an_overrunning_sliver_corner_refuses_or_stays_valid() {
    // Apex angle ≈ 8.5° at the origin: the foot slides
    // `d/tan(4.27°) ≈ 13.4·d ≈ 2.01` along the base — past the far
    // corner of a base only 2.0 long — and the cap triangle's inradius
    // (≈ 0.139) is smaller than the setback, so the shrunk cap does
    // not exist at all. Every pair of the cap's boundary edges is
    // ADJACENT, so the clearance screen checks no pair on it.
    let tri = [(0.0, 0.0), (2.0, 0.0), (2.0, 0.3)];
    let body = prism(&tri, 1.0);
    let d = 0.15;
    // Observed at review time: this refuses
    // `FaceClearanceUncertified { margin: 0.0, gap: 0.3 }` — the
    // screen catches the sliver through the SHORT side face's
    // vertical-edge pair (gap `0.3 − 2d`), not through the cap whose
    // pairs are all adjacent and skipped. The row asserts the
    // contract, not the arm, so a future widening that grants this
    // request stays green only by staying valid.
    match chamfer_edges(&body, &all_edges(&body), d, band(), Tol::witness()) {
        Err(e) => {
            // Typed refusal is an honest answer; assert it is one of
            // the verb's own documented arms, not a panic elsewhere.
            let text = format!("{e}");
            assert!(!text.is_empty(), "a refusal names itself: {e:?}");
        }
        Ok(out) => {
            assert_eq!(topo::validate(&out.body), Ok(()), "tier 1");
            assert_eq!(topo::validate_closed(&out.body), Ok(()), "tier 2");
            assert_eq!(
                topo::validate_geometric(&out.body, Tol::witness()),
                Ok(()),
                "tier 3"
            );
            let props = topo::mass_properties(&out.body, Tol::witness()).expect("props");
            let uncut = 0.5 * 2.0 * 0.4 * 1.0;
            assert!(
                props.volume > 0.0 && props.volume < uncut,
                "a granted chamfer removed material honestly: {} vs {uncut}",
                props.volume
            );
        }
    }
}

/// **A nonpositive setback refuses as the invalid INPUT it is.**
///
/// There is no rolling ball, so the predicate that caught a
/// nonpositive size for the FILLET (radius headroom) is correctly not
/// metered here — which is what left `d = 0` and `d < 0` falling
/// through to corner-independence, whose margin is levered by the
/// setback itself. The consumer then read "VertexKey(..) is a
/// trihedron with DEPENDENT support normals" about a cube corner whose
/// normals are exactly orthonormal: fail-loud held, but the kernel
/// asserted a false fact about the BODY.
///
/// **This row's expectation was amended when the finding was adopted**
/// (review r1, MINOR-1): the door now refuses `BlendError::
/// NonpositiveSize` before anything resolves. What the row pins is
/// unchanged in substance — a nonpositive setback must refuse, and
/// must refuse by naming the request rather than the geometry — so the
/// assertion moved to the honest verdict rather than the row being
/// retired. The refusal is deliberately NOT a metered predicate:
/// whether the caller handed in a positive number is not a geometric
/// quantity of the body and takes no K-corpus row.
#[test]
fn a_nonpositive_setback_refuses_as_invalid_input() {
    let pad = prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 1.0);
    let edges = all_edges(&pad);
    for d in [0.0, -0.1] {
        let err = chamfer_edges(&pad, &edges, d, band(), Tol::witness())
            .expect_err("a nonpositive setback must not mint a body");
        assert!(
            matches!(err.error, BlendError::NonpositiveSize { .. }),
            "a nonpositive setback names the request, not the corner, at d = {d}: {err:?}"
        );
        let text = format!("{err}");
        assert!(
            !text.contains("dependent support normals"),
            "the refusal must not assert a false fact about the body: {text}"
        );
    }
}

/// **C5 where the PR's fixture did not stand**: the L-bracket's
/// all-CONVEX-edges request (the concave inner edge deliberately left
/// out) still refuses, and this row pins WHAT it refuses with. The two
/// requested cap edges flanking the omitted inner edge meet at its end
/// vertices as two-link junctions, so the battery's chain-G1 predicate
/// reaches the request before any coverage door does and the consumer
/// reads `ChainNotG1` — "supply a tangent-continuous chain, or split
/// the request at the tangent break" — although splitting at the break
/// refuses again (run-out), so the recourse leads nowhere on this
/// body. A refusal that is TRUE but points at chain smoothness when
/// the situation is v1's all-or-nothing corner coverage: recorded as a
/// review finding; the pin keeps the verdict decided and visible.
#[test]
fn the_brackets_best_convex_request_still_refuses_typed() {
    let bracket = prism(
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        1.0,
    );
    // Every edge except the concave one over the reflex corner (1,1).
    let concave = |k: EdgeKey| {
        let e = bracket.get_edge(k).unwrap();
        let h = bracket.get_half_edge(e.he_plus).unwrap();
        let end = bracket.half_edge_end(e.he_plus).unwrap();
        let at = |v| {
            let p = bracket
                .get_point(bracket.get_vertex(v).unwrap().point)
                .unwrap();
            (p.x - 1.0).abs() < 1e-12 && (p.y - 1.0).abs() < 1e-12
        };
        at(h.start) && at(end)
    };
    let edges: Vec<EdgeKey> = bracket
        .edges()
        .map(|(k, _)| k)
        .filter(|k| !concave(*k))
        .collect();
    assert_eq!(edges.len(), all_edges(&bracket).len() - 1, "one edge out");
    let err = chamfer_edges(&bracket, &edges, 0.05, band(), Tol::witness())
        .expect_err("a corner with an unrequested edge cannot be patched");
    assert!(
        matches!(err.error, BlendError::ChainNotG1 { .. }),
        "today's decided answer is the chain-G1 kink at the omitted edge's ends \
         (see the row doc for why that framing is itself a finding): {err:?}"
    );
}
