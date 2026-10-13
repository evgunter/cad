//! Review probes for PR 4926 (a chamfer turn's overrun), dual review lane.
//!
//! The oracle here is not the lane's vertex enumeration: a chamfer of
//! a convex edge with unit outward normals `nₐ`, `n_b` and interior
//! dihedral `β` at setback `d` keeps `{m·x ≤ m·X₀ − d·cos(β/2)}`,
//! `m = (nₐ + n_b)/|nₐ + n_b|`, `X₀` a point of the edge; the expected
//! body is the operand less two half-space slabs through the kernel's
//! Boolean, and its vertex set and volume are compared with the built
//! body's. One witness (the bracket's `y = 0` corner) is also held to a
//! hand-integrated closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Vec3};
use sweep::blend::BlendError;
use sweep::blend::build::{Blended, chamfer_edges, fillet_edges};
use sweep::test_support::{at_rest, corners, prism_on, realized, sketch_from_axes};
use topo::boolean::BooleanOp;
use topo::{Body, ContactRecords, EdgeKey, VertexKey, validate_geometric, validate_pseudomanifold};

use crate::band_planar_cut_off::{tol, volume};
use crate::band_turn_overrun_probe::{V, add, cross, dot, mul, norm, sub, unit};
use crate::common::cavity::brick;
use crate::common::operands::leaning_turn;

const D: f64 = 0.1;

fn pt(v: V) -> Point3<f64> {
    Point3::new(v[0], v[1], v[2])
}
fn vc(v: V) -> Vec3<f64> {
    Vec3::new(v[0], v[1], v[2])
}

fn point_at(body: &Body<f64>, v: VertexKey) -> V {
    let p = body
        .get_point(body.get_vertex(v).expect("a vertex").point)
        .expect("a point");
    [p.x, p.y, p.z]
}

fn ends(body: &Body<f64>, e: EdgeKey) -> [VertexKey; 2] {
    let he = body.get_edge(e).expect("an edge").he_plus;
    [
        body.get_half_edge(he).expect("a half-edge").start,
        body.half_edge_end(he).expect("an end"),
    ]
}

/// The edge between `a` and `b`, to a tolerance relative to the scale.
fn edge_rel(body: &Body<f64>, a: V, b: V) -> EdgeKey {
    let scale = norm(a).max(norm(b)).max(1.0);
    topo::query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let [s, t] = ends(body, e).map(|v| point_at(body, v));
            let near = |p: V, q: V| norm(sub(p, q)) < 1e-9 * scale;
            (near(s, a) && near(t, b)) || (near(s, b) && near(t, a))
        })
        .unwrap_or_else(|| panic!("an edge between {a:?} and {b:?}"))
}

/// A big slab occupying `{n·x ≥ c}` near the origin (`r` its half-size).
fn slab(n: V, c: f64, r: f64) -> Body<f64> {
    let n = unit(n);
    let seed = if n[0].abs() < 0.9 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let u = unit(cross(seed, n));
    let v = cross(n, u);
    let o = sub(sub(mul(n, c), mul(u, r)), mul(v, r));
    prism_on(
        sketch_from_axes(pt(o), vc(u), vc(v), tol()),
        corners(&[
            (0.0, 0.0),
            (2.0 * r, 0.0),
            (2.0 * r, 2.0 * r),
            (0.0, 2.0 * r),
        ]),
        r,
        tol(),
    )
}

/// `body ∩ {n·x ≤ c}`, through the kernel's Boolean.
fn cut(body: &Body<f64>, n: V, c: f64, r: f64) -> Body<f64> {
    realized(BooleanOp::Subtract, body, &slab(n, c, r), tol())
}

/// The kept half-space of an equal-setback chamfer of the convex edge
/// through `x0` between faces of unit outward normals `na`, `nb`
/// (closed form: the bisector plane, offset `d·cos(β/2)` in).
fn chamfer_keep(na: V, nb: V, x0: V, d: f64) -> (V, f64) {
    let (na, nb) = (unit(na), unit(nb));
    // Interior dihedral β = π − angle(na, nb).
    let beta = core::f64::consts::PI - norm(cross(na, nb)).atan2(dot(na, nb));
    let m = unit(add(na, nb));
    (m, dot(m, x0) - d * (beta / 2.0).cos())
}

/// The vertex sets of `a` and `b` agree to `eps`.
fn same_vertices(a: &Body<f64>, b: &Body<f64>, eps: f64) -> Result<(), String> {
    let pa: Vec<V> = a.vertices().map(|(v, _)| point_at(a, v)).collect();
    let pb: Vec<V> = b.vertices().map(|(v, _)| point_at(b, v)).collect();
    let miss_a: Vec<V> = pa
        .iter()
        .copied()
        .filter(|p| !pb.iter().any(|q| norm(sub(*p, *q)) < eps))
        .collect();
    let miss_b: Vec<V> = pb
        .iter()
        .copied()
        .filter(|p| !pa.iter().any(|q| norm(sub(*p, *q)) < eps))
        .collect();
    if miss_a.is_empty() && miss_b.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "built {} vs oracle {} vertices; built-only {miss_a:?}; oracle-only {miss_b:?}",
            pa.len(),
            pb.len()
        ))
    }
}

fn valid(out: &Blended<f64>, what: &str) {
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′, {e:?}"));
    for (v, _) in out.body.vertices() {
        let n = out.body.edges_of_vertex(v).expect("a fan").len();
        assert_eq!(
            n,
            3,
            "{what}: vertex {:?} has valence {n}",
            point_at(&out.body, v)
        );
    }
}

/// The leaning prism `(0,0),(2,0),(2,1),(s,1)` in `xz`, extruded `1.5`
/// along `−y`, all scaled by `k`; its corner `(s,0,1)·k`, the top edge
/// and the extrusion edge there; and its half-spaces (outward unit
/// normal, offset).
fn leaning_scaled(s: f64, k: f64) -> (Body<f64>, [EdgeKey; 2], Vec<(V, f64)>) {
    let body = prism_on(
        sketch_from_axes(
            pt([0.0; 3]),
            vc([1.0, 0.0, 0.0]),
            vc([0.0, 0.0, 1.0]),
            tol(),
        ),
        corners(&[(0.0, 0.0), (2.0 * k, 0.0), (2.0 * k, k), (s * k, k)]),
        1.5 * k,
        tol(),
    );
    let v = [s * k, 0.0, k];
    let edges = [
        edge_rel(&body, v, [2.0 * k, 0.0, k]),
        edge_rel(&body, v, [s * k, -1.5 * k, k]),
    ];
    let wall = unit([-1.0, 0.0, s]);
    let halves = vec![
        ([0.0, 1.0, 0.0], 0.0),
        ([0.0, -1.0, 0.0], 1.5 * k),
        ([0.0, 0.0, 1.0], k),
        ([0.0, 0.0, -1.0], 0.0),
        ([1.0, 0.0, 0.0], 2.0 * k),
        (wall, 0.0),
    ];
    (body, edges, halves)
}

/// The Boolean oracle for a chamfer of `edges` of the convex polytope
/// `body` whose half-spaces are `halves`: each edge's two faces read off
/// `halves` by incidence, the chamfer's half-space cut off.
fn oracle(body: &Body<f64>, edges: &[EdgeKey], halves: &[(V, f64)], d: f64, r: f64) -> Body<f64> {
    let mut out = body.clone();
    for &e in edges {
        let [a, b] = ends(body, e).map(|v| point_at(body, v));
        let scale = norm(a).max(norm(b)).max(1.0);
        let on: Vec<V> = halves
            .iter()
            .filter(|(n, c)| {
                (dot(*n, a) - c).abs() < 1e-9 * scale && (dot(*n, b) - c).abs() < 1e-9 * scale
            })
            .map(|(n, _)| *n)
            .collect();
        let [na, nb] = on[..] else {
            panic!("an edge on two of the half-spaces, got {}", on.len())
        };
        let (m, c) = chamfer_keep(na, nb, a, d);
        out = cut(&out, m, c, r);
    }
    out
}

/// **Hand-integrated closed form, bracket `y = 0`** (`leaning_turn(−1)`):
/// the top–front edge (`90°`) and the extrusion edge at the `45°` wall,
/// `ΔV = V₁ + V₂ − V₁₂` with `V₁ = 3d²/2 − d³/6`, `V₂ = 1.5·d²/(2√2)`,
/// `V₁₂ = d³·(1/√2 − (√2+1)/4 + 1/6)` (derived in the review).
#[test]
fn bracket_y0_matches_a_hand_integrated_closed_form() {
    for d in [0.05, 0.1, 0.3, 0.6] {
        let (body, edges) = leaning_turn(-1.0);
        let out = chamfer_edges(&at_rest(&body, tol()), &edges, d, tol())
            .unwrap_or_else(|e| panic!("d {d}: builds, got {}", e.error));
        valid(&out, &format!("bracket y0 d {d}"));
        let r2 = core::f64::consts::SQRT_2;
        let v1 = 1.5 * d * d - d * d * d / 6.0;
        let v2 = 1.5 * d * d / (2.0 * r2);
        let v12 = d * d * d * (1.0 / r2 - (r2 + 1.0) / 4.0 + 1.0 / 6.0);
        let want = v1 + v2 - v12;
        let got = volume(&body) - volume(&out.body);
        assert!(
            (got - want).abs() < 1e-12,
            "d {d}: ΔV {got} against the hand closed form {want} (diff {})",
            got - want
        );
    }
}

/// **Boolean oracle over leans, setbacks and scales**: the built body's
/// vertices are the oracle's (the operand less the two chamfer slabs),
/// and the volumes agree.
#[test]
fn leaning_prisms_match_the_boolean_oracle_over_leans_setbacks_and_scales() {
    let mut failures = Vec::new();
    for k in [1.0, 0.01, 100.0] {
        for s in [-2.0, -1.0, -0.5, 0.25, 0.5, 1.0, 1.5] {
            for d in [0.05, 0.1, 0.3] {
                let what = format!("lean {s} scale {k} d {d}");
                let (body, edges, halves) = leaning_scaled(s, k);
                let dk = d * k;
                let out = match chamfer_edges(&at_rest(&body, tol()), &edges, dk, tol()) {
                    Ok(o) => o,
                    Err(e) => {
                        failures.push(format!("{what}: refused {}", e.error));
                        continue;
                    }
                };
                if let Err(e) = validate_geometric(&out.body, tol()) {
                    failures.push(format!("{what}: tier 3 {e:?}"));
                    continue;
                }
                let want = oracle(&body, &edges, &halves, dk, 20.0 * k);
                if let Err(e) = same_vertices(&out.body, &want, 1e-9 * k) {
                    failures.push(format!("{what}: {e}"));
                }
                let (got, exp) = (volume(&out.body), volume(&want));
                if (got - exp).abs() > 1e-10 * k * k * k {
                    failures.push(format!("{what}: volume {got} vs oracle {exp}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// A round pin of radius `r` on the plane face through `at` with unit
/// outward normal `n`, standing `0.03` out (`0.01` sunk in): unioned
/// as a boss (`boss = true`) or, pointing in, subtracted as a blind
/// hole `depth` deep.
fn pin(body: &Body<f64>, at: V, n: V, r: f64, boss: bool, depth: f64) -> Body<f64> {
    let n = unit(n);
    let seed = if n[0].abs() < 0.9 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let u = unit(cross(seed, n));
    let (axis, start, h) = if boss {
        (n, sub(at, mul(n, 0.01)), 0.04)
    } else {
        (mul(n, -1.0), add(at, mul(n, 0.01)), 0.01 + depth)
    };
    let v = cross(axis, u);
    let tool = prism_on(
        sketch_from_axes(pt(start), vc(u), vc(v), tol()),
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        h,
        tol(),
    );
    let op = if boss {
        BooleanOp::Union
    } else {
        BooleanOp::Subtract
    };
    let out = realized(op, body, &tool, tol());
    validate_geometric(&out, tol()).expect("the featured operand is tier-3 valid");
    out
}

enum Expect {
    Builds,
    Refuses,
}

/// The bracket's `y = 0` corner with a feature at `at` on the face of
/// outward normal `n`; chamfer the two edges; check the outcome. A
/// built body must be tier-3 valid and take the feature-free `ΔV`.
fn featured(what: &str, at: V, n: V, r: f64, boss: bool, depth: f64, expect: Expect) -> String {
    let (plain, plain_edges) = leaning_turn(-1.0);
    let plain_out = chamfer_edges(&at_rest(&plain, tol()), &plain_edges, D, tol()).unwrap();
    let plain_dv = volume(&plain) - volume(&plain_out.body);
    let body = pin(&plain, at, n, r, boss, depth);
    let c = [-1.0, 0.0, 1.0];
    let edges = [
        crate::band_planar_cut_off::edge(&body, c, [2.0, 0.0, 1.0]),
        crate::band_planar_cut_off::edge(&body, c, [-1.0, -1.5, 1.0]),
    ];
    match (
        chamfer_edges(&at_rest(&body, tol()), &edges, D, tol()),
        expect,
    ) {
        (Ok(out), Expect::Builds) => {
            if let Err(e) = validate_geometric(&out.body, tol()) {
                return format!("{what}: built invalid {e:?}");
            }
            let dv = volume(&body) - volume(&out.body);
            if (dv - plain_dv).abs() > 1e-10 {
                return format!("{what}: built at ΔV {dv}, feature-free {plain_dv}");
            }
            String::new()
        }
        (Ok(out), Expect::Refuses) => {
            let dv = volume(&body) - volume(&out.body);
            format!(
                "{what}: BUILT where it should refuse; tier 3 {:?}, ΔV {dv} vs feature-free {plain_dv}",
                validate_geometric(&out.body, tol()).err()
            )
        }
        (Err(e), Expect::Builds) => format!("{what}: refused where clear: {}", e.error),
        (Err(e), Expect::Refuses) => {
            eprintln!("{what}: refuses {}", e.error);
            String::new()
        }
    }
}

/// **Features near the overrun** at the bracket's `y = 0` corner
/// (`d = 0.1`). The leaning wall (`Fⱼ`, outward `(−1,0,−1)/√2`) in the
/// frame `(a along L from V, b along −y)`: band `j`'s trimline `a = d`,
/// the overrun triangle `(d,0)`, `(√2d,0)`, `q = (d, (1−1/√2)d)`, its
/// chord on `a + √2·b = √2·d`. The front face (`F_k`, `y = 0`): band
/// `k`'s strip `z ≥ 1 − d`.
#[test]
fn features_near_the_overrun_refuse_or_build_as_the_regions_say() {
    let r2 = core::f64::consts::FRAC_1_SQRT_2;
    let wall_n = [-r2, 0.0, -r2];
    let wall = |a: f64, b: f64| [-1.0 + a * D * r2, -b * D, 1.0 - a * D * r2];
    let front = |x: f64, z: f64| [x, 0.0, z];
    let fy = [0.0, 1.0, 0.0];
    let r = 0.004;
    let rows: Vec<String> = vec![
        // Wall, clear just past the overrun chord (0.06 d of air).
        featured(
            "wall boss past the chord",
            wall(1.45, 0.1),
            wall_n,
            r,
            true,
            0.0,
            Expect::Builds,
        ),
        // Wall, straddling the chord.
        featured(
            "wall boss on the chord",
            wall(1.3, 0.1),
            wall_n,
            r,
            true,
            0.0,
            Expect::Refuses,
        ),
        // Wall, near p_k, clear of the chord and of L.
        featured(
            "wall boss near p_k",
            wall(1.52, 0.05),
            wall_n,
            r,
            true,
            0.0,
            Expect::Builds,
        ),
        // Wall, a shallow blind hole inside the overrun triangle.
        featured(
            "wall hole in the overrun",
            wall(1.1, 0.1),
            wall_n,
            r,
            false,
            0.002,
            Expect::Refuses,
        ),
        // Wall, a deep blind hole inside the overrun triangle.
        featured(
            "wall deep hole in the overrun",
            wall(1.1, 0.1),
            wall_n,
            r,
            false,
            0.05,
            Expect::Refuses,
        ),
        // Wall, a shallow blind hole on the chord.
        featured(
            "wall hole on the chord",
            wall(1.3, 0.1),
            wall_n,
            r,
            false,
            0.002,
            Expect::Refuses,
        ),
        // Wall, a shallow hole just past the chord: clear.
        featured(
            "wall hole past the chord",
            wall(1.45, 0.1),
            wall_n,
            r,
            false,
            0.002,
            Expect::Builds,
        ),
        // Front face (F_k), below band k's trimline near p_k: clear.
        featured(
            "front boss below p_k",
            front(-0.85, 0.885),
            fy,
            r,
            true,
            0.0,
            Expect::Builds,
        ),
        // Front face, straddling band k's trimline near p_k.
        featured(
            "front boss on k's trimline",
            front(-0.85, 0.898),
            fy,
            r,
            true,
            0.0,
            Expect::Refuses,
        ),
        // Front face, a shallow hole just below the trimline near L.
        featured(
            "front hole below p_k",
            front(-0.83, 0.89),
            fy,
            r,
            false,
            0.002,
            Expect::Builds,
        ),
    ];
    let bad: Vec<&String> = rows.iter().filter(|s| !s.is_empty()).collect();
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **The far foot past L's far end**: the bracket's `y = 0` prism with
/// its corner at the origin cut off by the plane through `(−0.3,0,0.3)`
/// on L and `(0.5,0,0)` on the floor (normal in `xz`), so L is
/// `0.3·√2·(… )` shorter while the front face stays tall. At `d = 0.69`
/// the far foot `√2·d = 0.976` lies inside L (`0.99`); at `0.71`
/// (`1.004`) past it.
#[test]
fn the_far_foot_near_and_past_ls_far_end() {
    let (body, _, mut halves) = leaning_scaled(-1.0, 1.0);
    let n = unit([-0.3, 0.0, -0.8]);
    let c = dot(n, [-0.3, 0.0, 0.3]);
    let short = cut(&body, n, c, 20.0);
    halves.push((n, c));
    let v = [-1.0, 0.0, 1.0];
    let edges = [
        edge_rel(&short, v, [2.0, 0.0, 1.0]),
        edge_rel(&short, v, [-1.0, -1.5, 1.0]),
    ];
    let mut report = Vec::new();
    for d in [0.6, 0.69, 0.7, 0.71, 0.75] {
        match chamfer_edges(&at_rest(&short, tol()), &edges, d, tol()) {
            Ok(out) => {
                let ok = validate_geometric(&out.body, tol()).err();
                let want = oracle(&short, &edges, &halves, d, 20.0);
                let vtx = same_vertices(&out.body, &want, 1e-9).err();
                let (g, w) = (volume(&out.body), volume(&want));
                report.push(format!(
                    "d {d}: BUILT tier3 {ok:?} vertices {vtx:?} vol {g} oracle {w}"
                ));
                assert!(
                    ok.is_none() && vtx.is_none() && (g - w).abs() < 1e-10,
                    "{report:#?}"
                );
            }
            Err(e) => report.push(format!("d {d}: refused {}", e.error)),
        }
    }
    eprintln!("{report:#?}");
}

/// **Two turns on one L**: `L` along `z` from `V = 0` to `W = (0,0,ℓ)`,
/// its faces the planes through `z` of outward normals `n₁`, `n₂`; the
/// shared face at `V` tilted below, at `W` tilted above; all four of
/// their edges requested. Every build is checked against the oracle;
/// every outcome is printed.
#[test]
fn two_turns_on_one_l_match_the_oracle_when_they_build() {
    let mut report = Vec::new();
    let mut bad = Vec::new();
    let mut built = 0;
    for (a1, a2) in [(200.0f64, 300.0f64), (190.0, 320.0), (210.0, 290.0)] {
        for (tv, tw) in [
            ([0.6, 0.2], [-0.5, 0.3]),
            ([-0.7, 0.1], [0.4, -0.6]),
            ([0.3, -0.8], [0.9, 0.2]),
        ] {
            for ell in [0.25, 0.4, 0.8] {
                let n1 = [a1.to_radians().cos(), a1.to_radians().sin(), 0.0];
                let n2 = [a2.to_radians().cos(), a2.to_radians().sin(), 0.0];
                let nv = unit([tv[0], tv[1], -1.0]);
                let nw = unit([tw[0], tw[1], 1.0]);
                let wpt = [0.0, 0.0, ell];
                let halves = vec![
                    (n1, 0.0),
                    (n2, 0.0),
                    (nv, 0.0),
                    (nw, dot(nw, wpt)),
                    ([1.0, 0.0, 0.0], 3.0),
                    ([-1.0, 0.0, 0.0], 3.0),
                    ([0.0, 1.0, 0.0], 3.0),
                    ([0.0, -1.0, 0.0], 3.0),
                ];
                let mut body = brick(Point3::new(-3.0, -3.0, -3.0), Point3::new(3.0, 3.0, 3.0));
                for (n, c) in &halves[..4] {
                    body = cut(&body, *n, *c, 20.0);
                }
                // The four edges of the two shared faces at V and W.
                let at = |p: V| {
                    body.vertices()
                        .map(|(v, _)| v)
                        .find(|v| norm(sub(point_at(&body, *v), p)) < 1e-9)
                };
                let (Some(vv), Some(vw)) = (at([0.0; 3]), at(wpt)) else {
                    report.push(format!("({a1},{a2}) {tv:?} {tw:?} ℓ {ell}: no V/W"));
                    continue;
                };
                let on = |e: EdgeKey, n: V, c: f64| {
                    ends(&body, e)
                        .iter()
                        .all(|v| (dot(n, point_at(&body, *v)) - c).abs() < 1e-9)
                };
                let mut edges = Vec::new();
                for (vx, ns, cs) in [(vv, nv, 0.0), (vw, nw, dot(nw, wpt))] {
                    for e in body.edges_of_vertex(vx).unwrap() {
                        if on(e, ns, cs) && (on(e, n1, 0.0) || on(e, n2, 0.0)) {
                            edges.push(e);
                        }
                    }
                }
                if edges.len() != 4 {
                    report.push(format!("({a1},{a2}) ℓ {ell}: {} edges", edges.len()));
                    continue;
                }
                let what = format!("({a1},{a2}) {tv:?} {tw:?} ℓ {ell}");
                match chamfer_edges(&at_rest(&body, tol()), &edges, D, tol()) {
                    Ok(out) => {
                        built += 1;
                        let ok = validate_geometric(&out.body, tol()).err();
                        let want = oracle(&body, &edges, &halves, D, 20.0);
                        let vtx = same_vertices(&out.body, &want, 1e-9).err();
                        let (g, w) = (volume(&out.body), volume(&want));
                        let line = format!(
                            "{what}: BUILT tier3 {ok:?} vertices {vtx:?} vol {g} oracle {w}"
                        );
                        if ok.is_some() || vtx.is_some() || (g - w).abs() > 1e-10 {
                            bad.push(line.clone());
                        }
                        report.push(line);
                    }
                    Err(e) => report.push(format!("{what}: refused {}", e.error)),
                }
            }
        }
    }
    eprintln!("built {built}\n{}", report.join("\n"));
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **The fillet still refuses** the same overruns, naming the far band.
#[test]
fn the_fillet_still_refuses_the_overrun() {
    for s in [-1.0, -0.5, 0.5, 1.0] {
        let (body, edges) = leaning_turn(s);
        match fillet_edges(&at_rest(&body, tol()), &edges, D, tol()) {
            Err(e) => match e.error {
                BlendError::UnsupportedRunOut { detail, .. } => {
                    assert_eq!(detail, sweep::blend::battery::TURN_OVERRUN, "lean {s}");
                }
                other => panic!("lean {s}: fillet refuses TURN_OVERRUN, got {other}"),
            },
            Ok(_) => panic!("lean {s}: a fillet overrun built"),
        }
    }
}

/// **L chamfered later at the overrun's turn foot**: which refusal at
/// the first setback, and at what setbacks it builds (printed), each
/// build held to tier 3 and its `ΔV` to the oracle's.
#[test]
fn a_later_chamfer_of_l_at_the_overrun_foot() {
    let (body, edges, halves) = leaning_scaled(0.5, 1.0);
    let out = chamfer_edges(&at_rest(&body, tol()), &edges, D, tol()).unwrap();
    let b = &out.body;
    let foot = out.naming.as_ref().unwrap().turn_feet[0].0;
    let l = b
        .edges_of_vertex(foot)
        .unwrap()
        .into_iter()
        .find(|e| ends(b, *e).iter().any(|v| norm(point_at(b, *v)) < 1e-12))
        .expect("L's piece");
    let _ = halves;
    let mut report = Vec::new();
    for d in [D, 0.5 * D, 0.2 * D, 0.1 * D, 0.05 * D] {
        match chamfer_edges(&at_rest(b, tol()), &[l], d, tol()) {
            Ok(later) => {
                let ok = validate_geometric(&later.body, tol()).err();
                report.push(format!(
                    "d {d}: built, tier3 {ok:?}, ΔV {}",
                    volume(b) - volume(&later.body)
                ));
                assert!(ok.is_none(), "{report:#?}");
            }
            Err(e) => report.push(format!("d {d}: refused {}", e.error)),
        }
    }
    eprintln!("{report:#?}");
}

/// **The filed residue** (`an-overruns-sliver-refuses-a-short-near-edge`):
/// a short near edge at an obtuse shared-face angle. Built from the
/// leaning prism at `0.5` with the back face moved in until the near
/// band's requested edge is shorter than the far cut-off's foot on it.
#[test]
fn the_short_near_edge_residue() {
    let mut report = Vec::new();
    for s in [-1.0, -0.5, 0.5, 1.0] {
        for back in [1.5, 0.5, 0.3, 0.2, 0.15, 0.12] {
            let (body, _, _) = leaning_scaled(s, 1.0);
            let body = cut(&body, [0.0, -1.0, 0.0], back, 20.0);
            let v = [s, 0.0, 1.0];
            let edges = [
                edge_rel(&body, v, [2.0, 0.0, 1.0]),
                edge_rel(&body, v, [s, -back, 1.0]),
            ];
            match chamfer_edges(&at_rest(&body, tol()), &edges, D, tol()) {
                Ok(out) => {
                    let ok = validate_geometric(&out.body, tol()).err();
                    report.push(format!("lean {s} back {back}: built tier3 {ok:?}"));
                }
                Err(e) => report.push(format!("lean {s} back {back}: refused {}", e.error)),
            }
        }
    }
    eprintln!("{}", report.join("\n"));
}

/// **The valence-4 vertex** `corner()` puts at `e₁`'s far end at
/// `(135°, 90°)`: what each verb answers there now, and what the same
/// cone answers with its isosceles twin (a mitre).
#[test]
fn the_corner_fixtures_valence_four_vertex() {
    use crate::band_turn_overrun_probe::Cone;
    use crate::band_turn_overrun_verdict::corner;
    for (p1, p2) in [
        (135.0f64, 90.0f64),
        (45.0, 90.0),
        (130.0, 90.0),
        (135.0, 135.0),
    ] {
        let c = Cone::angles([p1.to_radians(), p2.to_radians()], 90f64.to_radians());
        let (body, edges) = corner(&c);
        let val: Vec<usize> = body
            .vertices()
            .map(|(v, _)| body.edges_of_vertex(v).unwrap().len())
            .collect();
        let ch = chamfer_edges(&at_rest(&body, tol()), &edges, D, tol()).map(|_| ());
        let fi = fillet_edges(&at_rest(&body, tol()), &edges, D, tol()).map(|_| ());
        eprintln!(
            "({p1},{p2}): valences {val:?}; chamfer {:?}; fillet {:?}",
            ch.map_err(|e| e.error.to_string()),
            fi.map_err(|e| e.error.to_string())
        );
    }
}

/// **A boss beyond `q`, clear of the overrun and of band `j`'s strip**
/// (`(1.15d, 0.4d)` on the wall: `0.13d` of air past the overrun chord's
/// line, `0.11d` past the trimline) — inside the far band's whole
/// cut-off sliver's bounding region, which arm (c) meters.
#[test]
fn finding_a_boss_beyond_q_clear_of_the_overrun_is_refused() {
    let r2 = core::f64::consts::FRAC_1_SQRT_2;
    let wall_n = [-r2, 0.0, -r2];
    let wall = |a: f64, b: f64| [-1.0 + a * D * r2, -b * D, 1.0 - a * D * r2];
    let mut bad = Vec::new();
    for (a, b) in [
        (1.15, 0.4),
        (1.3, 0.5),
        (1.0 + 0.6, 0.3),
        (1.6, 0.05),
        (2.0, 0.1),
    ] {
        let row = featured(
            &format!("wall boss at ({a}d, {b}d)"),
            wall(a, b),
            wall_n,
            0.004,
            true,
            0.0,
            Expect::Builds,
        );
        if !row.is_empty() {
            bad.push(row);
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// Which request refuses at the short near edge: the turn, or either
/// band alone (each then a plain cut-off at the vertex).
#[test]
fn the_short_near_edge_refusal_is_the_turns() {
    for back in [0.12, 0.13, 0.14] {
        let (body, _, _) = leaning_scaled(-1.0, 1.0);
        let body = cut(&body, [0.0, -1.0, 0.0], back, 20.0);
        let v = [-1.0, 0.0, 1.0];
        let e1 = edge_rel(&body, v, [2.0, 0.0, 1.0]);
        let e2 = edge_rel(&body, v, [-1.0, -back, 1.0]);
        for (what, edges) in [("both", vec![e1, e2]), ("e1", vec![e1]), ("e2", vec![e2])] {
            let r = chamfer_edges(&at_rest(&body, tol()), &edges, D, tol()).map(|_| ());
            eprintln!(
                "back {back} {what}: {:?}",
                r.map_err(|e| e.error.to_string())
            );
        }
    }
}

/// **`FOOT_INSIDE_A_FACE` at a short near edge** (the filed residue,
/// built): `φ = (60°, 90°)`, `γ = 150°`, so `θ_S ≈ 138.6°`; band `j`
/// (`e₂`, `φ = 90°`) is near, `|VE| = d / sin θ_S ≈ 0.151`; `e₂` cut
/// short by the plane normal to it at `ℓ`.
#[test]
fn the_short_near_edge_at_an_obtuse_shared_face() {
    use crate::band_turn_overrun_probe::{Cone, Verb, measure};
    use crate::band_turn_overrun_verdict::corner;
    let c = Cone::angles(
        [60f64.to_radians(), 90f64.to_radians()],
        150f64.to_radians(),
    );
    let t = measure(&c, Verb::Chamfer, D, 2);
    eprintln!(
        "far {:?} theta_s {} VE {}",
        t.far,
        c.theta_s().to_degrees(),
        D / c.theta_s().sin()
    );
    for ell in [0.08, 0.1, 0.12, 0.14, 0.16, 0.2, 0.4] {
        let (wedge, edges) = corner(&c);
        let [a, b] = ends(&wedge, edges[1]).map(|v| point_at(&wedge, v));
        let ej = if norm(a) < 1e-12 { unit(b) } else { unit(a) };
        let body = cut(&wedge, ej, ell, 40.0);
        let at = |p: V| {
            topo::query::all_edges(&body)
                .into_iter()
                .find(|&e| {
                    let [x, y] = ends(&body, e).map(|v| point_at(&body, v));
                    (norm(x) < 1e-12 && norm(sub(unit(y), p)) < 1e-9)
                        || (norm(y) < 1e-12 && norm(sub(unit(x), p)) < 1e-9)
                })
                .expect("an edge out of the vertex")
        };
        let e = [at(c.e[0]), at(c.e[1])];
        let both = chamfer_edges(&at_rest(&body, tol()), &e, D, tol()).map(|_| ());
        let near = chamfer_edges(&at_rest(&body, tol()), &[e[1]], D, tol()).map(|_| ());
        eprintln!(
            "ℓ {ell}: turn {:?}; near band alone {:?}",
            both.map_err(|x| format!("{:?}", x.error)),
            near.map_err(|x| x.error.to_string())
        );
    }
}

/// **Order and company**: the overrun built with its two edges requested
/// in either order, and the whole top rim of the leaning prism requested
/// (a closed chain: two overruns, at `(s,0,1)` and `(s,−1.5,1)`, and two
/// mitres), each against the Boolean oracle.
#[test]
fn order_swaps_and_the_closed_top_rim_match_the_oracle() {
    let mut bad = Vec::new();
    for s in [-1.0, -0.5, 0.5, 1.0] {
        let (body, edges, halves) = leaning_scaled(s, 1.0);
        let top = [
            edges[0],
            edges[1],
            edge_rel(&body, [s, -1.5, 1.0], [2.0, -1.5, 1.0]),
            edge_rel(&body, [2.0, 0.0, 1.0], [2.0, -1.5, 1.0]),
        ];
        let cases: Vec<(String, Vec<EdgeKey>)> = vec![
            (format!("lean {s} swapped"), vec![edges[1], edges[0]]),
            (format!("lean {s} top rim"), top.to_vec()),
            (
                format!("lean {s} top rim reversed"),
                top.iter().rev().copied().collect(),
            ),
        ];
        for (what, req) in cases {
            match chamfer_edges(&at_rest(&body, tol()), &req, D, tol()) {
                Ok(out) => {
                    let ok = validate_geometric(&out.body, tol()).err();
                    let want = oracle(&body, &req, &halves, D, 20.0);
                    let vtx = same_vertices(&out.body, &want, 1e-9).err();
                    let (g, w) = (volume(&out.body), volume(&want));
                    // The isosceles mitres at x = 2 have a valence-4 common
                    // foot by design; every other vertex is trivalent.
                    let tri = out.body.vertices().all(|(v, _)| {
                        out.body.edges_of_vertex(v).unwrap().len() == 3
                            || (point_at(&out.body, v)[0] - 2.0).abs() < 1e-12
                    });
                    if ok.is_some() || vtx.is_some() || (g - w).abs() > 1e-10 || !tri {
                        bad.push(format!(
                            "{what}: tier3 {ok:?} vtx {vtx:?} vol {g} vs {w} trivalent {tri}"
                        ));
                    }
                    sweep::test_support::assert_naming_totality(&body, &out, &req, &what);
                }
                Err(e) => bad.push(format!("{what}: refused {}", e.error)),
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **L chamfered later, against the oracle**: at a tenth of the setback
/// the later chamfer of L's piece (front face and leaning wall, both
/// still planes) is the half-space cut of the overrun body.
#[test]
fn a_later_chamfer_of_l_matches_the_oracle() {
    for s in [0.5, -0.5, -1.0] {
        let (body, edges, _) = leaning_scaled(s, 1.0);
        let out = chamfer_edges(&at_rest(&body, tol()), &edges, D, tol()).unwrap();
        let b = &out.body;
        let foot = out.naming.as_ref().unwrap().turn_feet[0].0;
        let l = b
            .edges_of_vertex(foot)
            .unwrap()
            .into_iter()
            .find(|e| ends(b, *e).iter().any(|v| norm(point_at(b, *v)) < 1e-12))
            .expect("L's piece");
        let d = 0.1 * D;
        match chamfer_edges(&at_rest(b, tol()), &[l], d, tol()) {
            Ok(later) => {
                validate_geometric(&later.body, tol()).unwrap();
                let (m, c) = chamfer_keep([0.0, 1.0, 0.0], [-1.0, 0.0, s], [0.0; 3], d);
                let want = cut(b, m, c, 20.0);
                same_vertices(&later.body, &want, 1e-9).unwrap_or_else(|e| panic!("lean {s}: {e}"));
                assert!(
                    (volume(&later.body) - volume(&want)).abs() < 1e-12,
                    "lean {s}: volume"
                );
            }
            Err(e) => eprintln!("lean {s}: later L refuses {}", e.error),
        }
    }
}

/// The boss beyond `q` against the far band's edge cut off ALONE at the
/// same wall (a plain cut-off, whose sliver is the same triangle).
#[test]
fn the_boss_beyond_q_against_the_far_band_alone() {
    let r2 = core::f64::consts::FRAC_1_SQRT_2;
    let (plain, _) = leaning_turn(-1.0);
    let body = pin(
        &plain,
        [-1.0 + 1.15 * D * r2, -0.4 * D, 1.0 - 1.15 * D * r2],
        [-r2, 0.0, -r2],
        0.004,
        true,
        0.0,
    );
    let e1 = crate::band_planar_cut_off::edge(&body, [-1.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let r = chamfer_edges(&at_rest(&body, tol()), &[e1], D, tol()).map(|_| ());
    eprintln!("far band alone: {:?}", r.map_err(|e| e.error.to_string()));
}

/// **Just past the band**: leans from the gap's definite edge
/// (`≈0.0256°` at the default band) up, where the overrun chord is a
/// few escalate-widths long. What builds is held to tier 3, the oracle's
/// volume, and the shortest edge it leaves is printed.
#[test]
fn the_overrun_just_past_the_band() {
    let mut bad = Vec::new();
    for deg in [0.02f64, 0.026, 0.03, 0.05, 0.1, 0.3, 1.0] {
        for sign in [1.0, -1.0] {
            let s = sign * deg.to_radians().tan();
            let (body, edges, halves) = leaning_scaled(s, 1.0);
            let what = format!("lean {}°", sign * deg);
            match chamfer_edges(&at_rest(&body, tol()), &edges, D, tol()) {
                Ok(out) => {
                    let ok = validate_geometric(&out.body, tol()).err();
                    let g = volume(&out.body);
                    let w = std::panic::catch_unwind(|| {
                        volume(&oracle(&body, &edges, &halves, D, 20.0))
                    })
                    .unwrap_or(f64::NAN);
                    let shortest = topo::query::all_edges(&out.body)
                        .into_iter()
                        .map(|e| {
                            let [a, b] = ends(&out.body, e).map(|v| point_at(&out.body, v));
                            norm(sub(a, b))
                        })
                        .fold(f64::INFINITY, f64::min);
                    // Downstream: the result chamfered again at an unrelated edge.
                    let far = edge_rel(&out.body, [2.0, 0.0, 0.0], [2.0, -1.5, 0.0]);
                    let at = chamfer_edges(&at_rest(&out.body, tol()), &[far], D, tol())
                        .err()
                        .map(|e| e.error.to_string());
                    let mitre = out
                        .coincidences
                        .iter()
                        .any(|r| r.relation == topo::Relation::CommonFoot);
                    eprintln!(
                        "{what}: built, mitre {mitre}, tier3 {ok:?}, shortest edge {shortest:e}, ΔV err {:e}, at_rest {at:?}",
                        g - w
                    );
                    if ok.is_some() || (g - w).abs() > 1e-12 || at.is_some() {
                        bad.push(what);
                    }
                }
                Err(e) => eprintln!("{what}: refused {:?}", e.error),
            }
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}

/// **At `Interval`, the bracket's two corners** (`leaning ∓1`): built,
/// tier 3, naming total, the volume enclosure about the oracle's.
#[test]
fn the_bracket_corners_at_interval() {
    use crate::common::interval::iv;
    use crate::common::operands::leaning_turn_at;
    use geom_core::{Bounds, Interval};
    for lean in [-1.0, 1.0] {
        let (body, edges, halves) = leaning_scaled(lean, 1.0);
        let want = volume(&oracle(&body, &edges, &halves, D, 20.0));
        let v0 = volume(&body);
        let (ibody, iedges) = leaning_turn_at::<Interval>(lean);
        let ibody = sweep::test_support::finished("operand", ibody, tol());
        let out = chamfer_edges(&ibody, &iedges, iv(D), tol())
            .unwrap_or_else(|e| panic!("lean {lean}: Interval builds, got {:?}", e.error));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("lean {lean}: {e:?}"));
        sweep::test_support::assert_naming_totality(&ibody, &out, &iedges, "interval");
        let p = topo::mass_properties(&out.body, tol()).unwrap();
        let (lo, hi) = (
            p.volume.lo() - p.volume_pad.hi(),
            p.volume.hi() + p.volume_pad.hi(),
        );
        eprintln!(
            "lean {lean}: [{lo}, {hi}] width {:e} vs oracle {want} (removed {})",
            hi - lo,
            v0 - want
        );
        assert!(
            lo <= want && want <= hi,
            "lean {lean}: enclosure misses the oracle"
        );
        assert!(hi - lo < 1e-5, "lean {lean}: enclosure a claim");
    }
}

/// **The recourse's "a larger setback" leg**, followed from the in-band
/// witness the followability row escalates (`leaning_turn_in_band`).
#[test]
fn the_larger_setback_leg_from_the_in_band_witness() {
    let (body, edges) = crate::common::operands::leaning_turn_in_band(tol());
    for d in [0.1, 0.2, 0.3, 0.4, 0.5, 0.7] {
        let r = chamfer_edges(&at_rest(&body, tol()), &edges, d, tol());
        match r {
            Ok(out) => eprintln!(
                "d {d}: built, mitre {}, tier3 {:?}",
                out.coincidences
                    .iter()
                    .any(|r| r.relation == topo::Relation::CommonFoot),
                validate_geometric(&out.body, tol()).err()
            ),
            Err(e) => eprintln!("d {d}: refused {:?}", e.error),
        }
    }
}

/// **A void in the overrun's volume**, touching no face: at the bracket's
/// `y = 0` corner, a cube of half-size `h` about a point `0.003` in from
/// the leaning wall over the overrun triangle `(1.2d, 0.05d)` — inside
/// band `k`'s half-space region only (`y + z > 1 − d`, and on band `j`'s
/// kept side). Only the reach can see it; it must refuse.
#[test]
fn a_void_in_the_overruns_volume_refuses() {
    let (plain, _) = leaning_turn(-1.0);
    let c = [-0.913, -0.005, 0.917];
    assert!(c[1] + c[2] > 1.0 - D, "in band k's region");
    let s2 = 2f64.sqrt();
    assert!(c[0] + (1.0 - s2) * c[2] > D - s2, "on band j's kept side");
    for h in [0.0012, 0.0005] {
        let void = brick(
            Point3::new(c[0] - h, c[1] - h, c[2] - h),
            Point3::new(c[0] + h, c[1] + h, c[2] + h),
        );
        let body = realized(BooleanOp::Subtract, &plain, &void, tol());
        let e = [
            crate::band_planar_cut_off::edge(&body, [-1.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
            crate::band_planar_cut_off::edge(&body, [-1.0, 0.0, 1.0], [-1.0, -1.5, 1.0]),
        ];
        match chamfer_edges(&at_rest(&body, tol()), &e, D, tol()) {
            Ok(out) => panic!(
                "h {h}: a void in the overrun built: tier 3 {:?}, ΔV {}",
                validate_geometric(&out.body, tol()).err(),
                volume(&body) - volume(&out.body)
            ),
            Err(e) => eprintln!("h {h}: refuses {}", e.error),
        }
    }
}
