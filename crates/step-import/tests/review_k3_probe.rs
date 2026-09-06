//! REVIEW PROBE (review/kernel, K3): a uniform-scale corruption of the
//! notched fixture sized so the OLD row-1 tolerance (print-precision
//! slop, "3000000000.0" -> 0.05 mm^3) ACCEPTS the wrong volume while
//! the NEW budget (pad + native pad + (2 faces + 2) ulps) catches it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// The 1.4142135623730951 literal below is not our approximation of
// sqrt(2): it is the EXACT token the committed notched.step prints for
// the circle radius, quoted so the string replacement can find it.
#![allow(clippy::approx_constant)]
use crate::common;

use common::{expect_sidecar, fixture};
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

#[test]
fn k3_scale_corruption_old_accepted_new_catches() {
    let s = 1.1e-12; // volume scales by (1+s)^3: dev ~ 0.0099 mm^3
    let text: String = fixture("notched", "step")
        .lines()
        .map(|l| {
            let scaled = if l.contains("CARTESIAN_POINT") {
                scale_triple(l, s)
            } else if l.contains("CIRCLE") || l.contains("CYLINDRICAL_SURFACE") {
                l.replace(
                    "1.4142135623730951",
                    &format!("{:.17}", 1.4142135623730951 * (1.0 + s)),
                )
            } else {
                l.to_owned()
            };
            scaled + "\n"
        })
        .collect();
    let body = match import_step(&text, &ImportOptions::default(), Tol::witness()).unwrap() {
        StepImport::Solid { body, .. } => body,
        StepImport::Wireframe { .. } => panic!("wireframe"),
    };
    assert_eq!(
        topo::validate_geometric(&body, Tol::witness()),
        Ok(()),
        "tier 3 must pass"
    );
    let props = topo::mass_properties(&body, Tol::witness()).unwrap();
    let expect = expect_sidecar("notched");
    let expected_m3 = expect.kernel_volume_mm3 * 1e-9;
    let ulp = expected_m3.next_up() - expected_m3;
    let dev = (props.volume - expected_m3).abs();
    // OLD budget: pad + half-last-printed-decimal of "3000000000.0" + faces ulps
    let old = props.volume_pad + 0.05e-9 + 6.0 * ulp;
    // NEW budget: pad_imported + pad_native + (2*faces+2) ulps
    let new = props.volume_pad + expect.kernel_volume_pad_mm3 * 1e-9 + 14.0 * ulp;
    println!(
        "dev={:e} m3, old budget={:e}, new budget={:e}",
        dev, old, new
    );
    assert!(
        dev <= old,
        "old tolerance would have ACCEPTED this corruption"
    );
    assert!(dev > new, "new tolerance must CATCH it");
}

fn scale_triple(line: &str, s: f64) -> String {
    let (pre, rest) = line.split_once('(').unwrap();
    let (_, coords) = rest.split_once('(').unwrap();
    let (coords, post) = coords.split_once(')').unwrap();
    let scaled: Vec<String> = coords
        .split(',')
        .map(|c| format!("{:.17}", c.trim().parse::<f64>().unwrap() * (1.0 + s)))
        .collect();
    format!("{pre}('', ({}){post}", scaled.join(", "))
}

/// K6, **FLIPPED by M7-3** (the S9 pattern; history carried): from
/// M7-1 until M7-3 this row pinned that `loft_prism.step` itself —
/// not a synthetic record — refused TYPED naming the
/// `B_SPLINE_SURFACE_WITH_KNOTS` frontier, the stated reason for its
/// row-1 exclusion. That exclusion reason is now retired by the very
/// unit the old pin's message pointed at: the fixture imports as a
/// first-class solid and sits in `SOLID_FIXTURES` (the full row-1
/// census/volume/tier obligations run in `roundtrip.rs`; the
/// NURBS-specific acceptance rows in `nurbs_import.rs`). What this
/// row keeps: the flip is an ACCEPTANCE, not a silent drift — the
/// import must produce a solid body, or the flip story is false.
#[test]
fn k6_loft_prism_imports_the_nurbs_frontier_retired() {
    let text = fixture("loft_prism", "step");
    match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { .. }) => {}
        Ok(StepImport::Wireframe { .. }) => {
            panic!("loft_prism must import as a solid, not a wireframe")
        }
        Err(e) => panic!("loft_prism must import since M7-3 — the flip regressed: {e}"),
    }
}
