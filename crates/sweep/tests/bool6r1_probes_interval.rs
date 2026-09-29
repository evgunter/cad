//! **BOOL-6 review probes (R1), the certified lane** — the per-slab
//! fold at the `interval` scalar.
//!
//! The fold sums `k − 1` displacements it did not sum before, each of
//! them `1/(k − 1)` of the old magnitude, so the question this lane
//! asks is whether an enclosure that was wide enough for the
//! end-to-end statement is still wide enough per slab. The row places
//! one interior slab at each edge of the run's band and asserts the
//! loft's answer against `decide` on the same number — the same
//! comparison the `f64` row makes, at the scalar where a lost
//! enclosure would show as an escalation the `f64` lane never sees.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Margin, Sign};
use geom_core::{Affine3, Interval, Point2, Tol, Vec3};
use profile::RawLoop;
use sweep::{LoftError, ProfileLoop, loft_body};

fn sq() -> sweep::Section {
    vec![ProfileLoop::polygon(
        [
            Point2::new(-0.05, -0.05),
            Point2::new(0.05, -0.05),
            Point2::new(0.05, 0.05),
            Point2::new(-0.05, 0.05),
        ]
        .into_iter(),
    )]
}

fn stacked(zs: &[f64]) -> Vec<Affine3<f64>> {
    zs.iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect()
}

/// **The band edges at the certified scalar.** Below ε the interior
/// slab is the loft's sliver; in the band it escalates carrying slab
/// 1; above `K·ε` it builds. Each row is checked against `decide` on
/// the step itself rather than against a written-out verdict, so the
/// row tracks whatever ε the run commits.
#[test]
fn an_interior_slab_at_the_band_edges_decides_the_same_at_interval() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let (eps, k) = (tol.eps(), tol.k());
    for step in [0.5 * eps, 0.5 * (1.0 + k) * eps, 2.0 * k * eps] {
        let sections = vec![sq(), sq(), sq(), sq()];
        let places = stacked(&[0.0, 1.0, 1.0 + step, 2.0]);
        let got = loft_body::<Interval>(&sections, &places, 2, tol).map(|_| ());
        let want = geom_core::k_stats::decide("loft_stacking", Margin::of(step), band);
        match (want, &got) {
            (Ok(Sign::Positive), Ok(())) => {}
            (Ok(Sign::Zero), Err(LoftError::DegenerateStacking { slab: 1 })) => {}
            (Ok(Sign::Negative), Err(LoftError::ReversedStacking { slab: 1 })) => {}
            (Err(_), Err(LoftError::StackingEscalated { slab: 1, .. })) => {}
            _ => panic!("interval interior step {step:e}: decide said {want:?}, loft said {got:?}"),
        }
    }
}

/// **A wholly forward stack still builds at the certified scalar**, at
/// every station count the fold now samples — the widening a `k − 1`
/// fold could have cost is measured here rather than argued.
#[test]
fn a_forward_stack_builds_at_interval_at_every_station_count() {
    let tol = Tol::witness();
    for k in [2usize, 3, 5, 9] {
        #[allow(clippy::cast_precision_loss)]
        let zs: Vec<f64> = (0..k).map(|i| i as f64).collect();
        let sections = vec![sq(); k];
        let out = loft_body::<Interval>(&sections, &stacked(&zs), k.min(4) - 1, tol);
        assert!(
            out.is_ok(),
            "the forward stack of {k} sections builds at interval"
        );
    }
}
