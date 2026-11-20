//! B3 review probe (not for merge): per-fixture disposition ledger.

use geom_core::Tol;
use std::path::{Path, PathBuf};
use step_import::{ImportOptions, StepImport, import_step};

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
fn b3_ledger() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    walk(&root.join("tests/fixtures"), &mut files);
    walk(&root.join("../step-export/tests/fixtures"), &mut files);
    let out = std::env::var("B3_LEDGER_OUT").unwrap();
    let mut lines = Vec::new();
    for f in files {
        let rel = f.strip_prefix(&root).unwrap().to_string_lossy().to_string();
        let text = std::fs::read(&f).unwrap();
        let text = String::from_utf8_lossy(&text).to_string();
        let t = std::time::Instant::now();
        let r = std::panic::catch_unwind(|| {
            import_step(&text, &ImportOptions::default(), Tol::witness())
        });
        let what = match r {
            Err(_) => "PANIC".to_string(),
            Ok(Ok(StepImport::Solid {
                body,
                normalizations,
                ..
            })) => {
                let built = normalizations
                    .iter()
                    .filter(|n| {
                        matches!(n.kind, step_import::NormalizationKind::CarrierBuilt { .. })
                    })
                    .count();
                let promoted = normalizations
                    .iter()
                    .filter(|n| {
                        matches!(n.kind, step_import::NormalizationKind::CurvePromoted { .. })
                    })
                    .count();
                let mut nurbs = 0;
                let mut circ = 0;
                let mut line = 0;
                let mut other = 0;
                for (_, e) in body.edges() {
                    match body.get_curve_geom(e.curve) {
                        Some(topo::CurveGeom::Certified(c)) => match c.carrier() {
                            geom::Curve3::Nurbs(_) => nurbs += 1,
                            geom::Curve3::Circle(_) => circ += 1,
                            geom::Curve3::Line(_) => line += 1,
                            _ => other += 1,
                        },
                        _ => other += 1,
                    }
                }
                format!(
                    "SOLID faces={} edges n/c/l/o={nurbs}/{circ}/{line}/{other} built={built} promoted={promoted}",
                    body.faces().count()
                )
            }
            Ok(Ok(StepImport::Wireframe { .. })) => "WIRE".to_string(),
            Ok(Ok(_)) => "OTHER".to_string(),
            Ok(Err(e)) => {
                let s = format!("{e}");
                format!(
                    "ERR {}",
                    s.chars().take(160).collect::<String>().replace('\n', " ")
                )
            }
        };
        lines.push(format!("{rel}\t{what}\t{:.1}s", t.elapsed().as_secs_f64()));
        std::fs::write(&out, lines.join("\n")).unwrap();
    }
}

#[test]
fn b3_dump_loft() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for rel in ["../step-export/tests/fixtures/nonuniform_loft.step"] {
        let text = std::fs::read_to_string(root.join(rel)).unwrap();
        match import_step(&text, &ImportOptions::default(), Tol::witness()) {
            Ok(StepImport::Solid { normalizations, .. }) => {
                for n in normalizations {
                    eprintln!("B3 {rel} {} {:?}", n.face, n.kind);
                }
            }
            Ok(_) => eprintln!("B3 other"),
            Err(e) => eprintln!("B3 err {e}"),
        }
    }
}
