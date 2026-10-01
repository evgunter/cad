//! Reviewer probe (CLEAVE, PR 3678, lane r2): a placed STEP instance
//! whose NURBS wall meets planes (the M7-8 class the importer adopts
//! through the lane) moves through `transform_rigid` at import.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use step_import::{ImportOptions, StepImport, import_step};
use topo::test_support as tc;

/// The wall's bow: the front face's z-rims are genuine quadratics.
const BOW: f64 = -0.25;

fn nurbs_wall() -> geom::Surface<f64> {
    let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let xs = [(0.0, 0.0), (0.5, BOW), (1.0, 0.0)];
    let ticks = [0.0, 0.5, 1.0];
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for &(x, y) in &xs {
        for &z in &ticks {
            control.push(Point3::new(x, y, z));
            weights.push(1.0);
        }
    }
    geom::Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(k.clone(), k, control, weights).unwrap(),
    ))
}

fn m7_8_cube() -> topo::Body<f64> {
    let cube = tc::geometric_cube::<f64>(Tol::witness());
    let mut body = cube.body;
    let wall = body
        .set_face_surface(
            cube.mefs[1].face,
            topo::FaceSurface::New {
                surface: nurbs_wall(),
                sense: true,
            },
        )
        .unwrap();
    let edges: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    for (edge_key, edge) in edges {
        let s1 = tc::face_surface_of_he(&body, edge.he_plus);
        let s2 = tc::face_surface_of_he(&body, edge.he_minus);
        if s1 != wall && s2 != wall {
            continue;
        }
        let start = body.get_half_edge(edge.he_plus).unwrap().start;
        let end = body.half_edge_end(edge.he_plus).unwrap();
        let p0 = *body
            .get_point(body.get_vertex(start).unwrap().point)
            .unwrap();
        let p1 = *body.get_point(body.get_vertex(end).unwrap().point).unwrap();
        let carrier = if (p0.x - p1.x).abs() > 0.5 {
            // A z-rim: the quadratic the wall's bow traces in the plane.
            let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2)
                .unwrap();
            let mid = Point3::new(0.5, BOW, p0.z);
            geom::Curve3::Nurbs(std::sync::Arc::new(
                geom::NurbsCurve3::new(kv, vec![p0, mid, p1], vec![1.0, 1.0, 1.0]).unwrap(),
            ))
        } else {
            let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            geom::Curve3::Nurbs(std::sync::Arc::new(
                geom::NurbsCurve3::new(kv, vec![p0, p1], vec![1.0, 1.0]).unwrap(),
            ))
        };
        body.set_edge_curve_nurbs_lane(
            edge_key,
            geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    s1,
                    s2,
                    witness: if (p0.x - p1.x).abs() > 0.5 {
                        Point3::new(0.5, 0.5 * BOW, p0.z)
                    } else {
                        p0.lerp(p1, 0.5)
                    },
                },
                carrier,
                param_start: 0.0,
                param_end: 1.0,
            },
            Tol::witness(),
        )
        .unwrap();
    }
    body
}

/// The `work/exch/a-placed-step-instance-with-a-plane-nurbs-edge-refuses-at-transform`
/// ask: a placed instance carrying the M7-8 class imports. With
/// `transform_rigid`'s policy read reverted to `None` this refuses
/// `Placement { Certify { NurbsLaneUnsupported { scalar: "f64" } } }`.
#[test]
fn rr2_placed_m7_8_instance_imports() {
    let body = m7_8_cube();
    let text =
        step_export::step_string(&body, &step_export::StepOptions::default(), Tol::witness())
            .expect("exports");
    let placed = match import_step(&place(&text), &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => body,
        Ok(StepImport::Wireframe { .. }) => panic!("expected a solid"),
        Err(e) => panic!("placed import refused: {e:?}"),
    };
    let m7_8 = placed
        .curves()
        .filter(|(_, c)| {
            matches!(c, topo::CurveGeom::Certified(c)
                if matches!(c.description(), geom_brep::EdgeDescription::Intersection { .. })
                    && matches!(c.carrier(), geom::Curve3::Nurbs(_)))
        })
        .count();
    assert!(m7_8 > 0, "the placed instance carries the M7-8 class");
    // The rotation by a quarter turn about z and the (5, 3, 2) shift.
    for (_, p) in placed.points() {
        assert!(
            (4.0..=5.0).contains(&p.x) && (3.0..=4.25).contains(&p.y),
            "{p:?}"
        );
    }
}

/// Places the exported representation into a fresh root
/// representation by a rotation about z plus a translation.
fn place(text: &str) -> String {
    let rep = text
        .lines()
        .find(|l| l.contains("ADVANCED_BREP_SHAPE_REPRESENTATION"))
        .and_then(|l| l.split_whitespace().next())
        .unwrap()
        .to_string();
    let line = text
        .lines()
        .find(|l| l.contains("ADVANCED_BREP_SHAPE_REPRESENTATION"))
        .unwrap();
    // `('', (#axis, #msb), #ctx)`
    let inner = &line[line
        .find("((")
        .map_or_else(|| line.find("(#").unwrap(), |i| i + 1)..];
    let ids: Vec<&str> = inner
        .split(|c: char| !(c == '#' || c.is_ascii_digit()))
        .filter(|t| t.starts_with('#'))
        .collect();
    let (axis, ctx) = (ids[0], ids[ids.len() - 1]);
    let block = format!(
        "#9000 = SHAPE_REPRESENTATION('', ({axis}), {ctx});\n\
         #9001 = ( REPRESENTATION_RELATIONSHIP('','',{rep},#9000) \
         REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION(#9002) SHAPE_REPRESENTATION_RELATIONSHIP() );\n\
         #9002 = ITEM_DEFINED_TRANSFORMATION('','',{axis},#9003);\n\
         #9003 = AXIS2_PLACEMENT_3D('',#9004,#9005,#9006);\n\
         #9004 = CARTESIAN_POINT('',(5.0,3.0,2.0));\n\
         #9005 = DIRECTION('',(0.,0.,1.));\n\
         #9006 = DIRECTION('',(0.,1.,0.));\n"
    );
    let at = text.rfind("ENDSEC;").unwrap();
    format!("{}{block}{}", &text[..at], &text[at..])
}
