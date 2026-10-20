//! Review probes for PR 5495 (each shell read about its first quadrature
//! face's start). `PROBE_REF` (local harness only) switches the rule:
//! `origin` (main before 5442), `first` (5442), unset (qfirst).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::AtRestBody;

use crate::common::operands::framed_bar;
use crate::germ_plane_torus_chord::half;
use crate::germ_plane_torus_frame::{six, torus};

fn moved(b: &AtRestBody<f64>, m: &Affine3<f64>, who: &str) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let motion = geom_core::test_support::rigid(m, tol);
    finished(who, topo::transform_rigid(b, &motion, tol).unwrap(), tol)
}

fn set_rule(r: &str) {
    let v = match r {
        "origin" => 1,
        "first" => 2,
        _ => 0,
    };
    topo::test_support::PROBE_RULE.store(v, std::sync::atomic::Ordering::SeqCst);
}

/// Every rotation of every shell's stored face order, under each rule:
/// the worst pad ratio, and any refusal.
fn sweep_orders(tag: &str, body: &topo::Body<f64>) {
    let tol = Tol::witness();
    let n = {
        let mut b = body.clone();
        *topo::test_support::probe_rotate_shell_faces(&mut b, 0).iter().max().unwrap()
    };
    for rule in ["origin", "first", "qfirst"] {
        set_rule(rule);
        let (mut lo, mut hi, mut errs) = (f64::INFINITY, 0.0f64, Vec::new());
        let mut pads = Vec::new();
        for k in 0..n {
            let mut b = body.clone();
            topo::test_support::probe_rotate_shell_faces(&mut b, k);
            match topo::mass_properties(&b, tol) {
                Ok(m) => {
                    lo = lo.min(m.volume_pad);
                    hi = hi.max(m.volume_pad);
                    pads.push(m.volume_pad);
                }
                Err(e) => errs.push((k, format!("{e:.120?}"))),
            }
        }
        eprintln!(
            "ORDERS eps={:e} {tag} rule={rule} n={n} min={lo:.3e} max={hi:.3e} ratio={:.2} errs={} {:?}",
            tol.eps(),
            hi / lo,
            errs.len(),
            errs.first()
        );
        eprintln!("PADS {tag} rule={rule} {pads:?}");
    }
    set_rule("qfirst");
}

fn bar(len: f64) -> AtRestBody<f64> {
    finished(
        "bar",
        framed_bar(Point3::new(0.3, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 0.0, len, 12.0),
        Tol::witness(),
    )
}

fn result(o: topo::BooleanResult<f64>) -> AtRestBody<f64> {
    o.body().unwrap().body.clone()
}

#[test]
fn q1_orders_home_and_bar() {
    let tol = Tol::witness();
    let t = torus(2.0, 0.5);
    let p = half(Point3::new(0.3, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).body;
    for (label, got) in six(&t, &p) {
        if let Ok(o) = got {
            if let Some(b) = o.body() {
                sweep_orders(&format!("home-{label}"), &b.body);
            }
        }
    }
    let u = result(topo::union(&t, &bar(900.0), tol).unwrap());
    sweep_orders("bar900-TuP", &u);
    let q = half(Point3::new(1.1, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).body;
    let d = result(topo::subtract(&q, &t, tol).unwrap());
    sweep_orders("x1.1-PmT", &d);
}

#[test]
fn q2_orders_turned_far() {
    let t = torus(2.0, 0.5);
    let p = half(Point3::new(0.3, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).body;
    let m = Affine3::translation(Vec3::new(8.0, -3.0, 5.0) * 100.0)
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.3, -0.5, 0.8).normalize(), 0.7);
    let (t, p) = (moved(&t, &m, "t"), moved(&p, &m, "p"));
    for (label, got) in six(&t, &p) {
        match got {
            Ok(o) => match o.body() {
                Some(b) => sweep_orders(&format!("far-{label}"), &b.body),
                None => eprintln!("far-{label}: empty"),
            },
            Err(e) => eprintln!("far-{label}: OP-ERR {:.200}", format!("{e:?}")),
        }
    }
}

/// Two tori 900 m apart, joined by a bar: one shell whose quadrature
/// faces sit at both ends.
#[test]
fn q3_two_tori_km_apart_one_shell() {
    let tol = Tol::witness();
    let t = torus(2.0, 0.5);
    let t2 = moved(&t, &Affine3::translation(Vec3::new(-900.0, 0.0, 0.0)), "t2");
    let u = result(topo::union(&t, &bar(900.0), tol).unwrap());
    let u = finished("u", u.into_body(), tol);
    for (tag, got) in [
        ("U+T2", topo::union(&u, &t2, tol)),
        ("T2+U", topo::union(&t2, &u, tol)),
    ] {
        match got {
            Ok(o) => sweep_orders(&format!("twotori-{tag}"), &result(o)),
            Err(e) => eprintln!("twotori-{tag}: OP-ERR {:.300}", format!("{e:?}")),
        }
    }
}

/// Two tori 900 m apart, both pierced by one 900.6 m bar: one shell
/// whose quadrature faces sit at both ends, built in one op.
#[test]
fn q4_two_tori_one_bar() {
    let tol = Tol::witness();
    let t = torus(2.0, 0.5);
    let t2 = moved(&t, &Affine3::translation(Vec3::new(-900.0, 0.0, 0.0)), "t2");
    let tt = match topo::union(&t, &t2, tol) {
        Ok(o) => result(o),
        Err(e) => return eprintln!("TT: OP-ERR {:.300}", format!("{e:?}")),
    };
    let b = bar(900.6);
    for (tag, got) in [
        ("TT+B", topo::union(&tt, &b, tol)),
        ("B+TT", topo::union(&b, &tt, tol)),
        ("B-TT", topo::subtract(&b, &tt, tol)),
        ("TT-B", topo::subtract(&tt, &b, tol)),
    ] {
        match got {
            Ok(o) => match o.body() {
                Some(r) => {
                    eprintln!("{tag}: solids/shells {:?}", topo::test_support::probe_rotate_shell_faces(&mut r.body.clone().into_body(), 0));
                    sweep_orders(&format!("twotori-{tag}"), &r.body)
                }
                None => eprintln!("{tag}: empty"),
            },
            Err(e) => eprintln!("twotori-{tag}: OP-ERR {:.300}", format!("{e:?}")),
        }
    }
}

/// A big torus at home and a small one 900 m off, both pierced by one
/// bar: the shell's quadrature faces are far apart and their widths
/// unequal, so which is first decides the big torus's lever.
#[test]
fn q5_big_and_small_tori_one_bar() {
    let tol = Tol::witness();
    let t = torus(2.0, 0.5);
    for (r_big, r_small, c) in [(0.2, 0.05, -900.27), (0.5, 0.1, -900.4)] {
        let t2 = moved(
            &torus(r_big, r_small),
            &Affine3::translation(Vec3::new(c, 0.0, 0.0)),
            "t2",
        );
        let tt = match topo::union(&t, &t2, tol) {
            Ok(o) => result(o),
            Err(e) => return eprintln!("TT: OP-ERR {:.300}", format!("{e:?}")),
        };
        let b = bar(900.6);
        for (tag, got) in [
            ("TT+B", topo::union(&tt, &b, tol)),
            ("B+TT", topo::union(&b, &tt, tol)),
            ("B-TT", topo::subtract(&b, &tt, tol)),
        ] {
            let tag = format!("small{r_big}-{tag}");
            match got {
                Ok(o) => match o.body() {
                    Some(r) => sweep_orders(&tag, &r.body),
                    None => eprintln!("{tag}: empty"),
                },
                Err(e) => eprintln!("{tag}: OP-ERR {:.300}", format!("{e:?}")),
            }
        }
    }
}

/// **Red on the head.** A big torus at home and a small one 900 m off,
/// pierced by one bar: one shell, its stored face order rotated. Under
/// qfirst the pad moves 2.6× (1e-9), 2.7× (1e-12) and 90× (1e-6) with
/// the order; about the world origin it does not move.
#[test]
fn q6_far_apart_quadrature_faces_pads_within_2x_across_orders() {
    let tol = Tol::witness();
    let t = torus(2.0, 0.5);
    let t2 = moved(&torus(0.2, 0.05), &Affine3::translation(Vec3::new(-900.27, 0.0, 0.0)), "t2");
    let tt = result(topo::union(&t, &t2, tol).unwrap());
    let body = result(topo::union(&tt, &bar(900.6), tol).unwrap()).into_body();
    let n = *topo::test_support::probe_rotate_shell_faces(&mut body.clone(), 0).iter().max().unwrap();
    let pads: Vec<f64> = (0..n)
        .map(|k| {
            let mut b = body.clone();
            topo::test_support::probe_rotate_shell_faces(&mut b, k);
            topo::mass_properties(&b, tol).unwrap().volume_pad
        })
        .collect();
    let (lo, hi) = pads.iter().fold((f64::INFINITY, 0.0f64), |(l, h), &p| (l.min(p), h.max(p)));
    assert!(hi <= 2.0 * lo, "pads across {n} orders: {lo:e} .. {hi:e} ({}x)", hi / lo);
}
