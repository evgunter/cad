//! **A face through two vertices on one point tessellates.** A pinch is
//! one vertex per cone, so a boolean that pinches ships a
//! face whose loop passes two vertices at one point. The mesher's CDT
//! dedups the equal positions to one handle; each triangle there takes
//! the id of the vertex whose corner of the face holds it
//! (`mesh::planar::Pinches`). One row per body review r2 of PR 4038
//! found the mesher panicking on (`r2_pinch_probes`, `FACE2V`): a prism
//! corner `v` on, along an edge of, or at a corner of a side-4 cube, or
//! on a cylinder's wall. Each asserts the shape is there, then that
//! `tessellate` passes its chord census and `check_mesh` the result,
//! and on a planar body that the mesh's volume is the body's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::test_support as fixtures;
use topo::{AtRestBody, Body, BooleanDeclarations, BooleanResult, LoopBoundary};

type V3 = [f64; 3];

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

fn axpy(s: f64, a: V3, b: V3) -> V3 {
    [s * a[0] + b[0], s * a[1] + b[1], s * a[2] + b[2]]
}

fn unit(a: V3) -> V3 {
    let k = 1.0 / dot(a, a).sqrt();
    a.map(|c| c * k)
}

fn to_v(a: V3) -> Vec3<f64> {
    Vec3::new(a[0], a[1], a[2])
}

/// A unit `(u, w)` with `(u, w, m)` right-handed, `m` renormalised as
/// review r2 reads it (the rows are its poses to the bit).
fn basis(m: V3) -> (V3, V3) {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u = unit(cross(seed, m));
    (u, cross(m, u))
}

/// `(u, w)` turned by `psi` about their normal.
fn turn((u, w): (V3, V3), psi: f64) -> (V3, V3) {
    let (c, s) = (psi.cos(), psi.sin());
    (axpy(c, u, w.map(|x| x * s)), axpy(-s, u, w.map(|x| x * c)))
}

/// Fibonacci direction `i` of `n`, renormalised as review r2 reads it.
fn fib(i: u32, n: u32) -> V3 {
    let ga = std::f64::consts::PI * (3.0 - 5f64.sqrt());
    let z = 1.0 - 2.0 * (f64::from(i) + 0.5) / f64::from(n);
    let r = (1.0 - z * z).sqrt();
    let t = ga * f64::from(i);
    unit([r * t.cos(), r * t.sin(), z])
}

/// A prism of height 1 over `profile`, and its corner `v`.
struct Prism {
    profile: &'static [(f64, f64)],
    v: V3,
}

const NOTCH: &[(f64, f64)] = &[(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)];
const NOTCH307: Prism = Prism {
    profile: NOTCH,
    v: [2.0, 1.0, 1.0],
};
const VEE300: Prism = Prism {
    profile: &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5), (0.0, 4.0)],
    v: [2.0, 0.5, 1.0],
};
const VEE224BOT: Prism = Prism {
    profile: &[(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.2), (0.0, 1.0)],
    v: [2.0, 0.2, 0.0],
};
const LBOT: Prism = Prism {
    profile: &[
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ],
    v: [1.0, 1.0, 0.0],
};

fn finished(what: &str, body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol()).unwrap_or_else(|e| panic!("{what}: {e:?}"))
}

/// The side-4 cube with `v` inside its near face (`face`), on its edge
/// (`edge`) or at its corner (`corner`), outward normal `−m` there.
fn cube(v: V3, m: V3, place: &str, psi: f64) -> AtRestBody<f64> {
    let (a0, b0) = match place {
        "face" => (-2.0, -2.0),
        "edge" => (0.0, -2.0),
        _ => (0.0, 0.0),
    };
    let (u, w) = turn(basis(m), psi);
    finished(
        "the cube",
        fixtures::mapped_cube::<f64>(
            move |x, y, z| {
                let p = axpy(a0 + 4.0 * x, u, axpy(b0 + 4.0 * y, w, axpy(4.0 * z, m, v)));
                Point3::new(p[0], p[1], p[2])
            },
            tol(),
        ),
    )
}

/// The radius-3, length-8 cylinder whose wall holds `v`, its axis `⊥ m`
/// turned by `psi`, its arc junctions on `v`'s meridian (review r2's
/// `seam`).
fn cylinder(v: V3, m: V3, psi: f64) -> AtRestBody<f64> {
    let (r, half) = (3.0, 4.0);
    let (u0, _) = basis(m);
    let (a, _) = turn((u0, cross(m, u0)), psi);
    let c = axpy(r, m, v);
    let axis = cross([0.0, 0.0, 1.0], a);
    let origin = Point3::new(0.0, 0.0, 0.0);
    let r1 = Affine3::rotation_about_axis(origin, to_v(unit(axis)), a[2].clamp(-1.0, 1.0).acos());
    let back = r1.inverse().transform_vec(to_v(m.map(|x| -x)));
    let rz = Affine3::rotation_about_axis(origin, Vec3::new(0.0, 0.0, 1.0), back.y.atan2(back.x));
    let map = Affine3::translation(to_v(c)) * r1 * rz;
    let raw = sweep::test_support::cylinder_of_arcs_at(
        4,
        r,
        Point2::new(0.0, 0.0),
        -half,
        2.0 * half,
        tol(),
    );
    finished(
        "the cylinder",
        topo::transform_rigid(&raw, &map, tol()).expect("a rigid map"),
    )
}

/// The poses whose boolean escalates at ε 1e-6, each on a margin of
/// its own in band, and the predicate it escalates on
/// (`work/join/two-pinch-poses-escalate-at-eps-1e-6.md`): the join's
/// partner order, read on `main` at `cadf2ed188` as well, and a pierce
/// sector's curvature side, in `sectors`, which no ray walk reaches.
const REFUSES_AT_1E6: &[(&str, &str)] = &[
    ("notch307 fib117 edge psi=1.9 cp S", "bool_join_nearest"),
    (
        "Lbot cyl fib4 psi=0.9 seam cp S",
        "bool_pierce_sector_side_curved",
    ),
];

/// The faces of `body` whose loops pass two distinct vertices at one
/// point.
fn faces_through_two_vertices_on_one_point(body: &Body<f64>) -> usize {
    let at: Vec<_> = body.vertex_points().collect();
    body.faces()
        .filter(|(_, f)| {
            let mut met = Vec::new();
            for &l in std::iter::once(&f.outer).chain(&f.rings) {
                if let LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary {
                    for he in body.loop_cycle(first).unwrap() {
                        let v = body.get_half_edge(he).unwrap().start;
                        met.push(at.iter().find(|x| x.0 == v).unwrap().1);
                    }
                }
            }
            met.iter()
                .enumerate()
                .any(|(i, p)| met[i + 1..].iter().any(|q| (*p - *q).norm() == 0.0))
        })
        .count()
}

#[test]
fn a_face_through_two_vertices_on_one_point_tessellates() {
    // (tag, prism, the other body, prism first?, op, chordal δ)
    type Row = (
        &'static str,
        Prism,
        AtRestBody<f64>,
        bool,
        &'static str,
        f64,
    );
    let on_cube = |p: &Prism, i, place, psi| cube(p.v, fib(i, 120), place, psi);
    let rows: Vec<Row> = vec![
        (
            "notch307 fib117 edge psi=0.3 pc S",
            NOTCH307,
            on_cube(&NOTCH307, 117, "edge", 0.3),
            true,
            "S",
            0.05,
        ),
        (
            "notch307 fib117 edge psi=1.9 cp S",
            NOTCH307,
            on_cube(&NOTCH307, 117, "edge", 1.9),
            false,
            "S",
            0.05,
        ),
        (
            "notch307 fib117 edge psi=4 cp S",
            NOTCH307,
            on_cube(&NOTCH307, 117, "edge", 4.0),
            false,
            "S",
            0.05,
        ),
        (
            "notch307 fib117 corner psi=4 cp S",
            NOTCH307,
            on_cube(&NOTCH307, 117, "corner", 4.0),
            false,
            "S",
            0.05,
        ),
        (
            "notch307 fib113 edge psi=4 cp S",
            NOTCH307,
            on_cube(&NOTCH307, 113, "edge", 4.0),
            false,
            "S",
            0.05,
        ),
        (
            "notch307 fib105 edge psi=0.3 pc U",
            NOTCH307,
            on_cube(&NOTCH307, 105, "edge", 0.3),
            true,
            "U",
            0.05,
        ),
        (
            "notch307 fib105 edge psi=0.3 cp U",
            NOTCH307,
            on_cube(&NOTCH307, 105, "edge", 0.3),
            false,
            "U",
            0.05,
        ),
        (
            "notch307 fib105 edge psi=4 cp S",
            NOTCH307,
            on_cube(&NOTCH307, 105, "edge", 4.0),
            false,
            "S",
            0.05,
        ),
        (
            "notch307 fib105 corner psi=1.9 cp S",
            NOTCH307,
            on_cube(&NOTCH307, 105, "corner", 1.9),
            false,
            "S",
            0.05,
        ),
        (
            "vee300 fib100 edge psi=4 cp S",
            VEE300,
            on_cube(&VEE300, 100, "edge", 4.0),
            false,
            "S",
            0.05,
        ),
        (
            "vee300 fib100 corner psi=0.3 pc U",
            VEE300,
            on_cube(&VEE300, 100, "corner", 0.3),
            true,
            "U",
            0.05,
        ),
        (
            "vee300 fib100 corner psi=0.3 cp U",
            VEE300,
            on_cube(&VEE300, 100, "corner", 0.3),
            false,
            "U",
            0.05,
        ),
        (
            "vee300 fib100 corner psi=1.9 cp S",
            VEE300,
            on_cube(&VEE300, 100, "corner", 1.9),
            false,
            "S",
            0.05,
        ),
        (
            "vee224bot fib1 edge psi=0.3 cp S",
            VEE224BOT,
            on_cube(&VEE224BOT, 1, "edge", 0.3),
            false,
            "S",
            0.05,
        ),
        // On a cylinder's wall: the trimmed lane. It meshes at δ = 1,
        // 0.5 and 0.2, and refuses `CertificateExceeded` at 0.3, 0.1 and
        // 0.05, as the same walls without a pinch do.
        (
            "Lbot cyl fib4 psi=0.9 seam cp S",
            LBOT,
            cylinder(LBOT.v, fib(4, 60), 0.9),
            false,
            "S",
            1.0,
        ),
    ];
    let decls = BooleanDeclarations::default();
    for (tag, prism, other, prism_first, op, delta) in rows {
        let prism = finished(
            "the prism",
            fixtures::prism::<f64>(prism.profile, 1.0, tol()).body,
        );
        let (x, y) = if prism_first {
            (&prism, &other)
        } else {
            (&other, &prism)
        };
        let run = match op {
            "U" => topo::union_with(x, y, &decls, tol()),
            _ => topo::subtract_with(x, y, &decls, tol()),
        };
        // At ε 1e-6 these poses read a margin in band ([`REFUSES_AT_1E6`]).
        if tol().eps() == 1e-6
            && let Some(&(_, predicate)) = REFUSES_AT_1E6.iter().find(|(t, _)| *t == tag)
        {
            let escalated = match &run {
                Err(topo::BooleanError::Escalated { diag, .. }) => diag.predicate,
                _ => None,
            };
            assert_eq!(
                escalated,
                Some(predicate),
                "{tag}: at ε 1e-6, expected an escalation on {predicate}, got {run:?}"
            );
            continue;
        }
        let Ok(BooleanResult::Body(bb)) = run else {
            panic!("{tag}: the op builds no body: {run:?}");
        };
        assert!(
            faces_through_two_vertices_on_one_point(&bb.body) > 0,
            "{tag}: no face passes two vertices at one point, so the row reaches no pinch"
        );
        let mesh = mesh::tessellate(&bb.body, delta, tol())
            .unwrap_or_else(|e| panic!("{tag}: tessellate refuses: {e:?}"));
        mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{tag}: check_mesh: {e:?}"));
        // A planar body's mesh is the body: its volume is exact. A
        // curved wall's falls short by its chords.
        if bb.body.faces().all(|(_, f)| {
            matches!(
                bb.body.get_surface(f.surface),
                Some(geom::Surface::Plane { .. })
            )
        }) {
            let (got, want) = (
                mesh::validate::signed_volume(&mesh),
                topo::mass_properties(&bb.body, tol()).unwrap().volume,
            );
            assert!(
                (got - want).abs() < 1e-9,
                "{tag}: the mesh holds {got}, the body {want}"
            );
        }
    }
}
