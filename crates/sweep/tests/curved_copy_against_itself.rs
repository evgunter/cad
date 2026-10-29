//! A curved body against its copy.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::revert_ops::subtract_both_orders_and_intersect;
use geom_core::{Tol, Vec3};
use sweep::test_support::{
    ball_poled_z, corners, dome, finished, lantern, revolved_about_y, waisted,
};
use topo::{Body, BooleanResult};

fn bodies() -> Vec<(&'static str, Body<f64>)> {
    let tol = Tol::witness();
    vec![
        ("ball", ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol)),
        ("dome", dome(1.0, tol)),
        ("waisted", waisted(tol)),
        ("lantern", lantern(tol)),
        (
            "frustum",
            revolved_about_y(
                corners(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]),
                sweep::Revolution::Full,
                tol,
            ),
        ),
    ]
}

#[test]
#[ignore = "measurement: cargo nextest run -p sweep curved_copy_probe --run-ignored only --nocapture"]
fn curved_copy_probe() {
    let tol = Tol::witness();
    eprintln!("eps {}", tol.eps());
    for (label, a) in bodies() {
        let fa = finished("A", a.clone(), tol);
        let mut got = vec![("A ∪ B", topo::union(&fa, &fa, tol))];
        got.extend(subtract_both_orders_and_intersect(&a, &a));
        for (op, r) in got {
            let s = match r {
                Ok(BooleanResult::Body(bb)) => format!("body, {} faces", bb.body.faces().count()),
                Ok(BooleanResult::Empty) => "Empty".into(),
                Err(e) => format!("{e:?}"),
            };
            eprintln!("{label} {op}: {s}");
        }
    }
}
