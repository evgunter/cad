//! Review probe (PR 5741): a held certificate measured against another body
//! with the same face arena.
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};
use topo::test_support::{pinned_resolution, measure_unvalidated};

fn scaled(text: &str, k: f64) -> String {
    text.lines()
        .map(|l| {
            if let Some(i) = l.find("CARTESIAN_POINT('',(") {
                let start = i + "CARTESIAN_POINT('',(".len();
                let end = l[start..].find(')').unwrap() + start;
                let nums: Vec<String> = l[start..end]
                    .split(',')
                    .map(|s| format!("{:?}", s.trim().parse::<f64>().unwrap() * k))
                    .collect();
                format!("{}{}{}", &l[..start], nums.join(","), &l[end..])
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn probe_certificate_of_another_body() {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/box.step"
    ))
    .unwrap();
    let tol = Tol::witness();
    let a = import_step(&text, &ImportOptions::default(), tol).unwrap();
    let b = import_step(&scaled(&text, 2.0), &ImportOptions::default(), tol).unwrap();
    let (StepImport::Solid { body: ba, certificate: ca, .. }, StepImport::Solid { body: bb, .. }) = (a, b) else {
        panic!()
    };
    let r = || pinned_resolution::<f64>(tol);
    let own_a = ca.measure(&ba, r()).unwrap();
    let fresh_b = measure_unvalidated(&bb, tol, r());
    let cross = ca.measure(&bb, r());
    eprintln!("A via own cert: {:?}", own_a.enclosure.volume);
    eprintln!("B fresh: {:?}", fresh_b.map(|m| m.enclosure.volume));
    eprintln!("A's cert against B: {:?}", cross.map(|m| (m.enclosure.volume, m.stop)));
}
