//! **Review probes for MESH-12 (R1): can any public door hand the
//! sphere parse a span past the winding bound?** (issue 1601, PR
//! 1617.)
//!
//! The PR's measurement 1 says no: every certifying door runs
//! certification's check 2, so a stored span reaching `props` is
//! within `τ + zero/R`, and the new parse decide can only ever refuse
//! a hand-built loop. That is a claim about a set of doors, and the
//! way to check it is to walk up to each one with the span it says it
//! refuses. These rows do that for the Euler door, the attach door
//! and the split door, and then feed a body each one DID mint to the
//! parse.
//!
//! Every offset is derived from the run's own `Band`; no ε literal.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_brep::props::{PropsError, curved_face, require_one_chart_branch};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{Body, FaceSurface, MevSite};

const TAU: f64 = core::f64::consts::TAU;
/// The sphere and its meridian carriers: R = 10 mm about +Z.
const RS: f64 = 0.010;

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}
fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: RS,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}
/// The meridian great circle in the x–z plane.
fn meridian() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, -1.0, 0.0),
        radius: RS,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// A body carrying one sphere face and one meridian edge of the given
/// span, minted through the Euler door. `Err` is the door's own
/// refusal, printed by the caller.
fn mev_meridian(dt: f64) -> Result<Body<f64>, String> {
    let tol = Tol::witness();
    let (t0, t1) = (0.3, 0.3 + dt);
    let carrier = meridian();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(carrier.eval(t0)).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(sphere()))
        .unwrap();
    body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        carrier.eval(t1),
        EdgeCurveSpec::arc_of_circle(carrier, t0, t1).unwrap(),
        tol,
    )
    .map_err(|e| format!("{e:?}"))?;
    Ok(body)
}

/// **The Euler door refuses every span the parse would.** Walks the
/// same ladder the parse is decided on: the coincidence band above τ
/// is minted, the ambiguity band escalates, past it the door refuses
/// — so the only spans `mev` can put in front of `props` are ones the
/// parse admits.
#[test]
fn r1_the_euler_door_mints_no_span_the_parse_would_refuse() {
    let bd = Band::linear(Tol::witness()).unwrap();
    let z = bd.zero() / RS;
    for k in [0.0, 0.5, 0.99] {
        let r = mev_meridian(TAU + k * z);
        println!(
            "R1-DOOR mev k={k} -> {}",
            if r.is_ok() { "minted" } else { "refused" }
        );
        assert!(
            r.is_ok(),
            "k = {k}: the door refuses a span certification admits"
        );
    }
    for k in [1.01, 9.9, 10.1, 40.0, 1.0e6] {
        let r = mev_meridian(TAU + k * z);
        let e = r
            .err()
            .unwrap_or_else(|| panic!("k = {k}: mev MINTED a span past the bound"));
        println!("R1-DOOR mev k={k} -> {e}");
        assert!(
            e.contains("ParamSpan") || e.contains("WindingExceeded"),
            "k = {k}: refused, but not by the span check: {e}"
        );
    }
    // The 3π span CERT-1's row used, at the door that would have to
    // mint it for the refused shape to be reachable from a body.
    let three_pi = mev_meridian(3.0 * core::f64::consts::PI);
    println!(
        "R1-DOOR mev 3pi -> {three_pi:?}",
        three_pi = three_pi.as_ref().err()
    );
    assert!(three_pi.is_err());
}

/// **The attach door and the split door are the same gate.**
/// `set_edge_curve` re-states an edge's interval and `split_edge`
/// re-mints two pieces from one; neither can widen a span past the
/// bound. The attach door is walked with a restated interval a whole
/// period longer than the one it replaces.
#[test]
fn r1_the_attach_and_split_doors_do_not_widen_a_span() {
    let tol = Tol::witness();
    let body = mev_meridian(1.0).unwrap();
    let edge = body.edges().next().map(|(k, _)| k).unwrap();

    let mut attach = body.clone();
    let restated = EdgeCurveSpec::arc_of_circle(meridian(), 0.3, 0.3 + 1.0 + TAU).unwrap();
    let r = attach.set_edge_curve(edge, restated, tol);
    println!("R1-DOOR set_edge_curve (+one period) -> {r:?}");
    let e = format!(
        "{:?}",
        r.expect_err("the attach door widened a span past the bound")
    );
    assert!(
        e.contains("ParamSpan") || e.contains("WindingExceeded"),
        "{e}"
    );

    let mut split = body.clone();
    let r = split.split_edge(edge, 0.3 + 0.5, tol);
    println!(
        "R1-DOOR split_edge -> {}",
        if r.is_ok() { "split" } else { "refused" }
    );
    if r.is_ok() {
        for (_, e) in split.edges() {
            let c = split.get_curve_geom(e.curve).unwrap().certified().unwrap();
            let (t0, t1) = c.params();
            println!("R1-DOOR piece span = {:e}", t1 - t0);
            assert!(
                t1 - t0 <= TAU + 1e-9 / RS,
                "a split piece is past the bound"
            );
        }
    }
}

/// **What the doors DO mint reaches the parse and is answered.** The
/// body from the coincidence-band rung is handed to the flux lane and
/// the branch door through `loop_edges`, the same route
/// `mass_properties` takes: neither reports the new name, so no
/// certified body is refused by this unit's decide.
#[test]
fn r1_no_body_a_door_minted_is_refused_by_the_new_decide() {
    let bd = Band::linear(Tol::witness()).unwrap();
    let z = bd.zero() / RS;
    for k in [0.0, 0.5, 0.99] {
        let body = mev_meridian(TAU + k * z).unwrap();
        for (_, face) in body.faces() {
            let surface = body.get_surface(face.surface).unwrap();
            let (outer, _) = topo::props::loop_edges(&body, face.outer).unwrap();
            let flux = curved_face(surface, &outer, face.sense_sign(), bd);
            let door = require_one_chart_branch(surface, &outer, bd);
            println!("R1-CERTIFIED k={k} flux={flux:?} door={door:?}");
            for r in [format!("{flux:?}"), format!("{door:?}")] {
                assert!(
                    !r.contains("props_meridian_span_winding"),
                    "a certified body was refused by the parse's new decide: {r}"
                );
            }
        }
    }
}

/// **The import door's own normalisation, executed.** The PR says
/// `step_import`'s `endpoint_params` mints `t1 ∈ (t0, t0 + τ]`. That
/// is a claim about a different crate, so this row only pins the half
/// this crate can see: a span exactly at the period — the value that
/// normalisation is allowed to produce — is admitted by the decide,
/// on the same shape the parse reads.
#[test]
fn r1_a_span_of_exactly_one_period_is_admitted() {
    let bd = Band::linear(Tol::witness()).unwrap();
    let body = mev_meridian(TAU).unwrap();
    for (_, face) in body.faces() {
        let surface = body.get_surface(face.surface).unwrap();
        let (outer, _) = topo::props::loop_edges(&body, face.outer).unwrap();
        let door = require_one_chart_branch(surface, &outer, bd);
        println!("R1-PERIOD door={door:?}");
        assert!(!matches!(
            door,
            Err(PropsError::NotIsoRectangle {
                what: "props_meridian_span_winding"
            })
        ));
    }
}
