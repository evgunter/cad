//! Probes for `work/seal/a-corner-reads-only-its-own-sectors.md`: a corner
//! whose near arm reads its faces parallel (Zero at a 1 mm arm) while its
//! far bound (10 m) reads off the other face's plane, through the public
//! union and the n-ary door. A parallelepiped `A` with a 10 m and a 1 mm
//! edge on its bottom face at the origin, and a unit parallelepiped `B`
//! below it cornered there, its top face tilted by `theta`.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stderr)]

use geom_core::{Band, Point3, Tol, Vec3};
use topo::test_support::{finished, mapped_cube};

fn scene(theta: f64, (ta, tb): (f64, f64), (b0, b1): (f64, f64), tol: Tol) -> [topo::Body<f64>; 2] {
    let a = mapped_cube(
        |u, v, w| {
            Point3::new(0.0, 0.0, 0.0)
                + Vec3::new(10.0, 0.0, 0.0) * u
                + Vec3::new(0.0, 1e-3, 0.0) * v
                + Vec3::new(0.0, 0.0, 1.0) * w
        },
        tol,
    );
    let dip = |x: f64, y: f64| Vec3::new(x, y, -theta * (ta * x + tb * y));
    let (u, v) = (
        dip(b0.to_radians().cos(), b0.to_radians().sin()),
        dip(b1.to_radians().cos(), b1.to_radians().sin()),
    );
    let b = mapped_cube(
        move |s, t, w| Point3::new(0.0, 0.0, 0.0) + v * s + u * t + Vec3::new(0.0, 0.0, -1.0) * w,
        tol,
    );
    [a, b]
}

#[test]
#[ignore = "probe"]
fn probe_near_zero_far_off() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let eps = band.zero();
    for k in [10.0, 100.0, 1000.0] {
        for tilt in [(1.0, 0.0), (0.8, 0.6), (0.6, -0.8), (-1.0, 0.0)] {
            for sector in [(45.0, 135.0), (-45.0, 45.0), (30.0, 60.0)] {
                let [a, b] = scene(k * eps, tilt, sector, tol);
                let (a, b) = (finished("A", a, tol), finished("B", b, tol));
                let out = topo::union(&a, &b, tol);
                let out = match out {
                    Ok(_) => "builds".to_string(),
                    Err(e) => format!("{e:?}"),
                };
                eprintln!("k={k} tilt={tilt:?} sector={sector:?}: {out}");
            }
        }
    }
}

fn margin_of(e: &topo::BooleanError) -> Option<f64> {
    match e {
        topo::BooleanError::Escalated { diag, .. } => {
            match diag.margin.diagnostic_f64_for_error_text() {
                geom_core::ErrorTextReading::Value(m) => Some(m),
                _ => None,
            }
        }
        _ => None,
    }
}

#[test]
#[ignore = "probe"]
fn probe_in_band_offer_followed() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let eps = band.zero();
    let k_mult = band.escalate() / band.zero();
    for theta in [0.3e-9, 0.5e-9, 0.8e-9] {
        for sector in [(-45.0, 45.0), (30.0, 60.0)] {
            let [a, b] = scene(theta, (1.0, 0.0), sector, tol);
            let (a, b) = (finished("A", a, tol), finished("B", b, tol));
            let out = topo::union(&a, &b, tol);
            let offer = out
                .as_ref()
                .err()
                .and_then(margin_of)
                .map(|m| m.abs() / k_mult);
            eprintln!(
                "theta={theta:e} sector={sector:?} eps={eps:e}: offer {offer:?}: {:?}",
                out.as_ref().err()
            );
        }
    }
}

#[test]
#[ignore = "probe"]
fn probe_fold_step() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let eps = band.zero();
    let [a, b] = scene(100.0 * eps, (1.0, 0.0), (-45.0, 45.0), tol);
    let far = topo::test_support::brick::<f64>((50.0, 51.0), (0.0, 1.0), (0.0, 1.0), tol);
    let (a, b, far) = (
        finished("A", a, tol),
        finished("B", b, tol),
        finished("far", far, tol),
    );
    for (label, order) in [
        ("a,b,far", [&a, &b, &far]),
        ("a,far,b", [&a, &far, &b]),
        ("far,a,b", [&far, &a, &b]),
    ] {
        let inputs: Vec<topo::MemberInputs> = (0..3)
            .map(|rank| topo::MemberInputs {
                rank,
                ..topo::MemberInputs::default()
            })
            .collect();
        let members: Vec<topo::NaryMember<'_, f64>> = order
            .into_iter()
            .zip(&inputs)
            .map(|(body, inputs)| topo::NaryMember { body, inputs })
            .collect();
        let run = topo::nary_boolean(
            topo::NaryOp::Union,
            &members,
            topo::SweepStrategy::Realized,
            tol,
        );
        eprintln!(
            "fold {label}: judgements {} steps {} end {:?}",
            run.judgements.len(),
            run.steps.len(),
            run.end
        );
    }
}

#[test]
#[ignore = "probe"]
fn probe_theta_from_env() {
    let tol = Tol::witness();
    let theta: f64 = std::env::var("PROBE_THETA")
        .expect("PROBE_THETA")
        .parse()
        .expect("f64");
    for sector in [(-45.0, 45.0), (30.0, 60.0)] {
        let [a, b] = scene(theta, (1.0, 0.0), sector, tol);
        let (a, b) = (finished("A", a, tol), finished("B", b, tol));
        eprintln!(
            "env theta={theta:e} eps={:e} sector={sector:?}: {:?}",
            tol.eps(),
            topo::union(&a, &b, tol).err()
        );
    }
}
