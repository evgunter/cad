//! **#207's export leg: the first curved-path `sweep_body` to reach
//! the wire.**
//!
//! `sweep_body` with a curved path had zero successful callers anywhere
//! in the tree until the skin fit stopped synthesizing a weight channel
//! for integral input (see `sweep/tests/m7_skin_integral.rs` for the
//! kernel-side pins and `sweep::skin::skin`'s docs for the argument).
//! This suite carries the other half of that pin: the body exports, and
//! both independent Part 21 oracles reconstruct it.
//!
//! It also pins the CONSEQUENCE of the fix on the wire, which is the
//! whole point of it. `crate::writer` emits the plain
//! `B_SPLINE_SURFACE_WITH_KNOTS` record only when every weight is
//! exactly `1.0`, and the `RATIONAL_B_SPLINE_SURFACE` complex instance
//! otherwise. Before the fix a polyline-profile elbow would have gone
//! out as a rational surface — a lie about the geometry's kind that no
//! importer could have unpicked, on a body that in fact never got that
//! far because assembly refused first.
//!
//! **The elbow IS in the committed fixture corpus** as of the #210
//! fold: `common::swept_elbow` is the one construction, this suite's
//! oracle rows and `fixtures/swept_elbow.step`'s golden bytes both hang
//! off it, and `fixtures/swept_elbow.expect` carries the FreeCAD/OCC
//! reading plus the kernel's certified volume. (It was
//! corpus-*adjacent* when #210 landed, with its own duplicate
//! construction; that duplicate is gone.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use step_export::{StepOptions, step_string};

// The two corpus bodies this suite exports. `swept_elbow` is the
// quarter-torus elbow — a square profile swept along a 90° arc in the
// world YZ plane, 9 stations at v-degree 3; `nonuniform_loft` is
// `loft_prism`'s sections at spacing 1 : 2. Constants and derivations
// live on the builders in `common`, and in
// `sweep/tests/m7_skin_integral.rs` (the Pappus bracket).
use common::{nonuniform_loft, swept_elbow};
use geom_core::Tol;

fn export(body: &topo::Body<f64>, name: &str) -> String {
    let options = StepOptions {
        product_name: name.to_owned(),
        uncertainty_m: Some(1e-9),
        ..StepOptions::default()
    };
    step_string(body, &options, Tol::witness()).expect("the body exports")
}

#[test]
fn the_swept_elbow_exports_and_both_oracles_reconstruct_it() {
    let body = swept_elbow();
    // Four walls, four wall–wall seams, two planar caps; the caps
    // contribute the four rim edges each shares with its walls.
    let census = common::census(&body);
    assert_eq!(census, (6, 12, 8), "kernel census (a topological box)");
    let text = export(&body, "swept_elbow");

    let exchange = ruststep::parser::parse(&text).expect("ruststep Part 21 parse");
    assert_eq!(exchange.data.len(), 1, "one data section");

    let table = truck_stepio::r#in::Table::from_step(&text).expect("truck table parse");
    let (_, shell) = table.shell.iter().next().expect("one shell");
    let cs = table
        .to_compressed_shell(shell)
        .expect("truck shell reconstruction");
    assert_eq!(
        (cs.faces.len(), cs.edges.len(), cs.vertices.len()),
        census,
        "the independent importer sees the kernel's census"
    );
}

/// **The wire says non-rational, and says it four times.** One plain
/// `B_SPLINE_SURFACE_WITH_KNOTS` per wall, and no
/// `RATIONAL_B_SPLINE_SURFACE` anywhere — the exported KIND is the
/// input's kind. This is the byte-level statement of #207's fix.
#[test]
fn the_swept_elbows_walls_go_out_non_rational() {
    let text = export(&swept_elbow(), "swept_elbow");
    assert_eq!(
        text.matches("B_SPLINE_SURFACE_WITH_KNOTS").count(),
        4,
        "one spline-surface record per wall"
    );
    assert_eq!(
        text.matches("RATIONAL_B_SPLINE_SURFACE").count(),
        0,
        "an integral sweep must not export a rational surface"
    );
    assert_eq!(
        text.matches("RATIONAL_B_SPLINE_CURVE").count(),
        0,
        "nor a rational seam carrier"
    );
}

/// The elbow's fold-mate: [`nonuniform_loft`], `loft_prism`'s minimal
/// pair. It carries the SAME statement one lane over — the drift #207
/// removed was driven by the section PARAMETERIZATION, so a
/// straight-path loft at spacing 1 : 2 was refused for exactly the
/// reason a curved-path sweep was. Both oracles reconstruct it, and its
/// walls go out plain.
#[test]
fn the_nonuniform_loft_exports_non_rational_and_both_oracles_reconstruct_it() {
    let body = nonuniform_loft();
    let census = common::census(&body);
    assert_eq!(census, (6, 12, 8), "kernel census (loft_prism's topology)");
    let text = export(&body, "nonuniform_loft");

    let exchange = ruststep::parser::parse(&text).expect("ruststep Part 21 parse");
    assert_eq!(exchange.data.len(), 1, "one data section");

    let table = truck_stepio::r#in::Table::from_step(&text).expect("truck table parse");
    let (_, shell) = table.shell.iter().next().expect("one shell");
    let cs = table
        .to_compressed_shell(shell)
        .expect("truck shell reconstruction");
    assert_eq!(
        (cs.faces.len(), cs.edges.len(), cs.vertices.len()),
        census,
        "the independent importer sees the kernel's census"
    );

    assert_eq!(
        text.matches("B_SPLINE_SURFACE_WITH_KNOTS").count(),
        4,
        "one spline-surface record per wall"
    );
    assert_eq!(
        text.matches("RATIONAL_B_SPLINE_SURFACE").count(),
        0,
        "a non-uniformly spaced integral loft must not export a rational surface"
    );
    assert_eq!(
        text.matches("RATIONAL_B_SPLINE_CURVE").count(),
        0,
        "nor a rational seam carrier"
    );
}
