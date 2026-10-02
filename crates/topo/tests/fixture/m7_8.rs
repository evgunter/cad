//! **The M7-8 cube** — the plane × described-NURBS edge class on the
//! smallest body that carries it, shared by every suite that moves,
//! grafts or validates that class.
//!
//! The unit cube with its `y = 0` wall restated as a described degree-2
//! NURBS net on the same plane: the wall's four edges are plane × NURBS
//! `Intersection`s minted through `Body::set_edge_curve`, and
//! the other eight are plane × plane `Intersection`s, so no edge keeps
//! the construction's scaffolding description.
//!
//! It is NOT valid at rest: the wall's half-edges carry no pcurve
//! cache, so check 7 answers `VolumeUncomputable` on the NURBS face.
//! `topo::mint_pcurves` closes that (the body then passes
//! `validate_geometric`), at about 85 s of debug-build quadrature per
//! validation, which is why the rows here count check 2 alone
//! ([`edge_findings`]) and claim nothing wider.

use geom_core::{Point3, Real, Tol};
use topo::Body;

/// The described NURBS net on `y = 0`, u along +x, v along +z, at `T`.
pub fn nurbs_wall<T: Real>() -> geom::Surface<T> {
    let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let ticks = [-1.0, 0.5, 2.0];
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for &x in &ticks {
        for &z in &ticks {
            control.push(Point3::new(x, 0.0, z));
            weights.push(1.0);
        }
    }
    let n = geom::NurbsSurface::new(k.clone(), k, control, weights).unwrap();
    assert!(!n.is_placeholder(), "the wall is DESCRIBED");
    geom::Surface::Nurbs(std::sync::Arc::new(n)).map_scalar(T::from_f64)
}

/// The M7-8 cube at any scalar that holds the lane (module docs).
pub fn m7_8_cube<T>() -> Body<T>
where
    T: geom_core::Decide + geom_core::CertifiedBounds + topo::AtRestPolicy,
{
    let tol = Tol::witness();
    let cube = topo::test_support::geometric_cube::<T>(tol);
    let mut body = cube.body;
    let wall = body
        .set_face_surface(
            cube.mefs[1].face,
            topo::FaceSurface::New {
                surface: nurbs_wall::<T>(),
                sense: true,
            },
        )
        .unwrap();
    let edges: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    let mut lane_edges = 0;
    for (edge_key, edge) in edges {
        let (s1, s2) = topo::readback::edge_sides(&body, edge_key)
            .unwrap()
            .surfaces();
        let start = body.get_half_edge(edge.he_plus).unwrap().start;
        let end = body.half_edge_end(edge.he_plus).unwrap();
        let p0 = *body
            .get_point(body.get_vertex(start).unwrap().point)
            .unwrap();
        let p1 = *body.get_point(body.get_vertex(end).unwrap().point).unwrap();
        let witness = p0.lerp(p1, T::from_f64(0.5));
        let description = geom_brep::EdgeDescriptionSpec::Intersection { s1, s2, witness };
        if s1 != wall && s2 != wall {
            let mut spec = geom_brep::EdgeCurveSpec::line_between(p0, p1);
            spec.description = description;
            body.set_edge_curve(edge_key, spec, tol).unwrap();
            continue;
        }
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(kv, vec![p0, p1], vec![1.0, 1.0]).unwrap(),
        ));
        body.set_edge_curve(
            edge_key,
            geom_brep::EdgeCurveSpec {
                description,
                carrier,
                param_start: T::zero(),
                param_end: T::one(),
            },
            tol,
        )
        .unwrap();
        lane_edges += 1;
    }
    assert_eq!(lane_edges, 4, "the front wall has four M7-8 edges");
    body
}

/// The check-2 findings (`EdgeCertification`) the certified at-rest door
/// raises on `body` — the carrier question alone; check 7 on the
/// described-NURBS face is a separate one.
pub fn edge_findings<T>(body: &Body<T>) -> usize
where
    T: geom_core::Decide + geom_core::CertifiedBounds + topo::AtRestPolicy,
{
    match topo::validate_pseudomanifold(body, &topo::ContactRecords::default(), Tol::witness()) {
        Ok(()) => 0,
        Err(errs) => errs
            .iter()
            .filter(|e| matches!(e, topo::ValidationError::EdgeCertification { .. }))
            .count(),
    }
}

/// How many M7-8 carriers (a `Nurbs` carrier under an `Intersection`)
/// `body` holds, so a row that says the class survived cannot pass on a
/// body that lost it.
pub fn m7_8_edges<T: Real>(body: &Body<T>) -> usize {
    body.curves()
        .filter(|(_, c)| match c {
            topo::CurveGeom::Certified(c) => {
                matches!(c.carrier(), geom::Curve3::Nurbs(_))
                    && matches!(
                        c.description(),
                        geom_brep::EdgeDescription::Intersection { .. }
                    )
            }
            topo::CurveGeom::NullScaffold(_) => false,
        })
        .count()
}

/// Strict-inside carried evidence for every shell of `cavity`.
pub fn carried<T: Real>(cavity: &Body<T>) -> topo::VoidEvidence {
    topo::VoidEvidence {
        shells: cavity
            .shells()
            .map(|(s, _)| {
                (
                    s,
                    topo::VoidContainment::Carried {
                        sign: geom_core::Sign::Positive,
                    },
                )
            })
            .collect(),
    }
}
