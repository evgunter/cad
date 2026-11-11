#![allow(dead_code)]
mod common {
    #[path = "/home/user/cad/crates/sweep/tests/common/quadrature.rs"]
    pub mod quadrature;
    #[path = "/home/user/cad/crates/sweep/tests/common/donut_slices.rs"]
    pub mod donut_slices;
}
use common::donut_slices::{volume, Op, Rod};
fn main() {
    let rows = [
        ("shadow U", Op::Union, Rod::AlongZ { at: (2.0, 0.0), rc: 0.2 }),
        ("offaxis I", Op::Intersect, Rod::AlongZ { at: (1.9, 0.5), rc: 0.2 }),
        ("across I", Op::Intersect, Rod::AlongZ { at: (2.0, 0.0), rc: 0.7 }),
        ("inner I", Op::Intersect, Rod::AlongZ { at: (1.5, 0.0), rc: 0.2 }),
        ("square S", Op::Subtract, Rod::AlongX { x: (-1.0, 1.0), at: (2.45, 0.0), rc: 0.1 }),
        ("radial both", Op::Subtract, Rod::AlongX { x: (1.0, 3.0), at: (0.0, 0.0), rc: 0.2 }),
        ("radial inner", Op::Subtract, Rod::AlongX { x: (1.0, 2.0), at: (0.0, 0.0), rc: 0.2 }),
        ("radial outer", Op::Subtract, Rod::AlongX { x: (2.0, 3.0), at: (0.0, 0.0), rc: 0.2 }),
        ("radial off", Op::Subtract, Rod::AlongX { x: (1.0, 3.0), at: (0.15, 0.1), rc: 0.2 }),
    ];
    for (n, op, rod) in rows { println!("{n} {:.17e}", volume(op, rod)); }
}
