//! **A carrier the kernel builds is reported where it is not the printed
//! one** (`[import]`, D7 step 2; `crates/step-import/src/build.rs`): an
//! edge's carrier is constructed from its description, and the ledger's
//! `CarrierBuilt` names every edge whose built carrier differs from the
//! printed one as a value, with the printed carrier's farthest sample
//! from it. A printed line standing off the two planes it is the meeting
//! of, within the file's band, is the row: the built line is the planes'
//! own, and the move is the stand-off.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

/// The cube fixture with edge `#14`'s printed line (on planes `x = 0` and
/// `z = 1`, through `#10 = (0, 1, 1)`) moved off both planes by `delta`.
fn moved_line(delta: f64) -> String {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../step-export/tests/fixtures/cube.step"
    ))
    .expect("the cube fixture reads");
    let at = "#10 = CARTESIAN_POINT('', (0.0, 1.0, 1.0));";
    assert!(src.contains(at), "edge #14's line passes through #10");
    src.replace(
        at,
        &format!(
            "#10 = CARTESIAN_POINT('', ({:?}, 1.0, {:?}));",
            delta,
            1.0 + delta
        ),
    )
}

/// The `CarrierBuilt` moves the ledger reports for edge `id`.
fn carrier_moves(text: &str, id: u64) -> Vec<f64> {
    let Ok(StepImport::Solid { normalizations, .. }) =
        import_step(text, &ImportOptions::default(), Tol::witness())
    else {
        panic!("the cube imports");
    };
    normalizations
        .iter()
        .filter(|n| n.face == id)
        .filter_map(|n| match n.kind {
            NormalizationKind::CarrierBuilt { moved_by } => Some(moved_by),
            _ => None,
        })
        .collect()
}

#[test]
fn a_line_printed_off_its_planes_is_reported_at_its_stand_off() {
    let delta = 2e-10;
    let moves = carrier_moves(&moved_line(delta), 14);
    let off = delta * core::f64::consts::SQRT_2;
    let [moved] = moves.as_slice() else {
        panic!("the ledger reports edge #14's built carrier once: {moves:?}");
    };
    assert!(
        (moved - off).abs() <= 1e-6 * off,
        "the move is the printed line's stand-off from the planes' line: {moved:e} vs {off:e}"
    );
}

#[test]
fn a_line_printed_on_its_planes_moves_by_nothing() {
    // The built line is anchored at its own point nearest the origin, so
    // it may differ from the printed one as a value; it is the same locus.
    let text = moved_line(0.0);
    for moved in carrier_moves(&text, 14) {
        assert_eq!(
            moved, 0.0,
            "the same line under another anchor moves nothing"
        );
    }
}
