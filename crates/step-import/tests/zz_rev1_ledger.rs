//! Reviewer-1 ledger probe (not for merge): one line per (fixture, eps_in).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::Tol;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use step_import::{ImportOptions, StepImport, import_step};

fn h(s: &str) -> u64 {
    let mut x = DefaultHasher::new();
    s.hash(&mut x);
    x.finish()
}

fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let mut es: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    es.sort();
    for p in es {
        if p.is_dir() {
            walk(&p, out)
        } else if matches!(
            p.extension().and_then(|e| e.to_str()),
            Some("step" | "stp" | "STEP" | "STP")
        ) {
            out.push(p)
        }
    }
}

#[test]
#[ignore]
fn zz_rev1_ledger() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut files = Vec::new();
    walk(&root.join("step-import/tests/fixtures"), &mut files);
    walk(&root.join("step-export/tests/fixtures"), &mut files);
    let only = std::env::var("REV1_ONLY").ok();
    let mut lines = Vec::new();
    for f in &files {
        let name = f.strip_prefix(&root).unwrap().display().to_string();
        if let Some(o) = &only {
            if !name.contains(o.as_str()) {
                continue;
            }
        }
        let Ok(src) = std::fs::read_to_string(f) else {
            lines.push(format!("{name}\tUNREADABLE"));
            continue;
        };
        for eps_in in [None, Some(1e-6), Some(1e-12)] {
            let opts = ImportOptions {
                eps_in,
                ..ImportOptions::default()
            };
            let t = std::time::Instant::now();
            let row = match import_step(&src, &opts, Tol::witness()) {
                Ok(imp) => {
                    let kind = match &imp {
                        StepImport::Solid { .. } => "solid",
                        StepImport::Wireframe { .. } => "wire",
                    };
                    let norms = format!("{:?}", imp.normalizations());
                    let mut kinds: Vec<String> = imp
                        .normalizations()
                        .iter()
                        .map(|n| {
                            let d = format!("{:?}", n.kind);
                            d.split(|c: char| !c.is_alphanumeric())
                                .next()
                                .unwrap()
                                .to_string()
                        })
                        .collect();
                    kinds.sort();
                    let mut census = std::collections::BTreeMap::new();
                    for k in kinds {
                        *census.entry(k).or_insert(0) += 1;
                    }
                    let body = match &imp {
                        StepImport::Solid { body, .. } => format!("{body:?}"),
                        StepImport::Wireframe { .. } => format!("{imp:?}"),
                    };
                    let body2 = match &imp {
                        StepImport::Solid { body, .. } => format!("{body:?}"),
                        StepImport::Wireframe { .. } => format!("{imp:?}"),
                    };
                    let stable = if body == body2 { "" } else { "!UNSTABLE" };
                    format!(
                        "OK {kind} promos={} census={census:?} norms#={:016x} body#={:016x}{stable}",
                        imp.curve_promotions().len(),
                        h(&norms),
                        h(&body)
                    )
                }
                Err(e) => format!("ERR {e}"),
            };
            let _ = t;
            lines.push(format!("{name}\t{eps_in:?}\t{row}"));
        }
    }
    let out = std::env::var("REV1_OUT").expect("REV1_OUT");
    std::fs::write(out, lines.join("\n") + "\n").unwrap();
}
