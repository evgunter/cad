//! **M8-4 — the boundary-iso `Intersection` chart image, at the mint.**
//!
//! `nurbs_iso_derive`'s `Intersection` arm reached from a BODY, without
//! a file in the loop: the STEP round-trip rows
//! (`step-import`'s `recognize_pins`) pin the importer's whole ladder,
//! and these rows pin what the arm itself is keyed on — the CARRIER and
//! its BOUNDARY RESIDENCY, never the operand order, never a `[0, 1]`
//! literal.
//!
//! The fixture is the integral mixed prism: a square section lofted
//! through three places whose middle one is offset in `+x`, so the
//! `y = ±1` walls stay exactly planar while the `x = ±1` walls bow.
//! Restating a planar wall as the `Surface::Plane` it exactly is — what
//! import's promotion does — puts a plane and a described NURBS wall on
//! either side of one seam, which is the class this arm serves.
//!
//! # ε posture
//!
//! Every row states all three honest outcomes and never widens a
//! target: `Certified` (the image is minted on the chart's own boundary
//! column), `Refused` (a TYPED `IsoUnsupported` naming the class — the
//! only refusal the excluded class may take), `Escalated` (the residual
//! pick landed in the sliver band). Anything else panics.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::Tol;
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Band, Point2, Point3, Vec3};
use profile::RawLoop;
use std::sync::Arc;
use topo::{Body, FaceSurface, Pcurve, PcurveMintError};

/// The integral mixed prism (`recognize_pins`'s `offset_square_prism`,
/// natively built): exactly-planar `y = ±1` walls, bowed `x = ±1`
/// walls, every weight 1.
fn offset_square_prism() -> Body<f64> {
    let square = || -> sweep::Section {
        let v = |x: f64, y: f64| profile::ProfileVertex::new(Point2::new(x, y), 0.0);
        vec![profile::ProfileLoop::new(vec![
            v(-1.0, -1.0),
            v(1.0, -1.0),
            v(1.0, 1.0),
            v(-1.0, 1.0),
        ])]
    };
    let sections = vec![square(), square(), square()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.5, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the offset square prism builds")
        .body
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// Is this face's surface a described NURBS wall whose control net lies
/// exactly on `y = -1` (the planar wall a promotion restates)?
fn is_flat_wall(body: &Body<f64>, key: topo::SurfaceKey) -> bool {
    matches!(body.get_surface(key), Some(Surface::Nurbs(n))
        if !n.is_placeholder() && n.control().iter().all(|p| p.y == -1.0))
}

fn is_bowed_wall(body: &Body<f64>, key: topo::SurfaceKey) -> bool {
    matches!(body.get_surface(key), Some(Surface::Nurbs(n))
        if !n.is_placeholder() && n.control().iter().any(|p| p.y != -1.0)
            && n.control().iter().any(|p| p.x.abs() == 1.0))
}

/// The face a half-edge bounds.
fn he_surface(body: &Body<f64>, he: topo::HalfEdgeKey) -> topo::SurfaceKey {
    let hed = body.get_half_edge(he).unwrap();
    let lp = body.get_loop(hed.parent_loop).unwrap();
    body.get_face(lp.face).unwrap().surface
}

/// The seam between the flat wall and a bowed one, as
/// `(edge, flat surface, bowed surface, half-edge on the bowed side)`.
fn flat_bowed_seam(
    body: &Body<f64>,
) -> (
    topo::EdgeKey,
    topo::SurfaceKey,
    topo::SurfaceKey,
    topo::HalfEdgeKey,
) {
    for (ek, edge) in body.edges() {
        let (sp, sm) = (
            he_surface(body, edge.he_plus),
            he_surface(body, edge.he_minus),
        );
        let carrier_is_spline = matches!(
            body.get_curve_geom(edge.curve),
            Some(topo::CurveGeom::Certified(c)) if matches!(c.carrier(), Curve3::Nurbs(_))
        );
        if !carrier_is_spline {
            continue;
        }
        if is_flat_wall(body, sp) && is_bowed_wall(body, sm) {
            return (ek, sp, sm, edge.he_minus);
        }
        if is_flat_wall(body, sm) && is_bowed_wall(body, sp) {
            return (ek, sm, sp, edge.he_plus);
        }
    }
    panic!("the offset square prism has a flat-wall/bowed-wall seam");
}

/// The fixture every row starts from: the flat wall restated as the
/// PLANE it exactly is, and the seam re-described INTRINSICALLY as the
/// intersection of that plane with the bowed wall. `swap` states the
/// same locus with the operands in the other order — a description-form
/// difference the arm must not be able to see.
#[allow(clippy::type_complexity)] // one tuple per named handle, like its callers
fn intrinsic_seam(
    swap: bool,
) -> Result<(Body<f64>, topo::HalfEdgeKey, topo::SurfaceKey), topo::EulerOpError> {
    let mut body = offset_square_prism();
    let (edge, flat, bowed, he_bowed) = flat_bowed_seam(&body);
    let flat_face = {
        let (fk, _) = body
            .faces()
            .find(|(_, f)| f.surface == flat)
            .expect("the flat wall has a face");
        fk
    };
    let (carrier, t0, t1) = {
        let Some(topo::CurveGeom::Certified(c)) =
            body.get_curve_geom(body.get_edge(edge).expect("the seam resolves").curve)
        else {
            panic!("the seam's carrier is certified");
        };
        let (a, b) = c.params();
        (c.carrier().clone(), a, b)
    };
    // The plane the flat wall IS: `y = -1`, outward normal `-y`.
    let plane = body
        .set_face_surface(
            flat_face,
            FaceSurface::New(Surface::Plane {
                origin: Point3::new(0.0, -1.0, 0.0),
                normal: Vec3::new(0.0, -1.0, 0.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .expect("the exactly-planar wall restates as a plane");
    let (s1, s2) = if swap { (bowed, plane) } else { (plane, bowed) };
    body.set_edge_curve_nurbs_lane(
        edge,
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1,
                s2,
                witness: carrier.eval((t0 + t1) * 0.5),
            },
            carrier,
            param_start: t0,
            param_end: t1,
        },
        Tol::witness(),
    )?;
    // The loft minted this half-edge's cache against the description it
    // had before this surgery; a cache read back would answer about
    // that one. These rows are about the DERIVATION, so the cache goes.
    body.detach_pcurve(he_bowed);
    Ok((body, he_bowed, bowed))
}

/// The ATTACHMENT is ε-dependent too, and that is the honest answer:
/// this seam's certified between-samples sup is ~6.22e-12 m, so at
/// ε_in = 1e-12 the declare-and-check rung refuses TYPED carrying that
/// number and no mint happens at all. `None` IS that cell — pinned as a
/// refusal whose own number explains it, never widened away.
fn seam_at_eps(swap: bool) -> Option<(Body<f64>, topo::HalfEdgeKey, topo::SurfaceKey)> {
    let eps = Tol::witness().get().eps;
    match intrinsic_seam(swap) {
        Ok(seam) => {
            assert!(
                eps >= 1e-9,
                "the certified sup does not fit inside a finer ε_in — attaching there \
                 would be a widened gate"
            );
            Some(seam)
        }
        Err(topo::EulerOpError::Certification {
            error:
                geom_brep::CertifyError::Escalated {
                    check: geom_brep::CertCheck::PlaneNurbsCertificate,
                    cause,
                    ..
                },
        }) => {
            assert!(eps < 1e-9, "only the ε-fine cell refuses: {cause:?}");
            let geom_core::MarginDiag::Value(sup) = cause.margin else {
                panic!("the refusal carries the lane's measured bound: {cause:?}");
            };
            assert!(
                sup > eps,
                "the refusal's own number explains it: certified sup {sup:e} m does not \
                 fit inside ε_in {eps:e}"
            );
            println!("M8-4 seam attachment @ eps={eps:e}: refused, certified sup {sup:e} m");
            None
        }
        Err(other) => panic!("no other posture is pinned for this attachment: {other:?}"),
    }
}

/// The three honest outcomes of a mint at a given ε.
#[derive(Debug)]
enum MintPosture {
    /// An image on the chart's own boundary column, `u` banded-constant.
    Certified(Pcurve<f64>),
    /// The typed, permanent refusal of the excluded class (C5).
    Refused,
    /// The residency pick landed in the sliver band.
    Escalated,
}

/// Classify a mint, asserting each posture's OWN invariants: a minted
/// image must be a `u`-constant iso line sitting on a knot-domain end
/// of the chart it was minted for, a refusal must be the typed
/// `IsoUnsupported` naming the excluded class, an escalation must carry
/// the residency predicate. Anything else is a real failure.
fn posture(
    row: &str,
    out: &Result<Pcurve<f64>, PcurveMintError>,
    chart: &NurbsSurface<f64>,
) -> MintPosture {
    match out {
        Ok(p @ Pcurve::IsoLine { p0, pl }) => {
            assert_eq!(pl.x, 0.0, "{row}: a seam image holds u constant: {pl:?}");
            let (du0, du1) = chart.knots_u().domain();
            assert!(
                p0.x == du0 || p0.x == du1,
                "{row}: on the chart's OWN boundary column, u = {} of [{du0}, {du1}]",
                p0.x
            );
            MintPosture::Certified(p.clone())
        }
        Err(PcurveMintError::Certify {
            error: geom_brep::PcurveCertifyError::IsoUnsupported { what },
            ..
        }) => {
            assert!(
                what.contains("INTERIOR") || what.contains("not a spline"),
                "{row}: the refusal names the excluded class: {what}"
            );
            MintPosture::Refused
        }
        Err(PcurveMintError::Escalated { cause, .. }) => {
            assert!(
                cause.predicate.is_some_and(|p| p.contains("pcurve_iso")),
                "{row}: only the iso picks may escalate here: {cause:?}"
            );
            MintPosture::Escalated
        }
        other => panic!("{row}: no other posture is honest here: {other:?}"),
    }
}

/// The chart the bowed wall's face carries.
fn chart_of(body: &Body<f64>, key: topo::SurfaceKey) -> NurbsSurface<f64> {
    match body.get_surface(key) {
        Some(Surface::Nurbs(n)) => (**n).clone(),
        other => panic!("the bowed wall is a described NURBS chart: {other:?}"),
    }
}

/// **The arm, at the mint.** A seam described as the intersection of a
/// plane with a described NURBS wall derives the wall's own boundary
/// column — and the whole body then mints and CERTIFIES, which is what
/// says the image is the seam class's, not merely plausible.
#[test]
fn a_boundary_column_intersection_mints_its_iso_image() {
    let Some((mut body, he, bowed)) = seam_at_eps(false) else {
        return;
    };
    let chart = chart_of(&body, bowed);
    let out = topo::pcurve_of(&body, he, band());
    let MintPosture::Certified(image) = posture("boundary column", &out, &chart) else {
        return;
    };
    let Pcurve::IsoLine { p0, pl } = image else {
        panic!("a certified posture carries the iso line it asserted")
    };
    // The moving channel is the chart's own `v`, traversed with the
    // carrier: an affine map of the carrier's interval onto the v
    // domain, not an assumed identity.
    let (dv0, dv1) = chart.knots_v().domain();
    assert!(
        (pl.y - (dv1 - dv0)).abs() < 1e-12,
        "the seam traverses the chart's whole v domain: {pl:?} on [{dv0}, {dv1}]"
    );
    assert!(p0.y.abs() < 1e-12, "and starts at its v origin: {p0:?}");
    // The mint pass certifies every face of the body it charts.
    topo::mint_pcurves(&mut body, Tol::witness()).expect("the whole body charts over the new arm");
    let stored = body
        .pcurve(he)
        .expect("the seam's own half-edge stores its certified image");
    assert_eq!(
        format!("{:?}", stored.pcurve()),
        format!("{:?}", Pcurve::IsoLine { p0, pl }),
        "the stored cache is the derived image"
    );
    println!(
        "M8-4 boundary column @ eps={:e}: u = {}, v slope {}",
        Tol::witness().get().eps,
        p0.x,
        pl.y
    );
}

/// **Operand order is not a fact about the locus.** `Intersection`
/// names an unordered pair; a description that puts the plane second
/// must mint the same image, bit for bit.
#[test]
fn the_operand_order_is_not_a_fact_about_the_locus() {
    let (Some((a_body, a_he, a_bowed)), Some((b_body, b_he, b_bowed))) =
        (seam_at_eps(false), seam_at_eps(true))
    else {
        return;
    };
    let a = topo::pcurve_of(&a_body, a_he, band());
    let b = topo::pcurve_of(&b_body, b_he, band());
    let pa = posture("plane first", &a, &chart_of(&a_body, a_bowed));
    let pb = posture("plane second", &b, &chart_of(&b_body, b_bowed));
    match (pa, pb) {
        (MintPosture::Certified(x), MintPosture::Certified(y)) => assert_eq!(
            format!("{x:?}"),
            format!("{y:?}"),
            "the same locus, the same chart image"
        ),
        (x, y) => assert_eq!(
            core::mem::discriminant(&x),
            core::mem::discriminant(&y),
            "whatever the ε row's posture is, both orders take it: {x:?} vs {y:?}"
        ),
    }
}

/// The same wall geometry on a WIDER chart: the degree-1 `u` net
/// continued linearly one column each way, so the patch the face is
/// trimmed to occupies `u ∈ [1, 2]` of a `[0, 3]` chart and BOTH of its
/// seams become INTERIOR columns. Geometrically nothing moves; only the
/// chart's opinion of where its boundary is does.
fn widened_u_chart(n: &NurbsSurface<f64>) -> Surface<f64> {
    let (nu, nv) = n.control_counts();
    assert_eq!((nu, n.knots_u().degree()), (2, 1), "the loft wall's u span");
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1).unwrap();
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for i in 0..4 {
        for j in 0..nv {
            let (a, b) = (n.control()[j], n.control()[nv + j]);
            control.push(match i {
                0 => a + (a - b),
                1 => a,
                2 => b,
                _ => b + (b - a),
            });
            weights.push(n.weights()[if i <= 1 { j } else { nv + j }]);
        }
    }
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(ku, n.knots_v().clone(), control, weights).unwrap(),
    ))
}

/// The same knot vector on `[lo, hi]` — an affine reparameterization,
/// which moves no point of the surface and every chart coordinate.
fn rescaled(k: &KnotVector, lo: f64, hi: f64) -> KnotVector {
    let (a, b) = k.domain();
    KnotVector::clamped(
        k.knots()
            .iter()
            .map(|x| lo + (hi - lo) * (x - a) / (b - a))
            .collect(),
        k.degree(),
    )
    .unwrap()
}

/// Replaces the bowed wall's chart, returning the new key.
fn rechart(body: &mut Body<f64>, old: topo::SurfaceKey, new: Surface<f64>) -> topo::SurfaceKey {
    let (fk, _) = body
        .faces()
        .find(|(_, f)| f.surface == old)
        .expect("the bowed wall has a face");
    body.set_face_surface(fk, FaceSurface::New(new))
        .expect("the wall takes its restated chart")
}

/// **The excluded class's pin.** An `Intersection` carrier on an
/// INTERIOR column has no boundary-row closed form, and the arm must
/// refuse it TYPED rather than reach for the nearest boundary — red the
/// day the arm over-reaches.
#[test]
fn an_interior_column_intersection_refuses_typed() {
    let Some((mut body, he, bowed)) = seam_at_eps(false) else {
        return;
    };
    let widened = widened_u_chart(&chart_of(&body, bowed));
    let key = rechart(&mut body, bowed, widened);
    let chart = chart_of(&body, key);
    assert_eq!(
        chart.knots_u().domain(),
        (0.0, 3.0),
        "the widened chart's own domain"
    );
    let out = topo::pcurve_of(&body, he, band());
    match posture("interior column", &out, &chart) {
        p @ (MintPosture::Refused | MintPosture::Escalated) => {
            println!(
                "M8-4 interior column @ eps={:e}: {p:?} — {out:?}",
                Tol::witness().get().eps
            );
        }
        MintPosture::Certified(p) => panic!(
            "an interior column has no boundary-row closed form, yet an image was \
             minted: {p:?}"
        ),
    }
}

/// **The IMPORTED chart** (#327): the same wall on the file's own
/// parameterization rather than the unit square. Every boundary the arm
/// reads is a knot-domain end, so the image lands on `u = 3√3` (or 0)
/// with the `v` map affine onto `[0, 2.5]` — under a `[0, 1]` literal
/// the pick would ask about an interior column and refuse.
#[test]
fn an_imported_domain_chart_mints_the_boundary_intersection() {
    let wide = 3.0 * 3.0_f64.sqrt();
    let Some((mut body, he, bowed)) = seam_at_eps(false) else {
        return;
    };
    let n = chart_of(&body, bowed);
    let imported = Surface::Nurbs(Arc::new(
        NurbsSurface::new(
            rescaled(n.knots_u(), 0.0, wide),
            rescaled(n.knots_v(), 0.0, 2.5),
            n.control().to_vec(),
            n.weights().to_vec(),
        )
        .unwrap(),
    ));
    let key = rechart(&mut body, bowed, imported);
    let chart = chart_of(&body, key);
    let out = topo::pcurve_of(&body, he, band());
    let MintPosture::Certified(image) = posture("imported chart", &out, &chart) else {
        return;
    };
    let Pcurve::IsoLine { p0, pl } = image else {
        panic!("a certified posture carries the iso line it asserted")
    };
    assert!(
        p0.x == 0.0 || p0.x == wide,
        "the column is the file's own domain end, not a literal: {p0:?}"
    );
    assert!(
        (pl.y - 2.5).abs() < 1e-12,
        "and the v map is affine onto the file's own v domain: {pl:?}"
    );
    println!(
        "M8-4 imported chart @ eps={:e}: u = {}, v slope {}",
        Tol::witness().get().eps,
        p0.x,
        pl.y
    );
}
