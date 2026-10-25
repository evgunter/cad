//! Reviewer probe (PR 5699): pads across rotations of multi-region shells,
//! head against merge base. Prints; asserts nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::AtRestBody;

use crate::common::operands::framed_bar;
use crate::germ_plane_torus_frame::torus;

fn moved(b: &AtRestBody<f64>, by: Vec3<f64>, who: &str) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let m = geom_core::test_support::rigid(&Affine3::translation(by), tol);
    finished(who, topo::transform_rigid(b, &m, tol).unwrap(), tol)
}

fn un(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Option<AtRestBody<f64>> {
    let tol = Tol::witness();
    match topo::union(a, b, tol) {
        Ok(r) => r.body().map(|b| b.body.clone()),
        Err(e) => {
            eprintln!("union refused: {e:?}");
            None
        }
    }
}


fn report(label: &str, body: topo::Body<f64>) {
    let tol = Tol::witness();
    let n = *topo::test_support::rotate_shell_faces(&mut body.clone(), 0)
        .iter()
        .max()
        .unwrap();
    let mut pads = Vec::new();
    let mut vols = Vec::new();
    for k in 0..n {
        let mut b = body.clone();
        topo::test_support::rotate_shell_faces(&mut b, k);
        match topo::mass_properties(
            &topo::test_support::finished("m", topo::Body::clone(&b), tol),
            tol,
        ) {
            Ok(m) => {
                pads.push(m.volume_pad);
                vols.push(m.volume);
            }
            Err(e) => eprintln!("PROBE {label} k={k}: refused {e:?}"),
        }
    }
    let mx = pads.iter().copied().fold(0.0, f64::max);
    let mn = pads.iter().copied().fold(f64::INFINITY, f64::min);
    let vmx = vols.iter().copied().fold(f64::MIN, f64::max);
    let vmn = vols.iter().copied().fold(f64::MAX, f64::min);
    eprintln!(
        "PROBE eps={:e} {label}: n={n} pad min {mn:.3e} max {mx:.3e} ratio {:.3} | vol {vmn:.12e}..{vmx:.12e} spread {:.3e}",
        tol.eps(),
        mx / mn,
        vmx - vmn
    );
}

/// Tori (R, r, centre) united first, then the bar from `x0` along −x.
fn tori_bar(tori: &[(f64, f64, Vec3<f64>)], x0: f64, len: f64) -> Option<AtRestBody<f64>> {
    let tol = Tol::witness();
    let mut acc: Option<AtRestBody<f64>> = None;
    for &(r1, r2, at) in tori {
        let t = moved(&torus(r1, r2), at, "torus");
        acc = Some(match acc {
            None => t,
            Some(a) => un(&a, &t)?,
        });
    }
    let b = finished(
        "bar",
        framed_bar(Point3::new(x0 + 0.3, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 0.0, len, 12.0),
        tol,
    );
    un(&acc?, &b)
}

#[test]
fn review_5699_multi_region_pads() {
    let v = Vec3::new;
    let home = (2.0, 0.5, v(0.0, 0.0, 0.0));
    let far = (0.2, 0.05, v(-900.27, 0.0, 0.0));
    let side_near = (2.0, 0.5, v(-5.0, 0.0, 5.7));
    let side_far = (2.0, 0.5, v(-895.0, 0.0, 5.7));
    let mid = (0.2, 0.05, v(-450.0, 0.0, 6.15));
    let cases: Vec<(&str, Vec<(f64, f64, Vec3<f64>)>, f64)> = vec![
        ("two tori on a bar", vec![home, far], 0.0),
        ("two tori + side torus near home", vec![home, far, side_near], 0.0),
        ("two tori + side torus near far end", vec![home, far, side_far], 0.0),
        ("home + two side tori on one top face", vec![home, side_near, side_far], 0.0),
        ("home torus + side torus near far end", vec![home, side_far], 0.0),
        ("two tori + mid small side torus", vec![home, far, mid], 0.0),
        ("side near + side far only (one region)", vec![side_near, side_far], 0.0),
    ];
    for (label, tori, x0) in cases {
        match tori_bar(&tori, x0, 900.6) {
            Some(b) => report(label, b.clone().into_body()),
            None => eprintln!("PROBE {label}: did not build"),
        }
    }
    let shift = 1.0e5;
    let sh = |t: (f64, f64, Vec3<f64>)| (t.0, t.1, t.2 + v(shift, 0.0, 0.0));
    match tori_bar(&[sh(home), sh(far)], shift, 900.6) {
        Some(b) => report("two tori on a bar built at 1e5 m", b.clone().into_body()),
        None => eprintln!("PROBE 1e5: did not build"),
    }
    match tori_bar(&[sh(home), sh(far), sh(side_near)], shift, 900.6) {
        Some(b) => report("three regions built at 1e5 m", b.clone().into_body()),
        None => eprintln!("PROBE 1e5 three: did not build"),
    }
}
