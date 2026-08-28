//! **The REAL gallery, opened through the viewer's typed doors — and
//! asserted on** (GUI-4 acceptance, the render-lane half).
//!
//! The exit-demo walk (`tests/assembly_walk.rs`) drives a
//! gallery-SHAPED fixture built in-crate, because the tour lives
//! outside the workspace and a `viewer` test cannot depend on it; its
//! module doc carries the argument. This example closes the gap from
//! the outside: it takes a directory the tour's `gallery` mode wrote,
//! opens every `.pncad` in it (and in `assembly/`) through
//! `SessionOp::Open`, and CHECKS, per document:
//!
//! - the open succeeds and every tree row evaluates `Ok` (the
//!   resolver, on the real store);
//! - per instance, hide either takes visible effect (the drawn
//!   triangle count moves) or refuses typed — never
//!   accepted-and-inert (the M1 class, measured on exactly this
//!   directory when it shipped);
//! - per eligible instance, a committed probe marks at least one
//!   drawn part distinct — same rule, other op.
//!
//! Any violation prints and the process exits nonzero, so the
//! render-lane step that runs it is a real verdict. It stays an
//! EXAMPLE (not a test) because its input exists only where
//! `demo-tour gallery <dir>` ran first:
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
    // An empty directory is a broken invocation, not a green run.
    let mut violations: Vec<String> = if files.is_empty() {
        vec!["no .pncad documents found — was the gallery generated?".to_owned()]
    } else {
        Vec::new()
    };

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
            violations.push(format!("{name}: OPEN REFUSED — {refusal}"));
            continue;
        }
        session.pump();
        let rows = session.tree_rows();
        let instances: Vec<_> = rows
            .iter()
            .filter(|r| r.kind == "InstantiatePart")
            .collect();
        let failed: Vec<_> = rows
            .iter()
            .filter(|r| !matches!(r.status, viewer::tree::RowStatus::Ok))
            .collect();
        println!(
            "  {name}: {} row(s), {} instance(s), {} not-ok",
            rows.len(),
            instances.len(),
            failed.len()
        );
        for row in &failed {
            violations.push(format!(
                "{name}: node {} is {:?} — the real gallery must resolve clean",
                row.id.0, row.status
            ));
        }
        // The G3 items, per instance: hide takes effect or refuses
        // typed; a committed probe marks something. Never a silent
        // no-op. (The baseline tessellation is paid only for
        // documents that HAVE instances — the part documents' cost
        // here is the open and the tree.)
        if instances.is_empty() {
            continue;
        }
        let baseline = index_triangles(&session);
        for row in &instances {
            let hide = session
                .perform(viewer::session::SessionOp::SetInstanceHidden {
                    instance: row.id,
                    hidden: true,
                })
                .refusal;
            match hide {
                None => {
                    let after = index_triangles(&session);
                    println!(
                        "      node {}: hide accepted, drawn triangles {baseline:?} -> {after:?}",
                        row.id.0
                    );
                    if after == baseline {
                        violations.push(format!(
                            "{name}: node {} hide ACCEPTED BUT DREW NOTHING DIFFERENT",
                            row.id.0
                        ));
                    }
                    session.perform(viewer::session::SessionOp::SetInstanceHidden {
                        instance: row.id,
                        hidden: false,
                    });
                }
                Some(refusal) => {
                    println!("      node {}: hide refused typed — {refusal}", row.id.0);
                }
            }
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
                    "      node {}: free-move committed, scene probe_parts={marked:?}",
                    row.id.0
                );
                if marked == Some(0) {
                    violations.push(format!(
                        "{name}: node {} probe COMMITTED BUT MARKED NOTHING",
                        row.id.0
                    ));
                }
                // Discard the probe so the next instance's baseline is
                // clean.
                session.perform(viewer::session::SessionOp::BeginFreeMove { instance: row.id });
                session.perform(viewer::session::SessionOp::PreviewFreeMove {
                    frame: pncad::document::Frame::IDENTITY,
                });
                session.perform(viewer::session::SessionOp::CommitFreeMove);
            } else {
                println!("      node {}: free-move refused typed", row.id.0);
            }
        }
        let mates = rows.iter().filter(|r| r.kind == "Mate").count();
        if mates > 0 {
            println!("      ({mates} authored mate node(s))");
        }
    }
    if violations.is_empty() {
        println!("gallery probe: OK");
    } else {
        for violation in &violations {
            eprintln!("VIOLATION: {violation}");
        }
        eprintln!("gallery probe: {} violation(s)", violations.len());
        std::process::exit(1);
    }
}
