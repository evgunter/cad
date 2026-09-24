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

use crate::common::approx::band;
use geom::Curve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::Tol;
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Point2, Point3, Vec3};
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
/// **The whole body then mints and validates at rest.** A chart wide
/// enough for a seam to be an interior column is wider than the face
/// it trims, so nothing on the face is on a chart boundary: the cap
/// rims measure their `u` map, and the OTHER seam — `Chart`-described
/// on the neighbour's own image, a spline carrier with no operand
/// pair — measures its column and certifies it EXACTLY, by the de Boor
/// collapse (`geom_brep::interior_iso_u`) rather than a boundary row.
/// The `Intersection` seam keeps `General`: nothing is downgraded. The
/// certificate is first taken at `PcurveCache::certify_general`
/// directly — the same door `mint_pcurves` calls, with the same
/// operands — and then the pass itself is run.
///
/// **What this row does NOT claim.** The body validates; the face's
/// TRIM REGION is not an axis-aligned rectangle of its chart, and the
/// quadrature and tessellation lanes refuse it typed. Which of their
/// filed refusals this body actually reaches is asserted at the end —
/// the opening measurement of the unit that lifts them
/// (`work/trim/general-pcurve-face-props-and-tess-refuse.md`).
#[test]
fn an_interior_column_intersection_mints_a_general_image() {
    let eps = Tol::witness().get().eps;
    let (mut body, he, bowed) = intrinsic_seam_at(false, INTERIOR_COLUMN_SCALE)
        .expect("the seam attaches at every ε this matrix draws — that is what the scale buys");
    let widened = widened_u_chart(&chart_of(&body, bowed));
    let (plane_key, _) = seam_plane(&body, he);
    let key = rechart(&mut body, bowed, widened);
    let chart = chart_of(&body, key);
    assert_eq!(
        chart.knots_u().domain(),
        (0.0, 3.0),
        "the widened chart's own domain"
    );
    // The description must name the chart the face NOW carries, or the
    // mint finds no mate and the join below would be untested.
    redescribe_against(&mut body, he, plane_key, key)
        .expect("the seam re-attaches against the widened chart");
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
    // ---- The certificate, at the door the mint pass uses, with the
    // mate the mint itself would find. ----
    let (carrier, t0, t1) = seam_carrier(&body, he);
    let mate = mate_the_mint_would_find(&body, he, key).expect(
        "mate_surface's precondition holds: the face's own surface is one of the \
         described pair, so the mint reaches certify_general with an operand pair \
         rather than FittedMateMissing",
    );
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
        geom_brep::FittedLane::certified(),
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
    // ---- The whole body, through the pass. ----
    topo::mint_pcurves(&mut body, Tol::witness())
        .unwrap_or_else(|e| panic!("every half-edge of the trimmed chart mints at rest: {e:?}"));
    let bowed_hes: Vec<_> = body
        .edges()
        .flat_map(|(_, e)| [e.he_plus, e.he_minus])
        .filter(|h| he_surface(&body, *h) == key)
        .collect();
    for h in &bowed_hes {
        assert!(
            body.pcurve(*h).is_some(),
            "the bowed face's cache set is complete: {h:?} carries none"
        );
    }
    assert!(
        matches!(body.pcurve(he).unwrap().pcurve(), Pcurve::General(_)),
        "the Intersection seam keeps its General image — no downgrade: {:?}",
        body.pcurve(he).unwrap().pcurve()
    );
    // The other seam: Chart-described on the neighbour's image, spline
    // carrier, no operand pair — the EXACT class on the interior
    // column `u = 1`, minted by the wall–seam arm's measured foot.
    let mut chart_seams = 0;
    for h in &bowed_hes {
        if *h == he {
            continue;
        }
        let (carrier, _, _) = seam_carrier(&body, *h);
        if !matches!(carrier, Curve3::Nurbs(_)) {
            continue;
        }
        chart_seams += 1;
        let cache = body.pcurve(*h).unwrap();
        let Pcurve::IsoLine { p0, pl } = cache.pcurve() else {
            panic!(
                "the Chart-described seam takes the exact class: {:?}",
                cache.pcurve()
            )
        };
        assert_eq!(pl.x, 0.0, "a column holds u constant: {pl:?}");
        assert!(
            (p0.x - 1.0).abs() < 1e-9,
            "on the chart's interior knot column u = 1 of [0, 3]: {p0:?}"
        );
        assert!(
            cache.certificate().envelope <= eps,
            "and its collapsed-row hull is inside ε: {:e}",
            cache.certificate().envelope
        );
        println!(
            "M8-4 chart seam {h:?} @ eps={eps:e}: IsoLine on u = {}, envelope {:e} m, {:?}",
            p0.x,
            cache.certificate().envelope,
            cache.certificate().statement
        );
    }
    assert_eq!(
        chart_seams, 1,
        "the wall has exactly one Chart-described seam"
    );
    let findings = topo::pcurves::validate_pcurves(&body, band());
    assert!(
        findings.is_empty(),
        "the body validates at rest: {findings:?}"
    );
    println!(
        "M8-4 interior column @ eps={eps:e}: General on u = 2 of [0, 3], envelope {:e} m, {:?}",
        cert.envelope, cert.statement
    );

    // ---- Which lane this body actually reaches. Quadrature no
    // longer refuses: TRIM-2 PR-1's dispatch sends a loop carrying a
    // `General` image to the trimmed lane, and THIS body's chart is
    // the degree-1 widening, whose `u` direction is only C⁰ at its
    // interior knots — so what the trimmed lane answers here is its
    // own measurement, printed rather than assumed.
    // Tessellation does NOT reach a trimmed-region site at all: the
    // widened chart has interior knots in its degree-1 `u` direction,
    // and `mesh`'s patch-bound gate refuses that C⁰ crease first
    // (`geom_brep::patch_bound::PatchBoundError::Degree1Crease`). ----
    let props = topo::mass_properties(&body, Tol::witness())
        .unwrap_or_else(|e| panic!("the trimmed lane answers the degree-1 body too: {e:?}"));
    let want = topo::mass_properties(&prism(INTERIOR_COLUMN_SCALE), Tol::witness())
        .expect("the oracle prism's own lanes answer it");
    println!(
        "M8-4 DEG1 mass_properties on the trimmed chart: volume {:e} ± {:e}, area {:e} ± {:e}",
        props.volume, props.volume_pad, props.surface_area, props.area_pad
    );
    assert!(
        props.volume - props.volume_pad <= want.volume + want.volume_pad
            && want.volume - want.volume_pad <= props.volume + props.volume_pad,
        "the degree-1 widening is the SAME solid as the oracle prism, so its volume \
         enclosure must overlap: {:e} ± {:e} vs {:e} ± {:e}",
        props.volume,
        props.volume_pad,
        want.volume,
        want.volume_pad
    );
    let tess = mesh::tessellate(&body, 1e-5, Tol::witness());
    let Err(mesh::TessellateError::UnsupportedNurbsFace { note, .. }) = tess else {
        panic!("the trimmed face's tessellation lane moved — re-pin this row")
    };
    assert!(
        note.contains("C⁰ crease"),
        "tessellation refuses at the crease gate before any trimmed-region site: {note}"
    );
    println!("M8-4 tessellate on the trimmed chart: {note}");
}

/// **The mate the MINT would find**, by `topo::pcurves::mate_surface`'s
/// own rule rather than by hand: read the edge's `Intersection`
/// description, and take whichever operand is NOT the face's CURRENT
/// surface key. `None` when that precondition fails — which is exactly
/// what `mint_face` sees, and what it turns into `FittedMateMissing`.
///
/// Hand-picking the plane here would have made the row assert a
/// certificate the mint cannot reproduce: the join between derivation
/// and certification is the thing under test, so it is read the way the
/// mint reads it.
fn mate_the_mint_would_find(
    body: &Body<f64>,
    he: topo::HalfEdgeKey,
    own: topo::SurfaceKey,
) -> Option<Surface<f64>> {
    let edge = body.get_edge(body.get_half_edge(he)?.edge)?;
    let topo::CurveGeom::Certified(c) = body.get_curve_geom(edge.curve)? else {
        return None;
    };
    let geom_brep::EdgeDescription::Intersection { s1, s2, .. } = *c.description() else {
        return None;
    };
    let other = if own == s1 {
        s2
    } else if own == s2 {
        s1
    } else {
        return None;
    };
    body.get_surface(other).cloned()
}

/// The seam's certified carrier and its parameter interval.
fn seam_carrier(body: &Body<f64>, he: topo::HalfEdgeKey) -> (Curve3<f64>, f64, f64) {
    let edge = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
    let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(edge.curve) else {
        panic!("the seam's carrier is certified")
    };
    let (t0, t1) = c.params();
    (c.carrier().clone(), t0, t1)
}

/// The plane operand named by the seam's description.
fn seam_plane(body: &Body<f64>, he: topo::HalfEdgeKey) -> (topo::SurfaceKey, Surface<f64>) {
    let edge = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
    let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(edge.curve) else {
        panic!("certified")
    };
    let geom_brep::EdgeDescription::Intersection { s1, s2, .. } = *c.description() else {
        panic!("intersection")
    };
    [s1, s2]
        .into_iter()
        .find_map(|k| match body.get_surface(k) {
            Some(p @ Surface::Plane { .. }) => Some((k, p.clone())),
            _ => None,
        })
        .expect("one operand is the plane the flat wall was restated as")
}

/// Re-states the seam's `Intersection` description against the chart
/// the face NOW carries.
///
/// `rechart` mints a NEW surface key (there is no in-place variant of
/// `FaceSurface`), and the description still names the OLD one — so
/// `mate_surface`'s precondition, "the face's own surface is one of the
/// pair", fails and the mint hands `certify_general` no mate at all.
/// That is a fixture artefact of restating a chart after describing an
/// edge, not a kernel fact, and this repairs it so the row measures the
/// join instead of asserting it.
fn redescribe_against(
    body: &mut Body<f64>,
    he: topo::HalfEdgeKey,
    plane: topo::SurfaceKey,
    wall: topo::SurfaceKey,
) -> Result<(), topo::EulerOpError> {
    let edge = body.get_half_edge(he).unwrap().edge;
    let (carrier, t0, t1) = seam_carrier(body, he);
    body.set_edge_curve_nurbs_lane(
        edge,
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: plane,
                s2: wall,
                witness: carrier.eval((t0 + t1) * 0.5),
            },
            carrier,
            param_start: t0,
            param_end: t1,
        },
        Tol::witness(),
    )?;
    body.detach_pcurve(he);
    Ok(())
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

/// The same wall geometry on a wider **degree-2** `u` chart: five
/// columns on knots `[0,0,0,1,2,3,3,3]` placed at the Greville abscissae
/// `ξ = (0, ½, 3/2, 5/2, 3)` by linear precision, so
/// `c_i(v) = a(v) + (ξ_i − 1)·(b(v) − a(v))` and `Σ ξ_i N_i(u) = u`
/// gives `S(u, v) = a(v) + (u − 1)·(b(v) − a(v))` exactly — the loft
/// wall's own bilinear-in-`u` surface, restated. The face occupies
/// `u ∈ [1, 2]`, both seams are interior columns, and every interior
/// knot is simple, so the degree-1 crease the `u`-linear widening
/// carries is gone.
fn widened_u_chart_deg2(n: &NurbsSurface<f64>) -> Surface<f64> {
    let (nu, nv) = n.control_counts();
    assert_eq!((nu, n.knots_u().degree()), (2, 1), "the loft wall's u span");
    assert!(
        n.weights().iter().all(|w| *w == 1.0),
        "linear precision places the columns of a POLYNOMIAL net; a rational \
         wall would need the weights carried through the same map"
    );
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2).unwrap();
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for xi in [0.0, 0.5, 1.5, 2.5, 3.0] {
        for j in 0..nv {
            let (a, b) = (n.control()[j], n.control()[nv + j]);
            control.push(a + (b - a) * (xi - 1.0));
            weights.push(1.0);
        }
    }
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(ku, n.knots_v().clone(), control, weights).unwrap(),
    ))
}

/// The degree-2 re-widened P-2 body, plus the half-edge carrying the
/// `Intersection` seam and the bowed wall's new surface key.
fn degree_two_body() -> (Body<f64>, topo::HalfEdgeKey, topo::SurfaceKey) {
    let (mut body, he, bowed) = intrinsic_seam_at(false, INTERIOR_COLUMN_SCALE)
        .expect("the seam attaches at every ε this matrix draws — that is what the scale buys");
    let widened = widened_u_chart_deg2(&chart_of(&body, bowed));
    let (plane_key, _) = seam_plane(&body, he);
    let key = rechart(&mut body, bowed, widened);
    redescribe_against(&mut body, he, plane_key, key)
        .expect("the seam re-attaches against the widened chart");
    (body, he, key)
}

/// **E1 — the degree-2 body MEASURES.**
///
/// The same wall on the degree-2 re-widening (`widened_u_chart_deg2`):
/// both seams are interior columns, every interior knot is simple, and
/// the `Intersection` seam keeps its `General` image — so the face's
/// trim region is what that image bounds and not a rectangle of its
/// chart. `mass_properties` answers it, and the answer must overlap
/// the ORACLE prism's: the same solid on its original charts, with no
/// restatement and no `General` anywhere.
///
/// **E3 rides along, recorded and not flipped**: `replace_face_offset`
/// on the bowed face is not this unit's frontier. It refuses at the
/// FITTED offset's own boundary rule — `FittedBoundaryUnsupported`,
/// which the oracle prism's own bowed wall earns identically with no
/// `General` in the body at all — or, where the fit cannot reach the
/// run's ε, at `Fit { BudgetExhausted }`. Both are SHELL's
/// `no-approx-faced-body-is-both-movable-and-valid` class. The row
/// prints them; asserting a variant here would pin another program's
/// frontier.
#[test]
fn a_degree_two_widening_measures_against_the_oracle() {
    let eps = Tol::witness().get().eps;
    let (mut body, he, key) = degree_two_body();
    topo::mint_pcurves(&mut body, Tol::witness())
        .unwrap_or_else(|e| panic!("the degree-2 chart mints at rest: {e:?}"));
    assert!(
        matches!(body.pcurve(he).unwrap().pcurve(), Pcurve::General(_)),
        "the Intersection seam keeps its General image on the degree-2 chart: {:?}",
        body.pcurve(he).unwrap().pcurve()
    );
    let findings = topo::pcurves::validate_pcurves(&body, band());
    assert!(
        findings.is_empty(),
        "the degree-2 body validates at rest: {findings:?}"
    );
    let props = topo::mass_properties(&body, Tol::witness())
        .unwrap_or_else(|e| panic!("the trimmed lane answers the degree-2 body: {e:?}"));
    let oracle = prism(INTERIOR_COLUMN_SCALE);
    let want = topo::mass_properties(&oracle, Tol::witness())
        .expect("the oracle prism's own lanes answer it");
    let overlaps = |a: (f64, f64), b: (f64, f64)| a.0 <= b.1 && b.0 <= a.1;
    let bracket = |v: f64, pad: f64| (v - pad, v + pad);
    let (got_v, want_v) = (
        bracket(props.volume, props.volume_pad),
        bracket(want.volume, want.volume_pad),
    );
    println!(
        "E1 @ eps={eps:e}: volume {got_v:?} vs oracle {want_v:?}; area {:?} vs {:?}",
        bracket(props.surface_area, props.area_pad),
        bracket(want.surface_area, want.area_pad)
    );
    assert!(
        overlaps(got_v, want_v),
        "E1: the trimmed lane's volume enclosure {got_v:?} and the oracle's {want_v:?} \
         describe the same solid and must overlap"
    );
    assert!(
        overlaps(
            bracket(props.surface_area, props.area_pad),
            bracket(want.surface_area, want.area_pad)
        ),
        "E1: the surface-area enclosures must overlap too"
    );
    // E3, recorded.
    let (fk, _) = body
        .faces()
        .find(|(_, f)| f.surface == key)
        .expect("the bowed wall has a face");
    let mut off = body.clone();
    let got = topo::replace_face_offset(
        &mut off,
        fk,
        INTERIOR_COLUMN_SCALE / 16.0,
        band(),
        Tol::witness(),
    );
    let (_, obowed, _, _) = flat_bowed_seam(&oracle, INTERIOR_COLUMN_SCALE);
    let (ofk, _) = oracle
        .faces()
        .find(|(_, f)| f.surface == obowed)
        .expect("the oracle's bowed wall has a face");
    let mut ooff = oracle.clone();
    let orc = topo::replace_face_offset(
        &mut ooff,
        ofk,
        INTERIOR_COLUMN_SCALE / 16.0,
        band(),
        Tol::witness(),
    );
    println!("E3 @ eps={eps:e}: offset(General-faced) {got:?}");
    println!("E3 @ eps={eps:e}: offset(oracle bowed)  {orc:?}");
    assert!(
        got.is_err() && orc.is_err(),
        "E3 records a refusal on both; a success here would be a different unit's news"
    );
}

/// The distinct mesh-vertex ids and triangle count of the patch on
/// the face carrying `key`'s surface — the per-patch reading E2
/// compares, rather than a whole-mesh position count that sums six
/// unrelated faces.
fn patch_size(body: &Body<f64>, m: &mesh::Mesh, key: topo::SurfaceKey) -> (usize, usize) {
    let (fk, _) = body
        .faces()
        .find(|(_, f)| f.surface == key)
        .expect("the surface has a face");
    let p = m
        .patches
        .iter()
        .find(|p| p.face == fk)
        .expect("the face has a patch");
    let mut ids = std::collections::BTreeSet::new();
    for t in &p.triangles {
        ids.extend(t.iter().copied());
    }
    (p.triangles.len(), ids.len())
}

/// **E2 — the degree-2 body TESSELLATES, and its `General`-faced wall
/// is the oracle's wall exactly** (`docs/TRIM-2-SPEC.md` §2, §3's e2e
/// table).
///
/// The same body E1 measures. Its `Intersection` seam carries a
/// `General` chart image, which until TRIM-2 PR-2 stopped the CHORD
/// pass dead — `TessellateError::UnsupportedCurve` at
/// `mesh::chords::nurbs_tighten`'s `General` arm, before any face lane
/// ran, at all three ε. Two arms flip that: the chord pass now sizes
/// the seam's UV steps from the image's own differenced control net,
/// and the trim walk reads the image at the shared chord parameters.
///
/// # What is compared, and why not the whole mesh
///
/// The comparison is PER PATCH. The widened wall and the oracle's
/// unwidened wall are the same surface over the face's `u ∈ [1, 2]`
/// (linear precision places the extra columns), the seam's chord
/// schedule is the same, and the two patches come out with the same
/// triangle and id counts — an EQUALITY, which is stronger than the
/// spec's "within the chord schedule's own ±" and is what this row
/// asserts.
///
/// The two bodies' whole-mesh position counts are NOT equal, and the
/// difference is not the `General` image's doing at all: the P-2 route
/// restates one flat wall as the `Surface::Plane` it exactly is, so
/// that wall takes the planar CDT lane while the oracle's takes the
/// described-NURBS lane. That one substitution is the entire deficit,
/// and the row asserts the identity rather than banding it.
///
/// # What this row is evidence FOR, and what it is not
///
/// The fixture's `General` image runs `u ∈ [2 − 2.2e-16, 2]` — a chart
/// image degenerate to within an ulp of the iso line beside it
/// (`work/trim/curved-trim-e2e-fixture-waits-for-a-producer.md`: the
/// only at-rest producer mints a 33-foot interpolant of a boundary
/// locus). So this is a SCHEDULE and WATERTIGHTNESS row, not a
/// curvature one. The curvature evidence for the speed bound is the
/// unit row
/// `mesh::chords::tests::general_uv_speeds_dominate_the_sampled_image_speeds`,
/// which carries an interior-maximum leg.
///
/// It is also not a bound on how wrong the sup may be: a sup too small
/// by a factor of ~2 still MESHES here, because grid sizing targets
/// δ/2 and that margin absorbs a boundary UV step of a few `h_v`
/// before any certificate is exceeded. A grosser one meets
/// `CertificateExceeded`. The domination row is where a degraded sup
/// dies; this row sees a schedule that CHANGED, which is what the
/// per-patch equality tests.
#[test]
fn a_degree_two_widening_tessellates_against_the_oracle() {
    let eps = Tol::witness().get().eps;
    // The spec's cell: 1e-5 of the model, which is what E1's scale
    // buys (the fixture is 1/1024 across, so an ABSOLUTE 1e-5 would
    // be a hundredth of the body and size every wall at its floor).
    let delta = 1e-5 * INTERIOR_COLUMN_SCALE;
    let (mut body, he, key) = degree_two_body();
    topo::mint_pcurves(&mut body, Tol::witness())
        .unwrap_or_else(|e| panic!("the degree-2 chart mints at rest: {e:?}"));
    assert!(
        matches!(body.pcurve(he).unwrap().pcurve(), Pcurve::General(_)),
        "E2 is about the General image; the seam carries {:?}",
        body.pcurve(he).unwrap().pcurve()
    );
    let got = mesh::tessellate(&body, delta, Tol::witness())
        .unwrap_or_else(|e| panic!("E2: the General-imaged body tessellates: {e:?}"));
    let oracle = prism(INTERIOR_COLUMN_SCALE);
    let want = mesh::tessellate(&oracle, delta, Tol::witness())
        .expect("the oracle prism tessellates on its own charts");

    // The `General`-faced wall against the oracle's four walls, which
    // are all one another's equals — so "the oracle's wall" is not a
    // pick.
    let (gt, gi) = patch_size(&body, &got, key);
    let oracle_walls: Vec<(usize, usize)> = oracle
        .faces()
        .filter(|(_, f)| matches!(oracle.get_surface(f.surface), Some(geom::Surface::Nurbs(_))))
        .map(|(_, f)| patch_size(&oracle, &want, f.surface))
        .collect();
    println!(
        "E2 @ eps={eps:e} delta={delta:e}: General-faced wall {gt} tris / {gi} ids; \
         oracle walls {oracle_walls:?}; whole mesh {} vs {} positions, {} vs {} patches",
        got.positions.len(),
        want.positions.len(),
        got.patches.len(),
        want.patches.len()
    );
    assert_eq!(
        oracle_walls.len(),
        4,
        "the oracle prism has four described-NURBS walls"
    );
    assert!(
        oracle_walls.iter().all(|w| *w == oracle_walls[0]),
        "the oracle's four walls are one another's equals: {oracle_walls:?}"
    );
    assert_eq!(
        (gt, gi),
        oracle_walls[0],
        "E2: the widened chart's face IS the oracle wall's surface over u ∈ [1, 2], \
         and its seam's chord schedule is the same, so its patch must come out with \
         the same triangle and id counts"
    );
    assert_eq!(
        got.patches.len(),
        want.patches.len(),
        "E2: the same six faces, so the same patch count"
    );

    // The whole-mesh deficit, named and asserted rather than banded:
    // it is exactly the flat wall the P-2 route restated as a plane,
    // meshed by the planar CDT lane instead of the described-NURBS
    // one. Nothing about the `General` image enters it.
    let (plane_key, _) = seam_plane(&body, he);
    let (_, plane_ids) = patch_size(&body, &got, plane_key);
    println!(
        "E2 @ eps={eps:e}: the plane-restated wall carries {plane_ids} ids where the \
         oracle's described wall carries {}; deficit {} = {}",
        oracle_walls[0].1,
        want.positions.len() - got.positions.len(),
        oracle_walls[0].1 - plane_ids
    );
    assert_eq!(
        want.positions.len() - got.positions.len(),
        oracle_walls[0].1 - plane_ids,
        "E2: the WHOLE difference between the two meshes is the restated flat wall's \
         planar lane standing in for a described-NURBS one"
    );

    // Watertightness is the claim the trim walk's arm must not move:
    // the 3-D positions are the carrier's chord points, shared with
    // the neighbour by id, and only this face's UV shape changed.
    mesh::validate::check_mesh(&got)
        .unwrap_or_else(|e| panic!("E2: the General-imaged body's mesh is watertight: {e:?}"));
}
