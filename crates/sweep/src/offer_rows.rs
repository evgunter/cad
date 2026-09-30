//! **Every tolerance a sphere refusal offers, executed** (D4 ¶1 (i)):
//! the cases `topo`'s executed-offer census counts as run here
//! (`topo::test_support::OFFERS_EXECUTED_IN_SWEEP`), whose raises need
//! the balls this crate builds. Each is a public Boolean at a fixed
//! margin chosen against the band at [`DESIGN_EPS`]; its child row raises
//! it at whatever tolerance its process runs at, and
//! [`test_utils::offer::execute`] re-runs it just below the value the
//! refusal offered (the harness's module docs state what is true and
//! false). The poses are the coincfr3 review's C1 sphere probes.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::test_support::{ball_poled_y, brick};
use geom_core::{Tol, Vec3};
use test_utils::offer::{DESIGN_EPS, Outcome, execute, report};

/// A margin in the middle of the band at [`DESIGN_EPS`].
const D: f64 = 5.5e-9;

/// A margin in the zero band at [`DESIGN_EPS`].
const Z: f64 = 5e-10;

fn outcome(got: Result<(), topo::BooleanError>) -> Outcome {
    match got {
        Ok(()) => Outcome::Pass,
        Err(err) => {
            let (key, defect) = topo::test_support::offer_key(&err);
            Outcome::Refused {
                key,
                defect,
                text: err.to_string(),
            }
        }
    }
}

/// A unit ball about `(2, 2, 0.5)` and a half-unit ball on its axis at
/// height `z` above that center: their union.
fn two_balls(z: f64) -> Result<(), topo::BooleanError> {
    let tol = Tol::witness();
    let big = ball_poled_y(1.0, Vec3::new(2.0, 2.0, 0.5), tol);
    let small = ball_poled_y(0.5, Vec3::new(2.0, 2.0, 0.5 + z), tol);
    topo::union(&big, &small, tol).map(|_| ())
}

/// A half-unit ball whose bottom stands `gap` above the top of the slab
/// `[0, 4]² × [0, 1]`: their union.
fn ball_over_a_slab(gap: f64) -> Result<(), topo::BooleanError> {
    let tol = Tol::witness();
    let slab = brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol);
    let ball = ball_poled_y(0.5, Vec3::new(2.0, 2.0, 1.5 + gap), tol);
    topo::union(&slab, &ball, tol).map(|_| ())
}

macro_rules! cases {
    ($($name:ident: $key:literal, $positive:literal => $raise:expr;)*) => {
        $(
            #[test]
            #[ignore = "a child row: every_sphere_offer_passes_just_below_it runs it"]
            fn $name() {
                report(stringify!($name), &outcome($raise));
            }
        )*
        const CASES: &[(&str, bool, &str)] = &[$(($key, $positive, stringify!($name))),*];
    };
}

cases! {
    apart_in_band: "Sphere(Apart)", true => two_balls(1.5 + D);
    // A decided zero the spheres' gap reads within the band: it refuses
    // as the in-band arm does.
    apart_in_the_zero_band: "Sphere(Apart)", true => two_balls(1.5 + Z);
    nested_in_band: "Sphere(Nested)", true => two_balls(0.5 - D);
    nested_in_the_zero_band: "Sphere(Nested)", true => two_balls(0.5 - Z);
    clear_of_a_slab_in_band: "Sphere(AgainstPlane)", false => ball_over_a_slab(D);
    into_a_slab_in_band: "Sphere(AgainstPlane)", true => ball_over_a_slab(-D);
}

/// **Every sphere offer passes just below the value offered**, and the
/// cases here are the ones `topo`'s census counts as run here.
#[test]
fn every_sphere_offer_passes_just_below_it() {
    assert_eq!(
        CASES,
        topo::test_support::OFFERS_EXECUTED_IN_SWEEP,
        "the cases topo's census counts as run here"
    );
    let module = module_path!()
        .split_once("::")
        .map_or(module_path!(), |(_, m)| m);
    let mut false_offers = Vec::new();
    for &(key, positive, name) in CASES {
        match execute(
            &format!("{module}::{name}"),
            key,
            topo::test_support::offer_same_decision,
        ) {
            Ok(chain) => {
                let Outcome::Refused { text, .. } = &chain[0].outcome else {
                    unreachable!("execute returns a chain that starts with its refusal");
                };
                let margin = text
                    .split_once("margin ")
                    .and_then(|(_, t)| t.split_whitespace().next())
                    .and_then(|m| m.parse::<f64>().ok());
                // `SpheresMeet` states its verdict, not its margin.
                assert!(
                    margin.is_none_or(|m| (m > 0.0) == positive),
                    "{name}: the refused margin is on the side the case states: {text}"
                );
                assert!(
                    chain[0].eps == DESIGN_EPS && chain.last().unwrap().outcome == Outcome::Pass,
                    "{name}: {chain:?}"
                );
                let path: Vec<String> = chain
                    .iter()
                    .map(|l| match &l.outcome {
                        Outcome::Pass => format!("{:e}: pass", l.eps),
                        Outcome::Refused { key, .. } => format!("{:e}: {key}", l.eps),
                    })
                    .collect();
                println!("OFFER {name}: {} [the public Boolean]", path.join(" -> "));
            }
            Err(why) => false_offers.push(why),
        }
    }
    assert!(
        false_offers.is_empty(),
        "false offers:\n{}",
        false_offers.join("\n\n")
    );
}
