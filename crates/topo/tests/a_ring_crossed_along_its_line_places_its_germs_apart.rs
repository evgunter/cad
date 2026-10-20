//! **An edge along an operand's zero-area ring places the two germs on
//! the ring's line apart.** The plate less a pyramid lying under its
//! top holds the pyramid's ray as a ring of the top: two coincident
//! edges, top | wall each, the two walls' material apart. A pyramid
//! with an edge along that ray meets the two wedges on one line, so
//! three or more sheets meet there and the material decides the edges
//! (`crates/topo/README.md`, tier 3): `recl` reads one germ per wedge
//! pair that crosses, each on its own side of the line.
//!
//! Two scenes, at rest and turned:
//! - **around**: the partner lies round the void, its edge along the
//!   ray past the ring's end. `x − y` is the void, a pyramid; `y − x`
//!   rings the partner's edge in the top; `x ∩ y` is the partner less
//!   the void, two edges along the ring, one past it;
//! - **under**: the partner's edge ends inside the ring, its bottom
//!   crossing the void's walls at the tip, so the tip holds two more
//!   germs than the line's.
//!
//! Each result is tier 3 and 3′, its volumes add up, and at points
//! along the line its edges are the creased material sectors the
//! operands' material decides, the result's material round the line the
//! op's. The union keeps only the top on both sides of the ring and
//! closes it across the line, which the join does not build: it refuses
//! `SlitLineCrossed` (`work/cleave/a-boolean-closes-a-slits-face-across-its-line.md`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{MEET, PLATE, Pose, apex_pyramid, bearing, lying, posed_box, poses};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{
    AtRestBody, BooleanError, BooleanResult, SolidContainment, intersect, mass_properties,
    point_in_solid, readback, subtract, union, validate_geometric, validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

fn built(what: &str, r: Result<BooleanResult<f64>, BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {:?}", other.map(|_| ())),
    }
}

fn volume(b: &AtRestBody<f64>) -> f64 {
    mass_properties(b, t()).unwrap().volume
}

/// `q`'s side of `b`, `None` on its boundary or within the band of it.
fn inside(b: &AtRestBody<f64>, q: Point3<f64>) -> Option<bool> {
    match point_in_solid(b, q, Band::linear(t()).unwrap(), t()).ok()? {
        SolidContainment::In => Some(true),
        SolidContainment::Out => Some(false),
        SolidContainment::OnBoundary => None,
    }
}

/// How many material sectors round the line through `p` along unit `d`
/// are creased, read at radius `rho` from `material`: each run of
/// material between two sheets is one edge, unless it is a half-turn
/// (a face continuing), and a full turn or none is no edge. A sample on
/// a boundary joins the run before it.
fn creases(
    p: Point3<f64>,
    d: Vec3<f64>,
    rho: f64,
    material: &dyn Fn(Point3<f64>) -> Option<bool>,
) -> usize {
    const N: usize = 240;
    let a = if d.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = (a - d * a.dot(d)).normalize();
    let v = d.cross(u);
    let read: Vec<Option<bool>> = (0..N)
        .map(|k| {
            let th = std::f64::consts::TAU * (k as f64 + 0.5) / N as f64;
            material(p + (u * th.cos() + v * th.sin()) * rho)
        })
        .collect();
    let mut last = read.iter().flatten().next().copied().unwrap_or(false);
    let filled: Vec<bool> = read
        .iter()
        .map(|s| {
            last = s.unwrap_or(last);
            last
        })
        .collect();
    let Some(start) = (0..N).find(|&k| !filled[k] && filled[(k + 1) % N]) else {
        return 0;
    };
    let (mut runs, mut len): (Vec<usize>, usize) = (Vec::new(), 0);
    for j in 1..=N {
        if filled[(start + j) % N] {
            len += 1;
        } else if len > 0 {
            runs.push(len);
            len = 0;
        }
    }
    runs.iter().filter(|&&l| l.abs_diff(N / 2) > 1).count()
}

/// The result's edges lying on the segment `o + s·d`, `s ∈ [0, reach]`,
/// as their spans along it.
fn spans(b: &AtRestBody<f64>, o: Point3<f64>, d: Vec3<f64>, reach: f64) -> Vec<(f64, f64)> {
    b.edges()
        .filter_map(|(_, e)| {
            let plus = b.get_half_edge(e.he_plus).unwrap();
            let next = b.get_half_edge(plus.next).unwrap();
            let s = [plus.start, next.start].map(|v| {
                let l = readback::vertex_point(b, v).unwrap() - o;
                (l.dot(d), (l - d * l.dot(d)).norm())
            });
            s.iter()
                .all(|&(s, off)| off < 1e-7 && (-1e-9..=reach + 1e-9).contains(&s))
                .then(|| (s[0].0.min(s[1].0), s[0].0.max(s[1].0)))
        })
        .collect()
}

/// A scene: the partner `x`, the plate less the void `y`, and the far
/// end of the line they share, relative to [`MEET`].
struct Scene {
    label: &'static str,
    x: [[f64; 3]; 3],
    void: [[f64; 3]; 3],
    reach: f64,
    /// The ring's end along the line and, where the partner's edge ends
    /// inside the ring, where it does: no sample sits on either.
    ends: &'static [f64],
}

fn scenes() -> [Scene; 2] {
    let below = |b: [[f64; 3]; 3]| b.map(|[x, y, z]| [x, y, -z]);
    let phi = 250.0;
    [
        Scene {
            label: "around",
            x: below([
                bearing(60.0, 0.6, 0.0),
                bearing(30.0, 0.6, 0.7),
                bearing(90.0, 0.6, 0.7),
            ]),
            void: below(lying(0.0)),
            reach: 0.6,
            ends: &[0.5],
        },
        Scene {
            label: "under",
            x: [
                bearing(phi, 0.3, 0.0),
                bearing(phi + 40.0, 0.4, -0.3),
                bearing(phi - 40.0, 0.4, -0.3),
            ],
            void: [
                bearing(phi + 20.0, 0.4, -0.5),
                bearing(phi - 20.0, 0.4, -0.5),
                bearing(phi, 0.5, 0.0),
            ],
            reach: 0.5,
            ends: &[0.3],
        },
    ]
}

/// The direction of the line the scene's partner and void share: the
/// void's corner on the top.
fn line_end(s: &Scene) -> [f64; 3] {
    *s.void.iter().find(|c| c[2] == 0.0).unwrap()
}

fn check(s: &Scene, pose: &Pose) {
    let plate = posed_box("the plate", PLATE, pose, t());
    let void = apex_pyramid(&s.void, pose, t());
    let x = apex_pyramid(&s.x, pose, t());
    let y = built("the plate less the void", subtract(&plate, &void, t()));
    let o = pose.at(MEET);
    let end = line_end(s);
    let d = (pose.at([0, 1, 2].map(|k| MEET[k] + end[k])) - o).normalize();
    type Keep = fn(bool, bool) -> bool;
    type Made = Result<BooleanResult<f64>, BooleanError>;
    let ops: [(&str, Keep, Made); 6] = [
        ("x − y", |a, b| a && !b, subtract(&x, &y, t())),
        ("y − x", |a, b| b && !a, subtract(&y, &x, t())),
        ("x ∩ y", |a, b| a && b, intersect(&x, &y, t())),
        ("y ∩ x", |a, b| a && b, intersect(&y, &x, t())),
        ("x ∪ y", |a, b| a || b, union(&x, &y, t())),
        ("y ∪ x", |a, b| a || b, union(&y, &x, t())),
    ];
    let mut vol = Vec::new();
    for (op, keep, r) in ops {
        let what = format!("{}, {}, {op}", s.label, pose.label);
        if op.contains('∪') {
            assert!(
                matches!(r, Err(BooleanError::SlitLineCrossed { .. })),
                "{what}: the union closes the top across the ring, got {:?}",
                r.map(|_| ())
            );
            continue;
        }
        let r = match r {
            Ok(BooleanResult::Body(r)) => r,
            other => panic!("{what}: builds, got {:?}", other.map(|_| ())),
        };
        assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
        assert_eq!(
            validate_pseudomanifold(&r.body, &r.contacts, t()),
            Ok(()),
            "{what}: tier 3′"
        );
        vol.push(volume(&r.body));
        let on = spans(&r.body, o, d, s.reach);
        for k in 0..4 {
            let at = s.reach * (f64::from(k) + 0.6) / 4.0;
            assert!(
                s.ends.iter().all(|e| (e - at).abs() > 0.01),
                "{what}: a sample on a vertex"
            );
            let p = o + d * at;
            let want = creases(p, d, 1e-3, &|q| Some(keep(inside(&x, q)?, inside(&y, q)?)));
            let got = on.iter().filter(|&&(a, b)| a < at && at < b).count();
            assert_eq!(
                got, want,
                "{what}: edges on the line at {at}, the material's creases"
            );
            let held = creases(p, d, 1e-3, &|q| inside(&r.body, q));
            assert_eq!(held, want, "{what}: the result's own material at {at}");
        }
        if op == "x − y" && s.label == "around" {
            // The partner holds the void whole: the difference is the void.
            assert_eq!(
                (
                    r.body.vertices().count(),
                    r.body.edges().count(),
                    r.body.faces().count()
                ),
                (4, 6, 4),
                "{what}: the void, a pyramid"
            );
            assert!(
                (vol[0] - volume(&void)).abs() < 1e-12,
                "{what}: the void's volume"
            );
        }
    }
    let (vx, vy) = (volume(&x), volume(&y));
    let [xy, yx, i, i2] = vol[..] else {
        unreachable!("four ops build")
    };
    let what = format!("{}, {}", s.label, pose.label);
    assert!((i - i2).abs() < 1e-12, "{what}: |x ∩ y| in both orders");
    assert!(
        (xy + i - vx).abs() < 1e-9,
        "{what}: |x − y| + |x ∩ y| = |x|"
    );
    assert!(
        (yx + i - vy).abs() < 1e-9,
        "{what}: |y − x| + |x ∩ y| = |y|"
    );
}

#[test]
fn an_edge_along_a_ring_places_its_germs_apart_in_every_op_it_builds() {
    let poses = poses();
    for s in &scenes() {
        for pose in [&poses[0], &poses[2]] {
            check(s, pose);
        }
    }
}
