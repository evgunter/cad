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
/// walls, every weight 1 — scaled by `scale`, which is `1.0` for every
/// row but the interior-column one.
/// The scale is a lever on the ONE ε-conditional thing about this
/// fixture: the seam's certified between-samples sup is a LENGTH, so it
/// shrinks with the model while ε does not
/// (`INTERIOR_COLUMN_SCALE` states the measurement). Every coordinate
/// is a product with an exact power of two at the scales used, so
/// `scale = 1.0` reproduces the original literals bit for bit.
fn prism(scale: f64) -> Body<f64> {
    let square = move || -> sweep::Section {
        let v = |x: f64, y: f64| profile::ProfileVertex::new(Point2::new(x, y), 0.0);
        vec![profile::ProfileLoop::new(vec![
            v(-scale, -scale),
            v(scale, -scale),
            v(scale, scale),
            v(-scale, scale),
        ])]
    };
    let sections = vec![square(), square(), square()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.5 * scale, 0.0, 1.0 * scale)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0 * scale)),
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
fn is_flat_wall(body: &Body<f64>, key: topo::SurfaceKey, scale: f64) -> bool {
    matches!(body.get_surface(key), Some(Surface::Nurbs(n))
        if !n.is_placeholder() && n.control().iter().all(|p| p.y == -scale))
}

fn is_bowed_wall(body: &Body<f64>, key: topo::SurfaceKey, scale: f64) -> bool {
    matches!(body.get_surface(key), Some(Surface::Nurbs(n))
        if !n.is_placeholder() && n.control().iter().any(|p| p.y != -scale)
            && n.control().iter().any(|p| p.x.abs() == scale))
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
    scale: f64,
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
        if is_flat_wall(body, sp, scale) && is_bowed_wall(body, sm, scale) {
            return (ek, sp, sm, edge.he_minus);
        }
        if is_flat_wall(body, sm, scale) && is_bowed_wall(body, sp, scale) {
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
    intrinsic_seam_at(swap, 1.0)
}

/// [`intrinsic_seam`] on a prism scaled by `scale`.
#[allow(clippy::type_complexity)] // one tuple per named handle, like its callers
fn intrinsic_seam_at(
    swap: bool,
    scale: f64,
) -> Result<(Body<f64>, topo::HalfEdgeKey, topo::SurfaceKey), topo::EulerOpError> {
    let mut body = prism(scale);
    let (edge, flat, bowed, he_bowed) = flat_bowed_seam(&body, scale);
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
                origin: Point3::new(0.0, -scale, 0.0),
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

/// The scale this row's prism is built at, and why it is not 1.
///
/// The fixture's ATTACHMENT is ε-conditional at scale 1: the seam's
/// certified between-samples sup is a LENGTH — 6.217e-12 m — so at
/// ε_in = 1e-12 the declare-and-check rung refuses and no body is built
/// at all, which is how the row this replaces came to assert nothing at
/// one of the three ε the matrix draws (#1167). The sup scales with the
/// model and ε does not, so the fixture is built small enough that
/// every cell of the ε table exercises the same thing.
///
/// A power of two, so every coordinate stays exact and the construction
/// is `offset_square_prism`'s, only smaller.
const INTERIOR_COLUMN_SCALE: f64 = 1.0 / 1024.0;

/// **#498's home for the interior column** — the re-expression of the
/// row #1167 filed as vacuous.
///
/// The same wall geometry on the widened chart, where both seams are
/// INTERIOR columns. An interior column has no boundary-row closed form
/// and never will, so U2's answer is the `General` curve in UV at the
/// honest Fitted grade: the image the wall's own foot schedule
/// measures, certified against the operand PAIR.
///
/// **What the row it replaces did wrong.** `posture()` accepted
/// `Refused` OR `Escalated` — teeth of "does not mint", which a fixture
/// that fails to build for an unrelated reason satisfies — and at
/// ε = 1e-12 the row returned before asserting anything at all, because
/// the seam does not ATTACH there at scale 1. Both are fixed: the
/// outcome asserted here is DEFINITE (a `General` image ON the interior
/// column, with a C2 certificate whose envelope is inside ε), and the
/// fixture is built at a scale where the attachment is not
/// ε-conditional (`prism`'s docs: the attachment's certified sup is a
/// LENGTH and shrinks with the model while ε does not).
///
/// **What this row does NOT claim, and why.** The whole-body mint is
/// blocked on this chart by a DIFFERENT arm: a chart wide enough for a
/// seam to be an interior column is by construction wider than the face
/// it trims, so the face's cap rims are not on a chart boundary either,
/// and the rim arms — which map `u` affinely onto the chart's whole
/// knot domain and pick `v` from the carrier's start point — refuse
/// with `"the carrier's start point lies on neither chart boundary"`.
/// That is asserted below, at a half-edge that is NOT this one, so the
/// row states exactly whose blocker it is. The certificate is therefore
/// taken at `PcurveCache::certify_general` directly — the same door
/// `mint_pcurves` calls, with the same operands — rather than through
/// the pass.
#[test]
fn an_interior_column_intersection_mints_a_general_image() {
    let eps = Tol::witness().get().eps;
    let (mut body, he, bowed) = intrinsic_seam_at(false, INTERIOR_COLUMN_SCALE)
        .expect("the seam attaches at every ε this matrix draws — that is what the scale buys");
    let widened = widened_u_chart(&chart_of(&body, bowed));
    let key = rechart(&mut body, bowed, widened);
    let chart = chart_of(&body, key);
    assert_eq!(
        chart.knots_u().domain(),
        (0.0, 3.0),
        "the widened chart's own domain"
    );
    // ---- The derivation: U2's General arm, on the interior column. ----
    let out = topo::pcurve_of(&body, he, band());
    let Ok(Pcurve::General(ref image)) = out else {
        panic!("an interior column's home is U2's General arm: {out:?}")
    };
    // The image IS the column `u = 2`: every control point holds `u`,
    // and STRICTLY interior — which is the whole difference from the
    // exact `IsoLine` class, whose certification requires a knot-domain
    // end (`geom-brep`'s `an_interior_column_still_refuses`, untouched
    // by this unit). A statement about the image's SHAPE in the chart's
    // own units; the statement in METRES is the envelope below.
    for p in image.control() {
        assert!(
            (p.x - 2.0).abs() < 1e-9,
            "a column holds u constant at the chart's own interior knot: {p:?}"
        );
    }
    let (dv0, dv1) = chart.knots_v().domain();
    let (a, b) = image.domain();
    for (t, want) in [(a, dv0), (b, dv1)] {
        assert!(
            (image.eval(t).y - want).abs() < 1e-9,
            "and traverses the chart's whole v domain: {:?} vs {want}",
            image.eval(t)
        );
    }
    // ---- The certificate, at the door the mint pass uses. ----
    let (carrier, t0, t1, mate) = seam_operands(&body, he);
    let window = out.as_ref().unwrap().chart_box(t0, t1);
    let cache = geom_brep::PcurveCache::certify_general(
        std::sync::Arc::clone(image),
        t0,
        t1,
        &carrier,
        &Surface::Nurbs(Arc::new(chart.clone())),
        Some(&mate),
        window,
        band(),
    )
    .expect("the interior column's image certifies against its operand pair");
    let cert = cache.certificate();
    assert!(
        matches!(cache.pcurve(), Pcurve::General(_)),
        "the certified cache is the General image: {:?}",
        cache.pcurve()
    );
    assert!(
        cert.envelope <= eps,
        "its between-samples bound is inside ε: {:e} vs {eps:e}",
        cert.envelope
    );
    assert!(
        cert.ssi.is_some(),
        "and it is the FULL C2 certificate, not the closed-form lane's: {cert:?}"
    );
    // ---- Whose blocker the whole-body mint is. ----
    let mint = topo::mint_pcurves(&mut body, Tol::witness());
    match mint {
        Ok(()) => panic!(
            "the rim arms learned to read a trimmed chart — good news, and this row's \
             blocker clause is now stale: fold the mint back into the assertions above"
        ),
        Err(PcurveMintError::Certify { half_edge, error }) => {
            assert_ne!(
                half_edge, he,
                "the interior column is not what blocks the body: {error:?}"
            );
            let geom_brep::PcurveCertifyError::IsoUnsupported { what } = error else {
                panic!("the rim arms refuse TYPED, naming the class: {error:?}")
            };
            assert!(
                what.contains("neither chart boundary"),
                "and the blocker is the rim arms' boundary assumption: {what}"
            );
        }
        other => panic!("no other posture is honest for this body: {other:?}"),
    }
    println!(
        "M8-4 interior column @ eps={eps:e}: General on u = 2 of [0, 3], envelope {:e} m, {:?}",
        cert.envelope, cert.statement
    );
}

/// The `(carrier, t0, t1, mate)` the fitted-grade certificate needs, read
/// from the seam's own certified edge — the mate being the operand of
/// the description's pair that is the PLANE (the face's chart was
/// restated after the description was written, so `topo`'s own
/// `mate_surface`, which matches on the face's CURRENT surface key,
/// cannot find it; that is a fixture fact about `rechart`, not a
/// kernel one).
fn seam_operands(
    body: &Body<f64>,
    he: topo::HalfEdgeKey,
) -> (Curve3<f64>, f64, f64, Surface<f64>) {
    let edge = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
    let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(edge.curve) else {
        panic!("the seam's carrier is certified")
    };
    let geom_brep::EdgeDescription::Intersection { s1, s2, .. } = *c.description() else {
        panic!("the seam is described as an intersection")
    };
    let plane = [s1, s2]
        .into_iter()
        .find_map(|k| match body.get_surface(k) {
            Some(p @ Surface::Plane { .. }) => Some(p.clone()),
            _ => None,
        })
        .expect("one operand of the pair is the plane the flat wall was restated as");
    let (t0, t1) = c.params();
    (c.carrier().clone(), t0, t1, plane)
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

/// TEMPORARY rim-arm probe v2 (deleted once its finding is encoded).
///
/// v1 was MIS-DESIGNED and its answer was an artefact: `pcurve_of`
/// short-circuits on a STORED CACHE, and `intrinsic_seam` detaches only
/// the seam's own half-edge, so v1 read three caches the loft minted
/// against the ORIGINAL chart and reported them as derivations. This
/// one detaches every cache on the face first, so what it prints is
/// what the ARM answers, and it runs `mint_pcurves` itself rather than
/// inferring what it would do.
#[test]
fn p2_rim_arm_probe() {
    let Ok((mut body, he, bowed)) = intrinsic_seam_at(false, INTERIOR_COLUMN_SCALE) else {
        panic!("the scaled seam attaches")
    };
    let widened = widened_u_chart(&chart_of(&body, bowed));
    let key = rechart(&mut body, bowed, widened);
    let chart = chart_of(&body, key);
    let (fk, _) = body.faces().find(|(_, f)| f.surface == key).unwrap();
    let face = body.get_face(fk).unwrap();
    let topo::entity::LoopBoundary::Cycle { first } = body.get_loop(face.outer).unwrap().boundary
    else {
        panic!("the bowed wall's outer loop is a cycle")
    };
    let cycle = body.loop_cycle(first).unwrap();
    println!(
        "=== RIM ARM PROBE v2: face {fk:?}, seam {he:?}, widened u domain {:?}",
        chart.knots_u().domain()
    );
    // 1. WHAT WAS CACHED — v1's answer, and why it was not the arm's.
    for h in &cycle {
        let cached = body.pcurve(*h).map(|c| match c.pcurve() {
            Pcurve::IsoLine { p0, pl } => format!("IsoLine p0={p0:?} pl={pl:?}"),
            other => format!("{other:?}").chars().take(40).collect(),
        });
        println!("  he {h:?} STORED CACHE: {cached:?}");
    }
    // 2. THE ARM'S OWN ANSWER, caches gone.
    for h in &cycle {
        body.detach_pcurve(*h);
    }
    for h in &cycle {
        let edge = body.get_edge(body.get_half_edge(*h).unwrap().edge).unwrap();
        let (desc, carrier) = match body.get_curve_geom(edge.curve) {
            Some(topo::CurveGeom::Certified(c)) => {
                let d = match c.description() {
                    geom_brep::EdgeDescription::Chart(cc) => format!(
                        "Chart(pcurve={})",
                        match &cc.pcurve {
                            Pcurve::IsoLine { .. } => "IsoLine",
                            Pcurve::IsoArc { .. } => "IsoArc",
                            Pcurve::Harmonic { .. } => "Harmonic",
                            Pcurve::Fitted(_) => "Fitted",
                            Pcurve::General(_) => "General",
                        }
                    ),
                    geom_brep::EdgeDescription::Intersection { .. } => "Intersection".into(),
                    geom_brep::EdgeDescription::Scaffold(_) => "Scaffold".into(),
                    other => format!("{other:?}"),
                };
                let k = match c.carrier() {
                    Curve3::Line { .. } => "Line",
                    Curve3::Circle { .. } => "Circle",
                    Curve3::Nurbs(_) => "Nurbs",
                    _ => "other",
                };
                (d, k)
            }
            other => (format!("{other:?}"), "?"),
        };
        let verdict = match topo::pcurve_of(&body, *h, band()) {
            Ok(Pcurve::General(_)) => "Ok(General)".to_string(),
            Ok(p) => format!("Ok({p:?})").chars().take(70).collect(),
            Err(e) => format!("{e:?}").chars().take(130).collect(),
        };
        println!("  he {h:?} desc={desc} carrier={carrier}\n     DERIVED -> {verdict}");
    }
    // 3. WHAT THE MINT ITSELF DOES — measured, never inferred.
    match topo::mint_pcurves(&mut body, Tol::witness()) {
        Ok(()) => {
            let findings = topo::pcurves::validate_pcurves(&body, band());
            println!("  mint_pcurves: OK; validate_pcurves: {} findings", findings.len());
            for f in findings.iter().take(3) {
                println!("    {f:?}");
            }
        }
        Err(e) => println!("  mint_pcurves: {e:?}"),
    }
}
