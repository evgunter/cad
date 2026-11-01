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
//! closes it across the line, no edge left there: around the void, the
//! union is the plate.
//!
//! A partner grooved along the ring, its reflex edge on the ring's line,
//! unites with the plate in either order: the zip keeps that edge
//! between the void's two walls.
//! A partner with a face flush with the top beside the ring builds in
//! every op: where the op keeps one of the two flush faces, the dropped
//! one is cut along the line, the cut ending in the full turn the ring
//! leaves at its tip.
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

/// `b` with its heights negated: corners under the top where `lying`
/// and the like give them above it.
fn below(b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    b.map(|[x, y, z]| [x, y, -z])
}

fn built(what: &str, r: Result<BooleanResult<f64>, BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {:?}", other.map(|_| ())),
    }
}

fn volume(b: &AtRestBody<f64>) -> f64 {
    mass_properties(b, t(), topo::test_support::pinned_resolution(t()))
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
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
    creases_of(&round(p, d, rho).map(material))
}

const N: usize = 240;

/// The `N` points at radius `rho` round the line through `p` along
/// unit `d`.
fn round(p: Point3<f64>, d: Vec3<f64>, rho: f64) -> [Point3<f64>; N] {
    let a = if d.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = (a - d * a.dot(d)).normalize();
    let v = d.cross(u);
    std::array::from_fn(|k| {
        let th = std::f64::consts::TAU * (k as f64 + 0.5) / N as f64;
        p + (u * th.cos() + v * th.sin()) * rho
    })
}

/// [`creases`] of the material read at [`round`]'s points.
fn creases_of(read: &[Option<bool>]) -> usize {
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
    /// The union's vertices, edges and faces: the top closed across
    /// the ring where the partner fills the void about it.
    union: (usize, usize, usize),
}

fn scenes() -> [Scene; 2] {
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
            union: (8, 12, 6),
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
            union: (15, 24, 12),
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
        if op.contains('∪') {
            // The top closes across the ring where the partner fills the
            // void about it: around, the whole ring, and the union is the
            // plate; under, from the apex to the partner's edge's end.
            assert_eq!(
                (
                    r.body.vertices().count(),
                    r.body.edges().count(),
                    r.body.faces().count()
                ),
                s.union,
                "{what}: the top closed across the ring"
            );
            if s.label == "around" {
                assert!(
                    (vol.last().unwrap() - volume(&plate)).abs() < 1e-12,
                    "{what}: the plate's volume"
                );
            }
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
    let [xy, yx, i, i2, u, u2] = vol[..] else {
        unreachable!("six ops build")
    };
    let what = format!("{}, {}", s.label, pose.label);
    assert!((i - i2).abs() < 1e-12, "{what}: |x ∩ y| in both orders");
    assert!((u - u2).abs() < 1e-12, "{what}: |x ∪ y| in both orders");
    assert!(
        (u + i - vx - vy).abs() < 1e-9,
        "{what}: |x ∪ y| + |x ∩ y| = |x| + |y|"
    );
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
fn an_edge_along_a_ring_places_its_germs_apart_and_the_union_closes_the_ring() {
    let poses = poses();
    for s in &scenes() {
        for pose in [&poses[0], &poses[2]] {
            check(s, pose);
        }
    }
}

/// **A partner flush with the ring's face, kept once, builds.** The
/// partner's edge runs along the ring, one of its faces lies on the top
/// beside it and the other in the far wall's wedge, on either side of
/// the ray. Where the op keeps one copy of the two flush faces and the
/// line holds two germs, the dropped copy is cut along the line, the cut
/// ending at the ring's tip in the full turn the ring leaves there.
/// Every op builds, in either order, tier 3 and 3′, its edges on the
/// line the material's creases, the two orders alike and the volumes
/// adding up; the unions close the top across the line.
#[test]
fn a_partner_flush_with_the_rings_face_builds_where_one_copy_is_kept() {
    let poses = poses();
    for (side, base) in [
        (
            "left",
            [
                bearing(60.0, 0.6, 0.0),
                bearing(150.0, 0.6, 0.0),
                bearing(20.0, 0.6, -0.7),
            ],
        ),
        (
            "right",
            [
                bearing(60.0, 0.6, 0.0),
                bearing(-30.0, 0.6, 0.0),
                bearing(100.0, 0.6, -0.7),
            ],
        ),
    ] {
        for pose in [&poses[0], &poses[2]] {
            let plate = posed_box("the plate", PLATE, pose, t());
            let void = apex_pyramid(&below(lying(0.0)), pose, t());
            let y = built("the plate less the void", subtract(&plate, &void, t()));
            let x = apex_pyramid(&base, pose, t());
            let o = pose.at(MEET);
            let end = lying(0.0)[2];
            let d = (pose.at([0, 1, 2].map(|k| MEET[k] + end[k])) - o).normalize();
            type Keep = fn(bool, bool) -> bool;
            type Made = Result<BooleanResult<f64>, BooleanError>;
            let ops: [(&str, Keep, Made); 6] = [
                ("x − y", |a, b| a && !b, subtract(&x, &y, t())),
                ("y − x", |a, b| b && !a, subtract(&y, &x, t())),
                ("y ∩ x", |a, b| a && b, intersect(&y, &x, t())),
                ("x ∪ y", |a, b| a || b, union(&x, &y, t())),
                ("x ∩ y", |a, b| a && b, intersect(&x, &y, t())),
                ("y ∪ x", |a, b| a || b, union(&y, &x, t())),
            ];
            let mut vol = Vec::new();
            for (op, keep, r) in ops {
                let what = format!("flush {side}, {}, {op}", pose.label);
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
                let on = spans(&r.body, o, d, 0.6);
                for k in 0..4 {
                    let at = 0.6 * (f64::from(k) + 0.6) / 4.0;
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
            }
            let [xy, yx, i, u, i2, u2] = vol[..] else {
                unreachable!("six ops build")
            };
            let (vx, vy) = (volume(&x), volume(&y));
            let what = format!("flush {side}, {}", pose.label);
            assert!((i - i2).abs() < 1e-9, "{what}: |x ∩ y| in both orders");
            assert!((u - u2).abs() < 1e-9, "{what}: |x ∪ y| in both orders");
            assert!(
                (xy + i - vx).abs() < 1e-9,
                "{what}: |x − y| + |x ∩ y| = |x|"
            );
            assert!(
                (yx + i - vy).abs() < 1e-9,
                "{what}: |y − x| + |x ∩ y| = |y|"
            );
            assert!(
                (u + i - vx - vy).abs() < 1e-9,
                "{what}: |x ∪ y| + |x ∩ y| = |x| + |y|"
            );
        }
    }
}

/// **A partner grooved along the ring unites with the plate in either
/// order.** A box straddling the top less a pyramid lying in it holds a
/// groove whose reflex edge runs along the ring to its end, and whose
/// faces the union drops about the line. That edge's section runs it
/// both ways against the ring's two edges, and the zip keeps it between
/// the void's two walls (`zip::kill_pair`), where `y ∪ x` used to kill
/// both its halves with the first pair. Both orders build the same body,
/// tier 3 and 3′, its edges on the line the material's creases, and
/// `|x ∪ y| + |x ∩ y| = |x| + |y|`.
#[test]
fn a_partner_grooved_along_the_ring_unites_in_either_order() {
    let poses = poses();
    for pose in [&poses[0], &poses[2]] {
        let plate = posed_box("the plate", PLATE, pose, t());
        let void = apex_pyramid(&below(lying(0.0)), pose, t());
        let y = built("the plate less the void", subtract(&plate, &void, t()));
        let straddle = posed_box(
            "the straddle",
            [(1.0, 2.3), (0.3, 1.9), (0.7, 1.3)],
            pose,
            t(),
        );
        let groove = apex_pyramid(
            &[
                bearing(20.0, 0.7, -0.5),
                bearing(100.0, 0.7, -0.5),
                bearing(60.0, 0.5, 0.0),
            ],
            pose,
            t(),
        );
        let x = built(
            "the straddle less the groove",
            subtract(&straddle, &groove, t()),
        );
        let o = pose.at(MEET);
        let end = lying(0.0)[2];
        let d = (pose.at([0, 1, 2].map(|k| MEET[k] + end[k])) - o).normalize();
        let mut vol = Vec::new();
        for (op, r) in [("x ∪ y", union(&x, &y, t())), ("y ∪ x", union(&y, &x, t()))] {
            let what = format!("grooved, {}, {op}", pose.label);
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
            assert_eq!(
                (
                    r.body.vertices().count(),
                    r.body.edges().count(),
                    r.body.faces().count()
                ),
                (24, 36, 17),
                "{what}: the union"
            );
            let on = spans(&r.body, o, d, 0.5);
            for at in [0.09, 0.21, 0.39, 0.45] {
                let p = o + d * at;
                let want = creases(p, d, 1e-3, &|q| Some(inside(&x, q)? || inside(&y, q)?));
                let got = on.iter().filter(|&&(a, b)| a < at && at < b).count();
                assert_eq!(
                    got, want,
                    "{what}: edges on the line at {at}, the material's creases"
                );
                assert!(want > 0, "{what}: the walls crease the material at {at}");
                let held = creases(p, d, 1e-3, &|q| inside(&r.body, q));
                assert_eq!(held, want, "{what}: the result's own material at {at}");
            }
            vol.push(volume(&r.body));
        }
        let i = volume(&built("x ∩ y", intersect(&x, &y, t())));
        let what = format!("grooved, {}", pose.label);
        assert!(
            (vol[0] - vol[1]).abs() < 1e-12,
            "{what}: |x ∪ y| in both orders"
        );
        assert!(
            (vol[0] + i - volume(&x) - volume(&y)).abs() < 1e-9,
            "{what}: |x ∪ y| + |x ∩ y| = |x| + |y|"
        );
    }
}

/// A convex polytope outside the kernel: its face loops and the
/// half-spaces `n · p ≤ d` it is the meet of, for
/// [`crate::union_flush_onto_edge_contact::clipped_volume`].
struct Convex {
    faces: Vec<Vec<[f64; 3]>>,
    planes: Vec<([f64; 3], f64)>,
}

impl Convex {
    /// The box `b`.
    fn cuboid(b: [(f64, f64); 3]) -> Self {
        let at = |i: usize, j: usize, k: usize| {
            [
                [b[0].0, b[0].1][i],
                [b[1].0, b[1].1][j],
                [b[2].0, b[2].1][k],
            ]
        };
        let faces = (0..3)
            .flat_map(|axis| {
                [0, 1].map(|side| {
                    let corner = |u: usize, v: usize| {
                        let mut ijk = [0; 3];
                        ijk[axis] = side;
                        ijk[(axis + 1) % 3] = u;
                        ijk[(axis + 2) % 3] = v;
                        at(ijk[0], ijk[1], ijk[2])
                    };
                    vec![corner(0, 0), corner(1, 0), corner(1, 1), corner(0, 1)]
                })
            })
            .collect();
        let planes = (0..3)
            .flat_map(|k| {
                let e: [f64; 3] = core::array::from_fn(|j| if j == k { 1.0 } else { 0.0 });
                [(e, b[k].1), (e.map(|x| -x), -b[k].0)]
            })
            .collect();
        Self { faces, planes }
    }

    /// The tetrahedron on the corners `p`.
    fn tetra(p: [[f64; 3]; 4]) -> Self {
        let v = |q: [f64; 3]| Vec3::new(q[0], q[1], q[2]);
        let mut faces = Vec::new();
        let mut planes = Vec::new();
        for skip in 0..4 {
            let f: Vec<[f64; 3]> = (0..4).filter(|&i| i != skip).map(|i| p[i]).collect();
            let n = (v(f[1]) - v(f[0])).cross(v(f[2]) - v(f[0])).normalize();
            let n = if n.dot(v(p[skip]) - v(f[0])) > 0.0 {
                -n
            } else {
                n
            };
            planes.push(([n.x, n.y, n.z], n.dot(v(f[0]))));
            faces.push(f);
        }
        Self { faces, planes }
    }

    /// Its volume, cut to every half-space of `others`.
    fn meet(&self, others: &[&Self]) -> f64 {
        let planes: Vec<([f64; 3], f64)> = others.iter().flat_map(|o| o.planes.clone()).collect();
        crate::union_flush_onto_edge_contact::clipped_volume(self.faces.clone(), &planes)
    }
}

/// The plate less the lying void at `pose`, the void as a polytope, and
/// the closed form of the plate less it.
fn ringed_plate(pose: &Pose) -> (AtRestBody<f64>, Convex, f64) {
    let void_at = below(lying(0.0));
    let plate = posed_box("the plate", PLATE, pose, t());
    let void = apex_pyramid(&void_at, pose, t());
    let y = built("the plate less the void", subtract(&plate, &void, t()));
    let corners = [0, 1, 2].map(|i| [0, 1, 2].map(|k| MEET[k] + void_at[i][k]));
    let void = Convex::tetra([corners[0], corners[1], corners[2], MEET]);
    let vy = Convex::cuboid(PLATE).meet(&[]) - void.meet(&[]);
    (y, void, vy)
}

/// The pyramid with its apex at `MEET + apex` over `base` (relative to
/// its apex), wound outward whichever way `base` runs, and its corners.
fn pyramid(apex: [f64; 3], base: &[[f64; 3]; 3], pose: &Pose) -> (AtRestBody<f64>, Convex) {
    let apex = [0, 1, 2].map(|k| MEET[k] + apex[k]);
    let v = |q: [f64; 3]| Vec3::new(q[0], q[1], q[2]);
    let n = (v(base[1]) - v(base[0])).cross(v(base[2]) - v(base[0]));
    let mut at: Vec<[f64; 3]> = base
        .iter()
        .map(|q| [0, 1, 2].map(|k| apex[k] + q[k]))
        .collect();
    // Counterclockwise seen from the apex, which lies off the base on
    // the side `n` points away from.
    if n.dot(v(base[0])) > 0.0 {
        at.reverse();
    }
    let body = common::meeting::posed_pyramid(&at, apex, pose, t());
    (body, Convex::tetra([at[0], at[1], at[2], apex]))
}

/// **The six ops of `x` and the plate less the lying void, each as the
/// closed form says**: `vi` is `|x ∩ y|`, and each op builds where its
/// volume is not zero, tier 3 and 3′, its edges on the ring's line the
/// material's creases at `at`; an op of no volume is empty. Returns the
/// spans along the line of the union's edges on it, alike in both
/// orders.
fn six_ops(
    what: &str,
    (x, y): (&AtRestBody<f64>, &AtRestBody<f64>),
    (vx, vy, vi): (f64, f64, f64),
    pose: &Pose,
    at: &[f64],
) -> Vec<(f64, f64)> {
    let o = pose.at(MEET);
    let d = (pose.at([0, 1, 2].map(|k| MEET[k] + bearing(60.0, 1.0, 0.0)[k])) - o).normalize();
    type Keep = fn(bool, bool) -> bool;
    type Made = Result<BooleanResult<f64>, BooleanError>;
    let ops: [(&str, Keep, f64, Made); 6] = [
        ("x − y", |a, b| a && !b, vx - vi, subtract(x, y, t())),
        ("y − x", |a, b| b && !a, vy - vi, subtract(y, x, t())),
        ("x ∩ y", |a, b| a && b, vi, intersect(x, y, t())),
        ("y ∩ x", |a, b| a && b, vi, intersect(y, x, t())),
        ("x ∪ y", |a, b| a || b, vx + vy - vi, union(x, y, t())),
        ("y ∪ x", |a, b| a || b, vx + vy - vi, union(y, x, t())),
    ];
    // Each sample's operands, read once for all six ops.
    let operands: Vec<[[Option<bool>; N]; 2]> = at
        .iter()
        .map(|&s| {
            let ring = round(o + d * s, d, 1e-3);
            [x, y].map(|b| ring.map(|q| inside(b, q)))
        })
        .collect();
    let mut unions = Vec::new();
    for (op, keep, want, r) in ops {
        let what = format!("{what}, {}, {op}", pose.label);
        let r = match r {
            Ok(BooleanResult::Body(r)) => r,
            Ok(BooleanResult::Empty) if want.abs() < 1e-12 => continue,
            other => panic!("{what}: builds, got {:?}", other.map(|_| ())),
        };
        assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
        assert_eq!(
            validate_pseudomanifold(&r.body, &r.contacts, t()),
            Ok(()),
            "{what}: tier 3′"
        );
        let v = volume(&r.body);
        assert!(
            (v - want).abs() < 1e-9,
            "{what}: volume {v}, closed form {want}"
        );
        let on = spans(&r.body, o, d, 1.0);
        for (&s, [xs, ys]) in at.iter().zip(&operands) {
            let p = o + d * s;
            let kept: Vec<Option<bool>> = xs
                .iter()
                .zip(ys)
                .map(|(&a, &b)| Some(keep(a?, b?)))
                .collect();
            let want = creases_of(&kept);
            let got = on.iter().filter(|&&(a, b)| a < s && s < b).count();
            assert_eq!(
                got, want,
                "{what}: edges on the line at {s}, the material's creases"
            );
            let held = creases(p, d, 1e-3, &|q| inside(&r.body, q));
            assert_eq!(held, want, "{what}: the result's own material at {s}");
        }
        if op.contains('∪') {
            let mut on = on;
            on.sort_by(|a, b| a.0.total_cmp(&b.0));
            unions.push((
                (
                    r.body.vertices().count(),
                    r.body.edges().count(),
                    r.body.faces().count(),
                ),
                on,
            ));
        }
    }
    let [(a, on), (b, _)] = &unions[..] else {
        panic!("{what}: both unions build")
    };
    assert_eq!(
        a, b,
        "{what}, {}: the union alike in both orders",
        pose.label
    );
    on.clone()
}

/// **A box with a corner or an edge at the ring's tip builds in every
/// op.** The box lies flush under the ringed top, or stands on it, with
/// a corner or an edge at the tip, where the top wraps the full turn.
/// Its face flush with the top meets the void's two walls along the
/// ring's ray in one sector: two germs along one direction, each on one
/// of the ring's two edges, which the walk orders by the side of the
/// line the top lies on beside each edge (`insert::beside_order`). The
/// sliver between them lies along the ring, both its ends the ring's
/// tips through their null-edge copies (`join::slit_tip_on`). Every op
/// builds as the closed form says, tier 3 and 3′, its edges on the line
/// the material's creases, both unions alike.
#[test]
fn a_box_at_the_rings_tip_builds_in_every_op() {
    let tip = [0, 1, 2].map(|k| MEET[k] + bearing(60.0, 0.5, 0.0)[k]);
    let poses = poses();
    for pose in [&poses[0], &poses[2]] {
        let (y, void, vy) = ringed_plate(pose);
        for (label, bx) in [
            (
                "corner at the tip",
                [(1.0, tip[0]), (0.5, tip[1]), (0.3, 1.0)],
            ),
            (
                "edge at the tip, along y",
                [(1.0, tip[0]), (0.5, 1.5), (0.3, 1.0)],
            ),
            (
                "edge at the tip, along x",
                [(1.0, 2.0), (0.5, tip[1]), (0.3, 1.0)],
            ),
            ("corner at the apex", [(1.5, 2.0), (1.0, 1.5), (0.3, 1.0)]),
            (
                "above, corner at the tip",
                [(1.0, tip[0]), (0.5, tip[1]), (1.0, 1.5)],
            ),
            (
                "above, edge at the tip",
                [(1.0, tip[0]), (0.5, 1.5), (1.0, 1.5)],
            ),
        ] {
            let x = posed_box("the box", bx, pose, t());
            let cube = Convex::cuboid(bx);
            let vx = cube.meet(&[]);
            let vi = cube.meet(&[&Convex::cuboid(PLATE)]) - void.meet(&[&cube]);
            six_ops(
                label,
                (&x, &y),
                (vx, vy, vi),
                pose,
                &[0.05, 0.15, 0.25, 0.35, 0.45, 0.55],
            );
        }
    }
}

/// **A partner whose edge lies inside the ring parts it in the union.**
/// A pyramid with its apex on the ring's ray, one face flush with the
/// top beside the ray and its edge along the ray from 0.1 or 0.2 to 0.3
/// or 0.4 of the ring's 0.5, fills the void about that stretch of the
/// line. The union keeps the top on both sides of it: the zip leaves
/// the stretch an edge of the top on both sides, between stretches of
/// the ring the union keeps, and the closing deletes it with `kemr`,
/// parting the ring into two rings of the top (`slit::part_mid_slit`).
/// Every op builds as the closed form says, tier 3 and 3′, its edges on
/// the line the material's creases, both unions alike.
#[test]
fn a_partner_inside_the_ring_parts_it_in_the_union() {
    let poses = poses();
    for pose in [&poses[0], &poses[2]] {
        let (y, void, vy) = ringed_plate(pose);
        for (from, r) in [(0.1, 0.3), (0.1, 0.2), (0.2, 0.2)] {
            for (side, base) in [
                (
                    "left",
                    [
                        bearing(60.0, r, 0.0),
                        bearing(150.0, 0.6, 0.0),
                        bearing(20.0, 0.6, -0.7),
                    ],
                ),
                (
                    "right",
                    [
                        bearing(60.0, r, 0.0),
                        bearing(-30.0, 0.6, 0.0),
                        bearing(100.0, 0.6, -0.7),
                    ],
                ),
            ] {
                let (x, partner) = pyramid(bearing(60.0, from, 0.0), &base, pose);
                // The partner lies under the top: its meet with the plate
                // less the void is it less its meet with the void.
                let vx = partner.meet(&[]);
                let vi = vx - partner.meet(&[&void]);
                let what = format!("{side}, from {from} along {r}");
                let ends = [from, from + r, 0.5];
                let at: Vec<f64> = [0.05, 0.15, 0.25, 0.35, 0.45, 0.55]
                    .into_iter()
                    .filter(|s| ends.iter().all(|e| (e - s).abs() > 0.01))
                    .collect();
                let on = six_ops(&what, (&x, &y), (vx, vy, vi), pose, &at);
                // The union's top holds two rings on the line, each two
                // edges: from the apex to the partner's stretch, and
                // from its end to the tip.
                let want = [(0.0, from), (0.0, from), (from + r, 0.5), (from + r, 0.5)];
                assert!(
                    on.len() == 4
                        && on
                            .iter()
                            .zip(want)
                            .all(|(a, b)| (a.0 - b.0).abs() < 1e-9 && (a.1 - b.1).abs() < 1e-9),
                    "{what}, {}: the union parts the ring into {want:?}, got {on:?}",
                    pose.label
                );
            }
        }
    }
}
