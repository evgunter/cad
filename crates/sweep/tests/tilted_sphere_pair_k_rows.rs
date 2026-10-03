//! **What a carved sphere's boolean records under the clearance name, at
//! the recording scalar.** A ball cut by another ball off its polar axis
//! splits its meridian edges at the section, and each fragment then
//! ENDS on the other ball. The fragment's residual against that ball is
//! exactly zero at that end, so its true one-sidedness margin is at most
//! zero: the circle clearance (`bool_circle_curved_clearance`) cannot
//! read it definitely clear, and every other answer falls through to the
//! endpoint arms. Asking it decided nothing. What it recorded was the
//! sampled enclosure's chord-dip charge, read as `−charge` about that
//! zero end. On lily wall 7 that gave 36 rows per ε, from −1.5e-10 to
//! −3.0e-6, bit-identical across ε, which the K lint read as micrometre
//! features below its floor (rule 3) and in the ambiguity band at 1e-6
//! (rule 1). The sweep arm now decides such an arc's endpoint sides
//! first and does not ask its clearance
//! (`topo::boolean::reduce::curved_face_arm`).
//!
//! The row: every clearance the poses below record sits at least the K
//! lint's metre floor (`tools/k-lint`'s `BASELINE_FLOOR_MARGIN`, 4e-5)
//! from zero, and the clearance is still asked of the arcs it can
//! decide.
//!
//! **CI EXECUTES THIS SUITE**: it is rostered in
//! `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and runs under the
//! default selection of `scripts/k_probe_sweep.sh`.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::{self, Probe};
use geom_core::{Affine3, Point2, Real, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y_at;
use topo::{Body, BooleanResult};

/// `tools/k-lint`'s `BASELINE_FLOOR_MARGIN`: the bottom edge of the
/// corpus's ε-independent definite margins. k-lint is a separate cargo
/// root, so the value is spelled here, and k-lint's
/// `the_sweep_rows_metre_floor_is_the_lints` pins the two equal.
const METRE_FLOOR: f64 = 4.0e-5;

/// A ball of radius `r` centred at `c`, poles on world `y`, lifted to
/// the recording scalar.
fn ball(r: f64, c: Vec3<f64>) -> Body<Probe> {
    let p = Probe::from_f64;
    let b = revolved_about_y_at::<Probe>(
        vec![
            (Point2::new(p(0.0), p(-r)), p(1.0)),
            (Point2::new(p(0.0), p(r)), p(0.0)),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    let to = Vec3::new(p(c.x), p(c.y), p(c.z));
    topo::transform_rigid(&b, &Affine3::translation(to), Tol::witness()).unwrap()
}

#[test]
fn a_carved_balls_meridian_fragments_record_no_clearance_charge() {
    let base = Vec3::new(2.0, 2.0, 0.5);
    let mut asked = 0usize;
    for (r, offset) in [
        (1.0, Vec3::new(1.4, 0.0, 0.0)),
        (1.0, Vec3::new(1.2, 0.6, 0.0)),
        (0.6, Vec3::new(0.9, 0.0, 0.0)),
    ] {
        let fin = |what, b| topo::test_support::finished(what, b, Tol::witness());
        let (a, b) = (fin("a", ball(1.0, base)), fin("b", ball(r, base + offset)));
        for (name, op) in [
            ("union", topo::boolean::union::<Probe> as fn(_, _, _) -> _),
            ("intersect", topo::boolean::intersect::<Probe>),
            ("subtract", topo::boolean::subtract::<Probe>),
        ] {
            k_stats::start_recording();
            let out = op(&a, &b, Tol::witness());
            let samples = k_stats::take_samples();
            assert!(
                matches!(out, Ok(BooleanResult::Body(_))),
                "{name} r {r} at {offset:?}: {out:?}"
            );
            for s in samples
                .iter()
                .filter(|s| s.predicate == "bool_circle_curved_clearance")
            {
                asked += 1;
                assert!(
                    s.margin.abs() >= METRE_FLOOR,
                    "{name} r {r} at {offset:?}: a clearance recorded {:e}, under the metre \
                     floor: an arc ending on the carrier was asked its clearance",
                    s.margin
                );
            }
        }
    }
    assert!(
        asked > 0,
        "no clearance was asked: the row would be vacuous"
    );
}
