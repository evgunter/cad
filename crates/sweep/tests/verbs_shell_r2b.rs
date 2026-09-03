//! **R2 review probes, round 2 (VERBS-SHELLFIX PR-1, ordinal 101).**
//!
//! Round 1's four reds were all MY fixtures landing in PR-2's territory
//! (`ReanchorOffCarrier`, #1081) or on the offset door's arc-carrier
//! lane, never reaching the rim surgery at all. These are re-cut to
//! reach it: an all-right-angle stepped meridian, and polygonal holes.
//!
//! In its own binary so a re-cut costs a small link rather than a
//! 35-minute rebuild of `tests/all.rs`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, FaceKey, ShellError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}
const FIT_TOL: f64 = 1e-6;

/// **One of NINE copies of this helper across five crates (#1123).**
/// `demos/tour` is a separate workspace and an integration test cannot
/// import a binary's module, so no existing home covers them all; the
/// issue carries the list and the shared-test-support fix.
fn rings_of(body: &Body<f64>) -> usize {
    body.faces().map(|(_, f)| f.rings.len()).sum()
}

fn genus_of(body: &Body<f64>) -> i64 {
    let (v, e, f) = (
        body.vertices().count() as i64,
        body.edges().count() as i64,
        body.faces().count() as i64,
    );
    let chi = v - e + f - rings_of(body) as i64;
    assert!(chi % 2 == 0, "v - e + f - r = {chi} is ODD");
    body.shells().count() as i64 - chi / 2
}

fn plane_chart_at_y(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

fn plane_chart_at_z(body: &Body<f64>, z: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - z).abs() < 1e-12
                        && normal.x.abs() < 1e-9
                        && normal.y.abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .collect()
}

fn poly(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    )
}

fn revolved(loops: Vec<ProfileLoop<f64>>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("a valid meridian");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

fn extruded(loops: Vec<ProfileLoop<f64>>, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("a valid profile");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("it extrudes")
        .body
}

/// **Claim 3's MINT on MY OWN stepped meridian** — eight stations, all
/// right angles (an oblique step lands in #1081/PR-2's territory
/// before the rim surgery is ever reached, which round 1 measured).
#[test]
fn r2b_squared_stepped_vase_mints_one_annular_rim() {
    let tol = Tol::witness();
    let (h, t) = (0.62, 0.03);
    let body = revolved(vec![poly(&[
        (0.0, 0.0),
        (0.30, 0.0),
        (0.30, 0.20),
        (0.44, 0.20),
        (0.44, 0.40),
        (0.34, 0.40),
        (0.34, h),
        (0.0, h),
    ])]);
    let chart = plane_chart_at_y(&body, h);
    println!("[r2b] vase mouth chart: {} face(s)", chart.len());
    let cup = topo::shell_open(&body, t, &chart, FIT_TOL, tol)
        .unwrap_or_else(|e| panic!("the squared vase must open: {e}"));
    assert_eq!(topo::validate_geometric(&cup, tol), Ok(()), "tier 3");
    assert_eq!(cup.shells().count(), 1);
    assert_eq!(
        (rings_of(&cup), genus_of(&cup)),
        (1, 0),
        "one annular rim, one ring, genus 0"
    );
    assert_eq!(plane_chart_at_y(&cup, h).len(), 1, "ONE rim face");
    for delta in [1e-2, 1e-3, 2e-4] {
        mesh::tessellate(&cup, delta, tol)
            .unwrap_or_else(|e| panic!("the vase rim must mesh at {delta}: {e:?}"));
    }
    // Closed form: the solid stack less the cavity stack, the cavity
    // running to the mouth plane (the lifted mouth disc removed).
    let stack = |r: &[(f64, f64, f64)]| -> f64 {
        r.iter()
            .map(|&(rad, y0, y1)| core::f64::consts::PI * rad * rad * (y1 - y0))
            .sum()
    };
    // The cavity is the boundary offset inward by `t` — every STEP
    // plane moves by `t` along its own normal too, so the stations
    // shift as well as the radii: the outward step at y = 0.20 faces
    // DOWN and moves up to 0.23; the inward step at y = 0.40 faces UP
    // and moves down to 0.37; the base moves up to `t`; the mouth
    // plane does not move, because the mouth is open.
    let solid = stack(&[(0.30, 0.0, 0.20), (0.44, 0.20, 0.40), (0.34, 0.40, h)]);
    let cav = stack(&[
        (0.30 - t, t, 0.20 + t),
        (0.44 - t, 0.20 + t, 0.40 - t),
        (0.34 - t, 0.40 - t, h),
    ]);
    let props = topo::mass_properties(&cup, tol).expect("props");
    println!(
        "[r2b] vase V = {} vs closed form {} (delta {})",
        props.volume,
        solid - cav,
        (props.volume - (solid - cav)).abs()
    );
    assert!(
        (props.volume - (solid - cav)).abs() <= 1e-12 + props.volume_pad,
        "vase cup volume: got {} want {}",
        props.volume,
        solid - cav
    );
}

/// **Claim 2's deliberate single-hole limitation**, with POLYGONAL
/// holes so the offset door's arc lane is not what refuses.
#[test]
fn r2b_two_holed_designation_refuses_typed() {
    let tol = Tol::witness();
    let outer = poly(&[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]);
    let h1 = poly(&[(0.30, 0.30), (0.30, 0.70), (0.70, 0.70), (0.70, 0.30)]);
    let h2 = poly(&[(1.30, 0.30), (1.30, 0.70), (1.70, 0.70), (1.70, 0.30)]);
    let body = extruded(vec![outer, h1, h2], 0.6);
    let top = plane_chart_at_z(&body, 0.6);
    println!("[r2b] two-holed top chart: {} face(s)", top.len());
    let opened = topo::shell_open(&body, 0.05, &top, FIT_TOL, tol);
    match &opened {
        Err(ShellError::OpenFaceRimNotExpressible { what, .. }) => {
            println!("[r2b] typed refusal: {what}");
            assert!(
                what.contains("single hole"),
                "the refusal must name the single-hole limitation, got {what}"
            );
        }
        other => panic!("[r2b] a two-holed designation must refuse typed; got {other:?}"),
    }
}

/// **The ONE-holed control, polygonal** — the single-hole pairing on a
/// shape with no revolve seam at all.
#[test]
fn r2b_one_holed_extrusion() {
    let tol = Tol::witness();
    let outer = poly(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    let hole = poly(&[(0.35, 0.35), (0.35, 0.65), (0.65, 0.65), (0.65, 0.35)]);
    let body = extruded(vec![outer, hole], 0.6);
    let top = plane_chart_at_z(&body, 0.6);
    let opened = topo::shell_open(&body, 0.05, &top, FIT_TOL, tol);
    match &opened {
        Ok(cup) => {
            println!(
                "[r2b] one-holed cup: shells={} faces={} rings={} genus={} tier3={:?}",
                cup.shells().count(),
                cup.faces().count(),
                rings_of(cup),
                genus_of(cup),
                topo::validate_geometric(cup, tol).is_ok()
            );
            assert_eq!(topo::validate_geometric(cup, tol), Ok(()), "tier 3");
            for delta in [1e-2, 1e-3, 2e-4] {
                mesh::tessellate(cup, delta, tol).unwrap_or_else(|e| {
                    panic!("VALIDATED WRONG BODY: valid but no mesh at {delta}: {e:?}")
                });
            }
            let props = topo::mass_properties(cup, tol).expect("props");
            println!("[r2b] one-holed cup V = {}", props.volume);
        }
        Err(e) => println!("[r2b] one-holed extrusion REFUSED: {e}"),
    }
}

/// **A PARTIAL revolve, through the rim surgery it used to be stopped
/// short of.** Round 1 measured that the sealed offset refused first
/// (`ReanchorOffCarrier`, #1081), so this row could only instrument
/// that the refusal was typed and no body came back. #1081's PR-2b
/// solves those corners — a wedge's meridian caps are planes CONTAINING
/// the axis, which is the door's azimuth arm — so the sealed offset
/// succeeds and the reachable outcomes are now the RIM's.
///
/// The row stays print-shaped on purpose: what it reports per θ is the
/// operand door's own verdict as well as the rim's, because the
/// axis-touching partial revolve's `NonManifoldAxisContact` is decided
/// against the RUN's epsilon and a meridian the sweep will not build
/// never reaches the verb at all.
#[test]
fn r2b_partial_revolve_reaches_the_rim() {
    let tol = Tol::witness();
    let (r, h, t) = (0.5, 0.4, 0.05);
    for theta in [core::f64::consts::FRAC_PI_2, 2.4] {
        let profile = Profile::new(
            SketchPlane::xy(),
            vec![poly(&[(0.0, 0.0), (r, 0.0), (r, h), (0.0, h)])],
        )
        .validate(tol)
        .expect("a valid meridian");
        // The AXIS-TOUCHING partial revolve is refused
        // `NonManifoldAxisContact` at eps = 1e-12 where it builds at
        // the default: `revolve`'s axis classification is decided
        // against the RUN's epsilon. Recorded as the operand-door fact
        // it is — a meridian the sweep will not build never reaches
        // the verb this row is about — rather than expected past.
        let swept = revolve(
            &profile,
            RevolveAxis {
                origin: p2(0.0, 0.0),
                dir: Vec2::new(0.0, 1.0),
            },
            Revolution::Partial(theta),
            tol,
        );
        let Ok(swept) = swept else {
            println!("[r2b] theta={theta} does NOT revolve at this epsilon: {swept:?}");
            continue;
        };
        let body = swept.body;
        let chart = plane_chart_at_y(&body, h);
        let opened = topo::shell_open(&body, t, &chart, FIT_TOL, tol);
        match &opened {
            Ok(cup) => {
                println!(
                    "[r2b] partial theta={theta} OPENED: rings={} genus={} tier3={:?}",
                    rings_of(cup),
                    genus_of(cup),
                    topo::validate_geometric(cup, tol).is_ok()
                );
                assert_eq!(topo::validate_geometric(cup, tol), Ok(()));
                for delta in [1e-2, 1e-3, 2e-4] {
                    mesh::tessellate(cup, delta, tol).unwrap_or_else(|e| {
                        panic!("VALIDATED WRONG BODY at delta = {delta}: {e:?}")
                    });
                }
            }
            Err(e) => println!("[r2b] partial theta={theta} REFUSED typed: {e}"),
        }
    }
}
