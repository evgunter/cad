//! The counterexample-SEARCH row for the in-plane walk's spiric
//! crossing on built bodies: every planar face a spiric bounds on the
//! vessel cavity and the hollowed Klein wall pair, probed near and far
//! at random against its boundary sampled densely from each edge's own
//! carrier. Folded in from PR 3924's review.
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

test_utils::gated_to![
    "crates/topo/src/splitting/spiric_arc.rs",
    "crates/topo/src/splitting/containment.rs",
    "crates/topo/src/boolean/solid_contain.rs",
    "crates/geom/src/curves.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/sweep/tests/common/",
    "crates/sweep/tests/pis_arc_capped_poses.rs",
];

use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use topo::Body;

use crate::common::torus_walls::vessel_cavity;
use crate::pis_arc_capped_poses::{body_surface, loop_carriers, loop_half_edges, loop_vertex};

fn tol() -> Tol {
    Tol::witness()
}

/// **Every spiric-bounded planar face of the vessel cavity at three
/// wall thicknesses, and of the hollowed Klein wall pair, read near and
/// far.** Points
/// are placed `δ ∈ [10⁻⁷, 10⁻³]` m off the spiric along its in-plane
/// normal — the band the dense-grid row above skips — and at random over
/// the face's neighbourhood, and every answer is checked against the
/// face's boundary sampled densely from its edges' own carriers. A
/// refusal is counted, never a wrong answer.
#[test]
fn every_spiric_face_reads_near_and_far_against_a_dense_boundary() {
    let band = Band::linear(tol()).expect("the witness band");
    let mut rng = test_utils::fuzz::start("spiric_faces");
    let (mut asked, mut refused, mut faces) = (0usize, 0usize, 0usize);
    let mut wrong = Vec::new();
    let mut bodies: Vec<(String, Body<f64>)> = [1.0 / 128.0, 1.0 / 64.0, 1.0 / 32.0]
        .into_iter()
        .map(|t| (format!("vessel t={t}"), vessel_cavity(t).1))
        .collect();
    // The shell7 "klein elbow by hand" wall pair, hollowed by 0.01 at the
    // simultaneous door (the assembled shell `topo::shell` refuses at
    // check 7 is this body's caps).
    for (r, t) in [(0.25, 0.01), (0.25, 0.02)] {
        let mut elbow = crate::common::torus_walls::klein_elbow(vec![
            bulge_loop(vec![
                (Point2::new(-(r + 0.025), 0.0), 1.0),
                (Point2::new(r + 0.025, 0.0), 1.0),
            ]),
            bulge_loop(vec![
                (Point2::new(-(r - 0.025), 0.0), 1.0),
                (Point2::new(r - 0.025, 0.0), 1.0),
            ]),
        ]);
        let moves = crate::common::charts::hollow_moves(&elbow, t);
        match topo::offset_charts_together(&mut elbow, &moves, band, tol()) {
            Ok(_) => bodies.push((format!("elbow t={t}"), elbow)),
            Err(e) => println!("elbow t={t}: the door refuses {e:?}"),
        }
    }
    for (label, cavity) in &bodies {
        let t = label;
        let cavity = cavity.clone();
        let spiric_faces: Vec<_> = cavity
            .faces()
            .filter(|(_, f)| {
                matches!(
                    body_surface(&cavity, f.surface),
                    Some(geom::Surface::Plane { .. })
                ) && core::iter::once(f.outer)
                    .chain(f.rings.iter().copied())
                    .any(|lk| {
                        loop_carriers(&cavity, lk)
                            .iter()
                            .any(|c| matches!(c, geom::Curve3::Spiric { .. }))
                    })
            })
            .map(|(k, _)| k)
            .collect();
        println!("{label}: {} spiric faces", spiric_faces.len());
        for face in spiric_faces {
            faces += 1;
            let data = cavity.get_face(face).expect("face");
            let Some(geom::Surface::Plane { normal, .. }) = body_surface(&cavity, data.surface)
            else {
                unreachable!()
            };
            let origin = loop_vertex(&cavity, data.outer);
            let across = normal.cross(Vec3::new(0.3, 0.5, 0.7)).normalize();
            let up = normal.cross(across);
            let planar = |p: Point3<f64>| ((p - origin).dot(across), (p - origin).dot(up));
            let mut segments = Vec::new();
            let mut spirics = Vec::new();
            for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
                for he in loop_half_edges(&cavity, lk) {
                    let edge = cavity
                        .get_edge(cavity.get_half_edge(he).expect("half edge").edge)
                        .expect("edge");
                    let curve = cavity
                        .get_curve_geom(edge.curve)
                        .and_then(|c| c.certified())
                        .expect("a certified edge");
                    let (t0, t1) = curve.params();
                    let is_spiric = matches!(curve.carrier(), geom::Curve3::Spiric { .. });
                    if is_spiric {
                        spirics.push((curve.carrier().clone(), t0, t1));
                    }
                    let n = if is_spiric { 20_000 } else { 2000 };
                    let at = |k: usize| {
                        planar(curve.carrier().eval(t0 + (t1 - t0) * k as f64 / n as f64))
                    };
                    for k in 0..n {
                        segments.push((at(k), at(k + 1)));
                    }
                }
            }
            let truth = |(x, y): (f64, f64)| -> (bool, f64) {
                let mut inside = false;
                let mut gap = f64::INFINITY;
                for &((ax, ay), (bx, by)) in &segments {
                    if (ay > y) != (by > y) && x < ax + (y - ay) / (by - ay) * (bx - ax) {
                        inside = !inside;
                    }
                    let (dx, dy) = (bx - ax, by - ay);
                    let len2 = dx * dx + dy * dy;
                    let s = if len2 > 0.0 {
                        (((x - ax) * dx + (y - ay) * dy) / len2).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    gap = gap.min((x - ax - s * dx).hypot(y - ay - s * dy));
                }
                (inside, gap)
            };
            let (lo, hi) = segments.iter().fold(
                ((f64::MAX, f64::MAX), (f64::MIN, f64::MIN)),
                |(l, h), &(p, _)| ((l.0.min(p.0), l.1.min(p.1)), (h.0.max(p.0), h.1.max(p.1))),
            );
            let mut probes = Vec::new();
            for _ in 0..test_utils::fuzz::scaled(40) {
                let pad = 0.2 * (hi.0 - lo.0).max(hi.1 - lo.1);
                probes.push((
                    rng.range(lo.0 - pad, hi.0 + pad),
                    rng.range(lo.1 - pad, hi.1 + pad),
                    1e-6,
                ));
            }
            for (carrier, t0, t1) in &spirics {
                for _ in 0..test_utils::fuzz::scaled(40) {
                    let v = t0 + (t1 - t0) * rng.range(0.01, 0.99);
                    let p = planar(carrier.eval(v));
                    let tg = carrier.deriv(v);
                    let (tx, ty) = (tg.dot(across), tg.dot(up));
                    let l = tx.hypot(ty);
                    let delta = 10f64.powf(rng.range(-7.0, -3.0))
                        * if rng.below(2) == 0 { 1.0 } else { -1.0 };
                    probes.push((
                        p.0 - ty / l * delta,
                        p.1 + tx / l * delta,
                        0.5 * delta.abs(),
                    ));
                }
            }
            for (x, y, min_gap) in probes {
                let (want, gap) = truth((x, y));
                if gap < min_gap.clamp(5e-8, 1e-6) {
                    continue;
                }
                asked += 1;
                let q = origin + across * x + up * y;
                match topo::test_support::point_in_face(&cavity, face, q, band) {
                    Ok(Some(got)) if got == want => {}
                    Ok(got) => wrong.push(format!(
                        "t={t} {face:?} ({x}, {y}) gap {gap}: want {want}, got {got:?}"
                    )),
                    Err(e) => {
                        refused += 1;
                        if refused <= 5 {
                            println!("t={t} {face:?} ({x}, {y}) gap {gap}: refused {e:?}");
                        }
                    }
                }
            }
        }
    }
    println!("{faces} spiric faces, {asked} probes asked, {refused} refused");
    assert!(
        faces >= 3 && asked >= 500,
        "not vacuous: {faces} faces, {asked} asked"
    );
    assert!(
        wrong.is_empty(),
        "{} wrong:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}
