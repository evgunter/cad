//! **Near-tangent pierce poses classify without a kernel-defect
//! invariant**: a prism's corner on the near face of a cube of side 4,
//! the face's plane `d` rad off one of the corner's edges
//! ([`NtPrism::near_normal`]), at the poses where the near-tangent
//! census (`examples/near_tangent_census_probe.rs`) measured each of
//! three classification sites refusing `ClassificationInvariant` on
//! valid input, at ε = 1e-9, 1e-6 and 1e-12. A static fixture: every
//! pose is one the census named at some ε row, so each row's own poses
//! are in the list whichever row runs it.
//!
//! - **An unglued corner pair read one carrier at the corner and apart
//!   at the origin** (`boolean/recl.rs` `require_same`,
//!   `boolean/vtxfac.rs`' coplanar lump): a sliver sector the cube's
//!   side face cuts off the prism's side face, coplanar with it at the
//!   sector's arm. The pair is read over the arm about the corner, so
//!   it refuses as the near-coincidence it is.
//! - **A pierce germ re-read by sector membership** (`boolean/vtxfac.rs`
//!   `pierce_germ_dir`): the germ along a bound read On lies a definite
//!   angle outside the sector at its arm, so neither ray read within.
//!   The ray is the transition's own.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::{BooleanError, BooleanResult};

use crate::common::differential::{NtPrism, nt_cube, nt_prisms};

/// `(prism, edge, sixteenths, tilt)`: the census's `"{prism} nt e{edge}
/// a{sixteenths} d{tilt}"`, each beside the site it refused at and the
/// ε row: `germ` the pierce germ, `coplanar` the coplanar lump, `recl`
/// the ON sector pair.
const POSES: [(&str, usize, u32, f64); 28] = [
    ("Lbot", 0, 12, -1e-9),    // germ, ε = 1e-9
    ("Lmirror", 0, 12, -1e-9), // germ, ε = 1e-9
    ("Ltop", 0, 12, -1e-9),    // germ, ε = 1e-9
    ("convex", 0, 3, 1e-5),    // coplanar, ε = 1e-9
    ("w345", 1, 4, 1e-5),      // coplanar, ε = 1e-9
    ("w60", 1, 4, 1e-5),       // coplanar, ε = 1e-9
    ("convex", 0, 11, -1e-6),  // recl, ε = 1e-9
    ("w345", 1, 11, -1e-5),    // recl, ε = 1e-9
    ("w60", 1, 11, -1e-5),     // recl, ε = 1e-9
    ("Lbot", 0, 12, -1e-6),    // germ, ε = 1e-6
    ("Lmirror", 1, 0, -1e-6),  // germ, ε = 1e-6
    ("Ltop", 0, 12, -1e-6),    // germ, ε = 1e-6
    ("convex", 0, 3, 1e-4),    // coplanar, ε = 1e-6
    ("w60", 1, 4, 1e-4),       // coplanar, ε = 1e-6
    ("convex", 0, 11, -1e-4),  // recl, ε = 1e-6
    ("w345", 1, 11, 1e-4),     // recl, ε = 1e-6
    ("w60", 1, 11, -1e-4),     // recl, ε = 1e-6
    ("asym", 2, 1, 1e-7),      // germ, ε = 1e-12
    ("convex", 0, 10, -1e-7),  // germ, ε = 1e-12
    ("notch307", 2, 1, 1e-7),  // germ, ε = 1e-12
    ("convex", 0, 3, 1e-7),    // coplanar, ε = 1e-12
    ("w345", 1, 4, 1e-7),      // coplanar, ε = 1e-12
    ("w60", 1, 4, 1e-7),       // coplanar, ε = 1e-12
    ("convex", 0, 11, -1e-7),  // recl, ε = 1e-12
    ("w345", 1, 11, 1e-7),     // recl, ε = 1e-12
    ("w60", 1, 11, -1e-7),     // recl, ε = 1e-12
    ("convex", 0, 4, 1e-5),    // recl, ε = 1e-9
    ("Ltop", 0, 12, 1e-9),     // germ, ε = 1e-9
];

#[test]
fn near_tangent_pierce_poses_refuse_no_classification_invariant() {
    let tol = Tol::witness();
    let prisms = nt_prisms();
    let prism = |name: &str| -> &NtPrism {
        prisms
            .iter()
            .find(|p| p.name == name)
            .unwrap_or_else(|| panic!("no near-tangent prism {name}"))
    };
    let mut defects = Vec::new();
    for &(name, edge, a, d) in &POSES {
        let pr = prism(name);
        let body = pr.body(tol);
        let cube = nt_cube(pr.v, pr.near_normal(edge, a, d), tol);
        let runs: [(&str, Result<BooleanResult<f64>, BooleanError>); 2] = [
            ("prism ∪ cube", topo::union(&body, &cube, tol)),
            ("cube ∪ prism", topo::union(&cube, &body, tol)),
        ];
        for (op, r) in runs {
            if let Err(e @ BooleanError::ClassificationInvariant { .. }) = r {
                defects.push(format!("{name} nt e{edge} a{a} d{d:e}, {op}: {e:?}"));
            }
        }
    }
    assert!(
        defects.is_empty(),
        "{} runs refuse a kernel-defect invariant:\n{}",
        defects.len(),
        defects.join("\n")
    );
}
