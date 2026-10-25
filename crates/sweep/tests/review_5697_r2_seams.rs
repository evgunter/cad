//! Review R2 of PR 5697 (head only): faces with a seam inside them (an
//! edge both of whose sides are the one face) against the PR pin's
//! in-band rays. Evidence, not pins.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use core::f64::consts::PI;

use geom_core::{Band, Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{disc_of_arcs, dome, revolved_about_y, sphere_zone};
use topo::Body;

fn self_seams(body: &Body<f64>) -> usize {
    let face_of = |h| {
        body.get_half_edge(h)
            .and_then(|he| body.get_loop(he.parent_loop))
            .map(|l| l.face)
    };
    body.edges()
        .filter(|(_, e)| face_of(e.he_plus).is_some() && face_of(e.he_plus) == face_of(e.he_minus))
        .count()
}

#[test]
#[ignore = "review probe"]
fn review_5697_r2_self_seam_faces() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let k_eps = tol.k() * tol.eps();
    let p = |x: f64, y: f64| (Point2::new(x, y), 0.0);
    let bodies: Vec<(&str, Body<f64>)> = vec![
        ("disc of 2 arcs (extrude)", disc_of_arcs(2, 0.5, 1.0, tol)),
        ("rev cylinder (axis-touching)", revolved_about_y(vec![p(0.0, 0.0), p(0.5, 0.0), p(0.5, 1.0), p(0.0, 1.0)], Revolution::Full, tol)),
        ("rev tube (annular, full)", revolved_about_y(vec![p(0.2, 0.0), p(0.5, 0.0), p(0.5, 1.0), p(0.2, 1.0)], Revolution::Full, tol)),
        ("rev tube (annular, 1.5pi)", revolved_about_y(vec![p(0.2, 0.0), p(0.5, 0.0), p(0.5, 1.0), p(0.2, 1.0)], Revolution::Partial(1.5 * PI), tol)),
        ("dome", dome(1.0, tol)),
        ("sphere zone full", sphere_zone(0.5, Revolution::Full, tol)),
    ];
    for (name, body) in bodies {
        let mut tally: std::collections::BTreeMap<String, usize> = Default::default();
        for f in topo::test_support::edge_frames(&body, &[0.5], band) {
            if f.label.starts_with("Line") && f.label.contains("[Plane | Plane]") {
                continue;
            }
            for o in [0.3 * k_eps, k_eps] {
                let r = topo::test_support::ray_at(&body, f.face, f.point + f.into * o, f.normal, band);
                let k = r.split(['(', ' ']).take(3).collect::<Vec<_>>().join(" ");
                *tally.entry(k).or_default() += 1;
            }
        }
        println!("R2S eps={:e} {name}: self-seam edges {}; in-band rays {tally:?}", tol.eps(), self_seams(&body));
    }
}
