//! Two stacked rods whose walls are two half-cylinders each
//! (`circle_split(.., 2, ..)`), the upper one's seams turned by θ about
//! the shared axis. The mating disc is a `Rest` contact and the four
//! wall-half pairs that meet across the mating circle are
//! continuations (`crates/topo/README.md`, C4's continuation clause).
//!
//! The union builds at every θ. With the seams aligned (θ = 0,
//! and θ = π with the halves swapped) the mating circle carries two
//! sites a half turn apart: each seam segment is a semicircle, so its
//! two end germs are perpendicular to their chord, and the two arcs
//! between the sites are parallel edges, which the sites alone do not
//! tell apart.
//! The oracle is closed form: πr²h per rod.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::circle_split;
use sweep::test_support::{extruded, finished, sketch_at};
use topo::{AtRestBody, Body, BooleanBody, BooleanError, BooleanResult};

use core::f64::consts::{FRAC_PI_2, PI};

/// The seam turns of the upper rod: aligned, turned, quarter-turned,
/// and aligned again with the two halves exchanged.
const POSES: [(&str, f64); 4] = [
    ("aligned", 0.0),
    ("turned 0.7", 0.7),
    ("quarter turn", FRAC_PI_2),
    ("half turn", PI),
];

fn tol() -> Tol {
    Tol::witness()
}

/// A rod of radius 1 and height 1 standing on `z0`, its wall the two
/// half-cylinders of a rim split at `phase`.
fn rod(z0: f64, phase: f64) -> AtRestBody<f64> {
    let rim =
        circle_split(Point2::new(0.0, 0.0), 1.0, 2, phase, tol()).expect("the two-arc rim authors");
    finished(
        "the rod",
        extruded(sketch_at(z0), vec![rim.into()], 1.0, tol()),
        tol(),
    )
}

fn volume(body: &Body<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(body), tol()),
        tol(),
    )
    .volume
}

/// A boolean that must build a body of volume `expect` (closed form),
/// valid at tier 3 and 3′.
fn builds(
    label: &str,
    out: Result<BooleanResult<f64>, BooleanError>,
    expect: f64,
) -> BooleanBody<f64> {
    let Ok(BooleanResult::Body(bb)) = out else {
        panic!("{label}: {out:?}");
    };
    let v = volume(&bb.body);
    assert!((v - expect).abs() <= 1e-12, "{label}: {v} vs {expect}");
    assert_eq!(
        topo::validate_geometric(&bb.body, tol()),
        Ok(()),
        "{label}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
        Ok(()),
        "{label}: tier 3′"
    );
    bb
}

/// **The union builds at every seam turn**: six faces (top,
/// bottom and the four wall halves, unmerged across the mating circle:
/// gluing them would close the wall's full period, which the merge
/// records as one skip), volume 2π.
#[test]
fn a_half_rod_stack_unions_at_every_seam_turn() {
    for (label, theta) in POSES {
        let (a, b) = (rod(0.0, 0.0), rod(1.0, theta));
        let bb = builds(label, topo::union(&a, &b, tol()), 2.0 * PI);
        assert_eq!(bb.body.faces().count(), 6, "{label}: faces");
        assert!(
            matches!(
                bb.naming.merge_skipped[..],
                [topo::SkippedMerge {
                    reason: topo::MergeCoplanarError::PeriodClosure { .. },
                    ..
                }]
            ),
            "{label}: the wall's one period closure: {:?}",
            bb.naming.merge_skipped
        );
    }
}

/// **The other three ops build at every seam turn**: ∩ is nothing and
/// each difference its minuend, π. They take the no-crossings path, the
/// coaxial wall halves touching across the mating circle with no
/// crossing event; the section pass defers the settled wall pairs, no
/// edge of either rod runs inside the other's wall, and the shells'
/// sides (each rod `Out` the other) stand.
#[test]
fn a_half_rod_stack_intersects_and_subtracts_at_every_seam_turn() {
    for (label, theta) in POSES {
        let (a, b) = (rod(0.0, 0.0), rod(1.0, theta));
        let meet = topo::intersect(&a, &b, tol());
        assert!(
            matches!(meet, Ok(BooleanResult::Empty)),
            "{label}: A ∩ B: {meet:?}"
        );
        builds(
            &format!("{label}, A ∖ B"),
            topo::subtract(&a, &b, tol()),
            PI,
        );
        builds(
            &format!("{label}, B ∖ A"),
            topo::subtract(&b, &a, tol()),
            PI,
        );
    }
}

/// **The union is a legal operand**: a third rod, aligned with the
/// lower one's seams, stacks on top.
#[test]
fn a_half_rod_stack_takes_a_third_rod() {
    for (label, theta) in POSES {
        let (a, b) = (rod(0.0, 0.0), rod(1.0, theta));
        let stack = builds(label, topo::union(&a, &b, tol()), 2.0 * PI).body;
        let c = rod(2.0, 0.0);
        builds(
            &format!("{label}, third rod"),
            topo::union(&stack, &c, tol()),
            3.0 * PI,
        );
    }
}
