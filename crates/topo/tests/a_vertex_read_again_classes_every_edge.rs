//! **A vertex read again classes every edge it reads**: every naming
//! row at `MEET`, in every built cell of every scene, read against an
//! analytic germ (a CSG formula over the plate's half-space and the
//! pyramids' cones, independent of the kernel), and every edge at
//! `MEET` that no row classes counted.
//!
//! Each scene is a pyramid against a body holding its own contact at
//! `MEET` (`a_vertex_read_by_two_sector_passes` builds most of the same
//! ones), nested to three levels on either side of the plate's top, an
//! island in a void buried in a block (no face through `MEET`), and
//! partners whose corner is neither convex nor hollow, read as polygon
//! cones (`sectors::cone_read`): darts of four and five faces, whose
//! apex is a reflex edge, on the plate, bare, and beside an arch; the
//! apex of a pyramid over an L; a saddle, bare and under an arch; and
//! near-flat quadrilateral voids, dented 1e-3 and ten zero bands (1e-8
//! at the default ε) either way, buried with an island and under the
//! plate's top. Every scene builds in every op and classes every edge
//! at `MEET`, none wrong, none missing and none doubled; the row prints
//! each scene's tally.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{
    MEET, PLATE, Pose, apex_pyramid, at, bearing, corners, mix, near_flat, nest, nest_polygon,
    posed_box, posed_crown, poses,
};
use geom_core::{Band, Tol, Vec3};
use std::collections::BTreeMap;
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
    /// Below the fan of planes from the origin through a ring's
    /// consecutive corners, counterclockwise from above around the z
    /// axis ([`posed_crown`]).
    Crown(Vec<[f64; 3]>),
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
            G::Crown(ring) => {
                let n = ring.len();
                let at = |q: [f64; 3]| q[1].atan2(q[0]).rem_euclid(std::f64::consts::TAU);
                let a = at(d);
                (0..n).any(|i| {
                    let (p, q) = (ring[i], ring[(i + 1) % n]);
                    let (a0, mut a1) = (at(p), at(q));
                    if a1 <= a0 {
                        a1 += std::f64::consts::TAU;
                    }
                    let a = if a < a0 { a + std::f64::consts::TAU } else { a };
                    let up = [
                        p[1] * q[2] - p[2] * q[1],
                        p[2] * q[0] - p[0] * q[2],
                        p[0] * q[1] - p[1] * q[0],
                    ];
                    a < a1 && (0..3).map(|k| up[k] * d[k]).sum::<f64>() < 0.0
                })
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

fn pick(x: &(AtRestBody<f64>, G)) -> (&AtRestBody<f64>, &G) {
    (&x.0, &x.1)
}

/// A world vector in the scene's unposed frame.
fn local(pose: &Pose, w: Vec3<f64>) -> [f64; 3] {
    let o = pose.at([0.0; 3]);
    let cols = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]].map(|e| pose.at(e) - o);
    cols.map(|cv| cv.x * w.x + cv.y * w.y + cv.z * w.z)
}

/// One scene's cells, over every pose and op: what refused, and each
/// edge at `MEET` of a built result's operands, by its rows against the
/// germ.
#[derive(Default, Debug)]
struct Tally {
    built: usize,
    refused: BTreeMap<String, usize>,
    rows: usize,
    wrong: usize,
    missing: usize,
    doubled: usize,
    /// The first wrong or missing row, to name in a failure.
    first: Option<String>,
}

/// Every op on `(x, y)` in both orders, tallied: each edge at `MEET` of
/// a built result's operands has one row, the germ's side.
fn check(
    label: &str,
    x: (&AtRestBody<f64>, &G),
    y: (&AtRestBody<f64>, &G),
    pose: &Pose,
    tally: &mut Tally,
) {
    let meet = at(pose.at(MEET));
    for (what, (a, ag), (b, bg), r) in [
        ("x − y", x, y, subtract(x.0, y.0, t())),
        ("y − x", y, x, subtract(y.0, x.0, t())),
        ("x ∪ y", x, y, union(x.0, y.0, t())),
        ("y ∪ x", y, x, union(y.0, x.0, t())),
        ("x ∩ y", x, y, intersect(x.0, y.0, t())),
        ("y ∩ x", y, x, intersect(y.0, x.0, t())),
    ] {
        let what = format!("{label}, {}, {what}", pose.label);
        let r = match r {
            Ok(BooleanResult::Body(r)) => r,
            Ok(BooleanResult::Empty) => continue,
            Err(e) => {
                let kind = format!("{e:?}");
                let kind = kind.split([' ', '{', '(']).next().unwrap_or("").to_owned();
                *tally.refused.entry(kind).or_default() += 1;
                continue;
            }
        };
        tally.built += 1;
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
                        let bad = match got.as_slice() {
                            [] => {
                                tally.missing += 1;
                                true
                            }
                            [g] => {
                                tally.rows += 1;
                                let wrong = *g != want;
                                tally.wrong += usize::from(wrong);
                                wrong
                            }
                            _ => {
                                tally.doubled += 1;
                                true
                            }
                        };
                        if bad && tally.first.is_none() {
                            tally.first = Some(format!(
                                "{what}: {op:?}'s edge {e:?} at MEET reads {got:?}, the germ {want:?}"
                            ));
                        }
                    }
                }
            }
        }
    }
}

/// The ring of [`saddle`]: four corners around `MEET`, risen and fallen
/// in turn, counterclockwise from above.
fn crown() -> Vec<[f64; 3]> {
    [0.0, 90.0, 180.0, 270.0]
        .iter()
        .zip([0.2, -0.2, 0.2, -0.2])
        .map(|(&b, z)| bearing(b, 0.5, z))
        .collect()
}

/// **Every edge a vertex read again reads is classed, against the
/// germ**, in every scene at every pose (module docs). Prints each
/// scene's tally, then asserts it.
#[test]
fn every_edge_a_vertex_read_again_reads_is_classed_against_the_germ() {
    let mut tallies: BTreeMap<&'static str, Tally> = BTreeMap::new();
    let mut order: Vec<&'static str> = Vec::new();
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
        // An arch and a dart apart, one body touching itself at MEET; a
        // pyramid inside the dart's cone pairs with both.
        let dart_up = [
            bearing(200.0, 0.45, 0.5),
            bearing(230.0, 0.6, 0.5),
            bearing(260.0, 0.45, 0.5),
            bearing(230.0, 0.5, 0.5),
        ];
        let arch_dart_b = built(
            "an arch and a dart",
            union(&arch_b, &apex_pyramid(&dart_up, &pose, t()), t()),
        );
        let arch_dart = u(c(arch), G::Fan(dart_up.to_vec()));
        let in_dart = [
            bearing(222.0, 0.4, 0.4),
            bearing(238.0, 0.4, 0.4),
            bearing(230.0, 0.448, 0.4),
        ];
        // A pentagonal void buried in the block: a vertex in one pair,
        // its partner hollow.
        let pentagon: Vec<[f64; 3]> = (0..5)
            .map(|k| bearing(120.0 + 72.0 * f64::from(k), 0.3, -0.45))
            .collect();
        let pentagonal_b = built(
            "a pentagonal void buried",
            subtract(&block_b, &apex_pyramid(&pentagon, &pose, t()), t()),
        );
        let pentagonal = dd(G::All, G::Cone(pentagon.clone()));
        // Near-flat quadrilateral voids, buried in the block with an
        // island in each, and under the plate's top: a dent of -1e-3 or
        // ten zero bands in is a reflex edge at the void's apex, and one
        // out a convex one. Ten zero bands is 1e-8 at the default ε, as
        // near flat as the run can tell from flat; a dent within the
        // band builds no pyramid.
        let ten = 10.0 * Band::linear(t()).unwrap().zero();
        let quads = [-1e-3, 1e-3, -ten, ten].map(|dent| {
            let q = near_flat(dent);
            let q_b = apex_pyramid(&q, &pose, t());
            // The review's island, a quadrilateral nested in the void,
            // whose own corner ten zero bands leaves in band, so that
            // void takes a triangle.
            let isle = if dent.abs() < 1e-4 {
                nest([q[0], q[1], q[3]], 0.7).to_vec()
            } else {
                nest_polygon(&q, 0.6)
            };
            let buried = built(
                "a near-flat void buried, an island in it",
                union(
                    &built("a near-flat void buried", subtract(&block_b, &q_b, t())),
                    &apex_pyramid(&isle, &pose, t()),
                    t(),
                ),
            );
            let under = built(
                "the plate less a near-flat void",
                subtract(&plate_b, &q_b, t()),
            );
            let fan = G::Fan(q.to_vec());
            (
                (buried, u(dd(G::All, fan.clone()), G::Fan(isle))),
                (under, dd(plate.clone(), fan)),
            )
        });
        // A dart of five faces, one edge reflex; a pyramid over an L,
        // whose apex has one reflex edge among six; and a saddle, its
        // edges ridges and valleys in turn. The dart and an arch apart
        // in one body.
        let dart5 = vec![
            pol(25.0, 0.45, 0.5),
            pol(45.0, 0.62, 0.5),
            pol(75.0, 0.62, 0.5),
            pol(95.0, 0.45, 0.5),
            pol(60.0, 0.52, 0.5),
        ];
        let dart5_b = apex_pyramid(&dart5, &pose, t());
        let dart5_g = G::Fan(dart5.clone());
        let in_dart5 = [
            pol(55.0, 0.59, 0.5),
            pol(65.0, 0.59, 0.5),
            pol(60.0, 0.575, 0.5),
        ]
        .map(|q| q.map(|x| 0.8 * x));
        let dart5_beside = [
            bearing(180.0, 0.45, 0.5),
            bearing(200.0, 0.62, 0.5),
            bearing(230.0, 0.62, 0.5),
            bearing(250.0, 0.45, 0.5),
            bearing(215.0, 0.52, 0.5),
        ];
        let arch_dart5_b = built(
            "an arch and a five-face dart",
            union(&arch_b, &apex_pyramid(&dart5_beside, &pose, t()), t()),
        );
        let arch_dart5 = u(c(arch), G::Fan(dart5_beside.to_vec()));
        let in_dart5_beside = [
            bearing(210.0, 0.59, 0.5),
            bearing(220.0, 0.59, 0.5),
            bearing(215.0, 0.575, 0.5),
        ]
        .map(|q| q.map(|x| 0.8 * x));
        let ell: Vec<[f64; 3]> = [
            (0.15, 0.15),
            (0.55, 0.15),
            (0.55, 0.35),
            (0.35, 0.35),
            (0.35, 0.55),
            (0.15, 0.55),
        ]
        .iter()
        .map(|&(x, y)| [x, y, 0.5])
        .collect();
        let ell_b = apex_pyramid(&ell, &pose, t());
        let ell_g = G::Fan(ell.clone());
        let in_ell = [[0.16, 0.16, 0.4], [0.4, 0.16, 0.4], [0.24, 0.24, 0.4]];
        let across_ell = [[0.2, 0.2, 0.4], [0.4, 0.2, 0.4], [0.2, 0.4, 0.4]];
        let saddle_b = posed_crown(&crown(), [0.0, 0.0, -1.0], &pose, t());
        let saddle = G::Crown(crown());
        let arch_saddle_b = built("an arch over a saddle", union(&arch_b, &saddle_b, t()));
        let arch_saddle = u(c(arch), saddle.clone());
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
        let (cross3, on2, cross_in, in_dart) = (pyr(cross3), pyr(on2), pyr(cross_in), pyr(in_dart));
        let (in_dart5, in_dart5_beside, in_ell, across_ell) = (
            pyr(in_dart5),
            pyr(in_dart5_beside),
            pyr(in_ell),
            pyr(across_ell),
        );
        let across_saddle = pyr([
            bearing(20.0, 0.5, -0.1),
            bearing(70.0, 0.5, 0.1),
            bearing(45.0, 0.3, 0.15),
        ]);
        let mut scenes = vec![
            ("the arches", pick(&cone), (&arches_b, &arches)),
            ("one standing pyramid", pick(&cone), (&one_b, &one)),
            ("over the arch", pick(&over), (&one_b, &one)),
            ("over the arches", pick(&over), (&arches_b, &arches)),
            ("hanging below the arch", pick(&hang), (&one_b, &one)),
            (
                "hanging across below the arch",
                pick(&hang_over),
                (&one_b, &one),
            ),
            (
                "standing over the cavity",
                pick(&cone),
                (&cavity_b, &cavity),
            ),
            (
                "hanging below the cavity",
                pick(&hang),
                (&cavity_b, &cavity),
            ),
            (
                "hanging across the cavity",
                pick(&hang_over),
                (&cavity_b, &cavity),
            ),
            (
                "hanging into the void",
                pick(&in_void),
                (&cavity_b, &cavity),
            ),
            ("standing on the plate", pick(&cone), (&plate_b, &plate)),
            ("the bare arches", pick(&cone), (&bare_b, &bare)),
            ("over the bare arches", pick(&over), (&bare_b, &bare)),
            ("two up, one over the arch", pick(&two_up), (&one_b, &one)),
            (
                "two up over the arch and void",
                pick(&two_up),
                (&both_b, &both),
            ),
            (
                "two down beside the void",
                pick(&two_down),
                (&both_b, &both),
            ),
            ("over the arch and void", pick(&over), (&both_b, &both)),
            (
                "hanging across the arch and void",
                pick(&hang_over),
                (&both_b, &both),
            ),
            (
                "in the island in the void",
                pick(&in_island),
                (&island_b, &island),
            ),
            (
                "in the void in the arch",
                pick(&in_hollow),
                (&hollow_b, &hollow),
            ),
            (
                "over a void in the bare arch",
                pick(&over),
                (&bare_hollow_b, &bare_hollow),
            ),
            (
                "two up over a void in the bare arch",
                pick(&two_up),
                (&bare_hollow_b, &bare_hollow),
            ),
            (
                "over the void in the arch",
                pick(&over),
                (&hollow_b, &hollow),
            ),
            (
                "crossing the void in the arch",
                pick(&cross3),
                (&hollow_b, &hollow),
            ),
            (
                "on the void's face in the arch",
                pick(&on2),
                (&hollow_b, &hollow),
            ),
            (
                "crossing the island in the void",
                pick(&cross_in),
                (&island_b, &island),
            ),
            (
                "beside the void in the arch",
                pick(&cone),
                (&hollow_b, &hollow),
            ),
            (
                "beside the island in the void",
                pick(&cone),
                (&island_b, &island),
            ),
            (
                "crossing three levels in the arch",
                pick(&cross3),
                (&deep_b, &deep),
            ),
            (
                "on the void's face, the island in it",
                pick(&on2),
                (&deep_b, &deep),
            ),
            ("over the deep arch", pick(&over), (&deep_b, &deep)),
            (
                "crossing an island in a buried void",
                pick(&cross_in),
                (&buried_island_b, &buried_island),
            ),
            ("over a dart", pick(&over), (&dart_one_b, &dart_one)),
            ("beside a dart", pick(&cone), (&dart_one_b, &dart_one)),
            (
                "hanging into a pentagonal void",
                pick(&hang),
                (&pentagonal_b, &pentagonal),
            ),
            (
                "hanging across a pentagonal void",
                pick(&hang_over),
                (&pentagonal_b, &pentagonal),
            ),
            ("over a bare dart", pick(&over), (&dart_b, &dart_g)),
            (
                "inside a bare dart beside a bare arch",
                pick(&in_dart),
                (&arch_dart_b, &arch_dart),
            ),
            ("over a five-face dart", pick(&over), (&dart5_b, &dart5_g)),
            ("in a five-face dart", pick(&in_dart5), (&dart5_b, &dart5_g)),
            (
                "in a five-face dart beside an arch",
                pick(&in_dart5_beside),
                (&arch_dart5_b, &arch_dart5),
            ),
            ("over an L's apex", pick(&over), (&ell_b, &ell_g)),
            ("in an L's apex", pick(&in_ell), (&ell_b, &ell_g)),
            ("across an L's notch", pick(&across_ell), (&ell_b, &ell_g)),
            ("above a saddle", pick(&cone), (&saddle_b, &saddle)),
            ("below a saddle", pick(&hang), (&saddle_b, &saddle)),
            (
                "across a saddle",
                pick(&across_saddle),
                (&saddle_b, &saddle),
            ),
            (
                "above a saddle beside an arch",
                pick(&cone),
                (&arch_saddle_b, &arch_saddle),
            ),
            (
                "in an arch over a saddle",
                pick(&in_hollow),
                (&arch_saddle_b, &arch_saddle),
            ),
        ];
        // rv4289 review probes: a crown with a thin fin (two faces folded
        // at a short edge), bare and as a buried void.
        let rv = std::env::var("RV_SCENES").is_ok();
        let sph = |az: f64, el: f64, l: f64| {
            let (sa, ca) = az.to_radians().sin_cos();
            let (se, ce) = el.to_radians().sin_cos();
            [l * ca * ce, l * sa * ce, l * se]
        };
        let rv_g: f64 = std::env::var("RV_FIN").ok().map_or(1e-5, |s| s.parse().unwrap());
        let rv_lb: f64 = std::env::var("RV_LB").ok().map_or(1e-3, |s| s.parse().unwrap());
        let fin_ring = vec![
            sph(0.0, 0.0, 0.5),
            sph(rv_g / 2.0, std::env::var("RV_EL").ok().map_or(80.0, |s| s.parse().unwrap()), rv_lb),
            sph(rv_g, 0.0, 0.5),
            sph(120.0, -15.0, 0.5),
            sph(240.0, 10.0, 0.5),
        ];
        let fin_b = if rv { Some(posed_crown(&fin_ring, [0.0, 0.3, -1.0], &pose, t())) } else { None };
        let fin_g = G::Crown(fin_ring.clone());
        let fin_void_b = fin_b.as_ref().map(|f| built("a fin void buried", subtract(&block_b, f, t())));
        let fin_void_g = dd(G::All, fin_g.clone());
        let rv_probes: Vec<_> = [10.0, 70.0, 130.0, 190.0, 250.0, 310.0]
            .iter()
            .flat_map(|&b| [pyr(corners(b, 0.3, 0.5)), pyr(corners(b, -0.3, 0.5)), pyr(corners(b, 0.05, 0.5))])
            .collect();
        let rv_labels: Vec<&'static str> = (0..rv_probes.len())
            .flat_map(|k| [format!("rv fin, probe {k}"), format!("rv fin void, probe {k}")])
            .map(|l| &*Box::leak(l.into_boxed_str()))
            .collect();
        if let (Some(fb), Some(fv)) = (&fin_b, &fin_void_b) {
            for (k, pr) in rv_probes.iter().enumerate() {
                scenes.push((rv_labels[2 * k], pick(pr), (fb, &fin_g)));
                scenes.push((rv_labels[2 * k + 1], pick(pr), (fv, &fin_void_g)));
            }
            scenes.retain(|s| s.0.starts_with("rv "));
        }
        let dents = ["-1e-3", "+1e-3", "-ten zero bands", "+ten zero bands"];
        let labels: Vec<_> = dents
            .iter()
            .flat_map(|d| {
                [
                    format!("near-flat quad void {d} buried, island, hang"),
                    format!("near-flat quad void {d} buried, island, hang_over"),
                    format!("near-flat quad void {d} under the top, cone"),
                    format!("near-flat quad void {d} under the top, hang"),
                    format!("near-flat quad void {d} under the top, hang_over"),
                ]
            })
            .map(|l| &*Box::leak(l.into_boxed_str()))
            .collect();
        for (k, ((buried, buried_g), (under, under_g))) in quads.iter().enumerate() {
            let l = &labels[5 * k..5 * k + 5];
            scenes.extend([
                (l[0], pick(&hang), (buried, buried_g)),
                (l[1], pick(&hang_over), (buried, buried_g)),
                (l[2], pick(&cone), (under, under_g)),
                (l[3], pick(&hang), (under, under_g)),
                (l[4], pick(&hang_over), (under, under_g)),
            ]);
        }
        if rv { scenes.retain(|s| s.0.starts_with("rv ")); }
        for (label, x, y) in scenes {
            if !tallies.contains_key(label) {
                order.push(label);
            }
            let tally = tallies.entry(label).or_default();
            check(label, x, y, &pose, tally);
        }
    }
    let mut failures = Vec::new();
    let mut rows = 0;
    eprintln!("scene | built | refused | rows | wrong | missing | doubled");
    for label in &order {
        let tally = &tallies[label];
        rows += tally.rows;
        eprintln!(
            "{label} | {} | {:?} | {} | {} | {} | {}",
            tally.built, tally.refused, tally.rows, tally.wrong, tally.missing, tally.doubled
        );
        if !tally.refused.is_empty() || tally.wrong + tally.missing + tally.doubled > 0 {
            failures.push(format!("{label}: {tally:?}"));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
    assert!(rows > 0, "rows were read");
}
