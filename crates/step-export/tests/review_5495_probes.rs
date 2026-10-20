//! Review probes for PR 5495: every rotation of each fixture's stored
//! face order under the three reference rules (local harness).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;

use crate::common;

fn set_rule(v: u8) {
    topo::test_support::PROBE_RULE.store(v, std::sync::atomic::Ordering::SeqCst);
}

#[test]
fn corpus_orders() {
    let tol = Tol::witness();
    for (name, body) in common::fixture_corpus() {
        let n = {
            let mut b = body.clone();
            *topo::test_support::probe_rotate_shell_faces(&mut b, 0).iter().max().unwrap_or(&0)
        };
        for (rule, v) in [("origin", 1), ("first", 2), ("qfirst", 0)] {
            set_rule(v);
            let (mut lo, mut hi, mut errs) = (f64::INFINITY, 0.0f64, Vec::new());
            for k in 0..n {
                let mut b = body.clone();
                topo::test_support::probe_rotate_shell_faces(&mut b, k);
                match topo::mass_properties(&b, tol) {
                    Ok(m) => {
                        lo = lo.min(m.volume_pad);
                        hi = hi.max(m.volume_pad);
                    }
                    Err(e) => errs.push((k, format!("{e:.100}"))),
                }
            }
            eprintln!(
                "ORDERS eps={:e} {name} rule={rule} n={n} min={lo:.3e} max={hi:.3e} ratio={:.2} errs={} {:?}",
                tol.eps(),
                hi / lo,
                errs.len(),
                errs.first()
            );
        }
        set_rule(0);
    }
}
