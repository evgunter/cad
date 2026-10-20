//! **The fin crown winds as a solid, and a crown wound against itself
//! refuses at rest.** An independent generalised winding number over a
//! body's planar faces (each face's loops as a fan of signed solid
//! angles, Van Oosterom–Strackee) reads points just off every face. A
//! solid's boundary winds 0 or 1 about every point off it.
//!
//! The fin crown's bottom ([`FIN_BOTTOM`]) lies where every ring edge
//! faces it from inside, and the crown winds 0 or 1 at every pose. From
//! the bottom `(0, 0.2, −0.6)` the fan's face on the fin's long edge
//! `c0 c1` leans across the slit and cuts through the faces beyond it,
//! and the boundary winds −1 near the slit's mouth: no solid. Check 7
//! reads that crown by every aim that decides, and where two of its rays
//! read opposite sides the at-rest gate refuses it
//! (`ValidationError::OrientationOpposed`), whatever face order meets
//! first.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::meeting::{
    FIN_BOTTOM, apex_pyramid, corners, crown_body, fin, posed_crown, posed_crown_kept, poses,
};
use geom_core::{Band, Tol};
use topo::{Body, BooleanResult, LoopBoundary, ValidationError, union};

fn t() -> Tol {
    Tol::witness()
}

type P = [f64; 3];

fn sub(a: P, b: P) -> P {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: P, b: P) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: P, b: P) -> P {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// The signed solid angle of the triangle `a b c` seen from `p`.
fn solid_angle(p: P, a: P, b: P, c: P) -> f64 {
    let (a, b, c) = (sub(a, p), sub(b, p), sub(c, p));
    let (la, lb, lc) = (dot(a, a).sqrt(), dot(b, b).sqrt(), dot(c, c).sqrt());
    let den = la * lb * lc + dot(a, b) * lc + dot(a, c) * lb + dot(b, c) * la;
    2.0 * dot(a, cross(b, c)).atan2(den)
}

/// Each face as its loops' corners, each loop run so that the face's
/// region carries its outward normal (its plane's normal by its sense),
/// with that normal.
fn faces(body: &Body<f64>) -> Vec<(Vec<Vec<P>>, P)> {
    body.faces()
        .map(|(_, f)| {
            let geom::Surface::Plane { normal, .. } = body.get_surface(f.surface).unwrap() else {
                panic!("the crown's faces are planar");
            };
            let s = if f.sense { 1.0 } else { -1.0 };
            let n = [s * normal.x, s * normal.y, s * normal.z];
            let loops = core::iter::once(f.outer)
                .chain(f.rings.iter().copied())
                .enumerate()
                .filter_map(|(k, l)| {
                    let LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary else {
                        return None;
                    };
                    let mut pts = Vec::new();
                    let mut h = first;
                    loop {
                        let he = body.get_half_edge(h).unwrap();
                        let v = body.get_vertex(he.start).unwrap();
                        let q = body.get_point(v.point).unwrap();
                        pts.push([q.x, q.y, q.z]);
                        h = he.next;
                        if h == first {
                            break;
                        }
                    }
                    let area = (0..pts.len()).fold([0.0; 3], |a, i| {
                        let c = cross(pts[i], pts[(i + 1) % pts.len()]);
                        [a[0] + c[0], a[1] + c[1], a[2] + c[2]]
                    });
                    if (k == 0) != (dot(area, n) > 0.0) {
                        pts.reverse();
                    }
                    Some(pts)
                })
                .collect();
            (loops, n)
        })
        .collect()
}

/// The winding number of the faces about `p`.
fn winding(fs: &[(Vec<Vec<P>>, P)], p: P) -> f64 {
    let mut sum = 0.0;
    for (loops, _) in fs {
        for l in loops {
            for i in 1..l.len() - 1 {
                sum += solid_angle(p, l[0], l[i], l[i + 1]);
            }
        }
    }
    sum / (4.0 * core::f64::consts::PI)
}

/// The windings, rounded, about points just off each face on either
/// side: from each face's outer loop's centroid towards each corner, a
/// fifth to 99% of the way, set off along the normal by 5e-8 to 1e-6.
fn windings(body: &Body<f64>) -> std::collections::BTreeSet<i64> {
    let fs = faces(body);
    let mut seen = std::collections::BTreeSet::new();
    for (loops, n) in &fs {
        let l = &loops[0];
        let m = l.len() as f64;
        let c = [0, 1, 2].map(|j| l.iter().map(|q| q[j]).sum::<f64>() / m);
        for q in l {
            for w in [0.2, 0.5, 0.8, 0.95, 0.99] {
                let at = [0, 1, 2].map(|j| c[j] + w * (q[j] - c[j]));
                for d in [-1e-6, -2e-7, -5e-8, 5e-8, 2e-7, 1e-6] {
                    let p = [0, 1, 2].map(|j| at[j] + d * n[j]);
                    seen.insert(winding(&fs, p).round() as i64);
                }
            }
        }
    }
    seen
}

/// The fin's crown builds only where its fold is out of band.
fn fin_builds() -> bool {
    let builds = Band::linear(t()).unwrap().zero() < 1e-8;
    if !builds {
        test_utils::vacuity::stood_down(
            "the fin crown's windings",
            "the fin's fold is in band at this ε and its crown does not build",
        );
    }
    builds
}

/// **The fin crown winds 0 or 1 about every point sampled, at every
/// pose, and the crown on the bottom `(0, 0.2, −0.6)` winds −1** — the
/// probe sees the region where it is.
#[test]
fn the_fin_crown_winds_as_a_solid_where_the_old_bottom_wound_minus_one() {
    if !fin_builds() {
        return;
    }
    for pose in poses() {
        let crown = crown_body(&fin(), FIN_BOTTOM, &pose, t());
        let seen = windings(&crown);
        assert!(
            seen.iter().all(|w| [0, 1].contains(w)),
            "{}: the fin crown winds {seen:?}",
            pose.label
        );
        assert!(
            posed_crown_kept(&fin(), FIN_BOTTOM, &pose, t()).is_ok(),
            "{}: the fin crown is a finished body",
            pose.label
        );
        let old = crown_body(&fin(), [0.0, 0.2, -0.6], &pose, t());
        let seen = windings(&old);
        assert!(
            seen.contains(&-1),
            "{}: the crown on the old bottom winds −1 somewhere, read {seen:?}",
            pose.label
        );
    }
}

/// **Every union of the fin crown with the probe pyramids winds as a
/// solid**: probes 1, 2 and 4 of the vertex-read battery
/// (`a_vertex_read_again_classes_every_edge`), in both orders, at every
/// pose, wind 0 or 1 about every point sampled.
#[test]
fn every_union_of_the_fin_crown_with_its_probes_winds_as_a_solid() {
    if !fin_builds() {
        return;
    }
    let mut read = 0;
    for pose in poses() {
        let crown = posed_crown(&fin(), FIN_BOTTOM, &pose, t());
        for (k, (bearing, rise)) in [(1, (130.0, 0.3)), (2, (130.0, -0.3)), (4, (250.0, -0.3))] {
            let probe = apex_pyramid(&corners(bearing, rise, 0.5), &pose, t());
            for (order, r) in [
                ("x ∪ y", union(&probe, &crown, t())),
                ("y ∪ x", union(&crown, &probe, t())),
            ] {
                let Ok(BooleanResult::Body(r)) = r else {
                    panic!("{}: probe {k}, {order} builds, read {r:?}", pose.label);
                };
                let seen = windings(&r.body);
                assert!(
                    seen.iter().all(|w| [0, 1].contains(w)),
                    "{}: probe {k}, {order} winds {seen:?}",
                    pose.label
                );
                read += 1;
            }
        }
    }
    assert_eq!(read, 30, "every union is read");
}

/// **A crown wound against itself refuses at rest where two aimed rays
/// read opposite sides**, which check 7 finds by reading every aim that
/// decides, whatever face order meets first. The crown on the bottom
/// `(0, 0.2, −0.6)` passed at every pose while the first ray that
/// decided answered. At the two flipped poses no aimed ray's closest
/// crossing enters the −1 region, so every one reads `Out` and the crown
/// passes: check 7 reads a side, it does not look for a boundary that
/// passes through itself.
#[test]
fn a_crown_wound_against_itself_refuses_at_rest_by_two_opposed_rays() {
    if !fin_builds() {
        return;
    }
    let opposed: Vec<_> = poses()
        .into_iter()
        .filter(|pose| {
            let read = posed_crown_kept(&fin(), [0.0, 0.2, -0.6], pose, t());
            let errors = read.err().unwrap_or_default();
            assert!(
                errors
                    .iter()
                    .all(|e| matches!(e, ValidationError::OrientationOpposed { .. })),
                "{}: the crown refuses only on two opposed rays, read {errors:?}",
                pose.label
            );
            !errors.is_empty()
        })
        .map(|pose| pose.label)
        .collect();
    assert_eq!(
        opposed,
        ["at rest", "turned about z", "turned"],
        "the poses at which two aimed rays read opposite sides"
    );
}

/// **What reading every aim costs**: the at-rest validation of a prism
/// over a regular `n`-gon (`n + 2` planar faces), timed, and again with
/// the aimed rays' crossings (`side_at_infinity_advance` verdicts)
/// counted. A probe,
/// not a gate: `cargo nextest run -p topo --test all --release
/// --run-ignored only -E 'test(check_7_reads_every_aim_at_a_cost)'
/// --no-capture`.
#[test]
#[ignore = "a cost probe: it times, it asserts nothing"]
fn check_7_reads_every_aim_at_a_cost() {
    use geom_core::k_stats::Bracket;
    for n in [16, 64, 256, 1024] {
        let profile: Vec<(f64, f64)> = (0..n)
            .map(|k| {
                let a = core::f64::consts::TAU * k as f64 / n as f64;
                (a.cos(), a.sin())
            })
            .collect();
        let body = common::prism::<f64>(&profile, 1.0, t()).body;
        let start = std::time::Instant::now();
        let read = topo::validate_geometric(&body, t());
        let took = start.elapsed();
        let bracket = Bracket::open();
        let _ = topo::validate_geometric(&body, t());
        let log = bracket.finish();
        let rays = log
            .verdicts
            .iter()
            .filter(|v| v.predicate == "side_at_infinity_advance")
            .count();
        eprintln!(
            "prism over a {n}-gon ({} faces): {:?} in {took:?}, {rays} aimed-ray advance verdicts",
            n + 2,
            read.is_ok()
        );
    }
}
