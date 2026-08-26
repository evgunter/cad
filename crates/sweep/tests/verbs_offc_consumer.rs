//! **The body-reachable consumer**: a lofted prism's NURBS walls carry
//! certified `Surface::Approx` surfaces at rest, and tier 3 validates
//! the body end to end by RE-DERIVING each face's certificate per call.
//!
//! All four walls convert together, and that is forced rather than
//! chosen: a vertical edge is shared by two walls, and the iso lane's
//! seam class requires the carrier and the chart's boundary row to sit
//! in ONE spline space, so one converted wall beside three unconverted
//! ones has no consistent carrier. The caps stay planar.
//!
//! # What the consumer is, and what it deliberately is not
//!
//! No new public verb. The replacement runs through the attach layer's
//! existing doors in the M6-1 surgery order — surfaces
//! (`set_face_surface`), then descriptions, then `mint_pcurves` — which
//! is the smallest honest path that makes an `Approx` face reachable.
//! A named `replace_face_surface` verb would be OFF-D's face-
//! replacement primitive built early, and the shell/rim surgery it
//! implies is fenced out of this unit.
//!
//! # Why the base is the wall PULLED BACK by `d`
//!
//! A face's edges lie on its surface. Replacing a wall's surface with
//! the offset of that same wall would move the face `d` away from its
//! own boundary — geometrically incoherent, and every edge certificate
//! would (correctly) go red. So the description used here names the
//! base whose OFFSET is the wall: the wall's own control net pulled
//! back `d` along the chart normal. `Offset { base, d }` then
//! describes exactly the surface the face already had, the fit
//! reproduces it to the certified tolerance, and what is under test is
//! the storage, the delegation and the re-derivation rather than a
//! geometry change nothing else in the body knows about.
//!
//! That the wall is planar is what makes the pull-back exact: a rigid
//! translation of a planar net has the same chart normal everywhere,
//! so `base + d·n` is the wall, bit for bit in exact arithmetic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, NurbsSurface, Surface};
use geom_brep::EdgeCurveSpec;
use geom_brep::keys::SurfaceKey;
use geom_core::{Affine3, Band, Point2, Tol, Vec3};
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use std::collections::HashMap;
use topo::{Body, CurveGeom, EdgeKey, FaceKey, FaceSurface};

/// The offset fit door's degree — the spline space every wall carrier
/// is elevated into (`geom_brep::offset_fit::OFFSET_FIT_DEGREE`).
const FIT_DEGREE: usize = geom_brep::offset_fit::OFFSET_FIT_DEGREE;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A straight-walled square prism lofted between two identical
/// sections — four PLANAR described-NURBS walls.
fn prism() -> Body<f64> {
    let v = |x: f64, y: f64| ProfileVertex::new(Point2::new(x, y), 0.0);
    let square = || {
        vec![ProfileLoop::new(vec![
            v(0.0, 0.0),
            v(2.0, 0.0),
            v(2.0, 2.0),
            v(0.0, 2.0),
        ])]
    };
    let places = [0.0, 1.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect::<Vec<_>>();
    sweep::loft_body::<f64>(&[square(), square()], &places, 1, Tol::witness())
        .expect("the square prism lofts")
        .body
}

/// The wall's net pulled back `d` along its chart normal — the base
/// whose offset at `+d` is the wall itself.
fn pulled_back(wall: &NurbsSurface<f64>, d: f64) -> NurbsSurface<f64> {
    let (u0, u1) = wall.knots_u().domain();
    let (v0, v1) = wall.knots_v().domain();
    let jet = wall.ders((u0 + u1) * 0.5, (v0 + v1) * 0.5);
    let n = jet.du.cross(jet.dv).normalize();
    NurbsSurface::new(
        wall.knots_u().clone(),
        wall.knots_v().clone(),
        wall.control().iter().map(|p| *p - n * d).collect(),
        wall.weights().to_vec(),
    )
    .expect("a translated net is a valid surface")
}

/// **The surgery**, in the M6-1 order: every wall's surface first
/// (`set_face_surface`), then every wall edge's DESCRIPTION re-pointed
/// at the new surface key, then `mint_pcurves`.
///
/// Two things beyond the surface write, both forced by the kind rather
/// than chosen:
///
/// - **Re-description.** `FaceSurface::New` mints a fresh arena key, so
///   every `IsoCurve` description naming the old wall is stale. Tier 3
///   says so (`DescriptionNotAdjacent`); the fix is the ordering
///   discipline's own second step.
/// - **Degree elevation of the wall carriers.** The iso lane's seam
///   class bounds `|B(v) − C(v)|` by a control-difference hull, which
///   is a partition-of-unity argument and therefore needs the chart's
///   boundary row and the carrier in ONE spline space. A loft wall is
///   bilinear; its offset fit is bicubic. Degree elevation is exact —
///   same locus, same parameterization, same endpoints — so raising
///   each wall carrier to the fit's space is a representation change,
///   not rim surgery. (Moving the carrier's LOCUS would be, and that
///   is OFF-D's.)
///
/// All four walls convert together: a vertical edge is shared by two
/// of them, and both sides have to agree on the carrier's spline
/// space. The two fits are built from congruent bases, so they land in
/// the same one.
fn approx_walls(body: &mut Body<f64>, d: f64, tolerance: f64) -> Vec<FaceKey> {
    let walls: Vec<(FaceKey, Arc<NurbsSurface<f64>>)> = body
        .faces()
        .filter_map(|(key, face)| match body.get_surface(face.surface) {
            Some(Surface::Nurbs(payload)) if !payload.is_placeholder() => {
                Some((key, Arc::clone(payload)))
            }
            _ => None,
        })
        .collect();
    assert_eq!(walls.len(), 4, "the square prism has four walls");

    // ---- 1: surfaces.
    let mut remap: HashMap<SurfaceKey, SurfaceKey> = HashMap::new();
    let mut faces = Vec::new();
    for (face, wall) in walls {
        let old = body.get_face(face).unwrap().surface;
        let base = Arc::new(pulled_back(&wall, d));
        let approx = geom_brep::approx_offset_surface(base, d, tolerance, band())
            .unwrap_or_else(|e| panic!("d = {d}: a planar wall's offset must fit: {e}"));
        let new = body
            .set_face_surface(face, FaceSurface::New(approx))
            .expect("the attach-layer door accepts a live face");
        remap.insert(old, new);
        faces.push(face);
    }

    // ---- 2: descriptions (and the carriers' spline space).
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    for edge in edges {
        let ck = body.get_edge(edge).unwrap().curve;
        let Some(curve) = body.get_curve_geom(ck).and_then(CurveGeom::certified) else {
            continue;
        };
        let re = curve
            .with_remapped_surfaces(|k| Some(remap.get(&k).copied().unwrap_or(k)))
            .expect("the remap answers for every key");
        let carrier = match re.carrier() {
            Curve3::Nurbs(c) if c.knots().degree() < FIT_DEGREE => Curve3::Nurbs(Arc::new(
                c.elevate_degree(FIT_DEGREE - c.knots().degree())
                    .expect("degree elevation of a clamped carrier"),
            )),
            other => other.clone(),
        };
        let (param_start, param_end) = re.params();
        body.set_edge_curve(
            edge,
            EdgeCurveSpec {
                description: *re.description(),
                carrier,
                param_start,
                param_end,
            },
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("edge {edge:?} re-attach: {e}"));
    }

    // ---- 3: the pcurve map.
    topo::mint_pcurves(body, Tol::witness()).expect("the Approx charts mint their iso images");
    faces
}

/// Builds the prism with all four walls carrying certified `Approx`
/// surfaces at the given signed distance.
fn prism_with_approx_walls(d: f64, tolerance: f64) -> (Body<f64>, Vec<FaceKey>) {
    let mut body = prism();
    let faces = approx_walls(&mut body, d, tolerance);
    (body, faces)
}

// ---------------------------------------------------------------------

/// **The end-to-end row, both signs of `d`.** The body carries an
/// `Approx` face and validates at tier 3 — which means the per-face
/// re-derivation ran and agreed, the face's edges still certify
/// against the new chart, and its pcurves re-minted through the spline
/// lane on the fit.
#[test]
fn an_approx_faced_body_validates_at_tier_three() {
    for d in [0.05_f64, -0.05] {
        let (body, faces) = prism_with_approx_walls(d, 1e-9);
        let face = faces[0];
        assert!(
            matches!(
                body.get_surface(body.get_face(face).unwrap().surface),
                Some(Surface::Approx(_))
            ),
            "d = {d}: the face carries the approximating surface"
        );
        assert_eq!(
            topo::validate_geometric(&body, Tol::witness()),
            Ok(()),
            "d = {d}: tier 3 on an Approx-faced body"
        );
    }
}

/// Every half-edge of the `Approx` face carries a STORED pcurve cache
/// at rest: the chart mints, and it mints through the spline lane —
/// the iso images of the fit, not a closed-form harmonic table.
#[test]
fn the_approx_face_mints_its_iso_caches() {
    let (body, faces) = prism_with_approx_walls(0.05, 1e-9);
    let face = faces[0];
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the wall's outer loop is a cycle");
    };
    let mut hes = 0usize;
    for he in body.loop_cycle(first).unwrap() {
        hes += 1;
        let cache = body
            .pcurve(he)
            .unwrap_or_else(|| panic!("half-edge {he:?} of an Approx face carries no cache"));
        assert!(
            matches!(cache.pcurve(), topo::Pcurve::IsoLine { .. }),
            "a planar wall's rims and seams are exact iso lines, got {:?}",
            cache.pcurve()
        );
    }
    assert_eq!(hes, 4, "the wall is a quadrilateral");
}

/// **The never-trust posture, at the body.** Degrade the stored fit
/// behind an otherwise-valid body — the surface keeps a certificate
/// that says it is fine — and tier 3 reports `ApproxCertification`
/// naming the face. The stored certificate is never consulted.
#[test]
fn a_degraded_fit_on_a_face_goes_red_at_tier_three() {
    let d = 0.05;
    let (mut body, faces) = prism_with_approx_walls(d, 1e-9);
    let face = faces[0];
    let Some(Surface::Approx(live)) = body.get_surface(body.get_face(face).unwrap().surface) else {
        panic!("the wall carries an approximating surface")
    };
    let geom::SurfaceDescription::Offset { base, .. } = live.description();
    let base = Arc::clone(base);
    let honest = geom_brep::approx_offset_surface(Arc::clone(&base), d, 1e-9, band()).unwrap();
    let Surface::Approx(good) = &honest else {
        panic!("the door mints the variant")
    };

    // Coarsen one control point of the FIT by a millimetre, and keep
    // the honest certificate: a planted claim.
    let fit = good.fit();
    let mut control = fit.control().to_vec();
    control[0] = control[0] + Vec3::new(0.0, 0.0, 1e-3);
    let coarsened = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        control,
        fit.weights().to_vec(),
    )
    .unwrap();
    let planted = geom::ApproxSurface::certify(
        geom::SurfaceSpec {
            description: geom::SurfaceDescription::Offset { base, d },
            fit: coarsened,
            window: good.window(),
            tolerance: good.tolerance(),
        },
        |_, _, _| Ok::<_, geom_brep::OffsetFitError>(*good.certificate()),
    )
    .unwrap();

    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(planted))))
        .unwrap();
    // The map re-mint may itself refuse on the moved chart; the claim
    // under test is tier 3's, so run it whatever the mint said.
    let _ = topo::mint_pcurves(&mut body, Tol::witness());
    let errors = topo::validate_geometric(&body, Tol::witness())
        .expect_err("a degraded fit must not validate");
    assert!(
        errors.iter().any(|e| matches!(
            e,
            topo::ValidationError::ApproxCertification { face: f, .. } if *f == face
        )),
        "tier 3 must report the re-derivation failure on face {face:?}, got {errors:?}"
    );
}

/// **The boolean gate refuses the operand pair-scoped**, by kind, and
/// does NOT treat the face as the NURBS its fit is: a plain
/// NURBS-walled prism passes the same gate.
#[test]
fn the_boolean_gate_refuses_an_approx_operand_by_kind() {
    let (approx_body, faces) = prism_with_approx_walls(0.05, 1e-9);
    let face = faces[0];
    let e = topo::union(&approx_body, &prism(), Tol::witness())
        .expect_err("an Approx operand is unsupported-kind for the boolean gate");
    assert!(
        matches!(
            e,
            topo::BooleanError::CurvedBooleanUnsupported { kind: geom_brep::SurfaceKind::Approx, face: f, .. } if f == face
        ),
        "expected the pair-scoped gate refusal naming SurfaceKind::Approx on {face:?}, got {e}"
    );
}

/// **Tessellation delegates**: the `Approx` face meshes through the
/// spline lane on its fit, producing triangles like any described
/// NURBS wall.
#[test]
fn the_approx_face_tessellates_through_the_delegate_path() {
    let (body, faces) = prism_with_approx_walls(0.05, 1e-9);
    let face = faces[0];
    let mesh = mesh::tessellate(&body, 0.05, Tol::witness()).expect("an Approx-faced body meshes");
    let patch = mesh
        .patches
        .iter()
        .find(|p| p.face == face)
        .expect("the Approx face has a patch");
    assert!(
        !patch.triangles.is_empty(),
        "the delegate path produced no triangles for the Approx face"
    );
}

/// **Transform refuses typed** on an `Approx` face: the composition law
/// holds (pinned in `geom-brep`'s suite), but this pass cannot
/// re-derive the mapped certificate, so it declines rather than carry
/// an unre-derived claim across a geometry change.
#[test]
fn transform_refuses_an_approx_face_typed() {
    let (body, _) = prism_with_approx_walls(0.05, 1e-9);
    let e = topo::transform_rigid(
        &body,
        &Affine3::translation(Vec3::new(1.0, 0.0, 0.0)),
        Tol::witness(),
    )
    .expect_err("the transform pass refuses the kind");
    assert!(
        matches!(e, topo::TransformError::ApproxSurface),
        "expected the typed Approx refusal, got {e}"
    );
}
