//! The twisted loft's wall offset fit: a bilinear saddle, whose fit
//! bound falls at the bicubic rate and needs more samples in one
//! direction than in the other.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::{band, nurbs_walls, twisted_loft};
use geom_brep::offset_fit::{OFFSET_FIT_SAMPLE_CAP, fit_offset, fit_offset_at};
use geom_brep::{OffsetFitError, OffsetFitWall};
use geom_core::Tol;

/// **The sample cap is a total, not a per-direction count.** At
/// `d = ±0.05` and 1e-9 the wall certifies on a 49 x 26 grid, round 9:
/// 49 samples in `u` alone, which a per-direction cap at
/// `sqrt(OFFSET_FIT_SAMPLE_CAP)` refuses on the grid before (41 x 26,
/// 1.008e-9 m, 0.8% over the tolerance). The certified bound is
/// `3.8503e-10`: `3.8534e-10` before the regularity floor became the
/// area quotient's inf (PR 4984), which floors this integral wall at
/// `1.97751` on the 64-rung cells against a sampled least
/// `‖S_u × S_v‖` of `1.97754` (`1.97616` before), and the
/// tangential term divides by that floor.
#[test]
fn a_saddle_wall_certifies_on_a_grid_longer_in_one_direction_than_a_square_cap() {
    let walls = nurbs_walls(&twisted_loft(0.3));
    let (_, wall) = &walls[0];
    for d in [0.05, -0.05] {
        let (fit, cert) = fit_offset_at(wall, d, 1e-9, band())
            .unwrap_or_else(|e| panic!("d = {d}: the saddle wall's fit refused at 1e-9: {e}"));
        let grid = fit.control_counts();
        assert_eq!((grid, cert.rounds), ((49, 26), 9), "d = {d}");
        assert!(
            grid.0 * grid.0 > OFFSET_FIT_SAMPLE_CAP && grid.0 * grid.1 <= OFFSET_FIT_SAMPLE_CAP,
            "d = {d}: the grid {grid:?} no longer outgrows a square of the cap in one direction"
        );
        assert!(
            (cert.hull_sup - 3.8503e-10).abs() < 1e-13,
            "d = {d}: the wall certifies at {:e}",
            cert.hull_sup
        );
    }
}

/// **A wall's fit at the run's ε, at both thicknesses and signs.** The
/// four walls are one saddle by symmetry, and the bound is linear in
/// `|d|` and even in its sign, so one wall at `d = ±0.05` and
/// `d = ±0.01` stands for the rest. Each certifies wherever ε is above
/// what the cap's last grid reaches, 62 x 35 on round 10: 1.3958e-10 m
/// at `|d| = 0.05` and 2.7918e-11 m at `|d| = 0.01`. Below that each
/// stops at the cap, carrying that bound.
#[test]
fn a_saddle_wall_fits_at_the_runs_eps_or_stops_at_the_cap() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let walls = nurbs_walls(&twisted_loft(0.3));
    let (_, wall) = &walls[0];
    for (d, reach) in [
        (0.05, 1.3958e-10),
        (-0.05, 1.3958e-10),
        (0.01, 2.7918e-11),
        (-0.01, 2.7918e-11),
    ] {
        match fit_offset(wall, d, tol) {
            Ok((_, cert)) => assert!(
                eps >= reach && cert.hull_sup <= eps,
                "eps {eps:e}, d = {d}: certified at {:e}",
                cert.hull_sup
            ),
            Err(OffsetFitError::WallReached {
                wall: OffsetFitWall::SampleCap { .. },
                grid,
                achieved,
                ..
            }) => assert!(
                eps < reach && grid == (62, 35) && (achieved - reach).abs() < reach * 1e-3,
                "eps {eps:e}, d = {d}: the cap stopped {grid:?} at {achieved:e}"
            ),
            Err(e) => panic!("eps {eps:e}, d = {d}: {e:?}"),
        }
    }
}
