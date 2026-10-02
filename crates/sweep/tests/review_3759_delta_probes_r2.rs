//! PCERT delta reviewer round 2 probes on PR 3759 (head a6ca7db3d). Not for merge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::{ChartWindow, PcurveCache};
use geom_core::{Band, Point2, Tol};
use topo::pcurves::validate_pcurves;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// DELIBERATE RED PROBE (round 2). The re-statement `Body::revert` makes: EVERY row of the wall's loop
/// moved one period over, consistently. Complete it reads clean (the PR's
/// own row). Detach each half-edge in turn: does the verdict depend on
/// which one is the gap? (Gap at cycle position 0 is now carried by the
/// derived image on its principal branch.)
#[test]
fn r2_a_loop_a_period_over_with_each_gap_in_turn() {
    let base = crate::common::operands::n_arc_boss::<f64>(Point2::new(0.0, 0.0), 3, 0.0, 1.0);
    let (wall, cycle) = base
        .faces()
        .find_map(|(fk, f)| {
            if !matches!(
                base.get_surface(f.surface).unwrap(),
                Surface::Cylinder { .. }
            ) {
                return None;
            }
            let topo::LoopBoundary::Cycle { first } = base.get_loop(f.outer).unwrap().boundary
            else {
                return None;
            };
            Some((fk, base.loop_cycle(first).unwrap()))
        })
        .unwrap();
    let surface = base
        .get_surface(base.get_face(wall).unwrap().surface)
        .unwrap()
        .clone();
    let tau = core::f64::consts::TAU;
    let mut moved = base.clone();
    for &he in &cycle {
        let cache = base.pcurve(he).unwrap();
        let shifted = cache
            .pcurve()
            .map_affine(|p| Point2::new(p.x + tau, p.y), |v| v);
        let edge = base.get_edge(base.get_half_edge(he).unwrap().edge).unwrap();
        let carrier = base
            .get_curve_geom(edge.curve)
            .unwrap()
            .certified()
            .unwrap()
            .carrier()
            .clone();
        let (t0, t1) = cache.params();
        let window = ChartWindow {
            u_min: -2.0 * tau,
            u_max: 3.0 * tau,
            v_min: -5.0,
            v_max: 5.0,
        };
        let row =
            PcurveCache::certify(shifted, t0, t1, &carrier, &surface, window, band()).unwrap();
        moved.attach_pcurve(he, row);
    }
    let complete = validate_pcurves(&moved, band());
    eprintln!("complete: {complete:?}");
    let mut loud = Vec::new();
    for (j, &gap) in cycle.iter().enumerate() {
        let mut b = moved.clone();
        b.detach_pcurve(gap);
        let f = validate_pcurves(&b, band());
        eprintln!("gap {j}: {f:?}");
        if f.iter()
            .any(|e| !matches!(e, topo::PcurveMintError::MissingCache { .. }))
        {
            loud.push(j);
        }
    }
    assert!(complete.is_empty(), "complete: {complete:?}");
    assert!(
        loud.is_empty(),
        "a valid period-moved loop reads loud with the gap at {loud:?}"
    );
}
