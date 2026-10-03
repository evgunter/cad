//! **A row stored as the sphere's involution twin certifies at the
//! certifying scalar.** The pole-crossing half cap's meridian arc is
//! stored by the loop walk as the sphere's twin `(u + π, π − v)`
//! (`topo::pcurves::sphere_twin`), and check 4's fidelity reads it
//! against the twin's branch (`geom_brep::whole_periods`, decided as
//! `pcurve_fidelity_twin`). An exact twin's azimuth offset is π, which
//! is the centred fold's jump: a selection read off that fold encloses
//! both signs over a box and kept the derivation's own name, so the cap
//! minted at f64 and refused at `Interval` (PR 3812's review R1,
//! MAJOR-1). Built at both scalars here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::spline::SpanLocate;
use geom_core::{Interval, Point3, Real, Tol, Vec3};
use topo::{AtRestPolicy, Body, FaceSurface, MefSite, MevSite};

fn half_cap<T: Real + SpanLocate + AtRestPolicy>() -> Result<(), String> {
    let tol = Tol::witness();
    let f = T::from_f64;
    let z = 0.5_f64;
    let r = (1.0 - z * z).sqrt();
    let p = |x: f64, y: f64, w: f64| Point3::new(f(x), f(y), f(w));
    let v = |x: f64, y: f64, w: f64| Vec3::new(f(x), f(y), f(w));
    let (a, b) = (p(r, 0.0, z), p(-r, 0.0, z));
    let rim = Curve3::Circle {
        center: p(0.0, 0.0, z),
        axis: v(0.0, 0.0, 1.0),
        radius: f(r),
        u_ref: v(1.0, 0.0, 0.0),
    };
    // Oriented as `mesh`'s witness body orients its pole-crossing half
    // cap (`mesh/tests/common/witness_bodies.rs`).
    let g = Curve3::Circle {
        center: p(0.0, 0.0, 0.0),
        axis: v(0.0, 1.0, 0.0),
        radius: f(1.0),
        u_ref: v(-r, 0.0, z),
    };
    let t_end = f(2.0 * core::f64::consts::FRAC_PI_3);
    let mut body = Body::<T>::new();
    let seed = body.mvfs(a, true).map_err(|e| format!("mvfs {e:?}"))?;
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Sphere {
                center: p(0.0, 0.0, 0.0),
                radius: f(1.0),
                axis: v(0.0, 0.0, 1.0),
                u_ref: v(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .map_err(|e| format!("surface {e:?}"))?;
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim, f(0.0), T::pi()).unwrap(),
            tol,
        )
        .map_err(|e| format!("mev {e:?}"))?;
    body.mef(
        MefSite::Chords {
            he1: e_rim.he_minus,
            he2: e_rim.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(g, f(0.0), t_end).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .map_err(|e| format!("mef {e:?}"))?;
    topo::pcurves::mint_pcurves(&mut body, tol).map_err(|e| format!("mint {e:?}"))?;
    Ok(())
}

#[test]
fn the_pole_crossing_half_cap_mints_at_f64_and_at_interval() {
    let at_f64 = half_cap::<f64>();
    let at_iv = half_cap::<Interval>();
    assert!(
        at_f64.is_ok() && at_iv.is_ok(),
        "f64 {at_f64:?} / Interval {at_iv:?}"
    );
}
