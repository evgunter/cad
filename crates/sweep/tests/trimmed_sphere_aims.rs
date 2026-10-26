//! **The side reader's aims on trimmed sphere faces**
//! (`work/flush/the-side-at-infinity-reader-offers-no-aim-on-a-trimmed-sphere-or-a-spline.md`):
//! shells made only of trimmed sphere faces (a lens, a sliver lens, two
//! balls, a bitten ball, and a lens, a sliver lens or two balls cut from a
//! block as a cavity) read their
//! side off rays aimed at those faces, not off their volume. The probe
//! prints which faces offer an aim and what each solid's and shell's rays
//! read: `cargo nextest run -p sweep --run-ignored only
//! probe_trimmed_sphere_aims --no-capture` (and `CAD_TOLERANCE_EPS=1e-6`
//! or `1e-12` for the other rows).

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Band, Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished};
use topo::{AtRestBody, BooleanError, BooleanResult};

/// A `y`-poled ball of radius `r` at `c`.
fn ball(r: f64, c: Vec3<f64>, tol: Tol) -> AtRestBody<f64> {
    let at = ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), tol);
    let moved = topo::transform_rigid(
        &at,
        &geom_core::RigidMotion::translation(c).expect("a translation"),
        tol,
    )
    .expect("move");
    finished("the ball", moved, tol)
}

/// The body `out` built, or `None` with what it did instead printed.
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
    // Off the unit ball's axis, so no aim sits on a symmetry of the pose.
    let off = || ball(0.8, Vec3::new(0.13, 0.07, 0.9), tol);
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
    push(
        "lens",
        topo::intersect(&unit(), &ball(0.8, z(0.9), tol), tol),
    );
    push(
        "two balls",
        topo::union(&unit(), &ball(0.8, z(0.9), tol), tol),
    );
    push(
        "bitten ball",
        topo::subtract(&unit(), &ball(0.8, z(0.9), tol), tol),
    );
    push(
        "bite",
        topo::subtract(&ball(0.8, z(0.9), tol), &unit(), tol),
    );
    push("lens off the axis", topo::intersect(&unit(), &off(), tol));
    push(
        "bitten ball off the axis",
        topo::subtract(&unit(), &off(), tol),
    );
    let lens = topo::intersect(&unit(), &ball(0.8, z(0.9), tol), tol)
        .ok()
        .and_then(|r| r.body().map(|b| b.body.clone()));
    if let Some(lens) = lens {
        push("block less a lens", topo::subtract(&block(), &lens, tol));
    }
    // Two unit balls 1.999 apart: caps 0.032 rad across, a thousandth thick.
    let sliver = || topo::intersect(&unit(), &ball(1.0, z(1.999), tol), tol);
    push("sliver lens", sliver());
    if let Some(sliver) = body("sliver", sliver()) {
        push(
            "block less a sliver lens",
            topo::subtract(&block(), &sliver, tol),
        );
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

/// **A shell of trimmed sphere faces reads its side off aimed rays.**
/// Every face of each fixture offers an aim that reads, and every shell's
/// rays read its role before any volume: an outer shell `Out`, a cavity
/// `In`. Each fixture passes the census with no contacts at the run's ε.
#[test]
fn trimmed_sphere_shells_read_their_side_off_aimed_rays() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let eps = tol.eps();
    let fixtures = fixtures(tol);
    assert_eq!(
        fixtures.len(),
        10,
        "at ε {eps:e}: a fixture's boolean did not build a body"
    );
    for (what, b) in fixtures {
        assert!(
            topo::validate_pseudomanifold(&b, &topo::ContactRecords::default(), tol).is_ok(),
            "{what} at ε {eps:e}: the census refuses the fixture"
        );
        for l in topo::test_support::each_aim(&b, band) {
            assert!(
                l.contains(": Ok(Side("),
                "{what} at ε {eps:e}: a face's aim reads nothing: {l}"
            );
        }
        let shells = topo::test_support::shell_aims(&b, band);
        let want: &[&str] = if what.starts_with("block") {
            &["Ok(Side(Out))", "Ok(Side(In))"]
        } else {
            &["Ok(Side(Out))"]
        };
        assert_eq!(shells, want, "{what} at ε {eps:e}: the shells' aimed rays");
    }
}

#[test]
#[ignore = "probe: run by hand"]
fn probe_trimmed_sphere_aims() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let eps = tol.eps();
    for (what, b) in fixtures(tol) {
        for l in topo::test_support::each_aim(&b, band) {
            println!("SSA eps={eps:e} {what}:   face {l}");
        }
        for l in topo::test_support::aimed_rays(&b, band) {
            println!("SSA eps={eps:e} {what}:   solid {l}");
        }
        for l in topo::test_support::shell_aims(&b, band) {
            println!("SSA eps={eps:e} {what}:   shell {l}");
        }
    }
}
