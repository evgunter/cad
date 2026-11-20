//! B2 review-2 probe: per-fixture disposition + promotion ledger, printed
//! for a diff between head and the merge-base. Not a gate.

#![allow(clippy::unwrap_used, clippy::panic, clippy::print_stdout)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    for e in rd {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(&p, out);
        } else if let Some(x) = p.extension().and_then(|x| x.to_str()) {
            if x.eq_ignore_ascii_case("step") || x.eq_ignore_ascii_case("stp") {
                out.push(p);
            }
        }
    }
}

#[test]
#[ignore = "probe"]
fn ledger() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut files = Vec::new();
    walk(&root.join("step-import/tests/fixtures"), &mut files);
    walk(&root.join("step-export/tests/fixtures"), &mut files);
    files.sort();
    let out_path = std::env::var("B2_LEDGER").unwrap();
    let mut out = String::new();
    for f in files {
        let name = f.strip_prefix(&root).unwrap().display().to_string();
        let Ok(text) = std::fs::read_to_string(&f) else {
            out += &format!("{name}\tUNREADABLE\n");
            continue;
        };
        let r = std::panic::catch_unwind(|| {
            import_step(&text, &ImportOptions::default(), Tol::witness())
        });
        match r {
            Err(_) => out += &format!("{name}\tPANIC\n"),
            Ok(Err(e)) => {
                let m = e.to_string();
                let m: String = m.chars().take(160).collect();
                out += &format!("{name}\tERR\t{m}\n");
            }
            Ok(Ok(imp)) => {
                let disp = match &imp {
                    StepImport::Solid { .. } => "SOLID".to_string(),
                    StepImport::Wireframe { curves, .. } => format!("WIRE({})", curves.len()),
                };
                let mut promos: Vec<String> = imp
                    .curve_promotions()
                    .iter()
                    .map(|p| format!("#{}:{:?}:{:.1e}", p.curve, p.kind, p.residual))
                    .collect();
                promos.sort();
                let mut kinds = std::collections::BTreeMap::<String, usize>::new();
                for n in imp.normalizations() {
                    let k = format!("{:?}", n.kind);
                    let k = k.split([' ', '{', '(']).next().unwrap().to_string();
                    *kinds.entry(k).or_default() += 1;
                }
                out += &format!(
                    "{name}\t{disp}\tpromos={}\t[{}]\tnorms={kinds:?}\n",
                    promos.len(),
                    promos.join(" ")
                );
                if let StepImport::Wireframe { curves, .. } = &imp {
                    out += &format!("{name}\tWIRECURVES\t{curves:?}\n");
                }
            }
        }
    }
    std::fs::write(out_path, out).unwrap();
}

#[test]
#[ignore = "probe"]
fn reversed_same_sense_on_a_promoted_arc() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/arc-trim/w_semi.step");
    let text = std::fs::read_to_string(p).unwrap();
    let e = "#1024=EDGE_CURVE('',#1004,#1006,#1023,.T.);";
    assert!(text.contains(e));
    let legit = text
        .replace(e, "#1024=EDGE_CURVE('',#1006,#1004,#1023,.F.);")
        .replace(
            "#1066=ORIENTED_EDGE('',*,*,#1024,.F.);",
            "#1066=ORIENTED_EDGE('',*,*,#1024,.T.);",
        )
        .replace(
            "#1111=ORIENTED_EDGE('',*,*,#1024,.T.);",
            "#1111=ORIENTED_EDGE('',*,*,#1024,.F.);",
        );
    let wrong = text.replace(e, "#1024=EDGE_CURVE('',#1004,#1006,#1023,.F.);");
    for (name, t) in [("legit-reversed", legit), ("wrong-reversed", wrong)] {
        let r = import_step(&t, &ImportOptions::default(), Tol::witness());
        match r {
            Ok(i) => println!("RSS {name}: OK promos={}", i.curve_promotions().len()),
            Err(e) => println!(
                "RSS {name}: ERR {}",
                e.to_string().chars().take(200).collect::<String>()
            ),
        }
    }
}
