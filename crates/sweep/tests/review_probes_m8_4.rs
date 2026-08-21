//! **M8-4 blinded-review probes — the mint's direction pick and the
//! wrong-pair attachment gate, at the body.**
//!
//! The shipped rows all mint seams whose carrier runs WITH the chart's
//! `v` (positive slope), so the 4-candidate schedule's two BACKWARD
//! candidates never win in the unit's own suite. P-E flips the chart's
//! `v` orientation and demands the backward candidate — red if the
//! direction half of the schedule is decorative. P-F checks that a
//! description whose operand pair cannot certify never reaches the arm
//! at all.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeGeometry};
use geom_core::{Affine3, Band, Point2, Point3, Tolerance, Vec3};
use profile::RawLoop;
use std::sync::Arc;
use topo::{Body, FaceSurface, Pcurve};
use geom_core::Tol;

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
    sweep::loft_body::<f64>(&sections, &places, 2)
        .expect("the offset square prism builds")
        .body
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn is_flat_wall(body: &Body<f64>, key: topo::SurfaceKey) -> bool {
    matches!(body.get_surface(key), Some(Surface::Nurbs(n))
        if !n.is_placeholder() && n.control().iter().all(|p| p.y == -1.0))
}

fn is_bowed_wall(body: &Body<f64>, key: topo::SurfaceKey) -> bool {
    matches!(body.get_surface(key), Some(Surface::Nurbs(n))
        if !n.is_placeholder() && n.control().iter().any(|p| p.y != -1.0)
            && n.control().iter().any(|p| p.x.abs() == 1.0))
}

fn he_surface(body: &Body<f64>, he: topo::HalfEdgeKey) -> topo::SurfaceKey {
    let hed = body.get_half_edge(he).unwrap();
    let lp = body.get_loop(hed.parent_loop).unwrap();
    body.get_face(lp.face).unwrap().surface
}

#[allow(clippy::type_complexity)]
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

/// The intrinsic-seam surgery (the unit's own fixture), with the bowed
/// wall's chart optionally REVERSED in `v` before the description is
/// attached — same surface point set, opposite `v` orientation.
/// `None` is the ε-FINE cell, asserted rather than skipped: this seam's
/// certified between-samples sup is ~6.22e-12 m, so at ε_in = 1e-12 the
/// declare-and-check rung refuses TYPED carrying that number and there
/// is no chart image to probe at all.
fn seam_on_chart(reverse_v: bool) -> Option<(Body<f64>, topo::HalfEdgeKey, topo::SurfaceKey)> {
    let mut body = offset_square_prism();
    let (edge, flat, bowed, he_bowed) = flat_bowed_seam(&body);
    let bowed = if reverse_v {
        let n = match body.get_surface(bowed) {
            Some(Surface::Nurbs(n)) => (**n).clone(),
            other => panic!("the bowed wall is a described chart: {other:?}"),
        };
        let (nu, nv) = n.control_counts();
        let mut control = Vec::with_capacity(nu * nv);
        let mut weights = Vec::with_capacity(nu * nv);
        for i in 0..nu {
            for j in (0..nv).rev() {
                control.push(n.control()[i * nv + j]);
                weights.push(n.weights()[i * nv + j]);
            }
        }
        let flipped = Surface::Nurbs(Arc::new(
            NurbsSurface::new(n.knots_u().clone(), n.knots_v().clone(), control, weights).unwrap(),
        ));
        let (fk, _) = body
            .faces()
            .find(|(_, f)| f.surface == bowed)
            .expect("the bowed wall has a face");
        body.set_face_surface(fk, FaceSurface::New(flipped))
            .expect("the v-reversed chart is the same point set")
    } else {
        bowed
    };
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
    let eps = Tol::witness().get().eps;
    match body.set_edge_curve_nurbs_lane(
        edge,
        EdgeCurveSpec {
            description: EdgeGeometry::Intersection {
                s1: plane,
                s2: bowed,
                witness: carrier.eval((t0 + t1) * 0.5),
            },
            carrier,
            param_start: t0,
            param_end: t1,
        },
    ) {
        Ok(_) => {
            assert!(
                eps >= 1e-9,
                "the seam's certified sup does not fit a finer ε_in"
            );
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
                "the refusal's own number explains it: certified sup {sup:e} m does not fit \
                 inside ε_in {eps:e}"
            );
            println!("P-E attachment @ eps={eps:e}: refused, certified sup {sup:e} m");
            return None;
        }
        Err(other) => panic!("no other posture is pinned for this attachment: {other:?}"),
    }
    body.detach_pcurve(he_bowed);
    Some((body, he_bowed, bowed))
}

/// **P-E: the direction half of the 4-candidate schedule selects but
/// can never certify — MEASURED.** On the `v`-reversed chart the same
/// carrier traverses the chart's `v` BACKWARD; the pick returns the
/// geometrically correct negative-slope image, and the seam class then
/// refuses it (`ResidualExceeded { Envelope }`): its parameter-map
/// slack is metered against the IDENTITY map `v(t) = t`, and its
/// control hull against the boundary row's own ordering, so no
/// backward image fits the certified inventory. The posture for a
/// v-opposed chart is therefore a typed refusal, never a silent mint —
/// pinned here as measured; red if either half of that ever changes
/// silently.
#[test]
fn probe_e_reversed_chart_takes_the_backward_candidate() {
    let Some((body, he, bowed)) = seam_on_chart(true) else {
        return;
    };
    let chart = match body.get_surface(bowed) {
        Some(Surface::Nurbs(n)) => (**n).clone(),
        other => panic!("the bowed wall is a described chart: {other:?}"),
    };
    let out = topo::pcurve_of(&body, he, band());
    let Ok(Pcurve::IsoLine { p0, pl }) = out else {
        panic!("the boundary seam on the reversed chart mints: {out:?}");
    };
    assert_eq!(pl.x, 0.0, "u stays banded-constant: {pl:?}");
    let (du0, du1) = chart.knots_u().domain();
    assert!(
        p0.x == du0 || p0.x == du1,
        "still a boundary column: u = {} of [{du0}, {du1}]",
        p0.x
    );
    let (dv0, dv1) = chart.knots_v().domain();
    assert!(
        (pl.y - (dv0 - dv1)).abs() < 1e-12,
        "the BACKWARD candidate wins on the reversed chart: slope {} on [{dv0}, {dv1}]",
        pl.y
    );
    // Certify the backward image directly through the seam class (the
    // whole-body mint is off the table here: reversing the chart
    // orphans the SIBLING edges' descriptions — probe-fixture damage,
    // not the arm's).
    let (carrier, t0, t1) = {
        let ek = body
            .edges()
            .find(|(_, e)| e.he_plus == he || e.he_minus == he)
            .map(|(k, _)| k)
            .expect("the seam edge resolves");
        let Some(topo::CurveGeom::Certified(c)) =
            body.get_curve_geom(body.get_edge(ek).unwrap().curve)
        else {
            panic!("the seam's carrier is certified");
        };
        let (a, b) = c.params();
        (c.carrier().clone(), a, b)
    };
    let verdict = geom_brep::PcurveCache::certify(
        Pcurve::IsoLine { p0, pl },
        t0,
        t1,
        &carrier,
        &Surface::Nurbs(Arc::new(chart.clone())),
        geom_brep::ChartWindow {
            u_min: chart.knots_u().domain().0,
            u_max: chart.knots_u().domain().1,
            v_min: chart.knots_v().domain().0,
            v_max: chart.knots_v().domain().1,
        },
        band(),
    );
    let refusal = verdict
        .map(|c| format!("{:?}", c.certificate()))
        .expect_err("MEASURED: no backward image fits the seam class's certified inventory");
    assert!(
        format!("{refusal:?}").contains("Envelope"),
        "the refusal is the envelope's own: {refusal:?}"
    );
    println!(
        "P-E @ eps={:e}: u = {}, v slope {}",
        Tol::witness().get().eps,
        p0.x,
        pl.y
    );
}

/// **P-F: a description whose operand pair cannot certify never
/// reaches the arm.** The same locus described as bowed × bowed (no
/// plane in the pair) must refuse at ATTACHMENT — the declare-and-check
/// gate — so the mint's indifference to `s1`/`s2` is safe: by the time
/// the arm runs, some certificate vouched for the pair.
#[test]
fn probe_f_uncertifiable_pair_refuses_at_attachment() {
    // ε-independent: a NURBS × NURBS pair has no certificate at any
    // tolerance, so this row runs at every matrix cell.
    let mut body = offset_square_prism();
    let (edge, _flat, bowed, _he) = flat_bowed_seam(&body);
    let (carrier, t0, t1) = {
        let Some(topo::CurveGeom::Certified(c)) =
            body.get_curve_geom(body.get_edge(edge).expect("the seam resolves").curve)
        else {
            panic!("the seam's carrier is certified");
        };
        let (a, b) = c.params();
        (c.carrier().clone(), a, b)
    };
    let err = body
        .set_edge_curve_nurbs_lane(
            edge,
            EdgeCurveSpec {
                description: EdgeGeometry::Intersection {
                    s1: bowed,
                    s2: bowed,
                    witness: carrier.eval((t0 + t1) * 0.5),
                },
                carrier,
                param_start: t0,
                param_end: t1,
            },
        )
        .expect_err("a NURBS × NURBS pair has no certificate and must refuse at attachment");
    println!("P-F refusal: {err:?}");
}
