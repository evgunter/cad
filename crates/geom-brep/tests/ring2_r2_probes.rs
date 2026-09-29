//! **RING-2 review probes (R2 lane), consumer side.** The register's
//! division-reachable shape driven into a public consumer that reads
//! one endpoint: `offset_meters::mig`. At the merge base a refused
//! quotient was a NaN pair and `mig` answered `0` by accident (every
//! comparison false); at this head the quotient carries real endpoints
//! and `mig` answers `0` because it asks the refusal by name.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::offset_meters::mig;
use geom_core::Bounds;
use geom_core::Interval;

fn ri(lo: f64, hi: f64) -> Interval {
    Interval::from_bounds(lo, hi)
}

#[test]
fn the_mignitude_refuses_a_zero_touching_quotient_by_name() {
    // A refused quotient whose every endpoint would pass the
    // comparison `mig` writes.
    let q = ri(1.0, 2.0) / ri(0.0, 5e-324);
    assert!(!q.is_certified(), "{q:?}");
    assert!(
        q.lo() > 0.0,
        "the hazard: an unguarded `lo() > 0` is TRUE here ({q:?})"
    );
    assert_eq!(mig(q), 0.0, "the mignitude must answer the refusing 0");
    let n = ri(-2.0, -1.0) / ri(0.0, 5e-324);
    assert!(!n.is_certified() && n.hi() < 0.0, "{n:?}");
    assert_eq!(mig(n), 0.0);
    // And the finite two-sided refusal, through a multiply.
    let f = (ri(1.0, 2.0) / ri(-1.0, 1.0)) * Interval::zero();
    assert!(!f.is_certified(), "{f:?}");
    assert_eq!(mig(f), 0.0);
    // A clean enclosure still reads its mignitude.
    assert_eq!(mig(ri(2.0, 3.0)), 2.0);
    assert_eq!(mig(ri(-3.0, -2.0)), 2.0);
    assert_eq!(mig(ri(-1.0, 1.0)), 0.0);
}
