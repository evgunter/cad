//! CERT-M3 R2 adversarial probes: what the at-rest doors still check on
//! a CORRUPT plane x NURBS (M7-8) edge at `f64`.
//!
//! Fixture helpers copied from `r1_p2_probes.rs` (itself verbatim from
//! `m8_4_intersection_iso.rs`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::Tol;
use geom_core::{Affine3, Point2, Point3, Vec3};
use profile::RawLoop;
use std::sync::Arc;
use topo::{Body, FaceSurface};

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

fn is_flat_wall(body: &Body<f64>, key: topo::SurfaceKey, scale: f64) -> bool {
    matches!(body.get_surface(key), Some(Surface::Nurbs(n))
        if !n.is_placeholder() && n.control().iter().all(|p| p.y == -scale))
}

fn is_bowed_wall(body: &Body<f64>, key: topo::SurfaceKey, scale: f64) -> bool {
    matches!(body.get_surface(key), Some(Surface::Nurbs(n))
        if !n.is_placeholder() && n.control().iter().any(|p| p.y != -scale)
            && n.control().iter().any(|p| p.x.abs() == scale))
}

fn he_surface(body: &Body<f64>, he: topo::HalfEdgeKey) -> topo::SurfaceKey {
    let hed = body.get_half_edge(he).unwrap();
    let lp = body.get_loop(hed.parent_loop).unwrap();
    body.get_face(lp.face).unwrap().surface
}

#[allow(clippy::type_complexity)]
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

const SCALE: f64 = 1.0 / 1024.0;

/// Builds the M7-8 body: a plane x described-NURBS `Intersection` edge
/// attached through `set_edge_curve_nurbs_lane`.
fn m7_8_body() -> (Body<f64>, topo::EdgeKey, topo::SurfaceKey) {
    let scale = SCALE;
    let mut body = prism(scale);
    let (edge, flat, bowed, _he_bowed) = flat_bowed_seam(&body, scale);
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
                origin: Point3::new(0.0, -scale, 0.0),
                normal: Vec3::new(0.0, -1.0, 0.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .expect("the exactly-planar wall restates as a plane");
    body.set_edge_curve_nurbs_lane(
        edge,
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: plane,
                s2: bowed,
                witness: carrier.eval((t0 + t1) * 0.5),
            },
            carrier,
            param_start: t0,
            param_end: t1,
        },
        Tol::witness(),
    )
    .expect("the M7-8 attach door certifies the seam");
    (body, edge, bowed)
}

/// Corrupts the wall AFTER attach: the stored certificate now names a
/// locus the geometry no longer has.
fn corrupt_wall(body: &mut Body<f64>, bowed: topo::SurfaceKey) {
    let chart = match body.get_surface(bowed) {
        Some(Surface::Nurbs(n)) => (**n).clone(),
        other => panic!("the bowed wall is a described NURBS chart: {other:?}"),
    };
    let mut ctrl: Vec<Point3<f64>> = chart.control().to_vec();
    for p in &mut ctrl {
        p.y += 64.0 * SCALE;
    }
    let moved = NurbsSurface::new(
        chart.knots_u().clone(),
        chart.knots_v().clone(),
        ctrl,
        chart.weights().to_vec(),
    )
    .expect("the displaced chart is a valid NURBS surface");
    let (fk, _) = body
        .faces()
        .find(|(_, f)| f.surface == bowed)
        .expect("the bowed wall has a face");
    body.set_face_surface(fk, FaceSurface::New(Surface::Nurbs(Arc::new(moved))))
        .expect("the wall takes its displaced chart");
}

fn report(tag: &str, body: &Body<f64>, edge: topo::EdgeKey) {
    let tol = Tol::witness();
    let records = topo::ContactRecords::default();
    let names = |v: &Vec<topo::ValidationError>| -> String {
        let mut hit = 0usize;
        let mut kinds: Vec<String> = Vec::new();
        for e in v {
            let s = format!("{e:?}");
            let head = s.split(['{', '(']).next().unwrap_or("?").trim().to_string();
            if let topo::ValidationError::EdgeCertification { edge: ek, .. } = e
                && *ek == edge
            {
                hit += 1;
            }
            kinds.push(head);
        }
        kinds.sort();
        kinds.dedup();
        format!(
            "n={} target_edge_certification_errors={hit} kinds={kinds:?}",
            v.len()
        )
    };
    println!(
        "M3R2|{tag}|validate_geometric        |{}",
        topo::validate_geometric(body, tol).map_or_else(|e| names(&e), |()| "OK".into())
    );
    println!(
        "M3R2|{tag}|validate_pseudomanifold   |{}",
        topo::validate_pseudomanifold(body, &records, tol)
            .map_or_else(|e| names(&e), |()| "OK".into())
    );
    println!(
        "M3R2|{tag}|validate_pm_certificate   |{}",
        topo::validate_pseudomanifold_certificate(body, &records, tol)
            .map_or_else(|e| names(&e), |_| "OK".into())
    );
    println!(
        "M3R2|{tag}|contact_marks             |{}",
        topo::contact_marks(body, tol).map_or_else(|e| names(&e), |_| "OK".into())
    );
    println!(
        "M3R2|{tag}|validate_geom_structural  |{}",
        topo::validate_geometric_structural(body, tol).map_or_else(|e| names(&e), |()| "OK".into())
    );
}

#[test]
fn m3r2_which_at_rest_doors_catch_a_corrupt_plane_nurbs_edge() {
    let (body, edge, bowed) = m7_8_body();
    report("clean", &body, edge);
    let mut corrupt = body;
    corrupt_wall(&mut corrupt, bowed);
    report("corrupt", &corrupt, edge);
}
