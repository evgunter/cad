//! Review probe for `work/cleave/a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop.md`:
//! a union of `[0,1]³` and a brick resting a gap `g` above its top.
#![allow(
    clippy::panic,
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::print_stdout
)]

use crate::common;
use common::{brick, finished};
use geom_core::Tol;
use topo::{BooleanOp, BooleanResult};

#[test]
fn cleave_zero_band_gap_table() {
    let tol = Tol::witness();
    let a: topo::AtRestBody<f64> =
        finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol), tol);
    let feet = [
        ("[.25,.75]^2", (0.25, 0.75), (0.25, 0.75)),
        ("[0,1]^2", (0.0, 1.0), (0.0, 1.0)),
        ("[-.5,.5]x[.25,.75]", (-0.5, 0.5), (0.25, 0.75)),
    ];
    let z0s = [
        ("0", 1.0),
        ("1ulp", f64::from_bits(1.0f64.to_bits() + 1)),
        ("4ulp", f64::from_bits(1.0f64.to_bits() + 4)),
        ("1e-15", 1.0 + 1e-15),
        ("1e-13", 1.0 + 1e-13),
        ("1e-11", 1.0 + 1e-11),
        ("1e-10", 1.0 + 1e-10),
        ("5e-10", 1.0 + 5e-10),
        ("9e-10", 1.0 + 9e-10),
        ("-1ulp", f64::from_bits(1.0f64.to_bits() - 1)),
        ("-1e-12", 1.0 - 1e-12),
        ("-1e-10", 1.0 - 1e-10),
    ];
    for (fname, x, y) in feet {
        for (gname, z0) in z0s {
            let b: topo::AtRestBody<f64> = finished("b", brick(x, y, (z0, z0 + 1.0), tol), tol);
            for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
                let r = topo::boolean_op_with(
                    op,
                    &a,
                    &b,
                    &Default::default(),
                    topo::SweepStrategy::Realized,
                    tol,
                );
                let s = match r {
                    Ok(BooleanResult::Body(_)) => "Ok".to_owned(),
                    Ok(BooleanResult::Empty) => "Empty".to_owned(),
                    Err(e) => format!("{e:?}").chars().take(140).collect(),
                };
                println!("PROBE foot={fname} g={gname} {op:?}: {s}");
            }
        }
    }
}
