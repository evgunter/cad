//! Review 6152 rows: a pierce near a crease, and a chart point that is not
//! the vertex's.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::boolean::{BooleanError, BooleanOp, GluedPairs, Operand, ReductionContacts, SideCode};
use crate::{Body, FaceKey, VertexKey};
use geom_core::{Band, Point3, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn ridge() -> geom::NurbsSurface<f64> {
    use geom_core::spline::KnotVector;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control: Vec<_> = [(0.0, 1.0), (0.25, 1.1), (0.5, 1.2), (0.75, 1.1), (1.0, 1.0)]
        .iter()
        .flat_map(|&(x, z)| [0.0, 1.0].map(|y| Point3::new(x, y, z)))
        .collect();
    geom::NurbsSurface::new(ku, kv, control, vec![1.0; 10]).unwrap()
}

fn ridged_cube() -> (Body<f64>, FaceKey) {
    let cube = crate::test_support_fixtures::prism::<f64>(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
        Tol::witness(),
    );
    let mut body = cube.body;
    body.set_face_surface_unvouched_for_tests(
        cube.top_face,
        crate::construction::FaceSurface::New {
            surface: geom::Surface::Nurbs(std::sync::Arc::new(ridge())),
            sense: true,
        },
    )
    .unwrap();
    (body, cube.top_face)
}

fn prism_above(p: Point3<f64>) -> (Body<f64>, VertexKey) {
    let prism = crate::test_support_fixtures::prism_z::<f64>(
        &[(p.x, p.y), (p.x - 0.2, p.y + 0.06), (p.x - 0.2, p.y - 0.06)],
        p.z,
        p.z + 0.2,
        Tol::witness(),
    );
    (prism.body, prism.bottom[0])
}

fn classify(
    piercing: &mut Body<f64>,
    corner: VertexKey,
    (pierced, face): (&Body<f64>, FaceKey),
    chart: Option<(f64, f64)>,
) -> Result<Vec<SideCode>, BooleanError> {
    super::classify_vertex_on_face(
        piercing,
        pierced,
        Operand::A,
        crate::boolean::VfContact {
            vertex: corner,
            face,
        },
        BooleanOp::Union,
        &GluedPairs::default(),
        &ReductionContacts::default(),
        None,
        None,
        chart,
        &crate::sector_shape::SlitTips::default(),
        band(),
    )
    .map(|out| out.classes.into_iter().map(|(_, c)| c).collect())
}

fn pierce_at(du: f64) -> Result<Vec<SideCode>, BooleanError> {
    let (cube, top) = ridged_cube();
    let uv = (0.5 - du, 0.5);
    let p = ridge().eval(uv.0, uv.1);
    let (mut prism, corner) = prism_above(p);
    classify(&mut prism, corner, (&cube, top), Some(uv))
}

/// Green: a chart point within the band of the ridge's crease refuses
/// by name, not only one exactly on it.
#[test]
fn rv_a_pierce_within_the_band_of_a_crease_refuses_by_name() {
    for du in [0.0, 1e-14, 1e-12, 1e-10] {
        let got = pierce_at(du);
        assert!(
            matches!(got, Err(BooleanError::NormalAtSplineCrease { .. })),
            "du {du:e}: {got:?}"
        );
    }
}

/// RED on f2681f2: a crease 1e-6 from the pierce, outside the band and
/// inside every sector's reach, is refused as a kind with no arm
/// (`CurvedBooleanUnsupported { kind: Nurbs }`, from the lever's
/// `spline_chart_arm` returning `None`), not by name.
#[test]
fn rv_a_pierce_a_micron_from_a_crease_refuses_by_name() {
    let got = pierce_at(1e-6);
    assert!(
        matches!(got, Err(BooleanError::NormalAtSplineCrease { .. })),
        "{got:?}"
    );
}

/// RED on f2681f2: a smooth chart point 0.25 from the crease, beyond
/// every sector's reach (about 0.21), is refused as a kind with no arm:
/// the patch bound refuses an integral chart with a C0 crease anywhere,
/// so no pierce of a creased integral chart reads.
#[test]
fn rv_a_pierce_far_from_a_crease_reads_its_sides() {
    let got = pierce_at(0.25);
    assert!(got.is_ok(), "{got:?}");
}

/// A chart point that is not the vertex's (a stale or loosely keyed
/// record) must not read: the gate refuses it.
#[test]
fn rv_a_chart_point_not_the_vertexs_does_not_read() {
    let (cube, top) = ridged_cube();
    let p = ridge().eval(0.25, 0.5);
    let (mut prism, corner) = prism_above(p);
    for uv in [(0.25 + 1e-6, 0.5), (0.25, 0.9), (0.5, 0.25), (0.75, 0.5)] {
        let got = classify(&mut prism, corner, (&cube, top), Some(uv));
        assert!(
            matches!(got, Err(BooleanError::ClassificationInvariant { .. })),
            "uv {uv:?}: {got:?}"
        );
    }
}
