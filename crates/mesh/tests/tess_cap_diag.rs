//! TESS-CAP-DIAG scratch survey (diagnostic branch only, not for main):
//! what `mesh::tessellate` does with a sphere face bounded by ONE
//! rim-only loop (the pole interior), beside the faces the kernel's
//! own verbs mint. Prints; asserts nothing. Run single-threaded with
//! `--nocapture`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout,
    clippy::print_stderr
)]

use crate::common;
use common::*;
use geom::{Curve3, Surface};
use geom_brep::props::{require_iso_rectangle, require_one_chart_branch};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use std::sync::Mutex;
use sweep::{Extrusion, Revolution, extrude, revolve};
use topo::{Body, FaceSurface, MefSite, MevSite};

static LAST_PANIC_SITE: Mutex<String> = Mutex::new(String::new());

fn hook() {
    std::panic::set_hook(Box::new(|info| {
        let loc = info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_default();
        *LAST_PANIC_SITE.lock().unwrap() = loc;
    }));
}

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}
fn sphere(r: f64) -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: r,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}
fn plane(z: f64, up: bool) -> Surface<f64> {
    Surface::Plane {
        origin: p3(0.0, 0.0, z),
        normal: v3(0.0, 0.0, if up { 1.0 } else { -1.0 }),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}
fn circle(z: f64, r: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: v3(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// `topo/tests/props_sphere_cap_door.rs::cut_ball`, generalised to any
/// (seed, made) surface pair sharing the circle of radius `r` at `z`.
fn cut_body(z: f64, r: f64, seed_s: Surface<f64>, made_s: Option<Surface<f64>>) -> Body<f64> {
    let tol = Tol::witness();
    let (a, b) = (p3(r, 0.0, z), p3(-r, 0.0, z));
    let pi = core::f64::consts::PI;
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(seed_s))
        .unwrap();
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(circle(z, r), 0.0, pi).unwrap(),
            tol,
        )
        .unwrap();
    let transverse = made_s.is_some();
    let made = body
        .mef(
            MefSite::Chords {
                he1: e_rim.he_minus,
                he2: e_rim.he_plus,
            },
            EdgeCurveSpec::arc_of_circle(circle(z, r), pi, core::f64::consts::TAU).unwrap(),
            made_s.map_or(FaceSurface::Inherit, FaceSurface::New),
            tol,
        )
        .unwrap();
    let s_seed = body.get_face(seed.face).unwrap().surface;
    let s_made = body.get_face(made.face).unwrap().surface;
    for (edge, witness) in [(e_rim.edge, p3(0.0, r, z)), (made.edge, p3(0.0, -r, z))] {
        if transverse {
            let curve = body.get_edge(edge).unwrap().curve;
            let spec = body
                .get_curve_geom(curve)
                .unwrap()
                .certified()
                .unwrap()
                .restated_spec();
            let spec = EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    s1: s_seed,
                    s2: s_made,
                    witness,
                },
                ..spec
            };
            body.set_edge_curve(edge, spec, tol).unwrap();
        } else {
            body.describe_at_rest(edge, s_seed, tol).unwrap();
        }
    }
    body
}

fn cut_ball(z: f64, seed_s: Surface<f64>, made_s: Option<Surface<f64>>) -> Body<f64> {
    cut_body(z, (1.0 - z * z).sqrt(), seed_s, made_s)
}

fn kind(s: &Surface<f64>) -> String {
    let d = format!("{s:?}");
    d.split([' ', '{', '(']).next().unwrap_or("").to_string()
}

fn carrier(c: &Curve3<f64>) -> String {
    match c {
        Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => format!(
            "Circle(c=({:.3},{:.3},{:.3}) ax=({:.2},{:.2},{:.2}) r={:.4})",
            center.x, center.y, center.z, axis.x, axis.y, axis.z, radius
        ),
        other => {
            let d = format!("{other:?}");
            d.split([' ', '{', '(']).next().unwrap_or("").to_string()
        }
    }
}

fn tri_area(p: [Point3<f64>; 3]) -> f64 {
    let (a, b) = (p[1] - p[0], p[2] - p[0]);
    0.5 * a.cross(b).norm()
}

/// Everything the survey asks of one body. `exact_area` is the curved
/// face's analytic area where there is one.
fn report(name: &str, body: &Body<f64>, delta: f64) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    println!("\n=== {name} (delta {delta})");
    println!(
        "  census V={} E={} F={}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count()
    );
    for (fk, face) in body.faces() {
        let s = body.get_surface(face.surface).unwrap();
        let (outer, _) = topo::props::loop_edges(body, face.outer).unwrap();
        println!(
            "  face {fk:?} {} sense={} rings={} outer edges={}",
            kind(s),
            face.sense,
            face.rings.len(),
            outer.len()
        );
        for e in &outer {
            println!(
                "      {} t[{:.4},{:.4}] fwd={}",
                carrier(&e.carrier),
                e.t0,
                e.t1,
                e.forward
            );
        }
        if !matches!(s, Surface::Plane { .. }) {
            println!(
                "      doors: iso_rectangle={:?} one_chart_branch={:?}",
                require_iso_rectangle(s, &outer, band),
                require_one_chart_branch(s, &outer, band)
            );
        }
    }
    println!(
        "  tiers: t1={:?} t2={:?} t3={:?}",
        topo::validate(body),
        topo::validate_closed(body),
        topo::validate_geometric(body, tol)
    );
    match topo::mass_properties(body, tol) {
        Ok(mp) => println!(
            "  props: volume={:.12e} (pad {:e}) area={:.12e} (pad {:e})",
            mp.volume, mp.volume_pad, mp.surface_area, mp.area_pad
        ),
        Err(e) => println!("  props: REFUSED {e:?}"),
    }
    let coh = topo::examine_chart_coherence(body, tol);
    println!(
        "  coherence: findings={} unexamined={} {:?}",
        coh.findings.len(),
        coh.unexamined.len(),
        coh.findings
            .iter()
            .map(|f| &f.condition)
            .collect::<Vec<_>>()
    );
    let t = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mesh::tessellate(body, delta, tol)
    }));
    match t {
        Err(p) => {
            let s = p
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
                .unwrap_or_default();
            println!(
                "  tessellate: PANIC at {} :: {}",
                LAST_PANIC_SITE.lock().unwrap(),
                &s[..s.len().min(400)]
            );
        }
        Ok(Err(e)) => println!("  tessellate: REFUSED {e:?}"),
        Ok(Ok(m)) => {
            println!(
                "  tessellate: OK positions={} triangles={} check_mesh={:?} signed_volume={:.9e}",
                m.positions.len(),
                mesh::validate::triangle_count(&m),
                mesh::validate::check_mesh(&m),
                mesh::validate::signed_volume(&m)
            );
            for patch in &m.patches {
                let face = body.get_face(patch.face).unwrap();
                let s = body.get_surface(face.surface).unwrap();
                let mut area = 0.0;
                let mut dev: f64 = 0.0;
                for t in &patch.triangles {
                    let tri = t.map(|i| m.positions[i as usize]);
                    area += tri_area(tri);
                    dev = dev.max(sampled_deviation(s, tri, 4));
                }
                println!(
                    "      patch {:?} {}: triangles={} area={:.9e} max sampled dev={:.3e}",
                    patch.face,
                    kind(s),
                    patch.triangles.len(),
                    area,
                    dev
                );
            }
            for b in &m.boundaries {
                println!("      boundary {:?}: {} points", b.edge, b.points.len());
            }
        }
    }
}

#[test]
fn q1_q2_q6_rim_only_caps() {
    hook();
    let tau = core::f64::consts::TAU;
    for z in [0.0_f64, 0.5, 0.9, -0.9] {
        println!(
            "\n##### cap ABOVE z={z} (north pole interior): analytic sphere-face area {:.9e}, volume {:.9e}",
            tau * (1.0 - z),
            core::f64::consts::PI * (1.0 - z).powi(2) * (2.0 + z) / 3.0
        );
        report(
            &format!("cap above z={z} + disc"),
            &cut_ball(z, sphere(1.0), Some(plane(z, false))),
            0.01,
        );
    }
    for z in [0.0_f64, 0.5, -0.9] {
        println!(
            "\n##### ball BELOW z={z} (south pole interior): analytic sphere-face area {:.9e}",
            tau * (1.0 + z)
        );
        report(
            &format!("ball below z={z} + disc"),
            &cut_ball(z, plane(z, true), Some(sphere(1.0))),
            0.01,
        );
    }
    for z in [0.0_f64, 0.5] {
        report(
            &format!("two rim-only caps, one rim at z={z} (MESH-12's f=0 shape, unit ball)"),
            &cut_ball(z, sphere(1.0), None),
            0.01,
        );
    }
    // MESH-12's own fixture at dv = 0: R = 10 mm, rim latitude 0.5 rad.
    let (r, v) = (0.010_f64, 0.5_f64);
    report(
        "mesh12 two_level_rim_cap(dv=0) R=10mm v=0.5",
        &cut_body(r * v.sin(), r * v.cos(), sphere(r), None),
        1e-4,
    );
}

fn dome() -> Body<f64> {
    // Quarter disc (0,0)-(1,0)-arc-(0,1), revolved about Y: a hemisphere.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), (core::f64::consts::PI / 8.0).tan()),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn rod() -> Body<f64> {
    // Rectangle touching the axis: a solid cylinder.
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn slab_below_y(y0: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        [(-2.0, -2.0), (2.0, -2.0), (2.0, y0), (-2.0, y0)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
            .collect(),
    );
    let p = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let b = extrude(&p, Extrusion::Distance(4.0), Tol::witness())
        .unwrap()
        .body;
    topo::transform_rigid(
        &b,
        &geom_core::Affine3::translation(v3(0.0, 0.0, -2.0)),
        Tol::witness(),
    )
    .unwrap()
}

#[test]
fn q4_the_kernels_own_verbs() {
    hook();
    report("revolve: ball()", &ball(), 0.05);
    report("revolve: dome (quarter disc)", &dome(), 0.05);
    report("revolve: cone()", &cone(), 0.05);
    report("revolve: rod (solid cylinder)", &rod(), 0.05);
    for y0 in [0.5_f64, 0.0] {
        let r = topo::boolean_op_with(
            topo::BooleanOp::Intersect,
            &ball(),
            &slab_below_y(y0),
            &topo::BooleanDeclarations::default(),
            topo::SweepStrategy::Realized,
            Tol::witness(),
        );
        match r {
            Err(e) => println!("\n=== boolean ball ∩ slab(y<={y0}): REFUSED {e:?}"),
            Ok(br) => match br.body() {
                None => println!("\n=== boolean ball ∩ slab(y<={y0}): EMPTY"),
                Some(b) => report(&format!("boolean ball ∩ slab(y<={y0})"), &b.body, 0.05),
            },
        }
        let plane = topo::splitting::SplitPlane {
            origin: p3(0.0, y0, 0.0),
            normal: v3(0.0, 1.0, 0.0),
        };
        match topo::splitting::split(&ball(), &plane, Tol::witness()) {
            Err(e) => println!("\n=== split ball at y={y0}: REFUSED {e:?}"),
            Ok(sr) => {
                if let Some(b) = sr.above.body() {
                    report(&format!("split ball at y={y0}: ABOVE"), b, 0.05);
                }
                if let Some(b) = sr.below.body() {
                    report(&format!("split ball at y={y0}: BELOW"), b, 0.05);
                }
            }
        }
    }
    // A plane that does NOT contain a latitude: normal ⊥ the ball's axis.
    let plane = topo::splitting::SplitPlane {
        origin: p3(0.0, 0.0, 0.5),
        normal: v3(0.0, 0.0, 1.0),
    };
    match topo::splitting::split(&ball(), &plane, Tol::witness()) {
        Err(e) => println!("\n=== split ball at z=0.5 (normal ⊥ axis): REFUSED {e:?}"),
        Ok(sr) => {
            if let Some(b) = sr.above.body() {
                report("split ball at z=0.5: ABOVE", b, 0.05);
            }
        }
    }
}

#[test]
fn q5_the_class_cone_apex_cap() {
    hook();
    // 45° cone about +Z, apex at the origin, rim at height 1 (radius 1),
    // closed by the disc z = 1 (outward +z): the solid cone, its lateral
    // face bounded by ONE rim with the apex interior.
    let cone_s = Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    for (name, seed, made) in [
        (
            "cone apex cap: cone seed(+u), disc made",
            cone_s.clone(),
            plane(1.0, true),
        ),
        (
            "cone apex cap: disc seed(+u), cone made(-u)",
            plane(1.0, true),
            cone_s.clone(),
        ),
    ] {
        let b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            cut_body(1.0, 1.0, seed, Some(made))
        }));
        match b {
            Ok(b) => report(name, &b, 0.05),
            Err(_) => println!(
                "\n=== {name}: construction panicked at {}",
                LAST_PANIC_SITE.lock().unwrap()
            ),
        }
    }
}

#[test]
fn q4_merge_faces_on_a_seamed_dome() {
    hook();
    for (name, mut body) in [("dome", dome()), ("ball", ball())] {
        let r = body.merge_coplanar_faces(Tol::witness());
        let d = format!("{r:?}");
        println!(
            "\n=== merge_coplanar_faces({name}): {}",
            &d[..d.len().min(600)]
        );
        report(&format!("{name} after merge_coplanar_faces"), &body, 0.05);
    }
}

#[test]
fn q5_the_class_cylinder_rim_only() {
    hook();
    let cyl = Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        cut_body(1.0, 1.0, cyl, Some(plane(1.0, true)))
    }));
    match b {
        Ok(b) => report(
            "cylinder bounded by ONE rim + disc (unbounded face)",
            &b,
            0.05,
        ),
        Err(_) => println!(
            "\n=== cylinder rim-only: construction panicked at {}",
            LAST_PANIC_SITE.lock().unwrap()
        ),
    }
}
