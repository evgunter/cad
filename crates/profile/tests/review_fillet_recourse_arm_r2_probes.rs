//! Adversarial review probes for the fillet recourse arm (R2).
//!
//! Two jobs. First, independent fixtures — re-derived here, not reused
//! from the suite that ships with the arm — for the in-band fillet
//! verdicts a caller can author: each is asserted to render the site,
//! its own sentence and no shared coincidence recourse, and the request
//! its sentence endorses is then built and validated.
//!
//! Second, the reachability question the arm rests on. Two gates
//! recorded as having no in-band witness through the public door have
//! one:
//!
//! - **`fillet_corner_turn`** is levered: its margin is `sin(phi) * arm`
//!   with `arm = min(leg extent, carrier radius)`, so a corner at a
//!   DEFINITE turn whose leg is short lands in the band exactly as a
//!   near-tangency does — and unlike a near-tangency it is not
//!   pre-empted by `path_carrier_meet`, because the raw carriers cross
//!   at 64 degrees. Every length-shaped gate on the same request reads
//!   the arm itself, which is above the band.
//! - **`fillet_offset_lever`** compares `|rho|` against
//!   `C*u*R2*scale^2/(d*eps)`, a threshold that GROWS with the scene
//!   while `|rho|` does not have to, so a scene large enough for the
//!   run's eps reaches the gate with both clearances definite and the
//!   carriers meeting transversally.
//!
//! Both witnesses are written in units of the run's eps (the lever
//! row's scene scale is solved from eps), so they hold at every
//! tolerance row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{
    ArcSweep, Center, FILLET_ENCLOSING_RECOURSE, FILLET_NO_CORNER_RECOURSE,
    FILLET_OFFSET_LEVER_RECOURSE, FILLET_TURN_INBAND_RECOURSE, Open, PathError, Profile,
    ProfileLoop, SketchPlane, Start,
};

fn tol() -> Tol {
    Tol::witness()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn predicate_of(err: &PathError<f64>) -> Option<&'static str> {
    match err {
        PathError::Escalated { source } => source.predicate,
        _ => None,
    }
}

/// The refusal names the corner it was resolving, carries the gate's own
/// sentence, and carries no coincidence recourse.
fn carries_its_own_recourse(err: &PathError<f64>, predicate: &str, sentence: &str, what: &str) {
    assert_eq!(predicate_of(err), Some(predicate), "{what}: {err}");
    let shown = err.to_string();
    assert!(
        shown.starts_with("the fillet at this corner is undecided"),
        "{what}: the refusal must name the site: {shown}"
    );
    assert!(shown.contains(sentence), "{what}: {shown}");
    assert!(
        !shown.contains(geom_core::COINCIDENCE_RECOURSE),
        "{what}: a fillet the caller asked for declares no joint: {shown}"
    );
}

fn builds_and_validates(lp: Result<ProfileLoop<f64>, PathError<f64>>, what: &str) {
    let lp = lp.unwrap_or_else(|e| panic!("{what}: the endorsed request must build, got {e:?}"));
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap_or_else(|e| panic!("{what}: and the loop it builds must validate, got {e:?}"));
}

// ------------------------------------------------------------ fixtures

/// A line x arc corner at scale 3: the straight run drops to the origin
/// heading east and the fillet closes onto the circle of radius 3 about
/// the origin. Offset line `y = r` against offset circle `3 - r`: the
/// clearance `3 - 2r` closes at `r = 1.5`.
fn line_arc_3(radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.at(p2(0.0, 3.0))
        .line_to(p2(0.0, 0.0), tol())?
        .toward(3.0, 0.0, tol())?
        .fillet_arc(
            radius,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            tol(),
        )
        .map(|c| c.loop_)
}

/// Two same-winding lobes of radius `r_carrier`, centres `d` apart. The
/// external clearance is `2*(r_carrier - radius) - d` and the enclosing
/// gate's rho is `r_carrier - radius`.
fn lobes(r_carrier: f64, d: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let h = (r_carrier * r_carrier - 0.25 * d * d).sqrt();
    Open.arc_fillet_arc(
        Center {
            c: p2(-0.5 * d, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.0, -h),
        },
        radius,
        Center {
            c: p2(0.5 * d, 0.0),
            winding: ArcSweep::Ccw,
            p: Start,
        },
        tol(),
    )
    .map(|c| c.loop_)
}

/// A mixed-winding arc x arc corner of radius 3 with centres 3 apart:
/// the legs offset to `R + r` and `R - r`, so the internal clearance
/// `3 - 2r` is the one that closes.
fn mixed_3(radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.arc_fillet_arc(
        Center {
            c: p2(-1.5, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(1.5, 0.0),
        },
        radius,
        Center {
            c: p2(1.5, 0.0),
            winding: ArcSweep::Cw,
            p: p2(4.5, 0.0),
        },
        tol(),
    )?
    .line_to(Start, tol())
    .map(|closed| closed.loop_)
}

/// A line x arc corner built from the corner outward: the arc carrier is
/// the circle of radius `r_carrier` about the origin, the corner is
/// `(r_carrier, 0)` where its ccw tangent is `(0, 1)`, and the incoming
/// ray arrives along a direction whose perp against that tangent is
/// exactly `sin_turn` — a definite turn of `asin(sin_turn)`. The
/// straight leg is `arm` long; the arc leg sweeps `sweep_frac` of a turn
/// past the corner.
fn corner_out(
    r_carrier: f64,
    sin_turn: f64,
    arm: f64,
    sweep_frac: f64,
    radius: f64,
) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let corner = p2(r_carrier, 0.0);
    let d = p2(sin_turn, (1.0 - sin_turn * sin_turn).sqrt());
    let start = p2(corner.x - arm * d.x, corner.y - arm * d.y);
    let ang = sweep_frac * core::f64::consts::TAU;
    Open.at(start)
        .toward(d.x, d.y, tol())?
        .fillet_arc(
            radius,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(r_carrier * ang.cos(), r_carrier * ang.sin()),
            },
            tol(),
        )?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

// ---------------------------------------------------------------- rows

/// The four clearance and enclosing verdicts a caller can author, on
/// fixtures written here: each renders its own sentence at the site, and
/// the smaller radius each sentence endorses builds and validates.
#[test]
fn r2_the_reachable_clearance_verdicts_render_their_own_sentence() {
    let eps = tol().eps();
    carries_its_own_recourse(
        &line_arc_3(1.5 - 2.5 * eps).expect_err("the line x circle clearance is in band"),
        "fillet_offset_line_circle",
        FILLET_NO_CORNER_RECOURSE,
        "line x circle",
    );
    carries_its_own_recourse(
        &lobes(2.0, 2.0, 1.0 - 2.5 * eps).expect_err("the external clearance is in band"),
        "fillet_offset_circles_external",
        FILLET_NO_CORNER_RECOURSE,
        "circles external",
    );
    carries_its_own_recourse(
        &mixed_3(1.5 - 2.5 * eps).expect_err("the internal clearance is in band"),
        "fillet_offset_circles_internal",
        FILLET_NO_CORNER_RECOURSE,
        "circles internal",
    );
    carries_its_own_recourse(
        &lobes(2.0, 2.0, 2.0 + 5.0 * eps).expect_err("the enclosing gate is in band"),
        "fillet_enclosing_carrier",
        FILLET_ENCLOSING_RECOURSE,
        "enclosing carrier",
    );

    builds_and_validates(line_arc_3(0.75), "line x circle, smaller radius");
    builds_and_validates(lobes(2.0, 2.0, 0.5), "circles external, smaller radius");
    builds_and_validates(mixed_3(0.75), "circles internal, smaller radius");
    builds_and_validates(lobes(2.0, 2.0, 0.4), "enclosing, radius moved downward");
}

/// **A definite turn on a short leg escalates `fillet_corner_turn` in
/// band through the public door**, so the turn sentence is rendered to a
/// caller. The margin is `sin(phi) * arm`: at `sin(phi) = 0.9` and an
/// arm of `10.5 eps` it is `9.45 eps`, inside the band, while the arm
/// itself (`10.5 eps`) is above it, so no length-shaped gate answers
/// first and the raw carriers cross at 64 degrees rather than being
/// parallel. The sentence's third lever — move the geometry so a real
/// corner exists — is followed: the same corner with a leg the fillet
/// can sit on builds and validates.
#[test]
fn r2_a_definite_turn_on_a_short_leg_renders_the_turn_sentence() {
    let eps = tol().eps();
    let err = corner_out(3.0, 0.9, 10.5 * eps, 0.25, 0.3)
        .expect_err("the levered turn margin is in band");
    carries_its_own_recourse(
        &err,
        "fillet_corner_turn",
        FILLET_TURN_INBAND_RECOURSE,
        "a 64-degree corner on a 10.5-eps leg",
    );
    builds_and_validates(
        corner_out(3.0, 0.9, 1.0, 0.25, 0.001),
        "the same corner with a real leg extent",
    );
}

/// **The conditioning gate is reachable in band through the public
/// door** at a scene scale solved from the run's eps: its threshold is
/// `C*u*R2*scale^2/(d*eps)`, which grows with the scene, so there is a
/// scale at which it meets `|rho|` with both clearances definite. The
/// scale is solved here and then bisected against the door's own
/// verdict, so the row is not pinned to a tolerance.
#[test]
fn r2_a_scene_scaled_to_eps_renders_the_offset_lever_sentence() {
    let eps = tol().eps();
    let c_u = 128.0 * f64::EPSILON;
    let d = 10.0;
    // |rho| = R - r, clear of the external clearance's zero at d/2.
    let rho = 0.5 * d + 1e-3 * d;
    // least_lever(R) = c_u * R * (d^2 + 2 rho^2) / (d * eps); the scale
    // that puts |rho| - least_lever at 5 eps.
    let analytic = (rho - 5.0 * eps) * d * eps / (c_u * (d * d + 2.0 * rho * rho));
    let (mut lo, mut hi) = (analytic * 0.5, analytic * 2.0);
    let mut found = None;
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        let outcome = lobes(mid, d, mid - rho);
        match &outcome {
            Err(e) if predicate_of(e) == Some("fillet_offset_lever") => {
                found = Some((mid, outcome.expect_err("in band")));
                break;
            }
            // Above the solution the lever is short and the gate refuses
            // definitely; below it the corner resolves some other way.
            Err(e) if format!("{e}").contains("offset lever") => hi = mid,
            _ => lo = mid,
        }
    }
    let (r_carrier, err) = found.expect("a scene scale reaching the conditioning gate in band");
    carries_its_own_recourse(
        &err,
        "fillet_offset_lever",
        FILLET_OFFSET_LEVER_RECOURSE,
        "the conditioning gate at scene scale",
    );
    // The sentence's first lever, followed: the radius moved away from
    // the leg's carrier radius. The door takes it — but only over a
    // NARROW window, because the threshold grows as rho^2 once rho
    // dominates the scene, so the row pins both ends. (What the door
    // emits at this scale does not `validate`: the lens is so shallow
    // that `arc_span` is itself in band. The lever sentence therefore
    // has no builds-and-validates follow-through at its own site.)
    lobes(r_carrier, d, r_carrier - 1.2 * rho)
        .expect("the radius moved away from the carrier radius builds");
    let far = lobes(r_carrier, d, r_carrier - 2.0 * rho)
        .expect_err("the same lever, moved twice as far, refuses again");
    assert!(
        format!("{far}").contains("offset lever"),
        "the lever sentence's request reverses beyond a narrow window: {far}"
    );
}

/// Exploration only: which radii at the lever site build AND validate.
#[test]
#[ignore = "exploration"]
fn r2_lever_follow_window() {
    let eps = tol().eps();
    let c_u = 128.0 * f64::EPSILON;
    let d = 10.0;
    let rho = 0.5 * d + 1e-3 * d;
    let r_carrier = (rho - 5.0 * eps) * d * eps / (c_u * (d * d + 2.0 * rho * rho));
    for mult in [1.001, 1.01, 1.05, 1.1, 1.2, 1.5, 1.8, 1.95, 1.99] {
        let radius = r_carrier - mult * rho;
        let outcome = lobes(r_carrier, d, radius);
        let verdict = match &outcome {
            Ok(lp) => match Profile::new(SketchPlane::xy(), vec![lp.clone()]).validate(tol()) {
                Ok(_) => "BUILT+VALIDATES".to_string(),
                Err(e) => format!("built, validate: {e}"),
            },
            Err(e) => format!("refused: {:?}", predicate_of(e)),
        };
        eprintln!("R2 lever follow mult={mult}: {verdict}");
    }
}

/// **The arm gate stays pre-empted, and this is why.** Its margin IS the
/// lever arm, so every way of putting it in the band puts a
/// length-shaped gate on the same request in the band too: a short
/// straight leg is the ray-order window's margin, a short arc extent is
/// `path_corner_reach_arc`'s. Reported per configuration rather than
/// asserted as a universal, which a sweep cannot be.
#[test]
fn r2_the_arm_gate_is_shadowed_by_a_gate_of_the_same_magnitude() {
    let eps = tol().eps();
    let r_carrier = 3.0;
    for extent_mult in [2.0, 5.0, 9.0] {
        let extent = extent_mult * eps;
        let sweep_frac = extent / r_carrier / core::f64::consts::TAU;
        let err = corner_out(r_carrier, 0.5, 1.0, sweep_frac, 0.3)
            .expect_err("an in-band arc extent cannot resolve a corner");
        assert_ne!(
            predicate_of(&err),
            Some("fillet_corner_arm"),
            "the arm gate answered first at extent {extent:e}: {err}"
        );
        assert_eq!(
            predicate_of(&err),
            Some("path_corner_reach_arc"),
            "the shadowing gate at extent {extent:e}: {err}"
        );
    }
}
