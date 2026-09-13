//! **Every `profile` fillet recourse sentence, measured against what a
//! caller actually reads — and followed to its promised outcome.**
//!
//! The sweep twin of this file
//! (`sweep/tests/blend_recourse_followability.rs`) composes each
//! recourse with the request it endorses. Here the composition cannot
//! start the same way, and the reason is the finding:
//!
//! **None of the six `FILLET_*_RECOURSE` sentences reaches a caller.**
//! Each is written by exactly one Display arm —
//! `ProfileError::Escalated { site: EscalationSite::Fillet, .. }`,
//! dispatched on the escalation's predicate name — and nothing in the
//! kernel constructs that value. The gates themselves fire: every one
//! of the nine `fillet_*` predicate names is decided in `sugar.rs`, and
//! an in-band verdict is real and reachable. But it leaves through
//! `PathError::Escalated`, whose Display has no fillet arm at all: it
//! falls to `"path junction classification: {source}"` and appends the
//! SHARED coincidence recourse. The tailored sentence — the one that
//! names the lever this caller can actually move — is written for a
//! caller nobody becomes.
//!
//! So each row below does three things, and the middle one is the pin
//! the class asks for:
//!
//! 1. drives the user situation the constant was written for through
//!    the PUBLIC door and asserts what refuses;
//! 2. asserts the rendered refusal does NOT carry the constant — the
//!    characterization, spelled against the constant so it goes red the
//!    day a producer lands and the sentence starts reaching people;
//! 3. EXECUTES the second request the sentence names anyway, and
//!    asserts it builds and validates — so the sentence is known TRUE
//!    in advance of being reachable, which is what the dead-recourse
//!    class costs when it is discovered the other way round.
//!
//! **What step 2 watches, and what it does not.** Every row here drives
//! the PATH door, so the value it inspects is a `PathError`. A producer
//! that landed on the OTHER side — something constructing
//! `ProfileError::Escalated { site: EscalationSite::Fillet, .. }`
//! directly, which is the arm that writes these six sentences — would
//! make them reach a caller without turning any row below red. Only
//! [`the_six_sentences_render_off_the_one_display_arm_that_has_no_producer`]
//! looks at that side, and it pins the dispatch RULE, not the absence
//! of a producer. Closing the gap needs a guard over the
//! `ProfileError` surface, which is the door change
//! `work/fillet/fillet-escalation-site-has-no-producer.md` owns.
//!
//! [`the_six_sentences_render_off_the_one_display_arm_that_has_no_producer`]
//! pins the render rule itself, so a producer landing finds the wiring
//! already held.
//!
//! **The advice a caller DOES read here is inline, not a constant.**
//! Every definite fillet refusal this file provokes ends in a second
//! request written into its own Display arm — `NoCornerForFillet`'s
//! "use a smaller radius", `AnchorOutsideTrimmedExtent`'s "reduce the
//! radius or move the anchor", `CornerReason::EnclosesLegCarrier`'s "try a
//! radius below that". Those are recourses by every working definition
//! this unit uses, and the rows below execute them: it is the same
//! second request in each case, so following the dead constant and
//! following the live prose are one act here, not two. Named as a
//! class because a sweep keyed on `RECOURSE: &str` cannot see it.
//!
//! **The anchor-fit sentence's number is not a radius amount.**
//! `AnchorOutsideTrimmedExtent`'s `setback − available` is the reported
//! leg's overrun in the SETBACK metric, and the recourse it precedes is
//! un-metered: the fit row below follows "a smaller radius" with a
//! radius it chose, not one read off the payload, and is an unremarked
//! instance of the class `fillet_overrun_nearest_fit` names and the
//! grid-A recourse census (`review_fillet_overrun_nearest_fit_r1_probes`)
//! measures — the reduction a corner needs ranges from 0.04× to 29× of
//! the reported number. A metered form is the residue
//! `work/blend/anchor-fit-refusal-reports-a-setback-excess-not-a-radius-reduction.md`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{
    ArcSweep, Center, CornerReason, CornerWindow, FILLET_ENCLOSING_RECOURSE, FILLET_FIT_RECOURSE,
    FILLET_FLATTENED_RECOURSE, FILLET_LEG_EXTENT_RECOURSE, FILLET_NO_CORNER_RECOURSE,
    FILLET_OFFSET_LEVER_RECOURSE, FILLET_SCENE_RESOLUTION_RECOURSE,
    FILLET_STORED_FORM_INBAND_RECOURSE, FILLET_TURN_INBAND_RECOURSE, Open, PathError, Profile,
    ProfileLoop, SketchPlane, Start,
};

fn tol() -> Tol {
    Tol::witness()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The six sentences, with a short name for the assertion message.
const ALL: [(&str, &str); 6] = [
    ("turn-in-band", FILLET_TURN_INBAND_RECOURSE),
    ("no-corner", FILLET_NO_CORNER_RECOURSE),
    ("offset-lever", FILLET_OFFSET_LEVER_RECOURSE),
    ("enclosing", FILLET_ENCLOSING_RECOURSE),
    ("fit", FILLET_FIT_RECOURSE),
    ("leg-extent", FILLET_LEG_EXTENT_RECOURSE),
];

/// **The characterization half**: what the caller reads carries no
/// tailored fillet recourse at all — not this constant, and not one of
/// its five siblings either.
fn carries_no_fillet_recourse(err: &PathError<f64>, what: &str) {
    let shown = err.to_string();
    for (name, sentence) in ALL {
        assert!(
            !shown.contains(sentence),
            "{what}: the `{name}` recourse reached a caller — its producer has landed, \
             so this row owes a composed pin instead of a characterization.\n  got: {shown}"
        );
    }
}

/// **The followed half**: the request a sentence endorses builds, and
/// the loop it builds validates.
fn builds_and_validates(lp: Result<ProfileLoop<f64>, PathError<f64>>, what: &str) {
    let lp = lp.unwrap_or_else(|e| panic!("{what}: the endorsed request must build, got {e:?}"));
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap_or_else(|e| panic!("{what}: and the loop it builds must validate, got {e:?}"));
}

// ------------------------------------------------------------- fixtures

/// **line × arc, internal tangency.** The straight run drops to the
/// origin, the ray heads east, and the fillet closes onto the circle
/// about the origin — the derived corner is `(2, 0)`. The offset line
/// `y = r` and the offset circle of radius `2 − r` meet only while
/// `r ≤ 1`, so a larger radius has no tangent circle at all.
fn line_arc_internal(radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.at(p2(0.0, 2.0))
        .line_to(p2(0.0, 0.0), tol())?
        .toward(2.0, 0.0, tol())?
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

/// The same corner class with the STRAIGHT side short: its ray starts
/// at `(start_x, 0)`, so the leg behind the derived corner `(2, 0)` is
/// `2 − start_x` long and the tangent setback can outrun it.
fn straight_leg(start_x: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.at(p2(start_x, 0.0))
        .toward(1.0, 0.0, tol())?
        .fillet_arc(
            radius,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, 2.0),
            },
            tol(),
        )?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

/// **Two unit lobes** whose crossing is a real corner. At a radius at
/// or above the lobe radius the fillet would swallow both carriers.
fn two_lobes(radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let tip = 0.75f64.sqrt();
    Open.arc_fillet_arc(
        Center {
            c: p2(-0.5, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.0, -tip),
        },
        radius,
        Center {
            c: p2(0.5, 0.0),
            winding: ArcSweep::Ccw,
            p: Start,
        },
        tol(),
    )
    .map(|c| c.loop_)
}

/// A line × line bend: the incoming ray runs east from `(start_x, 0)`,
/// the corner sits at `(4, 0)`, and the arrival leaves it at `theta`,
/// anchored three units along. `radius` rounds the corner.
fn bend(start_x: f64, theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let anchor = p2(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin());
    Open.at(p2(start_x, 0.0))
        .angle(0.0, tol())?
        .fillet(radius, tol())?
        .at(anchor, tol())?
        .angle(theta, tol())?
        .line(1.0, tol())?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

// ------------------------------------------------------------------ rows

/// **The render rule, with no producer behind it.**
///
/// All six sentences ARE written — by one Display arm, keyed on
/// `EscalationSite::Fillet` and the escalation's predicate name. The
/// value has to be built by hand here because nothing in the kernel
/// builds one, which is the whole finding; pinning the rule is what can
/// honestly be pinned about it, and it means a producer landing finds
/// the dispatch already held.
#[test]
fn the_six_sentences_render_off_the_one_display_arm_that_has_no_producer() {
    for (predicate, sentence) in [
        ("fillet_corner_turn", FILLET_TURN_INBAND_RECOURSE),
        ("fillet_leg_reach", FILLET_NO_CORNER_RECOURSE),
        ("fillet_offset_lever", FILLET_OFFSET_LEVER_RECOURSE),
        ("fillet_enclosing_carrier", FILLET_ENCLOSING_RECOURSE),
        ("fillet_leg_fit", FILLET_FIT_RECOURSE),
        ("fillet_corner_arm", FILLET_LEG_EXTENT_RECOURSE),
    ] {
        let rendered = crate::common::fillet_escalation_rendered(predicate, tol());
        assert!(
            rendered.contains(sentence),
            "`{predicate}` must render its own sentence: {rendered}"
        );
    }
}

/// **`FILLET_NO_CORNER_RECOURSE` — "use a smaller radius".**
///
/// A radius of 1.5 on the line × arc corner admits no tangent circle at
/// all. What the caller reads is the envelope's own prose — one
/// sentence per derived corner, the reachable one saying no circle of
/// that radius is tangent to both carriers there — which carries no
/// fillet recourse; the smaller radius the sentence endorses builds and
/// validates.
///
/// The sentence's other clause — "move the legs so a circle of that
/// radius can sit in the corner" — is a re-authoring rather than a
/// second request against this corner, and is not pinned.
#[test]
fn the_no_corner_recourse_reduces_to_a_radius_that_builds() {
    let err = line_arc_internal(1.5).expect_err("no tangent circle exists at r = 1.5");
    // One corner, and the row says which: the reachable crossing.
    let listed = crate::common::corners(&err);
    assert_eq!(
        listed.len(),
        1,
        "one corner reached the construction: {err:?}"
    );
    assert!(
        matches!(listed[0].reason, CornerReason::NoTangentCircle(_)),
        "the corner-existence gate is what refuses, got {err:?}"
    );
    carries_no_fillet_recourse(&err, "no corner for a fillet");
    builds_and_validates(line_arc_internal(0.5), "the smaller radius");
}

/// **`FILLET_FIT_RECOURSE` — "use a smaller radius or longer legs".**
///
/// Both clauses name a second request, and both are executed: the
/// straight leg is 0.1 long behind the corner while `r = 0.5` sets back
/// 0.586, and either shrinking the radius or lengthening the leg
/// answers. The smaller radius is 0.05, chosen: reading the payload's
/// 0.586 − 0.1 as a radius reduction would ask for r ≈ 0.014, an
/// overshoot of the class the file header names.
#[test]
fn the_fit_recourse_is_followed_by_a_smaller_radius_and_by_longer_legs() {
    let err = straight_leg(1.9, 0.5).expect_err("the setback outruns the leg");
    assert!(
        crate::common::anchor_fit(&err).is_some(),
        "the leg-fit gate's definite arm is what refuses, got {err:?}"
    );
    carries_no_fillet_recourse(&err, "the trimmed extent");
    builds_and_validates(straight_leg(1.9, 0.05), "the smaller radius");
    builds_and_validates(straight_leg(0.0, 0.5), "the longer leg");
}

/// **`FILLET_ENCLOSING_RECOURSE` — "move the radius clearly away from
/// the leg's carrier radius, downward, and expect to go well below
/// it".**
///
/// The refusal endorses no radius it cannot vouch for: it names the
/// EXISTENCE bound (the largest circle tangent to both carriers), not
/// the class bound, precisely because a radius between them re-refuses.
/// The row follows the bound the payload carries — 99% of it — and it
/// builds.
///
/// The IN-BAND arm of the same gate is reachable too (`rho` within the
/// band, at a radius a few eps above the lobe radius) and is the site
/// whose sentence this constant is; the row pins that it reaches the
/// caller without the sentence.
#[test]
fn the_enclosing_recourse_endorses_a_bound_that_builds() {
    let err = two_lobes(1.0 + 50.0 * tol().eps()).expect_err("a radius above the lobe radius");
    let listed = crate::common::corners(&err);
    assert_eq!(
        listed.len(),
        1,
        "the lens's bracketed crossing answers alone: {err:?}"
    );
    let Some((_, _, _, largest_tangent_radius)) = crate::common::enclosing(&err) else {
        panic!("the enclosing class is what refuses, got {err:?}")
    };
    carries_no_fillet_recourse(&err, "the enclosing class");
    let bound = largest_tangent_radius.expect("both carriers swallowed, so the bound exists");
    builds_and_validates(two_lobes(0.99 * bound), "the endorsed bound");

    // The in-band sibling — the arm this constant was written for.
    let inband = two_lobes(1.0 + 5.0 * tol().eps()).expect_err("rho inside the band");
    match &inband {
        PathError::Escalated { source } => assert_eq!(
            source.predicate,
            Some("fillet_enclosing_carrier"),
            "the gate escalates by name"
        ),
        other => panic!("expected the in-band escalation, got {other:?}"),
    }
    carries_no_fillet_recourse(&inband, "the enclosing gate in band");
}

/// **`FILLET_OFFSET_LEVER_RECOURSE` — "move the fillet radius away from
/// that leg's carrier radius".**
///
/// The lever gate conditions the arc × arc offset intersection and
/// fires only where `|rho|` collapses against the corner's scale. At
/// the run's default tolerance the enclosing gate reaches the same
/// geometry first — definitely above the carrier radius, in band beside
/// it — so no request here reaches the lever gate; the tree's one
/// witness for it needs `eps < 1e-10`
/// (`review_s2::a_collapsed_offset_lever_refuses_typed_at_every_band`).
///
/// The lever the sentence names is followed anyway: a radius moved
/// clearly away from the carrier radius builds.
#[test]
fn the_offset_lever_recourse_has_no_default_tolerance_witness() {
    for m in [5.0, 50.0] {
        let err = two_lobes(1.0 + m * tol().eps()).expect_err("a radius at the carrier radius");
        assert!(
            !matches!(err, PathError::FilletOffsetLeverTooShort { .. }),
            "the lever gate reached the caller at default eps — this row owes a \
             composed pin, got {err:?}"
        );
        carries_no_fillet_recourse(&err, "a radius at the carrier radius");
    }
    builds_and_validates(
        two_lobes(0.4),
        "a radius moved clearly away from the carrier",
    );
}

/// **`FILLET_TURN_INBAND_RECOURSE` — "move the geometry so a real
/// corner exists".**
///
/// A bend of `1e-9` radians is a corner whose turn lands inside the
/// band — the turn's margin is `sin(theta)` at the corner's own lever
/// arm of 7, so a turn of one epsilon lands at `7·eps`, inside
/// `(eps, K·eps)` at every tolerance the run can be given.
///
/// It escalates — but under the PATH family's own key, not the
/// `fillet_corner_turn` this sentence dispatches on, so even a
/// `ProfileError` wrapper would not select it. What the caller reads is
/// the shared coincidence recourse.
///
/// The clause that names a second request is followed: a real turn
/// builds and validates. The sentence's other two clauses — declaring
/// the tangency (held by `declared_tangency.rs`) and the cusp the
/// kernel refuses — endorse no request against this geometry.
#[test]
fn the_turn_in_band_recourse_is_followed_by_moving_the_geometry() {
    let err = bend(0.0, tol().eps(), 0.2).expect_err("a turn inside the band");
    match &err {
        PathError::Escalated { source } => assert!(
            matches!(
                source.predicate,
                Some("path_corner_turn" | "path_junction_turn")
            ),
            "the corner turn escalates under a PATH key, not a fillet one, got {:?}",
            source.predicate
        ),
        other => panic!("expected the in-band escalation, got {other:?}"),
    }
    carries_no_fillet_recourse(&err, "a turn inside the band");
    builds_and_validates(bend(0.0, 1.0, 0.2), "a real corner");
}

/// **`FILLET_LEG_EXTENT_RECOURSE` — "give the leg a real extent".**
///
/// The collapsed-arm gate (`fillet_corner_arm`) meters the minimum leg
/// lever arm, but a leg shrunk to nothing on the incoming side puts the
/// carrier intersection behind the ray's own start, and the ray-order
/// gate answers first — the envelope's entry for that corner reads
/// `OutsideAnchors(BehindIncomingRay)`. So the arm gate is not what a
/// caller with no leg extent meets.
///
/// The sentence's request is followed regardless: a leg with a real
/// extent rounds and validates.
#[test]
fn the_leg_extent_recourse_is_followed_by_giving_the_leg_an_extent() {
    let err = bend(4.0, 1.0, 0.2).expect_err("an incoming leg with no extent");
    // A straight pair derives one corner, and it is the ray's own
    // origin — which is exactly why the ray-order gate answers.
    crate::common::assert_corners(&err, &[(4.0, 0.0)], "the collapsed corner");
    assert!(
        matches!(
            crate::common::corners(&err)[0].reason,
            CornerReason::OutsideAnchors(CornerWindow::BehindIncomingRay)
        ),
        "the ray-order gate answers before the collapsed-arm one, got {err:?}"
    );
    carries_no_fillet_recourse(&err, "a leg with no extent");
    builds_and_validates(bend(0.0, 1.0, 0.2), "a leg with a real extent");
}

// ------------------------------------------------------------------
// The three STORED-FORM sentences, which DO reach a caller
// ------------------------------------------------------------------
//
// The six above are written by a Display arm nothing constructs. These
// three are not: the path door's stored-form read produces all three,
// so the rows below can do what the six cannot — assert the caller
// reads the sentence, then follow it.
//
// They are two situations and one undecided twin, and the point of
// keeping them apart is that their levers run in OPPOSITE directions.
// A row that followed "a larger radius" against the reconstruction loss
// would watch it get worse.

/// The same bend on a scene `shift` metres from the origin, with legs
/// to match — the reconstruction loss needs magnitude, not shallowness.
fn far_bend(shift: f64, theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let anchor = p2(shift + 4.0 + 3.0 * theta.cos(), shift + 3.0 * theta.sin());
    Open.at(p2(shift, shift))
        .angle(0.0, tol())?
        .fillet(radius, tol())?
        .at(anchor, tol())?
        .angle(theta, tol())?
        .line(3.0, tol())?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

/// The turn at which the stored sagitta `r(1 − cos(θ/2))` crosses the
/// run's ε — the flattening loss's own threshold, `√(8ε/r)`.
fn flattening_turn(radius: f64) -> f64 {
    (8.0 * tol().eps() / radius).sqrt()
}

fn carries(err: &PathError<f64>, sentence: &str, what: &str) {
    let shown = err.to_string();
    assert!(
        shown.contains(sentence),
        "{what}: the caller must read this sentence.\n  got: {shown}"
    );
}

/// **The flattening recourse, followed in its own regime.** Inside the
/// window the sentence reaches the caller; both levers it names are
/// then executed AT THE SAME ε and each builds and validates.
///
/// The larger-radius lever is offered conditionally, and this row is
/// where that condition is read rather than assumed: the radius that
/// lifts the sagitta clear of the band is `≈ 8Kε/θ²`, and past
/// `ε/2^-52` the other loss takes it away again. Where the two bounds
/// cross there is no radius at all, and the row asserts the sentence
/// still leaves a lever standing — the unconditional one.
#[test]
fn the_flattened_recourse_is_followed_by_a_larger_turn_and_a_larger_radius() {
    let radius = 0.2;
    let theta = 0.5 * flattening_turn(radius);
    let err = bend(0.0, theta, radius).expect_err("a turn inside the window refuses at the door");
    assert!(
        matches!(err, PathError::FilletArcFlattenedInStorage { .. }),
        "the window's refusal is the flattening one, got {err:?}"
    );
    carries(&err, FILLET_FLATTENED_RECOURSE, "the flattening refusal");

    // Lever 1, the larger turn: unconditional, and clear of the band at
    // a multiple of the threshold the sentence's own law fixes.
    builds_and_validates(
        bend(0.0, 32.0 * flattening_turn(radius), radius),
        "a larger turn",
    );

    // Lever 2, the larger radius, at the SAME turn — offered only while
    // the scene still resolves one.
    let needed = 8.0 * tol().eps() * tol().k() / (theta * theta);
    let ceiling = tol().eps() / f64::EPSILON;
    if needed * 4.0 < ceiling {
        builds_and_validates(bend(0.0, theta, needed * 4.0), "a larger radius");
    } else {
        // The impossible regime, stated as impossible rather than
        // skipped: no radius is large enough to store the arc and small
        // enough for the scene to read its carrier, and the sentence
        // must not have promised one unconditionally.
        assert!(
            FILLET_FLATTENED_RECOURSE.contains("while the scene still resolves one")
                && FILLET_FLATTENED_RECOURSE.contains("drop the fillet"),
            "at eps = {:e} and turn {theta:e} no radius works, so the sentence must offer \
             the radius conditionally and still leave a lever standing",
            tol().eps()
        );
    }

    // Lever 3, unconditional at every ε: the sharp corner.
    let anchor = p2(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin());
    builds_and_validates(
        Open.at(p2(0.0, 0.0))
            .line_to(p2(4.0, 0.0), tol())
            .and_then(|p| p.line_to(anchor, tol()))
            .and_then(|p| p.line_to(Start, tol()))
            .map(|c| c.loop_),
        "dropping the fillet",
    );
}

/// **The reconstruction recourse, followed in its own regime — and the
/// flattening sentence's lever shown running the wrong way here.**
///
/// The stored arc IS an arc (sagitta 6e-3 m at these numbers, metres
/// above every band): what cannot be read is a clearance taken as a
/// difference of lengths ten billion metres long. Both levers the
/// sentence names shrink that magnitude; the sibling's larger radius
/// does not, which is the reason the two sentences exist.
#[test]
fn the_scene_resolution_recourse_is_followed_by_a_smaller_radius_and_a_nearer_scene() {
    // The loss fires where the coordinate magnitude's own resolution,
    // `M·2^-52`, is coarser than ε; a thousand-fold margin on that puts
    // the row well inside it at whatever ε the run committed.
    let shift = 1e3 * tol().eps() / f64::EPSILON;
    let err = far_bend(shift, 0.5, 0.2).expect_err("a far scene refuses at the door");
    let (scale, radius) = match err {
        PathError::FilletCarrierBelowSceneResolution { scale, radius, .. } => (scale, radius),
        ref other => panic!("the far scene's loss is the reconstruction one, got {other:?}"),
    };
    carries(
        &err,
        FILLET_SCENE_RESOLUTION_RECOURSE,
        "the reconstruction refusal",
    );
    assert!(
        scale >= shift,
        "the sentence names the magnitude the arithmetic passed through, got {scale:e}"
    );

    // Lever 1: the geometry nearer the origin, at the same radius.
    builds_and_validates(far_bend(0.0, 0.5, radius), "the geometry nearer the origin");

    // Lever 2: a smaller radius, on a scene where the magnitude is the
    // radius itself rather than the coordinates.
    let huge = 64.0 * tol().eps() / f64::EPSILON;
    if let Err(e @ PathError::FilletCarrierBelowSceneResolution { .. }) = bend(0.0, 1e-3, huge) {
        carries(
            &e,
            FILLET_SCENE_RESOLUTION_RECOURSE,
            "a radius the scene cannot read",
        );
        builds_and_validates(bend(0.0, 1e-3, huge / 64.0), "a smaller radius");
    }

    // And the lever the SIBLING sentence names is not one here: a
    // larger radius leaves the far scene refusing.
    assert!(
        far_bend(shift, 0.5, 0.2 * 4.0).is_err(),
        "a larger radius is the other situation's lever, and must not be read as this one's"
    );
}

/// **The undecided twin.** When the stored form's own classification
/// lands in band the run has declined to say which loss it is, so the
/// sentence names both checks and the lever that settles either. The
/// row drives it and follows that lever.
#[test]
fn the_stored_form_inband_recourse_is_followed_by_dropping_the_fillet() {
    let radius = 0.2;
    // The band's own edge: the sagitta crosses ε at √(8ε/r), and the
    // escalating window sits just above it, at √(8Kε/r).
    let mut found = None;
    for k in 1..=64 {
        let theta = flattening_turn(radius) * (1.0 + f64::from(k) * 0.05);
        if let Err(e @ PathError::Escalated { .. }) = bend(0.0, theta, radius)
            && e.to_string().contains(FILLET_STORED_FORM_INBAND_RECOURSE)
        {
            found = Some((theta, e));
            break;
        }
    }
    let (theta, err) = found.expect(
        "somewhere across the band's own width the stored form's classification is undecided",
    );
    let shown = err.to_string();
    assert!(
        shown.starts_with("reading back the fillet arc this door is about to store"),
        "the escalation names the site the door read, not a junction.\n  got: {shown}"
    );
    // The wrong site's prose is what this row watches for: a junction
    // escalation's shared sentence tells the caller to declare a
    // coincidence they never authored.
    assert!(
        !shown.contains("declare the coincidence"),
        "a joint the door minted has no declaration for the caller to add.\n  got: {shown}"
    );
    let anchor = p2(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin());
    builds_and_validates(
        Open.at(p2(0.0, 0.0))
            .line_to(p2(4.0, 0.0), tol())
            .and_then(|p| p.line_to(anchor, tol()))
            .and_then(|p| p.line_to(Start, tol()))
            .map(|c| c.loop_),
        "dropping the fillet settles it either way",
    );
}
