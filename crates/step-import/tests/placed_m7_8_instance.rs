//! **A placed STEP instance**
//! whose NURBS wall meets planes (the M7-8 class the importer adopts
//! through the lane) moves through `transform_rigid` at import, and
//! re-mints through the plain edge doors and the cap offset after it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::test_support::upper;
use step_import::{ImportOptions, StepImport, import_step};
use topo::construction::Construct;

/// The `work/exch/a-placed-step-instance-with-a-plane-nurbs-edge-refuses-at-transform`
/// ask: a placed instance carrying the M7-8 class imports. With the
/// lane taken out of `transform_rigid` it refuses at `Placement`, on the
/// wall's edges.
///
/// The input is a committed document: the unit cube with its `y = 0`
/// face a biquadratic NURBS wall bowed to `y = −0.25` and that face's
/// four rims restated as M7-8 intersections, its other chords still the
/// scaffolds of `topo::test_support::geometric_cube`. That body is not
/// at rest (tier 2 names the scaffolds), so the export door, which takes
/// a finished body, cannot write it; the file is what it wrote.
#[test]
fn a_placed_m7_8_instance_imports() {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/m7-8-cube/m7_8_cube.step"
    ))
    .expect("the committed fixture reads");
    let placed = match import_step(&place(&text), &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => body,
        Ok(StepImport::Wireframe { .. }) => panic!("expected a solid"),
        Err(e) => panic!("placed import refused: {e:?}"),
    };
    let m7_8 = placed
        .body()
        .curves()
        .filter(|(_, c)| {
            matches!(c, topo::CurveGeom::Certified(c)
                if matches!(c.description(), geom_brep::EdgeDescription::Intersection { .. })
                    && matches!(c.carrier(), geom::Curve3::Nurbs(_)))
        })
        .count();
    assert!(m7_8 > 0, "the placed instance carries the M7-8 class");
    // The rotation by a quarter turn about z and the (5, 3, 2) shift.
    for (_, p) in placed.body().points() {
        assert!(
            (4.0..=5.0).contains(&p.x) && (3.0..=4.25).contains(&p.y),
            "{p:?}"
        );
    }
    re_mints_the_class(placed.body().clone(), m7_8);
    offsets_beside_the_wall(placed.body(), m7_8);
}

/// How many M7-8 edges `body` holds.
fn class_count(body: &topo::Body<f64>) -> usize {
    body.curves()
        .filter(|(_, c)| {
            matches!(c, topo::CurveGeom::Certified(c)
                if matches!(c.description(), geom_brep::EdgeDescription::Intersection { .. })
                    && matches!(c.carrier(), geom::Curve3::Nurbs(_)))
        })
        .count()
}

/// The planar faces that meet the wall along an M7-8 edge (the two caps
/// the wall's rims lie in), offset through `replace_face_offset`, whose
/// re-chart re-certifies the rim on the moved cap through the lane.
/// Inward the cap plane still crosses the wall's patch, so the offset
/// succeeds and the class survives; outward by `0.25` the plane misses
/// the patch, which ends at the cap, by exactly that distance, and the
/// lane refuses the moved rim with the measured gap (`OnLocus`).
fn offsets_beside_the_wall(body: &topo::Body<f64>, count: usize) {
    let tol = Tol::witness();
    let wall = body
        .faces()
        .find(|(_, f)| matches!(body.get_surface(f.surface), Some(geom::Surface::Nurbs(_))))
        .map(|(k, _)| k)
        .unwrap();
    let caps: Vec<_> = body
        .edges()
        .filter(|(_, e)| {
            matches!(body.get_curve_geom(e.curve), Some(topo::CurveGeom::Certified(c))
                if matches!(c.carrier(), geom::Curve3::Nurbs(_)))
        })
        .filter_map(|(_, e)| {
            [e.he_plus, e.he_minus]
                .into_iter()
                .map(|h| body.face_of_half_edge(h).unwrap())
                .find(|&f| f != wall)
        })
        .collect();
    assert_eq!(caps.len(), count, "one cap beside each M7-8 edge");
    for cap in caps {
        let mut inward = body.clone();
        topo::replace_face_offset(&mut inward, cap, -0.25, tol)
            .unwrap_or_else(|e| panic!("cap {cap:?} offsets inward: {e:?}"));
        assert_eq!(class_count(&inward), count, "the class survives the offset");
        let mut outward = body.clone();
        match topo::replace_face_offset(&mut outward, cap, 0.25, tol) {
            Err(topo::ReplaceFaceError::Op {
                error:
                    topo::EulerOpError::RechartFalsifies {
                        error:
                            geom_brep::CertifyError::PlaneNurbs(geom_brep::PlaneNurbsRefusal::Limb {
                                limb: geom_brep::SsiLimb::OnLocus,
                                margin,
                            }),
                        ..
                    },
                ..
            }) => assert!((upper(margin) - 0.25).abs() < 1e-9, "the gap: {margin}"),
            other => panic!("cap {cap:?} outward: {:?}", other.map(|_| ())),
        }
    }
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
