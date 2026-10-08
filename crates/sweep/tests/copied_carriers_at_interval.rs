//! **A copied arc carrier places at `Interval` without a contradiction.**
//!
//! The pinned lift and `map_scalar` copy an arc's stored f64 carrier
//! into the lane scalar verbatim. Over the reals such a carrier's rim at
//! its start is the radius only up to the f64 rounding it was built
//! with, so the sweep may state nothing about it past rigidity
//! (`sweep::swept::register_rigidity`, `register_placed_carrier_end`):
//! at `Interval` the exact witness would separate any fact that needs
//! the start on the carrier, and a `Contradicted` aborts live in
//! release. Each row here extrudes a copied carrier at `Interval` and
//! asks only that it builds or refuses typed.
#![allow(clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/sweep/src/swept.rs",
    "crates/geom-core/src/arc.rs",
    "crates/profile/src/lib.rs",
    "crates/profile/src/validate.rs",
];

use geom_core::{Arc2, Interval, Point2, Real, Tol};
use profile::{Bulge, Open, Profile, ProfileLoop, RawLoop, Segment, SketchPlane, Start};
use sweep::ExtrudeSide;
use sweep::{Extrusion, extrude};

/// Validates `lp` at f64, lifts it onto the `Interval` xy plane by the
/// pinned lift (which copies the carrier), and extrudes it: `Ok` for a
/// build, `Err` for a typed refusal. A contradiction panics.
fn lifted_extrude(lp: ProfileLoop<f64>, tol: Tol) -> Result<(), String> {
    let vp = Profile::new(SketchPlane::<f64>::xy(), vec![lp])
        .validate(tol)
        .map_err(|e| format!("validate: {e:?}"))?;
    let lifted = vp.lift_onto(SketchPlane::<Interval>::xy());
    extrude(
        &lifted,
        Extrusion::Distance {
            depth: Interval::from_f64(1.0),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .map(|_| ())
    .map_err(|e| format!("extrude: {e:?}"))
}

/// **Random ordinary arcs, copied and placed.** D-shapes (the arc and
/// its closing chord) with vertices within ±100, chords 0.1–10 and
/// bulges 0.05–1.95, lowered at f64 through the fixture door, validated
/// and lifted: none may abort. Before the sweep stopped reading the rim
/// about 1% of these separated at the exact witness.
#[test]
fn random_copied_carriers_extrude_at_interval_without_contradiction() {
    let tol = Tol::witness();
    let mut rng = test_utils::fuzz::start("copied_carriers_at_interval");
    for _ in 0..test_utils::fuzz::scaled(300) {
        let a = Point2::new(rng.range(-100.0, 100.0), rng.range(-100.0, 100.0));
        let (l, d) = (rng.range(0.1, 10.0), rng.range(0.0, core::f64::consts::TAU));
        let e = Point2::new(a.x + l * d.cos(), a.y + l * d.sin());
        let b = rng.range(0.05, 1.95);
        let lp = profile::test_support::bulge_loop(vec![(a, b), (e, 0.0)]);
        let got = std::panic::catch_unwind(|| lifted_extrude(lp, tol));
        assert!(
            got.is_ok(),
            "a = {a:?}, e = {e:?}, b = {b:e}: the copied carrier aborted at Interval; {}",
            test_utils::fuzz::replay()
        );
    }
}

/// **The pinned counterexample**, authored through the lattice as a
/// user would: validated and extruded at f64, then lifted to `Interval`
/// and extruded there. It built at f64 and aborted at `Interval` while
/// the sweep registered the placed landing, which reads the rim.
#[test]
fn a_lattice_arc_lifted_to_interval_extrudes() {
    let tol = Tol::witness();
    let a = Point2::new(-79.674_068_761_865_71, -8.743_422_184_344_226);
    let e = Point2::new(-79.456_229_843_363_16, -7.494_651_585_468_935);
    let closed = Open
        .at(a)
        .arc_to(
            Bulge {
                p: e,
                b: 1.649_230_685_601_469_6,
            },
            tol,
        )
        .expect("the arc authors")
        .line_to(Start, tol)
        .expect("the seam closes");
    assert_eq!(
        lifted_extrude(closed.into(), tol),
        Ok(()),
        "the lifted arc extrudes at Interval"
    );
}

/// **A valid table off its vertices by a tenth of ε**, lifted and
/// extruded: a disc of two half turns whose stored radius is
/// `0.5 + ε/10`. Validation accepts it at f64 (the rim is within the
/// band), and the lift copies the radius, so the placed carrier's end
/// is off the far vertex by ε/10 over the reals — which the sweep must
/// not claim is zero.
#[test]
fn a_valid_table_off_its_vertices_below_eps_extrudes_lifted() {
    let tol = Tol::witness();
    let r = 0.5 + tol.eps() / 10.0;
    let half = Segment::Arc(Arc2 {
        centre: Point2::new(0.0, 0.0),
        radius: r,
        sweep: f64::pi(),
    });
    let disc = <ProfileLoop<f64> as RawLoop<f64>>::new([
        (Point2::new(0.5, 0.0), half),
        (Point2::new(-0.5, 0.0), half),
    ]);
    assert_eq!(
        lifted_extrude(disc, tol),
        Ok(()),
        "the lifted table extrudes at Interval"
    );
}
