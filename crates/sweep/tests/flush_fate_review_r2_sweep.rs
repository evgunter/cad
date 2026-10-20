//! Review probe (R2), base-compatible: flush pairs over a grid of
//! offsets, every op in both orders, one outcome line per run, so the
//! refusal sets of two trees can be diffed.
//!
//! `cargo test -p sweep --release --test all flush_fate_review_r2_sweep -- --ignored --nocapture`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use sweep::test_support as st;
use topo::{AtRestBody, BooleanResult, intersect, mass_properties, subtract, union, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    topo::test_support::finished("brick", st::brick(x, y, z, tol()), tol())
}

fn line(tag: &str, r: Result<BooleanResult<f64>, topo::BooleanError>) {
    let t = tol();
    let s = match r {
        Ok(BooleanResult::Body(o)) => format!(
            "ok vol {:.6} t3 {}",
            mass_properties(&o.body, t).map(|m| m.volume).unwrap_or(f64::NAN),
            validate_geometric(&o.body, t).is_ok()
        ),
        Ok(BooleanResult::Empty) => "empty".into(),
        Err(e) => format!("ERR {e:?}"),
    };
    println!("{tag}: {s}");
}

#[test]
#[ignore = "review probe"]
fn flush_fate_review_r2_sweep() {
    let t = tol();
    let e = t.eps();
    let u = (0.0, 1.0);
    let ds = [0.0, 0.3 * e, 3.0 * e, 30.0 * e, 1e-4, 0.25];
    let x = brick((0.0, 2.0), (0.0, 2.0), u);
    let lshape = topo::test_support::finished(
        "L",
        st::prism(
            [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)]
                .iter()
                .map(|&(a, b)| (Point2::new(a, b), 0.0))
                .collect(),
            1.0,
            t,
        ),
        t,
    );
    let ushape = topo::test_support::finished(
        "U",
        st::prism(
            [(0.0, 0.0), (3.0, 0.0), (3.0, 2.0), (2.0, 2.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)]
                .iter()
                .map(|&(a, b)| (Point2::new(a, b), 0.0))
                .collect(),
            1.0,
            t,
        ),
        t,
    );
    for (bname, base) in [("box", &x), ("L", &lshape), ("U", &ushape)] {
        for &dx in &ds {
            for &dy in &ds {
                for (shape, y) in [
                    ("over", brick((1.0 + dx, 3.0 + dx), (1.0 + dy, 3.0 + dy), u)),
                    ("in", brick((0.5 + dx, 1.5 + dx), (0.5 + dy, 1.5 + dy), (0.5, 1.0))),
                    ("strip", brick((-0.5 + dx, 3.5 + dx), (1.5 + dy, 1.8 + dy), (0.5, 1.0))),
                    ("on", brick((1.0 + dx, 3.0 + dx), (1.0 + dy, 3.0 + dy), (1.0, 2.0))),
                ] {
                    let tag = format!("{bname} {shape} dx={dx:e} dy={dy:e}");
                    line(&format!("{tag} x∪y"), union(base, &y, t));
                    line(&format!("{tag} y∪x"), union(&y, base, t));
                    line(&format!("{tag} x∩y"), intersect(base, &y, t));
                    line(&format!("{tag} y∩x"), intersect(&y, base, t));
                    line(&format!("{tag} x−y"), subtract(base, &y, t));
                    line(&format!("{tag} y−x"), subtract(&y, base, t));
                }
            }
        }
    }
}
