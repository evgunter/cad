//! REVIEW PROBE (review/kernel, K3): a uniform-scale corruption of the
//! notched fixture sized so the OLD row-1 tolerance (print-precision
//! slop, "3000000000.0" -> 0.05 mm^3) ACCEPTS the wrong volume while
//! the NEW budget (pad + native pad + (2 faces + 2) ulps) catches it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// The 1.4142135623730951 literal below is not our approximation of
// sqrt(2): it is the EXACT token the committed notched.step prints for
// the circle radius, quoted so the string replacement can find it.
#![allow(clippy::approx_constant)]
mod common;

use common::{expect_sidecar, fixture};
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
    let body = match import_step(&text, &ImportOptions::default()).unwrap() {
        StepImport::Solid { body, .. } => body,
        StepImport::Wireframe { .. } => panic!("wireframe"),
    };
    assert_eq!(topo::validate_geometric(&body), Ok(()), "tier 3 must pass");
    let props = topo::mass_properties(&body).unwrap();
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

/// K6: loft_prism.step itself (not a synthetic record) must still
/// refuse TYPED, naming the B-spline surface frontier — the stated
/// reason for its row-1 exclusion.
#[test]
fn k6_loft_prism_refuses_typed_at_the_nurbs_frontier() {
    let text = fixture("loft_prism", "step");
    match import_step(&text, &ImportOptions::default()) {
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("B_SPLINE_SURFACE_WITH_KNOTS"),
                "refusal must name the frontier entity, got: {msg}"
            );
        }
        Ok(_) => panic!("loft_prism imported — the row-1 exclusion reason is GONE"),
    }
}
