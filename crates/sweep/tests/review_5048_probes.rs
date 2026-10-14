//! Review probes for PR 5048 (fillet corner patches pole on a foot).
//!
//! - `p1`: `corner_chart` over random, near-degenerate and extreme
//!   trihedra, every foot order, both sides: the pole is `±f_c`, the
//!   antipode is outside the patch, `f_x` is at `u = 0`, `f_y` at
//!   `u ∈ (0, π)`, and interior samples stay in `[0, θ]` (no wrap).
//! - `p2`: built fillets over oblique clips of a cube and of an acute
//!   and an obtuse prism by cubes turned about several axes: every
//!   corner face's pole is ON one of its vertices, its sampled boundary
//!   sits in one `u` window narrower than `π`, it reads `Encoded`
//!   agreeing with its bit, and the body passes tiers 1–3 and keeps its
//!   volume inside the `r²·Σℓ` bracket. Meshing is checked watertight
//!   where it succeeds; every oblique body refuses at a corner patch,
//!   identically on main (the filed tilted-circle tessellation issue).
//! - `p3` (diagnostic): which face the mesher refuses on an oblique clip.
//! - `p4` (diagnostic, gated on `R5048_BITS`): dumps box-corner
//!   pcurves, mesh bits and mass properties at every pose for a
//!   head-vs-main diff of the signed-zero axis flip.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::PI;

use geom::Surface;
use geom_core::{Affine3, Band, Point3, Sign, Tol, Vec3};
use sweep::blend::arms::{corner_ball, corner_chart};
use sweep::blend::build::fillet_edges;
use sweep::blend::{Convexity, Filleted};
use sweep::test_support::{at_rest, corners, cube, finished, prism};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, LoopBoundary, query};

/// Chart longitude of direction `d`.
fn lon(d: Vec3<f64>, u_ref: Vec3<f64>, axis: Vec3<f64>) -> f64 {
    d.dot(axis.cross(u_ref)).atan2(d.dot(u_ref))
}

/// Cramer weights of `p` over `[a, b, c]`.
fn weights(p: Vec3<f64>, [a, b, c]: [Vec3<f64>; 3]) -> [f64; 3] {
    let det = a.dot(b.cross(c));
    [
        p.dot(b.cross(c)) / det,
        a.dot(p.cross(c)) / det,
        a.dot(b.cross(p)) / det,
    ]
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                2.0 * self.next() - 1.0,
                2.0 * self.next() - 1.0,
                2.0 * self.next() - 1.0,
            );
            if v.norm() > 0.1 && v.norm() < 1.0 {
                return v.normalize();
            }
        }
    }
}

fn check_chart(tag: &str, feet: [Vec3<f64>; 3], rng: &mut Lcg) {
    let (u_ref, axis) = corner_chart(feet);
    let [f_x, f_y, f_c] = feet;
    assert!(
        (axis - f_c).norm() < 1e-12 || (axis + f_c).norm() < 1e-12,
        "{tag}: pole {axis:?} is not ±f_c {f_c:?}"
    );
    for pole in [axis, -axis] {
        if (pole - f_c).norm() < 1e-12 {
            continue;
        }
        let w = weights(pole, feet);
        assert!(
            w.iter().any(|&x| x < 0.0),
            "{tag}: the off-vertex pole {pole:?} is inside, weights {w:?}"
        );
    }
    let ux = lon(f_x, u_ref, axis);
    assert!(ux.abs() < 1e-12, "{tag}: f_x at u {ux}");
    let theta = lon(f_y, u_ref, axis);
    assert!(theta > 0.0 && theta < PI, "{tag}: f_y at u {theta}");
    for _ in 0..64 {
        let (a, b, c) = (rng.next(), rng.next(), rng.next());
        let p = (f_x * a + f_y * b + f_c * (c * 1e-3)).normalize();
        let u = lon(p, u_ref, axis);
        assert!(
            u >= -1e-12 && u <= theta + 1e-12,
            "{tag}: interior sample at u {u} outside [0, {theta}]"
        );
    }
}

#[test]
fn p1_corner_chart_poles_on_a_foot_over_random_and_extreme_trihedra() {
    let mut rng = Lcg(0x5048);
    let mut cases: Vec<(String, [Vec3<f64>; 3])> = Vec::new();
    for i in 0..400 {
        let n = [rng.unit(), rng.unit(), rng.unit()];
        if n[0].dot(n[1].cross(n[2])).abs() > 1e-3 {
            cases.push((format!("random {i}"), n));
        }
    }
    // Near-dependent: the third normal within `eps` of the plane of the
    // first two (very acute / very obtuse dihedrals at the corner).
    for eps in [1e-2, 1e-4, 1e-6] {
        for spread in [0.05f64, 1.0, 3.0] {
            let a = Vec3::new(1.0, 0.0, 0.0);
            let b = Vec3::new(spread.cos(), spread.sin(), 0.0);
            let c = Vec3::new(-0.6, 0.8, eps).normalize();
            cases.push((
                format!("near-dependent eps {eps} spread {spread}"),
                [a, b, c],
            ));
            cases.push((
                format!("near-dependent' eps {eps} spread {spread}"),
                [a, c, b],
            ));
        }
    }
    let mut read = 0;
    for (tag, n) in &cases {
        for side in [1.0, -1.0] {
            let f = n.map(|v| v * side);
            for perm in [
                [0, 1, 2],
                [1, 0, 2],
                [0, 2, 1],
                [2, 0, 1],
                [1, 2, 0],
                [2, 1, 0],
            ] {
                check_chart(
                    &format!("{tag} side {side} perm {perm:?}"),
                    perm.map(|k| f[k]),
                    &mut rng,
                );
                read += 1;
            }
            let conv = if side > 0.0 {
                Convexity::Convex
            } else {
                Convexity::Concave
            };
            let Surface::Sphere { axis, u_ref, .. } =
                corner_ball([Point3::origin(); 3], *n, 0.1, conv).surface
            else {
                panic!("sphere");
            };
            assert!((axis.cross(f[2])).norm() < 1e-12, "{tag}: corner_ball pole");
            assert!(
                u_ref.dot(axis).abs() < 1e-12 && lon(f[0], u_ref, axis).abs() < 1e-12,
                "{tag}: corner_ball seam"
            );
        }
    }
    assert!(read > 2000, "read {read}");
}

fn tilted_cube(angle: f64, axis: Vec3<f64>) -> Body<f64> {
    let big = sweep::test_support::brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0), Tol::witness());
    let t = topo::transform_rigid(
        &big,
        &geom_core::RigidMotion::translation(Vec3::new(-1.55, -1.55, -2.45)).unwrap(),
        Tol::witness(),
    )
    .unwrap();
    topo::transform_rigid(
        &t,
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(Point3::new(1.0, 1.0, 1.0), axis.normalize(), angle),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .unwrap()
}

fn clip(base: Body<f64>, angle: f64, axis: Vec3<f64>) -> Option<Body<f64>> {
    let out = boolean_op_with(
        BooleanOp::Intersect,
        &finished("base", base, Tol::witness()),
        &finished("tilted", tilted_cube(angle, axis), Tol::witness()),
        &topo::BooleanInputs::default(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .ok()?;
    Some(out.body()?.body.clone().into_body())
}

/// The face-level checks; returns how many corner faces were read.
fn corner_faces_hold(tag: &str, out: &Filleted<f64>) -> usize {
    let body = &out.body;
    let band = Band::linear(Tol::witness()).unwrap();
    for &face in &out.corner_faces {
        let f = body.get_face(face).unwrap();
        let surface = body.get_surface(f.surface).unwrap();
        let Surface::Sphere {
            center,
            axis,
            u_ref,
            ..
        } = *surface
        else {
            panic!("{tag}: corner is a sphere");
        };
        let (outer, _) = topo::props::loop_edges(body, f.outer).unwrap();
        let want = if f.sense {
            Sign::Positive
        } else {
            Sign::Negative
        };
        assert_eq!(
            geom_brep::props::boundary_material_sign(surface, &outer, band),
            Ok(geom_brep::props::MaterialSign::Encoded(want)),
            "{tag}: corner {face:?} encodes its bit"
        );
        let LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
            panic!("cycle");
        };
        let verts: Vec<Vec3<f64>> = body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| {
                let vk = body.get_half_edge(he).unwrap().start;
                let q = *body
                    .get_vertex(vk)
                    .and_then(|v| body.get_point(v.point))
                    .unwrap();
                (q - center).normalize()
            })
            .collect();
        assert!(
            verts
                .iter()
                .any(|v| (*v - axis).norm() < 1e-9 || (*v + axis).norm() < 1e-9),
            "{tag}: corner {face:?}'s pole {axis:?} is on none of its vertices {verts:?}"
        );
        // The u window of the sampled boundary, poles excluded.
        let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
        for e in &outer {
            for k in 0..=32 {
                let t = e.t0 + (e.t1 - e.t0) * f64::from(k) / 32.0;
                let d = (e.carrier.eval(t) - center).normalize();
                if d.cross(axis).norm() < 1e-6 {
                    continue;
                }
                let u = lon(d, u_ref, axis);
                lo = lo.min(u);
                hi = hi.max(u);
            }
        }
        assert!(
            lo > -1e-9 && hi < PI,
            "{tag}: corner {face:?}'s boundary spans u [{lo}, {hi}]"
        );
    }
    out.corner_faces.len()
}

#[test]
fn p2_oblique_clips_pole_on_a_vertex_and_hold_tiers_mesh_and_volume() {
    type Base = Box<dyn Fn() -> Body<f64>>;
    let bases: Vec<(&str, Base)> = vec![
        ("cube", Box::new(|| cube::<f64>(1.0, Tol::witness()))),
        (
            "acute prism",
            Box::new(|| {
                let a: f64 = 0.35;
                prism(
                    corners(&[(0.0, 0.0), (1.2, 0.0), (1.2 * a.cos(), 1.2 * a.sin())]),
                    1.0,
                    Tol::witness(),
                )
            }),
        ),
        (
            "obtuse prism",
            Box::new(|| {
                prism(
                    corners(&[(0.0, 0.2), (1.0, 0.0), (2.0, 0.2), (1.0, 1.0)]),
                    1.0,
                    Tol::witness(),
                )
            }),
        ),
    ];
    let tilts = [
        (0.4, Vec3::new(1.0, -1.0, 0.0)),
        (0.15, Vec3::new(1.0, 2.0, 3.0)),
        (0.9, Vec3::new(0.0, 1.0, 1.0)),
        (1.2, Vec3::new(3.0, -1.0, 2.0)),
    ];
    let mut built = 0;
    let mut read = 0;
    let mut refused = Vec::new();
    let mut unmeshed = 0;
    for (name, base) in &bases {
        for (angle, ax) in tilts {
            for r in [0.03, 0.06] {
                let tag = format!("{name} tilted {angle} about {ax:?}, r {r}");
                let Some(body) = clip(base(), angle, ax) else {
                    refused.push(format!("{tag}: clip refused"));
                    continue;
                };
                let edges = query::all_edges(&body);
                let out = match fillet_edges(
                    &at_rest(&body, Tol::witness()),
                    &edges,
                    r,
                    Tol::witness(),
                ) {
                    Ok(o) => o,
                    Err(e) => {
                        refused.push(format!("{tag}: {e:?}"));
                        continue;
                    }
                };
                built += 1;
                read += corner_faces_hold(&tag, &out);
                assert_eq!(topo::validate(&out.body), Ok(()), "{tag}: tier 1");
                assert_eq!(topo::validate_closed(&out.body), Ok(()), "{tag}: tier 2");
                assert_eq!(
                    topo::validate_geometric(&out.body, Tol::witness()),
                    Ok(()),
                    "{tag}: tier 3"
                );
                // An oblique corner's third side is a great circle
                // tilted against the chart, which the mesher refuses on
                // main too (`work/tess/sphere-face-bounded-by-a-tilted-
                // circle-has-no-tessellation-lane.md`); only that refusal
                // is admitted.
                match mesh::tessellate(&out.body, 2e-3, Tol::witness()) {
                    Ok(m) => mesh::validate::check_mesh(&m)
                        .unwrap_or_else(|e| panic!("{tag}: watertight: {e:?}")),
                    Err(mesh::TessellateError::UnsupportedCurvedShape { face, .. })
                        if out.corner_faces.contains(&face) =>
                    {
                        unmeshed += 1;
                    }
                    Err(e) => panic!("{tag}: tessellates: {e:?}"),
                }
                let total: f64 = edges
                    .iter()
                    .map(|&k| {
                        let c = body
                            .get_edge(k)
                            .and_then(|e| body.get_curve_geom(e.curve))
                            .and_then(|g| g.certified())
                            .unwrap();
                        let (t0, t1) = c.params();
                        (c.carrier().eval(t1) - c.carrier().eval(t0)).norm()
                    })
                    .sum();
                let vol = |b: &Body<f64>| topo::mass_properties(b, Tol::witness()).unwrap().volume;
                let (v0, v1) = (vol(&body), vol(&out.body));
                assert!(
                    v1 < v0 && v1 > v0 - r * r * total,
                    "{tag}: volume {v1} outside ({}, {v0})",
                    v0 - r * r * total
                );
            }
        }
    }
    println!("built {built}, corners read {read}, unmeshed at a corner {unmeshed}");
    for r in &refused {
        println!("refused: {r}");
    }
    assert!(built >= 6 && read >= 30, "built {built}, read {read}");
}

/// Diagnostic: does the oblique clip's fillet tessellate, and if not,
/// which face refuses (a corner patch or not)?
#[test]
fn p3_diag_oblique_clip_tessellation() {
    let mut lines = Vec::new();
    for (angle, ax) in [
        (0.4, Vec3::new(1.0, -1.0, 0.0)),
        (0.15, Vec3::new(1.0, 2.0, 3.0)),
    ] {
        for r in [0.03, 0.08] {
            let body = clip(cube::<f64>(1.0, Tol::witness()), angle, ax).unwrap();
            let out = fillet_edges(
                &at_rest(&body, Tol::witness()),
                &query::all_edges(&body),
                r,
                Tol::witness(),
            )
            .unwrap();
            let res = mesh::tessellate(&out.body, 2e-3, Tol::witness());
            let line = match res {
                Ok(m) => format!(
                    "{angle} r {r}: ok, watertight {:?}",
                    mesh::validate::check_mesh(&m).is_ok()
                ),
                Err(mesh::TessellateError::UnsupportedCurvedShape { face, source }) => {
                    let s = out
                        .body
                        .get_surface(out.body.get_face(face).unwrap().surface)
                        .unwrap();
                    format!(
                        "{angle} r {r}: {source:?} on {face:?} corner {} surface {s:?}",
                        out.corner_faces.contains(&face)
                    )
                }
                Err(e) => format!("{angle} r {r}: {e:?}"),
            };
            lines.push(line);
        }
    }
    println!("P3\n{}", lines.join("\n"));
}

/// Diagnostic for the `-0.0` axis flip on box corners: dumps every
/// corner face's pcurve rows, the mesh positions and patch indices,
/// and the mass properties of filleted cubes at every pose, to the
/// file `R5048_BITS` names, for a head-vs-main diff.
#[test]
fn p4_diag_box_corner_bits() {
    let Ok(path) = std::env::var("R5048_BITS") else {
        return;
    };
    let mut s = String::new();
    for (pose, a) in crate::common::poses::poses() {
        let body = topo::transform_rigid(
            &cube::<f64>(1.0, Tol::witness()),
            &geom_core::test_support::rigid(&a, Tol::witness()),
            Tol::witness(),
        )
        .unwrap();
        let out = fillet_edges(
            &at_rest(&body, Tol::witness()),
            &query::all_edges(&body),
            0.15,
            Tol::witness(),
        )
        .unwrap();
        s += &format!("POSE {pose}\n");
        for &face in &out.corner_faces {
            let f = out.body.get_face(face).unwrap();
            let LoopBoundary::Cycle { first } = out.body.get_loop(f.outer).unwrap().boundary else {
                panic!()
            };
            for he in out.body.loop_cycle(first).unwrap() {
                s += &format!("{face:?} {he:?} {:?}\n", out.body.pcurve(he));
            }
        }
        let m = mesh::tessellate(&out.body, 2e-3, Tol::witness()).unwrap();
        for p in &m.positions {
            s += &format!(
                "{:x} {:x} {:x}\n",
                p.x.to_bits(),
                p.y.to_bits(),
                p.z.to_bits()
            );
        }
        s += &format!("{:?}\n", m.patches);
        s += &format!(
            "{:?}\n",
            topo::mass_properties(&out.body, Tol::witness()).unwrap()
        );
    }
    std::fs::write(path, s).unwrap();
}
