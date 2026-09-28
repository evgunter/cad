//! **D9 on R2's own document: the rayon schedule drives bit-identically
//! to the sequential one** — the same serialization, content key and
//! decision counts.
//!
//! This row was `m10_7_r2_probes_interval`'s, and lives apart for one
//! reason: `gated_to!` gates a whole file, and this row's subject is the
//! parallel schedule alone. It drives R2's filleted bracket to 256
//! leaves with the symbolic tier on, which is minutes of CI per ε leg,
//! and what it can catch is state leaking between rayon workers — a
//! per-leaf session, a memo, a K-funnel frame. So it runs when a file
//! that holds such state, or that a leaf runs under rayon, moves; the
//! nightly runs it ungated. The set errs wide on purpose: every source
//! file in the drive's reach that uses rayon or a `thread_local!`, the
//! evaluation a leaf replays, the fixture, and the lockfile (a rayon
//! bump).
//!
//! Two drives, not three: the sequential drive against the parallel
//! one. A sequential repeat added nothing the pair does not already
//! catch — a schedule-dependent leak shows as the two disagreeing, and
//! a drive nondeterministic by itself would have to be so identically
//! on both schedules to pass.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/drive.rs",
    "crates/editor-core/src/eval/",
    "crates/geom-core/src/sym.rs",
    "crates/geom-core/src/sym/",
    "crates/geom-core/src/k_stats.rs",
    "crates/topo/src/props.rs",
    "crates/topo/src/merge_faces.rs",
    "crates/topo/src/surgery.rs",
    "crates/mesh/src/tessellate.rs",
    "crates/mesh/src/budget.rs",
    "crates/editor-core/tests/m10_7_r2_probes_interval.rs",
    "Cargo.lock",
];

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use geom_core::Tol;

use crate::m10_7_r2_probes_interval::bracket;

#[test]
fn r2_the_drive_is_bit_identical_under_the_rayon_schedule() {
    let tol = Tol::witness();
    let (doc, _, _) = bracket(1.0e-3, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = |parallel: bool| {
        let v = drive(
            &doc,
            &analyzed,
            &DriveConfig {
                parallel,
                max_leaves: 256,
                ..DriveConfig::default()
            },
            tol,
        )
        .expect("the bracket's nominal builds");
        (
            v.serialize(),
            format!("{:?}", v.content_key()),
            v.decisions(),
        )
    };
    let sequential = run(false);
    let parallel = run(true);
    assert_eq!(
        sequential.0, parallel.0,
        "the rayon schedule serialized differently"
    );
    assert_eq!(
        sequential.1, parallel.1,
        "the rayon schedule keyed differently"
    );
    assert_eq!(
        sequential.2, parallel.2,
        "the rayon schedule counted differently — a per-leaf session leaked"
    );
}
