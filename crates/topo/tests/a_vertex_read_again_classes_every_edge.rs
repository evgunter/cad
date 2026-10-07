//! **A vertex read again classes every edge it reads, or refuses
//! typed**: every naming row at `MEET`, in every built cell of every
//! scene, read against an analytic germ (a CSG formula over the plate's
//! half-space and the pyramids' cones, independent of the kernel), and
//! every edge at `MEET` that no row classes counted.
//!
//! Each scene is a pyramid against a body holding its own contact at
//! `MEET` (`a_vertex_read_by_two_sector_passes` builds the same ones),
//! nested to three levels on either side of the plate's top, an island
//! in a void buried in a block (no face through `MEET`), and a
//! dart, whose apex is a reflex edge no corner reading reads. Every
//! scene classes every edge at `MEET` in every op, with no row wrong
//! and none doubled, except:
//! - a pyramid touching the top beside a dart refuses `VertexReadTwice`;
//! - a pyramid paired with a bare dart alone keeps that pair's reading,
//!   which is none.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{MEET, PLATE, Pose, apex_pyramid, at, corners, mix, nest, posed_box, poses};
use geom_core::{Tol, Vec3};
use topo::{
    AtRestBody, BooleanError, BooleanResult, Operand, SideCode, intersect, readback, subtract,
    union, validate_geometric,
};

fn t() -> Tol {
    Tol::witness()
}

fn tet(base: [[f64; 3]; 3], pose: &Pose) -> AtRestBody<f64> {
    apex_pyramid(&base, pose, t())
}

/// A germ at `MEET`: a set of directions closed under scaling.
#[derive(Clone)]
enum G {
    /// The plate below its top.
    Half,
    /// A block around `MEET`.
    All,
    /// The cone over a convex base (apex at the origin).
    Cone(Vec<[f64; 3]>),
    /// The cone over a possibly non-convex planar polygon, by fan.
    Fan(Vec<[f64; 3]>),
    U(Box<G>, Box<G>),
    D(Box<G>, Box<G>),
}

fn solve3(m: [[f64; 3]; 3], d: [f64; 3]) -> [f64; 3] {
    // columns of m are m[0], m[1], m[2]
    let det = |a: [f64; 3], b: [f64; 3], c: [f64; 3]| {
        a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
            + a[2] * (b[0] * c[1] - b[1] * c[0])
    };
    let d0 = det(m[0], m[1], m[2]);
    [
        det(d, m[1], m[2]) / d0,
        det(m[0], d, m[2]) / d0,
        det(m[0], m[1], d) / d0,
    ]
}

impl G {
    fn has(&self, d: [f64; 3]) -> bool {
        match self {
            G::Half => d[2] < 0.0,
            G::All => true,
            G::Cone(b) => {
                // a convex cone over a triangle; a larger convex base by fan
                (1..b.len() - 1).any(|i| {
                    let l = solve3([b[0], b[i], b[i + 1]], d);
                    l.iter().all(|&x| x > 0.0)
                }) || (b.len() > 3 && G::Fan(b.clone()).has(d))
            }
            G::Fan(b) => {
                // a planar polygon: project the ray through the base plane
                let n = b.len();
                let (p0, p1, p2) = (b[0], b[1], b[2]);
                let u = [0, 1, 2].map(|k| p1[k] - p0[k]);
                let v = [0, 1, 2].map(|k| p2[k] - p0[k]);
                let nn = [
                    u[1] * v[2] - u[2] * v[1],
                    u[2] * v[0] - u[0] * v[2],
                    u[0] * v[1] - u[1] * v[0],
                ];
                let num: f64 = (0..3).map(|k| nn[k] * p0[k]).sum();
                let den: f64 = (0..3).map(|k| nn[k] * d[k]).sum();
                if den.abs() < 1e-300 || num / den <= 0.0 {
                    return false;
                }
                let s = num / den;
                let q = d.map(|x| x * s);
                // point in polygon (2D, drop the dominant axis)
                let ax = (0..3)
                    .max_by(|&i, &j| nn[i].abs().total_cmp(&nn[j].abs()))
                    .unwrap();
                let (i, j) = match ax {
                    0 => (1, 2),
                    1 => (2, 0),
                    _ => (0, 1),
                };
                let mut inside = false;
                for k in 0..n {
                    let (a, c) = (b[k], b[(k + 1) % n]);
                    if (a[j] > q[j]) != (c[j] > q[j])
                        && q[i] < (c[i] - a[i]) * (q[j] - a[j]) / (c[j] - a[j]) + a[i]
                    {
                        inside = !inside;
                    }
                }
                inside
            }
            G::U(a, b) => a.has(d) || b.has(d),
            G::D(a, b) => a.has(d) && !b.has(d),
        }
    }

    /// The germ's side of direction `d`: In / Out where every nearby
    /// direction agrees, On where they disagree.
    fn side(&self, d: [f64; 3]) -> SideCode {
        let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        let d = d.map(|x| x / l);
        let (mut i, mut o) = (0, 0);
        for k in 0..400 {
            let z = 1.0 - (2.0 * f64::from(k) + 1.0) / 400.0;
            let (s, c) = (2.399_963 * f64::from(k)).sin_cos();
            let r = z.mul_add(-z, 1.0).sqrt();
            let u = [r * c, r * s, z];
            let q = [0, 1, 2].map(|j| d[j] + 1e-7 * u[j]);
            if self.has(q) {
                i += 1;
            } else {
                o += 1;
            }
        }
        match (i, o) {
            (_, 0) => SideCode::In,
            (0, _) => SideCode::Out,
            _ => SideCode::On,
        }
    }
}

fn c(b: [[f64; 3]; 3]) -> G {
    G::Cone(b.to_vec())
}
fn u(a: G, b: G) -> G {
    G::U(Box::new(a), Box::new(b))
}
fn dd(a: G, b: G) -> G {
    G::D(Box::new(a), Box::new(b))
}

fn built(what: &str, r: Result<BooleanResult<f64>, BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {:?}", other.map(|_| ())),
    }
}

/// What a scene does at `MEET`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Expect {
    /// Every op that builds classes every edge at `MEET`.
    Classes,
    /// Every op refuses `VertexReadTwice`.
    Refuses,
    /// A single pair whose partner reads neither way keeps its own
    /// reading: no row.
    Unread,
}

fn pick(x: &(AtRestBody<f64>, G)) -> (&AtRestBody<f64>, &G) {
    (&x.0, &x.1)
}

/// A world vector in the scene's unposed frame.
fn local(pose: &Pose, w: Vec3<f64>) -> [f64; 3] {
    let o = pose.at([0.0; 3]);
    let cols = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]].map(|e| pose.at(e) - o);
    cols.map(|cv| cv.x * w.x + cv.y * w.y + cv.z * w.z)
}

/// Every op on `(x, y)` in both orders: each edge at `MEET` of a built
/// result's operands has one row, the germ's side, or none where
/// `expect` says so. Returns the rows read.
fn check(
    label: &str,
    x: (&AtRestBody<f64>, &G),
    y: (&AtRestBody<f64>, &G),
    expect: Expect,
    pose: &Pose,
) -> usize {
    let meet = at(pose.at(MEET));
    let mut rows = 0;
    for (what, (a, ag), (b, bg), r) in [
        ("x − y", x, y, subtract(x.0, y.0, t())),
        ("y − x", y, x, subtract(y.0, x.0, t())),
        ("x ∪ y", x, y, union(x.0, y.0, t())),
        ("y ∪ x", y, x, union(y.0, x.0, t())),
        ("x ∩ y", x, y, intersect(x.0, y.0, t())),
        ("y ∩ x", y, x, intersect(y.0, x.0, t())),
    ] {
        let what = format!("{label}, {}, {what}", pose.label);
        let r = match (r, expect) {
            (Err(BooleanError::VertexReadTwice { .. }), Expect::Refuses) => continue,
            (other, Expect::Refuses) => panic!(
                "{what}: refuses VertexReadTwice, got {:?}",
                other.map(|_| ())
            ),
            (Ok(BooleanResult::Body(r)), _) => r,
            (Ok(BooleanResult::Empty), _) => continue,
            (Err(e), _) => panic!("{what}: builds, got {e:?}"),
        };
        assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
        for (op, own, other) in [(Operand::A, a, bg), (Operand::B, b, ag)] {
            for (v, _) in own.vertices() {
                let p = readback::vertex_point(own, v).unwrap();
                if at(p) != meet {
                    continue;
                }
                for (e, _) in own.edges() {
                    let edge = own.get_edge(e).unwrap();
                    let plus = own.get_half_edge(edge.he_plus).unwrap();
                    let next = own.get_half_edge(plus.next).unwrap();
                    for (starts, here, far) in [
                        (true, plus.start, next.start),
                        (false, next.start, plus.start),
                    ] {
                        if here != v {
                            continue;
                        }
                        let want =
                            other.side(local(pose, readback::vertex_point(own, far).unwrap() - p));
                        let got: Vec<SideCode> = r
                            .naming
                            .edge_classes
                            .iter()
                            .filter(|row| {
                                row.operand == op
                                    && row.vertex == v
                                    && row.edge == e
                                    && row.starts == starts
                            })
                            .map(|row| row.class)
                            .collect();
                        match (got.as_slice(), expect) {
                            ([], Expect::Unread) => {}
                            ([], _) => panic!(
                                "{what}: {op:?}'s edge {e:?} at MEET has no row; the germ reads {want:?}"
                            ),
                            ([g], _) => {
                                rows += 1;
                                assert_eq!(
                                    *g, want,
                                    "{what}: {op:?}'s edge {e:?} at MEET against the germ"
                                );
                            }
                            (many, _) => {
                                panic!("{what}: {op:?}'s edge {e:?} at MEET has rows {many:?}")
                            }
                        }
                    }
                }
            }
        }
    }
    rows
}

/// **Every edge a vertex read again reads is classed, against the
/// germ**, in every scene at every pose (module docs).
#[test]
fn every_edge_a_vertex_read_again_reads_is_classed_against_the_germ() {
    let mut rows = 0;
    for pose in poses() {
        let p = |b: [[f64; 3]; 3]| tet(b, &pose);
        let arch = corners(60.0, 0.5, 0.4);
        let void = corners(120.0, -0.5, 0.4);
        let plate_b = posed_box("the plate", PLATE, &pose, t());
        let plate = G::Half;
        let arch_b = p(arch);
        let one_b = built("one", union(&plate_b, &arch_b, t()));
        let one = u(plate.clone(), c(arch));
        let others = [180.0, 300.0].map(|b| corners(b, 0.5, 0.4));
        let bare_b = others.iter().fold(arch_b.clone(), |acc, &b| {
            built("bare", union(&acc, &p(b), t()))
        });
        let bare = u(u(c(arch), c(others[0])), c(others[1]));
        let arches_b = built("arches", union(&plate_b, &bare_b, t()));
        let arches = u(plate.clone(), bare.clone());
        let cavity_b = built("cavity", subtract(&plate_b, &p(void), t()));
        let cavity = dd(plate.clone(), c(void));
        let both_b = built("both", subtract(&one_b, &p(void), t()));
        let both = dd(one.clone(), c(void));
        let isle = nest(void, 0.7);
        let island_b = built("island", union(&cavity_b, &p(isle), t()));
        let island = u(cavity.clone(), c(isle));
        let hvoid = nest(arch, 0.7);
        let hollow_b = built("hollow", subtract(&one_b, &p(hvoid), t()));
        let hollow = dd(one.clone(), c(hvoid));
        let bare_hollow_b = built("bare hollow", subtract(&arch_b, &p(hvoid), t()));
        let bare_hollow = dd(c(arch), c(hvoid));
        let hisle = nest(hvoid, 0.7);
        let deep_b = built("deep", union(&hollow_b, &p(hisle), t()));
        let deep = u(hollow.clone(), c(hisle));
        let two = |x: [[f64; 3]; 3], y: [[f64; 3]; 3]| {
            (built("two", union(&p(x), &p(y), t())), u(c(x), c(y)))
        };
        let two_up = two(corners(40.0, 0.6, 0.5), corners(280.0, 0.6, 0.5));
        let two_down = two(corners(200.0, -0.6, 0.5), corners(110.0, -0.6, 0.5));
        let third = 1.0 / 3.0;
        let cross3 = mix(
            arch,
            [[third, third, third], [0.75, 0.15, 0.1], [0.45, 0.22, 0.33]],
            0.6,
        );
        let on2 = mix(
            arch,
            [[0.4, 0.4, 0.2], [0.45, 0.45, 0.1], [0.7, 0.25, 0.05]],
            0.6,
        );
        let cross_in = mix(
            void,
            [[third, third, third], [0.7, 0.2, 0.1], [1.2, -0.3, 0.1]],
            0.6,
        );
        let pol = |deg: f64, r: f64, z: f64| {
            let (s, k) = deg.to_radians().sin_cos();
            [r * k, r * s, z]
        };
        // A dart: a quadrilateral base with a dent, its apex a reflex edge.
        let dart = vec![
            pol(30.0, 0.45, 0.5),
            pol(60.0, 0.6, 0.5),
            pol(90.0, 0.45, 0.5),
            pol(60.0, 0.5, 0.5),
        ];
        let dart_b = apex_pyramid(&dart, &pose, t());
        let dart_g = G::Fan(dart.clone());
        let dart_one_b = built("a dart on the plate", union(&plate_b, &dart_b, t()));
        let dart_one = u(plate.clone(), dart_g.clone());
        // A void buried in a block, `MEET` inside it, an island in the
        // void: the vertex there is in pairs alone, its outermost cone a
        // void.
        let block_b = posed_box(
            "a block around MEET",
            [(1.0, 2.0), (0.5, 1.5), (0.3, 1.5)],
            &pose,
            t(),
        );
        let buried_b = built("a buried void", subtract(&block_b, &p(void), t()));
        let buried_island_b = built("an island in it", union(&buried_b, &p(isle), t()));
        let buried_island = u(dd(G::All, c(void)), c(isle));
        let pyr = |b: [[f64; 3]; 3]| (p(b), c(b));
        let (cone, over) = (pyr(corners(240.0, 0.7, 0.5)), pyr(corners(50.0, 0.7, 0.5)));
        let (hang, hang_over) = (
            pyr(corners(240.0, -0.6, 0.5)),
            pyr(corners(130.0, -0.6, 0.5)),
        );
        let (in_void, in_island, in_hollow) = (
            pyr(nest(void, 1.4)),
            pyr(nest(isle, 0.7)),
            pyr(nest(hvoid, 0.7)),
        );
        let (cross3, on2, cross_in) = (pyr(cross3), pyr(on2), pyr(cross_in));
        use Expect::{Classes, Refuses, Unread};
        for (label, x, y, expect) in [
            ("the arches", pick(&cone), (&arches_b, &arches), Classes),
            ("one standing pyramid", pick(&cone), (&one_b, &one), Classes),
            ("over the arch", pick(&over), (&one_b, &one), Classes),
            (
                "over the arches",
                pick(&over),
                (&arches_b, &arches),
                Classes,
            ),
            (
                "hanging below the arch",
                pick(&hang),
                (&one_b, &one),
                Classes,
            ),
            (
                "hanging across below the arch",
                pick(&hang_over),
                (&one_b, &one),
                Classes,
            ),
            (
                "standing over the cavity",
                pick(&cone),
                (&cavity_b, &cavity),
                Classes,
            ),
            (
                "hanging below the cavity",
                pick(&hang),
                (&cavity_b, &cavity),
                Classes,
            ),
            (
                "hanging across the cavity",
                pick(&hang_over),
                (&cavity_b, &cavity),
                Classes,
            ),
            (
                "hanging into the void",
                pick(&in_void),
                (&cavity_b, &cavity),
                Classes,
            ),
            (
                "standing on the plate",
                pick(&cone),
                (&plate_b, &plate),
                Classes,
            ),
            ("the bare arches", pick(&cone), (&bare_b, &bare), Classes),
            (
                "over the bare arches",
                pick(&over),
                (&bare_b, &bare),
                Classes,
            ),
            (
                "two up, one over the arch",
                pick(&two_up),
                (&one_b, &one),
                Classes,
            ),
            (
                "two up over the arch and void",
                pick(&two_up),
                (&both_b, &both),
                Classes,
            ),
            (
                "two down beside the void",
                pick(&two_down),
                (&both_b, &both),
                Classes,
            ),
            (
                "over the arch and void",
                pick(&over),
                (&both_b, &both),
                Classes,
            ),
            (
                "hanging across the arch and void",
                pick(&hang_over),
                (&both_b, &both),
                Classes,
            ),
            (
                "in the island in the void",
                pick(&in_island),
                (&island_b, &island),
                Classes,
            ),
            (
                "in the void in the arch",
                pick(&in_hollow),
                (&hollow_b, &hollow),
                Classes,
            ),
            (
                "over a void in the bare arch",
                pick(&over),
                (&bare_hollow_b, &bare_hollow),
                Classes,
            ),
            (
                "two up over a void in the bare arch",
                pick(&two_up),
                (&bare_hollow_b, &bare_hollow),
                Classes,
            ),
            (
                "over the void in the arch",
                pick(&over),
                (&hollow_b, &hollow),
                Classes,
            ),
            (
                "crossing the void in the arch",
                pick(&cross3),
                (&hollow_b, &hollow),
                Classes,
            ),
            (
                "on the void's face in the arch",
                pick(&on2),
                (&hollow_b, &hollow),
                Classes,
            ),
            (
                "crossing the island in the void",
                pick(&cross_in),
                (&island_b, &island),
                Classes,
            ),
            (
                "beside the void in the arch",
                pick(&cone),
                (&hollow_b, &hollow),
                Classes,
            ),
            (
                "beside the island in the void",
                pick(&cone),
                (&island_b, &island),
                Classes,
            ),
            (
                "crossing three levels in the arch",
                pick(&cross3),
                (&deep_b, &deep),
                Classes,
            ),
            (
                "on the void's face, the island in it",
                pick(&on2),
                (&deep_b, &deep),
                Classes,
            ),
            ("over the deep arch", pick(&over), (&deep_b, &deep), Classes),
            (
                "crossing an island in a buried void",
                pick(&cross_in),
                (&buried_island_b, &buried_island),
                Classes,
            ),
            (
                "over a dart",
                pick(&over),
                (&dart_one_b, &dart_one),
                Refuses,
            ),
            (
                "beside a dart",
                pick(&cone),
                (&dart_one_b, &dart_one),
                Refuses,
            ),
            ("over a bare dart", pick(&over), (&dart_b, &dart_g), Unread),
        ] {
            rows += check(label, x, y, expect, &pose);
        }
    }
    assert!(rows > 0, "rows were read");
}
