//! **Review probes (R2) for the fillet door's stored-form check** — the
//! constructions the unit's own rows do not reach, each read at whatever
//! ε the run committed.
//!
//! The door's promise is narrow and exact: no loop it emits carries a
//! declared tangency `Profile::validate` contradicts. These rows push on
//! the edges of that promise — a joint whose stored NEIGHBOUR is what
//! loses the tangency (a leg between two fillets too short to hold its
//! own direction), a fixed turn that sits inside the window on one ε
//! row and outside it on another, the seam fillet, the fused-incoming
//! fillet, an arc × arc corner at tiny turns — and the recourse the
//! refusal names, followed at every decade it fires, against the
//! lever the spec first drafted.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use crate::common;

use common::{p2, tol};
use geom_core::k_stats::Bracket;
use geom_core::{Point2, Tol};
use profile::{
    ArcSweep, Center, Open, PathError, Profile, ProfileError, ProfileLoop, SketchPlane, Start,
};

/// The fillet radius every corner here is rounded with.
const R: f64 = 0.2;

/// `√ε`: the unit the window's turns are counted in.
fn scale() -> f64 {
    tol().eps().sqrt()
}

/// The band's escalating edge, `K·ε`.
fn band_top() -> f64 {
    tol().eps() * tol().k()
}

/// The stored arc's sagitta, `r(1 − cos(θ/2))` — the margin
/// `segment_straightness` classifies.
fn sagitta(radius: f64, theta: f64) -> f64 {
    radius * (1.0 - (theta * 0.5).cos())
}

// ------------------------------------------------------------------
// The unit's three corners, copied so a probe can vary the radius.
// ------------------------------------------------------------------

fn line_line(theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let anchor = p2(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin());
    Open.at(p2(0.0, 0.0))
        .angle(0.0, tol())?
        .fillet(radius, tol())?
        .at(anchor, tol())?
        .angle(theta, tol())?
        .line(1.0, tol())?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

fn line_arc_centre(theta: f64) -> Point2<f64> {
    p2(4.0 - 2.0 * theta.sin(), 2.0 * theta.cos())
}

fn line_arc(theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let c = line_arc_centre(theta);
    let start = c + (p2(2.0 * theta.cos(), 2.0 * theta.sin()) - p2(0.0, 0.0));
    Open.at(start)
        .line_to(p2(0.0, 0.0), tol())?
        .toward(1.0, 0.0, tol())?
        .fillet_arc(
            radius,
            Center {
                c,
                winding: ArcSweep::Ccw,
                p: Start,
            },
            tol(),
        )
        .map(|c| c.loop_)
}

fn arc_arc(theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.arc_fillet_arc(
        Center {
            c: p2(-theta, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(2.0 - theta, 0.0),
        },
        radius,
        Center {
            c: p2(theta, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(theta - 2.0, 0.0),
        },
        tol(),
    )?
    .line_to(Start, tol())
    .map(|c| c.loop_)
}

type Corner = (
    &'static str,
    fn(f64, f64) -> Result<ProfileLoop<f64>, PathError<f64>>,
);

fn corners() -> [Corner; 3] {
    [
        ("line x line", line_line),
        ("line x arc", line_arc),
        ("arc x arc", arc_arc),
    ]
}

// ------------------------------------------------------------------
// R2's own constructions.
// ------------------------------------------------------------------

/// A rotated rectangle whose two right-hand corners are both rounded
/// with `radius`, so close together that the straight leg left between
/// the two fillet arcs is `gap` long. Rotated by 0.3 rad so every
/// coordinate carries rounding, the way a real part's would.
fn two_fillets_with_a_leg_of(gap: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let phi = 0.3_f64;
    let (s, c) = phi.sin_cos();
    let rot = |x: f64, y: f64| p2(x * c - y * s, x * s + y * c);
    let height = 2.0 * radius + gap;
    // The anchor on the short side sits half way along the leg the two
    // trims leave, so both fit margins are gap / 2.
    let mid = rot(4.0, radius + gap * 0.5);
    let far = rot(3.0, height);
    Open.at(rot(0.0, 0.0))
        .angle(phi, tol())?
        .fillet(radius, tol())?
        .at(mid, tol())?
        .angle(phi + std::f64::consts::FRAC_PI_2, tol())?
        .fillet(radius, tol())?
        .at(far, tol())?
        .angle(phi + std::f64::consts::PI, tol())?
        .line(3.0, tol())?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

/// The item's bend with its corner AT THE SEAM: the loop enters at
/// (1, 0) heading east, comes back along a ray through the origin
/// turned by `theta` off the entry ray, and closes with a seam fillet
/// at the origin, whose arc becomes the closing segment and retrims
/// the entry vertex.
fn seam_fillet(theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let (s, c) = theta.sin_cos();
    Open.at(p2(1.0, 0.0))
        .angle(0.0, tol())?
        .line(3.0, tol())?
        .line_to(p2(4.0, 3.0), tol())?
        .line_to(p2(-3.0 * c, -3.0 * s), tol())?
        .angle(theta, tol())?
        .fillet(radius, tol())?
        .to(Start, tol())
        .map(|c| c.loop_)
}

/// A FUSED-incoming fillet at a small turn: the radius-2 circle about
/// the origin, swept clockwise from (0, 2), meets the westbound line
/// `y = −2cos θ` at a point where the circle's tangent is `theta` off
/// the line.
fn fused_incoming(theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let y = -2.0 * theta.cos();
    Open.arc_fillet(
        Center {
            c: p2(0.0, 0.0),
            winding: ArcSweep::Cw,
            p: p2(0.0, 2.0),
        },
        radius,
        tol(),
    )?
    .toward(-1.0, 0.0, tol())?
    .to(p2(-4.0, y), tol())?
    .line_to(p2(-4.0, 3.0), tol())?
    .line_to(p2(3.0, 3.0), tol())?
    .line_to(Start, tol())
    .map(|c| c.loop_)
}

// ------------------------------------------------------------------
// Reading an outcome.
// ------------------------------------------------------------------

/// What one door attempt and, if it built, validation made of it.
/// The payloads are read through `Debug` in the printed tables.
#[derive(Debug)]
#[allow(dead_code)]
enum Outcome {
    /// The door refused typed for the stored form: the predicate and
    /// margin it names.
    DoorStoredForm(&'static str, f64),
    /// The door escalated (in band), naming the predicate.
    DoorEscalated(String),
    /// The door refused for some other reason.
    DoorOther(String),
    /// Built, and validation accepted it.
    Validates,
    /// Built, and validation refused it FOR ITS DECLARATION — the one
    /// outcome the door promises never happens.
    Contradicted(String),
    /// Built, and validation refused it for something else.
    RefusedElsewhere(String),
}

fn outcome(built: Result<ProfileLoop<f64>, PathError<f64>>) -> Outcome {
    match built {
        Err(PathError::FilletArcCannotCarryTangency {
            predicate, margin, ..
        }) => Outcome::DoorStoredForm(predicate, margin),
        Err(PathError::Escalated { source }) => {
            Outcome::DoorEscalated(source.predicate.unwrap_or("?").to_string())
        }
        Err(e) => Outcome::DoorOther(e.to_string().chars().take(90).collect()),
        Ok(lp) => match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
            Ok(_) => Outcome::Validates,
            Err(
                e @ (ProfileError::TangencyContradicted { .. }
                | ProfileError::UndeclaredTangency { .. }),
            ) => Outcome::Contradicted(e.to_string()),
            Err(e) => Outcome::RefusedElsewhere(e.to_string().chars().take(110).collect()),
        },
    }
}

fn never_contradicted(what: &str, o: &Outcome) {
    assert!(
        !matches!(o, Outcome::Contradicted(_)),
        "{what}: the door built a declaration validation contradicts: {o:?}"
    );
}

// ------------------------------------------------------------------
// The rows
// ------------------------------------------------------------------

/// **A different loss than flattening.** Two 90° fillets with a leg
/// of `gap` between them: the fillet arcs' sagitta (0.0586 m) is above
/// every band CI gates, so the stored ARC is fine — but the stored LEG
/// is a chord of length `gap` whose direction carries the endpoints'
/// rounding divided by `gap`, and the line/arc joint's
/// `carrier_line_circle` margin is `r` times that angle. The door must
/// still never mint a contradicted declaration; what the row also
/// records is which lever the refusal's sentence names and whether it
/// helps here (it names the turn and a LARGER radius; the loss here is
/// the leg, which a larger radius shortens).
#[test]
fn a_leg_between_two_fillets_too_short_to_hold_its_direction() {
    let gaps = [
        0.0, 1e-9, 2e-9, 5e-9, 1e-8, 2e-8, 3e-8, 5e-8, 1e-7, 2e-7, 5e-7, 1e-6, 1e-5, 1e-4, 1e-2,
    ];
    let mut stored_form_hits = Vec::new();
    for gap in gaps {
        let o = outcome(two_fillets_with_a_leg_of(gap, R));
        println!("LEG eps={:e} gap={gap:e} r={R}: {o:?}", tol().eps());
        never_contradicted(&format!("gap {gap:e}"), &o);
        if let Outcome::DoorStoredForm(predicate, margin) = o {
            stored_form_hits.push((gap, predicate, margin));
        }
    }
    for (gap, predicate, margin) in stored_form_hits {
        // The sentence's levers, followed at this gap: a larger radius
        // (what it names) and a smaller one (what the spec drafted).
        let larger = outcome(two_fillets_with_a_leg_of(gap, R * 1.25));
        let smaller = outcome(two_fillets_with_a_leg_of(gap, R * 0.25));
        println!(
            "LEG-RECOURSE eps={:e} gap={gap:e}: refused on '{predicate}' at {margin:e}; \
             LARGER radius -> {larger:?}; smaller radius -> {smaller:?}",
            tol().eps()
        );
        never_contradicted("larger radius", &larger);
        never_contradicted("smaller radius", &smaller);
        assert_ne!(
            predicate, "chord_side",
            "the loss here is a line/arc joint, not a flattened fillet"
        );
    }
}

/// **A fixed turn is inside the window on one ε row and outside it on
/// another.** At `θ = 2e-3`, `r = 0.2` the sagitta is 1e-7 m: ten
/// times the band's top at the default ε and a thousand times it at
/// 1e-12, where the bend builds and validates — and a tenth of ε at
/// 1e-6, where the door refuses it typed. One geometry, three
/// verdicts, decided by the row and nothing else.
#[test]
fn a_fixed_turn_crosses_the_window_with_the_eps_row() {
    let theta = 2e-3;
    let s = sagitta(R, theta);
    let o = outcome(line_line(theta, R));
    println!(
        "FIXED eps={:e} theta={theta:e} sagitta={s:e} band_top={:e}: {o:?}",
        tol().eps(),
        band_top()
    );
    if s >= 4.0 * band_top() {
        assert!(
            matches!(o, Outcome::Validates),
            "sagitta {s:e} is clear of the band; the bend builds and validates: {o:?}"
        );
    } else if s <= tol().eps() * 0.25 {
        assert!(
            matches!(o, Outcome::DoorStoredForm("chord_side", _)),
            "sagitta {s:e} is below ε; the door refuses on the line/line arm: {o:?}"
        );
    }
    for (name, build) in corners() {
        let o = outcome(build(theta, R));
        println!("FIXED eps={:e} {name} theta={theta:e}: {o:?}", tol().eps());
        never_contradicted(name, &o);
    }
}

/// **The recourse, followed at every decade it fires.** For each turn
/// inside the window the refusal fires; the larger turn and the
/// larger radius each build and validate; and the spec's original
/// lever — a SMALLER radius — leaves the refusal in place with a
/// smaller sagitta, i.e. it moves the wrong way.
#[test]
fn the_recourse_followed_at_every_decade_and_the_spec_lever_reversed() {
    for c in [0.1, 0.3, 1.0, 2.0] {
        let theta = c * scale();
        let Outcome::DoorStoredForm(predicate, margin) = outcome(line_line(theta, R)) else {
            panic!("c = {c}: the window turn refuses typed");
        };
        // Lever 1: the larger turn.
        let turned = outcome(line_line(32.0 * scale(), R));
        assert!(
            matches!(turned, Outcome::Validates),
            "c = {c}: larger turn: {turned:?}"
        );
        // Lever 2: the larger radius, sized from the law the sentence
        // states: r(1 − cos(θ/2)) has to clear the band. The lever has
        // a ceiling the sentence does not state: the stored bulge
        // pins the carrier's radius only to a few ulps of the RADIUS,
        // so once the radius the sagitta law asks for exceeds
        // ε / (a few · ε_mach) the reconstructed carrier's clearance is
        // itself in band and the door escalates `carrier_line_circle`
        // instead. That happens for θ ≲ 3e-7 whatever ε is, i.e. at
        // ε = 1e-12's 0.1·√ε decade.
        let bigger = 4.0 * band_top() / (1.0 - (theta * 0.5).cos());
        let widened = outcome(line_line(theta, bigger));
        // The reconstruction error is a few ulps of the radius; where
        // that is a decade clear of ε on either side the verdict is
        // pinned, and in between it is recorded.
        let ulp_of_radius = bigger * f64::EPSILON;
        println!(
            "RECOURSE eps={:e} c={c} theta={theta:e}: refused '{predicate}' at {margin:e}; \
             larger radius needed = {bigger:e} m (setback {:e} m, ulp of the radius {:e}) \
             -> {widened:?}",
            tol().eps(),
            bigger * (theta * 0.5).tan(),
            ulp_of_radius
        );
        if ulp_of_radius * 64.0 < tol().eps() {
            assert!(
                matches!(widened, Outcome::Validates),
                "c = {c}: the larger radius {bigger:e} builds and validates: {widened:?}"
            );
        } else if ulp_of_radius > tol().eps() {
            assert!(
                matches!(
                    widened,
                    Outcome::DoorEscalated(_) | Outcome::DoorStoredForm(..)
                ),
                "c = {c}: a radius of {bigger:e} m cannot be reconstructed from its bulge \
                 to eps, so the door refuses it too: {widened:?}"
            );
        }
        never_contradicted("larger radius", &widened);
        // The spec's lever: a smaller radius at the same turn.
        let shrunk = outcome(line_line(theta, R * 0.25));
        println!(
            "RECOURSE eps={:e} c={c}: smaller radius -> {shrunk:?}",
            tol().eps()
        );
        assert!(
            matches!(
                shrunk,
                Outcome::DoorStoredForm(..) | Outcome::DoorEscalated(_)
            ),
            "c = {c}: a smaller radius still cannot be stored: {shrunk:?}"
        );
        assert!(sagitta(R * 0.25, theta) < sagitta(R, theta));
    }
}

/// **Arc × arc at tiny turns, three radii.** Whatever the door does
/// at each turn — refuse typed, escalate, refuse the corner, or build
/// — nothing it builds is contradicted.
#[test]
fn arc_x_arc_at_tiny_turns_is_never_contradicted() {
    for c in [0.03, 0.1, 0.3, 1.0, 2.0, 4.0, 8.0, 32.0, 1024.0] {
        for radius in [0.05, 0.2, 0.5] {
            let theta = c * scale();
            let o = outcome(arc_arc(theta, radius));
            println!("ARCARC eps={:e} c={c} r={radius}: {o:?}", tol().eps());
            never_contradicted(&format!("c = {c}, r = {radius}"), &o);
        }
    }
}

/// **The fillet at the seam.** The seam arc is the closing segment
/// and joint 0 is its outgoing joint; inside the window it must refuse
/// at the door like the interior witness, and clear of it build and
/// validate.
#[test]
fn a_seam_fillet_inside_the_window_refuses_at_the_door() {
    for c in [0.1, 0.3, 1.0, 2.0, 8.0, 32.0, 1024.0] {
        let theta = c * scale();
        let o = outcome(seam_fillet(theta, R));
        println!("SEAM eps={:e} c={c} theta={theta:e}: {o:?}", tol().eps());
        never_contradicted(&format!("seam c = {c}"), &o);
        if c <= 2.0 {
            // Inside the window the seam arc never reaches validation:
            // the stored-form refusal, its escalation, or — at the
            // smallest turns — the corner refusing the radius first.
            assert!(
                matches!(
                    o,
                    Outcome::DoorStoredForm(..) | Outcome::DoorEscalated(_) | Outcome::DoorOther(_)
                ),
                "seam c = {c}: inside the window the seam fillet refuses at the door: {o:?}"
            );
        }
        if (1.0..=2.0).contains(&c) {
            assert!(
                matches!(o, Outcome::DoorStoredForm("chord_side", _)),
                "seam c = {c}: the seam arc flattens like the interior witness: {o:?}"
            );
        }
        // Clear of the window the seam fillet validates — read at
        // `32·√ε` only, since at a turn of a radian the fixture's own
        // closing leg crosses its first one.
        if c == 32.0 {
            assert!(
                matches!(o, Outcome::Validates),
                "seam c = {c}: clear of the window the seam fillet validates: {o:?}"
            );
        }
    }
}

/// **The fused-incoming fillet at a small turn.** The incoming side is
/// an authored arc carrier; the fillet's incoming joint is arc/arc and
/// its outgoing joint arc/line.
#[test]
fn a_fused_incoming_fillet_at_small_turns_is_never_contradicted() {
    for c in [0.1, 0.3, 1.0, 2.0, 8.0, 32.0, 1024.0] {
        let theta = c * scale();
        let o = outcome(fused_incoming(theta, R));
        println!("FUSED eps={:e} c={c} theta={theta:e}: {o:?}", tol().eps());
        never_contradicted(&format!("fused c = {c}"), &o);
    }
}

/// The item's bend on a scene `scale` times larger, so a fillet of a
/// large radius has legs to sit on.
fn scaled_line_line(
    theta: f64,
    radius: f64,
    scale: f64,
) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let anchor = p2(scale * (4.0 + 3.0 * theta.cos()), scale * 3.0 * theta.sin());
    Open.at(p2(0.0, 0.0))
        .angle(0.0, tol())?
        .fillet(radius, tol())?
        .at(anchor, tol())?
        .angle(theta, tol())?
        .line(scale, tol())?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

/// **The other way a stored fillet loses its carrier: radius, not
/// flattening.** A fillet of radius `r` through `θ = 1e-3` on a
/// 20-metre scene has a sagitta of `r · 1.25e-7` — metres, for a
/// kilometre-scale radius, above every band — yet its bulge pins the
/// carrier's radius only to a few ulps OF THAT RADIUS, and at
/// ε = 1e-12 a radius past a few thousand metres reconstructs with a
/// clearance the band cannot classify. The refusal's sentence names
/// the sagitta law and a LARGER radius; for this loss the lever runs
/// the other way. The row asserts only the promise (never
/// contradicted) and records what each lever does.
#[test]
fn a_large_radius_fillet_loses_its_carrier_to_the_bulge_not_the_sagitta() {
    let theta = 1e-3;
    let mut first_hit = None;
    for radius in [1e1, 1e2, 1e3, 3e3, 1e4, 3e4, 1e5] {
        let o = outcome(scaled_line_line(theta, radius, 20.0));
        println!(
            "BIGR eps={:e} r={radius:e} sagitta={:e}: {o:?}",
            tol().eps(),
            sagitta(radius, theta)
        );
        never_contradicted(&format!("r = {radius:e}"), &o);
        if first_hit.is_none()
            && matches!(o, Outcome::DoorStoredForm(..) | Outcome::DoorEscalated(_))
        {
            first_hit = Some(radius);
        }
    }
    if let Some(radius) = first_hit {
        let larger = outcome(scaled_line_line(theta, radius * 4.0, 20.0));
        let smaller = outcome(scaled_line_line(theta, radius * 0.25, 20.0));
        println!(
            "BIGR-RECOURSE eps={:e} first refused r={radius:e}: LARGER radius -> {larger:?}; \
             smaller radius -> {smaller:?}",
            tol().eps()
        );
        never_contradicted("larger radius", &larger);
        never_contradicted("smaller radius", &smaller);
    }
}

/// **The corpus loops the head builds and validation still refuses**,
/// at the two ε rows the PR's differential did not table (one at
/// 1e-6, two at 1e-12 in R2's re-run): what refuses them, so the
/// promise's "for its declaration" boundary is read off the error.
#[test]
fn corpus_loops_built_at_the_head_that_validation_refuses_elsewhere() {
    let rows: [(Corner, f64, f64); 3] = [
        (("arc x arc", arc_arc), 0.3, 0.05),
        (("line x arc", line_arc), 128.0, 0.2),
        (("line x arc", line_arc), 32.0, 0.5),
    ];
    for ((name, build), c, radius) in rows {
        let o = outcome(build(c * scale(), radius));
        println!(
            "CORPUS eps={:e} {name} c={c} r={radius}: {o:?}",
            tol().eps()
        );
        never_contradicted(&format!("{name} c = {c} r = {radius}"), &o);
    }
}

/// **K count.** Every funnel decision one door run makes, per corner
/// kind, at three turns, as a per-predicate histogram — the same
/// measurement taken on the merge base gives the check's share.
#[test]
fn r2_k_count_per_fillet() {
    for (name, build) in corners() {
        for theta in [32.0 * scale(), 1.0, 1e-3] {
            let bracket = Bracket::open();
            let out = build(theta, R);
            let rec = bracket.finish();
            let mut hist: std::collections::BTreeMap<&'static str, usize> = Default::default();
            for v in &rec.verdicts {
                *hist.entry(v.predicate).or_default() += 1;
            }
            for e in &rec.escalations {
                *hist.entry(e.predicate()).or_default() += 1;
            }
            println!(
                "KCOUNT {name} theta={theta:e} built={} decisions={} ({} verdicts, {} escalations) {hist:?}",
                out.is_ok(),
                rec.verdicts.len() + rec.escalations.len(),
                rec.verdicts.len(),
                rec.escalations.len()
            );
        }
    }
}

/// Validation's verdict on a loop, as a `Result` a row can read.
#[allow(dead_code)]
fn validates(lp: ProfileLoop<f64>, tol: Tol) -> Result<(), ProfileError> {
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .map(|_| ())
}
