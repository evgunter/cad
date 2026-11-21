//! Reviewer probe (exch-d-rev2): per-fixture ledger. Writes to $REV2_LEDGER.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use std::path::{Path, PathBuf};
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    let mut es: Vec<PathBuf> = rd.map(|e| e.unwrap().path()).collect();
    es.sort();
    for p in es {
        if p.is_dir() {
            walk(&p, out);
        } else if p
            .extension()
            .is_some_and(|x| x.eq_ignore_ascii_case("step") || x.eq_ignore_ascii_case("stp"))
        {
            out.push(p);
        }
    }
}

#[test]
fn zz_rev2_ledger() {
    let Ok(out) = std::env::var("REV2_LEDGER") else {
        return;
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    walk(&root.join("tests/fixtures"), &mut files);
    walk(&root.join("../step-export/tests/fixtures"), &mut files);
    let tol = geom_core::Tol::witness();
    let mut lines = Vec::new();
    let only = std::env::var("REV2_ONLY").ok();
    let epsins: Vec<Option<f64>> = match std::env::var("REV2_EPSIN") {
        Ok(v) => v
            .split(',')
            .map(|x| {
                if x == "None" {
                    None
                } else {
                    Some(x.parse().unwrap())
                }
            })
            .collect(),
        Err(_) => vec![None, Some(1e-6), Some(1e-12)],
    };
    for f in &files {
        if let Some(o) = &only {
            if !f.display().to_string().contains(o.as_str()) {
                continue;
            }
        }
        let src = std::fs::read_to_string(f).unwrap();
        let name = f.strip_prefix(root).unwrap_or(f).display().to_string();
        for &eps_in in &epsins {
            let opts = ImportOptions {
                eps_in,
                ..ImportOptions::default()
            };
            let r = std::panic::catch_unwind(|| import_step(&src, &opts, tol));
            let row = match r {
                Err(_) => "PANIC".to_string(),
                Ok(Err(e)) => format!("ERR {e}"),
                Ok(Ok(imp)) => {
                    let norms = imp.normalizations();
                    let cp = imp.curve_promotions().len();
                    let cb: Vec<String> = norms
                        .iter()
                        .filter_map(|n| match n.kind {
                            NormalizationKind::CarrierBuilt { moved_by } => {
                                Some(format!("{}:{:e}", n.face, moved_by))
                            }
                            _ => None,
                        })
                        .collect();
                    let all: Vec<String> = norms
                        .iter()
                        .map(|n| format!("{}:{:?}", n.face, n.kind))
                        .collect();
                    match imp {
                        StepImport::Solid { body, .. } => {
                            let c = topo::readback::euler_counts(&body);
                            format!(
                                "SOLID v{} e{} f{} r{} s{} promos{} built[{}] all[{}]",
                                c.v,
                                c.e,
                                c.f,
                                c.r,
                                c.s,
                                cp,
                                cb.join(","),
                                all.join(";")
                            )
                        }
                        StepImport::Wireframe { .. } => {
                            format!("WIRE promos{} all[{}]", cp, all.join(";"))
                        }
                    }
                }
            };
            lines.push(format!("{name}\t{eps_in:?}\t{row}"));
        }
    }
    std::fs::write(out, lines.join("\n") + "\n").unwrap();
}
