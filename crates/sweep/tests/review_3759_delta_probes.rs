//! PCERT delta reviewer probes on PR 3759 (head f98f60c05). Not for merge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::PcurveCache;
use geom_core::{Band, Tol};
use topo::PcurveMintError;
use topo::pcurves::validate_pcurves;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// DELIBERATE RED PROBE. One stored row restated a whole carrier period
/// over (interval and image both shifted by 2π): its ends evaluate to its
/// edge's vertices, so `RowInterval` reads it equal, and it certifies.
/// Only one-branch continuity can see it. With one other row detached,
/// which (row, shift, gap) choices does tier 3 report nothing but the gap?
/// At f98f60c05: (0, ±, 1). The row at cycle position 0 with its
/// successor detached: the wrap joint (last row -> first row) is checked
/// only through closure, which a half-minted loop skips, so the same
/// shift is silent at position 0 and loud at position 2.
/// (A row WIDENED by a period refuses `AzimuthPeriodExceeded` at certify,
/// so that shape cannot be built.)
#[test]
fn delta_a_row_widened_by_a_whole_period() {
    let base =
        crate::common::operands::n_arc_boss::<f64>(geom_core::Point2::new(0.0, 0.0), 3, 0.0, 1.0);
    let (wall, cycle) = base
        .faces()
        .find_map(|(fk, f)| {
            if !matches!(base.get_surface(f.surface).unwrap(), Surface::Cylinder { .. }) {
                return None;
            }
            let topo::LoopBoundary::Cycle { first } = base.get_loop(f.outer).unwrap().boundary
            else {
                return None;
            };
            Some((fk, base.loop_cycle(first).unwrap()))
        })
        .unwrap();
    let surface = base.get_surface(base.get_face(wall).unwrap().surface).unwrap().clone();
    let tau = core::f64::consts::TAU;
    let mut silent = Vec::new();
    let mut total = 0;
    for (i, &h1) in cycle.iter().enumerate() {
        let edge = base.get_edge(base.get_half_edge(h1).unwrap().edge).unwrap();
        let carrier = base.get_curve_geom(edge.curve).unwrap().certified().unwrap().carrier().clone();
        if !matches!(carrier, Curve3::Circle { .. }) {
            continue;
        }
        let cache = base.pcurve(h1).unwrap().clone();
        let (t0, t1) = cache.params();
        for (end, (lo, hi)) in [("shift+", (t0 + tau, t1 + tau)), ("shift-", (t0 - tau, t1 - tau))] {
            for (j, &h2) in cycle.iter().enumerate() {
                let mut body = base.clone();
                let wide = cache.pcurve().clone();
                let window = wide.chart_box(lo, hi);
                let row = PcurveCache::certify(wide, lo, hi, &carrier, &surface, window, band())
                    .expect("certifies one period over");
                body.attach_pcurve(h1, row);
                if j != i {
                    body.detach_pcurve(h2);
                }
                let f = validate_pcurves(&body, band());
                let names_h1 = f.iter().any(|e| match e {
                    PcurveMintError::RowInterval { half_edge }
                    | PcurveMintError::LoopDiscontinuity { half_edge }
                    | PcurveMintError::Certify { half_edge, .. }
                    | PcurveMintError::Escalated { half_edge, .. } => *half_edge == h1,
                    _ => false,
                });
                let any_but_gap = f.iter().any(|e| !matches!(e, PcurveMintError::MissingCache { .. }));
                eprintln!("row {i} end {end} gap {j}: {f:?}");
                if !any_but_gap {
                    silent.push((i, end, j));
                }
                let _ = names_h1;
                total += 1;
            }
        }
    }
    eprintln!("silent (only the gap reported, or nothing): {silent:?} of {total}");
    assert!(silent.is_empty(), "a period-wide stale row slips: {silent:?} of {total}");
}
