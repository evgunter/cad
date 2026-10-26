//! Review 5801 probes: pads across rotations of the stored face list.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::AtRestBody;

use crate::common::operands::framed_bar;
use crate::germ_plane_torus_frame::torus;

fn tori_on_bar(tori: &[(f64, f64, Vec3<f64>)], len: f64) -> topo::Body<f64> {
    let tol = Tol::witness();
    let placed = |&(big, small, at): &(f64, f64, Vec3<f64>)| {
        finished(
            "torus",
            topo::transform_rigid(
                &torus(big, small),
                &geom_core::test_support::rigid(&Affine3::translation(at), tol),
                tol,
            )
            .unwrap(),
            tol,
        )
    };
    let union = |a: &AtRestBody<f64>, b: &AtRestBody<f64>| {
        topo::union(a, b, tol).unwrap().body().unwrap().body.clone()
    };
    let tori = tori
        .iter()
        .map(placed)
        .reduce(|a, b| union(&a, &b))
        .unwrap();
    let bar = finished(
        "bar",
        framed_bar(
            Point3::new(0.3, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            0.0,
            len,
            12.0,
        ),
        tol,
    );
    union(&tori, &bar).into_body()
}

fn report(what: &str, body: &topo::Body<f64>) {
    let tol = Tol::witness();
    let regions = topo::test_support::region_carries_of(body);
    for (_, rs) in &regions {
        let shape: Vec<String> = rs
            .iter()
            .map(|(f, c)| format!("{}{}", f.len(), if c.is_some() { "C" } else { "" }))
            .collect();
        eprintln!("PROBE {what} eps={} regions(rot0)={shape:?}", tol.eps());
    }
    let n = *topo::test_support::rotate_shell_faces(&mut body.clone(), 0)
        .iter()
        .max()
        .unwrap();
    let pads: Vec<f64> = (0..n)
        .map(|k| {
            let mut b = body.clone();
            topo::test_support::rotate_shell_faces(&mut b, k);
            if std::env::var("PROBE_ROT").is_ok() {
                for (_, rs) in topo::test_support::region_carries_of(&b) {
                    let shape: Vec<String> = rs
                        .iter()
                        .map(|(f, c)| match c {
                            Some((p, cy)) => format!(
                                "{}C@({:.2},{:.2},{:.2})carry{:?}",
                                f.len(),
                                p.x,
                                p.y,
                                p.z,
                                cy
                            ),
                            None => format!("{}", f.len()),
                        })
                        .collect();
                    let c = topo::test_support::shell_references(&b);
                    eprintln!(
                        "PROBE   rot {k}: c={:?} {shape:?}",
                        c.iter().map(|(_, p)| (p.x, p.y, p.z)).collect::<Vec<_>>()
                    );
                }
            }
            topo::mass_properties(
                &topo::test_support::finished("m", topo::Body::clone(&b), tol),
                tol,
            )
            .unwrap()
            .volume_pad
        })
        .collect();
    let max = pads.iter().copied().fold(0.0, f64::max);
    let min = pads.iter().copied().fold(f64::INFINITY, f64::min);
    eprintln!(
        "PROBE {what} eps={} n={n} min={min:.4e} max={max:.4e} ratio={:.4} pads={pads:?}",
        tol.eps(),
        max / min
    );
}

#[test]
#[ignore = "review probe"]
fn probe_bodies() {
    let which = std::env::var("PROBE").unwrap_or_default();
    let bodies: Vec<(&str, Vec<(f64, f64, Vec3<f64>)>, f64)> = vec![
        (
            "R1",
            vec![
                (2.0, 0.5, Vec3::new(0.0, 0.0, 0.0)),
                (2.0, 0.5, Vec3::new(-895.0, 0.0, 5.7)),
            ],
            900.6,
        ),
        (
            "R2",
            vec![
                (2.0, 0.5, Vec3::new(0.0, 0.0, 0.0)),
                (0.2, 0.05, Vec3::new(-900.27, 0.0, 0.0)),
                (2.0, 0.5, Vec3::new(-450.0, 0.0, 6.3)),
            ],
            900.6,
        ),
        (
            "B1_two_unequal_on_top",
            vec![
                (2.0, 0.5, Vec3::new(-5.0, 0.0, 5.7)),
                (0.6, 0.15, Vec3::new(-895.0, 0.0, 5.85)),
            ],
            900.6,
        ),
        (
            "B2_home_plus_two_unequal_on_top",
            vec![
                (2.0, 0.5, Vec3::new(0.0, 0.0, 0.0)),
                (2.0, 0.5, Vec3::new(-5.0, 0.0, 5.7)),
                (0.6, 0.15, Vec3::new(-895.0, 0.0, 5.85)),
            ],
            900.6,
        ),
        (
            "B3_three_on_top",
            vec![
                (2.0, 0.5, Vec3::new(-5.0, 0.0, 5.7)),
                (0.6, 0.15, Vec3::new(-450.0, 0.0, 5.85)),
                (1.0, 0.25, Vec3::new(-895.0, 0.0, 5.8)),
            ],
            900.6,
        ),
        (
            "B4_home_and_one_on_top_near",
            vec![
                (2.0, 0.5, Vec3::new(0.0, 0.0, 0.0)),
                (2.0, 0.5, Vec3::new(-60.0, 0.0, 5.7)),
            ],
            70.6,
        ),
        (
            "B6_home_and_one_on_top_at_5",
            vec![
                (2.0, 0.5, Vec3::new(0.0, 0.0, 0.0)),
                (2.0, 0.5, Vec3::new(-5.0, 0.0, 5.7)),
            ],
            900.6,
        ),
        (
            "B7_home_and_two_on_top_far_pair",
            vec![
                (2.0, 0.5, Vec3::new(0.0, 0.0, 0.0)),
                (2.0, 0.5, Vec3::new(-885.0, 0.0, 5.7)),
                (0.6, 0.15, Vec3::new(-895.0, 0.0, 5.85)),
            ],
            900.6,
        ),
        (
            "B5_end_edge",
            vec![
                (2.0, 0.5, Vec3::new(0.0, 0.0, 0.0)),
                (2.0, 0.5, Vec3::new(-0.7, 0.0, 5.0)),
                (2.0, 0.5, Vec3::new(-895.0, 0.0, 5.7)),
            ],
            900.6,
        ),
    ];
    for (what, tori, len) in bodies {
        if !which.is_empty() && !which.split(',').any(|w| w == what) {
            continue;
        }
        let body = tori_on_bar(&tori, len);
        report(what, &body);
    }
}
