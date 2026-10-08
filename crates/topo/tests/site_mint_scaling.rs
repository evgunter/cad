//! **How the site mint's cost grows with the loop it lands on.** The
//! minted cylinder-wall sheet's bottom rim is split `n` times, then a
//! strut is grown up the ruling at every split vertex: `n` operators on
//! one minted face whose outer loop grows by two halves with each.
//! Each phase is timed and printed per `n`, so a run reads off how the
//! per-operator cost scales with the loop; the face is checked complete
//! and valid at the end.
//!
//! Evidence, not a gate (timings on a shared box are shape only):
//! `cargo nextest run -p topo [--release] --test all site_mint_scaling
//! --run-ignored only --no-capture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::Instant;

use geom_core::{Band, Tol};
use topo::pcurves::validate_pcurves;
use topo::test_support::{CylFrame, cyl_wall_sheet};
use topo::{Body, HalfEdgeKey, MevSite};

#[test]
#[ignore = "timing evidence; run with --run-ignored only --no-capture"]
fn site_mint_scaling() {
    let tol = Tol::witness();
    let frame = CylFrame::canonical(1.0);
    for n in [25_usize, 50, 100, 200, 400] {
        let mut body = Body::<f64>::new();
        let wall = cyl_wall_sheet(&mut body, frame, None, (0.2, 1.4), (0.0, 1.0), tol);
        let rim = body
            .edges()
            .map(|(e, _)| e)
            .find(|&e| {
                let c = body
                    .get_curve_geom(body.get_edge(e).unwrap().curve)
                    .and_then(topo::CurveGeom::certified)
                    .unwrap();
                matches!(c.carrier(), geom::Curve3::Circle { .. }) && c.params() == (0.2, 1.4)
            })
            .unwrap();
        let on_wall = |body: &Body<f64>, he: HalfEdgeKey| {
            let lp = body.get_half_edge(he).unwrap().parent_loop;
            body.get_loop(lp).unwrap().face == wall
        };
        let azimuth = |k: usize| 1.4 - (k + 1) as f64 * 1.2 / (n + 1) as f64;
        // One surgery scope over both phases, as a producer running
        // many operators opens: the tier-1 sweep, O(body) per door, is
        // paid once at the close and timed apart, so the phases time
        // the operators alone.
        let mut scope = body.begin_surgery();
        let started = Instant::now();
        let made: Vec<_> = (0..n)
            .map(|k| scope.split_edge(rim, azimuth(k), tol).unwrap())
            .collect();
        let splits = started.elapsed();
        // The wall's half leaving split vertex `k`: the second child's
        // plus half where that half is on the wall, else the half after
        // its minus half, which ends there.
        let sites: Vec<(HalfEdgeKey, f64)> = made
            .iter()
            .enumerate()
            .map(|(k, split)| {
                let leaving = if on_wall(&scope, split.he_plus) {
                    split.he_plus
                } else {
                    scope.get_half_edge(split.he_minus).unwrap().next
                };
                assert_eq!(scope.get_half_edge(leaving).unwrap().start, split.vertex);
                (leaving, azimuth(k))
            })
            .collect();
        let started = Instant::now();
        for &(he, u) in &sites {
            scope
                .mev_line(MevSite::Fan { he1: he, he2: he }, frame.at(u, 0.5), tol)
                .unwrap();
        }
        let struts = started.elapsed();
        let started = Instant::now();
        scope.sweep_and_close();
        let sweep = started.elapsed();
        println!(
            "n = {n:>3}: {n} rim splits {:.4} s, {n} struts {:.4} s, one tier-1 sweep {:.4} s",
            splits.as_secs_f64(),
            struts.as_secs_f64(),
            sweep.as_secs_f64()
        );
        let band = Band::linear(tol).unwrap();
        assert_eq!(
            validate_pcurves(&body, band),
            vec![],
            "n = {n}: the wall is valid at tier 3"
        );
        let face = body.get_face(wall).unwrap();
        let topo::LoopBoundary::Cycle { first } = body.get_loop(face.outer).unwrap().boundary
        else {
            panic!("the wall's outer loop is a cycle")
        };
        let cycle = body.loop_cycle(first).unwrap();
        assert!(
            cycle.iter().all(|&he| body.pcurve(he).is_some()),
            "n = {n}: every half of the wall's grown loop stores a row"
        );
    }
}
