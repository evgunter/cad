//! **Probe (ONEDOOR loop lift, PR 6851, probe b): a non-meridian circle
//! through a pole, meeting another edge there.** A measurement, not a
//! product test: it prints what each reader does.
//!
//! The unit sphere about `z`. A small circle tilted `β` from the axis
//! passes through the north pole `N` (`chart_window`'s own
//! `small_circle(β)`), leaving it along `−y`, and reaches `P` at polar
//! angle `2β` on the `x z` meridian. The loop is that circle's half from
//! `N` to `P` and the meridian from `P` back up to `N`: a lens, and its
//! complement. At `N` the meridian and the small circle meet.
//!
//! For each face it reports what the mint stored on every half-edge
//! (image kind and joint element), what `face_azimuth_window_traces`
//! does, what `loop_lift` gives, what tier 3 (`validate_pcurves`) says,
//! and what the mesh does.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom_brep::{EdgeCurveSpec, Pcurve};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
use topo::{Body, FaceKey, LoopBoundary, face_azimuth_window_traces, validate, validate_pcurves};

pub(crate) fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

pub(crate) fn unit_sphere() -> geom::Surface<f64> {
    geom::Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// `chart_window`'s `small_circle(β)`: through `N` at `t = 0`, its far
/// point `(sin 2β, 0, cos 2β)` at `t = π`.
fn small_circle(beta: f64) -> Curve3<f64> {
    let (s, c) = beta.sin_cos();
    Curve3::circle(
        Point3::new(s * c, 0.0, c * c),
        Vec3::new(s, 0.0, c),
        s,
        Vec3::new(-c, 0.0, s),
    )
}

/// The `x z` great circle from `N`: `eval(t) = (sin t, 0, cos t)`.
fn meridian_from_north() -> Curve3<f64> {
    Curve3::circle(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Vec3::new(0.0, 0.0, 1.0),
    )
}

/// Every half-edge of every loop of `fk`: its image kind, its joint
/// element, and its start vertex's point.
pub(crate) fn dump_face(body: &Body<f64>, fk: FaceKey) {
    let face = body.get_face(fk).unwrap();
    println!("== face {fk:?} sense {} surface {:?}", face.sense, face.surface);
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            println!("   loop {lk:?}: not a cycle");
            continue;
        };
        println!("   loop {lk:?}{}", if lk == face.outer { " (outer)" } else { " (ring)" });
        for he in body.loop_cycle(first).unwrap() {
            let h = body.get_half_edge(he).unwrap();
            let p = body
                .vertex_points()
                .find(|(v, _)| *v == h.start)
                .map(|(_, p)| p)
                .unwrap();
            let kind = body.pcurve(he).map(|c| match c.pcurve() {
                Pcurve::Harmonic { .. } => "Harmonic",
                Pcurve::Projected(_) => "Projected",
                Pcurve::Fitted(_) => "Fitted",
                Pcurve::General(_) => "General",
                Pcurve::Spiric { .. } => "Spiric",
                Pcurve::FocalSection(_) => "FocalSection",
                _ => "other",
            });
            let ends = body.pcurve(he).map(|c| {
                let (t0, t1) = c.params();
                let (a, b) = (c.pcurve().eval(t0), c.pcurve().eval(t1));
                format!("({:.4},{:.4})->({:.4},{:.4})", a.x, a.y, b.x, b.y)
            });
            println!(
                "     he {he:?} start ({:.4},{:.4},{:.4}) image {kind:?} {} joint {:?}",
                p.x,
                p.y,
                p.z,
                ends.unwrap_or_default(),
                body.joint(he)
            );
        }
        match body.loop_lift(lk) {
            Ok(rows) => {
                let s: Vec<String> = rows
                    .iter()
                    .map(|r| {
                        let he = body.get_half_edge(r.half_edge).unwrap();
                        let fwd = body.get_edge(he.edge).unwrap().he_plus == r.half_edge;
                        let (t0, t1) = r.row.params();
                        let (a, b) = if fwd { (t0, t1) } else { (t1, t0) };
                        let (p, q) = (r.pcurve.eval(a), r.pcurve.eval(b));
                        format!(
                            "[{:?} ({:.4},{:.4})->({:.4},{:.4})]",
                            r.lift, p.x, p.y, q.x, q.y
                        )
                    })
                    .collect();
                println!("   loop_lift: {}", s.join(" "));
            }
            Err(e) => println!("   loop_lift: {e:?}"),
        }
    }
    println!(
        "   face_azimuth_window_traces: {:?}",
        face_azimuth_window_traces(body, fk, band())
    );
    println!(
        "   loop_windings: {:?}",
        topo::loop_windings(body, fk, body.face_surface(fk).unwrap(), band())
    );
}

pub(crate) fn dump_body(name: &str, body: &Body<f64>) {
    println!("######## {name}");
    println!("tier 1: {:?}", validate(body));
    println!("tier 3 (validate_pcurves): {:?}", validate_pcurves(body, band()));
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    for fk in faces {
        dump_face(body, fk);
    }
    match mesh::tessellate(body, 0.02, Tol::witness()) {
        Ok(m) => {
            for p in &m.patches {
                let area: f64 = p
                    .triangles
                    .iter()
                    .map(|t| {
                        let [a, b, c] = t.map(|i| m.positions[i as usize]);
                        0.5 * (b - a).cross(c - a).norm()
                    })
                    .sum();
                println!(
                    "mesh: face {:?}: {} triangles, area {area:.5}",
                    p.face,
                    p.triangles.len()
                );
            }
        }
        Err(e) => println!("mesh: tessellate refuses: {e:?}"),
    }
}

/// The lens at tilt `beta`, minted by the Euler doors.
fn lens(beta: f64, sense: bool) -> Body<f64> {
    let tol = Tol::witness();
    let small = small_circle(beta);
    let n = small.eval(0.0);
    let p = small.eval(core::f64::consts::PI);
    let mut body = Body::<f64>::new();
    let start = body.mvfs(n, sense).unwrap();
    body.set_face_surface(
        start.face,
        FaceSurface::New {
            surface: unit_sphere(),
            sense,
        },
    )
    .unwrap();
    let m = body
        .mev(
            MevSite::Lone {
                r#loop: start.r#loop,
            },
            p,
            EdgeCurveSpec::arc_of_circle(small, 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .unwrap();
    // The chord runs from `start(he1)` = N to `start(he2)` = P.
    body.mef(
        MefSite::Chords {
            he1: m.he_plus,
            he2: m.he_minus,
        },
        EdgeCurveSpec::arc_of_circle(meridian_from_north(), 0.0, 2.0 * beta).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

#[test]
fn probe_a_tilted_circle_through_the_pole() {
    for beta in [0.3, 0.6] {
        for sense in [true, false] {
            let body = lens(beta, sense);
            dump_body(&format!("lens β = {beta}, sense {sense}"), &body);
            let mut minted = body.clone();
            let got = topo::mint_pcurves(&mut minted, Tol::witness());
            println!("-- after mint_pcurves (the at-rest mint pass): {got:?}");
            if got.is_ok() {
                dump_body(&format!("lens β = {beta}, sense {sense}, re-minted"), &minted);
            }
        }
    }
}
