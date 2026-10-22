//! Review 5590 probes: the pncad-py `mass_properties` spelling (gate, then
//! measure) against the one-quadrature spelling (the gate's certificate,
//! continued), on curved bodies: equal answers, and the cost of each.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Tol, Vec3};
use std::time::Instant;

#[test]
fn review_5590_door_twice_vs_certificate_once() {
    let tol = Tol::witness();
    let bodies = [
        ("ball", sweep::test_support::ball_poled(0.5, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), tol)),
        ("lantern", sweep::test_support::lantern(tol)),
        ("domed_cavity", sweep::test_support::domed_cavity(tol)),
    ];
    for (name, body) in bodies {
        let reps = 5;
        let t = Instant::now();
        let mut twice = None;
        for _ in 0..reps {
            let at_rest = topo::AtRestBody::validate(body.clone(), tol).expect("finishes");
            twice = Some(topo::mass_properties(&at_rest, tol).unwrap());
        }
        let t_twice = t.elapsed() / reps;
        let t = Instant::now();
        let mut once = None;
        for _ in 0..reps {
            let cert = topo::validate_geometric_certificate(&body, tol).expect("finishes");
            once = Some(cert.measure().ok().expect("measures"));
        }
        let t_once = t.elapsed() / reps;
        let t = Instant::now();
        for _ in 0..reps {
            topo::validate_geometric(&body, tol).unwrap();
        }
        let t_gate = t.elapsed() / reps;
        let (a, b) = (twice.unwrap(), once.unwrap());
        println!(
            "REVIEW5590 {name}: gate+door {t_twice:?}, certificate once {t_once:?}, gate alone {t_gate:?}; bitwise equal: {}",
            (a.volume.to_bits(), a.surface_area.to_bits(), a.volume_pad.to_bits()) == (b.volume.to_bits(), b.surface_area.to_bits(), b.volume_pad.to_bits())
        );
    }
}
