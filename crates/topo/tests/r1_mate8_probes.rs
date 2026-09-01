//! R1 review probes for MATE-8 (issue 1435). Not part of the unit;
//! recorded on the review branch only.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom_core::Tol;
use topo::{Body, ContactRecords, FaceKey, PatchContact, ValidationError};

fn overhang_seat() -> (Body<f64>, FaceKey, FaceKey) {
    let post: common::Prism<f64> = common::prism_z(
        &[
            (0.20, 0.20),
            (0.40, 0.30),
            (0.60, 0.42),
            (0.70, 0.30),
            (0.80, 0.42),
            (0.85, 0.50),
            (0.15, 0.50),
            (0.25, 0.30),
        ],
        0.0,
        0.5,
    );
    shelf_over(post)
}

fn spike_overhang_seat() -> (Body<f64>, FaceKey, FaceKey) {
    let post: common::Prism<f64> = common::prism_z(
        &[
            (0.10, 0.10),
            (0.70, 0.10),
            (0.70, 0.20),
            (0.55, 0.30),
            (0.45, 0.40),
            (0.35, 0.30),
            (0.10, 0.20),
        ],
        0.0,
        0.5,
    );
    shelf_over(post)
}

fn shelf_over(post: common::Prism<f64>) -> (Body<f64>, FaceKey, FaceKey) {
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let shelf_bottom = keys.face(shelf.bottom_face).unwrap();
    (body, post.top_face, shelf_bottom)
}

fn errors(body: &Body<f64>, a: FaceKey, b: FaceKey) -> Vec<ValidationError> {
    let records = ContactRecords {
        patches: vec![PatchContact {
            face_a: a,
            face_b: b,
        }],
        ..ContactRecords::default()
    };
    match topo::validate_pseudomanifold(body, &records, Tol::witness()) {
        Ok(()) => Vec::new(),
        Err(e) => e,
    }
}

/// Prints all four cells of the red-first table without short-circuiting,
/// so the BEFORE state can be read in one run (`--nocapture`).
#[test]
fn r1_the_four_cells_of_the_table() {
    for (name, seat) in [
        ("spike", spike_overhang_seat as fn() -> _),
        ("overhang", overhang_seat as fn() -> _),
    ] {
        let (body, cap, shelf) = seat();
        println!("{name} cap-first : {:?}", errors(&body, cap, shelf));
        println!("{name} shelf-first: {:?}", errors(&body, shelf, cap));
    }
}
