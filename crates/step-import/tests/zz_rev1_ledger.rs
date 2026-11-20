//! Review probe (rev1): every fixture's import outcome and promotion ledger.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]
use geom_core::Tol;
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap().filter_map(Result::ok) {
        let p = e.path();
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
fn rev1_ledger() {
    let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/.."));
    let mut files = Vec::new();
    walk(&root.join("step-import/tests/fixtures"), &mut files);
    walk(&root.join("step-export/tests/fixtures"), &mut files);
    files.sort();
    for p in files {
        let name = p.strip_prefix(&root).unwrap().display().to_string();
        let Ok(text) = std::fs::read_to_string(&p) else { continue };
        let r = std::panic::catch_unwind(|| import_step(&text, &ImportOptions::default(), Tol::witness()));
        let line = match r {
            Err(_) => "PANIC".to_owned(),
            Ok(Ok(StepImport::Solid { ref normalizations, .. })) => {
                let mut kinds: Vec<String> = normalizations
                    .iter()
                    .filter_map(|n| match n.kind {
                        NormalizationKind::SurfacePromotion { to, .. } => Some(format!("{}:{to:?}", n.face)),
                        _ => None,
                    })
                    .collect();
                kinds.sort();
                let joins = normalizations.iter().filter(|n| n.kind == NormalizationKind::JoinedEdges).count();
                format!("OK solid joins={joins} promo={kinds:?}")
            }
            Ok(Ok(_)) => "OK wireframe".to_owned(),
            Ok(Err(e)) => format!("ERR {}", e.to_string().lines().next().unwrap_or("")),
        };
        println!("LEDGER {name}: {line}");
    }
}
