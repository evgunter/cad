//! PROBE (not for landing): the station-frame fork's measurements on
//! `onehome/skin-turn`. Run with `--run-ignored only`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use core::f64::consts::TAU;
use std::time::Instant;

use geom::NurbsCurve3;
use geom_core::sym::{SymBudget, with_session};
use geom_core::{Decide, Interval, Point2, Point3, Real, Sym, Tol};
use profile::{ProfileLoop, SketchPlane};

use crate::common::{normal_start_place, quad};

fn tol() -> Tol {
    Tol::witness()
}

fn helix(turns: f64, pitch: f64) -> NurbsCurve3<f64> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = ((32.0 * turns).ceil() as usize).max(32);
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|k| {
            let s = f64::from(u32::try_from(k).unwrap()) / f64::from(u32::try_from(n).unwrap());
            let a = TAU * turns * s;
            Point3::new(a.cos(), a.sin(), pitch * turns * s)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

fn closed_spine() -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let a = TAU * f64::from(k % 32) / 32.0;
            Point3::new(3.0 * a.cos(), 3.0 * a.sin(), 0.0)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

fn s_duct() -> NurbsCurve3<f64> {
    let r = 2.0;
    let quarter = core::f64::consts::FRAC_PI_2;
    let points: Vec<Point3<f64>> = (0..=8)
        .map(|k| {
            let th = quarter * f64::from(k) / 8.0;
            Point3::new(0.0, r * (1.0 - th.cos()), r * th.sin())
        })
        .chain((1..=8).map(|k| {
            let ph = quarter * f64::from(k) / 8.0;
            Point3::new(0.0, r + r * ph.sin(), 2.0 * r - r * ph.cos())
        }))
        .collect();
    NurbsCurve3::interpolate(&points, 3).unwrap()
}

struct Row {
    name: &'static str,
    path: fn() -> NurbsCurve3<f64>,
    stations: usize,
    half: (f64, f64),
}

fn rows() -> Vec<Row> {
    vec![
        Row { name: "ribbon370", path: || helix(370.0 / 360.0, 0.0), stations: 48, half: (0.1, 0.02) },
        Row { name: "helix2", path: || helix(2.0, 0.4), stations: 65, half: (0.08, 0.08) },
        Row { name: "halfturn", path: || helix(0.5, 0.4), stations: 33, half: (0.08, 0.08) },
        Row { name: "closed_spine", path: closed_spine, stations: 9, half: (0.25, 0.25) },
        Row { name: "s_duct", path: s_duct, stations: 13, half: (0.25, 0.25) },
    ]
}

fn plane_at<T: Decide>(place: &SketchPlane<f64>) -> SketchPlane<T> {
    SketchPlane::from_frame(geom_core::test_support::rigid(
        &place.placement().map(T::from_f64),
        tol(),
    ))
}

fn lifted<T: Real>(loops: &[ProfileLoop<f64>]) -> Vec<ProfileLoop<T>> {
    loops.iter().map(|l| l.map_scalar(T::from_f64)).collect()
}

fn build<T: Decide + topo::AtRestPolicy>(row: &Row) -> Result<usize, String> {
    let path = (row.path)();
    let place = plane_at::<T>(&normal_start_place(&path));
    let (a, b) = row.half;
    let profile = lifted::<T>(&quad([(-a, -b), (a, -b), (a, b), (-a, b)]));
    sweep::test_support::sweep_body_construction::<T>(&profile, place, &path, row.stations, 3, tol())
        .map(|l| l.body.faces().count())
        .map_err(|e| format!("{e:?}"))
}

fn budget() -> SymBudget {
    let terms = std::env::var("PROBE_TERMS").ok().and_then(|t| t.parse().ok()).unwrap_or(4096);
    SymBudget { max_terms: terms, max_degree: if terms < 4096 { 32 } else { 128 } }
}

fn counts(c: &geom_core::sym::SymCounts) -> String {
    format!(
        "decisions={} sym={} num={} rref={} rcon={} disp={} chk={}/{} frozen={}",
        c.decisions(),
        c.symbolic,
        c.numeric,
        c.registrations_refused,
        c.registrations_contradicted,
        c.theorems_disputed,
        c.checks_expanded,
        c.checks_expanded + c.checks_frozen,
        c.frozen
    )
}

fn short(r: &Result<usize, String>) -> String {
    match r {
        Ok(f) => format!("OK faces={f}"),
        Err(e) => format!("REFUSED {}", &e[..e.len().min(400)]),
    }
}

/// (1): the five rows and the wide start frame at `Sym<Interval>`.
#[test]
#[ignore = "probe"]
fn probe_sym_interval_rows() {
    let eps = tol().eps();
    let only = std::env::var("PROBE_ROW").ok();
    for row in rows() {
        if only.as_deref().is_some_and(|o| o != row.name) {
            continue;
        }
        eprintln!("PROBE1-START {}", row.name);
        let t = Instant::now();
        let iv = build::<Interval>(&row);
        let iv_s = t.elapsed().as_secs_f64();
        let name = row.name;
        let t = Instant::now();
        let got = test_utils::own_thread::caught(move || {
            with_session(budget(), || build::<Sym<Interval>>(&row))
        });
        let s = t.elapsed().as_secs_f64();
        match got {
            Ok((r, c)) => println!(
                "PROBE1 eps={eps:e} {name}: Interval {} ({iv_s:.1}s) | Sym<Interval> {} ({s:.1}s) {}",
                short(&iv),
                short(&r),
                counts(&c)
            ),
            Err(p) => println!("PROBE1 eps={eps:e} {name}: Sym<Interval> PANIC {p}"),
        }
    }
    if only.as_deref().is_some_and(|o| o != "wide_start") {
        return;
    }
    // The wide start frame: its x offset over `box_about(0)` as a parameter.
    let w = tol().eps() / 256.0;
    let t = Instant::now();
    let got = test_utils::own_thread::caught(move || {
        with_session(budget(), || {
            let x = Sym::param_over(
                geom_core::sym::ParamSymbol::new(1),
                Interval::from_bounds(-w, w),
                -w,
                w,
            );
            let corner = |x: f64, y: f64| (Point2::new(Sym::<Interval>::from_f64(x), Sym::from_f64(y)), Sym::zero());
            let section = vec![profile::test_support::bulge_loop(vec![
                corner(-1.0, -1.0),
                corner(1.0, -1.0),
                corner(1.2, 1.0),
                corner(-1.0, 1.0),
            ])];
            sweep::test_support::sweep_body_construction::<Sym<Interval>>(
                &section,
                SketchPlane::xy_at(Point3::new(x, Sym::zero(), Sym::zero())),
                &sweep::test_support::elbow_path(),
                3,
                2,
                tol(),
            )
            .map(|l| l.body.faces().count())
            .map_err(|e| format!("{e:?}"))
        })
    });
    let s = t.elapsed().as_secs_f64();
    match got {
        Ok((r, c)) => println!("PROBE1 eps={eps:e} wide_start: Sym<Interval> {} ({s:.1}s) {}", short(&r), counts(&c)),
        Err(p) => println!("PROBE1 eps={eps:e} wide_start: PANIC {p}"),
    }
}
