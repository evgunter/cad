//! Review-2 probes for PR 4256 (not for merge): every naming row at
//! `MEET` checked against an analytic germ oracle (a CSG formula over
//! the scene's half-space and cones, independent of `point_in_solid`),
//! plus a census of the edges at `MEET` left with no row, and of edges
//! given two rows. Run with `--no-capture` to read the table.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{MEET, PLATE, Pose, at, posed_box, posed_pyramid, poses};
use geom_core::{Tol, Vec3};
use topo::{
    AtRestBody, BooleanError, BooleanResult, Operand, SideCode, intersect, readback, subtract,
    union, validate_geometric,
};

fn t() -> Tol {
    Tol::witness()
}

fn corners(bearing: f64, rise: f64, r: f64) -> [[f64; 3]; 3] {
    let corner = |d: f64, r: f64| {
        let (s, c) = (bearing + d).to_radians().sin_cos();
        [r * c, r * s, rise]
    };
    [corner(15.0, r), corner(-15.0, r), corner(0.0, 0.6 * r)]
}

fn nest(base: [[f64; 3]; 3], s: f64) -> [[f64; 3]; 3] {
    [0, 1, 2].map(|i| {
        [0, 1, 2]
            .map(|k| (3.0 * base[i][k] + base[(i + 1) % 3][k] + base[(i + 2) % 3][k]) * s / 5.0)
    })
}

/// Barycentric mixes of `base`, scaled by `s`.
fn mix(base: [[f64; 3]; 3], w: [[f64; 3]; 3], s: f64) -> [[f64; 3]; 3] {
    w.map(|w| [0, 1, 2].map(|k| s * (0..3).map(|i| w[i] * base[i][k]).sum::<f64>()))
}

/// A pyramid with its apex at `MEET` over `base` (relative), wound so
/// it is outward.
fn pyr(base: &[[f64; 3]], pose: &Pose) -> AtRestBody<f64> {
    let n = base.len();
    let mut nrm = [0.0; 3];
    let mut cen = [0.0; 3];
    for i in 0..n {
        let (a, b) = (base[i], base[(i + 1) % n]);
        nrm[0] += (a[1] - b[1]) * (a[2] + b[2]);
        nrm[1] += (a[2] - b[2]) * (a[0] + b[0]);
        nrm[2] += (a[0] - b[0]) * (a[1] + b[1]);
        for k in 0..3 {
            cen[k] += a[k] / n as f64;
        }
    }
    let dot = -(0..3).map(|k| nrm[k] * cen[k]).sum::<f64>();
    let mut b: Vec<[f64; 3]> = base
        .iter()
        .map(|q| [0, 1, 2].map(|k| MEET[k] + q[k]))
        .collect();
    if dot < 0.0 {
        b.reverse();
    }
    posed_pyramid(&b, MEET, pose, t())
}

fn tet(base: [[f64; 3]; 3], pose: &Pose) -> AtRestBody<f64> {
    pyr(&base, pose)
}

/// A germ at `MEET`: a set of directions closed under scaling.
#[derive(Clone)]
enum G {
    /// The plate below its top.
    Half,
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

fn built(what: &str, r: Result<BooleanResult<f64>, BooleanError>) -> Option<AtRestBody<f64>> {
    match r {
        Ok(BooleanResult::Body(r)) => Some(r.body),
        other => {
            eprintln!("SCENE-BUILD-REFUSED {what}: {:?}", other.map(|_| ()));
            None
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Tally {
    cells: usize,
    built: usize,
    empty: usize,
    refused: usize,
    rows: usize,
    wrong: usize,
    missing: usize,
    missing_crossing: usize,
    twice: usize,
}

/// Local direction of a world vector.
fn local(pose: &Pose, w: Vec3<f64>) -> [f64; 3] {
    let o = pose.at([0.0; 3]);
    let cols = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]].map(|e| pose.at(e) - o);
    cols.map(|cv| cv.x * w.x + cv.y * w.y + cv.z * w.z)
}

fn check(
    label: &str,
    x: &AtRestBody<f64>,
    xg: &G,
    y: &AtRestBody<f64>,
    yg: &G,
    pose: &Pose,
    tally: &mut Tally,
) {
    let meet = at(pose.at(MEET));
    for (what, a, ag, b, bg, r) in [
        ("x − y", x, xg, y, yg, subtract(x, y, t())),
        ("y − x", y, yg, x, xg, subtract(y, x, t())),
        ("x ∪ y", x, xg, y, yg, union(x, y, t())),
        ("y ∪ x", y, yg, x, xg, union(y, x, t())),
        ("x ∩ y", x, xg, y, yg, intersect(x, y, t())),
        ("y ∩ x", y, yg, x, xg, intersect(y, x, t())),
    ] {
        tally.cells += 1;
        let what = format!("{label}, {}, {what}", pose.label);
        let r = match r {
            Ok(BooleanResult::Body(r)) => r,
            Ok(BooleanResult::Empty) => {
                tally.empty += 1;
                continue;
            }
            Err(e) => {
                tally.refused += 1;
                eprintln!("REFUSED {what}: {e:?}");
                continue;
            }
        };
        tally.built += 1;
        if validate_geometric(&r.body, t()).is_err() {
            eprintln!("TIER3 {what}");
        }
        for (op, own, other_g) in [(Operand::A, a, bg), (Operand::B, b, ag)] {
            let _ = ag;
            for (v, _) in own.vertices() {
                let p = readback::vertex_point(own, v).unwrap();
                if at(p) != meet {
                    continue;
                }
                for (e, _) in own.edges() {
                    let edge = own.get_edge(e).unwrap();
                    let plus = own.get_half_edge(edge.he_plus).unwrap();
                    let next = own.get_half_edge(plus.next).unwrap();
                    for (starts, far) in [(true, next.start), (false, plus.start)] {
                        let here = if starts { plus.start } else { next.start };
                        if here != v {
                            continue;
                        }
                        let dw = readback::vertex_point(own, far).unwrap() - p;
                        let want = other_g.side(local(pose, dw));
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
                        match got.as_slice() {
                            [] => {
                                tally.missing += 1;
                                if want != SideCode::On {
                                    // Does the edge cross the other's boundary at MEET?
                                    // A row is wanted wherever the edge leaves MEET
                                    // into a side; we record the miss by side.
                                    tally.missing_crossing += 1;
                                }
                                eprintln!(
                                    "MISSING {what}: {op:?} v{v:?} e{e:?} starts {starts}, oracle {want:?}"
                                );
                            }
                            [g] => {
                                tally.rows += 1;
                                if *g != want {
                                    tally.wrong += 1;
                                    eprintln!(
                                        "WRONG {what}: {op:?} v{v:?} e{e:?} starts {starts}: row {g:?}, oracle {want:?}"
                                    );
                                }
                            }
                            many => {
                                tally.rows += many.len();
                                tally.twice += 1;
                                eprintln!("TWICE {what}: {op:?} v{v:?} e{e:?}: {many:?}, oracle {want:?}");
                                if many.iter().any(|g| *g != want) {
                                    tally.wrong += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

struct Sc {
    label: &'static str,
    x: AtRestBody<f64>,
    xg: G,
    y: AtRestBody<f64>,
    yg: G,
}

fn scenes(pose: &Pose) -> Vec<Sc> {
    let p = |b| tet(b, pose);
    let arch = corners(60.0, 0.5, 0.4);
    let void = corners(120.0, -0.5, 0.4);
    let plate_b = posed_box("the plate", PLATE, pose, t());
    let plate = G::Half;
    let arch_b = p(arch);
    let one_b = built("one", union(&plate_b, &arch_b, t())).unwrap();
    let one = u(plate.clone(), c(arch));
    let bare_b = [180.0, 300.0].iter().fold(arch_b.clone(), |acc, &b| {
        built("bare", union(&acc, &p(corners(b, 0.5, 0.4)), t())).unwrap()
    });
    let bare = u(
        u(c(arch), c(corners(180.0, 0.5, 0.4))),
        c(corners(300.0, 0.5, 0.4)),
    );
    let arches_b = built("arches", union(&plate_b, &bare_b, t())).unwrap();
    let arches = u(plate.clone(), bare.clone());
    let cavity_b = built("cavity", subtract(&plate_b, &p(void), t())).unwrap();
    let cavity = dd(plate.clone(), c(void));
    let both_b = built("both", subtract(&one_b, &p(void), t())).unwrap();
    let both = dd(one.clone(), c(void));
    let island_v = nest(void, 0.7);
    let island_b = built("island", union(&cavity_b, &p(island_v), t())).unwrap();
    let island = u(cavity.clone(), c(island_v));
    let hvoid = nest(arch, 0.7);
    let hollow_b = built("hollow", subtract(&one_b, &p(hvoid), t())).unwrap();
    let hollow = dd(one.clone(), c(hvoid));
    let bare_hollow_b = built("bare hollow", subtract(&arch_b, &p(hvoid), t())).unwrap();
    let bare_hollow = dd(c(arch), c(hvoid));
    let hisle = nest(hvoid, 0.7);
    let deep_b = built("deep", union(&hollow_b, &p(hisle), t()));
    let deep = u(hollow.clone(), c(hisle));
    let cone = corners(240.0, 0.7, 0.5);
    let over = corners(50.0, 0.7, 0.5);
    let hang = corners(240.0, -0.6, 0.5);
    let hang_over = corners(130.0, -0.6, 0.5);
    let in_void = nest(void, 1.4);
    let in_island = nest(island_v, 0.7);
    let in_hollow = nest(hvoid, 0.7);
    let two_up = (corners(40.0, 0.6, 0.5), corners(280.0, 0.6, 0.5));
    let two_up_b = built("two up", union(&p(two_up.0), &p(two_up.1), t())).unwrap();
    let two_up_g = u(c(two_up.0), c(two_up.1));
    let two_down = (corners(200.0, -0.6, 0.5), corners(110.0, -0.6, 0.5));
    let two_down_b = built("two down", union(&p(two_down.0), &p(two_down.1), t())).unwrap();
    let two_down_g = u(c(two_down.0), c(two_down.1));
    // Depth 3 on the Out side: one corner in the island, one in the void
    // only, one in the arch only.
    let third = 1.0 / 3.0;
    let cross3 = mix(
        arch,
        [[third, third, third], [0.85, 0.1, 0.05], [0.5, 0.28, 0.22]],
        0.6,
    );
    // An edge on the void's face inside the arch (On at depth 2).
    let on2 = mix(
        arch,
        [[0.4, 0.4, 0.2], [0.45, 0.45, 0.1], [0.7, 0.25, 0.05]],
        0.6,
    );
    // Depth on the In side: island, void only, plate only.
    let cross_in = mix(
        void,
        [[third, third, third], [0.7, 0.2, 0.1], [1.2, -0.3, 0.1]],
        0.6,
    );
    // A partner sharing an edge ray with the arch, apart from it.
    let share = [
        arch[0],
        {
            let (s, k) = 95f64.to_radians().sin_cos();
            [0.4 * k, 0.4 * s, 0.5]
        },
        {
            let (s, k) = 85f64.to_radians().sin_cos();
            [0.24 * k, 0.24 * s, 0.5]
        },
    ];
    // The along pyramid: an edge in the arch's outer face.
    let [ap, aq, _] = arch;
    let outp = |c: [f64; 3]| {
        let (s, k) = 60f64.to_radians().sin_cos();
        [1.2 * c[0] + 0.2 * k, 1.2 * c[1] + 0.2 * s, 1.2 * c[2]]
    };
    let mid = [0, 1, 2].map(|k| 0.6 * (ap[k] + aq[k]));
    let along = [mid, outp(ap), outp(aq)];
    // A dart: a non-convex quadrilateral base, a reflex edge at the apex.
    let pol = |deg: f64, r: f64, z: f64| {
        let (s, k) = deg.to_radians().sin_cos();
        [r * k, r * s, z]
    };
    let dart = vec![
        pol(30.0, 0.45, 0.5),
        pol(90.0, 0.45, 0.5),
        pol(60.0, 0.5, 0.5),
        pol(60.0, 0.6, 0.5),
    ];
    // ordered around: 30 -> 60 far -> 90 -> 60 near (dent)
    let dart = vec![dart[0], dart[3], dart[1], dart[2]];
    let dart_b = pyr(&dart, pose);
    let dart_g = G::Fan(dart.clone());
    let dart_one = built("dart on the plate", union(&plate_b, &dart_b, t()));
    let mut v = vec![
        Sc { label: "the arches", x: p(cone), xg: c(cone), y: arches_b.clone(), yg: arches.clone() },
        Sc { label: "one standing", x: p(cone), xg: c(cone), y: one_b.clone(), yg: one.clone() },
        Sc { label: "over the arch", x: p(over), xg: c(over), y: one_b.clone(), yg: one.clone() },
        Sc { label: "over the arches", x: p(over), xg: c(over), y: arches_b.clone(), yg: arches.clone() },
        Sc { label: "hanging below the arch", x: p(hang), xg: c(hang), y: one_b.clone(), yg: one.clone() },
        Sc { label: "hanging across below the arch", x: p(hang_over), xg: c(hang_over), y: one_b.clone(), yg: one.clone() },
        Sc { label: "standing over the cavity", x: p(cone), xg: c(cone), y: cavity_b.clone(), yg: cavity.clone() },
        Sc { label: "hanging below the cavity", x: p(hang), xg: c(hang), y: cavity_b.clone(), yg: cavity.clone() },
        Sc { label: "hanging across the cavity", x: p(hang_over), xg: c(hang_over), y: cavity_b.clone(), yg: cavity.clone() },
        Sc { label: "hanging into the void", x: p(in_void), xg: c(in_void), y: cavity_b.clone(), yg: cavity.clone() },
        Sc { label: "standing on the plate", x: p(cone), xg: c(cone), y: plate_b.clone(), yg: plate.clone() },
        Sc { label: "the bare arches", x: p(cone), xg: c(cone), y: bare_b.clone(), yg: bare.clone() },
        Sc { label: "over the bare arches (pair path)", x: p(over), xg: c(over), y: bare_b.clone(), yg: bare.clone() },
        Sc { label: "two up, one over the arch", x: two_up_b.clone(), xg: two_up_g.clone(), y: one_b.clone(), yg: one.clone() },
        Sc { label: "two up over arch and void", x: two_up_b.clone(), xg: two_up_g.clone(), y: both_b.clone(), yg: both.clone() },
        Sc { label: "two down beside the void", x: two_down_b.clone(), xg: two_down_g.clone(), y: both_b.clone(), yg: both.clone() },
        Sc { label: "two up over the bare arch", x: two_up_b.clone(), xg: two_up_g.clone(), y: arch_b.clone(), yg: c(arch) },
        Sc { label: "over the arch and void", x: p(over), xg: c(over), y: both_b.clone(), yg: both.clone() },
        Sc { label: "hanging across the arch and void", x: p(hang_over), xg: c(hang_over), y: both_b.clone(), yg: both.clone() },
        Sc { label: "in the island in the void", x: p(in_island), xg: c(in_island), y: island_b.clone(), yg: island.clone() },
        Sc { label: "in the void in the arch", x: p(in_hollow), xg: c(in_hollow), y: hollow_b.clone(), yg: hollow.clone() },
        Sc { label: "along the arch", x: p(along), xg: c(along), y: one_b.clone(), yg: one.clone() },
        // New.
        Sc { label: "NEW over the bare hollow arch (pair, void partner)", x: p(over), xg: c(over), y: bare_hollow_b.clone(), yg: bare_hollow.clone() },
        Sc { label: "NEW two up over the bare hollow arch", x: two_up_b.clone(), xg: two_up_g.clone(), y: bare_hollow_b.clone(), yg: bare_hollow.clone() },
        Sc { label: "NEW over the hollow arch", x: p(over), xg: c(over), y: hollow_b.clone(), yg: hollow.clone() },
        Sc { label: "NEW crossing the void in the arch, three levels", x: p(cross3), xg: c(cross3), y: hollow_b.clone(), yg: hollow.clone() },
        Sc { label: "NEW On the void's face in the arch", x: p(on2), xg: c(on2), y: hollow_b.clone(), yg: hollow.clone() },
        Sc { label: "NEW crossing the island in the void, In side", x: p(cross_in), xg: c(cross_in), y: island_b.clone(), yg: island.clone() },
        Sc { label: "NEW standing beside the hollow arch", x: p(cone), xg: c(cone), y: hollow_b.clone(), yg: hollow.clone() },
        Sc { label: "NEW standing beside the island cavity", x: p(cone), xg: c(cone), y: island_b.clone(), yg: island.clone() },
    ];
    if let Some(deep_b) = deep_b {
        v.push(Sc { label: "NEW crossing island-in-void-in-arch (depth 3)", x: p(cross3), xg: c(cross3), y: deep_b.clone(), yg: deep.clone() });
        v.push(Sc { label: "NEW On the void face, island present", x: p(on2), xg: c(on2), y: deep_b.clone(), yg: deep.clone() });
        v.push(Sc { label: "NEW over the deep arch", x: p(over), xg: c(over), y: deep_b, yg: deep });
    }
    if let Some(sh) = built("shared ray", union(&one_b, &p(share), t())) {
        let g = u(one.clone(), c(share));
        v.push(Sc { label: "NEW over two arches sharing a ray", x: p(over), xg: c(over), y: sh.clone(), yg: g.clone() });
        v.push(Sc { label: "NEW standing beside two arches sharing a ray", x: p(cone), xg: c(cone), y: sh, yg: g });
    }
    if let Some(al) = built("along partner", union(&one_b, &p(along), t())) {
        let g = u(one.clone(), c(along));
        v.push(Sc { label: "NEW over arch with an along-partner", x: p(over), xg: c(over), y: al.clone(), yg: g.clone() });
        v.push(Sc { label: "NEW standing beside arch with along-partner", x: p(cone), xg: c(cone), y: al, yg: g });
    }
    if let Some(dart_one) = dart_one {
        let g = u(plate.clone(), dart_g.clone());
        v.push(Sc { label: "NEW over a dart (reflex partner)", x: p(over), xg: c(over), y: dart_one.clone(), yg: g.clone() });
        v.push(Sc { label: "NEW standing beside a dart", x: p(cone), xg: c(cone), y: dart_one, yg: g });
    }
    v.push(Sc { label: "NEW over a bare dart (pair)", x: p(over), xg: c(over), y: dart_b, yg: dart_g });
    v
}

#[test]
fn review2_oracle_over_every_scene_and_pose() {
    let mut total = Tally::default();
    for pose in poses() {
        for s in scenes(&pose) {
            let mut tl = Tally::default();
            check(s.label, &s.x, &s.xg, &s.y, &s.yg, &pose, &mut tl);
            eprintln!("SCENE {} | {}: {tl:?}", pose.label, s.label);
            for (a, b) in [
                (&mut total.cells, tl.cells),
                (&mut total.built, tl.built),
                (&mut total.empty, tl.empty),
                (&mut total.refused, tl.refused),
                (&mut total.rows, tl.rows),
                (&mut total.wrong, tl.wrong),
                (&mut total.missing, tl.missing),
                (&mut total.missing_crossing, tl.missing_crossing),
                (&mut total.twice, tl.twice),
            ] {
                *a += b;
            }
        }
    }
    eprintln!("TOTAL {total:?}");
    assert_eq!(total.wrong, 0, "wrong naming rows");
}
