//! **A vertex its description fixes is built where its surfaces meet**
//! (`[import]`, D7 step 2; `crates/step-import/src/build.rs`,
//! `build_vertices`): the corner three planes state is the root of the
//! third along the line the other two meet in, whatever point the file
//! printed for it within its band. The foot of the printed point on any
//! one of the corner's edge lines is not the corner where the printed
//! point stands off all three, so this row tells the root from a foot.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

/// The cube fixture with vertex `#7`'s printed point `(0, 1, 1)` moved
/// off its corner by `delta` along every axis: off each of the three
/// edge lines through the corner, within the file's band.
fn moved_corner(delta: f64) -> String {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../step-export/tests/fixtures/cube.step"
    ))
    .expect("the cube fixture reads");
    let at = "#6 = CARTESIAN_POINT('', (0.0, 1.0, 1.0));";
    assert!(src.contains(at), "vertex #7's point is #6 at the corner");
    src.replace(
        at,
        &format!(
            "#6 = CARTESIAN_POINT('', ({:?}, {:?}, {:?}));",
            delta,
            1.0 + delta,
            1.0 + delta
        ),
    )
}

#[test]
fn a_corner_printed_off_its_planes_is_built_at_their_meeting() {
    let delta = 2e-10;
    let text = moved_corner(delta);
    let Ok(StepImport::Solid {
        body,
        normalizations,
        ..
    }) = import_step(&text, &ImportOptions::default(), Tol::witness())
    else {
        panic!("the cube with a corner printed within its band imports");
    };
    let corner = Point3::new(0.0, 1.0, 1.0);
    let built: Vec<Point3<f64>> = body
        .points()
        .map(|(_, p)| *p)
        .filter(|p| p.distance(corner) < 1e-6)
        .collect();
    assert_eq!(built.len(), 1, "one vertex stands at the corner: {built:?}");
    let bits = |p: Point3<f64>| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
    assert_eq!(
        bits(built[0]),
        bits(corner),
        "the vertex is the three planes' meeting, not a foot of the printed point: {:?}",
        built[0]
    );
    let moved: Vec<f64> = normalizations
        .iter()
        .filter(|n| n.face == 7)
        .filter_map(|n| match n.kind {
            NormalizationKind::VertexBuilt { moved_by } => Some(moved_by),
            _ => None,
        })
        .collect();
    let off = Point3::new(delta, 1.0 + delta, 1.0 + delta).distance(corner);
    assert_eq!(
        moved,
        vec![off],
        "the ledger reports the corner's move off its printed point"
    );
}
