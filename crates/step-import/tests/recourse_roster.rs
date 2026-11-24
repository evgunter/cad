//! **The roster: every site where step-import decides a predicate name
//! is one the census can read**, on the harness every roster shares
//! (`test_utils::source::roster_violations`): no spelling it cannot
//! read, no file it does not walk, every const site a declared carrier,
//! and no forwarder that hands its parameter to a decide site without
//! being declared a carrier.
//!
//! The `STEP_*` consts the band funnels are handed are declared below,
//! so the tree-wide coverage row
//! (`test-utils/tests/predicate_name_coverage.rs`) resolves a reference
//! to one against step-import's census.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source::{NameCarrier, crate_dir, predicate_census, roster_violations};

/// The `const`s step-import's names are spelled in, and the funnels that
/// hand on the name their caller gives them.
/// An entry that matches nothing in `src` now is not a red: it reads
/// nothing and excuses nothing, and it goes stale silently.
const CARRIERS: &[NameCarrier] = &[
    NameCarrier::Call("read"),
    NameCarrier::Call("agree"),
    NameCarrier::Call("not_apart"),
    NameCarrier::Call("on_as"),
    NameCarrier::Call("agree_at"),
    NameCarrier::Const("STEP_AT_DOMAIN_END"),
    NameCarrier::Const("STEP_BAND_RIM_COAXIAL"),
    NameCarrier::Const("STEP_COLLAPSED_SIDE"),
    NameCarrier::Const("STEP_BAND_SEAM_VERTEX"),
    NameCarrier::Const("STEP_BUILT_SENSE"),
    NameCarrier::Const("STEP_EDGE_ON_DESCRIPTION"),
    NameCarrier::Const("STEP_ISO_CIRCLE"),
    NameCarrier::Const("STEP_ISO_ENDS"),
    NameCarrier::Const("STEP_ISO_IN_PLANE"),
    NameCarrier::Const("STEP_ISO_STRAIGHT"),
    NameCarrier::Const("STEP_LINE_DIRECTION_UNIT"),
    NameCarrier::Const("STEP_ONE_RECORD"),
    NameCarrier::Const("STEP_ONE_RECORD_FACING"),
    NameCarrier::Const("STEP_PLACEMENT_AXES"),
    NameCarrier::Const("STEP_PLACEMENT_IDENTITY"),
    NameCarrier::Const("STEP_PLANES_CROSS"),
    NameCarrier::Const("STEP_POSITIVE_LENGTH"),
    NameCarrier::Const("STEP_RECOGNIZED_CLOSURE"),
    NameCarrier::Const("STEP_RECOGNIZED_CONDITIONING"),
    NameCarrier::Const("STEP_RECOGNIZED_PLANE_FRAME"),
    NameCarrier::Const("STEP_RECOGNIZED_RESIDUAL"),
    NameCarrier::Const("STEP_RECOGNIZED_REVOLUTION_FRAME"),
    NameCarrier::Const("STEP_RIM_LINE_FRAME"),
    NameCarrier::Const("STEP_RING_CONTAINMENT"),
    NameCarrier::Const("STEP_SINGULARITY_POINT"),
    NameCarrier::Const("STEP_SNAP_FRAME"),
    NameCarrier::Const("STEP_SNAP_MOVE"),
    NameCarrier::Const("STEP_SNAP_PARALLEL"),
    NameCarrier::Const("STEP_SNAP_RELATION"),
    NameCarrier::Const("STEP_SNAP_SIDE"),
    NameCarrier::Const("STEP_STRAIGHT_MAP"),
    NameCarrier::Const("STEP_TRIM_COVERED"),
    NameCarrier::Const("STEP_VERTEX_BUILT"),
    NameCarrier::Const("STEP_VERTEX_LOOP_ON_SPHERE"),
];

/// **The census meets the harness every roster shares**, and it reads
/// names, so the row does not pass over an empty walk.
#[test]
fn every_decided_name_is_read() {
    let src = crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let census = predicate_census(&src, CARRIERS);
    assert!(
        !census.names.is_empty(),
        "the census read no name in {}",
        src.display()
    );
    let violations = roster_violations(&src, &census, CARRIERS, None);
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
