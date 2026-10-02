//! Delta-review probe for PR #3844 at ae4b1dbe: the rounded stack's
//! planted ∩ result (B thickened by δ) through the backstop, at three
//! scales and two rotations. Wire into `crates/sweep/tests/all.rs` as
//! `#[path = "delta3844_sweep.rs"] mod delta3844_sweep;`. Compiles
//! unchanged on `origin/main`. Outline after r2's `reach3844_r2_probe.rs`.
#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout, clippy::unwrap_used)]

use geom_core::{Point2, Tol};
use profile::{Open, ProfileLoop, Start};
use sweep::test_support::{extruded, sketch_at};
use topo::{BooleanError, BooleanOp};

fn tol() -> Tol {
    Tol::witness()
}

fn rounded(s: f64, th: f64) -> ProfileLoop<f64> {
    let t = tol();
    let (w, h, r) = (6.0 * s, 4.0 * s, 0.5 * s);
    let (c, sn) = (th.cos(), th.sin());
    let rot = |x: f64, y: f64| Point2::new(c * x - sn * y, sn * x + c * y);
    let dir = |x: f64, y: f64| (c * x - sn * y, sn * x + c * y);
    let (d0, d1, d2, d3) = (dir(1.0, 0.0), dir(0.0, 1.0), dir(-1.0, 0.0), dir(0.0, -1.0));
    Open.at(rot(w / 2.0, 0.0))
        .toward(d0.0, d0.1, t).unwrap().fillet(r, t).unwrap()
        .at(rot(w, h / 2.0), t).unwrap()
        .toward(d1.0, d1.1, t).unwrap().fillet(r, t).unwrap()
        .at(rot(w / 2.0, h), t).unwrap()
        .toward(d2.0, d2.1, t).unwrap().fillet(r, t).unwrap()
        .at(rot(0.0, h / 2.0), t).unwrap()
        .toward(d3.0, d3.1, t).unwrap().fillet(r, t).unwrap()
        .to(Start, t).unwrap()
        .into()
}

fn tag(r: &Result<topo::AtRestOutcome, BooleanError>) -> String {
    match r {
        Ok(_) => "PASS".into(),
        Err(BooleanError::ResultVolumeImplausible { .. }) => "refuse".into(),
        Err(e) => format!("other[{}]", format!("{e:?}").split([' ', '{', '(']).next().unwrap()),
    }
}

#[test]
fn ds_rounded_stack_thickened() {
    println!("DS eps={:e}", tol().eps());
    for s in [1e-3, 1.0, 1e3] {
        for th in [0.0, 0.3] {
            let a = extruded(sketch_at(0.0), vec![rounded(s, th)], s, tol());
            for (pose, z0) in [("sunk", 0.25 * s), ("top", 0.5 * s)] {
                let b = extruded(sketch_at(z0), vec![rounded(s, th)], 0.5 * s, tol());
                let gate = |d: f64| {
                    let wrong = extruded(sketch_at(z0), vec![rounded(s, th)], 0.5 * s + d, tol());
                    topo::test_support::volume_backstop(BooleanOp::Intersect, &a, &b, &wrong, tol())
                };
                let mut line = format!("DS s={s:e} th={th} {pose}: δ=0 {}", tag(&gate(0.0)));
                for k in [1.4e-8, 4e-9, 1e-12] {
                    line.push_str(&format!(" | δ={k:e}·s {}", tag(&gate(k * s))));
                }
                // the smallest δ that refuses: the backstop's resolution here
                let (mut lo, mut hi) = (0.0f64, 1e-6 * s);
                if gate(hi).is_ok() {
                    line.push_str(" | 1e-6·s PASSES");
                } else {
                    for _ in 0..60 {
                        let mid = 0.5 * (lo + hi);
                        if gate(mid).is_ok() { lo = mid } else { hi = mid }
                    }
                    line.push_str(&format!(" | smallest refused δ/s {:.2e}", hi / s));
                }
                println!("{line}");
            }
        }
    }
}

/// Every op of the rounded stack through the REAL door, one line each,
/// so a mutant's verdict on each pose/op is visible on its own (the
/// committed row stops at its first red).
#[test]
fn ds_rounded_stack_each_op() {
    let a = extruded(sketch_at(0.0), vec![rounded(1.0, 0.0)], 1.0, tol());
    for (pose, z0) in [("sunk", 0.25), ("top", 0.5), ("bottom", 0.0)] {
        let b = extruded(sketch_at(z0), vec![rounded(1.0, 0.0)], 0.5, tol());
        let dab = topo::test_support::flush_declarations(&a, &b, tol());
        let dba = topo::test_support::flush_declarations(&b, &a, tol());
        let cell = |r: Result<topo::BooleanResult<f64>, BooleanError>| match r {
            Ok(topo::BooleanResult::Body(bb)) => format!("{:.17}", topo::mass_properties(&bb.body, tol()).unwrap().volume),
            Ok(topo::BooleanResult::Empty) => "empty".into(),
            Err(BooleanError::ResultVolumeImplausible { which, .. }) => format!("REFUSE[{which}]"),
            Err(e) => format!("{}", format!("{e:?}").split([' ', '{', '(']).next().unwrap()),
        };
        println!(
            "DE {pose}: A∩B {} | A∖B {} | B∪A {} | B∖A {}",
            cell(topo::intersect_with(&a, &b, &dab, tol())),
            cell(topo::subtract_with(&a, &b, &dab, tol())),
            cell(topo::union_with(&b, &a, &dba, tol())),
            cell(topo::subtract_with(&b, &a, &dba, tol())),
        );
    }
}
