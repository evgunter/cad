//! **An arc-bearing loft builds at every section spacing and v-degree.**
//!
//! `common::arc_section` lofted with pure `+z` translations: three line
//! walls and one rational arc wall, whose two corner columns are the
//! seams it shares with its line neighbours. `topo::mint_pcurves`
//! certifies each seam by one spline space — the line wall's boundary
//! column and the arc wall's corner column must carry the same knots
//! and the same weights, compared exactly — so the arc wall's corner
//! columns have to come out of the skin at exactly the `1.0` the line
//! wall holds. They do because `skin_on` picks its lane per control
//! row: a row integral in every section is solved in Cartesian ℝ³.
//!
//! The spacings are the variable: evenly stacked sections parameterize
//! at exact `k/(n − 1)`, and every other spacing parameterizes off that
//! grid, which is where a homogeneous solve of the corner row lands an
//! ulp off `1.0`.

// Panicking is a test's failure mechanism (workspace lint policy).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{arc_section, stacked};
use geom_core::{Interval, Tol};
use sweep::test_support::{lift_places, lift_sections};
use sweep::{Section, loft_body};

/// `n` copies of the arc section.
fn sections(n: usize) -> Vec<Section> {
    (0..n).map(|_| arc_section(1.0)).collect()
}

/// `n` heights over `[0, 2]`: evenly spaced, on the quadratic
/// `0.6t + 0.4t²`, and evenly spaced with each interior height moved
/// by `0.15·sin(7k)/n`.
#[allow(clippy::cast_precision_loss)]
fn spacing(kind: &str, n: usize) -> Vec<f64> {
    (0..n)
        .map(|k| {
            let t = k as f64 / (n - 1) as f64;
            2.0 * match kind {
                "uniform" => t,
                "quadratic" => 0.6 * t + 0.4 * t * t,
                "jittered" if k == 0 || k == n - 1 => t,
                "jittered" => t + 0.15 / n as f64 * ((7 * k) as f64).sin(),
                _ => unreachable!("no spacing named {kind}"),
            }
        })
        .collect()
}

/// Lofts at `z` and v-degree `deg`, then validates through tier 3.
fn lofts_and_validates(what: &str, z: &[f64], deg: usize) {
    let lofted = loft_body::<f64>(&sections(z.len()), &stacked(z, 1.0), deg, Tol::witness())
        .unwrap_or_else(|e| panic!("{what}: the arc loft at z = {z:?}, degree {deg} refuses: {e:?}"));
    topo::validate_geometric(&lofted.body, Tol::witness())
        .unwrap_or_else(|e| panic!("{what}: tier 3 refuses the arc loft at z = {z:?}: {e:?}"));
}

/// The spacings the defect was found at — two that refused, one that
/// built either way — at v-degree 2.
#[test]
fn the_carve_spacings_loft_and_validate() {
    for z in [
        &[0.0, 1.0, 3.0][..],
        &[0.0, 0.25, 0.7, 1.3, 2.0, 2.1],
        &[0.0, 0.3, 1.1, 2.0],
    ] {
        lofts_and_validates("carve row", z, 2);
    }
}

/// Degrees 2 and 3, `n = 3..=9` sections, three spacings: every cell
/// lofts and validates.
#[test]
fn every_spacing_lofts_at_degree_two_and_three() {
    for deg in [2, 3] {
        for kind in ["uniform", "quadratic", "jittered"] {
            for n in (deg + 1).max(3)..=9 {
                lofts_and_validates(&format!("{kind} n={n} degree {deg}"), &spacing(kind, n), deg);
            }
        }
    }
}

/// Lofts evenly stacked sections at `Interval`, at heights `0, 1, …,
/// n − 1`.
fn lofts_at_interval(n: usize, deg: usize) {
    #[allow(clippy::cast_precision_loss)]
    let z: Vec<f64> = (0..n).map(|k| k as f64).collect();
    let places = lift_places::<Interval>(&stacked(&z, 1.0), Tol::witness());
    if let Err(e) = loft_body::<Interval>(
        &lift_sections::<Interval>(&sections(n)),
        &places,
        deg,
        Tol::witness(),
    ) {
        panic!("{n} evenly stacked arc sections at degree {deg} refuse at Interval: {e:?}");
    }
}

/// At `Interval` even stacking is no shelter: the enclosure solve lands
/// the corner row off `1.0` at eight sections.
#[test]
fn eight_stacked_sections_loft_at_interval() {
    lofts_at_interval(8, 2);
    lofts_at_interval(8, 3);
}

/// The same at 16 and 40 sections.
#[test]
fn many_stacked_sections_loft_at_interval() {
    lofts_at_interval(16, 2);
    lofts_at_interval(40, 2);
    lofts_at_interval(40, 3);
}
