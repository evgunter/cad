//! Review probes for PR 5442 (per-shell flux reference).
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

fn report(tag: &str, t: &AtRestBody<f64>, p: &AtRestBody<f64>) {
    let tol = Tol::witness();
    for (label, got) in six(t, p) {
        match got {
            Ok(o) => match o.body() {
                Some(b) => match topo::mass_properties(&b.body, tol) {
                    Ok(m) => eprintln!(
                        "PROBE {tag} {label}: vol {:.17e} pad {:.6e}",
                        m.volume, m.volume_pad
                    ),
                    Err(e) => eprintln!("PROBE {tag} {label}: MP-ERR {e:?}"),
                },
                None => eprintln!("PROBE {tag} {label}: empty"),
            },
            Err(e) => eprintln!("PROBE {tag} {label}: OP-ERR {:.300}", format!("{e:?}")),
        }
    }
}

/// The six ops at home: pads to compare against the merge base.
#[test]
fn p1_home_pads() {
    let t = torus(2.0, 0.5);
    let p = half(Point3::new(0.3, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).body;
    report("home", &t, &p);
}

/// A long half-space bar: the shell of T ∪ P / P ∖ T reaches L.
#[test]
fn p2_long_bar() {
    let tol = Tol::witness();
    let t = torus(2.0, 0.5);
    for l in [12.0, 100.0, 400.0, 900.0] {
        let p = finished(
            "long bar",
            framed_bar(Point3::new(0.3, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), 0.0, l, 12.0),
            tol,
        );
        report(&format!("long{l}"), &t, &p);
        // the same bar entering from the far side: the bar starts at -L
        let q = finished(
            "long bar back",
            framed_bar(Point3::new(0.3, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 0.0, l, 12.0),
            tol,
        );
        report(&format!("back{l}"), &t, &q);
    }
}

/// Other far poses.
#[test]
fn p3_far_poses() {
    let t = torus(2.0, 0.5);
    let p = half(Point3::new(0.3, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).body;
    let poses: [(&str, Vec3<f64>, Vec3<f64>, f64); 5] = [
        ("neg100", Vec3::new(8.0, -3.0, 5.0) * -100.0, Vec3::new(0.3, -0.5, 0.8), 0.7),
        ("y950", Vec3::new(0.0, 950.0, 0.0), Vec3::new(1.0, 1.0, 0.2), 2.1),
        ("diag", Vec3::new(500.0, 500.0, 500.0), Vec3::new(-0.2, 0.9, 0.1), 1.3),
        ("s50", Vec3::new(8.0, -3.0, 5.0) * 50.0, Vec3::new(0.3, -0.5, 0.8), 0.7),
        ("near", Vec3::new(3.0, -2.0, 1.0), Vec3::new(0.6, 0.1, -0.4), 0.4),
    ];
    for (tag, tr, ax, ang) in poses {
        let m = Affine3::translation(tr)
            * Affine3::rotation_about_axis(Point3::origin(), ax.normalize(), ang);
        report(tag, &moved(&t, &m, "t"), &moved(&p, &m, "p"));
    }
}

/// Other spiric cuts: a plane at other offsets and a tilted normal.
#[test]
fn p4_other_spirics() {
    let t = torus(2.0, 0.5);
    let m = Affine3::translation(Vec3::new(8.0, -3.0, 5.0) * 100.0)
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.3, -0.5, 0.8).normalize(), 0.7);
    for (tag, o, n) in [
        ("x1.1", Point3::new(1.1, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
        ("tilt", Point3::new(0.5, 0.0, 0.0), Vec3::new(1.0, 0.2, 0.3)),
    ] {
        let p = half(o, n).body;
        report(&format!("{tag}-home"), &t, &p);
        report(&format!("{tag}-far"), &moved(&t, &m, "t"), &moved(&p, &m, "p"));
    }
}

/// **Red on the head, green on the merge base.** `T ∪ P` and `P ∪ T` are
/// one solid stored in two face orders; the PR's cost argument bounds
/// the shell-reference lever at 2× the origin's on a centred body. With
/// a 900 m bar the two orders' pads differ ~14× at ε 1e-9 (and the
/// at-home pad is ~14× main's): the first face's vertex is at the far
/// end in one order and beside the torus in the other.
#[test]
fn p5_one_solid_two_orders_pads_within_2x() {
    let tol = Tol::witness();
    let t = torus(2.0, 0.5);
    let p = finished(
        "long bar back",
        framed_bar(Point3::new(0.3, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 0.0, 900.0, 12.0),
        tol,
    );
    let pad = |x: &AtRestBody<f64>, y: &AtRestBody<f64>| {
        let got = topo::union(x, y, tol).unwrap();
        topo::mass_properties(&got.body().unwrap().body, tol).unwrap().volume_pad
    };
    let (tp, pt) = (pad(&t, &p), pad(&p, &t));
    assert!(
        tp.max(pt) <= 2.0 * tp.min(pt),
        "one solid, two orders: T ∪ P pad {tp:e}, P ∪ T pad {pt:e}"
    );
}
