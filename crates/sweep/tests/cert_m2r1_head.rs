//! CERT-M2 R1 probe (HEAD only): the structural half at Dual64 against
//! the f64 structural half, error vectors of the composed door vs the
//! structural door on corrupt bodies, and the closed-form/quadrature
//! agreement of the certified path.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::common::cert_corpus;

use geom_core::{Dual64, Tol};
use topo::{Body, validate_geometric, validate_geometric_structural};

#[test]
fn m2r1_structural_dual_matches_f64_structural_and_shows_what_the_dual_loses() {
    let tol = Tol::witness();
    let f = cert_corpus::corpus::<f64>();
    let d = cert_corpus::corpus::<Dual64>();
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
    for (n, fb) in cert_corpus::f64_only_corpus() {
        println!(
            "M2R1H|{n}|f64_structural|{:?}",
            validate_geometric_structural(&fb, tol)
        );
        println!("M2R1H|{n}|f64_composed|{:?}", validate_geometric(&fb, tol));
    }
}

/// Bitwise pin of the dual's edge intervals against f64's (the
/// geometric_cube claim, on curved bodies): a dual edge carries no
/// certificate (DL3), so its interval is what it has to compare.
#[test]
fn m2r1_dual_structural_value_channel_is_f64s() {
    let f = cert_corpus::corpus::<f64>();
    let d = cert_corpus::corpus::<Dual64>();
    for ((n, fb), (_, db)) in f.iter().zip(d.iter()) {
        let fr: Vec<(u64, u64)> = fb
            .curves()
            .filter_map(|(_, c)| c.certified().map(|c| c.params()))
            .map(|(a, b)| (a.to_bits(), b.to_bits()))
            .collect();
        let dr: Vec<(u64, u64)> = db
            .curves()
            .filter_map(|(_, c)| c.certified().map(|c| c.params()))
            .map(|(a, b)| (a.value.to_bits(), b.value.to_bits()))
            .collect();
        assert_eq!(fr, dr, "{n}: edge interval value channel");
        let _ = Body::<f64>::new;
    }
}
