//! Reviewer probes for PR 5699 (per-region carry). Prints; few asserts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stderr)]

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::finished;

use crate::common::operands::framed_bar;
use crate::germ_plane_torus_frame::torus;

fn moved(b: &topo::AtRestBody<f64>, v: Vec3<f64>, who: &str) -> topo::AtRestBody<f64> {
    try_moved(b, v, who).unwrap()
}

fn try_moved(b: &topo::AtRestBody<f64>, v: Vec3<f64>, who: &str) -> Option<topo::AtRestBody<f64>> {
    let tol = Tol::witness();
    let motion = geom_core::test_support::rigid(&Affine3::translation(v), tol);
    match topo::transform_rigid(b, &motion, tol) {
        Ok(m) => Some(finished(who, m, tol)),
        Err(e) => {
            eprintln!("PROBE build {who}: {e:?}");
            None
        }
    }
}

fn bar() -> topo::AtRestBody<f64> {
    finished(
        "bar",
        framed_bar(Point3::new(0.3, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 0.0, 900.6, 12.0),
        Tol::witness(),
    )
}

fn at(b: Result<topo::BooleanResult<f64>, topo::BooleanError>, what: &str) -> topo::AtRestBody<f64> {
    b.unwrap_or_else(|e| panic!("{what}: {e:?}")).body().unwrap().body.clone()
}


fn ok(b: Result<topo::BooleanResult<f64>, topo::BooleanError>, what: &str) -> Option<topo::AtRestBody<f64>> {
    match b {
        Ok(r) => r.body().map(|b| b.body.clone()),
        Err(e) => {
            eprintln!("PROBE build {what}: {e:?}");
            None
        }
    }
}

pub(crate) fn bodies() -> Vec<(&'static str, topo::Body<f64>)> {
    let tol = Tol::witness();
    let far = moved(&torus(0.2, 0.05), Vec3::new(-900.27, 0.0, 0.0), "far");
    let home = torus(2.0, 0.5);
    let tt = at(topo::union(&home, &far, tol), "tt");
    let two = at(topo::union(&tt, &bar(), tol), "two");
    let mut out = vec![("two tori", Some(two.clone()))];
    out.push(("concave both ends", ok(topo::subtract(&bar(), &tt, tol), "concave")));
    let m1 = ok(topo::union(&home, &bar(), tol), "m1");
    out.push(("convex home, concave far", m1.and_then(|m| ok(topo::subtract(&m, &far, tol), "mixed"))));
    let mid = moved(&torus(2.0, 0.5), Vec3::new(-450.0, 0.0, 6.3), "mid");
    let ttm = at(topo::union(&tt, &mid, tol), "ttm");
    out.push(("merged by a mid torus on a side", ok(topo::union(&ttm, &bar(), tol), "merged")));
    out.push(("two tori at 1e5 m", try_moved(&two, Vec3::new(1e5, 3e4, -2e4), "distant")));
    let tt2 = at(topo::union(&tt, &moved(&tt, Vec3::new(0.0, 100.0, 0.0), "tt2"), tol), "tt4");
    let bars = at(topo::union(&bar(), &moved(&bar(), Vec3::new(0.0, 100.0, 0.0), "bar2"), tol), "bars");
    out.push(("two solids of two tori", ok(topo::union(&tt2, &bars, tol), "two shells")));
    let ttt = at(topo::union(&tt, &moved(&torus(0.2, 0.05), Vec3::new(-450.0, 0.0, 6.1), "small mid"), tol), "ttt");
    out.push(("merged by a small mid torus", ok(topo::union(&ttt, &bar(), tol), "three")));
    out.into_iter().filter_map(|(l, b)| b.map(|b| (l, b.into_body()))).collect()
}

/// Per body: volume and pad per rotation, printed.
#[test]
fn print_pads_over_rotations() {
    let tol = Tol::witness();
    for (label, body) in bodies() {
        let n = *topo::test_support::rotate_shell_faces(&mut body.clone(), 0).iter().max().unwrap();
        let mut rows = Vec::new();
        for k in 0..n {
            let mut b = body.clone();
            topo::test_support::rotate_shell_faces(&mut b, k);
            let at = topo::test_support::finished("m", b, tol);
            let t0 = std::time::Instant::now();
            let got = topo::mass_properties(&at, tol);
            let dt = t0.elapsed().as_secs_f64();
            match got {
                Ok(p) => rows.push((p.volume, p.volume_pad, dt)),
                Err(e) => eprintln!("PROBE {label} k={k}: refused {e:?}"),
            }
        }
        let pads: Vec<f64> = rows.iter().map(|r| r.1).collect();
        let lo = pads.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = pads.iter().copied().fold(0.0, f64::max);
        let vlo = rows.iter().map(|r| r.0 + r.1).fold(f64::INFINITY, f64::min);
        let vhi = rows.iter().map(|r| r.0 - r.1).fold(f64::NEG_INFINITY, f64::max);
        let t: f64 = rows.iter().map(|r| r.2).sum::<f64>() / rows.len().max(1) as f64;
        eprintln!(
            "PROBE eps={} {label}: n={n} vol0={:.17e} pad min {lo:.3e} max {hi:.3e} ratio {:.3} overlap {} mean {t:.3}s",
            tol.eps(),
            rows.first().map_or(f64::NAN, |r| r.0),
            hi / lo,
            vlo >= vhi
        );
    }
}

/// Wall time of the doors on each probe body, best of 15.
#[test]
fn print_door_timings() {
    let tol = Tol::witness();
    for (label, body) in bodies() {
        let at = topo::test_support::finished("m", body.clone(), tol);
        let best = |f: &dyn Fn()| {
            (0..15)
                .map(|_| {
                    let t = std::time::Instant::now();
                    f();
                    t.elapsed().as_secs_f64()
                })
                .fold(f64::INFINITY, f64::min)
        };
        let mp = best(&|| {
            let _ = topo::mass_properties(&at, tol);
        });
        let v3 = best(&|| {
            let _ = topo::validate_geometric(&body, tol);
        });
        let cs = best(&|| {
            let _ = topo::classify_shells(&at, tol);
        });
        eprintln!("PROBET eps={} {label}: mass_properties {mp:.5}s validate_geometric {v3:.5}s classify_shells {cs:.5}s", tol.eps());
    }
}

/// Wall time of the boolean that builds the two tori on a bar (its volume backstop refines past the target), best of 9.
#[test]
fn print_boolean_timing() {
    let tol = Tol::witness();
    let far = moved(&torus(0.2, 0.05), Vec3::new(-900.27, 0.0, 0.0), "far");
    let tt = at(topo::union(&torus(2.0, 0.5), &far, tol), "tt");
    let b = bar();
    let best = (0..9)
        .map(|_| {
            let t = std::time::Instant::now();
            let _ = topo::union(&tt, &b, tol).unwrap();
            t.elapsed().as_secs_f64()
        })
        .fold(f64::INFINITY, f64::min);
    let concave = (0..9)
        .map(|_| {
            let t = std::time::Instant::now();
            let _ = topo::subtract(&b, &tt, tol).unwrap();
            t.elapsed().as_secs_f64()
        })
        .fold(f64::INFINITY, f64::min);
    eprintln!("PROBET eps={} union(tt, bar) {best:.5}s subtract(bar, tt) {concave:.5}s", tol.eps());
}
