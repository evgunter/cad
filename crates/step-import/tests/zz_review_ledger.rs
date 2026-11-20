//! Review probe (PR 7172): a per-fixture ledger of dispositions and surface
//! promotions over every committed STEP fixture, written to
//! `$REVIEW_LEDGER_OUT`. Not a shipped row.
#![allow(clippy::unwrap_used, clippy::panic)]

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use geom_core::Tol;
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk(&p, out);
        } else {
            let s = p.to_string_lossy().to_lowercase();
            if s.ends_with(".step") || s.ends_with(".stp") {
                out.push(p);
            }
        }
    }
}

#[test]
#[ignore = "review probe"]
fn review_ledger() {
    let Ok(out) = std::env::var("REVIEW_LEDGER_OUT") else {
        return;
    };
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    walk(&root.join("tests/fixtures"), &mut files);
    walk(&root.join("../step-export/tests/fixtures"), &mut files);
    let eps_rows: Vec<Option<f64>> = match std::env::var("REVIEW_EPS_IN").as_deref() {
        Ok("all") => vec![None, Some(1e-6), Some(1e-12)],
        _ => vec![None],
    };
    let mut text_out = String::new();
    for f in &files {
        let rel = f.strip_prefix(&root).unwrap_or(f).to_string_lossy();
        let text = std::fs::read_to_string(f).unwrap();
        for eps_in in &eps_rows {
            let options = ImportOptions {
                eps_in: *eps_in,
                ..ImportOptions::default()
            };
            let t0 = std::time::Instant::now();
            let res = std::panic::catch_unwind(|| import_step(&text, &options, Tol::witness()));
            let dt = t0.elapsed().as_secs_f64();
            let line = match res {
                Err(_) => "PANIC".to_string(),
                Ok(Ok(StepImport::Solid {
                    body,
                    normalizations,
                    ..
                })) => {
                    let mut promos: Vec<String> = normalizations
                        .iter()
                        .filter_map(|n| match n.kind {
                            NormalizationKind::SurfacePromotion { to, .. } => {
                                Some(format!("{}:{to:?}", n.face))
                            }
                            _ => None,
                        })
                        .collect();
                    promos.sort();
                    format!(
                        "SOLID ({},{},{},{},{}) promos[{}] {}",
                        body.solids().count(),
                        body.shells().count(),
                        body.faces().count(),
                        body.edges().count(),
                        body.vertices().count(),
                        promos.len(),
                        promos.join(" ")
                    )
                }
                Ok(Ok(_)) => "WIREFRAME".to_string(),
                Ok(Err(e)) => {
                    let s = format!("{e:?}");
                    format!("ERR {}", s.chars().take(160).collect::<String>())
                }
            };
            writeln!(text_out, "{rel}\t{eps_in:?}\t{line}\t[{dt:.1}s]").unwrap();
            eprintln!("{rel}\t{eps_in:?}\t{line}\t[{dt:.1}s]");
        }
    }
    std::fs::write(out, text_out).unwrap();
}
