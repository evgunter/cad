//! R1 review probes for VERBS-SHELLFIX PR-2b (#1180) — probe branch
//! only.
//!
//! Attacks, per the claims-to-falsify:
//! - the axis gate on a body that is curved but NOT axial (a bulged
//!   box: one cylinder whose side planes neither contain the axis nor
//!   sit normal to it) — must refuse typed, never build;
//! - the thin partial-revolve wedge whose inward offset has NO cavity
//!   (the two moved meridian planes cross outside the shrunk wall):
//!   every corner solves locally, so the question is what global net
//!   catches the crossed cross-section;
//! - the cone mint's MIRROR-NAPPE consumer obligation, reproduced at
//!   the mint itself (the `ConeOffset` contract), against the door's
//!   corrected sign.
//!
//! `r1p5_the_axis_gates_third_outcome_is_unreachable_from_the_sweeps`
//! was added at the FIX PASS, on this file's own subject: R2-MIN-3's
//! planted red, which measures that the gate's escalation is not
//! reachable from any operand this crate builds and pins the two
//! definite verdicts instead.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::Body;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const FIT_TOL: f64 = 1e-6;
const T: f64 = 1.0 / 128.0;

/// A box with ONE bulged side: an extruded profile whose arc mints a
/// cylinder, while the straight sides mint planes that neither contain
/// that cylinder's axis nor sit normal to it. `is_axial` must say no,
/// and the body must keep the per-chart door's own typed refusal.
fn bulged_box() -> Body<f64> {
    let lp = RawLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        // The bulge on the (w,0)->(w,d) edge: sweep < pi, transversal
        // at both junctions.
        ProfileVertex::new(p2(0.05, 0.0), 0.5),
        ProfileVertex::new(p2(0.05, 0.04), 0.0),
        ProfileVertex::new(p2(0.0, 0.04), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the bulged profile validates");
    extrude(&profile, Extrusion::Distance(0.06), Tol::witness())
        .expect("the bulged box extrudes")
        .body
}

/// A partial revolve of `angle` radians: the drum meridian, wedge cut.
fn wedge_of(angle: f64, r: f64, h: f64) -> Body<f64> {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ])],
    )
    .validate(Tol::witness())
    .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(angle),
        Tol::witness(),
    )
    .expect("the wedge revolves")
    .body
}

/// **P1 — the axis gate, attacked with a curved body that is not a
/// body of revolution.** The bulged box's cylinder is real and its
/// side planes are neither normal to that axis nor through it, so the
/// axial door must never take it; the outcome must be the per-chart
/// door's own typed refusal (its cylinder∩side-plane junctions are the
/// oblique class), never a built body with axial-door corners.
#[test]
fn r1p1_a_bulged_box_is_not_axial_and_refuses_typed() {
    let tol = Tol::witness();
    let body = bulged_box();
    match topo::shell(&body, T, FIT_TOL, band(), tol) {
        Ok(_) => panic!("a non-axial curved body must not hollow through the axial door"),
        Err(e) => println!("[r1p1] bulged box refuses: {e:?}"),
    }
    // And the door itself, asked directly, must name the shape.
    let mut charts: Vec<(topo::SurfaceKey, Vec<topo::FaceKey>)> = Vec::new();
    for (k, f) in body.faces() {
        match charts.iter_mut().find(|(s, _)| *s == f.surface) {
            Some((_, v)) => v.push(k),
            None => charts.push((f.surface, vec![k])),
        }
    }
    let moves: Vec<topo::ChartMove<f64>> = charts
        .into_iter()
        .map(|(_, faces)| topo::ChartMove {
            faces,
            distance: -T,
        })
        .collect();
    let mut work = body.clone();
    let e = topo::offset_charts_together(&mut work, &moves, band(), tol)
        .expect_err("the axial door must refuse a non-axial body");
    println!("[r1p1] offset_charts_together: {e:?}");
    assert!(
        matches!(
            e,
            topo::ReplaceFaceError::TogetherNotAxial { .. }
                | topo::ReplaceFaceError::TogetherAxialUnsupported { .. }
        ),
        "the refusal must be the axis gate's own: {e:?}"
    );
}

/// **P2 — the wedge at an angle whose inward offset has NO cavity.**
/// At `angle = 0.3`, `t = 1/128`, the two moved meridian planes cross
/// at `t / sin(angle/2) ≈ 0.052` from the axis — OUTSIDE the shrunk
/// wall radius `r − t ≈ 0.031` — so the offset cross-section is empty
/// and there is no cavity to build. Every rim corner still solves
/// locally (each meets only ONE meridian plane), so this is the global
/// shape the local meters cannot see. A typed refusal is the only
/// right answer; a returned body here is a validated wrong body.
#[test]
fn r1p2_a_sliver_wedge_with_no_cavity_must_refuse() {
    let tol = Tol::witness();
    let (r, h) = (3.0 / 64.0, 8.0 / 64.0);
    for angle in [0.3, 0.45, core::f64::consts::FRAC_PI_2] {
        let body = wedge_of(angle, r, h);
        let cross = T / (angle / 2.0).sin();
        println!(
            "[r1p2] angle {angle}: moved meridians cross at rho {cross:.6}, wall at {:.6}",
            r - T
        );
        match topo::shell(&body, T, FIT_TOL, band(), tol) {
            Ok(hollow) => {
                let props = topo::mass_properties(&hollow, tol).expect("props");
                let outer = topo::mass_properties(&body, tol).expect("props").volume;
                println!(
                    "[r1p2] angle {angle}: HOLLOWS, wall volume {} of outer {outer}",
                    props.volume
                );
                assert!(
                    cross < r - T,
                    "angle {angle}: the cavity's cross-section is EMPTY (meridians cross at \
                     {cross} > wall {}), yet a body shipped with wall volume {}",
                    r - T,
                    props.volume
                );
                assert_eq!(
                    topo::validate_geometric(&hollow, tol),
                    Ok(()),
                    "angle {angle}: tier 3"
                );
            }
            Err(e) => {
                println!("[r1p2] angle {angle}: refuses: {e:?}");
                assert!(
                    cross >= r - T,
                    "angle {angle}: a real cavity exists and must hollow, got {e:?}"
                );
            }
        }
    }
}

/// **P4 — a bare BALL: one sphere chart, two axis poles, no planes.**
/// The corpus's sphere rows always pair the sphere with caps; the pole
/// arm whose one constraint is the sphere itself (`Profile::Circle`,
/// `side_of` on the equator) is only reachable here. The wall must be
/// the difference of two balls.
#[test]
fn r1p4_a_bare_ball_hollows_to_its_closed_form() {
    let tol = Tol::witness();
    let r: f64 = 3.0 / 64.0;
    let lp: ProfileLoop<f64> = RawLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 1.0),
        ProfileVertex::new(p2(0.0, 2.0 * r), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the half-disc validates");
    let ball = revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the ball revolves")
    .body;
    match topo::shell(&ball, T, FIT_TOL, band(), tol) {
        Ok(hollow) => {
            let got = topo::mass_properties(&hollow, tol).expect("props").volume;
            let want = 4.0 / 3.0 * core::f64::consts::PI * (r.powi(3) - (r - T).powi(3));
            println!("[r1p4] ball wall volume {got} vs closed form {want}");
            assert!(
                (got - want).abs() <= 1e-15,
                "the ball's wall is the difference of two balls: got {got}, want {want}"
            );
        }
        Err(e) => println!("[r1p4] the ball REFUSES (a finding, not a failure here): {e:?}"),
    }
}

/// **P3 — the cone mint's nappe contract, reproduced at the mint.**
/// `offset_surface(cone, d)` slides the apex by `−axis·(d/sin α)`,
/// which moves the `v < 0` nappe's material `−d` along its own chart
/// normal — the ConeOffset home documents exactly this. The frustum's
/// wall is below its apex, so an inward `−t` request GROWS it at the
/// mint; the door's `nappe_signed` is what corrects the sign. Both
/// facts asserted here, so the latent-defect report stays true and the
/// correction stays load-bearing.
#[test]
fn r1p3_the_cone_mint_is_nappe_blind_and_the_door_corrects_it() {
    use geom::Surface;
    let band = band();
    // The sf2b frustum's own cone: r0 = 4/64 at y = 0, r1 = 2/64 at
    // y = h — apex ABOVE the body, wall on the negative nappe.
    let (r0, r1, h): (f64, f64, f64) = (4.0 / 64.0, 2.0 / 64.0, 8.0 / 64.0);
    let tan_a = (r0 - r1) / h;
    let alpha = tan_a.atan();
    let apex_y = r0 / tan_a;
    let cone: Surface<f64> = Surface::Cone {
        apex: geom_core::Point3::new(0.0, apex_y, 0.0),
        axis: geom_core::Vec3::new(0.0, 1.0, 0.0),
        half_angle: alpha,
        u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
    };
    let moved = geom_brep::offset_surface(&cone, -T, band).expect("the cone offsets");
    let Surface::Cone { apex, .. } = moved else {
        panic!("a cone's offset is a cone");
    };
    // The raw mint slides the apex UP for d = −t, so the radius at any
    // station below it GROWS: the mint answered the OPPOSITE nappe.
    let grown = (apex.y - 0.0) * tan_a;
    println!(
        "[r1p3] raw mint at d = -t: base radius {r0} -> {grown} (apex {apex_y} -> {})",
        apex.y
    );
    assert!(
        grown > r0,
        "the raw mint must grow the below-apex wall on an inward request: {grown} vs {r0}"
    );
    // And the corrected sign shrinks it by exactly t/cos α at the base
    // station — which is the offset frustum the closed-form row pins.
    let corrected = geom_brep::offset_surface(&cone, T, band).expect("the cone offsets");
    let Surface::Cone { apex: apex_c, .. } = corrected else {
        panic!("a cone's offset is a cone");
    };
    let shrunk = apex_c.y * tan_a;
    println!("[r1p3] corrected sign: base radius {r0} -> {shrunk}");
    assert!(
        (r0 - shrunk - T / alpha.cos()).abs() < 1e-15,
        "the corrected offset shrinks the base radius by t/cos α"
    );
}

/// **P5 (added at the fix pass, for R2-MIN-3): the axis gate's third
/// outcome, and the measurement that it is not reachable from here.**
///
/// `is_axial` returns `Result<bool, _>` now: an ambiguity-band margin
/// escalates instead of folding into `false`, because folding it would
/// turn "I cannot tell" into a silent branch choice. This row is the
/// planted red for that — and it reports, rather than asserts, the
/// escalation, because the escalation cannot be reached on any operand
/// this crate's own sweeps will build:
///
/// - every margin the gate takes is EXACTLY zero on a revolve. The
///   caps' normals, the wall's axis and the frame's direction are all
///   minted from ONE `AxisFrame`, so `n̂ × â` and `m̂ · â` are exact
///   zeros rather than small numbers — the sweep below reads `Ok(true)`
///   from `1e-30` to `1e-12`, eighteen decades, with no band anywhere;
/// - and the obvious way to perturb one — revolving about a 2-D axis
///   that is not `±x`/`±y` — does not get past `revolve` at all, which
///   this row also records rather than assumes.
///
/// So the definite verdicts are what is pinned (`Ok(true)` here,
/// `Ok(false)` on P1's non-axial body), and the day a door mints an
/// axial body from more than one frame this row is where the third
/// outcome shows up.
#[test]
fn r1p5_the_axis_gates_third_outcome_is_unreachable_from_the_sweeps() {
    let tol = Tol::witness();
    let (r, h) = (3.0 / 64.0, 8.0 / 64.0);
    let meridian = || {
        Profile::new(
            SketchPlane::xy(),
            vec![ProfileLoop::new(vec![
                ProfileVertex::new(p2(0.0, 0.0), 0.0),
                ProfileVertex::new(p2(r, 0.0), 0.0),
                ProfileVertex::new(p2(r, h), 0.0),
                ProfileVertex::new(p2(0.0, h), 0.0),
            ])],
        )
        .validate(tol)
        .expect("meridian")
    };
    let wedge = revolve(
        &meridian(),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol,
    )
    .expect("a quarter revolve")
    .body;

    let mut escalations = 0;
    for e in [1e-30, 1e-24, 1e-20, 1e-18, 1e-17, 1e-16, 1e-14, 1e-12] {
        let verdict = topo::is_axial(&wedge, Band::new(e, 10.0 * e).expect("band"));
        println!("[r1p5] wedge at eps={e:e}: {verdict:?}");
        match verdict {
            Ok(true) => {}
            Ok(false) => panic!("this wedge IS axial at every scale; got false at {e:e}"),
            Err(_) => escalations += 1,
        }
    }
    assert_eq!(
        escalations, 0,
        "if this ever fires, the third outcome became reachable and the row's own prose \
         is what needs re-deriving — not the assertion"
    );

    // And the obvious perturbation is refused by the SWEEP, not by the
    // gate: there is no constructible tilted-axis revolve to feed it.
    for ax in [Vec2::new(3.0, 4.0), Vec2::new(1.0, 3.0)] {
        assert!(
            revolve(
                &meridian(),
                RevolveAxis {
                    origin: p2(0.0, 0.0),
                    dir: ax,
                },
                Revolution::Partial(core::f64::consts::FRAC_PI_2),
                tol,
            )
            .is_err(),
            "a non-axis-aligned revolve axis is refused upstream of this gate: {ax:?}"
        );
    }
}
