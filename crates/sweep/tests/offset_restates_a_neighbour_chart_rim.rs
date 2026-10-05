//! **An offset door restates a rim described in its neighbour's chart,
//! and the describing door refuses the moved wall it bounds.**
//!
//! `work/topo/a-listed-spec-cannot-name-a-fresh-chart-a-neighbour-keeps-the-old-key-of`'s
//! production shape. A drum's bottom rim, re-described as an image in
//! the bottom cap's chart; the wall offset inward, the caps held
//! (`offset_charts_together` keeps a chart asked to move nothing). The
//! door restates the rim on the cap's key — the moved rim does lie on
//! the cap — and hands it to `set_face_surfaces_describing`, where the
//! wall moves onto a freshly minted cylinder the rim's spec never
//! names. The move is sound (the offset door checks the moved rim on
//! the new wall), but no curved residual is read, so it is refused
//! `RechartUnvouched`; the caller is the offset door, which has no
//! re-description of its own to list. This pins today's refusal; the
//! row's fix turns it into the offset wall.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use crate::common::approx::band;
use geom::Surface;
use geom_brep::EdgeDescriptionSpec;
use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::readback::edge_sides;
use topo::{ChartMove, EulerOpError, RechartDoor, ReplaceFaceError, offset_charts_together};

#[test]
fn a_rim_in_the_caps_chart_refuses_the_walls_offset() {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(3.0 / 64.0, 0.0), 0.0),
            (Point2::new(3.0 / 64.0, 8.0 / 64.0), 0.0),
            (Point2::new(0.0, 8.0 / 64.0), 0.0),
        ])],
    )
    .validate(Tol::witness())
    .unwrap();
    let mut body = revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    let kind = |key| body.get_surface(key).unwrap();
    let wall = body
        .faces()
        .find(|(_, f)| matches!(kind(f.surface), Surface::Cylinder { .. }))
        .map(|(_, f)| f.surface)
        .unwrap();
    let (rim, cap) = body
        .edges()
        .find_map(|(e, _)| match edge_sides(&body, e).unwrap().surfaces() {
            (s, cap) | (cap, s) if s == wall && cap != wall => Some((e, cap)),
            _ => None,
        })
        .unwrap();
    assert!(
        matches!(kind(cap), Surface::Plane { .. }),
        "a rim meets a cap"
    );
    let mut spec = body
        .get_edge(rim)
        .and_then(|e| body.get_curve_geom(e.curve))
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .restated_spec();
    spec.description = EdgeDescriptionSpec::chart(cap);
    body.set_edge_curve(rim, spec, Tol::witness())
        .expect("the rim lies in the cap's chart");

    let moves: Vec<ChartMove<f64>> = crate::common::charts::charts(&body)
        .into_iter()
        .map(|faces| ChartMove {
            distance: if body.get_face(faces[0]).unwrap().surface == wall {
                -1.0 / 64.0
            } else {
                0.0
            },
            faces,
        })
        .collect();
    let got = offset_charts_together(&mut body, &moves, band(), Tol::witness());
    assert!(
        matches!(
            got,
            Err(ReplaceFaceError::Op {
                edge: None,
                error: EulerOpError::RechartUnvouched {
                    door: RechartDoor::SetFaceSurfacesDescribing,
                    ref edges,
                    chord: false,
                    ..
                },
            }) if *edges == [rim]
        ),
        "the wall's offset, its rim in the cap's chart: {got:?}"
    );
}
