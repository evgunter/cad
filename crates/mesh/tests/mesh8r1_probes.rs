//! R1 review probes for MESH-8 (issue 868, PR #1585), which deleted
//! the walk's `closing_column` assertion: the issue-1571 body at
//! δ = 0.5, its outcome PRINTED and the one thing asserted being that
//! no panic there is the deleted assertion's — run with `walk.rs` at
//! the merge base this row goes red. Since MESH-11 that body no longer
//! reaches the walk at all: the branch door
//! (`props::require_one_chart_branch`) refuses it typed at every δ, so
//! what this row now prints is that refusal, and the assertion — no
//! panic — holds for a second reason.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use topo::construction::Construct;

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use topo::Body;
use topo::construction::{FaceSurface, MefSite, MevSite};

/// Issue 1571's body — the half-cap with ONE great-circle arc over the
/// north pole.
///
/// Re-derived rather than imported (`mesh8r2_probes.rs` keeps another
/// copy, and `common::witness_bodies` the shared one): an independent
/// instrument over its own subject.
fn pole_crossing_half_cap() -> Body<f64> {
    let tol = Tol::witness();
    let z = 0.5_f64;
    let r = (1.0 - z * z).sqrt();
    let a = Point3::new(r, 0.0, z);
    let b = Point3::new(-r, 0.0, z);
    let rim = Curve3::circle(
        Point3::new(0.0, 0.0, z),
        Vec3::new(0.0, 0.0, 1.0),
        r,
        Vec3::new(1.0, 0.0, 0.0),
    );
    let great = |axis: Vec3<f64>| {
        Curve3::circle(Point3::new(0.0, 0.0, 0.0), axis, 1.0, Vec3::new(-r, 0.0, z))
    };
    let mut g = great(Vec3::new(0.0, 1.0, 0.0));
    if g.eval(core::f64::consts::FRAC_PI_2).z < z {
        g = great(Vec3::new(0.0, -1.0, 0.0));
    }
    let t_end = g.param_near(a, 0.0).unwrap();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a, true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Sphere {
                frame: geom::Frame::new(
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 0.0),
                ),
                radius: 1.0,
            },
            sense: true,
        },
    )
    .unwrap();
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim, 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e_rim.he_minus,
            he2: e_rim.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(g, 0.0, t_end).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

/// Tessellates under `catch_unwind` and returns what happened as text:
/// the panic's first line, or the `Ok`/`Err` shape with `check_mesh`.
fn tessellate_outcome(body: &Body<f64>, delta: f64) -> String {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mesh::tessellate(body, delta, Tol::witness())
    }));
    std::panic::set_hook(hook);
    match out {
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_string()))
                .unwrap_or_default();
            format!("PANIC {}", msg.lines().next().unwrap_or(""))
        }
        Ok(Ok(m)) => format!(
            "Ok positions={} check_mesh={:?}",
            m.positions.len(),
            mesh::validate::check_mesh(&m)
        ),
        Ok(Err(e)) => format!("Err({e:?})"),
    }
}

/// **The BEFORE / AFTER half of the relocation, at δ = 0.5 on issue
/// 1571's body.** Printed, and one thing asserted: whatever says
/// something here, it is not the deleted `closing_column` assertion.
/// With `crates/mesh/src/walk.rs` checked out at the merge base this
/// row goes RED with that assertion's text, which is the "before".
#[test]
fn the_pole_crossing_body_at_delta_half_is_not_announced_by_the_deleted_assertion() {
    let body = pole_crossing_half_cap();
    // The tiers, for the record: issue 1571's text says its Euler-door
    // bodies were tier-3 refused at rest for construction artefacts.
    // Printed, whichever it is.
    println!(
        "PROBE 1571 tiers: validate={:?} validate_closed={:?} validate_geometric={:?}",
        topo::validate(&body).map_err(|e| e.len()),
        topo::validate_closed(&body).map_err(|e| e.len()),
        topo::validate_geometric(&body, Tol::witness()).map_err(|e| format!("{e:?}"))
    );
    for delta in [0.5, 0.1] {
        let got = tessellate_outcome(&body, delta);
        println!("PROBE 1571 delta={delta}: {got}");
        assert!(
            !got.contains("closing meridian's carrier-midpoint azimuth"),
            "the deleted assertion is what announced this body: {got}"
        );
    }
}
