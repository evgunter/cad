//! **The fin crown is a solid.** Three reads of a body of planar
//! faces, each independent of the at-rest gate:
//! - the census, tier 3′ with no contacts
//!   (`topo::validate_pseudomanifold`), which decides every pair of cells
//!   at the run's band;
//! - an exact crossing test: every edge of each face against every face
//!   it shares no vertex with, orientations taken in integers. It counts
//!   strict piercings only, so it is blind to an edge with an end on the
//!   other face, and to coplanar or grazing contact; the census covers
//!   those at the band;
//! - a generalised winding number (each face's loops as a fan of signed
//!   solid angles, Van Oosterom–Strackee) about points just off every
//!   face. A solid's boundary winds 0 or 1 about every point off it.
//!
//! The fin crown ([`posed_fin_crown`]) passes all three at every pose. The
//! crown on the single-apex bottom `(0, 0.2, −0.6)` is no solid: the fan's
//! face on the fin's long edge `c0 c1` leans across the slit and cuts
//! through the faces beyond it, and the boundary winds −1 near the slit's
//! mouth. The census refuses it, and the at-rest gate, which runs no
//! census, passes it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::meeting::{
    apex_pyramid, corners, crown_body, fin, fin_crown_body, posed_crown_kept, posed_fin_crown,
    poses,
};
use geom_core::{Band, Tol};
use num_bigint::BigInt;
use topo::{Body, BooleanResult, ContactRecords, LoopBoundary, union, validate_pseudomanifold};

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
/// with that normal. The solid-angle winding has two more copies
/// (`m5_pr12_fix_pass.rs`, `strut_cover_on_cylinder_pairs.rs`) and no
/// shared home yet.
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
                .map(|(k, l)| {
                    let LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary else {
                        panic!("a loop of a finished planar body is a cycle");
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
                    pts
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
/// Each lies within 1e-3 of a whole number before it is rounded (f64
/// solid angles of a sliver face seen from 1e-7 off it lose about 1e-5),
/// so a boundary that leaks reads as one.
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
                    let w = winding(&fs, p);
                    assert!((w - w.round()).abs() < 1e-3, "a winding of {w} at {p:?}");
                    seen.insert(w.round() as i64);
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
        let crown = posed_fin_crown(&pose, t());
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

/// `x` exactly, as an integer in units of `2^-1100`: every finite `f64`
/// is one. (`boolean::sectors::cone_fuzz`'s `Rat::of` does the same, but
/// it is crate-private test code, out of this suite's reach.)
fn exact(x: f64) -> BigInt {
    if x == 0.0 {
        return BigInt::from(0);
    }
    let bits = x.to_bits();
    let exp = ((bits >> 52) & 0x7ff) as i64;
    let frac = bits & ((1 << 52) - 1);
    let (m, e) = if exp == 0 {
        (frac, -1074)
    } else {
        (frac | (1 << 52), exp - 1075)
    };
    let v = BigInt::from(m) << usize::try_from(e + 1100).unwrap();
    if x < 0.0 { -v } else { v }
}

type X = [BigInt; 3];

fn xs(p: P) -> X {
    p.map(exact)
}

/// The sign of the volume `(b − a) · ((c − a) × (d − a))`, exactly.
fn orient(a: &X, b: &X, c: &X, d: &X) -> i8 {
    let s = |u: &X, v: &X| -> X { [&u[0] - &v[0], &u[1] - &v[1], &u[2] - &v[2]] };
    let (u, v, w) = (s(b, a), s(c, a), s(d, a));
    let det = &u[0] * (&v[1] * &w[2] - &v[2] * &w[1]) - &u[1] * (&v[0] * &w[2] - &v[2] * &w[0])
        + &u[2] * (&v[0] * &w[1] - &v[1] * &w[0]);
    match det.sign() {
        num_bigint::Sign::Plus => 1,
        num_bigint::Sign::Minus => -1,
        num_bigint::Sign::NoSign => 0,
    }
}

/// Whether the segment `p q` passes through the triangle `t`'s interior,
/// crossing its plane strictly.
fn pierces(p: &X, q: &X, t: &[X; 3]) -> bool {
    let (s1, s2) = (
        orient(&t[0], &t[1], &t[2], p),
        orient(&t[0], &t[1], &t[2], q),
    );
    if s1 * s2 >= 0 {
        return false;
    }
    let o = [
        orient(p, q, &t[0], &t[1]),
        orient(p, q, &t[1], &t[2]),
        orient(p, q, &t[2], &t[0]),
    ];
    o.iter().all(|&x| x > 0) || o.iter().all(|&x| x < 0)
}

/// The pairs of faces of a body of triangles that cross: an edge of one
/// through the other's interior, where the edge shares no end with it.
fn crossings(body: &Body<f64>) -> usize {
    let tris: Vec<[P; 3]> = faces(body)
        .into_iter()
        .map(|(loops, _)| {
            let l = &loops[0];
            assert_eq!(l.len(), 3, "the crown's faces are triangles");
            [l[0], l[1], l[2]]
        })
        .collect();
    let mut crossing = 0;
    for (i, a) in tris.iter().enumerate() {
        for b in &tris[i + 1..] {
            let (ax, bx) = (a.map(xs), b.map(xs));
            let through = |t: &[P; 3], tx: &[X; 3], u: &[P; 3]| {
                (0..3).any(|k| {
                    let (p, q) = (t[k], t[(k + 1) % 3]);
                    !u.contains(&p) && !u.contains(&q) && pierces(&xs(p), &xs(q), tx)
                })
            };
            if through(a, &bx, b) || through(b, &ax, a) {
                crossing += 1;
            }
        }
    }
    crossing
}

/// **The fin crown is a solid at every pose, by the census, the exact
/// crossing test and the windings; the crown on the bottom
/// `(0, 0.2, −0.6)` is refused by the census, crosses itself and winds
/// −1, though the at-rest gate passes it.**
#[test]
fn the_fin_crown_is_a_solid_where_the_old_crown_crossed_itself() {
    if !fin_builds() {
        return;
    }
    let census = |b: &Body<f64>| validate_pseudomanifold(b, &ContactRecords::default(), t());
    for pose in poses() {
        let crown = fin_crown_body(&pose, t());
        assert_eq!(
            census(&crown),
            Ok(()),
            "{}: the census passes the fin crown",
            pose.label
        );
        assert_eq!(
            crossings(&crown),
            0,
            "{}: the fin crown's faces cross",
            pose.label
        );
        let seen = windings(&crown);
        assert!(
            seen.iter().all(|w| [0, 1].contains(w)),
            "{}: the fin crown winds {seen:?}",
            pose.label
        );
        let old = crown_body(&fin(), [0.0, 0.2, -0.6], &pose, t());
        assert!(
            census(&old).is_err(),
            "{}: the census refuses the old crown",
            pose.label
        );
        assert!(
            crossings(&old) > 0,
            "{}: the old crown crosses itself",
            pose.label
        );
        assert!(
            windings(&old).contains(&-1),
            "{}: the old crown winds −1 somewhere",
            pose.label
        );
        assert!(
            posed_crown_kept(&fin(), [0.0, 0.2, -0.6], &pose, t()).is_ok(),
            "{}: the at-rest gate passes the old crown, which runs no census",
            pose.label
        );
    }
}
