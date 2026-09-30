//! Review probe (namedepth-r1): save, canonical bytes, pin and load of
//! every corpus document and every committed document file, written to
//! `$RV_OUT` so main's and the head's can be diffed byte for byte.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use crate::corpus::documents;
use editor_core::{ProfileDoc, canonical_bytes, content_pin, load, save};
use geom_core::Tol;
use std::fmt::Write as _;

fn out_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("RV_OUT").map(std::path::PathBuf::from)
}

#[test]
fn rv_bytes_of_every_document() {
    let Some(dir) = out_dir() else { return };
    std::fs::create_dir_all(&dir).unwrap();
    let tol = Tol::witness();
    let mut summary = String::new();
    let mut t_save = std::time::Duration::ZERO;
    let mut t_load = std::time::Duration::ZERO;
    let mut t_canon = std::time::Duration::ZERO;
    for d in documents() {
        for (label, snapshot, edits) in [
            (
                format!("{}_log", d.name),
                ProfileDoc::empty_derived("rv", tol),
                d.edits.clone(),
            ),
            (format!("{}_snapshot", d.name), d.doc.clone(), Vec::new()),
        ] {
            let t = std::time::Instant::now();
            let text = save(&snapshot, &edits, tol).expect("save");
            t_save += t.elapsed();
            std::fs::write(dir.join(format!("{label}.save")), &text).unwrap();
            let t = std::time::Instant::now();
            let canon = canonical_bytes(&snapshot, tol).expect("canon");
            t_canon += t.elapsed();
            std::fs::write(dir.join(format!("{label}.canon")), &canon).unwrap();
            let pin = content_pin(&snapshot, tol).expect("pin");
            let t = std::time::Instant::now();
            let loaded = load(&text, tol).expect("load");
            t_load += t.elapsed();
            let resaved = save(&loaded.snapshot, &loaded.edits, tol).expect("resave");
            writeln!(summary, "{label} pin={pin:?} resave_equal={}", resaved == text).unwrap();
        }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let files = [
        "crates/editor-core/tests/corpus/die_tool.pncad",
        "crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
        "crates/pncad/tests/plate_param.pncad",
        "crates/viewer/tests/gallery_ring.pncad",
        "crates/editor-core/tests/golden/golden.cad",
    ]
    .into_iter()
    .map(String::from)
    .chain((1..=19).map(|v| format!("crates/editor-core/tests/bool13_goldens/v{v}_golden.cad")));
    for f in files {
        let text = std::fs::read_to_string(root.join(&f)).unwrap();
        let label = f.replace('/', "_");
        match load(&text, tol) {
            Ok(loaded) => {
                let resaved = save(&loaded.snapshot, &loaded.edits, tol);
                let resaved = match resaved {
                    Ok(s) => s,
                    Err(e) => format!("SAVE-ERR {e:?}"),
                };
                std::fs::write(dir.join(format!("{label}.resave")), &resaved).unwrap();
                let canon = canonical_bytes(&loaded.doc, tol)
                    .map(|b| String::from_utf8(b).unwrap())
                    .unwrap_or_else(|e| format!("ERR {e:?}"));
                std::fs::write(dir.join(format!("{label}.canon")), &canon).unwrap();
                let pin = content_pin(&loaded.doc, tol);
                writeln!(
                    summary,
                    "{label} loaded pin={pin:?} resave_equal_file={}",
                    resaved == text
                )
                .unwrap();
            }
            Err(e) => writeln!(summary, "{label} LOAD-ERR {e:?}").unwrap(),
        }
    }
    writeln!(summary, "TIME save={t_save:?} load={t_load:?} canon={t_canon:?}").unwrap();
    std::fs::write(dir.join("summary.txt"), summary).unwrap();
}


/// A load refusal inside a nested name: where does it say it is?
#[test]
fn rv_refusal_inside_a_name() {
    let Some(dir) = out_dir() else { return };
    let tol = Tol::witness();
    let d = documents()
        .into_iter()
        .find(|d| d.name == "die")
        .expect("the die");
    let text = save(&d.doc, &[], tol).unwrap();
    let mut out = String::new();
    let first = text.find("\"FromA\"").expect("a FromA");
    let second = first + 1 + text[first + 1..].find("\"FromA\"").expect("a nested FromA");
    let after = |t: &str, from: &str, to: &str| {
        let at = second + t[second..].find(from).expect("found");
        format!("{}{}{}", &t[..at], to, &t[at + from.len()..])
    };
    let cases = [
        ("variant", format!("{}\"FromZ\"{}", &text[..second], &text[second + 7..])),
        ("field", after(&text, "\"node\":", "\"nodx\":")),
        ("kind", after(&text, "\"kind\": \"", "\"kind\": \"X")),
        ("type", after(&text, "\"node\": ", "\"node\": \"s\", \"zz\": ")),
    ];
    for (what, bad) in cases {
        let e = load(&bad, tol).map(|_| ()).unwrap_err();
        writeln!(out, "{what}: {e:?}").unwrap();
    }
    std::fs::write(dir.join("refusal.txt"), out).unwrap();
}

/// The older-build goldens (schema line dropped): what each load says.
#[test]
fn rv_older_goldens_refusals() {
    let Some(dir) = out_dir() else { return };
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut out = String::new();
    for v in 1..=19 {
        let text = std::fs::read_to_string(root.join(format!("tests/bool13_goldens/v{v}_golden.cad"))).unwrap();
        let text = text.split_once('\n').unwrap().1;
        let r = load(text, Tol::witness()).map(|l| content_pin(&l.doc, Tol::witness()));
        writeln!(out, "v{v}: {r:?}").unwrap();
    }
    std::fs::write(dir.join("goldens.txt"), out).unwrap();
}
