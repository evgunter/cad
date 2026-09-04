//! DOCM-5 review lane R1 — landings the unit did not build, dumped as
//! observables so the same probe can be run on the merge base and
//! diffed. Lines tagged `R1-COUNT` are stripped for the base run.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use std::sync::Arc;

use common::asm;
use common::{ang, insert, len, len3, scl3, shape};
use pncad::document::{Doc, ProductError, ProfileProgram};
use pncad::document::gathers_on_this_thread; // R1-COUNT
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use viewer::evalseam::EvalDone;
use viewer::session::{DocSession, Landing, SessionOp};
use viewer::sketch::ProfileShape;

fn reland(session: &mut DocSession) -> u64 {
    let evaluation = Arc::clone(session.evaluation_arc().expect("landed"));
    let generation = session.landed_generation().expect("generation");
    let before = gathers_on_this_thread(); // R1-COUNT
    assert_eq!(
        session.land(EvalDone {
            generation,
            evaluation,
        }),
        Landing::Landed
    );
    return gathers_on_this_thread() - before; // R1-COUNT
    #[allow(unreachable_code)]
    0
}

fn dump(label: &str, session: &DocSession) -> String {
    let fault = match session.product_fault() {
        None => "fault=None".to_string(),
        Some(f) => format!("fault=Some({f:?}) display={f}"),
    };
    let checks = match session.checks() {
        None => "checks=None".to_string(),
        Some(r) => format!("checks=Some(findings={} skipped={:?})", r.findings.len(), r.skipped),
    };
    let badge = format!("at_rest={:?}", session.at_rest());
    let tree_failed = session
        .tree_rows()
        .iter()
        .filter(|row| matches!(row.status, viewer::tree::RowStatus::Failed { .. }))
        .count();
    format!("[{label}] {fault} | {checks} | {badge} | failed_rows={tree_failed}")
}

fn seat() -> pncad::document::Alignment {
    asm::seat_alignment(asm::SHELF_LENGTH / 2.0, None)
}

/// Every case: the dump, and (on the head) the gather count of one
/// re-landing.
#[test]
fn r1_landings_the_unit_did_not_build() {
    let tol = Tol::witness();
    let mut out = Vec::new();

    // 1. The gallery ring — a saved multi-node part document.
    {
        let doc = pncad::document::load(&common::gallery_ring_at(tol), tol)
            .expect("loads")
            .snapshot;
        let mut session = DocSession::inline(doc, tol);
        assert_eq!(session.pump(), vec![Landing::Landed]);
        let n = reland(&mut session);
        out.push(format!("{} gathers={n}", dump("gallery-ring", &session)));
        assert_eq!(n, 1, "gallery ring"); // R1-COUNT
    }

    // 2. An assembly with a seated Rest mate: certified, minted 1.
    {
        let bench = asm::bench("r1-rest", tol);
        let mut session = asm::open_bench(&bench, tol);
        session.perform(SessionOp::AddMate {
            a: asm::in_part(bench.post_a, &bench.post_top),
            b: asm::in_part(bench.shelf_i, &bench.shelf_bottom),
            class: ContactClass::Rest,
            alignment: seat(),
        });
        session.pump();
        let n = reland(&mut session);
        out.push(format!("{} gathers={n}", dump("asm-rest-mate", &session)));
        assert_eq!(n, 1, "rest mate"); // R1-COUNT
    }

    // 3. An assembly with a Tangent mate: a mint refusal at the gate.
    {
        let bench = asm::bench("r1-tangent", tol);
        let mut session = asm::open_bench(&bench, tol);
        session.perform(SessionOp::AddMate {
            a: asm::in_part(bench.post_b, &bench.post_top),
            b: asm::in_part(bench.shelf_i, &bench.shelf_bottom),
            class: ContactClass::Tangent,
            alignment: seat(),
        });
        session.pump();
        let n = reland(&mut session);
        out.push(format!("{} gathers={n}", dump("asm-tangent-mint-refusal", &session)));
        assert_eq!(n, 1, "tangent"); // R1-COUNT
    }

    // 4. An assembly whose part document is missing: the gather refuses
    //    (a failed root) on an ASSEMBLY-SHAPED document.
    {
        let bench = asm::bench("r1-missing", tol);
        std::fs::remove_file(bench.dir.join(format!("{}.pncad", bench.post.id))).expect("rm");
        let mut session = asm::open_bench(&bench, tol);
        let n = reland(&mut session);
        out.push(format!("{} gathers={n}", dump("asm-gather-refusal", &session)));
        assert_eq!(n, 1, "missing part"); // R1-COUNT
    }

    // 5. A part document whose two roots collide in the name table:
    //    two transforms of one extrude.
    {
        let mut session = DocSession::inline(Doc::<ProfileProgram>::empty_derived("r1-naming", tol), tol);
        let plane = common::xy_frame_in(&mut session);
        let profile = insert(
            &mut session,
            SessionOp::AddProfile {
                plane,
                loops: vec![shape(&ProfileShape::Rectangle {
                    width: 0.01,
                    height: 0.01,
                })],
            },
        );
        let extrude = insert(
            &mut session,
            SessionOp::AddExtrude {
                profile,
                distance: len(0.01),
            },
        );
        for dx in [0.03, 0.06] {
            insert(
                &mut session,
                SessionOp::AddTransform {
                    input: extrude,
                    translation: len3([dx, 0.0, 0.0]),
                    rotation_axis: scl3([0.0, 0.0, 1.0]),
                    rotation_angle: ang(0.0),
                },
            );
        }
        session.pump();
        let n = reland(&mut session);
        out.push(format!("{} gathers={n}", dump("part-naming-collision", &session)));
        assert_eq!(n, 1, "naming"); // R1-COUNT
        assert!(matches!(session.product_fault(), Some(ProductError::Naming { .. })), "{:?}", session.product_fault());
    }

    // 6. An assembly with a dangling mate reference (Reference refusal).
    {
        let bench = asm::bench("r1-dangling", tol);
        let mut session = asm::open_bench(&bench, tol);
        session.perform(SessionOp::AddMate {
            a: asm::in_part(bench.post_a, &bench.shelf_bottom),
            b: asm::in_part(bench.shelf_i, &bench.shelf_bottom),
            class: ContactClass::Rest,
            alignment: seat(),
        });
        session.pump();
        let n = reland(&mut session);
        out.push(format!("{} gathers={n}", dump("asm-dangling-reference", &session)));
        assert_eq!(n, 1, "dangling"); // R1-COUNT
    }

    let text = out.join("\n");
    println!("R1-DUMP-BEGIN\n{text}\nR1-DUMP-END");
    if let Ok(path) = std::env::var("R1_DUMP") {
        std::fs::write(path, format!("{text}\n")).expect("write dump");
    }
}
