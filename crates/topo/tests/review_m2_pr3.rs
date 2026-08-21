//! M2 PR 3 adversarial review — body-level attacks through the PUBLIC
//! API only (falsification targets 2, 6, 8 of the review spec; the
//! certification-layer attacks live in
//! `geom-brep/tests/review_m2_pr3_certify.rs`).
//!
//! Convention: `survives_*` tests are attacks the implementation
//! resisted, promoted to CI verbatim. `fixed_*` tests were the review's
//! `finding_*` pins of defective behavior, FLIPPED by the fix pass to
//! assert the corrected behavior (each keeps its finding lineage in its
//! doc comment). `fixed_n4_*` are the fix pass's raw-operator
//! precondition tests (N4: the sugar wrappers pre-resolve site keys, so
//! the raw ops' own precondition paths were shadowed in coverage).
//! `e2e_*` is the mini-extrude consumer program demanded by the spec.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::{MappedCurve, SketchSegment, newell_plane};
use geom_core::{Affine3, Band, Decide, Point2, Point3, Tolerance, Vec3};
use topo::{
    Body, EdgeCurveSpec, EdgeGeometry, EulerOpError, FaceSurface, MefSite, MevSite, SurfaceKey,
    ValidationError, validate, validate_closed, validate_geometric,
};
use geom_core::Tol;

mod common;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn eps() -> f64 {
    Tol::witness().get().eps
}

// =====================================================================
// Target 6 — the mini-extrude e2e: a triangular prism hand-built by the
// EXACT recipe the PR promises PR 4 (real MappedCurve mint specs:
// ExtrudedPoint struts, PlacedSegment rims; FaceSurface::New Newell
// planes for the sides; seed-face cap via set_face_surface; corner
// joins upgraded to Intersection via set_edge_curve at rest), then
// tiers 1–3 demanded clean.
// =====================================================================

/// The prism build, generic over the scalar lane. Sketch triangle
/// a(0,0) b(1,0) c(0.3,0.8) placed at identity, extruded along +z by 1.
fn triangle_prism<T: Decide>() -> (Body<T>, topo::MvfsCreated, [topo::MefCreated; 4]) {
    let f = T::from_f64;
    let sp = |x: f64, y: f64| Point2::new(f(x), f(y));
    let wp = |x: f64, y: f64, z: f64| Point3::new(f(x), f(y), f(z));
    let (sa, sb, sc) = (sp(0.0, 0.0), sp(1.0, 0.0), sp(0.3, 0.8));
    let (a, b, c) = (wp(0.0, 0.0, 0.0), wp(1.0, 0.0, 0.0), wp(0.3, 0.8, 0.0));
    let (a1, b1, c1) = (wp(0.0, 0.0, 1.0), wp(1.0, 0.0, 1.0), wp(0.3, 0.8, 1.0));
    let w = Vec3::new(f(0.0), f(0.0), f(1.0));
    let place_bottom = Affine3::identity();
    let place_top = Affine3::translation(w);

    // A bottom/top profile rim: PlacedSegment (the sweep's own form).
    let rim = |s0: Point2<T>, s1: Point2<T>, p0: Point3<T>, p1: Point3<T>, place| EdgeCurveSpec {
        description: EdgeGeometry::MappedCurve(MappedCurve::PlacedSegment {
            segment: SketchSegment::Line { a: s0, b: s1 },
            place,
        }),
        carrier: Curve3::Line {
            origin: p0,
            dir: (p1 - p0) / p0.distance(p1),
        },
        param_start: T::zero(),
        param_end: p0.distance(p1),
    };
    // A side strut: ExtrudedPoint (the sweep's own form), s in [0,1].
    let strut_spec = |s0: Point2<T>, p0: Point3<T>| EdgeCurveSpec {
        description: EdgeGeometry::MappedCurve(MappedCurve::ExtrudedPoint {
            point: s0,
            place: place_bottom,
            vec: w,
        }),
        carrier: Curve3::Line {
            origin: p0,
            dir: w / w.norm(),
        },
        param_start: T::zero(),
        param_end: w.norm(),
    };
    let plane = |corners: &[Point3<T>]| newell_plane(corners, band()).unwrap();

    let mut body = Body::<T>::new();
    let seed = body.mvfs(a).unwrap();
    let e_ab = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            rim(sa, sb, a, b, place_bottom),
            Tol::witness(),
        )
        .unwrap();
    let e_bc = body
        .mev(
            MevSite::Fan {
                he1: e_ab.he_minus,
                he2: e_ab.he_minus,
            },
            c,
            rim(sb, sc, b, c, place_bottom),
            Tol::witness(),
        )
        .unwrap();
    // Bottom cap: outward −z, loop C→B→A; the closing mef edge runs
    // start(he1)=C → start(he2)=A.
    let f_bottom = body
        .mef(
            MefSite::Chords {
                he1: e_bc.he_minus,
                he2: e_ab.he_plus,
            },
            rim(sc, sa, c, a, place_bottom),
            FaceSurface::New(plane(&[c, b, a])),
            Tol::witness(),
        )
        .unwrap();
    // Struts (ExtrudedPoint descriptions — the real sweep shape).
    let strut = |body: &mut Body<T>, at, s0, p0, p_top| {
        body.mev(MevSite::Fan { he1: at, he2: at }, p_top, strut_spec(s0, p0), Tol::witness())
            .unwrap()
    };
    let e_aa = strut(&mut body, e_ab.he_plus, sa, a, a1);
    let e_bb = strut(&mut body, e_bc.he_plus, sb, b, b1);
    let e_cc = strut(&mut body, f_bottom.he_plus, sc, c, c1);
    // Side faces (Newell planes, outward via the interior-left order);
    // top rims are PlacedSegments on the translated sketch plane.
    let f_ab = body
        .mef(
            MefSite::Chords {
                he1: e_aa.he_minus,
                he2: e_bb.he_minus,
            },
            rim(sa, sb, a1, b1, place_top),
            FaceSurface::New(plane(&[a, b, b1, a1])),
            Tol::witness(),
        )
        .unwrap();
    let f_bc = body
        .mef(
            MefSite::Chords {
                he1: e_bb.he_minus,
                he2: e_cc.he_minus,
            },
            rim(sb, sc, b1, c1, place_top),
            FaceSurface::New(plane(&[b, c, c1, b1])),
            Tol::witness(),
        )
        .unwrap();
    let f_ca = body
        .mef(
            MefSite::Chords {
                he1: e_cc.he_minus,
                he2: f_ab.he_plus,
            },
            rim(sc, sa, c1, a1, place_top),
            FaceSurface::New(plane(&[c, a, a1, c1])),
            Tol::witness(),
        )
        .unwrap();
    // The seed face survives as the top cap (outward +z: A′ B′ C′).
    body.set_face_surface(seed.face, FaceSurface::New(plane(&[a1, b1, c1])))
        .unwrap();
    (body, seed, [f_bottom, f_ab, f_bc, f_ca])
}

/// The strongest single test of the PR: the promised PR 4 flow works
/// end to end through the public API — mint with real sweep-shaped
/// MappedCurve descriptions, Newell side/cap planes, then the
/// prefer-intrinsic upgrade at rest, tiers 1–3 clean after. (Updated by
/// the M2 PR 4 fix pass: prefer-intrinsic now has tier-3 teeth, so the
/// PRE-upgrade body at rest honestly reports its nine transverse
/// MappedCurve chords by name instead of passing.)
#[test]
fn e2e_mini_extrude_triangle_prism_passes_tiers_and_upgrades() {
    let (mut body, _seed, _mefs) = triangle_prism::<f64>();
    assert_eq!(body.vertices().count(), 6);
    assert_eq!(body.edges().count(), 9);
    assert_eq!(body.faces().count(), 5);
    assert_eq!(body.surfaces().count(), 5);
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(validate_closed(&body), Ok(()));
    // Prefer-intrinsic enforcement (D2, ratified 2026-07-19): every
    // prism edge is a definitely-transverse plane-pair corner, so at
    // rest the un-upgraded body names all nine — and nothing else.
    let errs = validate_geometric(&body, Tol::witness()).unwrap_err();
    assert_eq!(errs.len(), 9, "{errs:?}");
    assert!(
        errs.iter()
            .all(|e| matches!(e, ValidationError::TransverseNotIntrinsic { .. })),
        "{errs:?}"
    );
    // The prefer-intrinsic upgrade: all nine upgrade via set_edge_curve.
    common::describe_as_intersections(&mut body);
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
    assert!(body.curves().all(|(_, c)| matches!(
        c.certified().map(topo::EdgeCurve::description),
        Some(EdgeGeometry::Intersection { .. })
    )));
    // Determinism (D9): a replayed build is certificate-identical.
    let (body2, _, _) = triangle_prism::<f64>();
    let dump = |b: &Body<f64>| {
        b.curves()
            .map(|(k, c)| {
                let c = c.certified().unwrap();
                format!("{k:?} {:?} {:?}\n", c.params(), c.certificate())
            })
            .collect::<String>()
    };
    let (fresh, _, _) = triangle_prism::<f64>();
    assert_eq!(dump(&body2), dump(&fresh));
}

/// The same consumer at the Dual lane: value channel takes the
/// identical certified path (tangents never decide). Both lanes get the
/// prefer-intrinsic upgrade first (M2 PR 4 fix pass: transverse edges
/// must carry Intersection at rest).
#[test]
fn e2e_prism_dual_lane_matches_f64() {
    use geom_core::Dual64;
    let (mut f, _, _) = triangle_prism::<f64>();
    let (mut d, _, _) = triangle_prism::<Dual64>();
    common::describe_as_intersections(&mut f);
    common::describe_as_intersections(&mut d);
    assert_eq!(validate_geometric(&d, Tol::witness()), Ok(()));
    let fr: Vec<f64> = f
        .curves()
        .map(|(_, c)| c.certified().unwrap().certificate().max_residual)
        .collect();
    let dr: Vec<f64> = d
        .curves()
        .map(|(_, c)| c.certified().unwrap().certificate().max_residual.value)
        .collect();
    assert_eq!(fr.len(), dr.len());
    for (a, b) in fr.iter().zip(&dr) {
        assert_eq!(a.to_bits(), b.to_bits());
    }
}

// =====================================================================
// Target 2 — certified-by-construction, attacked from outside; and
// operator/setter atomicity proved by deep snapshot.
// =====================================================================

/// SURVIVES: every failing mutation path leaves the body DEEP-equal
/// (full Debug snapshot of all arenas) and tier-1 valid: op
/// certification failure, stale FaceSurface::Shared key, setter
/// adjacency refusal, setter certification failure, stale setter keys.
#[test]
fn survives_atomicity_deep_snapshots_on_every_failure_path() {
    let (mut body, seed, _mefs) = triangle_prism::<f64>();
    let snapshot = |b: &Body<f64>| format!("{b:?}");
    let stale_surface = SurfaceKey::default();

    // 1. mev with a poisoned point: certification failure.
    let before = snapshot(&body);
    let he = body.half_edges().next().map(|(k, _)| k).unwrap();
    let err = body
        .mev_line(
            MevSite::Fan { he1: he, he2: he },
            Point3::new(f64::NAN, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap_err();
    assert!(matches!(err, EulerOpError::Certification { .. }), "{err:?}");
    assert_eq!(snapshot(&body), before, "mev cert failure mutated body");

    // 2. mef with a stale Shared surface key (the semantic-rewrite
    //    successor of the M1 stale-anchor path).
    let err = body
        .mef(
            MefSite::Chords { he1: he, he2: he },
            EdgeCurveSpec::self_loop_circle_at(Point3::origin()),
            FaceSurface::Shared(stale_surface),
            Tol::witness(),
        )
        .unwrap_err();
    assert!(matches!(err, EulerOpError::StaleGeometry { .. }), "{err:?}");
    assert_eq!(snapshot(&body), before, "stale Shared mutated body");

    // 3. mef whose curve fails certification (wrong carrier): the
    //    self-loop site (trivially same-loop) with a carrier pinned
    //    nowhere near start(he1).
    let mut bad = EdgeCurveSpec::self_loop_circle_at(Point3::new(50.0, 50.0, 50.0));
    bad.param_start = 0.0; // keep the spec well-formed; endpoints are wrong
    let err = body
        .mef(
            MefSite::Chords { he1: he, he2: he },
            bad,
            FaceSurface::Inherit,
            Tol::witness(),
        )
        .unwrap_err();
    assert!(matches!(err, EulerOpError::Certification { .. }), "{err:?}");
    assert_eq!(snapshot(&body), before, "mef cert failure mutated body");

    // 4. set_edge_curve: non-adjacent Intersection description.
    let foreign = body.get_face(seed.face).unwrap().surface;
    let (ek, e) = body.edges().next().map(|(k, e)| (k, e.clone())).unwrap();
    let p0 = *body
        .get_point(
            body.get_vertex(body.get_half_edge(e.he_plus).unwrap().start)
                .unwrap()
                .point,
        )
        .unwrap();
    let p1 = *body
        .get_point(
            body.get_vertex(body.half_edge_end(e.he_plus).unwrap())
                .unwrap()
                .point,
        )
        .unwrap();
    let mut spec = EdgeCurveSpec::line_between(p0, p1);
    spec.description = EdgeGeometry::Seam { surface: foreign };
    let err = body.set_edge_curve(ek, spec, Tol::witness()).unwrap_err();
    assert!(
        matches!(
            err,
            EulerOpError::DescriptionNotAdjacent { .. } | EulerOpError::Certification { .. }
        ),
        "{err:?}"
    );
    assert_eq!(snapshot(&body), before, "setter refusal mutated body");

    // 5. set_edge_curve: certification failure (shifted carrier).
    let mut spec = EdgeCurveSpec::line_between(p0, p1);
    spec.carrier = Curve3::Line {
        origin: p0 + Vec3::new(0.0, 0.0, 100.0 * eps()),
        dir: (p1 - p0) / p0.distance(p1),
    };
    let err = body.set_edge_curve(ek, spec, Tol::witness()).unwrap_err();
    assert!(matches!(err, EulerOpError::Certification { .. }), "{err:?}");
    assert_eq!(snapshot(&body), before, "setter cert failure mutated body");

    // 6. set_face_surface: stale Shared key.
    let err = body
        .set_face_surface(seed.face, FaceSurface::Shared(stale_surface))
        .unwrap_err();
    assert!(matches!(err, EulerOpError::StaleGeometry { .. }), "{err:?}");
    assert_eq!(snapshot(&body), before, "surface setter mutated body");

    assert_eq!(validate(&body), Ok(()));
}

/// SURVIVES: re-pointing a face's surface AFTER intrinsic upgrades is
/// detected at tier 3 (DescriptionNotAdjacent + PlanarFaceResidual),
/// and the old surface is ANCHORED by the edge descriptions (its key
/// still resolves — no DanglingDescription, no silent reap).
#[test]
fn survives_surface_swap_behind_intersection_edges_detected_at_rest() {
    let t = common::geometric_cube::<f64>();
    let mut body = t.body;
    common::describe_as_intersections(&mut body);
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));

    let old_surface = body.get_face(t.seed.face).unwrap().surface;
    // Replace the top cap's plane with one shifted definitely off the
    // top vertices (1000·eps ≥ K·eps at every CI row).
    let shifted = Surface::Plane {
        origin: Point3::new(0.5, 0.5, 1.0 + 1000.0 * eps()),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    body.set_face_surface(t.seed.face, FaceSurface::New(shifted))
        .unwrap();

    // Anchoring: the old plane is still referenced by four Intersection
    // descriptions, so it must survive the orphan sweep.
    assert!(
        body.get_surface(old_surface).is_some(),
        "descriptions must anchor the old surface"
    );
    // Tiers 1–2 still structurally clean; tier 3 reports the swap.
    assert_eq!(validate_closed(&body), Ok(()));
    let errs = validate_geometric(&body, Tol::witness()).unwrap_err();
    let not_adjacent = errs
        .iter()
        .filter(|e| matches!(e, ValidationError::DescriptionNotAdjacent { .. }))
        .count();
    let planar = errs
        .iter()
        .filter(|e| matches!(e, ValidationError::PlanarFaceResidual { .. }))
        .count();
    assert_eq!(not_adjacent, 4, "four top edges lost coherence: {errs:?}");
    assert_eq!(planar, 4, "four top corners off the new plane: {errs:?}");
}

// =====================================================================
// Target 8 — tier-3 corruption sweep through the public API.
// =====================================================================

/// The two-face lamina through one segment (both faces bounded by the
/// two A–B edges) — the minimal body whose dihedral is controlled
/// entirely by two set_face_surface calls.
fn lamina() -> (Body<f64>, topo::MvfsCreated, topo::MefCreated) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::origin()).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    let split = body
        .mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        }, Tol::witness())
        .unwrap();
    (body, seed, split)
}

/// SURVIVES: an isolated in-band (3·eps) dihedral at rest is reported
/// as SliverDihedral and nothing else; the coplanar-shared-key variant
/// is definitely Smooth and tier-3 clean (the legal π wedge).
#[test]
fn survives_sliver_dihedral_isolated_at_rest() {
    let theta = 3.0 * eps();
    let (mut body, seed, split) = lamina();
    let flat = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let tilted = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(0.0, theta.sin(), theta.cos()),
        u_ref: Vec3::unit_x(),
    };
    body.set_face_surface(seed.face, FaceSurface::New(flat.clone()))
        .unwrap();
    body.set_face_surface(split.face, FaceSurface::New(tilted))
        .unwrap();
    assert_eq!(validate_closed(&body), Ok(()));
    let errs = validate_geometric(&body, Tol::witness()).unwrap_err();
    assert!(!errs.is_empty());
    assert!(
        errs.iter()
            .all(|e| matches!(e, ValidationError::SliverDihedral { .. })),
        "only sliver reports expected: {errs:?}"
    );

    // Coplanar variant through Shared (the ratified sharing story):
    // definitely smooth, tier 3 clean.
    let (mut body, seed, split) = lamina();
    let key = body
        .set_face_surface(seed.face, FaceSurface::New(flat.clone()))
        .unwrap();
    body.set_face_surface(split.face, FaceSurface::Shared(key))
        .unwrap();
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
}

/// SURVIVES: the mvfs seed story — Nurbs at rest is refused by name
/// (UncertifiableSurface), the gate on tiers 1–2 keeps mid-construction
/// states reported structurally (never geometric noise), and the
/// public setter path clears it.
#[test]
fn survives_nurbs_seed_gate_and_clear() {
    // Mid-construction (open) state: tier-3 must surface the TIER-2
    // failures verbatim, not geometric reports.
    let mut body = Body::<f64>::new();
    let _open_seed = body.mvfs(Point3::origin()).unwrap();
    let errs = validate_geometric(&body, Tol::witness()).unwrap_err();
    assert!(
        errs.iter()
            .all(|e| !matches!(e, ValidationError::UncertifiableSurface { .. })),
        "gate breached — geometric report on a structurally open body: {errs:?}"
    );
    // Closed lamina with the seed's Nurbs still in place: refused by
    // name at tier 3.
    let (mut body, seed, split) = lamina();
    let flat = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let key = body
        .set_face_surface(split.face, FaceSurface::New(flat.clone()))
        .unwrap();
    let errs = validate_geometric(&body, Tol::witness()).unwrap_err();
    assert!(
        errs.iter().any(
            |e| matches!(e, ValidationError::UncertifiableSurface { face } if *face == seed.face)
        ),
        "{errs:?}"
    );
    // Clearing it through the public setter makes tier 3 pass.
    body.set_face_surface(seed.face, FaceSurface::Shared(key))
        .unwrap();
    assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
    let _ = seed;
}

// =====================================================================
// Targets 1+2+8 combined — wrong-at-rest through the public API.
// =====================================================================

/// FIXED (was `finding_planar_face_arc_boundary_bulge_passes_tier3`,
/// S3): tier 3 used to accept a cube whose "planar" face was bounded
/// by an edge whose carrier is a half-circle bulging 0.5 m off that
/// face's plane — the planar-face pass checked VERTEX residuals only.
/// The fix: tier 3's check 5 re-uses the dihedral pass's interior
/// carrier samples and classifies them against each ADJACENT PLANAR
/// face's plane (linear cost, same schedule), reporting
/// `PlanarBoundaryResidual` per edge–face pair. The arc lies in the
/// bottom face's plane (z = 0, clean) but bulges off the front face's
/// (y = 0): exactly one report, naming the front face and the edge.
#[test]
fn fixed_planar_face_arc_boundary_bulge_reported_at_tier3() {
    let t = common::geometric_cube::<f64>();
    let mut body = t.body;
    // Upgrade the cube first (M2 PR 4 fix pass: transverse chords must
    // carry Intersection at rest), so the arc corruption below is the
    // only conventional description left in the body.
    common::describe_as_intersections(&mut body);
    // The bottom front edge A(0,0,0) -> B(1,0,0): re-describe as the
    // half-circle in the z = 0 plane bulging to y = −0.5 (a genuine,
    // honestly certified arc — description and carrier agree exactly).
    let edge = t.mevs[0].edge;
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::MappedCurve(MappedCurve::PlacedSegment {
            segment: SketchSegment::Arc {
                a: Point2::new(0.0, 0.0),
                b: Point2::new(1.0, 0.0),
                bulge: 1.0, // half circle
            },
            place: Affine3::identity(),
        }),
        carrier: Curve3::Circle {
            center: Point3::new(0.5, 0.0, 0.0),
            axis: Vec3::unit_z(),
            radius: 0.5,
            u_ref: Vec3::new(-1.0, 0.0, 0.0),
        },
        param_start: 0.0,
        param_end: core::f64::consts::PI,
    };
    body.set_edge_curve(edge, spec, Tol::witness()).unwrap();
    // The arc's apex is 0.5 m off the front face's plane (y = 0): tier
    // 3 reports exactly that face-boundary breach — plus the
    // prefer-intrinsic report for the same edge (the arc is a
    // definitely-transverse edge carrying a MappedCurve description,
    // which the M2 PR 4 enforcement names first in the shared edge
    // sweep) — and nothing else (the bottom face's plane contains the
    // whole arc).
    let front = t.mefs[1].face;
    assert_eq!(
        validate_geometric(&body, Tol::witness()),
        Err(vec![
            ValidationError::TransverseNotIntrinsic { edge },
            ValidationError::PlanarBoundaryResidual { face: front, edge },
        ]),
        "the planar-boundary pass must name the bulged-off face"
    );
}

/// FIXED (was
/// `finding_aliased_interval_survives_tier3_via_public_setter`, S1):
/// the winding-aliasing hole was reachable through the PUBLIC setter —
/// a wrong-by-8-full-turns parameter cache aliased the 9-sample
/// schedule at attachment AND at tier 3. The fix: the circle winding
/// bound (|t1 − t0| ≤ tau) refuses the whole 8k·tau alias family at
/// the gate, so the wrong cache never enters the body; the honest
/// half-circle interval still attaches and tier 3 stays clean.
#[test]
fn fixed_aliased_interval_refused_at_public_setter() {
    use core::f64::consts::{PI, TAU};
    let t = common::geometric_cube::<f64>();
    let mut body = t.body;
    // Upgrade first (M2 PR 4 fix pass — see the previous test) so the
    // at-rest reports below stay scoped to the attacked edge.
    common::describe_as_intersections(&mut body);
    let edge = t.mevs[0].edge;
    let mk = |t1: f64| EdgeCurveSpec {
        description: EdgeGeometry::MappedCurve(MappedCurve::PlacedSegment {
            segment: SketchSegment::Arc {
                a: Point2::new(0.0, 0.0),
                b: Point2::new(1.0, 0.0),
                bulge: 1.0,
            },
            place: Affine3::identity(),
        }),
        carrier: Curve3::Circle {
            center: Point3::new(0.5, 0.0, 0.0),
            axis: Vec3::unit_z(),
            radius: 0.5,
            u_ref: Vec3::new(-1.0, 0.0, 0.0),
        },
        param_start: 0.0,
        param_end: t1,
    };
    // A 1-turn-wrong interval is refused by the gate (as before —
    // interior samples caught it; now the winding bound names it).
    assert!(matches!(
        body.set_edge_curve(edge, mk(PI + TAU), Tol::witness()).unwrap_err(),
        EulerOpError::Certification { .. }
    ));
    // The 8-turn-wrong interval — the exact schedule alias — is now
    // refused by the same typed gate, and the body is untouched.
    assert!(matches!(
        body.set_edge_curve(edge, mk(PI + 8.0 * TAU), Tol::witness()).unwrap_err(),
        EulerOpError::Certification {
            error: geom_brep::CertifyError::WindingExceeded
        }
    ));
    // The honest half-circle interval attaches through the gate. (Its
    // apex genuinely bulges off the front face's plane, so tier 3's
    // planar-boundary pass — the S3 fix, previous test — reports that
    // geometric fact, and the M2 PR 4 prefer-intrinsic enforcement
    // reports the transverse arc's conventional description; what
    // matters here is that no certification/winding error remains: the
    // S1 gate's job is the winding.)
    body.set_edge_curve(edge, mk(PI), Tol::witness()).unwrap();
    assert_eq!(
        validate_geometric(&body, Tol::witness()),
        Err(vec![
            ValidationError::TransverseNotIntrinsic { edge },
            ValidationError::PlanarBoundaryResidual {
                face: t.mefs[1].face,
                edge,
            },
        ]),
        "honest interval at rest: only the attacked edge's at-rest reports remain"
    );
}

/// FIXED (was `finding_self_loop_dihedral_is_vacuous_at_rest`, B2+S3):
/// tier 3 used to be blind on self-loop edges (chord = 0 ⇒ margin = 0
/// ⇒ vacuously Smooth, and no boundary containment at all): a circular
/// scaffolding face claiming a plane a full meter away from its own
/// boundary circle — at a genuine 90° wedge to its neighbor — passed
/// clean. Now the extent arm is the carrier diameter (the 90° wedge
/// classifies definitely Transverse — real teeth, no sliver report),
/// and the planar-boundary pass reports the boundary circle lying off
/// the claimed plane.
#[test]
fn fixed_self_loop_dihedral_and_containment_have_teeth_at_rest() {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::origin()).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    let split = body
        .mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        }, Tol::witness())
        .unwrap();
    // Self-loop circular face at B (scaffolding circle in the z = 0
    // plane, center (2,0,0), radius 1).
    let circ = body
        .mef_chord(MefSite::Chords {
            he1: seg.he_minus,
            he2: seg.he_minus,
        }, Tol::witness())
        .unwrap();
    let z0 = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let y0 = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
    };
    body.set_face_surface(seed.face, FaceSurface::New(z0))
        .unwrap();
    body.set_face_surface(split.face, FaceSurface::New(y0.clone()))
        .unwrap();
    // The circular face claims the y = 0 plane — its boundary circle
    // reaches y = ±1, a meter off; and its wedge against z = 0 is a
    // true right angle. The wedge now classifies definitely Transverse
    // through the honest carrier-diameter arm (no vacuous Smooth, no
    // sliver report), and the containment breach is reported by name.
    // With the M2 PR 4 prefer-intrinsic enforcement, the transverse
    // classification ALSO names every one of these edges for its
    // conventional description (all three are definitely-transverse
    // MappedCurve chords/circles by construction here — the two A–B
    // chords sit on the z0/y0 corner, the self-loop on its right-angle
    // wedge), in edge-arena order, before the boundary report.
    body.set_face_surface(circ.face, FaceSurface::New(y0.clone()))
        .unwrap();
    assert_eq!(validate_closed(&body), Ok(()));
    assert_eq!(
        validate_geometric(&body, Tol::witness()),
        Err(vec![
            ValidationError::TransverseNotIntrinsic { edge: seg.edge },
            ValidationError::TransverseNotIntrinsic { edge: split.edge },
            ValidationError::TransverseNotIntrinsic { edge: circ.edge },
            ValidationError::PlanarBoundaryResidual {
                face: circ.face,
                edge: circ.edge,
            },
        ]),
        "tier 3 must see the self-loop boundary leave its claimed plane"
    );
}

// =====================================================================
// N4 (fix pass) — raw-operator precondition coverage: the sugar
// wrappers (`mev_line`, `mef_chord`) pre-resolve site keys/points
// before delegating, so through-sugar tests exercised the sugar's own
// resolution errors while the RAW operators' documented precondition
// paths went unexercised. These call the raw ops directly with
// well-formed specs and assert the documented first-failing
// precondition, body untouched (deep snapshot).
// =====================================================================

/// Raw `mev` preconditions: stale Fan half-edge, Fan start mismatch,
/// Lone on a non-empty loop — each the raw operator's own typed error,
/// each leaving the body deep-equal.
#[test]
fn fixed_n4_raw_mev_precondition_paths() {
    let (mut body, _seed, _mefs) = triangle_prism::<f64>();
    let snapshot = |b: &Body<f64>| format!("{b:?}");
    let before = snapshot(&body);
    let p = Point3::new(9.0, 9.0, 9.0);
    let spec = EdgeCurveSpec::line_between(Point3::origin(), p);

    // Stale he1 (the raw Fan resolution path, not the sugar's).
    let stale = topo::HalfEdgeKey::default();
    let err = body
        .mev(
            MevSite::Fan {
                he1: stale,
                he2: stale,
            },
            p,
            spec.clone(),
            Tol::witness(),
        )
        .unwrap_err();
    assert!(matches!(err, EulerOpError::StaleKey { .. }), "{err:?}");
    assert_eq!(snapshot(&body), before, "stale Fan mutated body");

    // Fan halves starting at different vertices: FanStartMismatch.
    let hes: Vec<_> = body.half_edges().map(|(k, he)| (k, he.start)).collect();
    let (he1, s1) = hes[0];
    let (he2, _) = hes
        .iter()
        .copied()
        .find(|&(_, s)| s != s1)
        .expect("prism has half-edges at distinct vertices");
    let err = body
        .mev(MevSite::Fan { he1, he2 }, p, spec.clone(), Tol::witness())
        .unwrap_err();
    assert!(
        matches!(err, EulerOpError::FanStartMismatch { .. }),
        "{err:?}"
    );
    assert_eq!(snapshot(&body), before, "fan mismatch mutated body");

    // Lone on a non-empty (cycle) loop: LoopNotEmpty from the raw op.
    let cycle_loop = body.get_half_edge(he1).unwrap().parent_loop;
    let err = body
        .mev(MevSite::Lone { r#loop: cycle_loop }, p, spec.clone(), Tol::witness())
        .unwrap_err();
    assert!(matches!(err, EulerOpError::LoopNotEmpty { .. }), "{err:?}");
    assert_eq!(snapshot(&body), before, "lone-on-cycle mutated body");
}

/// Raw `mef` preconditions: stale Chords half-edge, Chords across two
/// loops, Lone on a non-empty loop — each the raw operator's own typed
/// error, each leaving the body deep-equal.
#[test]
fn fixed_n4_raw_mef_precondition_paths() {
    let (mut body, _seed, _mefs) = triangle_prism::<f64>();
    let snapshot = |b: &Body<f64>| format!("{b:?}");
    let before = snapshot(&body);
    let spec = EdgeCurveSpec::line_between(Point3::origin(), Point3::new(9.0, 9.0, 9.0));

    // Stale he2 in a Chords site.
    let he1 = body.half_edges().next().map(|(k, _)| k).unwrap();
    let stale = topo::HalfEdgeKey::default();
    let err = body
        .mef(
            MefSite::Chords { he1, he2: stale },
            spec.clone(),
            FaceSurface::Inherit,
            Tol::witness(),
        )
        .unwrap_err();
    assert!(matches!(err, EulerOpError::StaleKey { .. }), "{err:?}");
    assert_eq!(snapshot(&body), before, "stale Chords mutated body");

    // Chords across two different loops: NotSameLoop.
    let l1 = body.get_half_edge(he1).unwrap().parent_loop;
    let hes: Vec<_> = body
        .half_edges()
        .map(|(k, he)| (k, he.parent_loop))
        .collect();
    let (he2, _) = hes
        .iter()
        .copied()
        .find(|&(_, l)| l != l1)
        .expect("prism has multiple loops");
    let err = body
        .mef(
            MefSite::Chords { he1, he2 },
            spec.clone(),
            FaceSurface::Inherit,
            Tol::witness(),
        )
        .unwrap_err();
    assert!(matches!(err, EulerOpError::NotSameLoop { .. }), "{err:?}");
    assert_eq!(snapshot(&body), before, "cross-loop Chords mutated body");

    // Lone on a non-empty (cycle) loop: LoopNotEmpty from the raw op.
    let err = body
        .mef(
            MefSite::Lone { r#loop: l1 },
            spec.clone(),
            FaceSurface::Inherit,
            Tol::witness(),
        )
        .unwrap_err();
    assert!(matches!(err, EulerOpError::LoopNotEmpty { .. }), "{err:?}");
    assert_eq!(snapshot(&body), before, "lone-on-cycle mutated body");
}

// =====================================================================
// Interval lane: the e2e prism and the setters, at the certified
// interval scalar (target 6 + 3d).
// =====================================================================
#[cfg(feature = "interval")]
mod interval_lane {
    use super::*;
    use geom_core::{Interval, Real};

    /// FIXED (was `finding_interval_lane_refuses_non_dyadic_rims`,
    /// BLOCKER B1): the mini-extrude e2e could not run at the interval
    /// scalar — the very first non-axis-aligned rim mint was refused
    /// because `norm`'s old `sqrt(dot(v,v))` squared a zero-straddling
    /// difference enclosure through plain interval `Mul` (negative
    /// lo), the sqrt clamped, and the decoration poisoned every
    /// decision. With `norm_squared` computing tight per-component
    /// squares, the FULL mini-extrude e2e now runs at `Interval`:
    /// non-dyadic rims, Newell side planes, the seed-cap setter, the
    /// prefer-intrinsic upgrade of all nine edges, and tiers 1–3 clean
    /// after the upgrade (the M2 PR 4 enforcement names the nine
    /// pre-upgrade conventional descriptions at rest) — Q1's replay
    /// lane works for real (non-dyadic) geometry, which is exactly
    /// what PR 4's extrude and PR 5's revolve need.
    #[test]
    fn fixed_interval_lane_runs_full_mini_extrude_e2e() {
        let (mut body, _seed, _mefs) = triangle_prism::<Interval>();
        assert_eq!(body.vertices().count(), 6);
        assert_eq!(body.edges().count(), 9);
        assert_eq!(body.faces().count(), 5);
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(validate_closed(&body), Ok(()));
        // Prefer-intrinsic enforcement (M2 PR 4 fix pass): the interval
        // lane classifies the nine transverse corners definitely too,
        // and names their pre-upgrade conventional descriptions.
        let errs = validate_geometric(&body).unwrap_err();
        assert_eq!(errs.len(), 9, "{errs:?}");
        assert!(
            errs.iter()
                .all(|e| matches!(e, ValidationError::TransverseNotIntrinsic { .. })),
            "{errs:?}"
        );
        // The prefer-intrinsic upgrade at the interval scalar.
        common::describe_as_intersections(&mut body);
        assert_eq!(validate_geometric(&body), Ok(()));
        assert!(body.curves().all(|(_, c)| matches!(
            c.certified().map(topo::EdgeCurve::description),
            Some(EdgeGeometry::Intersection { .. })
        )));
    }

    /// FIXED (was `finding_interval_lane_refuses_self_loop_scaffolding`,
    /// B1's scaffolding side): the self-loop sugar — `mef_chord` on a
    /// self-loop site attaches the canonical circle — was refused at
    /// Interval even on fully dyadic coordinates (the full-period
    /// endpoint at tau is inexact). It now certifies: interval REPLAY
    /// of f64 histories that used self-loop scaffolding works, keeping
    /// Q1's replay-shares-topology story intact.
    #[test]
    fn fixed_interval_lane_certifies_self_loop_scaffolding() {
        let f = Interval::from_f64;
        let mut body = Body::<Interval>::new();
        let seed = body.mvfs(Point3::new(f(0.0), f(0.0), f(0.0))).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(f(1.0), f(0.0), f(0.0)),
            )
            .unwrap(); // axis-aligned dyadic: certifies
        let split = body
            .mef_chord(MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            })
            .unwrap(); // dyadic digon chord: certifies
        // The self-loop site: the sugar's scaffolding circle now
        // certifies at Interval (tight squares ⇒ clean [0, hi]
        // residual enclosures ⇒ Zero, decoration intact).
        let circ = body
            .mef_chord(MefSite::Chords {
                he1: seg.he_minus,
                he2: seg.he_minus,
            })
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        let _ = (split, circ);
    }
}
