//! NEEDLE-DESIGN-X probes (not pins): needles 1 × w × w, both
//! orientations, at rest; and the volume's enclosure on each.
#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::Tol;
use topo::{AtRestBody, classify_shells};

use crate::common;

fn short<E: core::fmt::Debug>(e: &E) -> String {
    let s = format!("{e:?}");
    s.chars().take(220).collect()
}

#[test]
#[ignore = "design probe"]
fn ndx_needles() {
    let tol = Tol::witness();
    let e = tol.eps();
    for m in [10.5, 12.0, 15.0, 21.0, 30.0] {
        let w = m * e;
        let Ok(raw) = std::panic::catch_unwind(|| common::brick::<f64>((0.0, 1.0), (0.0, w), (0.0, w), tol)) else {
            println!("NDXN eps={e:e} m={m} does not build");
            continue;
        };
        let props = topo::mass_properties(&raw, tol);
        let vol = props.as_ref().map(|p| (p.volume, p.volume_pad)).map_err(short);
        let solo = AtRestBody::validate(raw.clone(), tol).map(|_| ());
        let inv = AtRestBody::validate(raw.revert(), tol).map(|_| ());
        println!(
            "NDXN eps={e:e} m={m} vol(V,pad)={vol:?}\n   solo={}\n   inv={}\n   classify={}",
            short(&solo),
            short(&inv),
            short(&classify_shells(&raw, tol).map(|c| c.iter().map(|c| c.role).collect::<Vec<_>>())),
        );
    }
}
