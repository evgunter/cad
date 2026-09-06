//! R1 review probes for BOOL-3 (issue 1011, torus half). NOT shipped
//! rows: they instrument claims the PR states as numbers, so a reader
//! can see the numbers rather than the assertion that bounds them.
//!
//! * the analytic oracle's ACTUAL escalation count and lattice size
//!   (the shipped row bounds the rate and prints nothing);
//! * a denser oracle lattice, to push "zero wrong answers" harder than
//!   2431 points do;
//! * whether the tangency-shell guard's ±2× window can actually
//!   DISTINGUISH the cube-root law it asserts from the `√ε` law it says
//!   it rules out, at each shipped ε;
//! * the three refusals the spindle receipt names, each asked
//!   separately.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Point3, Tol, Vec3};
use profile::RawLoop;
use profile::{ProfileLoop, ProfileVertex};
use revolve_common::*;
use sweep::{Revolution, revolve};
use topo::{Body, SolidContainment, point_in_solid};

const R: f64 = 1.0;
const MINOR: f64 = 0.3;
const EXT: f64 = R + MINOR;

fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).unwrap()
}

fn donut() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(R, -MINOR), 1.0),
        ProfileVertex::new(p2(R, MINOR), 1.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn clearance(q: Point3<f64>) -> f64 {
    let rho = q.x.hypot(q.z);
    ((rho - R).powi(2) + q.y * q.y).sqrt() - MINOR
}

/// The shipped oracle row asserts `escalated * 20 < total` and prints
/// nothing. The PR claims **zero** escalations and zero wrong answers.
/// This is the same sweep, denser, with the numbers on stdout.
#[test]
fn r1_oracle_sweep_prints_its_own_numbers() {
    let body = donut();
    for (ni, nj, nk, label) in [
        (17, 13, 11, "shipped lattice"),
        (35, 27, 23, "dense lattice"),
    ] {
        let mut wrong = Vec::new();
        let mut escalated = 0usize;
        let mut total = 0usize;
        let mut skipped = 0usize;
        for i in 0..ni {
            for j in 0..nj {
                for k in 0..nk {
                    let q = Point3::new(
                        -1.6 + 3.2 * f64::from(i) / f64::from(ni - 1),
                        -0.6 + 1.2 * f64::from(j) / f64::from(nj - 1),
                        -1.6 + 3.2 * f64::from(k) / f64::from(nk - 1),
                    );
                    let c = clearance(q);
                    if c.abs() < 1e-3 {
                        skipped += 1;
                        continue;
                    }
                    total += 1;
                    let want = if c < 0.0 {
                        SolidContainment::In
                    } else {
                        SolidContainment::Out
                    };
                    match point_in_solid(&body, q, band(), Tol::witness()) {
                        Ok(got) if got == want => {}
                        Ok(got) => wrong.push(format!("{q:?} clearance {c:.9} answered {got:?}")),
                        Err(_) => escalated += 1,
                    }
                }
            }
        }
        println!(
            "R1 oracle [{label}] eps={:e}: total={total} skipped={skipped} \
             escalated={escalated} wrong={}",
            Tol::witness().get().eps,
            wrong.len()
        );
        for w in wrong.iter().take(5) {
            println!("    WRONG {w}");
        }
        assert!(wrong.is_empty(), "{} wrong answers", wrong.len());
    }
}

/// **Can the guard row's window tell the two laws apart?** The shipped
/// guard asserts `law/2 < shell < law*2` with `law = 0.143·(K·ε·ext²)^⅓`.
/// The PR's argument for that being a real check is that "a `√ε` law is
/// off by ten at the 1e-12 row". This measures the separation at the ε
/// the run actually drew, which is what the guard's discriminating power
/// depends on.
#[test]
fn r1_the_shell_guard_window_versus_the_rejected_sqrt_law() {
    let body = donut();
    let mut shell = 0.0_f64;
    let mut d = 0.1_f64;
    while d > 1e-13 {
        let q = Point3::new(0.0, MINOR + d, R);
        if !matches!(
            point_in_solid(&body, q, band(), Tol::witness()),
            Ok(SolidContainment::Out)
        ) {
            shell = shell.max(d);
        }
        d /= 1.05;
    }
    let eps = Tol::witness().get().eps;
    let k = eps * 10.0;
    let cube = (k * EXT.powi(2)).cbrt() * 0.143;
    // BOOL-2's law, the one the PR says is ruled out, with the same
    // fitted constant it would need to match at THIS eps.
    let sqrt_law = (k * EXT).sqrt();
    let ratio = shell / sqrt_law;
    println!(
        "R1 shell law: eps={eps:e} measured={shell:e} cube-law={cube:e} (x{:.3}) \
         sqrt(K*eps*ext)={sqrt_law:e} (x{ratio:.3}) -- guard window is [{:e}, {:e}]",
        shell / cube,
        cube / 2.0,
        cube * 2.0
    );
    let sqrt_inside = sqrt_law > cube / 2.0 && sqrt_law < cube * 2.0;
    println!(
        "R1 shell law: at this eps the rejected sqrt law is {} the guard's window",
        if sqrt_inside { "INSIDE" } else { "outside" }
    );
}

/// The spindle receipt names three refusals. The shipped row asks the
/// public door only; this asks each named door it can reach separately,
/// so "all three close the mint" is not resting on one of them.
#[test]
fn r1_the_spindle_refusals_asked_one_at_a_time() {
    // (1) `revolve` at construction: a profile that really is a spindle.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.2, -0.5), 1.0),
        ProfileVertex::new(p2(0.2, 0.5), 1.0),
    ]);
    let got = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    );
    println!(
        "R1 spindle: revolve(R=0.2, r=0.5) -> {}",
        match &got {
            Ok(_) => "Ok(BODY) -- the mint door did NOT refuse".to_string(),
            Err(e) => format!("Err({e:?})"),
        }
    );
    assert!(got.is_err(), "the spindle must not be mintable by revolve");

    // (2) a spindle that only just is one (R just under r).
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.499, -0.5), 1.0),
        ProfileVertex::new(p2(0.499, 0.5), 1.0),
    ]);
    let got = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    );
    println!(
        "R1 spindle: revolve(R=0.499, r=0.5) -> {}",
        match &got {
            Ok(_) => "Ok(BODY)".to_string(),
            Err(e) => format!("Err({e:?})"),
        }
    );

    // (3) the horn case R == r exactly, the boundary of the convention.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.5, -0.5), 1.0),
        ProfileVertex::new(p2(0.5, 0.5), 1.0),
    ]);
    let got = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    );
    println!(
        "R1 spindle: revolve(R=r=0.5, the HORN torus) -> {}",
        match &got {
            Ok(_) => "Ok(BODY) -- R > r is premised by the arm but not enforced here".to_string(),
            Err(e) => format!("Err({e:?})"),
        }
    );
}

/// The flipped verbs_gate row claims its volume assertion witnesses "the
/// hole of the donut still free space — which only a four-root ray can
/// say". `vol` is a surface integral: this asks whether the containment
/// door is on that row's path at all, by asking the door directly at the
/// hole centre and reporting both.
#[test]
fn r1_the_hole_is_free_space_asked_of_the_door_itself() {
    let body = donut();
    let hole = Point3::new(0.0, 0.0, 0.0);
    println!(
        "R1 hole: point_in_solid(centre) = {:?}",
        point_in_solid(&body, hole, band(), Tol::witness())
    );
    assert_eq!(
        point_in_solid(&body, hole, band(), Tol::witness()).unwrap(),
        SolidContainment::Out
    );
    // and just inside the near wall, where a quadratic fold would answer In
    let near = Point3::new(0.0, 0.0, R - MINOR + 1e-2);
    println!(
        "R1 hole: point_in_solid(just inside the near wall) = {:?}",
        point_in_solid(&body, near, band(), Tol::witness())
    );
    let _ = Vec3::new(0.0, 1.0, 0.0);
}
