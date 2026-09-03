//! **R2 review probes for MESH-11 (issue 1571): the two witnesses
//! through the public `tessellate`, PRINTED, on either side of the
//! merge base.** Self-contained on purpose — it uses nothing this
//! unit added — so the same file compiles on the merge base sources
//! and on the head, and the printed outcomes are the before/after.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use topo::{Body, FaceSurface, MefSite, MevSite};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}
fn arc(circle: Curve3<f64>, t0: f64, t1: f64) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec::arc_of_circle(circle, t0, t1).expect("arc spec")
}

/// Issue 1571's body: the unit sphere, a rim at z = 0.5 (u = 0 → π)
/// and ONE great-circle arc from the rim's u = π end over the north
/// pole to its u = 0 end.
fn pole_crossing_half_cap() -> Body<f64> {
    let tol = Tol::witness();
    let z = 0.5_f64;
    let r = (1.0 - z * z).sqrt();
    let a = p3(r, 0.0, z);
    let b = p3(-r, 0.0, z);
    let rim = Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: v3(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let great = |axis: Vec3<f64>| Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis,
        radius: 1.0,
        u_ref: v3(-r, 0.0, z),
    };
    let mut g = great(v3(0.0, 1.0, 0.0));
    if g.eval(core::f64::consts::FRAC_PI_2).z < z {
        g = great(v3(0.0, -1.0, 0.0));
    }
    let t_end = g.param_near(a, 0.0).unwrap();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Sphere {
            center: p3(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: v3(0.0, 0.0, 1.0),
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            arc(rim, 0.0, core::f64::consts::PI),
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e_rim.he_minus,
            he2: e_rim.he_plus,
        },
        arc(g, 0.0, t_end),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

/// The cone bow tie: the 45° cone about +Z, apex at the origin, rims
/// at signed slant ∓1, two generator segments THROUGH the apex.
fn apex_crossing_bowtie() -> Body<f64> {
    let tol = Tol::witness();
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let (a, b, c, d) = (
        p3(s, 0.0, -s),
        p3(-s, 0.0, -s),
        p3(-s, 0.0, s),
        p3(s, 0.0, s),
    );
    let rim = |z: f64| Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: v3(0.0, 0.0, 1.0),
        radius: s,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let line = EdgeCurveSpec::line_between;
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Cone {
            apex: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e_ab = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            arc(rim(-s), 0.0, core::f64::consts::PI),
            tol,
        )
        .unwrap();
    let strut = |body: &mut Body<f64>, at, to, spec| {
        body.mev(MevSite::Fan { he1: at, he2: at }, to, spec, tol)
            .unwrap()
    };
    let e_bd = strut(&mut body, e_ab.he_minus, d, line(b, d));
    let e_dc = strut(
        &mut body,
        e_bd.he_minus,
        c,
        arc(rim(s), 0.0, core::f64::consts::PI),
    );
    body.mef(
        MefSite::Chords {
            he1: e_dc.he_minus,
            he2: e_ab.he_plus,
        },
        line(c, a),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

fn outcome(body: &Body<f64>, d: f64, tol: Tol) -> String {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mesh::tessellate(body, d, tol)
    }));
    std::panic::set_hook(hook);
    match out {
        Ok(Ok(m)) => format!(
            "Ok({} positions, {} tris, check_mesh={:?})",
            m.positions.len(),
            m.patches.iter().map(|p| p.triangles.len()).sum::<usize>(),
            mesh::validate::check_mesh(&m).map(|_| ())
        ),
        Ok(Err(e)) => format!("Err({e:?})"),
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_string()))
                .unwrap_or_default();
            format!("PANIC({})", msg.lines().next().unwrap_or(""))
        }
    }
}

#[test]
fn r2_both_witnesses_through_tessellate_printed() {
    let tol = Tol::witness();
    println!(
        "R2-WITNESS debug_assertions={} eps={:e}",
        cfg!(debug_assertions),
        tol.eps()
    );
    for (name, body) in [
        ("half-cap", pole_crossing_half_cap()),
        ("bow-tie", apex_crossing_bowtie()),
    ] {
        let mp = topo::mass_properties(&body, tol).map(|m| m.volume);
        println!("R2-WITNESS {name} mass_properties volume = {mp:?}");
        for d in [0.5_f64, 0.3, 0.2, 0.1, 0.05, 0.02] {
            println!("R2-WITNESS {name} delta={d}: {}", outcome(&body, d, tol));
        }
    }
}
