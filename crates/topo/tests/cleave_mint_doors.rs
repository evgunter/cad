//! **The edge-mint doors read the plane × NURBS lane off the scalar's
//! policy** — the M7-8 cube (`fixture::m7_8`) re-described, split and
//! re-charted through the public doors at `f64`, and the same doors at a
//! dual refusing the class typed, naming the scalar.
//!
//! The fan `mev`'s re-basing gate is pinned beside its fixture, in
//! `euler`'s own tests (the pillow is crate-internal).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use crate::fixture::m7_8::{edge_findings, m7_8_cube, m7_8_edges, nurbs_wall};
use geom_core::spline::{Knots, SpanLocate};
use topo::construction::Construct;

use geom_core::{Dual64, Point3, Real, Tol};
use topo::construction::Rechart;
use topo::{Body, EdgeKey, EulerOpError, FaceKey};

/// The M7-8 edges of `body` with their restated specs, in arena order.
fn wall_edges<T: SpanLocate>(body: &Body<T>) -> Vec<(EdgeKey, geom_brep::EdgeCurveSpec<T>)> {
    body.edges()
        .filter_map(|(k, e)| match body.get_curve_geom(e.curve) {
            Some(topo::CurveGeom::Certified(c))
                if matches!(c.carrier(), geom::Curve3::Nurbs(_))
                    && matches!(
                        c.description(),
                        geom_brep::EdgeDescription::Intersection { .. }
                    ) =>
            {
                Some((k, c.restated_spec()))
            }
            _ => None,
        })
        .collect()
}

/// The face on the plane `z = 1`, which meets the wall along one edge.
fn top_face(body: &Body<f64>) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.z.abs() > 0.5 && origin.z == 1.0)
        })
        .map(|(k, _)| k)
        .unwrap()
}

#[test]
fn set_edge_curve_re_describes_every_m7_8_edge_at_f64() {
    let mut body = m7_8_cube::<f64>();
    let edges = wall_edges(&body);
    assert_eq!(edges.len(), 4, "the wall's four edges are the class");
    for (edge, spec) in edges {
        body.set_edge_curve(edge, spec, Tol::witness())
            .unwrap_or_else(|e| panic!("edge {edge:?} re-describes at f64: {e:?}"));
    }
    assert_eq!(
        m7_8_edges(&body),
        4,
        "the class survives the re-description"
    );
    assert_eq!(edge_findings(&body), 0, "check 2 is clean after it");
}

/// The split is construction state, and its pcurves minted, tier 3
/// says so through check 11's UNDECIDED arm (Ev's decision 2 in PR
/// 4251: a joinable reading the band cannot decide refuses rather than
/// passing). The two children are one transverse locus of the plane and
/// the NURBS wall, so whether the split vertex is a regular point is
/// read through the wedge between them, whose lever arm on a NURBS
/// surface is not implemented (`dihedral_arm`, an unreadable margin):
/// the reading is undecided at every ε, and the gate refuses it with the
/// join's own words. Beside it, the minted body's one other verdict is
/// check 7's on the wall's sub-range trim image, which the quadrature
/// declines: made although check 11 refused, and reported first, which
/// is the composed door's order when check 11 is the only structural
/// refusal (`validate_geometric_certificate`). This is the at-rest body whose join reading is undecided:
/// a split nearer a pole or an apex refuses at the split's own readings
/// in the same band, and the halfcap files at the import's reading of
/// the pole.
#[test]
fn split_edge_splits_an_m7_8_edge_into_two_of_the_class_at_f64() {
    let mut body = m7_8_cube::<f64>();
    let (edge, _) = wall_edges(&body)[0].clone();
    let split = body
        .split_edge(edge, 0.5, Tol::witness())
        .expect("an M7-8 edge splits at f64")
        .vertex;
    assert_eq!(m7_8_edges(&body), 5, "both children are of the class");
    assert_eq!(edge_findings(&body), 0, "check 2 is clean after the split");

    topo::mint_pcurves(&mut body, Tol::witness()).expect("the split body's pcurves mint");
    let errors = topo::validate_geometric(&body, Tol::witness())
        .expect_err("a split vertex is construction state");
    let [
        topo::ValidationError::VolumeUncomputable { .. },
        undecided @ topo::ValidationError::JoinUndecidedAtRest { undecided: reading },
    ] = &errors[..]
    else {
        panic!("check 7's quadrature refusal, then check 11's undecided arm: {errors:?}");
    };
    assert_eq!(reading.vertex, split, "it names the split vertex");
    let topo::JoinReading::Regularity(diag) = &reading.reading else {
        panic!("the regularity reading: {reading:?}");
    };
    assert_eq!(diag.predicate, Some("dihedral_arm"), "{diag:?}");
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's band");
    assert_eq!(
        undecided.to_string(),
        format!(
            "whether two edges meeting at a vertex on one curve are one edge is undecided \
             (margin is invalid (NaN or a refused enclosure) against the ambiguity band ({:?}, {:?})). \
             Recourse: move the vertex clear of the pole, apex or tangency it sits near; an \
             unreadable margin may indicate a kernel or file defect worth reporting",
            band.zero(),
            band.escalate()
        )
    );
}

/// The top face moves onto a fresh copy of its own plane, and every
/// edge on it — one of them the wall's — re-certifies on the new chart.
#[test]
fn a_rechart_beside_the_wall_re_certifies_its_m7_8_edge_at_f64() {
    let mut body = m7_8_cube::<f64>();
    let top = top_face(&body);
    let face = body.get_face(top).unwrap().clone();
    let plane = body.get_surface(face.surface).unwrap().clone();
    let specs: Vec<_> = body
        .edges()
        .filter(|(_, e)| {
            [e.he_plus, e.he_minus]
                .iter()
                .any(|&h| body.face_of_half_edge(h) == Some(top))
        })
        .map(|(k, e)| match body.get_curve_geom(e.curve) {
            Some(topo::CurveGeom::Certified(c)) => (k, c.restated_spec()),
            other => panic!("edge {k:?} on the top face is certified: {other:?}"),
        })
        .collect();
    assert!(
        specs
            .iter()
            .any(|(_, s)| matches!(s.carrier, geom::Curve3::Nurbs(_))),
        "the listed edges include the wall's"
    );
    body.set_face_surfaces_describing(
        vec![Rechart::new(plane, top, face.sense)],
        &specs,
        Tol::witness(),
    )
    .expect("the top face re-charts with its M7-8 edge at f64");
    assert_ne!(
        body.get_face(top).unwrap().surface,
        face.surface,
        "the face wears the new chart"
    );
    assert_eq!(m7_8_edges(&body), 4, "the class survives the re-chart");
    assert_eq!(edge_findings(&body), 0, "check 2 is clean after it");
}

/// The cube at `T` with its front face on the described wall, before
/// any wall edge is re-described, and the plane × NURBS spec of one wall
/// edge, its witness on both surfaces. Returns the body, that edge, the
/// spec and the wall's face.
fn bare_wall<T>() -> (Body<T>, EdgeKey, geom_brep::EdgeCurveSpec<T>, FaceKey)
where
    T: geom_core::Decide + topo::AtRestPolicy,
{
    let tol = Tol::witness();
    let cube = common::geometric_cube::<T>(tol);
    let mut body = cube.body;
    let wall_face = cube.mefs[1].face;
    let wall = body
        .set_face_surface(
            wall_face,
            topo::construction::FaceSurface::New {
                surface: nurbs_wall::<T>(),
                sense: true,
            },
        )
        .unwrap();
    let sides = |k| topo::readback::edge_sides(&body, k).unwrap().surfaces();
    let (edge, e) = body
        .edges()
        .find(|(k, _)| {
            let (a, b) = sides(*k);
            a == wall || b == wall
        })
        .map(|(k, e)| (k, e.clone()))
        .unwrap();
    let (s1, s2) = sides(edge);
    let point = |he| {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    };
    let (p0, p1): (Point3<T>, Point3<T>) = (point(e.he_plus), point(e.he_minus));
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let spec = geom_brep::EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            pair: geom_brep::SurfacePair::new(s1, s2),
            selector: geom_brep::CurveSelector::Witness(p0.lerp(p1, T::from_f64(0.5))),
        },
        carrier: geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                Knots::from(kv).lift(geom_core::Real::from_f64),
                vec![p0, p1],
                vec![Real::one(); 2],
            )
            .unwrap(),
        )),
        param_start: T::zero(),
        param_end: T::one(),
    };
    (body, edge, spec, wall_face)
}

/// The control the dual rows lean on: the same construction certifies
/// at `f64`.
#[test]
fn the_bare_wall_spec_certifies_at_f64() {
    let (mut body, edge, spec, _) = bare_wall::<f64>();
    body.set_edge_curve(edge, spec, Tol::witness())
        .expect("the spec certifies at f64");
    assert_eq!(m7_8_edges(&body), 1);
}

/// At a dual the edge door mints the class (DL3): the edge takes the
/// spec, carrying no certificate.
#[test]
fn at_a_dual_set_edge_curve_mints_the_class() {
    let (mut body, edge, spec, _) = bare_wall::<Dual64>();
    body.set_edge_curve(edge, spec, Tol::witness())
        .expect("the dual's door mints the spec");
    assert_eq!(m7_8_edges(&body), 1);
}

/// The wall face re-charts onto a fresh copy of its own net, and the
/// listed spec names the wall's current key: the re-chart reads the
/// same edge door as the edge setter, which at a dual mints.
#[test]
fn at_a_dual_a_rechart_mints_the_class() {
    let (mut body, edge, spec, wall_face) = bare_wall::<Dual64>();
    let face = body.get_face(wall_face).unwrap().clone();
    let net = body.get_surface(face.surface).unwrap().clone();
    let stranded: Vec<_> = body
        .edges()
        .filter(|(_, e)| {
            [e.he_plus, e.he_minus]
                .iter()
                .any(|&h| body.face_of_half_edge(h) == Some(wall_face))
        })
        .map(|(k, _)| k)
        .collect();
    assert!(stranded.contains(&edge));
    let got = body
        .set_face_surfaces_describing(
            vec![Rechart::new(net, wall_face, face.sense)],
            &[(edge, spec)],
            Tol::witness(),
        )
        .map(|_| ());
    assert_eq!(got, Ok(()), "the dual's door mints the listed spec");
    assert_ne!(body.get_face(wall_face).unwrap().surface, face.surface);
}

/// `kev_describing` killing the edge that leaves a wall corner off the
/// wall, with the wall edge at that corner listed under its plane × NURBS
/// spec. Returns the door's answer.
fn kev_describing_the_class<T>() -> Result<(), EulerOpError>
where
    T: geom_core::Decide + topo::AtRestPolicy,
{
    let (mut body, edge, spec, wall_face) = bare_wall::<T>();
    let e = body.get_edge(edge).unwrap().clone();
    let corner = body.get_half_edge(e.he_plus).unwrap().start;
    let (he, _) = body
        .half_edges()
        .find(|(k, h)| {
            body.half_edge_end(*k) == Some(corner)
                && body.face_of_half_edge(*k) != Some(wall_face)
                && {
                    let ed = body.get_edge(h.edge).unwrap();
                    body.face_of_half_edge(ed.he_plus) != Some(wall_face)
                        && body.face_of_half_edge(ed.he_minus) != Some(wall_face)
                }
        })
        .map(|(k, h)| (k, h.clone()))
        .unwrap();
    body.kev_describing(he, &[(edge, spec)], Tol::witness())
        .map(|_| ())
}

/// `kev_describing` runs every edge it re-bases through the scalar's
/// edge door: at `f64` the unlisted member the kill moves is certified
/// and its start pin refuses, and at the dual the same member is minted,
/// which reads no endpoint (DL3: the dual's evaluation is read against
/// the `f64` one, which refused).
#[test]
fn kev_describing_runs_the_class_through_the_scalars_door() {
    let f = kev_describing_the_class::<f64>();
    assert!(
        matches!(
            f,
            Err(EulerOpError::RebasedCarrier {
                error: geom_brep::CertifyError::ResidualExceeded {
                    check: geom_brep::CertCheck::EndpointStart,
                    ..
                },
                ..
            })
        ),
        "f64 certifies the moved member and its start pin refuses: {f:?}"
    );
    let dual = kev_describing_the_class::<Dual64>();
    assert_eq!(dual, Ok(()), "the dual's door mints the moved member");
}

/// **The join door refuses whole** (`Body::join_edges`): an ordinary
/// line edge split at its midpoint (a joinable vertex, first in arena
/// order), then an M7-8 edge split (its join reading undecided at every
/// ε, the body this file's first row reaches). The door refuses
/// `JoinUndecided` before any kill, so the ordinary vertex is not joined
/// either and the body is exactly as it was. Before import made a sphere
/// pole a vertex this was read on `step-import`'s half-cap, whose arc
/// through the pole no longer ships.
#[test]
fn a_join_refused_in_the_band_leaves_the_body_as_found() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let mut body = m7_8_cube::<f64>();
    let (line, (t0, t1)) = body
        .edges()
        .find_map(|(k, e)| match body.get_curve_geom(e.curve) {
            Some(topo::CurveGeom::Certified(c))
                if matches!(c.carrier(), geom::Curve3::Line { .. }) =>
            {
                Some((k, c.params()))
            }
            _ => None,
        })
        .expect("a line edge");
    let ordinary = body.split_edge(line, 0.5 * (t0 + t1), tol).unwrap().vertex;
    let (edge, _) = wall_edges(&body)[0].clone();
    body.split_edge(edge, 0.5, tol).unwrap();
    topo::mint_pcurves(&mut body, tol).expect("pcurves mint");
    // The ordinary vertex alone joins: the same split on a fresh cube.
    let mut alone = m7_8_cube::<f64>();
    let lone = alone.split_edge(line, 0.5 * (t0 + t1), tol).unwrap().vertex;
    assert_eq!(lone, ordinary, "the same arena slot");
    alone
        .join_edges(band, tol)
        .expect("an ordinary split vertex joins");
    let before = format!("{body:?}");
    let got = body.join_edges(band, tol);
    assert!(
        matches!(got, Err(topo::BooleanError::JoinUndecided(_))),
        "{got:?}"
    );
    assert_eq!(format!("{body:?}"), before, "refused, so untouched");
}
