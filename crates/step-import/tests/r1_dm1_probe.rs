//! **R1 review probe (PR #264, C4)** — review branch only: dm1's
//! refusal is exactly the PRE-EXISTING M7-4 Leg D instancing gate,
//! fired at the second differing transform (#186, bolt_4), i.e. AFTER
//! the whole shape resolved (resolve_shape runs strictly before
//! resolve_assembly_placement) — which is what makes the PR's
//! "geometry distance retired" claim executable.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;
use step_import::{ImportOptions, StepImportError, import_step};

fn dm1() -> String {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "wild",
        "stepcode",
        "dm1-id-214.stp",
    ]
    .iter()
    .collect();
    std::fs::read_to_string(&path).unwrap()
}

#[test]
fn dm1_refuses_at_the_second_differing_transform_after_full_resolution() {
    match import_step(&dm1(), &ImportOptions::default()) {
        Err(StepImportError::Structure { id, what }) => {
            assert_eq!(id, 186, "the second differing transform (bolt_4)");
            assert!(what.contains("assembly instancing"), "{what}");
        }
        other => panic!("dm1 must refuse at the instancing gate: {other:?}"),
    }
}
