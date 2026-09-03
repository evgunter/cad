//! **R2 review probes for MESH-12** (issue 1601, measurement 1): the
//! certifying doors' admission set for a sphere meridian span past
//! the winding bound, executed through `Body::mev` and
//! `Body::set_edge_curve`; and the one certified edge the parse now
//! answers differently from certification — a meridian carrier stated
//! a hair inside the sphere's radius, whose span certifies at the
//! carrier's lever and escalates at the sphere's.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom::{Curve3, Surface};
use geom_brep::props::{PropsError, curved_face, require_one_chart_branch};
use geom_brep::{CertCheck, CertifyError, EdgeCurveSpec};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};
use topo::{Body, EulerOpError, FaceSurface, MevSite};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;
/// The sphere under every row: R = 10 mm about +Z at the origin.
const RS: f64 = 0.010;

fn meridian(r: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// A sphere face seeded at the carrier's `t0` point, with one `mev`
/// along the meridian carrier of radius `r_c` over `[t0, t0 + dt]`.
fn mev_span(r_c: f64, t0: f64, dt: f64) -> Result<(Body<f64>, topo::MevCreated), EulerOpError> {
    let tol = Tol::witness();
    let carrier = meridian(r_c);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(carrier.eval(t0)).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: RS,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let created = body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        carrier.eval(t0 + dt),
        EdgeCurveSpec::arc_of_circle(carrier, t0, t0 + dt).unwrap(),
        tol,
    )?;
    Ok((body, created))
}

fn describe(r: &Result<(Body<f64>, topo::MevCreated), EulerOpError>) -> String {
    match r {
        Ok(_) => "certified".to_owned(),
        Err(EulerOpError::Certification {
            error: CertifyError::WindingExceeded,
        }) => "WindingExceeded".to_owned(),
        Err(EulerOpError::Certification {
            error:
                CertifyError::Escalated {
                    check: CertCheck::ParamSpan,
                    ..
                },
        }) => "Escalated(ParamSpan)".to_owned(),
        Err(e) => format!("{e:?}"),
    }
}

/// **`mev` bounds the span at certification's band**: `3π` and
/// `τ + 10.1·ε/R` refuse `WindingExceeded`, the ambiguity band
/// escalates `ParamSpan`, `τ + 0.99·ε/R` certifies. No door here
/// hands the parse a span past `τ + ε/R`.
#[test]
fn r2_mev_bounds_a_meridian_span_at_certifications_band() {
    let eps = Tol::witness().eps();
    let rows = [
        (3.0 * PI, "WindingExceeded"),
        (TAU + 10.1 * eps / RS, "WindingExceeded"),
        (TAU + 5.0 * eps / RS, "Escalated(ParamSpan)"),
        (TAU + 1.01 * eps / RS, "Escalated(ParamSpan)"),
        (TAU + 0.99 * eps / RS, "certified"),
        (TAU + 0.5 * eps / RS, "certified"),
    ];
    for (dt, want) in rows {
        let got = describe(&mev_span(RS, 0.3, dt));
        println!("R2-MEV dt-τ={:+e} rad: {got}", dt - TAU);
        assert_eq!(got, want, "dt − τ = {:e}", dt - TAU);
    }
}

/// **`set_edge_curve` re-certifies the restated span** and refuses
/// `3π` the same way; a certified edge cannot be widened past the
/// bound after the fact.
#[test]
fn r2_set_edge_curve_refuses_a_span_past_the_winding_bound() {
    let tol = Tol::witness();
    let (mut body, created) = mev_span(RS, 0.3, PI).unwrap();
    let edge = created.edge;
    let r = body.set_edge_curve(
        edge,
        EdgeCurveSpec::arc_of_circle(meridian(RS), 0.3, 0.3 + 3.0 * PI).unwrap(),
        tol,
    );
    println!("R2-SET-EDGE-CURVE 3π: {r:?}");
    assert!(
        matches!(
            r,
            Err(EulerOpError::Certification {
                error: CertifyError::WindingExceeded
            })
        ),
        "{r:?}"
    );
}

/// **A certified edge the parse escalates.** The meridian carrier is
/// stated at `R − 0.9ε` (inside `props_meridian_great`'s band) with a
/// span `τ + x`, `x` midway through `(ε/R, ε/r_c]`: `mev` certifies
/// it (headroom levered at `r_c` is inside the coincidence band) and
/// the parse escalates `props_meridian_span_winding` (the same
/// headroom levered at `R` is outside it). Before this unit the parse
/// folded such a span; after it, a body a certifying door built gets
/// an escalation from the flux lane and the branch door. A window of
/// ~`ε²/R²` radians, on a carrier a band off the sphere — recorded
/// for the PR's "same margin, band and lever" claim, not weighed.
#[test]
fn r2_a_certified_edge_at_the_carrier_lever_escalates_at_the_sphere_lever() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let band = Band::linear(tol).unwrap();
    let r_c = RS - 0.9 * eps;
    let x = 0.5 * (eps / RS + eps / r_c);
    let built = mev_span(r_c, 0.0, TAU + x);
    println!("R2-LEVER-MEV x={x:e}: {}", describe(&built));
    let (body, _) = built.expect("the edge certifies at the carrier's lever");
    let (face, _) = body.faces().next().unwrap();
    let face = body
        .faces()
        .find(|(k, _)| *k == face)
        .map(|(_, f)| f)
        .unwrap();
    let surface = body.get_surface(face.surface).unwrap();
    // The window `(ε/R, ε/r_c]` is ~ε²/R² radians; at ε = 1e-12 it is
    // below f64's resolution at τ, and the row has nothing to place.
    if (eps / r_c - eps / RS) <= 4.0 * f64::EPSILON * TAU {
        println!("R2-LEVER-MEV window below f64 resolution at eps {eps:e}; not asserted");
        return;
    }
    let (outer, _) = topo::props::loop_edges(&body, face.outer).unwrap();
    let flux = curved_face(surface, &outer, face.sense_sign(), band);
    let door = require_one_chart_branch(surface, &outer, band);
    let mp = topo::mass_properties(&body, tol);
    println!("R2-LEVER-MEV flux={flux:?}\nR2-LEVER-MEV door={door:?}\nR2-LEVER-MEV mass={mp:?}");
    let escalated = |r: &Result<(), PropsError>| matches!(r, Err(PropsError::Escalated { cause }) if cause.predicate == Some("props_meridian_span_winding"));
    assert!(
        escalated(&flux.map(|_| ())) && escalated(&door),
        "expected the parse and the door to escalate the certified span under the winding name"
    );
}
