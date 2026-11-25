//! **Import and native construction agree at a chart singularity.** A
//! full revolve reaches each pole and apex by one wrap edge its wall
//! walks out and back (DESIGN `[topology]`'s seam clause), and the
//! export states that edge as one `EDGE_CURVE` the face's loop cites
//! both ways. Import adopts it as stated: the body that comes back has
//! the built body's census and volume, passes tier 3, meshes, and
//! re-exports to a file that re-imports to the same census and the same
//! text.
//!
//! The same holds for the files that state a singularity some other
//! way and are re-minted into that form: Open CASCADE's whole sphere
//! (an edge-free face) and its full cone and cap (a seam ending at the
//! apex or pole, the wrap tip as written).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use std::f64::consts::PI;

use common::arena_census;
use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use step_export::{StepOptions, step_string};
use step_import::{StepImport, import_step};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::AtRestBody;
use topo::test_support::finished;

/// The certified volume `(value, pad)` at the pinned resolution.
fn volume(body: &AtRestBody<f64>, tol: Tol) -> (f64, f64) {
    let enclosure = body
        .measure(topo::test_support::pinned_resolution::<f64>(tol))
        .expect("the body measures")
        .enclosure;
    (enclosure.volume, enclosure.volume_pad)
}

fn export(name: &str, body: &AtRestBody<f64>, tol: Tol) -> String {
    step_string(
        body,
        &StepOptions {
            product_name: name.to_owned(),
            ..StepOptions::default()
        },
        tol,
    )
    .unwrap_or_else(|e| panic!("{name}: export: {e:?}"))
}

fn import(name: &str, text: &str, tol: Tol) -> AtRestBody<f64> {
    match import_step(text, &common::own_import_options(), tol) {
        Ok(StepImport::Solid { body, .. }) => body,
        Ok(other) => panic!("{name}: imported as something other than a solid: {other:?}"),
        Err(e) => panic!("{name}: import: {e_at}", e_at = e.render()),
    }
}

/// The imported body passes tier 3 and meshes, and its re-export
/// re-imports to the same census and volume and, where `text` is set,
/// exports the same text again.
fn settles(name: &str, back: &AtRestBody<f64>, text: bool, tol: Tol) {
    assert_eq!(topo::validate_geometric(back, tol), Ok(()), "{name}: tier 3");
    assert!(
        mesh::tessellate(back, 1e-3, tol).is_ok(),
        "{name}: the imported body meshes"
    );
    let first = export(name, back, tol);
    let again = import(name, &first, tol);
    assert_eq!(
        arena_census(&again),
        arena_census(back),
        "{name}: the re-import's census"
    );
    let ((v, pad), (v_again, pad_again)) = (volume(back, tol), volume(&again, tol));
    assert!(
        (v_again - v).abs() <= pad + pad_again + 1e-12 * v.abs(),
        "{name}: the re-import's volume {v_again} ± {pad_again} against {v} ± {pad}"
    );
    if text {
        assert_eq!(export(name, &again, tol), first, "{name}: the re-export's text");
    }
}

/// The closed sketch chain `verts` (bulge form) revolved a full turn
/// about the sketch y axis, exported, and imported: the census
/// `(faces, edges, vertices)` the revolve builds, held by the import,
/// and the volume both certify against `closed_form`.
fn round_trips(
    name: &str,
    verts: Vec<(Point2<f64>, f64)>,
    census: (usize, usize, usize),
    closed_form: f64,
    text: bool,
) {
    let tol = Tol::witness();
    let profile = Profile::new(SketchPlane::xy(), vec![bulge_loop(verts)])
        .validate(tol)
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let built = revolve(&profile, axis, Revolution::Full, tol)
        .unwrap_or_else(|e| panic!("{name}: the revolve: {e:?}"))
        .body
        .into_body();
    let built = finished(name, built, tol);
    let counts = |b: &AtRestBody<f64>| (b.faces().count(), b.edges().count(), b.vertex_points().count());
    assert_eq!(counts(&built), census, "{name}: the built census");
    let back = import(name, &export(name, &built, tol), tol);
    assert_eq!(
        arena_census(&back),
        arena_census(&built),
        "{name}: census (solids, shells, faces, edges, vertices)"
    );
    let ((v_built, pad_built), (v_back, pad_back)) = (volume(&built, tol), volume(&back, tol));
    let slop = 1e-12 * v_built.abs();
    assert!(
        (v_back - v_built).abs() <= pad_back + pad_built + slop,
        "{name}: imported volume {v_back} ± {pad_back} against built {v_built} ± {pad_built}"
    );
    assert!(
        (v_built - closed_form).abs() <= 1e-9 * closed_form,
        "{name}: built volume {v_built} against the closed form {closed_form}"
    );
    settles(name, &back, text, tol);
}

/// A ball: one face, one wrap edge pole to pole.
#[test]
fn a_revolved_ball_imports_as_built() {
    let r = 0.75;
    round_trips(
        "ball",
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        (1, 1, 2),
        4.0 * PI * r * r * r / 3.0,
        true,
    );
}

/// A cone: its wall reaches the apex by one wrap edge from its rim.
#[test]
fn a_revolved_cone_imports_as_built() {
    let (r, h) = (0.5, 1.25);
    round_trips(
        "cone",
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(0.0, h), 0.0),
        ],
        (2, 2, 2),
        PI * r * r * h / 3.0,
        true,
    );
}

/// A cone under a hemisphere: one wall reaches the apex, the other the
/// pole, each by its own wrap edge from the rim they share. Its text is
/// not held: import re-foots the rim vertex a hair further each hop
/// (`work/importgate/a-free-vertexs-foot-creeps-on-every-reimport.md`).
#[test]
fn a_revolved_cone_under_a_hemisphere_imports_as_built() {
    let (r, h) = (0.5, 1.25);
    round_trips(
        "cone and hemisphere",
        vec![
            (Point2::new(0.0, -h), 0.0),
            (Point2::new(r, 0.0), (PI / 8.0).tan()),
            (Point2::new(0.0, r), 0.0),
        ],
        (2, 3, 3),
        PI * r * r * h / 3.0 + 2.0 * PI * r * r * r / 3.0,
        false,
    );
}

/// The files that state a pole or an apex otherwise settle in the same
/// form: Open CASCADE's whole sphere (an edge-free face, re-minted to a
/// ball's one wrap edge), its full cone and its cap (a seam ending at
/// the apex or the pole, adopted as written).
#[test]
fn a_foreign_pole_or_apex_settles_as_a_wrap_tip() {
    let tol = Tol::witness();
    for (path, census) in [
        ("freecad/sphere.step", (1, 1, 2)),
        ("freecad/cone_apex.step", (2, 2, 2)),
        ("rim-only-cap/seamed.step", (2, 2, 2)),
        ("occ/cap.step", (2, 2, 2)),
    ] {
        let text = std::fs::read_to_string(format!(
            "{}/tests/fixtures/{path}",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        let back = match import_step(&text, &step_import::ImportOptions::default(), tol) {
            Ok(StepImport::Solid { body, .. }) => body,
            other => panic!("{path}: {other:?}"),
        };
        assert_eq!(
            (back.faces().count(), back.edges().count(), back.vertex_points().count()),
            census,
            "{path}: the census"
        );
        settles(path, &back, true, tol);
    }
}
