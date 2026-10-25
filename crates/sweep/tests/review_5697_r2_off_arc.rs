//! Review R2 of PR 5697, claim 11 (head only): a single curved face whose
//! own rim's carrier circle passes near a root outside its arc. A ray is
//! cast through a point of the carrier in the face's gap, within the band
//! of the rim's carrier but far from the rim's arc, on to a point inside
//! the face. Evidence, not pins.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use core::f64::consts::PI;

use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::Body;

/// `ray_at`'s outward direction for `normal` (side_reader's
/// `AIM_INCIDENCE` and golden-angle turn, restated here).
fn out_of(n: Vec3<f64>) -> Vec3<f64> {
    let (along, across) = (0.984_807_753_012_208, 0.173_648_177_666_930_33);
    let (b1, b2) = n.orthonormal_basis();
    let (s, c) = geom_core::GOLDEN_ANGLE.sin_cos();
    n * along + (b1 * c + b2 * s) * across
}

/// The `normal` whose aimed ray runs along `-d` into its aim.
fn normal_for(d: Vec3<f64>) -> Vec3<f64> {
    let want = d * -1.0;
    let mut n = want;
    for _ in 0..200 {
        n = (n + (want - out_of(n))).normalize();
    }
    assert!((out_of(n) - want).norm() < 1e-14, "normal solve");
    n
}

#[test]
#[ignore = "review probe"]
fn review_5697_r2_root_off_the_arc() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let k_eps = tol.k() * tol.eps();
    let p = |x: f64, y: f64, b: f64| (Point2::new(x, y), b);
    let quarter = (PI / 8.0).tan();
    let cases: Vec<(&str, Body<f64>, f64, f64, f64)> = vec![
        // name, body, rim radius, the material's turn, the rims' heights (lo, hi) read below
        ("half cylinder", revolved_about_y(vec![p(0.0, 0.0, 0.0), p(0.5, 0.0, 0.0), p(0.5, 1.0, 0.0), p(0.0, 1.0, 0.0)], Revolution::Partial(PI), tol), 0.5, PI, 1.0),
        ("1.5pi tube", revolved_about_y(vec![p(0.2, 0.0, 0.0), p(0.5, 0.0, 0.0), p(0.5, 1.0, 0.0), p(0.2, 1.0, 0.0)], Revolution::Partial(1.5 * PI), tol), 0.5, 1.5 * PI, 1.0),
        ("1.5pi hemisphere", revolved_about_y(vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, quarter), p(0.0, 1.0, 0.0)], Revolution::Partial(1.5 * PI), tol), 1.0, 1.5 * PI, 0.0),
    ];
    for (name, body, r, turn, rim_y) in cases {
        let face = topo::test_support::edge_frames(&body, &[0.5], band)[0].face;
        // The sense of the turn: the gap's middle is out.
        let gap_mid = turn + (2.0 * PI - turn) / 2.0;
        let deep = |phi: f64| Point3::new(0.6 * r * phi.cos(), 0.3, 0.6 * r * phi.sin());
        let dir = if topo::point_in_solid(&body, deep(gap_mid), band, tol) == Ok(topo::SolidContainment::In) { -1.0 } else { 1.0 };
        let at = |phi: f64, rr: f64, y: f64| Point3::new(rr * phi.cos(), y, dir * rr * phi.sin());
        let mut tally: std::collections::BTreeMap<String, usize> = Default::default();
        for frac in [0.5, 0.25, 0.1] {
            let phi1 = turn + (2.0 * PI - turn) * frac;
            for dy in [-1.0, -0.3, 0.0, 0.3, 1.0] {
                for (k, (phi2, y2)) in [(turn / 2.0, 0.4), (turn * 0.3, 0.2), (turn * 0.7, 0.6)].into_iter().enumerate() {
                    // The root: on the carrier in the gap, `dy` bands off the rim's height.
                    let y1 = rim_y + dy * k_eps;
                    let rr1 = if name.contains("hemisphere") { (r * r - y1 * y1).max(0.0).sqrt() } else { r };
                    let p1 = at(phi1, rr1, y1);
                    // The aim: inside the face's own arc, on its carrier.
                    let rr2 = if name.contains("hemisphere") { (r * r - y2 * y2).sqrt() } else { r };
                    let p2 = at(phi2, rr2, y2);
                    let d = (p2 - p1).normalize();
                    let rd = topo::test_support::ray_at(&body, face, p2, normal_for(d), band);
                    let key = rd.split(['(', ' ', '{']).take(4).collect::<Vec<_>>().join(" ");
                    if !rd.contains("Side(Out)") {
                        println!("R2O eps={:e} {name} frac={frac} dy={dy} aim{k}: {}", tol.eps(), rd.chars().take(260).collect::<String>());
                    }
                    *tally.entry(key).or_default() += 1;
                }
            }
        }
        println!("R2O eps={:e} {name}: {tally:?}", tol.eps());
    }
}
