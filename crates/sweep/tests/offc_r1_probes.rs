//! OFF-C reviewer probes (r1): an `Approx`-faced body whose walls are
//! genuinely CURVED — a twisted loft (skinned bilinear saddles), not
//! the PR's straight prism — validated tier 3 end to end, tessellated,
//! and driven at the boolean doors.
//!
//! What these rows add over `verbs_offc_consumer.rs`:
//!
//! - the pulled-back-base construction on a NON-planar wall, where the
//!   pull-back is only approximately the inverse of the offset (the
//!   chart normal varies over a saddle), so `d` must be small enough
//!   that the description's locus stays inside the edge bands — the
//!   first body-level exercise of the re-derivation arithmetic on a
//!   curved chart;
//! - the never-trust red row on that curved chart;
//! - the boolean refusal SHAPE on a lofted `Approx` operand (the edge
//!   rule fires first — recorded, not assumed) beside the germ-pair
//!   refusal from a doubly-curved SKINNED base on a box cap.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, NurbsSurface, Surface};
use geom_brep::EdgeCurveSpec;
use geom_brep::keys::SurfaceKey;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use std::collections::HashMap;
use topo::{Body, CurveGeom, EdgeKey, FaceKey, FaceSurface};

const FIT_DEGREE: usize = geom_brep::offset_fit::OFFSET_FIT_DEGREE;

/// Small enough that the pull-back error `d·(n(u,v) − n₀)` on the
/// twisted walls stays well inside the default ε = 1e-9 band (at
/// d = 2e-9 the edge residual measured 2.3e-9 and ESCALATED in the
/// ambiguity band — the honest record of how tight the coherence
/// budget is for a curved pulled-back base); large enough to be far
/// above f64 dust.
const D: f64 = 5e-10;
const FIT_TOL: f64 = 1e-6;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A loft between a square and the SAME square rotated 0.05 rad about
/// its own centre (rotation applied to the 2-D section, places pure
/// translations) — four congruent bilinear saddle walls (nonplanar).
fn twisted_loft() -> Body<f64> {
    let v = |x: f64, y: f64| ProfileVertex::new(Point2::new(x, y), 0.0);
    let rotated = |theta: f64| {
        let (s, c) = theta.sin_cos();
        let rv = move |x: f64, y: f64| {
            let (dx, dy) = (x - 1.0, y - 1.0);
            ProfileVertex::new(
                Point2::new(1.0 + c * dx - s * dy, 1.0 + s * dx + c * dy),
                0.0,
            )
        };
        vec![ProfileLoop::new(vec![
            rv(0.0, 0.0),
            rv(2.0, 0.0),
            rv(2.0, 2.0),
            rv(0.0, 2.0),
        ])]
    };
    let square = vec![ProfileLoop::new(vec![
        v(0.0, 0.0),
        v(2.0, 0.0),
        v(2.0, 2.0),
        v(0.0, 2.0),
    ])];
    let places = vec![
        Affine3::translation(Vec3::new(0.0, 0.0, 0.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
    ];
    sweep::loft_body::<f64>(&[square, rotated(0.05)], &places, 1, Tol::witness())
        .expect("the twisted square lofts")
        .body
}

/// The wall's net pulled back `d` along its MID-POINT chart normal —
/// exact on a plane, approximate (to `d·Δn`) on a saddle.
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

/// The consumer surgery, in the M6-1 order (surfaces → descriptions +
/// carrier spline space → pcurves), on every non-placeholder NURBS
/// wall.
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
    assert_eq!(walls.len(), 4, "the twisted loft has four spline walls");

    let mut remap: HashMap<SurfaceKey, SurfaceKey> = HashMap::new();
    let mut faces = Vec::new();
    let mut fit_interior_v: Vec<f64> = Vec::new();
    for (face, wall) in walls {
        let old = body.get_face(face).unwrap().surface;
        let base = Arc::new(pulled_back(&wall, d));
        let approx = geom_brep::approx_offset_surface(base, d, tolerance, band())
            .unwrap_or_else(|e| panic!("d = {d}: the saddle wall's offset must fit: {e}"));
        if let Surface::Approx(a) = &approx {
            let kv = a.fit().knots_v().knots();
            fit_interior_v = kv[4..kv.len() - 4].to_vec();
        }
        let new = body
            .set_face_surface(face, FaceSurface::New(approx))
            .expect("the attach-layer door accepts a live face");
        remap.insert(old, new);
        faces.push(face);
    }

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
            // Degree elevation AND knot refinement into the fit's own
            // spline space: on a curved wall the fit refines past the
            // seed grid, so elevation alone no longer lands the seam
            // carrier in the chart's boundary-row space — the OFF-D
            // face-replacement primitive owes its edges BOTH.
            Curve3::Nurbs(c) if c.knots().degree() < FIT_DEGREE => {
                let mut e = c
                    .elevate_degree(FIT_DEGREE - c.knots().degree())
                    .expect("degree elevation of a clamped carrier");
                if !fit_interior_v.is_empty() {
                    e = e
                        .refine_knots(&fit_interior_v)
                        .expect("knot refinement into the fit's space");
                }
                Curve3::Nurbs(Arc::new(e))
            }
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

    topo::mint_pcurves(body, Tol::witness()).expect("the Approx charts mint their iso images");
    faces
}

/// A genuinely doubly-curved SKINNED base: bicubic interpolation-shaped
/// net over `[0,1]²` with a bump — nothing about it is planar or
/// ruled.
fn skinned_base() -> NurbsSurface<f64> {
    let kv =
        geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3)
            .unwrap();
    let mut control = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            let (u, v) = (f64::from(i) / 3.0, f64::from(j) / 3.0);
            control.push(Point3::new(
                2.0 * u,
                2.0 * v,
                0.95 + 0.4 * (u * (1.0 - u)) * (v * (1.0 - v)) * 4.0 + 0.02 * u,
            ));
        }
    }
    NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 16]).unwrap()
}

fn unit_box() -> Body<f64> {
    let v = |x: f64, y: f64| ProfileVertex::new(Point2::new(x, y), 0.0);
    let lp = ProfileLoop::new(vec![v(0.0, 0.0), v(2.0, 0.0), v(2.0, 2.0), v(0.0, 2.0)]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a square is a valid profile");
    sweep::extrude(&profile, sweep::Extrusion::Distance(1.0), Tol::witness())
        .expect("a square prism extrudes")
        .body
}

fn moved_box() -> Body<f64> {
    topo::transform_rigid(
        &unit_box(),
        &Affine3::translation(Vec3::new(0.7, 0.3, 0.4)),
        Tol::witness(),
    )
    .expect("a planar box transforms")
}

fn top_face(body: &Body<f64>) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.z.abs() > 0.5 && origin.z > 0.5
            )
        })
        .map(|(k, _)| k)
        .expect("the extruded box has a top cap")
}

// ---------------------------------------------------------------------

/// The twisted loft's walls really are curved — the premise the whole
/// file rests on, asserted rather than assumed.
#[test]
fn the_twisted_walls_are_not_planar() {
    let body = twisted_loft();
    let mut spline_walls = 0;
    for (_, face) in body.faces() {
        if let Some(Surface::Nurbs(p)) = body.get_surface(face.surface)
            && !p.is_placeholder()
        {
            spline_walls += 1;
            // A bilinear saddle: the mixed partial is nonzero, so the
            // normal varies. Compare corner normals.
            let n00 = {
                let j = p.ders(0.0, 0.0);
                j.du.cross(j.dv).normalize()
            };
            let n11 = {
                let j = p.ders(1.0, 1.0);
                j.du.cross(j.dv).normalize()
            };
            assert!(
                (n00 - n11).norm() > 1e-3,
                "a twisted wall's normal must vary; this wall is planar"
            );
        }
    }
    assert_eq!(spline_walls, 4);
}

/// **Tier 3, end to end, both signs**, on curved `Approx` walls: the
/// re-derivation runs against a genuinely curved chart and agrees.
#[test]
fn a_curved_approx_walled_body_validates_at_tier_three() {
    for d in [D, -D] {
        let mut body = twisted_loft();
        let faces = approx_walls(&mut body, d, FIT_TOL);
        assert!(
            matches!(
                body.get_surface(body.get_face(faces[0]).unwrap().surface),
                Some(Surface::Approx(_))
            ),
            "d = {d}: the wall carries the approximating surface"
        );
        assert_eq!(
            topo::validate_geometric(&body, Tol::witness()),
            Ok(()),
            "d = {d}: tier 3 on the curved Approx-walled body"
        );
    }
}

/// The never-trust red row on the CURVED chart: coarsen one wall's fit
/// behind an honest stored certificate; tier 3 names the face.
#[test]
fn a_degraded_curved_fit_goes_red_at_tier_three() {
    let mut body = twisted_loft();
    let faces = approx_walls(&mut body, D, FIT_TOL);
    let face = faces[0];
    let Some(Surface::Approx(live)) = body.get_surface(body.get_face(face).unwrap().surface) else {
        panic!("the wall carries an approximating surface")
    };
    let geom::SurfaceDescription::Offset { base, .. } = live.description();
    let base = Arc::clone(base);
    let honest = geom_brep::approx_offset_surface(Arc::clone(&base), D, FIT_TOL, band()).unwrap();
    let Surface::Approx(good) = &honest else {
        panic!("the door mints the variant")
    };
    let fit = good.fit();
    let mut control = fit.control().to_vec();
    let mid = control.len() / 2;
    control[mid] = control[mid] + Vec3::new(0.0, 0.0, 1e-4);
    let coarsened = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        control,
        fit.weights().to_vec(),
    )
    .unwrap();
    let planted = geom::ApproxSurface::certify(
        geom::SurfaceSpec {
            description: geom::SurfaceDescription::Offset { base, d: D },
            fit: coarsened,
            window: good.window(),
            tolerance: good.tolerance(),
        },
        |_, _, _| Ok::<_, geom_brep::OffsetFitError>(*good.certificate()),
    )
    .unwrap();
    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(planted))))
        .unwrap();
    let _ = topo::mint_pcurves(&mut body, Tol::witness());
    let errors = topo::validate_geometric(&body, Tol::witness())
        .expect_err("a degraded curved fit must not validate");
    assert!(
        errors.iter().any(|e| matches!(
            e,
            topo::ValidationError::ApproxCertification { face: f, .. } if *f == face
        )),
        "tier 3 must report the re-derivation failure on face {face:?}, got {errors:?}"
    );
}

/// **Tessellation** of the curved `Approx` walls through the delegate
/// path: every wall gets triangles.
#[test]
fn the_curved_approx_walls_tessellate() {
    let mut body = twisted_loft();
    let faces = approx_walls(&mut body, D, FIT_TOL);
    let mesh = mesh::tessellate(&body, 0.05, Tol::witness()).expect("the twisted body meshes");
    for face in faces {
        let patch = mesh
            .patches
            .iter()
            .find(|p| p.face == face)
            .expect("every Approx wall has a patch");
        assert!(!patch.triangles.is_empty(), "no triangles for {face:?}");
    }
}

/// **The boolean against the twisted body refuses TYPED at the edge
/// rule** — a lofted operand's wall carriers are rung-3 splines, so the
/// gate's body-scoped edge rule fires before the face rule ever sees
/// the `Approx` kind. Recorded as the refusal's true shape for THIS
/// operand (the germ-pair shape needs `Line` carriers; next row).
#[test]
fn a_boolean_against_the_twisted_approx_body_refuses_typed() {
    let mut a = twisted_loft();
    let _ = approx_walls(&mut a, D, FIT_TOL);
    let e = topo::union(&a, &moved_box(), Tol::witness())
        .expect_err("a lofted Approx operand is outside the boolean envelope");
    assert!(
        matches!(e, topo::BooleanError::CurvedEdgeUnsupported { .. }),
        "expected the edge rule's typed refusal on a spline carrier, got {e}"
    );
}

/// **The germ-pair refusal from a genuinely skinned base**: a
/// doubly-curved bicubic base's certified offset surface sits on a box
/// cap (`Line` carriers, so the FACE rule decides), and the pair-scoped
/// gate refuses naming `SurfaceKind::Approx` against `Plane`.
///
/// The base here is nothing like the PR's planar pull-backs: the fit
/// has real curvature in both directions and a `d` five orders above ε.
#[test]
fn a_skinned_base_approx_face_earns_the_germ_pair_refusal() {
    let base = Arc::new(skinned_base());
    let approx = geom_brep::approx_offset_surface(Arc::clone(&base), 0.05, 1e-5, band())
        .expect("the skinned base's offset fits to 1e-5");
    let Surface::Approx(a_surf) = &approx else {
        panic!("the door mints the variant")
    };
    // The certificate is real: re-derivation agrees at rest.
    let re = geom_brep::offset_fit::recertify_approx(a_surf, band())
        .expect("the skinned offset re-certifies");
    assert!(re.hull_sup <= 1e-5);

    let mut a = unit_box();
    let face = top_face(&a);
    a.set_face_surface(face, FaceSurface::New(approx))
        .expect("the attach-layer door accepts a live face");
    let e = topo::union(&a, &moved_box(), Tol::witness())
        .expect_err("an Approx operand is unsupported-kind for the boolean gate");
    assert!(
        matches!(
            e,
            topo::BooleanError::CurvedPairUnsupported {
                kind: geom_brep::SurfaceKind::Approx,
                other_kind: geom_brep::SurfaceKind::Plane,
                face: f,
                ..
            } if f == face
        ),
        "expected the germ-pair refusal naming SurfaceKind::Approx on {face:?}, got {e}"
    );
}

/// TEMP diagnostic: what spline spaces are in play on the twisted wall?
#[test]
fn diag_spline_spaces() {
    let body = twisted_loft();
    for (fk, face) in body.faces() {
        if let Some(Surface::Nurbs(p)) = body.get_surface(face.surface)
            && !p.is_placeholder()
        {
            let base = Arc::new(pulled_back(p, D));
            let approx = geom_brep::approx_offset_surface(Arc::clone(&base), D, FIT_TOL, band())
                .expect("fit");
            let Surface::Approx(a) = &approx else {
                panic!()
            };
            println!(
                "face {fk:?}: wall ku={:?} kv={:?} | fit ku={:?} kv={:?}",
                p.knots_u().knots(),
                p.knots_v().knots(),
                a.fit().knots_u().knots(),
                a.fit().knots_v().knots()
            );
            break;
        }
    }
    for (ek, edge) in body.edges() {
        if let Some(c) = body
            .get_curve_geom(edge.curve)
            .and_then(CurveGeom::certified)
            && let Curve3::Nurbs(n) = c.carrier()
        {
            println!(
                "edge {ek:?}: carrier degree {} knots {:?}",
                n.knots().degree(),
                n.knots().knots()
            );
        }
    }
}
