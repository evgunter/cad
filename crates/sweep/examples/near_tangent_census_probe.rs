//! **What the near-tangent census cannot decide**: a measurement probe
//! for `near-tangent-boolean-results-ship-with-an-escalated-tier-3-census` (JOIN, closed by PR 4335).
//!
//! The poses are PR 4026 review r1's near-tangent set (`r1_pierce_probes
//! cube` under `R1_NT_D`): a prism's corner `v` on the near face of a
//! cube of side 4, the face's normal at `a` sixteenths of a turn about
//! one of the corner's three edges `e` and tilted `d` along it, so the
//! face's plane lies `d` rad off that edge ([`NtPrism`], shared with
//! the `near_tangent_pierce_poses` suite). Every op in both orders
//! prints one `outcome` line against a kernel-free clipping oracle.
//!
//! For a body that builds and fails tier 3′, the census runs again
//! through its trace door and every pair it decided against is dumped
//! as a `PAIR` JSON line: the two entities, their exact `f64`
//! coordinates, and the census's own reading of each predicate on the
//! pair. A refusal prints its findings, and a sliver shell the gate
//! refuses as in band its certified `V/A`. `scripts/near_tangent_census_classify.py`
//! re-reads every `PAIR` at 60 digits and classifies it.
//!
//! `NT_D` (comma list) sets the tilts, `NT_ONLY` one prism, `NT_POSE`
//! a substring of the pose tag, and `NT_DUMP=1` turns on the dump.
//!
//! `cargo run -p sweep --release --example near_tangent_census_probe`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "../tests/common/differential.rs"]
#[allow(dead_code, unreachable_pub)]
mod differential;

use differential::{NtPrism, nt_basis, nt_cube, nt_prisms, outcome};
use geom_core::{Band, Point3, Tol};
use topo::entity::{EntityId, LoopBoundary};
use topo::{
    AtRestBody, Body, BooleanBody, BooleanDecision, BooleanDeclarations, BooleanError,
    BooleanResult, CensusStrategy, RegionLane, ValidationError, mass_properties,
};

type V3 = [f64; 3];
type Plane = (V3, f64); // n·x ≤ d

fn tol() -> Tol {
    Tol::witness()
}
fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn scale(a: V3, s: f64) -> V3 {
    a.map(|c| c * s)
}
fn unit(a: V3) -> V3 {
    scale(a, 1.0 / dot(a, a).sqrt())
}

/// Each convex piece of the prism as half-spaces.
fn piece_planes(pr: &NtPrism) -> Vec<Vec<Plane>> {
    pr.pieces
        .iter()
        .map(|poly| {
            let n = poly.len();
            let mut out: Vec<Plane> = (0..n)
                .map(|i| {
                    let (p, q) = (poly[i], poly[(i + 1) % n]);
                    let nn = [q.1 - p.1, -(q.0 - p.0), 0.0];
                    (nn, nn[0] * p.0 + nn[1] * p.1)
                })
                .collect();
            out.push(([0.0, 0.0, 1.0], pr.h));
            out.push(([0.0, 0.0, -1.0], 0.0));
            out
        })
        .collect()
}

type Poly = Vec<Vec<V3>>; // faces, each CCW seen from outside

fn clip(poly: &Poly, (n, d): Plane) -> Poly {
    let mut out: Poly = Vec::new();
    let mut cap: Vec<V3> = Vec::new();
    for f in poly {
        let mut g = Vec::new();
        for i in 0..f.len() {
            let (p, q) = (f[i], f[(i + 1) % f.len()]);
            let (sp, sq) = (dot(n, p) - d, dot(n, q) - d);
            if sp <= 0.0 {
                g.push(p);
            }
            if (sp < 0.0 && sq > 0.0) || (sp > 0.0 && sq < 0.0) {
                let t = sp / (sp - sq);
                let x = add(p, scale(sub(q, p), t));
                g.push(x);
                cap.push(x);
            }
            if sp == 0.0 {
                cap.push(p);
            }
        }
        if g.len() >= 3 {
            out.push(g);
        }
    }
    if cap.len() >= 3 {
        let c = scale(
            cap.iter().fold([0.0; 3], |a, &b| add(a, b)),
            1.0 / cap.len() as f64,
        );
        let (e1, e2) = nt_basis(unit(n));
        let mut pts: Vec<(f64, V3)> = cap
            .iter()
            .map(|&p| {
                let r = sub(p, c);
                (dot(r, e2).atan2(dot(r, e1)), p)
            })
            .collect();
        pts.sort_by(|a, b| a.0.total_cmp(&b.0));
        pts.dedup_by(|a, b| dot(sub(a.1, b.1), sub(a.1, b.1)) < 1e-24);
        out.push(pts.into_iter().map(|p| p.1).collect());
    }
    out
}

/// The polytope's volume, its divergence sum taken about one of its own
/// vertices: about the world origin, a sliver's volume drowns in its
/// faces' rounding.
fn volume(poly: &Poly) -> f64 {
    let Some(&o) = poly.first().and_then(|f| f.first()) else {
        return 0.0;
    };
    poly.iter()
        .map(|f| {
            (1..f.len() - 1)
                .map(|i| dot(sub(f[0], o), cross(sub(f[i], o), sub(f[i + 1], o))))
                .sum::<f64>()
        })
        .sum::<f64>()
        / 6.0
}

/// The cube `v + a u + b w + c m`, a, b ∈ [−2, 2], c ∈ [0, 4], as faces.
fn cube_poly(v: V3, u: V3, w: V3, m: V3) -> Poly {
    let p = |a: f64, b: f64, c: f64| add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
    let c = |i: usize| {
        let (a, b, z) = ((i & 1) as f64, ((i >> 1) & 1) as f64, ((i >> 2) & 1) as f64);
        p(-2.0 + 4.0 * a, -2.0 + 4.0 * b, 4.0 * z)
    };
    let quads = [
        [0, 2, 3, 1],
        [4, 5, 7, 6],
        [0, 1, 5, 4],
        [2, 6, 7, 3],
        [0, 4, 6, 2],
        [1, 3, 7, 5],
    ];
    quads
        .iter()
        .map(|q| q.iter().map(|&i| c(i)).collect())
        .collect()
}

fn common_cube(pr: &NtPrism, cube: &Poly) -> f64 {
    piece_planes(pr)
        .iter()
        .map(|pl| volume(&pl.iter().fold(cube.clone(), |acc, &p| clip(&acc, p))))
        .sum()
}

fn area(poly: &Poly) -> f64 {
    poly.iter()
        .map(|f| {
            let n = (1..f.len() - 1).fold([0.0; 3], |acc, i| {
                add(acc, cross(sub(f[i], f[0]), sub(f[i + 1], f[0])))
            });
            dot(n, n).sqrt() / 2.0
        })
        .sum()
}

/// Each convex piece of `pr ∩ cube`: its volume, its area, their ratio
/// (the role read's margin, `V/A`) and its greatest thickness off the
/// cube's near face (`m·x ≥ m·v`), the lump a sliver shell would be.
fn piece_lumps(pr: &NtPrism, cube: &Poly, v: V3, m: V3) -> Vec<String> {
    piece_planes(pr)
        .iter()
        .enumerate()
        .map(|(i, pl)| {
            let p = pl.iter().fold(cube.clone(), |acc, &q| clip(&acc, q));
            let (vol, ar) = (volume(&p), area(&p));
            let thick = p
                .iter()
                .flatten()
                .map(|&x| dot(m, sub(x, v)))
                .fold(0.0, f64::max);
            format!(
                "  LUMP piece{i} volume={vol:.6e} area={ar:.6e} v/a={:.6e} thickness={thick:.6e}",
                if ar > 0.0 { vol / ar } else { 0.0 }
            )
        })
        .collect()
}

fn pt(p: Point3<f64>) -> String {
    format!("[{:?},{:?},{:?}]", p.x, p.y, p.z)
}

/// A line edge's `(v0, v1)` and their points, the census's own reading
/// (`he_plus`'s start to its end).
fn edge_ends(body: &Body<f64>, e: topo::EdgeKey) -> Option<String> {
    let edge = body.get_edge(e)?;
    let plus = body.get_half_edge(edge.he_plus)?;
    let (v0, v1) = (plus.start, body.half_edge_end(edge.he_plus)?);
    let (k0, k1) = (body.get_vertex(v0)?.point, body.get_vertex(v1)?.point);
    let (p0, p1) = (*body.get_point(k0)?, *body.get_point(k1)?);
    let face = |he| -> Option<topo::FaceKey> {
        Some(body.get_loop(body.get_half_edge(he)?.parent_loop)?.face)
    };
    let (f0, f1) = (face(edge.he_plus)?, face(edge.he_minus)?);
    Some(format!(
        "{{\"key\":\"{e:?}\",\"v\":[\"{v0:?}\",\"{v1:?}\"],\"pk\":[\"{k0:?}\",\"{k1:?}\"],\"faces\":[\"{f0:?}\",\"{f1:?}\"],\"p\":[{},{}]}}",
        pt(p0),
        pt(p1)
    ))
}

/// A planar face's plane and loops, every loop's vertex points in walk
/// order.
fn face_geo(body: &Body<f64>, f: topo::FaceKey) -> Option<String> {
    let face = body.get_face(f)?;
    let (origin, normal) = match body.get_surface(face.surface)? {
        geom::Surface::Plane { origin, normal, .. } => (*origin, *normal),
        _ => return None,
    };
    let mut loops = Vec::new();
    for lk in std::iter::once(face.outer).chain(face.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(lk)?.boundary else {
            continue;
        };
        let ring: Vec<String> = body
            .loop_cycle(first)?
            .into_iter()
            .filter_map(|he| {
                let v = body.get_half_edge(he)?.start;
                let k = body.get_vertex(v)?.point;
                Some(format!(
                    "{{\"v\":\"{v:?}\",\"pk\":\"{k:?}\",\"p\":{}}}",
                    pt(*body.get_point(k)?)
                ))
            })
            .collect();
        loops.push(format!("[{}]", ring.join(",")));
    }
    Some(format!(
        "{{\"key\":\"{f:?}\",\"origin\":{},\"normal\":[{:?},{:?},{:?}],\"loops\":[{}]}}",
        pt(origin),
        normal.x,
        normal.y,
        normal.z,
        loops.join(",")
    ))
}

fn vertex_geo(body: &Body<f64>, v: topo::VertexKey) -> Option<String> {
    let k = body.get_vertex(v)?.point;
    Some(format!(
        "{{\"key\":\"{v:?}\",\"pk\":\"{k:?}\",\"p\":{}}}",
        pt(*body.get_point(k)?)
    ))
}

fn entity_geo(body: &Body<f64>, id: EntityId) -> String {
    match id {
        EntityId::Vertex(v) => vertex_geo(body, v),
        EntityId::Edge(e) => edge_ends(body, e),
        EntityId::Face(f) => face_geo(body, f),
        _ => None,
    }
    .unwrap_or_else(|| "null".into())
}

/// One finding as a short tag: an escalation's predicate and margin,
/// anything else its `Debug` head.
fn finding(e: &ValidationError) -> String {
    match e {
        ValidationError::CensusEscalated { cause } => {
            format!("esc {} {:e}", cause.predicate.unwrap_or("?"), cause.margin)
        }
        other => format!("{other:?}").chars().take(160).collect(),
    }
}

/// The census's trace over a built body: every finding, and every
/// pair any sweep decided against, with its geometry.
fn dump(tag: &str, body: &BooleanBody<f64>) -> Vec<String> {
    let mut out = Vec::new();
    let band = Band::linear(tol()).unwrap();
    let (errors, trace) = topo::census_traces(
        &body.body,
        &body.contacts,
        band,
        tol(),
        Some(RegionLane::certified()),
        CensusStrategy::Realized,
    );
    for e in &errors {
        out.push(format!("  FINDING {tag} | {}", finding(e)));
    }
    for (kind, pairs) in [
        ("vv", &trace.vv),
        ("ve", &trace.ve),
        ("vf", &trace.vf),
        ("ef", &trace.ef),
        ("ee", &trace.ee),
    ] {
        for &(a, b) in &pairs.accepted {
            out.push(format!(
                "PAIR {{\"tag\":\"{tag}\",\"kind\":\"{kind}\",\"a\":{},\"b\":{}}}",
                entity_geo(&body.body, a),
                entity_geo(&body.body, b)
            ));
        }
    }
    out
}

type Op = fn(
    &AtRestBody<f64>,
    &AtRestBody<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

fn run_all(
    tag: &str,
    prism: &AtRestBody<f64>,
    cube: &AtRestBody<f64>,
    va: f64,
    vb: f64,
    common: f64,
    lumps: &[String],
) {
    let decls = BooleanDeclarations::default();
    let dumping = std::env::var("NT_DUMP").is_ok();
    for (order, x, y, vx) in [("pc", prism, cube, va), ("cp", cube, prism, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vx - common),
        ];
        for (op, f, want) in ops {
            let r = f(x, y, &decls, tol());
            let line = format!("{tag} {order} {op}");
            let mut after = Vec::new();
            match &r {
                Ok(res) => {
                    if let Some(bb) = res.body()
                        && let Err(errors) =
                            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
                    {
                        for e in &errors {
                            after.push(format!("  T3 {line} | {}", finding(e)));
                        }
                        if dumping {
                            after.extend(dump(&line, bb));
                        }
                    }
                }
                Err(BooleanError::ResultInvalid { errors }) => {
                    for e in errors {
                        after.push(format!("  REFUSAL {line} | {e:?}"));
                    }
                    if op == "I" {
                        after.extend(lumps.iter().cloned());
                    }
                }
                // The finished-body gate's in-band typing: a sliver shell.
                Err(BooleanError::Escalated {
                    decision: BooleanDecision::ShellRole { others, .. },
                    diag,
                }) => {
                    after.push(format!(
                        "  ESCALATED {line} | ShellRole {:e} (+{others} in band)",
                        diag.margin
                    ));
                    if op == "I" {
                        after.extend(lumps.iter().cloned());
                    }
                }
                Err(_) => {}
            }
            println!("{line}: {}", outcome(r, want, tol()));
            for a in after {
                println!("{a}");
            }
        }
    }
}

fn main() {
    let ds: Vec<f64> = std::env::var("NT_D").map_or_else(
        |_| {
            vec![
                1e-5, -1e-5, 1e-6, -1e-6, 1e-7, -1e-7, 1e-8, -1e-8, 1e-9, -1e-9,
            ]
        },
        |s| s.split(',').map(|x| x.parse().unwrap()).collect(),
    );
    let only = std::env::var("NT_ONLY").ok();
    let pose = std::env::var("NT_POSE").ok();
    for pr in nt_prisms() {
        if only.as_deref().is_some_and(|o| o != pr.name) {
            continue;
        }
        let body = pr.body(tol());
        let va = mass_properties(&body, tol()).unwrap().volume;
        for (j, a) in (0..3).flat_map(|j| (0..16).map(move |a| (j, a))) {
            for &d in &ds {
                let tag = format!("{} nt e{j} a{a} d{d:e}", pr.name);
                if pose.as_deref().is_some_and(|p| !tag.contains(p)) {
                    continue;
                }
                let m = pr.near_normal(j, a, d);
                let (u, w) = nt_basis(m);
                let v = pr.v;
                let cube = nt_cube(v, m, tol());
                let poly = cube_poly(v, u, w, m);
                let common = common_cube(&pr, &poly);
                let lumps = piece_lumps(&pr, &poly, v, m);
                run_all(&tag, &body, &cube, va, 64.0, common, &lumps);
            }
        }
    }
}
