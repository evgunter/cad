//! Review probe: every committed fixture's import disposition, written to
//! `$CORPUS_OUT`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(&p, out);
        } else if matches!(
            p.extension().and_then(|x| x.to_str()),
            Some("step" | "stp" | "STEP" | "STP")
        ) {
            out.push(p);
        }
    }
}

#[test]
fn review_corpus_dispositions() {
    let Ok(path) = std::env::var("CORPUS_OUT") else {
        return;
    };
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut files = Vec::new();
    walk(&root, &mut files);
    files.sort();
    let mut lines = Vec::new();
    for f in &files {
        let text = std::fs::read_to_string(f).unwrap_or_default();
        let r = import_step(&text, &ImportOptions::default(), Tol::witness());
        let rel = f.strip_prefix(&root).unwrap().display().to_string();
        let line = match r {
            Ok(StepImport::Solid { normalizations, .. }) => {
                format!("{rel}\tOK\t{normalizations:?}")
            }
            Ok(other) => format!(
                "{rel}\tOK-other\t{}",
                format!("{other:?}").chars().take(200).collect::<String>()
            ),
            Err(e) => format!("{rel}\tERR\t{e:?}"),
        };
        lines.push(line);
    }
    std::fs::write(path, lines.join("\n")).unwrap();
}
