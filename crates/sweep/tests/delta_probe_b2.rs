//! REVIEW PROBE (B2 fix-pass delta): not for merge. Run with
//! `CAD_TOLERANCE_EPS` set per ε row; prints a table, asserts nothing but
//! panics-free tallies at the end of each test.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::membership::{self, Region};
use crate::common::wrap_tip::{NORTH, one_wrap_edge};
use core::f64::consts::{FRAC_PI_2, PI, TAU};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use std::panic::{AssertUnwindSafe, catch_unwind};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanResult, intersect, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

fn cone(alpha: f64) -> topo::Body<f64> {
    let r = alpha.to_radians().tan();
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn forms(alpha: f64) -> [(&'static str, topo::Body<f64>); 2] {
    [
        ("tipped", one_wrap_edge(cone(alpha), true).0),
        (
            "revolved",
            crate::revolve_common::needle(alpha, None, tol()).into_body(),
        ),
    ]
}

fn moved(body: &topo::Body<f64>, m: Affine3<f64>) -> AtRestBody<f64> {
    finished(
        "moved",
        topo::transform_rigid(body, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap(),
        tol(),
    )
}

fn the_cone(r: f64) -> Region {
    Box::new(move |p: Point3<f64>| (r * (1.0 - p.y) - p.x.hypot(p.z)).min(p.y).min(1.0 - p.y))
}

fn moved_region(region: Region, m: Affine3<f64>) -> Region {
    let back = m.inverse();
    Box::new(move |p| region(back.transform_point(p)))
}

fn volume(alpha: f64) -> f64 {
    PI * alpha.to_radians().tan().powi(2) / 3.0
}

/// Whether the wrapped interval `[lo, hi]` (width < 2π) contains `x`
/// strictly inside, by more than `m`.
fn strictly_in(x: f64, lo: f64, hi: f64, m: f64) -> bool {
    let d = (x - lo).rem_euclid(TAU);
    d > m && d < (hi - lo) - m
}

/// My own pinch rule: the box covers the azimuth interval `[lo, hi]`; the
/// remainder of a cone face is pinched where both cut rulings lie on one
/// face, the seam(s) in the remainder: tipped (seam 0) where 0 is
/// strictly outside the closed box interval; revolved (seams 0, π) where
/// neither seam is in the closed box interval.
fn pinches(form: &str, lo: f64, hi: f64) -> bool {
    let m = 1e-7;
    let in_closed = |x: f64| {
        let d = (x - lo).rem_euclid(TAU);
        d <= (hi - lo) + m || d >= TAU - m
    };
    let _ = strictly_in;
    if form == "tipped" { !in_closed(0.0) } else { !in_closed(0.0) && !in_closed(PI) }
}

fn run_box(
    out: &mut Vec<String>,
    what: &str,
    bx: ((f64, f64), (f64, f64), (f64, f64)),
    m: Affine3<f64>,
    alpha: f64,
    share: f64,
    pinch: &dyn Fn(&str) -> bool,
) {
    let ((x0, x1), (y0, y1), (z0, z1)) = bx;
    let vb = (x1 - x0) * (y1 - y0) * (z1 - z0);
    let b = moved(&topo::test_support::brick(bx.0, bx.1, bx.2, tol()), m);
    let r = alpha.to_radians().tan();
    let va = volume(alpha);
    let ext = (x1 - x0).abs().max((z1 - z0).abs()) + 0.5;
    for (form, body) in forms(alpha) {
        let c = finished("c", body, tol());
        let ra = || the_cone(r);
        let rb = || moved_region(membership::cuboid(bx.0, bx.1, bx.2), m);
        let ops: [(&str, u8); 6] = [
            ("a∩b", 0),
            ("b∩a", 1),
            ("a∪b", 2),
            ("b∪a", 3),
            ("a∖b", 4),
            ("b∖a", 5),
        ];
        for (op, k) in ops {
            let want = match k {
                0 | 1 => share,
                2 | 3 => va + vb - share,
                4 => va - share,
                _ => vb - share,
            };
            let expect_pinch = pinch(form) && matches!(k, 2 | 3 | 4);
            let res = catch_unwind(AssertUnwindSafe(|| match k {
                0 => intersect(&c, &b, tol()),
                1 => intersect(&b, &c, tol()),
                2 => union(&c, &b, tol()),
                3 => union(&b, &c, tol()),
                4 => subtract(&c, &b, tol()),
                _ => subtract(&b, &c, tol()),
            }));
            let line = match res {
                Err(_) => "PANIC".to_string(),
                Ok(Err(topo::BooleanError::ResultPinchedAtConeApex { .. })) => {
                    if expect_pinch { "pinch ok".into() } else { "PINCH-UNEXPECTED".into() }
                }
                Ok(Err(e)) => format!(
                    "ERR{} {}",
                    if expect_pinch { "(want pinch)" } else { "" },
                    format!("{e:?}").chars().take(140).collect::<String>()
                ),
                Ok(Ok(BooleanResult::Empty)) => {
                    if want.abs() < 1e-9 { "empty ok".into() } else { format!("EMPTY want {want}") }
                }
                Ok(Ok(BooleanResult::Body(body))) => {
                    let mut s = String::new();
                    if expect_pinch {
                        s += "BUILT-WANT-PINCH ";
                    }
                    match catch_unwind(AssertUnwindSafe(|| {
                        topo::test_support::pinned_volume(what, &body.body, tol())
                    })) {
                        Ok(v) => {
                            let err = (v.volume - want).abs();
                            if err > v.volume_pad + 1e-9 {
                                s += &format!("VOL-WRONG {:.9} want {want:.9} pad {:.1e}", v.volume, v.volume_pad);
                            } else {
                                s += &format!("vol ok ({err:.1e})");
                            }
                        }
                        Err(_) => s += "VOL-PANIC",
                    }
                    let region: Region = {
                        let (x, y) = (ra(), rb());
                        match k {
                            0 | 1 => Box::new(move |p| x(p).min(y(p))),
                            2 | 3 => Box::new(move |p| x(p).max(y(p))),
                            4 => Box::new(move |p| x(p).min(-y(p))),
                            _ => Box::new(move |p| y(p).min(-x(p))),
                        }
                    };
                    let lat = catch_unwind(AssertUnwindSafe(|| {
                        membership::holds_short_of_tier_3_prime(
                            what,
                            &body.body,
                            want,
                            &region,
                            (Point3::new(-ext, -0.25, -ext), Point3::new(ext, 1.85, ext)),
                            7,
                        );
                    }));
                    if lat.is_err() {
                        s += " LATTICE/TIER-FAIL";
                    }
                    s
                }
            };
            out.push(format!("{what} {form} | {op} | {line}"));
        }
    }
}

fn dump(name: &str, out: &[String]) {
    let eps = tol().eps();
    let dir = std::env::var("PROBE_OUT").unwrap_or("/tmp".into());
    let path = format!("{dir}/{name}-eps{eps:e}.txt");
    std::fs::write(&path, out.join("\n")).unwrap();
    let bad = out
        .iter()
        .filter(|l| l.contains("WRONG") || l.contains("PANIC") || l.contains("UNEXPECTED") || l.contains("WANT") || l.contains("FAIL") || l.contains("EMPTY") || l.contains("want pinch"))
        .count();
    eprintln!("DUMP {path}: {} ops, {bad} flagged", out.len());
}

/// The two box-edge poses at turns and apertures outside the PR's rows,
/// the box grown so its far faces stay clear of the cone at wide α.
fn box_edges(alphas: &[f64], turns: &[f64], name: &str) {
    let mut out = Vec::new();
    for &alpha in alphas {
        let r = alpha.to_radians().tan();
        let l = 0.45 * r + 1.0;
        for &turn in turns {
            let m = Affine3::rotation_about_axis(NORTH, Vec3::new(0.0, 1.0, 0.0), turn);
            let v = volume(alpha) * 0.45f64.powi(3);
            // across: covers azimuths [-t-π/2, π/2-t]
            run_box(
                &mut out,
                &format!("across α{alpha} t{turn:.6}"),
                ((0.0, l), (0.55, 1.0), (-l, l + 0.3)),
                m,
                alpha,
                v / 2.0,
                &|f| pinches(f, -turn - FRAC_PI_2, FRAC_PI_2 - turn),
            );
            run_box(
                &mut out,
                &format!("along α{alpha} t{turn:.6}"),
                ((0.0, l), (0.55, 1.6), (0.0, l + 0.1)),
                m,
                alpha,
                v / 4.0,
                &|f| pinches(f, -turn, FRAC_PI_2 - turn),
            );
        }
    }
    dump(name, &out);
}

#[test]
fn d1_new_turns() {
    box_edges(
        &[30.0, 45.0, 60.0],
        &[0.2, 1.0, PI / 4.0, 2.6, 3.5, 4.0, 5.2, 1e-4, PI + 1e-4, FRAC_PI_2 - 1e-4, TAU - 1e-6],
        "d1",
    );
}

#[test]
fn d2_new_alphas() {
    box_edges(&[15.0, 50.0, 75.0, 80.0], &[0.0, 0.4, FRAC_PI_2, 2.0, PI, 3.0 * FRAC_PI_2, TAU - 1e-4], "d2");
}

/// The apex on a box face square to the axis (a pierced plane with no
/// extremal ruling: one whole-turn apex sector, `BoolSector::middle`
/// reading the two bounds' azimuth), above and below, turned.
#[test]
fn d3_apex_on_a_face_square_to_the_axis() {
    let mut out = Vec::new();
    for alpha in [30.0f64, 60.0] {
        let r = alpha.to_radians().tan();
        let l = r + 1.0;
        for turn in [0.0, 0.4, 2.0, TAU - 1e-4] {
            let m = Affine3::rotation_about_axis(NORTH, Vec3::new(0.0, 1.0, 0.0), turn);
            // top face y = 1 holds the apex, box below: holds the cone above 0.5.
            run_box(
                &mut out,
                &format!("below α{alpha} t{turn}"),
                ((-l, l + 0.2), (0.5, 1.0), (-l - 0.1, l)),
                m,
                alpha,
                volume(alpha) * 0.125,
                &|_| false,
            );
            // bottom face y = 1 holds the apex, box above: touches at a point.
            run_box(
                &mut out,
                &format!("above α{alpha} t{turn}"),
                ((-l, l + 0.2), (1.0, 1.5), (-l - 0.1, l)),
                m,
                alpha,
                0.0,
                &|_| false,
            );
            // the box's top tilted by a fraction of the band about x.
            let eps = tol().eps();
            let tilt = Affine3::rotation_about_axis(NORTH, Vec3::new(1.0, 0.0, 0.0), 0.3 * eps / l);
            run_box(
                &mut out,
                &format!("below-tilted α{alpha} t{turn}"),
                ((-l, l + 0.2), (0.5, 1.0), (-l - 0.1, l)),
                tilt * m,
                alpha,
                volume(alpha) * 0.125,
                &|_| false,
            );
        }
    }
    dump("d3", &out);
}
