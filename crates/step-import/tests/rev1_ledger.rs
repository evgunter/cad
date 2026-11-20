//! Review probe (exch-b2-rev1): disposition + promotion ledger per
//! committed fixture, printed for diffing head against the merge-base.

use geom_core::Tol;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(&p, out);
        } else if matches!(
            p.extension()
                .and_then(|x| x.to_str())
                .map(str::to_ascii_lowercase)
                .as_deref(),
            Some("step" | "stp")
        ) {
            out.push(p);
        }
    }
}

#[test]
fn rev1_ledger() {
    let root = std::env::var("REV1_FIXTURES").map_or_else(
        |_| Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"),
        PathBuf::from,
    );
    let mut files = Vec::new();
    walk(&root, &mut files);
    files.sort();
    let mut lines = Vec::new();
    for f in &files {
        let text = std::fs::read(f).unwrap();
        let text = String::from_utf8_lossy(&text).into_owned();
        let name = f.strip_prefix(&root).unwrap().display().to_string();
        let r = std::panic::catch_unwind(|| {
            import_step(&text, &ImportOptions::default(), Tol::witness())
        });
        let line = match r {
            Err(_) => format!("{name}\tPANIC"),
            Ok(Err(e)) => {
                let s = e.to_string();
                format!("{name}\tERR\t{}", s.chars().take(160).collect::<String>())
            }
            Ok(Ok(imp)) => {
                let disp = match &imp {
                    StepImport::Solid { .. } => "SOLID",
                    StepImport::Wireframe { .. } => "WIRE",
                    #[allow(unreachable_patterns)]
                    _ => "OTHER",
                };
                let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
                for n in imp.normalizations() {
                    let k = match &n.kind {
                        NormalizationKind::CurvePromoted { kind, .. } => {
                            format!("CurvePromoted:{kind:?}")
                        }
                        other => format!("{other:?}")
                            .split([' ', '{', '('])
                            .next()
                            .unwrap()
                            .to_owned(),
                    };
                    *kinds.entry(k).or_default() += 1;
                }
                let mut cp: BTreeMap<String, usize> = BTreeMap::new();
                for p in imp.curve_promotions() {
                    *cp.entry(format!("{:?}", p.kind)).or_default() += 1;
                }
                format!("{name}\t{disp}\tnorm={kinds:?}\tpromos={cp:?}")
            }
        };
        lines.push(line);
    }
    let out = std::env::var("REV1_LEDGER_OUT").unwrap_or_else(|_| "/dev/stdout".into());
    std::fs::write(out, lines.join("\n") + "\n").unwrap();
}

/// The `w_semi` rim edge restated `.F.` with its vertices swapped: the same
/// edge, so it must still import.
#[test]
fn rev1_w_semi_rim_reversed_same_sense_imports() {
    let p = format!(
        "{}/tests/fixtures/arc-trim/w_semi.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(p).unwrap();
    let old = "#1024=EDGE_CURVE('',#1004,#1006,#1023,.T.);";
    assert!(text.contains(old));
    let flipped = text
        .replace(old, "#1024=EDGE_CURVE('',#1006,#1004,#1023,.F.);")
        .replace("*,*,#1024,.T.);", "*,*,#1024,@T@);")
        .replace("*,*,#1024,.F.);", "*,*,#1024,.T.);")
        .replace("@T@", ".F.");
    let r = import_step(&flipped, &ImportOptions::default(), Tol::witness());
    println!(
        "rev1 .F. rim: {:?}",
        r.as_ref()
            .map(|i| i.curve_promotions())
            .map_err(ToString::to_string)
    );
    assert!(r.is_ok(), "{:?}", r.err().map(|e| e.to_string()));
    // and the complement in .F. spelling: start=#1006 end=#1004 same_sense .T. names 180->90 the long way... skip
}

/// Flip each EDGE_CURVE of `w_semi` to its `.F.` spelling, one at a time.
#[test]
fn rev1_w_semi_flip_each_edge() {
    let p = std::env::var("REV1_WSEMI").unwrap_or_else(|_| {
        format!(
            "{}/tests/fixtures/arc-trim/w_semi.step",
            env!("CARGO_MANIFEST_DIR")
        )
    });
    let text = std::fs::read_to_string(p).unwrap();
    for line in text.lines().filter(|l| l.contains("=EDGE_CURVE(")) {
        let (head, rest) = line.split_once("=EDGE_CURVE('',").unwrap();
        let parts: Vec<&str> = rest.trim_end_matches(");").split(',').collect();
        let flipped = format!(
            "{head}=EDGE_CURVE('',{},{},{},.F.);",
            parts[1], parts[0], parts[2]
        );
        let mut t = text.replace(line, &flipped);
        for o in [".T.", ".F."] {
            let from = format!("*,*,{head},{o});");
            let to = format!("*,*,{head},{});", if o == ".T." { "@T@" } else { "@F@" });
            t = t.replace(&from, &to);
        }
        t = t.replace("@T@", ".F.").replace("@F@", ".T.");
        let r = import_step(&t, &ImportOptions::default(), Tol::witness());
        println!(
            "rev1 flip {head}: {:?}",
            r.as_ref()
                .map(|i| i.curve_promotions().len())
                .map_err(ToString::to_string)
        );
    }
}

/// `cyl_quad`'s top rim with its last control moved off the first by a
/// multiple of ε_in (1e-7 mm) BACKWARD along the circle: a closure gap.
#[test]
fn rev1_near_closed_rim() {
    let p = std::env::var("REV1_CYLQUAD").unwrap_or_else(|_| {
        format!(
            "{}/tests/fixtures/closed-rim/cyl_quad.step",
            env!("CARGO_MANIFEST_DIR")
        )
    });
    let text = std::fs::read_to_string(p).unwrap();
    let old = "#902406 = CARTESIAN_POINT('',(0.5,-1.2246467991473532e-16,1.0));";
    assert!(text.contains(old));
    for gap in [0.5e-7, 0.9e-7, 1.5e-7, 3e-7, 6e-7, 1e-6, 1e-5] {
        let t = text.replace(
            old,
            &format!("#902406 = CARTESIAN_POINT('',(0.5,{:e},1.0));", -gap),
        );
        let r = import_step(&t, &ImportOptions::default(), Tol::witness());
        println!(
            "rev1 gap {gap:e} mm: {:?}",
            r.as_ref()
                .map(|i| i
                    .curve_promotions()
                    .iter()
                    .map(|c| format!("{:?}", c.kind))
                    .collect::<Vec<_>>())
                .map_err(ToString::to_string)
        );
    }
}
