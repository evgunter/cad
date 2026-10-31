//! **A vertex piercing a spline face** ([`super::classify_vertex_on_face`]):
//! the pierced face's outward normal is its chart's at the chart point
//! the sweep placed the pierce at, in the face's sense
//! ([`crate::face_normal::spline_outward_normal_at`]).
//!
//! The pierced body is the unit cube with its top recarried on the bump
//! `z = 1 + 0.4·u(1 − u)` (`x = u`, `y = v`), whose normal at `u = ¼`
//! leans `(−0.2, 0, 1)`. The piercing body is a thin triangular prism
//! whose corner sits on the bump at `(¼, ½)`: its two horizontal edges
//! run toward `−x` or `+x`, where the bump falls away below or rises
//! above them, and its third edge is vertical. Against `+z`, the bump's
//! corners' plane, both horizontal edges would read `On`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::boolean::{BooleanError, BooleanOp, GluedPairs, Operand, ReductionContacts, SideCode};
use crate::{Body, FaceKey, VertexKey};
use geom_core::{Band, Point3, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const UV: (f64, f64) = (0.25, 0.5);

fn bump() -> geom::NurbsSurface<f64> {
    use geom_core::spline::KnotVector;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control: Vec<_> = [(0.0, 1.0), (0.5, 1.2), (1.0, 1.0)]
        .iter()
        .flat_map(|&(x, z)| [0.0, 1.0].map(|y| Point3::new(x, y, z)))
        .collect();
    geom::NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap()
}

/// The unit cube with its top on [`bump`] at `sense`, and the top face.
fn bumped_cube(sense: bool) -> (Body<f64>, FaceKey) {
    let cube = crate::test_support_fixtures::prism::<f64>(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
        Tol::witness(),
    );
    let mut body = cube.body;
    body.set_face_surface_unvouched_for_tests(
        cube.top_face,
        crate::construction::FaceSurface::New {
            surface: geom::Surface::Nurbs(std::sync::Arc::new(bump())),
            sense,
        },
    )
    .unwrap();
    (body, cube.top_face)
}

/// The thin prism whose corner is the bump's point at [`UV`]: above it,
/// its horizontal edges toward `−x` (`toward = −1`), or below it, toward
/// `+x` (`toward = 1`). The body and the corner.
fn prism_at_the_bump(toward: f64) -> (Body<f64>, VertexKey) {
    let p = bump().eval(UV.0, UV.1);
    let profile = [
        (p.x, p.y),
        (p.x + 0.2 * toward, p.y - 0.06 * toward),
        (p.x + 0.2 * toward, p.y + 0.06 * toward),
    ];
    let (z0, z1) = if toward < 0.0 {
        (p.z, p.z + 0.2)
    } else {
        (p.z - 0.2, p.z)
    };
    let prism = crate::test_support_fixtures::prism_z::<f64>(&profile, z0, z1, Tol::witness());
    let corner = if toward < 0.0 {
        prism.bottom[0]
    } else {
        prism.top[0]
    };
    let at = *prism
        .body
        .get_point(prism.body.get_vertex(corner).unwrap().point)
        .unwrap();
    assert!(
        (at - p).norm() < 1e-12,
        "the corner is the bump's point: {at:?}"
    );
    (prism.body, corner)
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

/// **A vertex piercing a NURBS wall reads each of its edges on the
/// side the chart's normal at the placed point puts it, in either
/// sense of the face.** Above the bump every edge is outside the cube's
/// material and below it every edge is inside; the `false` face's
/// material is the other side, and every class swaps. A normal read as
/// `+z` reads the horizontal edges `On`; one read without the sense, or
/// with it folded twice, swaps one sense's classes.
#[test]
fn a_vertex_piercing_a_nurbs_wall_reads_its_side_in_either_sense() {
    for sense in [true, false] {
        let (cube, top) = bumped_cube(sense);
        for (toward, above) in [(-1.0, true), (1.0, false)] {
            let (mut prism, corner) = prism_at_the_bump(toward);
            let classes = classify(&mut prism, corner, (&cube, top), Some(UV))
                .unwrap_or_else(|e| panic!("sense {sense}, above {above}: {e:?}"));
            let want = if above == sense {
                SideCode::Out
            } else {
                SideCode::In
            };
            assert!(
                classes.len() == 3 && classes.iter().all(|&c| c == want),
                "sense {sense}, above {above}: the corner's three edges read {classes:?}, \
                 not all {want:?}"
            );
        }
    }
}

/// **A pierce of a spline face the sweep placed at no chart point
/// refuses C5's typed refusal, naming the kind**, as it did before the
/// face had an arm: the pass reads no foot of its own.
#[test]
fn a_pierce_of_a_spline_face_with_no_chart_point_refuses_the_kind() {
    let (cube, top) = bumped_cube(true);
    let (mut prism, corner) = prism_at_the_bump(-1.0);
    let got = classify(&mut prism, corner, (&cube, top), None);
    assert!(
        matches!(
            got,
            Err(BooleanError::CurvedBooleanUnsupported {
                operand: Operand::B,
                face,
                kind: geom::SurfaceKind::Nurbs,
            }) if face == top
        ),
        "{got:?}"
    );
}

/// **A pierce on a crease of the pierced chart refuses by name**
/// ([`BooleanError::NormalAtSplineCrease`]): the ridge chart's double
/// knot at `u = ½` folds the face, and a pierce placed on it has a
/// normal on each side.
#[test]
fn a_pierce_on_a_crease_of_the_pierced_chart_refuses_by_name() {
    use geom_core::spline::KnotVector;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control: Vec<_> = [(0.0, 1.0), (0.25, 1.1), (0.5, 1.2), (0.75, 1.1), (1.0, 1.0)]
        .iter()
        .flat_map(|&(x, z)| [0.0, 1.0].map(|y| Point3::new(x, y, z)))
        .collect();
    let ridge = geom::NurbsSurface::new(ku, kv, control, vec![1.0; 10]).unwrap();
    let (mut cube, top) = bumped_cube(true);
    cube.set_face_surface_unvouched_for_tests(
        top,
        crate::construction::FaceSurface::New {
            surface: geom::Surface::Nurbs(std::sync::Arc::new(ridge.clone())),
            sense: true,
        },
    )
    .unwrap();
    let p = ridge.eval(0.5, 0.5);
    let prism = crate::test_support_fixtures::prism_z::<f64>(
        &[(p.x, p.y), (p.x - 0.2, p.y + 0.06), (p.x - 0.2, p.y - 0.06)],
        p.z,
        p.z + 0.2,
        Tol::witness(),
    );
    let (mut body, corner) = (prism.body, prism.bottom[0]);
    let got = classify(&mut body, corner, (&cube, top), Some((0.5, 0.5)));
    assert!(
        matches!(
            got,
            Err(BooleanError::NormalAtSplineCrease { operand: Operand::B, face, .. }) if face == top
        ),
        "{got:?}"
    );
}
