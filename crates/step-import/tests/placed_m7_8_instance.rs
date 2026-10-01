//! **A placed STEP instance**
//! whose NURBS wall meets planes (the M7-8 class the importer adopts
//! through the lane) moves through `transform_rigid` at import, and
//! re-mints through the plain edge doors after it.
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
        body.set_edge_curve(
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
/// ask: a placed instance carrying the M7-8 class imports. With the
/// lane taken out of `transform_rigid` it refuses at `Placement`, on the
/// wall's edges.
#[test]
fn a_placed_m7_8_instance_imports() {
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
    re_mints_the_class(placed, m7_8);
}

/// The imported class re-mints through the plain edge doors at `f64`:
/// each M7-8 edge re-describes with its own restated spec through
/// `set_edge_curve`, then one splits through `split_edge` into two of
/// the class. Both doors read the lane off `f64`'s policy.
fn re_mints_the_class(mut body: topo::Body<f64>, count: usize) {
    let class: Vec<_> = body
        .edges()
        .filter_map(|(k, e)| match body.get_curve_geom(e.curve) {
            Some(topo::CurveGeom::Certified(c))
                if matches!(
                    c.description(),
                    geom_brep::EdgeDescription::Intersection { .. }
                ) && matches!(c.carrier(), geom::Curve3::Nurbs(_)) =>
            {
                Some((k, c.restated_spec()))
            }
            _ => None,
        })
        .collect();
    assert_eq!(class.len(), count);
    for (edge, spec) in &class {
        body.set_edge_curve(*edge, spec.clone(), Tol::witness())
            .unwrap_or_else(|e| panic!("imported edge {edge:?} re-describes: {e:?}"));
    }
    let (edge, spec) = &class[0];
    let mid = (spec.param_start + spec.param_end) * 0.5;
    body.split_edge(*edge, mid, Tol::witness())
        .unwrap_or_else(|e| panic!("imported edge {edge:?} splits: {e:?}"));
    let after = body
        .curves()
        .filter(|(_, c)| {
            matches!(c, topo::CurveGeom::Certified(c)
                if matches!(c.description(), geom_brep::EdgeDescription::Intersection { .. })
                    && matches!(c.carrier(), geom::Curve3::Nurbs(_)))
        })
        .count();
    assert_eq!(
        after,
        count + 1,
        "both children of the split are of the class"
    );
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

/// Reviewer e2e (PR 3720): every door the PR body's measurement table
/// names, on the imported, placed M7-8 instance at `f64`.
#[test]
fn review_placed_m7_8_doors_e2e() {
    let tol = Tol::witness();
    let body = m7_8_cube();
    let text = step_export::step_string(&body, &step_export::StepOptions::default(), tol).unwrap();
    let placed = match import_step(&place(&text), &ImportOptions::default(), tol) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("{:?}", other.map(|_| ())),
    };
    let is_class = |b: &topo::Body<f64>, e: topo::EdgeKey| {
        matches!(b.get_curve_geom(b.get_edge(e).unwrap().curve),
            Some(topo::CurveGeom::Certified(c))
                if matches!(c.description(), geom_brep::EdgeDescription::Intersection { .. })
                    && matches!(c.carrier(), geom::Curve3::Nurbs(_)))
    };
    let faces_of = |b: &topo::Body<f64>, e: topo::EdgeKey| {
        let ed = b.get_edge(e).unwrap();
        [b.face_of_half_edge(ed.he_plus).unwrap(), b.face_of_half_edge(ed.he_minus).unwrap()]
    };
    let wall = placed
        .faces()
        .find(|(_, f)| matches!(placed.get_surface(f.surface), Some(geom::Surface::Nurbs(_))))
        .map(|(k, _)| k)
        .unwrap();
    let class: Vec<_> = placed.edges().map(|(k, _)| k).filter(|&e| is_class(&placed, e)).collect();
    let beside: Vec<_> = class.iter().map(|&e| faces_of(&placed, e).into_iter().find(|&f| f != wall).unwrap()).collect();
    let far = placed
        .faces()
        .map(|(k, _)| k)
        .find(|&f| f != wall && !beside.contains(&f))
        .unwrap();
    eprintln!("class edges {}, beside {}", class.len(), beside.len());

    // A re-chart of a face beside the wall onto a copy of its own plane.
    let mut b = placed.clone();
    let face = b.get_face(beside[0]).unwrap().clone();
    let plane = b.get_surface(face.surface).unwrap().clone();
    let specs: Vec<_> = b
        .edges()
        .filter(|(k, _)| faces_of(&b, *k).contains(&beside[0]))
        .map(|(k, e)| match b.get_curve_geom(e.curve) {
            Some(topo::CurveGeom::Certified(c)) => (k, c.restated_spec()),
            other => panic!("{other:?}"),
        })
        .collect();
    let r = b.set_face_surfaces_describing(vec![topo::Rechart::new(plane, beside[0], face.sense)], &specs, tol);
    eprintln!("rechart beside the wall: {:?}", r.as_ref().map(|_| ()));
    assert!(r.is_ok());

    // A fan mev at a class edge's start vertex, at the old point.
    let mut b = placed.clone();
    let hp = b.get_edge(class[0]).unwrap().he_plus;
    let v = b.get_half_edge(hp).unwrap().start;
    let here = *b.get_point(b.get_vertex(v).unwrap().point).unwrap();
    let orbit = b.vertex_orbit(hp).unwrap();
    let r = b.mev(
        topo::MevSite::Fan { he1: hp, he2: orbit[1] },
        here,
        geom_brep::EdgeCurveSpec::self_loop_circle_at(here),
        tol,
    );
    eprintln!("fan mev at the old point: {:?}", r.as_ref().map(|_| ()));

    for (name, f) in [("beside", beside[0]), ("far", far)] {
        let mut b = placed.clone();
        let r = topo::replace_face_offset(&mut b, f, 0.25, tol);
        eprintln!("replace_face_offset({name}, +0.25): {:?}", r.as_ref().map(|_| ()));
    }
    let mut b = placed.clone();
    let r = topo::replace_face_offset(&mut b, wall, 0.1, tol);
    eprintln!("replace_face_offset(wall, +0.1): {:?}", r.as_ref().map(|_| ()));
    let r = topo::shell(&placed, 0.1, tol);
    eprintln!("shell(0.1): {:?}", r.as_ref().map(|_| ()).map_err(|e| format!("{e:?}").chars().take(300).collect::<String>()));
    let r = sweep::blend::fillet_edges(&placed, &[class[0]], 0.1, tol);
    eprintln!("fillet(class edge): {:?}", r.as_ref().map(|_| ()));
    let r = sweep::blend::chamfer_edges(&placed, &[class[0]], 0.1, tol);
    eprintln!("chamfer(class edge): {:?}", r.as_ref().map(|_| ()));
    let plain = placed
        .edges()
        .map(|(k, _)| k)
        .find(|&e| !is_class(&placed, e) && !faces_of(&placed, e).contains(&wall))
        .unwrap();
    let r = sweep::blend::fillet_edges(&placed, &[plain], 0.1, tol);
    eprintln!("fillet(an edge away from the wall): {:?}", r.as_ref().map(|_| ()));
}
