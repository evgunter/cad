//! **Review probe (GUI-4 R1): open the REAL gallery through the
//! viewer's typed `Open` door and report what the tree says.**
//!
//! The exit-demo walk (`tests/assembly_walk.rs`) drives a
//! gallery-SHAPED fixture built in-crate, disclosed as a deviation
//! because the tour lives outside the workspace and a `viewer` test
//! cannot depend on it. This example closes that gap from the outside:
//! it takes a directory the tour's `gallery` mode wrote and opens every
//! `.pncad` in it (and in `assembly/`) through `SessionOp::Open`,
//! printing each document's tree rows and each instance's free-move
//! eligibility.
//!
//! An EXAMPLE, not a test: it needs an argument that only
//! `demo-tour gallery <dir>` produces, so it can never be a gate. It
//! prints; it asserts nothing beyond the open itself.
//!
//! ```text
//! cargo run --manifest-path demos/tour/Cargo.toml -- gallery /tmp/gal
//! cargo run -p viewer --example r1_gallery_probe -- /tmp/gal
//! ```

/// The drawn triangle count under the session's current display view,
/// or `None` when the scene will not build.
fn index_triangles(session: &viewer::session::DocSession) -> Option<usize> {
    let (doc, eval) = session.landed_pair()?;
    let generation = session.landed_generation()?;
    let delta = viewer::scene::DisplayTolerance::new(1.0e-3).ok()?;
    let index = viewer::pick::PickIndex::build(doc, eval, generation, delta, session.tol()).ok()?;
    Some(
        index
            .scene_for(&session.display_view())
            .ok()?
            .stats()
            .triangles,
    )
}

/// How many drawn parts carry the probe marking.
fn probe_parts(session: &viewer::session::DocSession) -> Option<usize> {
    let (doc, eval) = session.landed_pair()?;
    let generation = session.landed_generation()?;
    let delta = viewer::scene::DisplayTolerance::new(1.0e-3).ok()?;
    let index = viewer::pick::PickIndex::build(doc, eval, generation, delta, session.tol()).ok()?;
    Some(
        index
            .scene_for(&session.display_view())
            .ok()?
            .stats()
            .probe_parts,
    )
}

fn main() {
    let dir = match std::env::args().nth(1) {
        Some(d) => std::path::PathBuf::from(d),
        None => {
            eprintln!("usage: r1_gallery_probe <gallery-dir>");
            std::process::exit(2);
        }
    };
    let tol = pncad::tolerance::witness();
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for root in [dir.clone(), dir.join("assembly")] {
        let Ok(entries) = std::fs::read_dir(&root) else {
            continue;
        };
        let mut here: Vec<std::path::PathBuf> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "pncad"))
            .collect();
        here.sort();
        files.append(&mut here);
    }
    println!("gallery {} — {} document(s)", dir.display(), files.len());

    let mut opened = 0usize;
    let mut with_failed_rows = 0usize;
    for path in files {
        let mut session = viewer::session::DocSession::inline(
            pncad::document::Doc::empty_derived("r1-gallery-boot", tol),
            tol,
        );
        let outcome = session.perform(viewer::session::SessionOp::Open(path.clone()));
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if let Some(refusal) = outcome.refusal {
            println!("  {name}: OPEN REFUSED — {refusal}");
            continue;
        }
        opened += 1;
        session.pump();
        let rows = session.tree_rows();
        let instances: Vec<_> = rows
            .iter()
            .filter(|r| r.kind == "InstantiatePart")
            .collect();
        let failed: Vec<_> = rows
            .iter()
            .filter(|r| matches!(r.status, viewer::tree::RowStatus::Failed { .. }))
            .collect();
        if !failed.is_empty() {
            with_failed_rows += 1;
        }
        println!(
            "  {name}: {} row(s), {} instance(s), {} failed",
            rows.len(),
            instances.len(),
            failed.len()
        );
        for row in &failed {
            if let viewer::tree::RowStatus::Failed { message } = &row.status {
                println!("      node {} FAILED: {message}", row.id.0);
            }
        }
        // The G3 items, per instance: is the free-move probe available,
        // and does hiding the instance change the DRAWN picture?
        let baseline = index_triangles(&session);
        for row in &instances {
            match viewer::display::free_move_check(session.doc(), row.id) {
                Ok(()) => println!("      node {}: free-move ELIGIBLE", row.id.0),
                Err(fault) => println!("      node {}: free-move refused — {fault}", row.id.0),
            }
            let accepted = session
                .perform(viewer::session::SessionOp::SetInstanceHidden {
                    instance: row.id,
                    hidden: true,
                })
                .refusal
                .is_none();
            let after = index_triangles(&session);
            println!(
                "      node {}: hide accepted={accepted}, drawn triangles {:?} -> {:?}{}",
                row.id.0,
                baseline,
                after,
                if accepted && after == baseline {
                    "   <-- ACCEPTED BUT DREW NOTHING DIFFERENT"
                } else {
                    ""
                }
            );
            session.perform(viewer::session::SessionOp::SetInstanceHidden {
                instance: row.id,
                hidden: false,
            });
            // And the probe: does a displaced instance draw displaced,
            // and marked? Same question, the other G3 item.
            if session
                .perform(viewer::session::SessionOp::BeginFreeMove { instance: row.id })
                .refusal
                .is_none()
            {
                session.perform(viewer::session::SessionOp::PreviewFreeMove {
                    frame: pncad::document::Frame::translation([0.5, 0.0, 0.0]),
                });
                session.perform(viewer::session::SessionOp::CommitFreeMove);
                let marked = probe_parts(&session);
                println!(
                    "      node {}: free-move committed, scene probe_parts={marked:?}{}",
                    row.id.0,
                    if marked == Some(0) {
                        "   <-- COMMITTED BUT MARKED NOTHING"
                    } else {
                        ""
                    }
                );
                session.perform(viewer::session::SessionOp::BeginFreeMove { instance: row.id });
                session.perform(viewer::session::SessionOp::PreviewFreeMove {
                    frame: pncad::document::Frame::IDENTITY,
                });
                session.perform(viewer::session::SessionOp::CommitFreeMove);
            }
        }
        let mates = rows.iter().filter(|r| r.kind == "Mate").count();
        if mates > 0 {
            println!("      ({mates} authored mate node(s))");
        }
        // Every OTHER drawn root: can the G3 hide reach it? Hide and
        // free-move key on `InstantiatePart` ids, so a `Pattern` over an
        // instance — the flat-pack's four posts — has no per-part
        // display identity at all.
        for row in rows.iter().filter(|r| r.kind != "InstantiatePart") {
            let refused = session
                .perform(viewer::session::SessionOp::SetInstanceHidden {
                    instance: row.id,
                    hidden: true,
                })
                .refusal;
            match refused {
                None => println!("      node {} ({}): hide ACCEPTED", row.id.0, row.kind),
                Some(r) => println!("      node {} ({}): hide refused — {r}", row.id.0, row.kind),
            }
        }
    }
    println!("opened {opened} document(s); {with_failed_rows} with at least one failed row");
}
