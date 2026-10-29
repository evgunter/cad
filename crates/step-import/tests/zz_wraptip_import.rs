//! PROBE (foreign-wraptip): OCC's one-meridian statements of a pole
//! (`seamed.step`, a sphere cap; `freecad/cone_apex.step`) imported,
//! validated, measured, meshed, cut. Prints; asserts nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::Tol;
use std::panic::{AssertUnwindSafe, catch_unwind};
use step_import::{ImportOptions, StepImport, import_step};
use topo::{BooleanResult, ContactRecords};

fn read(rel: &str) -> String {
    let p = format!("{}/tests/fixtures/{rel}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&p).unwrap()
}

fn row(name: &str, f: impl FnOnce() -> String) {
    let mode = if std::env::var_os("CAD_WRAPTIP").is_some() { "wraptip" } else { "main" };
    let out = catch_unwind(AssertUnwindSafe(f)).unwrap_or_else(|p| {
        let m = p.downcast_ref::<String>().cloned().unwrap_or_default();
        format!("PANIC {}", m.chars().take(600).collect::<String>())
    });
    println!("ROW [{mode}] {name}: {out}");
}

fn vol(b: &topo::AtRestBody<f64>, want: f64) -> String {
    let tol = Tol::witness();
    match topo::mass_properties(b, tol, topo::test_support::pinned_resolution(tol)) {
        Ok(m) => match m.met() {
            Ok(e) => format!(
                "vol {:.15e} want {want:.15e} rel {:.2e}",
                e.volume,
                (e.volume - want).abs() / want
            ),
            Err(u) => format!("vol unmet {u}"),
        },
        Err(e) => format!("vol refused {e}"),
    }
}

fn tiers(b: &topo::AtRestBody<f64>) -> String {
    let tol = Tol::witness();
    format!(
        "tier3 {:?} tier3' {:?}",
        topo::validate_geometric(b, tol).is_ok(),
        topo::validate_pseudomanifold(b, &ContactRecords::default(), tol).is_ok()
    )
}

#[test]
fn zz_wraptip_import() {
    let tol = Tol::witness();
    // seamed.step: R = 10 mm, rim at latitude 0.5 rad.
    let r = 0.010_f64;
    let h = r * (1.0 - 0.5_f64.sin());
    let cap = core::f64::consts::PI * h * h * (3.0 * r - h) / 3.0;
    // cone_apex.step: FreeCAD Part.makeCone(1, 0, h) in mm; read its
    // closed form off the file below.
    for (name, file, want) in [
        ("seamed.step", "wraptip/seamed.step", cap),
        ("spur.step (meridian stops short of the pole)", "wraptip/spur.step", cap),
        ("cone_apex.step", "freecad/cone_apex.step", core::f64::consts::PI / 3.0 * 1e-9),
    ] {
        row(name, || {
            let imp = match import_step(&read(file), &ImportOptions::default(), tol) {
                Ok(i) => i,
                Err(e) => return format!("import REFUSED {e:?}"),
            };
            let StepImport::Solid { body, normalizations, .. } = imp else {
                return "not a solid".into();
            };
            let b = topo::test_support::finished("imported", body.body().clone(), tol);
            let mut s = format!(
                "V/E/F {:?} normalizations {:?} {} {}",
                (b.vertices().count(), b.edges().count(), b.faces().count()),
                normalizations,
                tiers(&b),
                vol(&b, want)
            );
            s += &match mesh::tessellate(&b, 1e-4, tol) {
                Ok(m) => format!(" mesh ok ({:?})", mesh::validate::check_mesh(&m).is_ok()),
                Err(e) => format!(" mesh REFUSED {e:?}"),
            };
            // A Boolean through the pole/apex: subtract the x > 0 half space.
            let cutter = topo::test_support::finished(
                "cutter",
                sweep::test_support::brick((0.0, 1.0), (-1.0, 1.0), (-1.0, 1.0), tol),
                tol,
            );
            s += &match topo::subtract(&b, &cutter, tol) {
                Ok(BooleanResult::Body(o)) => format!(" | minus x>0: {} {}", vol(&o.body, want / 2.0), tiers(&o.body)),
                other => format!(" | minus x>0: {other:?}"),
            };
            s
        });
    }
}
