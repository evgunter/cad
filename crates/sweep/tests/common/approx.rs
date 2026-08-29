//! The `Surface::Approx` surgery vocabulary — shared by the OFF-C
//! consumer suite and the r1 probe suite, which built it twice before
//! it landed here (the S52 shape).
//!
//! It sits in `common` rather than in either suite because it is
//! **section-and-body authoring**: what a suite builds a body FROM,
//! which is exactly what this module's routing rule sends here.
//!
//! # The surgery, and why each step is forced
//!
//! [`approx_walls`] runs the M6-1 order — surfaces
//! (`set_face_surface`), then descriptions, then `mint_pcurves` — plus
//! one step the kind forces:
//!
//! - **Re-description.** `FaceSurface::New` mints a fresh arena key, so
//!   every description naming the old wall goes stale and tier 3 says
//!   so (`DescriptionNotAdjacent`).
//! - **The carriers' SPLINE SPACE.** The iso lane's seam class bounds
//!   `|B(v) − C(v)|` by a control-difference hull — a
//!   partition-of-unity argument — so the chart's boundary row and the
//!   carrier must share knots, degree and weights. A loft wall is
//!   bilinear and its offset fit is bicubic, so the carrier needs
//!   **degree elevation AND knot refinement**: on a curved wall the fit
//!   refines past the seed grid, and elevation alone no longer lands
//!   the carrier in the boundary row's space. Both are exact — same
//!   locus, same parameterization, same endpoints — so this is a
//!   representation change, not rim surgery. **The OFF-D
//!   face-replacement primitive owes its edges both.**
//!
//! All walls convert together: a vertical edge is shared by two of
//! them and both sides must agree on the carrier's space.
//!
//! # The pulled-back base
//!
//! A face's edges lie on its surface, so replacing a wall's surface
//! with the offset of that same wall would move the face `d` off its
//! own boundary. [`pulled_back`] instead builds the base whose OFFSET
//! is the wall — the wall's net translated `−d·n` — so
//! `Offset { base, d }` describes the surface the face already had.
//! On a PLANE that is exact; on a saddle it is exact only to `d·Δn`,
//! which is why the curved rows run at a much smaller `d`.

#![allow(dead_code)] // loaded once per consumer; each uses a subset

use std::collections::HashMap;
use std::sync::Arc;

use geom::{Curve3, NurbsSurface, Surface};
use geom_brep::EdgeCurveSpec;
use geom_brep::keys::SurfaceKey;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use topo::{Body, CurveGeom, EdgeKey, FaceKey, FaceSurface};

/// The offset fit door's degree — the spline space every wall carrier
/// is elevated into (`geom_brep::offset_fit::OFFSET_FIT_DEGREE`).
pub const FIT_DEGREE: usize = geom_brep::offset_fit::OFFSET_FIT_DEGREE;

/// The run's linear band.
pub fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A straight-walled square prism lofted between two identical
/// sections — four PLANAR described-NURBS walls.
pub fn prism() -> Body<f64> {
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

/// A loft between a square and the SAME square rotated `theta` about
/// its own centre — four congruent bilinear SADDLE walls (nonplanar).
pub fn twisted_loft(theta: f64) -> Body<f64> {
    let v = |x: f64, y: f64| ProfileVertex::new(Point2::new(x, y), 0.0);
    let (s, c) = theta.sin_cos();
    let rv = |x: f64, y: f64| {
        let (dx, dy) = (x - 1.0, y - 1.0);
        ProfileVertex::new(
            Point2::new(1.0 + c * dx - s * dy, 1.0 + s * dx + c * dy),
            0.0,
        )
    };
    let square = vec![ProfileLoop::new(vec![
        v(0.0, 0.0),
        v(2.0, 0.0),
        v(2.0, 2.0),
        v(0.0, 2.0),
    ])];
    let rotated = vec![ProfileLoop::new(vec![
        rv(0.0, 0.0),
        rv(2.0, 0.0),
        rv(2.0, 2.0),
        rv(0.0, 2.0),
    ])];
    let places = vec![
        Affine3::translation(Vec3::new(0.0, 0.0, 0.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
    ];
    sweep::loft_body::<f64>(&[square, rotated], &places, 1, Tol::witness())
        .expect("the twisted square lofts")
        .body
}

/// The box `[0,2]² x [0,1]` — planar faces and `Line` carriers
/// throughout, so the boolean gate's FACE rule is what decides on it.
pub fn unit_box() -> Body<f64> {
    let v = |x: f64, y: f64| ProfileVertex::new(Point2::new(x, y), 0.0);
    let lp = ProfileLoop::new(vec![v(0.0, 0.0), v(2.0, 0.0), v(2.0, 2.0), v(0.0, 2.0)]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a square is a valid profile");
    sweep::extrude(&profile, sweep::Extrusion::Distance(1.0), Tol::witness())
        .expect("a square prism extrudes")
        .body
}

/// [`unit_box`] moved to a general position — no face of it is
/// coplanar with the unmoved one's, so the pair reaches the operand
/// gate rather than a coincidence refusal.
pub fn moved_box() -> Body<f64> {
    topo::transform_rigid(
        &unit_box(),
        &Affine3::translation(Vec3::new(0.7, 0.3, 0.4)),
        Tol::witness(),
    )
    .expect("a planar box transforms")
}

/// The box's `z = 1` cap.
pub fn top_face(body: &Body<f64>) -> FaceKey {
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

/// A flat bilinear patch at height `z` over `[0,2]²`.
pub fn planar_patch(z: f64) -> NurbsSurface<f64> {
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, z),
        Point3::new(0.0, 2.0, z),
        Point3::new(2.0, 0.0, z),
        Point3::new(2.0, 2.0, z),
    ];
    NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap()
}

/// The wall's net pulled back `d` along its MID-POINT chart normal —
/// exact on a plane, approximate (to `d·Δn`) on a saddle. See the
/// module docs for why the base is the pull-back and not the wall.
pub fn pulled_back(wall: &NurbsSurface<f64>, d: f64) -> NurbsSurface<f64> {
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

/// Every non-placeholder spline wall of `body`, keyed.
pub fn nurbs_walls(body: &Body<f64>) -> Vec<(FaceKey, Arc<NurbsSurface<f64>>)> {
    body.faces()
        .filter_map(|(key, face)| match body.get_surface(face.surface) {
            Some(Surface::Nurbs(payload)) if !payload.is_placeholder() => {
                Some((key, Arc::clone(payload)))
            }
            _ => None,
        })
        .collect()
}

/// The re-attach gate's honest refusal: an edge whose `IsoCurve`
/// residual does not certify against the run's ε.
///
/// **Why the residual is carried here rather than read off the
/// error.** `CertifyError::ResidualExceeded` reports a VERDICT — which
/// check, which sample — and no number, unlike the escalating refusals
/// of the #921 family whose `Indeterminate` carries an enclosure. So
/// the fixture measures the quantity the check classifies
/// ([`iso_residual`]) alongside the call, and
/// [`reattach_certifies_at`] cross-checks that measurement against the
/// kernel's own classifier so a drifted replica cannot pass silently.
#[derive(Debug)]
pub struct ReattachRefusal {
    /// The first edge that refused.
    pub edge: EdgeKey,
    /// The attach layer's typed error.
    pub error: topo::EulerOpError,
    /// The largest `IsoCurve` residual measured over every edge the
    /// surgery re-attaches, in metres — the threshold this fixture
    /// certifies at or above.
    pub max_iso_residual: f64,
}

/// The quantity `CertCheck::ChartResidual` classifies, for one
/// `IsoCurve`-described spec: `max_i |C(tᵢ) − S(u, v(tᵢ))|` over the
/// certification schedule, with `v` affine in the parameter — the
/// kernel's own formula (`geom_brep::certify`'s iso arm), replicated
/// so the fixture has a NUMBER where the refusal gives a verdict.
fn iso_residual(body: &Body<f64>, spec: &EdgeCurveSpec<f64>) -> Option<f64> {
    let geom_brep::EdgeDescriptionSpec::Chart {
        surface,
        image: Some(geom_brep::Pcurve::IsoLine { p0, pl }),
        ..
    } = spec.description
    else {
        return None;
    };
    let (u, v0, v1) = (
        p0.x,
        p0.y + pl.y * spec.param_start,
        p0.y + pl.y * spec.param_end,
    );
    let s = body.get_surface(surface)?;
    let n = f64::from(geom_brep::CERT_SAMPLES - 1);
    let mut worst = 0.0_f64;
    for i in 0..geom_brep::CERT_SAMPLES {
        // The schedule parameter is the KERNEL's own
        // (`geom_brep::sample_param`), so only the `v` affine map is
        // replicated here — and `reattach_certifies_at` cross-checks
        // even that against the real classifier.
        let t = geom_brep::sample_param(spec.param_start, spec.param_end, i);
        let v = v0 + (v1 - v0) * (f64::from(i) / n);
        worst = worst.max(spec.carrier.eval(t).distance(s.eval(u, v)));
    }
    Some(worst)
}

/// **The surgery** (module docs), fallible: every spline wall's surface
/// becomes the certified `Approx` of its pulled-back base, every
/// carrier is carried into the fit's spline space, and the pcurve map
/// is re-minted.
///
/// The re-attach is the one step that can honestly refuse — on a
/// CURVED fixture the pull-back is exact only to `d·Δn`, so the
/// `IsoCurve` residual is construction arithmetic and a tight enough ε
/// classifies it as a definite excess. That refusal is the kernel
/// being right, so it is returned rather than panicked: the caller
/// takes the two arms.
///
/// # Errors
///
/// [`ReattachRefusal`] naming the first edge that did not certify, and
/// the measured residual that explains it.
pub fn try_approx_walls(
    body: &mut Body<f64>,
    d: f64,
    tolerance: f64,
) -> Result<Vec<FaceKey>, ReattachRefusal> {
    let walls = nurbs_walls(body);
    assert!(!walls.is_empty(), "the fixture has spline walls to convert");

    // ---- 1: surfaces.
    let mut remap: HashMap<SurfaceKey, SurfaceKey> = HashMap::new();
    let mut faces = Vec::new();
    let mut fit_interior_v: Vec<f64> = Vec::new();
    let mut max_iso_residual = 0.0_f64;
    for (face, wall) in walls {
        let old = body.get_face(face).unwrap().surface;
        let base = Arc::new(pulled_back(&wall, d));
        let approx = geom_brep::approx_offset_surface(base, d, tolerance, band())
            .unwrap_or_else(|e| panic!("d = {d}: the wall's offset must fit: {e}"));
        if let Surface::Approx(a) = &approx {
            let kv = a.fit().knots_v().knots();
            fit_interior_v = kv[FIT_DEGREE + 1..kv.len() - (FIT_DEGREE + 1)].to_vec();
        }
        let new = body
            .set_face_surface(face, FaceSurface::New(approx))
            .expect("the attach-layer door accepts a live face");
        remap.insert(old, new);
        faces.push(face);
    }

    // ---- 2: descriptions, and the carriers' spline space.
    //
    // Every spec is built and MEASURED first, then attached. The two
    // passes are not a style choice: attaching is per edge, so a
    // single pass would report a max truncated at whichever edge
    // refused first — a number that depends on the refusal rather than
    // describing the fixture. Building all the specs up front is sound
    // because attaching one edge does not change another's curve.
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let mut specs: Vec<(EdgeKey, EdgeCurveSpec<f64>)> = Vec::new();
    for edge in edges {
        let ck = body.get_edge(edge).unwrap().curve;
        let Some(curve) = body.get_curve_geom(ck).and_then(CurveGeom::certified) else {
            continue;
        };
        let re = curve
            .with_remapped_surfaces(|k| Some(remap.get(&k).copied().unwrap_or(k)))
            .expect("the remap answers for every key");
        let carrier = match re.carrier() {
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
        let spec = EdgeCurveSpec {
            description: re.restated_description(),
            carrier,
            param_start,
            param_end,
        };
        if let Some(r) = iso_residual(body, &spec) {
            max_iso_residual = max_iso_residual.max(r);
        }
        specs.push((edge, spec));
    }
    for (edge, spec) in specs {
        if let Err(error) = body.set_edge_curve(edge, spec, Tol::witness()) {
            return Err(ReattachRefusal {
                edge,
                error,
                max_iso_residual,
            });
        }
    }

    // ---- 3: the pcurve map.
    topo::mint_pcurves(body, Tol::witness()).expect("the Approx charts mint their iso images");
    Ok(faces)
}

/// [`try_approx_walls`] for the fixtures whose re-attach cannot
/// honestly refuse — the PLANAR pull-backs, where `Δn = 0` and the
/// `IsoCurve` residual is f64 dust at every ε the suite runs at.
pub fn approx_walls(body: &mut Body<f64>, d: f64, tolerance: f64) -> Vec<FaceKey> {
    try_approx_walls(body, d, tolerance)
        .unwrap_or_else(|r| panic!("edge {:?} re-attach: {}", r.edge, r.error))
}

/// **The cross-check that keeps [`iso_residual`]'s replica honest**:
/// does the kernel's own certification of `spec` succeed against a
/// band at `eps`?
///
/// A replica that drifted from `geom_brep::certify`'s iso arm would
/// pin a stale constant in silence; running the real classifier either
/// side of the measured threshold makes that loud.
pub fn reattach_certifies_at(body: &Body<f64>, edge: EdgeKey, eps: f64) -> bool {
    let Some(e) = body.get_edge(edge) else {
        return false;
    };
    let Some(curve) = body.get_curve_geom(e.curve).and_then(CurveGeom::certified) else {
        return false;
    };
    let Some(plus) = body.get_half_edge(e.he_plus) else {
        return false;
    };
    let Some(end_v) = body.half_edge_end(e.he_plus) else {
        return false;
    };
    let (Some(start), Some(end)) = (
        body.get_vertex(plus.start)
            .and_then(|v| body.get_point(v.point)),
        body.get_vertex(end_v).and_then(|v| body.get_point(v.point)),
    ) else {
        return false;
    };
    let (start, end) = (*start, *end);
    let (param_start, param_end) = curve.params();
    let spec = EdgeCurveSpec {
        description: curve.restated_description(),
        carrier: curve.carrier().clone(),
        param_start,
        param_end,
    };
    let Ok(band) = geom_core::Band::new(eps, eps * 10.0) else {
        return false;
    };
    geom_brep::EdgeCurve::certify(spec, start, end, |k| body.get_surface(k).cloned(), band).is_ok()
}
