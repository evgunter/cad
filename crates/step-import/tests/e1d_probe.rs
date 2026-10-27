//! Review probe (E1d): a curve off its face at the import door.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::Tol;
use step_import::{ImportOptions, import_step};

fn base() -> String {
    std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../step-export/tests/fixtures/cone.step")).unwrap()
}

fn run(label: &str, text: &str, eps_in: Option<f64>) {
    let r = import_step(text, &ImportOptions { eps_in, ..ImportOptions::default() }, Tol::witness());
    match r {
        Ok(_) => println!("PROBE {label} eps_in={eps_in:?}: OK (imported)"),
        Err(e) => {
            let t = e.to_string();
            println!(
                "PROBE {label} eps_in={eps_in:?}: eps_in_word={} kernel_defect={} damaged={} declared={} lever={} offchart={}\n  {t}",
                t.contains("ε_in"),
                t.contains("kernel defect"),
                t.contains("damaged") || t.contains("the file"),
                t.contains("declared"),
                t.contains("Recourse:"),
                t.contains("does not lie on") || t.contains("no curve of"),
            );
        }
    }
}

#[test]
fn e1d_probe_curve_off_its_face_at_import() {
    let t = base();
    for d in [1e-12, 1e-10, 3e-9, 1e-8, 1e-7, 1e-6, 1e-4, 1e-2] {
        // The cone's half-angle nudged: the rim circles leave the cone.
        let a = 0.7853981633974483 + d;
        let text = t
            .replace(
                "#5 = CONICAL_SURFACE('', #4, 0.0, 0.7853981633974483);",
                &format!("#5 = CONICAL_SURFACE('', #4, 0.0, {a:.17});"),
            )
            .replace(
                "#38 = CONICAL_SURFACE('', #37, 0.0, 0.7853981633974483);",
                &format!("#38 = CONICAL_SURFACE('', #37, 0.0, {a:.17});"),
            );
        for eps in [None, Some(1e-6), Some(1e-3)] {
            run(&format!("cone-angle+{d:e}"), &text, eps);
        }
        // The apex line #22's direction tilted about the apex.
        let c = 0.7071067811865475_f64;
        let (dx, dy) = (c * (1.0 + d), -c);
        let n = (dx * dx + dy * dy).sqrt();
        let text2 = t.replace(
            "#20 = DIRECTION('', (0.7071067811865475, -0.7071067811865475, 0.0));",
            &format!("#20 = DIRECTION('', ({:.17}, {:.17}, 0.0));", dx / n, dy / n),
        );
        for eps in [None, Some(1e-6), Some(1e-3)] {
            run(&format!("apex-line-tilt+{d:e}"), &text2, eps);
        }
        // The rim circle #14's radius nudged: off the cone and on the plane.
        let r = 1.0 + d;
        let text = t.replace("#14 = CIRCLE('', #13, 1.0);", &format!("#14 = CIRCLE('', #13, {r:.17});"));
        for eps in [None, Some(1e-6), Some(1e-3)] {
            run(&format!("rim-radius+{d:e}"), &text, eps);
        }
    }
}
