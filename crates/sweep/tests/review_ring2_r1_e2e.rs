//! RING-2 review probe (R1): an end-to-end certification of the
//! NURBS-walled loft prism, printing every limb's bits so the head and
//! the same tree with only `ring_interval.rs` swapped back can be
//! compared number by number.
//!
//! Prints rather than pins: this row is a measuring instrument for one
//! comparison and is not a gate.

use geom_core::Tol;
use sweep::test_support::loft_prism;

#[test]
fn ring2_r1_end_to_end_certificate_bits() {
    let tol = Tol::witness();
    let body = loft_prism(tol);
    topo::validate_geometric(&body, tol).expect("the prism is tier-3 valid");
    let props = topo::mass_properties(&body, tol).expect("mass properties");
    println!(
        "R1E2E volume={:.17e} bits={:#018x} pad={:.17e} pad_bits={:#018x}",
        props.volume,
        props.volume.to_bits(),
        props.volume_pad,
        props.volume_pad.to_bits()
    );
    println!(
        "R1E2E area={:.17e} bits={:#018x} apad={:.17e} apad_bits={:#018x}",
        props.surface_area,
        props.surface_area.to_bits(),
        props.area_pad,
        props.area_pad.to_bits()
    );
    for chordal in [1e-1, 1e-2, 1e-3] {
        let mesh = mesh::tessellate(&body, chordal, tol).expect("tessellates");
        println!(
            "R1E2E chordal={chordal:e} triangles={} positions={} patches={}",
            mesh.patches.iter().map(|p| p.triangles.len()).sum::<usize>(),
            mesh.positions.len(),
            mesh.patches.len()
        );
    }
}
