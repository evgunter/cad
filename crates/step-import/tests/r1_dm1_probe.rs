//! **R1 review probe (PR #264, C4), retired into its successor (M8
//! instancing).**
//!
//! What it pinned: dm1's refusal was exactly the M7-4 Leg D instancing
//! gate, fired at the second differing transform (#186, bolt_4), AFTER
//! the whole shape resolved — the executable form of PR #264's
//! "geometry distance retired" claim.
//!
//! That gate is gone. `REPRESENTATION_RELATIONSHIP` says which
//! component each `ITEM_DEFINED_TRANSFORMATION` places (`rep_1`), so
//! dm1's 7 occurrences of its 3 breps materialize as 7 placed
//! instances instead of refusing at the second map. This file keeps
//! the same subject and asserts the successor claims:
//!
//! 1. **the gate is gone, not moved** — no refusal of dm1 mentions
//!    assembly instancing, and none names #186;
//! 2. **the seven instances are read as seven** — the file's own
//!    occurrence structure, counted from the text, is what the resolver
//!    walks (3 component representations, 7 relationships, each naming
//!    one of them);
//! 3. **the placement layer is now BEHIND the geometry** — dm1's
//!    remaining refusal is reachable only once every instance's frame
//!    was read and applied. Since #327 (stage-1 CURVE recognition)
//!    that refusal has moved AGAIN, and past the whole D7 ladder: the
//!    file's rational-quadratic rim carriers are recognized as circles
//!    and promoted, every edge of every instance adopts, every pcurve
//!    mints and certifies, and the first thing that refuses is the
//!    SHARED AT-REST GATE — `VolumeUncomputable` /
//!    `QuadratureBudget` on the rational cylinder wall, i.e. the
//!    banked rational-patch-flux lane this crate's own docs name
//!    ("their rational walls have no volume quadrature yet"). This
//!    probe pins where the frontier actually is, so a claim that it
//!    moved has to be executable too.
//!
//! Per-instance placement CORRECTNESS (each frame on its own component
//! and no other) is pinned where a file that IMPORTS can carry it:
//! `freecad.rs::refusals_survive_the_dialect_relaxations` (d), on
//! planted mutations of `twobody_importexport`'s real transforms.
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
fn dm1_no_longer_refuses_at_the_instancing_gate() {
    let text = dm1();

    // (2) The file really does state seven occurrences of three
    // component representations — counted from the text, so the claim
    // does not rest on the reader agreeing with itself.
    let occurrences = text.matches("NEXT_ASSEMBLY_USAGE_OCCURRENCE").count();
    let transforms = text.matches("ITEM_DEFINED_TRANSFORMATION").count();
    let breps = text.matches("MANIFOLD_SOLID_BREP").count();
    assert_eq!(occurrences, 7, "l-bracket + 3 bolts + 3 nuts");
    assert_eq!(transforms, 7, "one per occurrence");
    assert_eq!(breps, 3, "three component representations, seven instances");

    // (1) and (3): the disposition.
    match import_step(&text, &ImportOptions::default()) {
        Err(StepImportError::Structure { id, what }) => {
            panic!("the assembly layer must not refuse dm1 any more: #{id} {what}")
        }
        Err(StepImportError::Adoption { id, attempts }) => panic!(
            "the D7 ladder must not refuse dm1 any more (#327 retired edge #685): \
             #{id}, {} candidate(s)",
            attempts.len()
        ),
        Err(e @ StepImportError::TierInvalid { .. }) => {
            let shown = e.to_string();
            assert!(
                shown.contains("the certified quadrature enclosure stalled at"),
                "the frontier is now the rational-patch-flux lane, not the ladder: {shown}"
            );
        }
        other => panic!("dm1's refusal has moved out of the at-rest gate; got {other:?}"),
    }
}
