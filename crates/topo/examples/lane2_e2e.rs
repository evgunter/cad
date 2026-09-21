//! R1 end-to-end exercise: a straddling declared face pair through the
//! public tier-3' doors at f64 and at Dual64.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::print_stdout)]

use geom_core::{Dual64, Tol};
use topo::{ContactRecords, PatchContact, ValidationError};

fn render(label: &str, r: Result<(), Vec<ValidationError>>) {
    match r {
        Ok(()) => println!("{label}: OK (certified/clean)"),
        Err(es) => {
            println!("{label}: {} error(s)", es.len());
            for e in &es {
                println!("    debug   : {e:?}");
                println!("    display : {e}");
            }
        }
    }
}

fn main() {
    let tol = Tol::witness();

    // --- f64: the straddle seat with its declared patch pair ---
    let seat = topo::test_support::straddle_seat(tol);
    let mut records = ContactRecords::default();
    records.patches.push(PatchContact {
        face_a: seat.post_top,
        face_b: seat.shelf_bottom,
    });

    render(
        "f64 validate_pseudomanifold (declared)",
        topo::validate_pseudomanifold(&seat.body, &records, tol),
    );
    render(
        "f64 validate_pseudomanifold_structural (declared)",
        topo::validate_pseudomanifold_structural(&seat.body, &records, tol),
    );
    render(
        "f64 validate_pseudomanifold_structural (undeclared)",
        topo::validate_pseudomanifold_structural(&seat.body, &ContactRecords::default(), tol),
    );

    // --- Dual64: the same seat rebuilt ---
    let post: topo::test_support::Prism<Dual64> = topo::test_support::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.0,
        0.5,
        tol,
    );
    let shelf: topo::test_support::Prism<Dual64> = topo::test_support::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
        tol,
    );
    let mut dual = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut dual, &shelf.body, tol).unwrap();
    let mut drecords = ContactRecords::default();
    drecords.patches.push(PatchContact {
        face_a: post.top_face,
        face_b: keys.face(shelf.bottom_face).unwrap(),
    });
    render(
        "Dual64 validate_pseudomanifold_structural (declared)",
        topo::validate_pseudomanifold_structural(&dual, &drecords, tol),
    );
}
