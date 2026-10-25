//! **Probes of the side reader's aims on trimmed spheres and splines**
//! (`work/flush/the-side-at-infinity-reader-offers-no-aim-on-a-trimmed-sphere-or-a-spline.md`):
//! which faces of each body offer an aim, and what each solid's and each
//! shell's aimed rays read before the volume. Evidence for review, not
//! pins: run with
//! `cargo nextest run -p sweep --run-ignored only sphere_spline_aim_probes --no-capture`
//! (and `CAD_TOLERANCE_EPS=1e-6` or `1e-12` for the other rows).

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Affine3, Band, Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished};
use topo::{AtRestBody, BooleanError, BooleanResult};

/// A `y`-poled ball of radius `r` at `c`.
fn ball(r: f64, c: Vec3<f64>, tol: Tol) -> AtRestBody<f64> {
    let at = ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), tol);
    let at = topo::transform_rigid(
        &at,
        &geom_core::test_support::rigid(&Affine3::identity(), tol),
        tol,
    )
    .expect("turn");
    let moved = topo::transform_rigid(&at, &geom_core::RigidMotion::translation(c).unwrap(), tol)
        .expect("move");
    finished("the ball", moved, tol)
}

fn body(what: &str, out: Result<BooleanResult<f64>, BooleanError>) -> Option<AtRestBody<f64>> {
    match out {
        Ok(r) => match r.body() {
            Some(b) => Some(b.body.clone()),
            None => {
                println!("SSA {what}: empty");
                None
            }
        },
        Err(e) => {
            println!("SSA {what}: refused: {e:?}");
            None
        }
    }
}

/// The bodies: shells of trimmed sphere faces only, or with them.
fn fixtures(tol: Tol) -> Vec<(&'static str, AtRestBody<f64>)> {
    let z = |d: f64| Vec3::new(0.0, 0.0, d);
    let unit = || ball(1.0, z(0.0), tol);
    let block = || {
        finished(
            "the block",
            brick((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol),
            tol,
        )
    };
    let mut out = Vec::new();
    let mut push = |what: &'static str, r| {
        if let Some(b) = body(what, r) {
            out.push((what, b));
        }
    };
    push("lens", topo::intersect(&unit(), &ball(0.8, z(0.9), tol), tol));
    push("two balls", topo::union(&unit(), &ball(0.8, z(0.9), tol), tol));
    push("bitten ball", topo::subtract(&unit(), &ball(0.8, z(0.9), tol), tol));
    push("bite", topo::subtract(&ball(0.8, z(0.9), tol), &unit(), tol));
    let lens = topo::intersect(&unit(), &ball(0.8, z(0.9), tol), tol)
        .ok()
        .and_then(|r| r.body().map(|b| b.body.clone()));
    if let Some(lens) = lens {
        push("block less a lens", topo::subtract(&block(), &lens, tol));
    }
    push(
        "block less two balls",
        topo::subtract(
            &block(),
            &body("two", topo::union(&unit(), &ball(0.8, z(0.9), tol), tol)).expect("two balls"),
            tol,
        ),
    );
    out
}

#[test]
#[ignore = "probe: run by hand"]
fn probe_sphere_and_spline_aims() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    for (what, b) in fixtures(tol) {
        let census = topo::validate_pseudomanifold(&b, &topo::ContactRecords::default(), tol);
        println!("SSA eps={:e} {what}: census {:?}", tol.eps(), census.is_ok());
        for l in topo::test_support::each_aim(&b, band) {
            println!("SSA eps={:e} {what}:   face {}", tol.eps(), trim(&l));
        }
        for l in topo::test_support::aimed_rays(&b, band) {
            println!("SSA eps={:e} {what}:   solid {}", tol.eps(), trim(&l));
        }
        for l in topo::test_support::shell_aims(&b, band) {
            println!("SSA eps={:e} {what}:   shell {}", tol.eps(), trim(&l));
        }
    }
}

fn trim(s: &str) -> String {
    s.chars().take(240).collect()
}
