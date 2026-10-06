//! LANE-PRIVATE PROBE (zip-chord, branch `zip/chord-probe`; never for
//! main). `#[ignore]`d; asserts nothing. The boss-flush scene of
//! `work/zip/a-boss-flush-with-a-block-edge-refuses-its-declared-union.md`
//! through the viewer's op vocabulary: a block extruded from a centred
//! rectangle, a `Datum::FaceFrame` off its top cap (spin 0), the boss
//! path on it, extruded, and the union — undeclared, the resting cap
//! pair declared, and the cap pair plus the flush walls' continuation.
//! Run with `ZIP_CHORD_PROBE=<file>` to get the kernel's chord lines:
//!
//! ```text
//! ZIP_CHORD_PROBE=/path/to/log cargo test -p editor-core --test all \
//!     zip_chord_boss_probe -- --ignored --nocapture
//! ```

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ang, desc, fname, insert, len};
use editor_core::{
    BooleanCoincidence, BooleanOp, CapEnd, ExtrudeSide, Node, ProfileDoc, RoleSeg, SitedRef,
};
use geom_core::Tol;

#[test]
#[ignore = "zip-chord probe; asserts nothing"]
fn probe_boss_flush_through_the_op_vocabulary() {
    for s in [1e-3, 1.0] {
        let doc = ProfileDoc::empty_derived("zip_chord_boss", Tol::witness());
        let (doc, blk) = block(
            doc,
            (-20.0 * s, 20.0 * s),
            (-10.0 * s, 10.0 * s),
            0.0,
            10.0 * s,
        );
        let (doc, top) = insert(
            doc,
            Node::Datum(editor_core::Datum::FaceFrame {
                at: blk,
                face: fname(blk, RoleSeg::Cap(CapEnd::End)),
                spin: ang(0.0),
            }),
        );
        let (doc, prof) = insert(
            doc,
            Node::Profile(desc(
                top,
                vec![vec![
                    (10.0 * s, -5.0 * s),
                    (20.0 * s, -5.0 * s),
                    (20.0 * s, 5.0 * s),
                    (10.0 * s, 5.0 * s),
                ]],
            )),
        );
        let (doc, boss) = insert(
            doc,
            Node::Extrude {
                profile: prof,
                distance: len(4.0 * s),
                side: ExtrudeSide::Along,
            },
        );
        let cap_pair = (
            (
                SitedRef::at_mint(fname(blk, RoleSeg::Cap(CapEnd::End))),
                SitedRef::at_mint(fname(boss, RoleSeg::Cap(CapEnd::Start))),
            ),
            BooleanCoincidence::REST,
        );
        // The offer: every refusal's finding declared in turn, the way
        // the boolean tool's offer accepts them one at a time.
        let mut offered: Vec<editor_core::DeclaredPair> = Vec::new();
        for round in 0..4 {
            let (doc, u) = insert(
                doc.clone(),
                Node::Boolean {
                    op: BooleanOp::Union,
                    a: blk,
                    b: boss,
                    declare: offered.clone(),
                },
            );
            let ev = run(&doc);
            let f = failure(&ev, u);
            let line = format!(
                "boss op-vocabulary scale {s}, offer round {round} ({} declared): {:?}",
                offered.len(),
                f.map(|e| format!("{e:?}").chars().take(400).collect::<String>())
            );
            eprintln!("{line}");
            if let Ok(path) = std::env::var("ZIP_CHORD_PROBE") {
                use std::io::Write;
                let mut fh = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .unwrap();
                writeln!(fh, "[editor-core boss] {line}").unwrap();
            }
            match f {
                Some(editor_core::NodeErrorKind::UndeclaredCoincidence { finding, .. }) => {
                    offered.push((finding.pair.clone(), finding.class));
                }
                _ => break,
            }
        }
        for (what, declare) in [("undeclared", vec![]), ("cap pair", vec![cap_pair.clone()])] {
            let (doc, u) = insert(
                doc.clone(),
                Node::Boolean {
                    op: BooleanOp::Union,
                    a: blk,
                    b: boss,
                    declare,
                },
            );
            let ev = run(&doc);
            let line = format!(
                "boss op-vocabulary scale {s}, {what}: {:?}",
                failure(&ev, u)
            );
            eprintln!("{line}");
            if let Ok(path) = std::env::var("ZIP_CHORD_PROBE") {
                use std::io::Write;
                let mut f = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .unwrap();
                writeln!(f, "[editor-core boss] {line}").unwrap();
            }
        }
    }
}
