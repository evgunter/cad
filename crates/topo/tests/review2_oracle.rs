//! REVIEW 2 PROBE (PR 4300), scratch branch only: an analytic
//! membership oracle in every pose and op, for the arch's prisms and
//! the three cones, with the unread fraction per op.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stderr)]

use crate::common;
use common::meeting::{
    Hole, MEET, PLATE, Pose, apex_pyramid, arch, arch_cone, branching_cone, comb, posed_box,
    posed_prism, poses,
};
use geom_core::{Point3, Tol};
use topo::{AtRestBody, BooleanResult, intersect, subtract, union};

fn t() -> Tol {
    Tol::witness()
}

fn in_polygon(poly: &[(f64, f64)], (y, z): (f64, f64)) -> bool {
    let mut inside = false;
    for i in 0..poly.len() {
        let ((y0, z0), (y1, z1)) = (poly[i], poly[(i + 1) % poly.len()]);
        if (z0 > z) != (z1 > z) && y < y0 + (z - z0) / (z1 - z0) * (y1 - y0) {
            inside = !inside;
        }
    }
    inside
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn plate(q: [f64; 3]) -> bool {
    (0..3).all(|i| PLATE[i].0 < q[i] && q[i] < PLATE[i].1)
}

fn cone(base: &[[f64; 3]], q: [f64; 3]) -> bool {
    let d = [0, 1, 2].map(|i| q[i] - MEET[i]);
    let poly: Vec<(f64, f64)> = base.iter().map(|c| (c[1], c[2])).collect();
    d[0] > 0.0 && d[0] <= 0.5 && {
        let s = d[0] / 0.5;
        in_polygon(&poly, (d[1] / s, d[2] / s))
    }
}

fn prism(h: &Hole, q: [f64; 3]) -> bool {
    let [o, u, v, n] = h.frame();
    let d = [0, 1, 2].map(|i| q[i] - o[i]);
    let z = dot(d, n);
    z > 0.0 && z < h.length && in_polygon(&h.profile(), (dot(d, u), dot(d, v)))
}

/// The pose's inverse, read off its images.
fn rest_of(pose: &Pose, q: Point3<f64>) -> [f64; 3] {
    let o = pose.at([0.0; 3]);
    let col = |i: usize| {
        let mut e = [0.0; 3];
        e[i] = 1.0;
        let p = pose.at(e);
        [p.x - o.x, p.y - o.y, p.z - o.z]
    };
    let d = [q.x - o.x, q.y - o.y, q.z - o.z];
    [dot(col(0), d), dot(col(1), d), dot(col(2), d)]
}

#[derive(Default, Debug)]
struct Tally {
    read: usize,
    wrong: usize,
    boundary: usize,
    escalated: usize,
}

fn read(body: &AtRestBody<f64>, q: Point3<f64>, tally: &mut Tally) -> Option<bool> {
    let band = geom_core::Band::linear(t()).unwrap();
    match topo::point_in_solid(body, q, band, t()) {
        Ok(topo::SolidContainment::In) => Some(true),
        Ok(topo::SolidContainment::Out) => Some(false),
        Ok(topo::SolidContainment::OnBoundary) => {
            tally.boundary += 1;
            None
        }
        Err(
            topo::PointInSolidError::Escalated { .. }
            | topo::PointInSolidError::Loop(topo::PointInLoopError::Escalated { .. }),
        ) => {
            tally.escalated += 1;
            None
        }
        Err(e) => panic!("point in solid at {q:?}: {e:?}"),
    }
}

#[test]
fn review2_analytic_oracle_every_pose_and_op() {
    type Member = Box<dyn Fn([f64; 3]) -> bool>;
    let scenes: Vec<(&str, Member, Box<dyn Fn(&Pose) -> AtRestBody<f64>>)> = vec![
        (
            "arch",
            Box::new(|q| arch().iter().any(|h| prism(h, q))),
            Box::new(|pose: &Pose| {
                let ps: Vec<_> = arch().iter().map(|h| posed_prism(h, pose, t())).collect();
                ps[1..].iter().fold(ps[0].clone(), |u, q| match union(&u, q, t()) {
                    Ok(BooleanResult::Body(r)) => r.body,
                    r => panic!("{r:?}"),
                })
            }),
        ),
        ("comb", Box::new(|q| cone(&comb(), q)), Box::new(|p: &Pose| apex_pyramid(&comb(), p, t()))),
        (
            "arch_cone",
            Box::new(|q| cone(&arch_cone(), q)),
            Box::new(|p: &Pose| apex_pyramid(&arch_cone(), p, t())),
        ),
        (
            "branching_cone",
            Box::new(|q| cone(&branching_cone(), q)),
            Box::new(|p: &Pose| apex_pyramid(&branching_cone(), p, t())),
        ),
    ];
    let mut total_wrong = 0;
    for (name, member, build) in &scenes {
        for pose in poses() {
            let p = posed_box("the plate", PLATE, &pose, t());
            let u = build(&pose);
            let ops: [(&str, _, fn(bool, bool) -> bool); 6] = [
                ("P-U", subtract(&p, &u, t()), |p, u| p && !u),
                ("U-P", subtract(&u, &p, t()), |p, u| u && !p),
                ("PuU", union(&p, &u, t()), |p, u| p || u),
                ("UuP", union(&u, &p, t()), |p, u| p || u),
                ("PiU", intersect(&p, &u, t()), |p, u| p && u),
                ("UiP", intersect(&u, &p, t()), |p, u| p && u),
            ];
            for (op, r, keep) in ops {
                let body = match r {
                    Ok(BooleanResult::Body(b)) => b.body,
                    other => panic!("{name} {} {op}: {other:?}", pose.label),
                };
                let mut seed: u64 = 0x2545_F491_4F6C_DD1D;
                let mut rnd = || {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    (seed >> 11) as f64 / (1u64 << 53) as f64
                };
                let mut tally = Tally::default();
                for i in 0..3000 {
                    // half within 0.6 (denser near the apex), some within 1e-4
                    let rad = if i % 10 == 0 { 1e-4 * rnd() } else { 0.6 * rnd().powi(2) };
                    let rest = [0, 1, 2].map(|k| rad.mul_add(2.0f64.mul_add(rnd(), -1.0), MEET[k]));
                    let q = pose.at(rest);
                    let back = rest_of(&pose, q);
                    let want = keep(plate(back), member(back));
                    if let Some(got) = read(&body, q, &mut tally) {
                        tally.read += 1;
                        if got != want {
                            tally.wrong += 1;
                            if tally.wrong <= 3 {
                                eprintln!("WRONG {name} {} {op} at rest {rest:?}: got {got} want {want}", pose.label);
                            }
                        }
                    }
                }
                total_wrong += tally.wrong;
                eprintln!(
                    "ORACLE {name:15} {:22} {op}: read {} wrong {} boundary {} escalated {} (unread {:.2}%)",
                    pose.label,
                    tally.read,
                    tally.wrong,
                    tally.boundary,
                    tally.escalated,
                    100.0 * (tally.boundary + tally.escalated) as f64 / 3000.0
                );
            }
        }
    }
    assert_eq!(total_wrong, 0);
}
