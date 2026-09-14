//! **SYM-7 R2's base-vs-head dump** — drives two freezing documents at
//! the driver's default dials and writes each verdict's serialization
//! to `$CAD_R2_OUT/<doc>-<schedule>.txt`. Written against the API the
//! merge base already has, so the same file runs on the base and on
//! the head and the two directories can be diffed.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-core/src/sym.rs",
    "crates/geom-core/src/sym/",
    "crates/editor-core/src/drive.rs",
];

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use geom_core::Tol;

use crate::m10_7_plate::plate;
use crate::m10_derived_frame_tilted_interval::boss_on_tilted;

#[test]
#[ignore = "evidence-only: R2's base-vs-head serialization dump"]
fn r2_dump_serializations() {
    let out = std::env::var("CAD_R2_OUT").expect("CAD_R2_OUT names the output directory");
    std::fs::create_dir_all(&out).unwrap();
    let tol = Tol::witness();
    for (label, doc) in [
        ("tilted-derived", boss_on_tilted(1.0e-3, true)),
        ("plate", plate(5.0e-5, 1.0e-5, tol).0),
    ] {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        for parallel in [false, true] {
            let cfg = DriveConfig {
                max_leaves: 48,
                parallel,
                ..DriveConfig::default()
            };
            let v = drive(&doc, &analyzed, &cfg, tol).unwrap();
            let sched = if parallel { "par" } else { "seq" };
            let mut s = v.serialize();
            // The per-leaf decision counts, `frozen` included, beside
            // the serialization — the base sums them into the drive's
            // column and the head does not, so both are on record.
            for (i, l) in v.certified().iter().enumerate() {
                s.push_str(&format!("\ncertified-decisions {i} {:?}", l.decisions));
            }
            for (i, l) in v.refused().iter().enumerate() {
                s.push_str(&format!("\nrefused-decisions {i} {:?}", l.decisions));
            }
            std::fs::write(format!("{out}/{label}-{sched}.txt"), s).unwrap();
            println!(
                "{label} {sched}: receipt {:?} decisions {:?}",
                v.receipt(),
                v.decisions()
            );
        }
    }
}
