//! Measurement probes (embed-run): finished bodies whose face is narrow
//! beside a shared edge and wide elsewhere along its line. Records what
//! tier 3, the census and the embedding question read on each. Not a
//! row of the suite: prints, asserts nothing but construction.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::Body;
use crate::boolean::ContactRecords;
use crate::construction::{FaceSurface, MefSite, MevSite};
use crate::test_support_fixtures::{describe_as_intersections, line, plane_every_face};
use crate::{EdgeKey, FaceKey, HalfEdgeKey, VertexKey};
use geom_core::{Band, Point3, Tol};
use std::collections::{BTreeMap, BTreeSet};

type P = [f64; 3];

fn tol() -> Tol {
    Tol::witness()
}

fn pt(p: P) -> Point3<f64> {
    Point3::new(p[0], p[1], p[2])
}

/// A polyhedron by Euler ops from named corners and faces (each a CCW
/// cycle of corner names about its outward normal). Greedy: struts
/// along face cycles, a face closed by `mef` once one edge of it is
/// missing and its run is contiguous in the seed face's loop.
#[allow(dead_code)]
pub(super) struct Built {
    pub body: Body<f64>,
    pub faces: BTreeMap<&'static str, FaceKey>,
    pub verts: BTreeMap<&'static str, VertexKey>,
}

fn nxt(body: &Body<f64>, he: HalfEdgeKey) -> HalfEdgeKey {
    body.get_half_edge(he).unwrap().next
}
fn start(body: &Body<f64>, he: HalfEdgeKey) -> VertexKey {
    body.get_half_edge(he).unwrap().start
}

pub(super) fn build(
    corners: &[(&'static str, P)],
    faces: &[(&'static str, &[&'static str])],
) -> Built {
    let pos: BTreeMap<&str, P> = corners.iter().copied().collect();
    let t = tol();
    let mut body = Body::<f64>::new();
    let f0 = faces[0].1;
    let seed = body.mvfs(pt(pos[f0[0]]), true).unwrap();
    let big = seed.face;
    let mut verts: BTreeMap<&'static str, VertexKey> = BTreeMap::new();
    verts.insert(f0[0], seed.vertex);
    let m = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(pos[f0[1]]),
            line(pt(pos[f0[0]]), pt(pos[f0[1]])),
            t,
        )
        .unwrap();
    verts.insert(f0[1], m.vertex);
    let mut edges: BTreeSet<(&str, &str)> = BTreeSet::new();
    let ek = |a: &'static str, b: &'static str| if a < b { (a, b) } else { (b, a) };
    edges.insert(ek(f0[0], f0[1]));
    let mut closed: BTreeMap<&'static str, FaceKey> = BTreeMap::new();
    let n_faces = faces.len();
    let mut guard = 0;
    while closed.len() < n_faces - 1 {
        guard += 1;
        assert!(guard < 1000, "builder stuck: closed {:?}", closed.keys());
        // 1. close a face with one edge missing and a contiguous run.
        let mut progressed = false;
        for &(name, cyc) in faces {
            if closed.contains_key(name) {
                continue;
            }
            let k = cyc.len();
            if !cyc.iter().all(|c| verts.contains_key(c)) {
                continue;
            }
            let missing: Vec<usize> = (0..k)
                .filter(|&i| !edges.contains(&ek(cyc[i], cyc[(i + 1) % k])))
                .collect();
            if missing.len() != 1 {
                continue;
            }
            // Rotate so the missing edge is cyc[k-1] -> cyc[0].
            let r = (missing[0] + 1) % k;
            let rot: Vec<&'static str> = (0..k).map(|i| cyc[(r + i) % k]).collect();
            let Some(he1) = body.find_half_edge(big, verts[rot[0]], verts[rot[1]]) else {
                continue;
            };
            let mut he = he1;
            let mut ok = true;
            for i in 0..k - 1 {
                if start(&body, he) != verts[rot[i]] {
                    ok = false;
                    break;
                }
                he = nxt(&body, he);
            }
            if !ok || start(&body, he) != verts[rot[k - 1]] {
                continue;
            }
            let made = body
                .mef(
                    MefSite::Chords { he1, he2: he },
                    line(pt(pos[rot[0]]), pt(pos[rot[k - 1]])),
                    FaceSurface::Inherit,
                    t,
                )
                .unwrap();
            edges.insert(ek(rot[0], rot[k - 1]));
            closed.insert(name, made.face);
            progressed = true;
            break;
        }
        if progressed {
            continue;
        }
        // 2. a strut along some face: c_{i-1} -> c_i present, c_{i+1} new.
        'outer: for &(name, cyc) in faces {
            if closed.contains_key(name) {
                continue;
            }
            let k = cyc.len();
            for i in 0..k {
                let (a, b, c) = (cyc[(i + k - 1) % k], cyc[i], cyc[(i + 1) % k]);
                if verts.contains_key(c) || !edges.contains(&ek(a, b)) {
                    continue;
                }
                let Some(inb) = body.find_half_edge(big, verts[a], verts[b]) else {
                    continue;
                };
                let h = nxt(&body, inb);
                let s = body
                    .mev(
                        MevSite::Fan { he1: h, he2: h },
                        pt(pos[c]),
                        line(pt(pos[b]), pt(pos[c])),
                        t,
                    )
                    .unwrap();
                verts.insert(c, s.vertex);
                edges.insert(ek(b, c));
                progressed = true;
                break 'outer;
            }
        }
        assert!(progressed, "no move: closed {:?}", closed.keys());
    }
    let last = faces
        .iter()
        .find(|(n, _)| !closed.contains_key(n))
        .unwrap()
        .0;
    closed.insert(last, big);
    plane_every_face(&mut body, t);
    describe_as_intersections(&mut body, t);
    Built {
        body,
        faces: closed,
        verts,
    }
}

fn shared_edge(body: &Body<f64>, a: FaceKey, b: FaceKey) -> Option<EdgeKey> {
    body.edges().map(|(k, _)| k).find(|&e| {
        let f = crate::edge_faces(body, e);
        f.contains(&a) && f.contains(&b)
    })
}

fn short(s: String) -> String {
    if s.len() > 1500 {
        format!("{}…[{} chars]", &s[..1500], s.len())
    } else {
        s
    }
}

fn budgets() -> crate::clearance::Budgets {
    crate::clearance::Budgets {
        max_cell_depth: crate::clearance::DEFAULT_MAX_CELL_DEPTH,
        max_cell_pairs: 65_536,
    }
}

/// Every door on `b`, and the question on each of `pairs` and on all.
pub(super) fn measure(what: &str, b: &Built, pairs: &[(&str, &str)], runs: &[(&str, &str)]) {
    let t = tol();
    let band = Band::linear(t).unwrap();
    let body = &b.body;
    let name_of = |f: FaceKey| {
        b.faces
            .iter()
            .find(|(_, k)| **k == f)
            .map_or("?", |(n, _)| *n)
    };
    println!("=== {what} ===");
    println!(
        "faces: {}",
        b.faces
            .iter()
            .map(|(n, k)| format!("{n}={k:?}"))
            .collect::<Vec<_>>()
            .join(" ")
    );
    let tier12 = crate::validate::validate_closed(body);
    println!("tier 1-2 (validate_closed): {}", short(format!("{tier12:?}")));
    let geo = crate::geometric(body, t);
    let (geometric, door) = match &geo {
        Ok(g) => (g.clone(), "tier-3 local battery (topo::geometric)"),
        Err(e) => {
            println!(
                "tier 3 local battery (topo::geometric) REFUSES: {}",
                short(format!("{e:?}"))
            );
            (
                crate::Geometric::fixture(body),
                "Geometric::fixture (tier 3 refused; runs only)",
            )
        }
    };
    let full = crate::validate_geometric(body, t);
    println!(
        "tier 3 (validate_geometric): {}",
        short(format!("{full:?}"))
    );
    match crate::finish(body.clone(), t) {
        Ok(at) => {
            let census = at.validate_pseudomanifold(&ContactRecords::default(), t);
            println!(
                "tier 3' census (AtRestBody::validate_pseudomanifold): {}",
                short(format!("{census:?}"))
            );
        }
        Err(e) => println!("finish refuses: {} findings", e.as_slice().len()),
    }
    println!("question asked on: {door}");
    let facts = geometric.facts();
    for &(x, y) in runs {
        let (fx, fy) = (b.faces[x], b.faces[y]);
        if let Some(e) = shared_edge(body, fx, fy) {
            println!(
                "  edge {x}|{y} {e:?}: class {:?}; run({x}) {:?}; run({y}) {:?}",
                facts.class(e),
                facts.edge_run(fx, e).map(|l| l.length()),
                facts.edge_run(fy, e).map(|l| l.length()),
            );
        } else {
            println!("  {x}|{y}: no shared edge");
        }
    }
    for &(x, y) in pairs {
        let faces = if x == y {
            vec![b.faces[x]]
        } else {
            vec![b.faces[x], b.faces[y]]
        };
        let r = crate::clearance::embedded(&geometric, faces, band, budgets());
        println!(
            "  embedded{{{x},{y}}}: {} | receipt {:?}",
            short(format!("{:?}", r.verdict)),
            r.receipt
        );
    }
    let all: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    let r = crate::clearance::embedded(&geometric, all, band, budgets());
    println!(
        "  embedded{{ALL}}: {} | receipt {:?}",
        short(format!("{:?}", r.verdict)),
        r.receipt
    );
    let _ = name_of;
}

// ---------------------------------------------------------------- B1

const SINE: f64 = 5e-5;

/// B1: P z=0 over [0,2]x[0,1]; S through the x axis at sine 5e-5, L
/// shaped (narrow `a` across for x in [0,1], to y=1 for x in [1,2]);
/// T from S's narrow far edge at 45 deg to the back wall y=1.
fn b1(a_width: f64) -> Built {
    let t = SINE / (1.0 - SINE * SINE).sqrt();
    let a = a_width * (1.0 - SINE * SINE).sqrt();
    let h = a * t + (1.0 - a);
    build(
        &[
            ("P0", [0.0, 0.0, 0.0]),
            ("P1", [2.0, 0.0, 0.0]),
            ("P2", [2.0, 1.0, 0.0]),
            ("P3", [0.0, 1.0, 0.0]),
            ("S2", [2.0, 1.0, t]),
            ("S3", [1.0, 1.0, t]),
            ("S4", [1.0, a, a * t]),
            ("S5", [0.0, a, a * t]),
            ("T0", [0.0, 1.0, h]),
            ("T1", [1.0, 1.0, h]),
        ],
        &[
            ("S", &["P0", "P1", "S2", "S3", "S4", "S5"]),
            ("P", &["P0", "P3", "P2", "P1"]),
            ("T", &["S5", "S4", "T1", "T0"]),
            ("step", &["S4", "S3", "T1"]),
            ("end0", &["P0", "S5", "T0", "P3"]),
            ("end2", &["P1", "P2", "S2"]),
            ("back", &["P3", "T0", "T1", "S3", "S2", "P2"]),
        ],
    )
}

/// B1n: the narrow arm in the middle (x in [0.5, 1.5]), wide arms both
/// sides, so no closing vertex sits on an end wall.
fn b1n(a_width: f64) -> Built {
    let t = SINE / (1.0 - SINE * SINE).sqrt();
    let a = a_width * (1.0 - SINE * SINE).sqrt();
    let h = a * t + (1.0 - a);
    build(
        &[
            ("P0", [0.0, 0.0, 0.0]),
            ("P1", [2.0, 0.0, 0.0]),
            ("P2", [2.0, 1.0, 0.0]),
            ("P3", [0.0, 1.0, 0.0]),
            ("A0", [0.0, 1.0, t]),
            ("A1", [0.5, 1.0, t]),
            ("N0", [0.5, a, a * t]),
            ("N1", [1.5, a, a * t]),
            ("B1", [1.5, 1.0, t]),
            ("B2", [2.0, 1.0, t]),
            ("T0", [0.5, 1.0, h]),
            ("T1", [1.5, 1.0, h]),
        ],
        &[
            ("S", &["P0", "P1", "B2", "B1", "N1", "N0", "A1", "A0"]),
            ("P", &["P0", "P3", "P2", "P1"]),
            ("T", &["N0", "N1", "T1", "T0"]),
            ("stepL", &["N0", "T0", "A1"]),
            ("stepR", &["N1", "B1", "T1"]),
            ("end0", &["P0", "A0", "P3"]),
            ("end2", &["P1", "P2", "B2"]),
            ("back", &["P3", "A0", "A1", "T0", "T1", "B1", "B2", "P2"]),
        ],
    )
}

/// B1t: the narrow arm tapers: S's far edge runs from P0 on the shared
/// edge up to (1, a, a t), so no closing vertex hangs over P's interior
/// at x = 0; T holds that tapering edge.
fn b1t(a_width: f64) -> Built {
    let t = SINE / (1.0 - SINE * SINE).sqrt();
    let a = a_width * (1.0 - SINE * SINE).sqrt();
    let h = a * t + (1.0 - a);
    // T's plane through P0, S4, T1; T0 at x=0, y=1 on it.
    let s4 = [1.0, a, a * t];
    let t1 = [1.0, 1.0, h];
    let n = [
        s4[1] * t1[2] - s4[2] * t1[1],
        s4[2] * t1[0] - s4[0] * t1[2],
        s4[0] * t1[1] - s4[1] * t1[0],
    ];
    let zt0 = -n[1] / n[2];
    build(
        &[
            ("P0", [0.0, 0.0, 0.0]),
            ("P1", [2.0, 0.0, 0.0]),
            ("P2", [2.0, 1.0, 0.0]),
            ("P3", [0.0, 1.0, 0.0]),
            ("S2", [2.0, 1.0, t]),
            ("S3", [1.0, 1.0, t]),
            ("S4", s4),
            ("T0", [0.0, 1.0, zt0]),
            ("T1", t1),
        ],
        &[
            ("S", &["P0", "P1", "S2", "S3", "S4"]),
            ("P", &["P0", "P3", "P2", "P1"]),
            ("T", &["P0", "S4", "T1", "T0"]),
            ("step", &["S4", "S3", "T1"]),
            ("end0", &["P0", "T0", "P3"]),
            ("end2", &["P1", "P2", "S2"]),
            ("back", &["P3", "T0", "T1", "S3", "S2", "P2"]),
        ],
    )
}

#[test]
fn run_probe_b1() {
    for (what, a) in [("B1 (a=1e-7)", 1e-7), ("B1c (a=1e-3)", 1e-3)] {
        let b = b1(a);
        measure(
            what,
            &b,
            &[
                ("S", "P"),
                ("T", "P"),
                ("step", "P"),
                ("end0", "P"),
                ("S", "T"),
                ("end0", "end0"),
                ("S", "S"),
            ],
            &[("S", "P"), ("S", "T"), ("end0", "P"), ("end0", "S")],
        );
    }
}

#[test]
fn run_probe_b1n() {
    for (what, a) in [("B1n (a=1e-7)", 1e-7), ("B1nc (a=1e-3)", 1e-3)] {
        let b = b1n(a);
        measure(
            what,
            &b,
            &[
                ("S", "P"),
                ("T", "P"),
                ("stepL", "P"),
                ("stepR", "P"),
                ("S", "T"),
                ("S", "S"),
            ],
            &[("S", "P"), ("S", "T"), ("stepL", "S"), ("stepR", "S")],
        );
    }
}

#[test]
fn run_probe_b1t() {
    for (what, a) in [("B1t (a=1e-7)", 1e-7), ("B1tc (a=1e-3)", 1e-3)] {
        let b = b1t(a);
        measure(
            what,
            &b,
            &[("S", "P"), ("T", "P"), ("step", "P"), ("S", "T"), ("S", "S")],
            &[("S", "P"), ("S", "T"), ("T", "end0")],
        );
    }
}
