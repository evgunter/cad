//! CERT-M2 R1 probe (HEAD only): the structural half at Dual64 against
//! the f64 structural half, error vectors of the composed door vs the
//! structural door on corrupt bodies, and the closed-form/quadrature
//! agreement of the certified path.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::cert_m2r1_passes as passes;

use geom_core::{Dual64, Tol};
use topo::{Body, validate_geometric, validate_geometric_structural};

#[test]
fn m2r1_structural_dual_matches_f64_structural_and_shows_what_the_dual_loses() {
    let tol = Tol::witness();
    let f = passes::corpus::<f64>();
    let d = passes::corpus::<Dual64>();
    assert_eq!(f.len(), d.len());
    for ((n, fb), (_, db)) in f.iter().zip(d.iter()) {
        let sf = validate_geometric_structural(fb, tol);
        let sd = validate_geometric_structural(db, tol);
        let cf = validate_geometric(fb, tol);
        println!("M2R1H|{n}|f64_structural|{sf:?}");
        println!("M2R1H|{n}|dual_structural|{sd:?}");
        println!("M2R1H|{n}|f64_composed|{cf:?}");
        assert_eq!(
            sf, sd,
            "{n}: structural half differs between f64 and Dual64"
        );
    }
    for (n, fb) in passes::f64_only_corpus() {
        println!(
            "M2R1H|{n}|f64_structural|{:?}",
            validate_geometric_structural(&fb, tol)
        );
        println!("M2R1H|{n}|f64_composed|{:?}", validate_geometric(&fb, tol));
    }
}

/// Bitwise pin of the dual's structural-half certificates against f64's
/// (the geometric_cube claim, on curved bodies).
#[test]
fn m2r1_dual_structural_value_channel_is_f64s() {
    let f = passes::corpus::<f64>();
    let d = passes::corpus::<Dual64>();
    for ((n, fb), (_, db)) in f.iter().zip(d.iter()) {
        let fr: Vec<u64> = fb
            .curves()
            .filter_map(|(_, c)| {
                c.certified()
                    .map(|c| c.certificate().max_residual.to_bits())
            })
            .collect();
        let dr: Vec<u64> = db
            .curves()
            .filter_map(|(_, c)| {
                c.certified()
                    .map(|c| c.certificate().max_residual.value.to_bits())
            })
            .collect();
        assert_eq!(fr, dr, "{n}: certificate value channel");
        let _ = Body::<f64>::new;
    }
}
