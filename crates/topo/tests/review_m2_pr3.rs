//! M2 PR 3 adversarial review — body-level attacks through the PUBLIC
//! API only (falsification targets 2, 6, 8 of the review spec; the
//! certification-layer attacks live in
//! `geom-brep/tests/review_m2_pr3_certify.rs`).
//!
//! Convention: `finding_*` tests PIN CURRENT defective/questionable
//! behavior (the fix pass flips their assertions); `survives_*` tests
//! are attacks the implementation resisted and promote to CI verbatim.
//! `e2e_*` is the mini-extrude consumer program demanded by the spec.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{MappedCurve, SketchSegment, newell_plane};
use geom_core::{Affine3, Band, Decide, Point2, Point3, Tolerance, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;
use topo::{
    Body, EdgeCurveSpec, EdgeGeometry, EulerOpError, FaceSurface, MefSite, MevSite, SurfaceKey,
    ValidationError, validate, validate_closed, validate_geometric,
};

mod common;

fn band() -> Band {
    Band::linear().unwrap()
}

fn eps() -> f64 {
    Tolerance::get().eps
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
        )
        .unwrap();
    // Struts (ExtrudedPoint descriptions — the real sweep shape).
    let strut = |body: &mut Body<T>, at, s0, p0, p_top| {
        body.mev(MevSite::Fan { he1: at, he2: at }, p_top, strut_spec(s0, p0))
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
/// prefer-intrinsic upgrade at rest, tiers 1–3 clean before AND after.
#[test]
fn e2e_mini_extrude_triangle_prism_passes_tiers_and_upgrades() {
    let (mut body, _seed, _mefs) = triangle_prism::<f64>();
    assert_eq!(body.vertices().count(), 6);
    assert_eq!(body.edges().count(), 9);
    assert_eq!(body.faces().count(), 5);
    assert_eq!(body.surfaces().count(), 5);
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(validate_closed(&body), Ok(()));
    assert_eq!(validate_geometric(&body), Ok(()));
    // The prefer-intrinsic upgrade: every edge of the prism is a
    // transverse plane-pair corner; all nine upgrade via set_edge_curve.
    common::upgrade_edges_to_intersections(&mut body);
    assert_eq!(validate_geometric(&body), Ok(()));
    assert!(
        body.curves()
            .all(|(_, c)| matches!(c.description(), EdgeGeometry::Intersection { .. }))
    );
    // Determinism (D9): a replayed build is certificate-identical.
    let (body2, _, _) = triangle_prism::<f64>();
    let dump = |b: &Body<f64>| {
        b.curves()
            .map(|(k, c)| format!("{k:?} {:?} {:?}\n", c.params(), c.certificate()))
            .collect::<String>()
    };
    let (fresh, _, _) = triangle_prism::<f64>();
    assert_eq!(dump(&body2), dump(&fresh));
}

/// The same consumer at the Dual lane: value channel takes the
/// identical certified path (tangents never decide).
#[test]
fn e2e_prism_dual_lane_matches_f64() {
    use geom_core::Dual64;
    let (f, _, _) = triangle_prism::<f64>();
    let (d, _, _) = triangle_prism::<Dual64>();
    assert_eq!(validate_geometric(&d), Ok(()));
    let fr: Vec<f64> = f
        .curves()
        .map(|(_, c)| c.certificate().max_residual)
        .collect();
    let dr: Vec<f64> = d
        .curves()
        .map(|(_, c)| c.certificate().max_residual.value)
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
    let err = body.set_edge_curve(ek, spec).unwrap_err();
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
    let err = body.set_edge_curve(ek, spec).unwrap_err();
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
    common::upgrade_edges_to_intersections(&mut body);
    assert_eq!(validate_geometric(&body), Ok(()));

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
    let errs = validate_geometric(&body).unwrap_err();
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
        )
        .unwrap();
    let split = body
        .mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        })
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
    body.set_face_surface(seed.face, FaceSurface::New(flat))
        .unwrap();
    body.set_face_surface(split.face, FaceSurface::New(tilted))
        .unwrap();
    assert_eq!(validate_closed(&body), Ok(()));
    let errs = validate_geometric(&body).unwrap_err();
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
        .set_face_surface(seed.face, FaceSurface::New(flat))
        .unwrap();
    body.set_face_surface(split.face, FaceSurface::Shared(key))
        .unwrap();
    assert_eq!(validate_geometric(&body), Ok(()));
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
    let errs = validate_geometric(&body).unwrap_err();
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
        .set_face_surface(split.face, FaceSurface::New(flat))
        .unwrap();
    let errs = validate_geometric(&body).unwrap_err();
    assert!(
        errs.iter().any(
            |e| matches!(e, ValidationError::UncertifiableSurface { face } if *face == seed.face)
        ),
        "{errs:?}"
    );
    // Clearing it through the public setter makes tier 3 pass.
    body.set_face_surface(seed.face, FaceSurface::Shared(key))
        .unwrap();
    assert_eq!(validate_geometric(&body), Ok(()));
    let _ = seed;
}

// =====================================================================
// Targets 1+2+8 combined — wrong-at-rest through the public API.
// =====================================================================

/// FINDING: tier 3 accepts a cube whose "planar" faces are bounded by
/// an edge whose carrier is a half-circle bulging 0.5 m off one of its
/// adjacent face planes. The planar-face pass checks VERTEX residuals
/// only; MappedCurve descriptions carry no surface reference, so
/// nothing ties an edge's carrier to its faces' surfaces. (The
/// documented not-checked list names "face-boundary containment on
/// CURVED surfaces (M3)" — but this is a PLANAR face, whose reader
/// expects the planar pass to have teeth.)
#[test]
fn finding_planar_face_arc_boundary_bulge_passes_tier3() {
    let t = common::geometric_cube::<f64>();
    let mut body = t.body;
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
    body.set_edge_curve(edge, spec).unwrap();
    // The arc's apex is 0.5 m off the front face's plane (y = 0), yet:
    assert_eq!(
        validate_geometric(&body),
        Ok(()),
        "FINDING pinned: tier 3 accepts a planar face bounded by an arc \
         bulging half a meter off its plane"
    );
}

/// FINDING: the winding-aliasing hole is reachable through the PUBLIC
/// setter and survives tier 3 at rest — a wrong-by-8-full-turns
/// parameter cache on a cube edge is certified at attachment
/// (set_edge_curve) and re-certified clean by validate_geometric (the
/// same 9-sample schedule aliases both times). Wrong-but-certified,
/// at rest, from outside.
#[test]
fn finding_aliased_interval_survives_tier3_via_public_setter() {
    use core::f64::consts::{PI, TAU};
    let t = common::geometric_cube::<f64>();
    let mut body = t.body;
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
    // Sanity: a 1-turn-wrong interval IS refused by the gate.
    assert!(matches!(
        body.set_edge_curve(edge, mk(PI + TAU)).unwrap_err(),
        EulerOpError::Certification { .. }
    ));
    // The 8-turn-wrong interval passes the gate AND tier 3.
    body.set_edge_curve(edge, mk(PI + 8.0 * TAU)).unwrap();
    assert_eq!(
        validate_geometric(&body),
        Ok(()),
        "FINDING pinned: aliased parameter cache is wrong-but-certified at rest"
    );
}

/// FINDING: tier 3's dihedral pass is VACUOUS on self-loop edges (chord
/// = 0 ⇒ margin = 0 ⇒ definitely Smooth): a circular scaffolding face
/// whose surface is a plane a full meter away from its own boundary
/// circle — and at a genuine 90-degree wedge to its neighbor — passes
/// tier 3 clean. The same zero-arm fold that refuses full-period
/// Intersection rims (geom-brep suite) here ACCEPTS garbage silently.
#[test]
fn finding_self_loop_dihedral_is_vacuous_at_rest() {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::origin()).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, 0.0),
        )
        .unwrap();
    let split = body
        .mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        })
        .unwrap();
    // Self-loop circular face at B (scaffolding circle in the z = 0
    // plane, center (2,0,0), radius 1).
    let circ = body
        .mef_chord(MefSite::Chords {
            he1: seg.he_minus,
            he2: seg.he_minus,
        })
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
    body.set_face_surface(split.face, FaceSurface::New(y0))
        .unwrap();
    // The circular face claims the y = 0 plane — its boundary circle
    // reaches y = ±1, a meter off; and its wedge against z = 0 is a
    // true right angle. Both are invisible: chord 0 ⇒ Smooth.
    body.set_face_surface(circ.face, FaceSurface::New(y0))
        .unwrap();
    assert_eq!(validate_closed(&body), Ok(()));
    assert_eq!(
        validate_geometric(&body),
        Ok(()),
        "FINDING pinned: self-loop edges make tier 3's dihedral and \
         containment blind"
    );
}

// =====================================================================
// Interval lane: the e2e prism and the setters, at the certified
// interval scalar (target 6 + 3d).
// =====================================================================
#[cfg(feature = "interval")]
mod interval_lane {
    use super::*;
    use geom_brep::CertCheck;
    use geom_core::{Interval, Real};

    /// FINDING (BLOCKER): the mini-extrude e2e CANNOT run at the
    /// interval scalar. The very first non-axis-aligned rim mint (the
    /// B → C profile edge) is refused: `carrier(t1) = B + dir·len`
    /// does not reproduce C's singleton exactly, the difference
    /// enclosure straddles zero, `norm`'s `sqrt(dot(v,v))` squares the
    /// straddle through plain interval Mul (negative lo), the sqrt
    /// clamps to decoration Trv, and Decide reads that as poison:
    /// `Escalated { EndpointEnd, margin: Invalid }`. Consequence: the
    /// interval replay lane (Q1's pure-replay model) only works for
    /// exactly-dyadic axis-aligned geometry — the existing interval
    /// cube tests pass by that accident, and PR 4's extrude (any real
    /// profile) will be uncertifiable at Interval until the distance
    /// enclosures are computed poison-free (tight even powers /
    /// pown-style squaring in `norm_squared`, or an at-zero clamp that
    /// preserves the decoration).
    #[test]
    fn finding_interval_lane_refuses_non_dyadic_rims() {
        let f = Interval::from_f64;
        let b3 = Point3::new(f(1.0), f(0.0), f(0.0));
        let c3 = Point3::new(f(0.3), f(0.8), f(0.0));
        let mut body = Body::<Interval>::new();
        let seed = body.mvfs(b3).unwrap();
        // The rim spec exactly as the prism builder constructs it.
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::MappedCurve(MappedCurve::PlacedSegment {
                segment: SketchSegment::Line {
                    a: Point2::new(f(1.0), f(0.0)),
                    b: Point2::new(f(0.3), f(0.8)),
                },
                place: Affine3::identity(),
            }),
            carrier: Curve3::Line {
                origin: b3,
                dir: (c3 - b3) / b3.distance(c3),
            },
            param_start: Interval::from_f64(0.0),
            param_end: b3.distance(c3),
        };
        let err = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                c3,
                spec,
            )
            .unwrap_err();
        assert!(
            matches!(
                err,
                EulerOpError::Certification {
                    error: geom_brep::CertifyError::Escalated {
                        check: CertCheck::EndpointEnd,
                        ..
                    }
                }
            ),
            "FINDING pinned: interval lane refuses non-dyadic geometry: {err:?}"
        );
    }

    /// FINDING (same blocker, scaffolding side): the self-loop sugar —
    /// `mef_chord` on a self-loop site attaches the canonical circle —
    /// is refused at Interval even on fully dyadic coordinates (the
    /// full-period endpoint at tau is inexact). Interval REPLAY of any
    /// f64 history that used self-loop scaffolding therefore fails,
    /// breaking Q1's replay-shares-topology story.
    #[test]
    fn finding_interval_lane_refuses_self_loop_scaffolding() {
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
        // The self-loop site: the sugar's scaffolding circle poisons.
        let err = body
            .mef_chord(MefSite::Chords {
                he1: seg.he_minus,
                he2: seg.he_minus,
            })
            .unwrap_err();
        assert!(
            matches!(err, EulerOpError::Certification { .. }),
            "FINDING pinned: self-loop scaffolding dies at Interval: {err:?}"
        );
        let _ = split;
    }
}
