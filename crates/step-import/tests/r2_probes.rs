//! R2 review probes for MESH-3 (issue 896). Not part of the PR.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn fixture(dir: &str, name: &str) -> String {
    let p = format!("{}/tests/fixtures/{dir}/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("reading {p}: {e}"))
}

/// R2: reproduce the PR's en-route halfcap report — `halfcap_eps7.step`
/// imports Pass and then panics inside the mesh walk.
#[test]
fn r2_halfcap_eps7_imports_then_panics_in_the_walk() {
    let eps = Tol::witness().get().eps;
    let out = import_step(
        &fixture("halfcap", "halfcap_eps7.step"),
        &ImportOptions::default(),
        Tol::witness(),
    );
    let Ok(StepImport::Solid { body, .. }) = out else {
        panic!("halfcap_eps7 did NOT import at eps {eps:e}: {out:?}");
    };
    topo::validate_geometric(&body, Tol::witness()).expect("tier 3");
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mesh::tessellate(&body, 0.001, Tol::witness())
    }));
    match r {
        Ok(v) => panic!(
            "R2: halfcap_eps7 tessellated at eps {eps:e}: {:?}",
            v.map(|m| m.positions.len())
        ),
        Err(e) => {
            let s = e
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_string()))
                .unwrap_or_default();
            panic!("R2 PANIC at eps {eps:e}: {s}");
        }
    }
}

/// R2: measure whether halfcap_eps7 puts a vertex inside the walk's
/// pole band, i.e. whether it reaches `pole_v`'s decision at all.
#[test]
fn r2_halfcap_eps7_vertex_pole_distance() {
    let eps = Tol::witness().get().eps;
    let Ok(StepImport::Solid { body, .. }) = import_step(
        &fixture("halfcap", "halfcap_eps7.step"),
        &ImportOptions::default(),
        Tol::witness(),
    ) else {
        panic!("no import");
    };
    let mut lines = vec![format!("eps = {eps:e}")];
    for (fk, f) in body.faces() {
        let Some(geom::Surface::Sphere {
            center,
            radius,
            axis,
            ..
        }) = body.get_surface(f.surface)
        else {
            continue;
        };
        for sgn in [1.0f64, -1.0] {
            let pole: geom_core::Point3<f64> = *center + *axis * (*radius * sgn);
            let mut best = f64::INFINITY;
            for (_vk, v) in body.vertices() {
                let p = *body.get_point(v.point).unwrap();
                best = best.min((p - pole).norm());
            }
            lines.push(format!(
                "  face {fk:?} pole {sgn:+} : nearest vertex {best:.6e} m -> within eps? {}",
                best <= eps
            ));
        }
    }
    let m = mesh::tessellate(&body, 0.001, Tol::witness());
    lines.push(format!(
        "  tessellate -> {}",
        match &m {
            Ok(mm) => format!("Ok, {} positions", mm.positions.len()),
            Err(e) => format!("Err {e:?}"),
        }
    ));
    panic!("R2 REPORT\n{}", lines.join("\n"));
}
