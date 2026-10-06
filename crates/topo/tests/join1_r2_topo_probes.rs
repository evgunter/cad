//! JOIN-1 review (lane r2): the newly building multi-spike corner meet,
//! held to tier 3′ and the at-rest certificate as well as its volume,
//! in both operand orders and under every op.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{brick, finished, flush_declarations, prism_z};
use geom_core::Tol;
use topo::BooleanResult;

#[test]
fn r2_multi_spike_corner_meet_is_tier_three() {
    let t = Tol::witness();
    let a = finished("a", brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), t), t);
    let b = finished("b", brick::<f64>((1.0, 3.0), (1.0, 3.0), (0.0, 1.0), t), t);
    let ab = match topo::intersect_with(&a, &b, &flush_declarations(&a, &b, t), t).unwrap() {
        BooleanResult::Body(bb) => bb.body,
        BooleanResult::Empty => panic!(),
    };
    let c = prism_z::<f64>(
        &[(1.0, 1.0), (2.0, 0.0), (3.0, 1.0), (2.0, 2.0)],
        0.0,
        1.0,
        t,
    )
    .body;
    let c = finished("c", c, t);
    let (vab, vc, ov) = (1.0, 2.0, 0.5);
    let mut wrong = Vec::new();
    for (ord, x, y, vx, vy) in [("AB·C", &ab, &c, vab, vc), ("C·AB", &c, &ab, vc, vab)] {
        let d = flush_declarations(x, y, t);
        for (op, r, want) in [
            ("∩", topo::intersect_with(x, y, &d, t), ov),
            ("∪", topo::union_with(x, y, &d, t), vx + vy - ov),
            ("∖", topo::subtract_with(x, y, &d, t), vx - ov),
        ] {
            let what = format!("{ord} {op}");
            match r {
                Err(e) => println!("[r2] {what}: REFUSES {e:?}"),
                Ok(BooleanResult::Empty) => println!("[r2] {what}: EMPTY"),
                Ok(BooleanResult::Body(bb)) => {
                    let mut bad = Vec::new();
                    if let Err(e) = topo::validate_closed(&bb.body) {
                        bad.push(format!("tier 2 {e:?}"));
                    }
                    if let Err(e) = topo::validate_pseudomanifold(&bb.body, &bb.contacts, t) {
                        bad.push(format!("tier 3′ {e:?}"));
                    }
                    if let Err(e) = topo::validate_geometric_certificate(&bb.body, t) {
                        bad.push(format!("cert {e:?}"));
                    }
                    let v = topo::mass_properties(&bb.body, t).unwrap().volume;
                    if (v - want).abs() > 1e-12 {
                        bad.push(format!("volume {v} against {want}"));
                    }
                    println!("[r2] {what}: BUILDS {bad:?}");
                    if !bad.is_empty() {
                        wrong.push(format!("{what}: {bad:?}"));
                    }
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
