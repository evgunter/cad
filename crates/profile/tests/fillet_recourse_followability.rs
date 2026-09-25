//! **Every `profile` fillet recourse sentence, measured against what a
//! caller actually reads — and followed to its promised outcome.**
//!
//! The sweep twin of this file
//! (`sweep/tests/blend_recourse_followability.rs`) composes each
//! recourse with the request it endorses; this file does the same at
//! the profile fillet doors.
//!
//! **Every one of the six `FILLET_*_RECOURSE` sentences reaches the
//! caller the gate was written for.** All six are rendered by one arm —
//! `PathError::Escalated`'s, which asks `fillet_recourse_for` for the
//! sentence belonging to the escalation's predicate name, names the site
//! ("the fillet at this corner is undecided") and appends that sentence
//! with no coincidence tail. `fillet_recourse_for` is the crate's ONE
//! name-to-sentence map; nothing else spells it.
//!
//! So each row below does three things:
//!
//! 1. drives the user situation the sentence was written for through
//!    the PUBLIC door and asserts what refuses;
//! 2. asserts what the caller READS — the tailored sentence present, and
//!    the shared `COINCIDENCE_RECOURSE` absent, because a fillet the
//!    caller asked for has no joint they declared and nothing to declare
//!    at one;
//! 3. EXECUTES the second request the sentence names and asserts it
//!    builds and validates.
//!
//! **Which gates a caller can actually drive in band, and which not.**
//! Nine `fillet_*` predicates are decided in `sugar.rs`. **Six of them
//! take an in-band verdict from a request a caller can author** —
//! `fillet_enclosing_carrier`, the three offset clearances
//! (`fillet_offset_line_circle`, `fillet_offset_circles_external`,
//! `fillet_offset_circles_internal`), `fillet_corner_turn` and
//! `fillet_offset_lever`. Three do not, and the reasons are different
//! shapes of fact:
//!
//! - **`fillet_corner_arm` is shadowed by magnitude, not by ordering.**
//!   Its margin IS the lever arm, so every way of putting it inside the
//!   band puts a length-shaped gate on the same request inside the band
//!   too: a short straight leg is the ray-order window's own margin
//!   (`path_corner_advance`, and below the band
//!   `CornerWindow::BehindIncomingRay`), a short arc extent is
//!   `path_corner_reach_arc`'s, and a collapsed carrier radius is
//!   `path_arc_center_radius`'s. Reported per configuration by
//!   `review_fillet_recourse_arm_r1_probes::the_arm_gate_is_pre_empted_by_the_advance_gate_on_the_same_margin`
//!   and
//!   `review_fillet_recourse_arm_r2_probes::r2_the_arm_gate_is_shadowed_by_a_gate_of_the_same_magnitude`
//!   — a family of witnesses, not a proof that no configuration exists.
//! - **`fillet_leg_fit` and `fillet_leg_reach` are unreachable AT THIS
//!   SCALAR, and only at this scalar.** Both classify against the
//!   exact-order band `(f64::from_bits(1), f64::from_bits(2))`, inside
//!   which no representable f64 lies, so `sign_within` is total on every
//!   finite f64 margin. That is a statement about `f64` and nothing
//!   wider: at the interval scalar a margin is an enclosure, and
//!   `tests/interval_lane.rs` already drives `fillet_leg_fit` in band
//!   there. The two rows below are scalar-scoped by construction.
//!
//! The turn and lever gates were read as pre-empted when this suite was
//! first written, on the strength of the two-lobe and bend fixtures it
//! had; both are reachable, and the rows that drive them say by what
//! mechanism. Their sentences were corrected to be true at the sites
//! they actually fire at (the A3-2 rule), which is why
//! `FILLET_TURN_INBAND_RECOURSE` now names the leg extent and
//! `FILLET_OFFSET_LEVER_RECOURSE` now names the bound on its own window.
//!
//! [`every_fillet_predicate_has_its_own_sentence_and_never_the_shared_one`]
//! holds the render rule for all nine: it renders the door's own error
//! value for each name and reads the text off it. It is also the census
//! the map owes — it takes the gate names from the `decide("fillet_…")`
//! CALL SITES across `crates/profile/src`, not from one file, so a gate
//! added anywhere in the crate without a sentence turns it red.
//!
//! **One mouth of the public door is not covered by any of this.** A
//! guided replay (`replay_guided`) wraps an in-band `fillet_*` verdict in
//! `StructureRefusal::indeterminate`, whose Display prints the
//! `Indeterminate` whole — so there the caller reads the shared
//! coincidence recourse and not the gate's sentence.
//! `review_fillet_recourse_arm_r1_probes::a_guided_replay_still_renders_the_coincidence_recourse_for_a_fillet_gate`
//! characterizes today's text; the repair is
//! `work/blend/the-guided-replay-door-renders-the-shared-recourse-for-a-fillet-gate.md`.
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

/// **What a pre-empted gate's refusal reads like**: an earlier gate
/// answered, so the situation the caller is in is that gate's, and none
/// of the six fillet sentences belongs to it.
fn carries_no_fillet_recourse(err: &PathError<f64>, what: &str) {
    let shown = err.to_string();
    for (name, sentence) in ALL {
        assert!(
            !shown.contains(sentence),
            "{what}: the `{name}` recourse belongs to a gate that did not answer here.\n  \
             got: {shown}"
        );
    }
}

/// **What an in-band fillet verdict reads like**: the site the door was
/// resolving, the gate's own sentence, and never the shared coincidence
/// recourse — the caller authored no joint here, so there is no
/// coincidence for them to declare.
fn carries_its_own_recourse(err: &PathError<f64>, sentence: &str, what: &str) {
    let shown = err.to_string();
    assert!(
        shown.starts_with("the fillet at this corner is undecided"),
        "{what}: the refusal must name the site the door was resolving.\n  got: {shown}"
    );
    assert!(
        shown.contains(sentence),
        "{what}: the caller must read the gate's own sentence.\n  got: {shown}"
    );
    assert!(
        !shown.contains(geom_core::COINCIDENCE_RECOURSE),
        "{what}: a fillet the caller asked for has no joint they declared.\n  got: {shown}"
    );
}

/// The escalation's predicate name, or a panic naming what came instead.
fn escalating_predicate(err: &PathError<f64>, what: &str) -> &'static str {
    match err {
        PathError::Escalated { source } => source
            .predicate
            .unwrap_or_else(|| panic!("{what}: the escalation must name its predicate")),
        other => panic!("{what}: expected an in-band escalation, got {other:?}"),
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

/// **The mixed-winding arc × arc corner.** The legs turn opposite ways,
/// so the two offset carriers go to `R + r` and `R − r` and the INTERNAL
/// clearance `d − |ρ₁ − ρ₂| = 2 − 2r` is the one that closes.
fn mixed_corner(radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.arc_fillet_arc(
        Center {
            c: p2(-1.0, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(1.0, 0.0),
        },
        radius,
        Center {
            c: p2(1.0, 0.0),
            winding: ArcSweep::Cw,
            p: p2(3.0, 0.0),
        },
        tol(),
    )?
    .line_to(Start, tol())
    .map(|closed| closed.loop_)
}

/// **A short straight leg meeting a radius-2 circle at a real angle.**
/// The ray `y = 2 − delta` heads east from `a` behind its first crossing
/// with the circle about the origin; at that crossing the leg and the
/// circle's tangent meet at `sin(phi) = sqrt(delta)`, and the fillet's
/// lever arm is the straight leg's extent `a`, so the turn gate's
/// levered margin is `a * sqrt(delta)`. Both factors are the caller's,
/// which is the whole point: the margin can sit in the band with the
/// angle nowhere near degenerate.
fn short_leg_at_angle(a: f64, delta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let y = 2.0 - delta;
    let cx = -(4.0 - y * y).sqrt();
    Open.at(p2(cx - a, y))
        .toward(1.0, 0.0, tol())?
        .fillet_arc(
            radius,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -2.0),
            },
            tol(),
        )?
        .line_to(p2(-3.0, -3.0), tol())?
        .line_to(p2(-3.0, 3.0), tol())?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

/// **The lever lens.** Two carriers of radius `big`, mixed winding, so
/// the offsets are `big + r` and `big − r = rho2`; the centres sit
/// `d = big + r` apart, which puts the small offset circle's centre ON
/// the large one. Both clearances then equal `rho2` and are definite,
/// and the enclosing gate is definite too, so the conditioning margin
/// `rho2 − least_lever` is the only thing that can land in the band.
fn lever_lens(big: f64, rho2: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let r = big - rho2;
    let d = big + r;
    Open.arc_fillet_arc(
        Center {
            c: p2(0.0, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.0, -big),
        },
        r,
        Center {
            c: p2(d, 0.0),
            winding: ArcSweep::Cw,
            p: p2(d, big),
        },
        tol(),
    )?
    .line_to(p2(d, 2.0 * big), tol())?
    .line_to(p2(-1.5 * big, 2.0 * big), tol())?
    .line_to(p2(-1.5 * big, -1.5 * big), tol())?
    .line_to(Start, tol())
    .map(|c| c.loop_)
}

/// The conditioning gate's own threshold at the lens, computed from
/// `sugar::ArcCarrier::offset_circles`' law with its shipped constant.
/// A fixed point, because the threshold's scene scale depends weakly on
/// the `rho2` being solved for.
fn lever_lens_least_lever(big: f64, eps: f64) -> f64 {
    let law = |rho2: f64| {
        let r = big - rho2;
        let d = big + r;
        let rho1 = big + r;
        let scale2 = d * d + rho1 * rho1 + rho2 * rho2;
        128.0 * f64::EPSILON * big * scale2 / (d * eps)
    };
    let mut l = law(0.0);
    for _ in 0..4 {
        l = law(l);
    }
    l
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

/// **The census: every `fillet_*` gate the CRATE decides has a sentence
/// of its own, and none of them renders the shared one.**
///
/// The names are read off the `decide("fillet_…")` call sites across
/// every `.rs` under `crates/profile/src`, not out of one file, so a
/// gate added in `path/arc_fillet.rs` or anywhere else in the crate is
/// as visible to this row as one added in `sugar.rs` — a `sugar.rs`-only
/// scan would have missed it while the crate README promised flatly that
/// it could not. For each name, the door's own error value is rendered
/// and the text read off it: the site, the gate's sentence from the one
/// map, and no coincidence recourse.
///
/// The count is asserted as well as the mapping, so a gate that
/// DISAPPEARS is a red row too: this census is an equality with the
/// tree, not a lower bound on it.
#[test]
fn every_fillet_predicate_has_its_own_sentence_and_never_the_shared_one() {
    let decided = decided_fillet_predicates();
    assert_eq!(
        decided.len(),
        9,
        "the fillet gate family is nine names; found {decided:?}"
    );
    for predicate in decided {
        let sentence = profile::fillet_recourse_for(predicate).unwrap_or_else(|| {
            panic!("`{predicate}` is decided in sugar.rs and owes a recourse sentence")
        });
        let err = PathError::<f64>::Escalated {
            source: geom_core::Indeterminate {
                margin: geom_core::MarginDiag::Value(-5.0 * tol().eps()),
                band: geom_core::Band::linear(tol()).expect("the run's band forms"),
                predicate: Some(predicate),
            },
        };
        carries_its_own_recourse(&err, sentence, predicate);
        assert!(
            err.to_string().contains(predicate),
            "the refusal names the gate that could not be classified: {err}"
        );
    }
}

/// **The pairing, checked against a prose line the map does not read.**
///
/// The census above proves every gate HAS a sentence; it cannot prove
/// the gate has the RIGHT one, because it asks the map for the sentence
/// it then looks for. The driven rows prove the pairing wherever a gate
/// can be reached — but `fillet_leg_fit` and `fillet_leg_reach` cannot
/// be reached at this scalar, so swapping their two sentences reddens
/// nothing that executes them.
///
/// This row closes that hole from prose. Each sentence's doc comment in
/// `validate.rs` declares the gates it serves on a `/// Gates:` line —
/// written for a reader, not consumed by the code — and the row asserts
/// that two gates share a sentence in the MAP exactly when they are
/// declared under one sentence in the DOCS. A swap inside a group is
/// invisible to that (turn ↔ arm are both singletons), which is the
/// division of labour: the driven rows catch those, and this catches a
/// name moved between groups, which is the only swap the unreachable
/// pair admits.
///
/// This is not an independent source: the `Gates:` lines live in the
/// same file as the map, so an edit that moves a name in BOTH stays
/// green (measured at the fix-pass head). It guards a lone edit to
/// either side, which is the only guard a pair no row can execute
/// admits at this scalar.
#[test]
fn the_map_groups_the_gates_the_way_the_sentences_docs_say_it_does() {
    let text = std::fs::read_to_string(
        test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src/validate.rs"),
    )
    .expect("the validation module is readable");
    // The `Gates:` lines live in doc comments, so the PROSE view is what
    // holds them — a code view blanks exactly this.
    let prose = test_utils::source::comments_only(&text);
    let mut declared: Vec<Vec<String>> = Vec::new();
    for line in prose.lines() {
        let Some(rest) = line.trim_start().strip_prefix("/// Gates:") else {
            continue;
        };
        let group: Vec<String> = rest
            .split('`')
            .filter(|piece| piece.starts_with("fillet_"))
            .map(str::to_string)
            .collect();
        assert!(!group.is_empty(), "a `Gates:` line names no gate: {line}");
        declared.push(group);
    }
    assert_eq!(
        declared.len(),
        6,
        "six sentences declare their gates; found {declared:?}"
    );
    let flat: Vec<&String> = declared.iter().flatten().collect();
    let mut seen = flat.clone();
    seen.sort();
    let before = seen.len();
    seen.dedup();
    assert_eq!(before, seen.len(), "a gate is declared twice: {flat:?}");
    assert_eq!(
        seen.len(),
        decided_fillet_predicates().len(),
        "the declarations and the decided gates are not the same set"
    );

    for a in &flat {
        for b in &flat {
            let same_group = declared.iter().any(|g| g.contains(a) && g.contains(b));
            let same_sentence = profile::fillet_recourse_for(a)
                .expect("a declared gate is in the map")
                == profile::fillet_recourse_for(b).expect("a declared gate is in the map");
            assert_eq!(
                same_group,
                same_sentence,
                "`{a}` and `{b}`: the docs group them {}, the map groups them {}",
                if same_group { "together" } else { "apart" },
                if same_sentence { "together" } else { "apart" }
            );
        }
    }
}

/// Every `fillet_*` name the crate hands to a `decide` door, sorted.
///
/// **Call sites, not string literals**, and the walk is
/// `test_utils::source::predicate_census` — the tree's one home for it.
/// This census wrote its own copy of that walk before the shared one
/// existed, and a hand-rolled copy is exactly what two reviewers
/// defeated by mutation on the roster suites next door: what the shared
/// reader cannot read it REPORTS, and the roster rows red on each kind
/// of report, so this row inherits a walk that is checked rather than
/// one it would have to check itself.
///
/// The filter is the family: `validate.rs`'s own map is not a call site
/// (its arms are match patterns) and `path/program.rs`'s `"fillet_arc"`
/// step name never reaches the funnel, so neither is here — without
/// that a crate-wide scan would read the map it is supposed to check
/// and pass by construction.
///
/// The names are leaked so they carry the `'static` lifetime an
/// `Indeterminate`'s predicate needs; this is one test binary and each
/// source file is read once.
fn decided_fillet_predicates() -> Vec<&'static str> {
    let src = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    // The carriers the roster suite declares: a `fillet_*` name handed
    // to the funnel through one of them is as much a gate as one
    // written at the call.
    use test_utils::source::NameCarrier::Call;
    let carriers = [Call("travel"), Call("gate_positive"), Call("coincident")];
    test_utils::source::predicate_census(&src, &carriers)
        .names
        .into_iter()
        .filter(|name| name.starts_with("fillet_"))
        .map(|name| -> &'static str { Box::leak(name.into_boxed_str()) })
        .collect()
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

/// **`FILLET_NO_CORNER_RECOURSE` at the gate it was written for — the
/// offset-carrier clearances, in band, read off the caller's own
/// refusal.**
///
/// The same corner-existence question the definite refusal above answers
/// with a `CornerReason`, asked where the answer is undecidable. Two of
/// the three clearance gates take an in-band verdict from a request a
/// caller can author, and each is driven here at a margin fixed in units
/// of the run's ε, so the row holds at every tolerance the run can be
/// given:
///
/// - the **line × circle** clearance, at the radius where the offset
///   line and the offset circle are tangent: the straight run's offset
///   is `y = r` and the circle's is `2 − r`, so they touch at `r = 1`
///   and the margin `|ρ| − |h|` is `2·(1 − r)`;
/// - the **circle × circle** external clearance on the two unit lobes,
///   where `|ρ₁| + |ρ₂| − d` is `2·(0.5 − r)`;
/// - the **circle × circle** internal clearance on the mixed-winding
///   corner, whose legs offset to `R + r` and `R − r`, so
///   `d − |ρ₁ − ρ₂|` is `2 − 2r`.
///
/// All three render the site and `FILLET_NO_CORNER_RECOURSE`, and none
/// renders the shared coincidence recourse. The sentence's request — a
/// smaller radius — then builds and validates at each corner.
#[test]
fn the_offset_clearance_recourse_reaches_the_caller_and_reduces() {
    let eps = tol().eps();

    let line_circle =
        line_arc_internal(1.0 - 2.5 * eps).expect_err("the offset carriers' clearance is in band");
    assert_eq!(
        escalating_predicate(&line_circle, "the line x circle clearance"),
        "fillet_offset_line_circle"
    );
    carries_its_own_recourse(
        &line_circle,
        FILLET_NO_CORNER_RECOURSE,
        "the line x circle clearance",
    );
    builds_and_validates(line_arc_internal(0.5), "the smaller radius");

    let circles =
        two_lobes(0.5 - 2.5 * eps).expect_err("the offset carriers' clearance is in band");
    assert_eq!(
        escalating_predicate(&circles, "the circle x circle clearance"),
        "fillet_offset_circles_external"
    );
    carries_its_own_recourse(
        &circles,
        FILLET_NO_CORNER_RECOURSE,
        "the circle x circle clearance",
    );
    builds_and_validates(two_lobes(0.4), "the smaller radius");

    let mixed = mixed_corner(1.0 - 2.5 * eps).expect_err("the internal clearance is in band");
    assert_eq!(
        escalating_predicate(&mixed, "the mixed corner's internal clearance"),
        "fillet_offset_circles_internal"
    );
    carries_its_own_recourse(
        &mixed,
        FILLET_NO_CORNER_RECOURSE,
        "the mixed corner's internal clearance",
    );
    builds_and_validates(mixed_corner(0.5), "the smaller radius");
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

    // The in-band sibling — the arm this constant was written for, and
    // the one a caller reads it from.
    let inband = two_lobes(1.0 + 5.0 * tol().eps()).expect_err("rho inside the band");
    assert_eq!(
        escalating_predicate(&inband, "the enclosing gate in band"),
        "fillet_enclosing_carrier"
    );
    carries_its_own_recourse(
        &inband,
        FILLET_ENCLOSING_RECOURSE,
        "the enclosing gate in band",
    );
    // The lever the sentence names, followed from the in-band site: a
    // radius moved clearly below the leg's carrier radius builds.
    builds_and_validates(two_lobes(0.4), "the radius moved clearly downward");
}

/// **`FILLET_OFFSET_LEVER_RECOURSE` reaches the caller at its own site,
/// and the sentence is measured there.**
///
/// The conditioning gate compares `|rho|` against
/// `C*u*R2*scale^2/(d*eps)`, so it is reached by making the SCENE large
/// rather than by making the radius small: the threshold grows with the
/// corner's magnitude while `|rho|` does not. The fixture is a
/// mixed-winding lens whose two carriers of radius `big` sit
/// `d = big + r` apart — which puts the small offset circle's centre on
/// the large one, so both clearances are definite at `|rho2|` and the
/// enclosing gate is definite too, leaving the conditioning margin as
/// the only thing in the band. `big` is a fixed multiple of the run's
/// eps, so the least lever (proportional to `R^2/eps`) stays a fixed
/// fraction of the scene and the row holds at every tolerance.
///
/// The sentence's request is then followed AT THIS SITE: the radius
/// moved clearly away from the carrier radius builds and validates. The
/// bound the sentence now names is real and is pinned by
/// `review_fillet_recourse_arm_r2_probes::r2_a_scene_scaled_to_eps_renders_the_offset_lever_sentence`,
/// where a scene scaled until `rho` dominates it refuses the same lever
/// again past a narrow window — which is why the sentence says the
/// direction AND the bound rather than promising the direction alone.
#[test]
fn the_offset_lever_recourse_reaches_the_caller_at_its_own_site() {
    let eps = tol().eps();
    let big = 3e10 * eps;
    let least = lever_lens_least_lever(big, eps);
    for k in [-5.0, 5.0] {
        let err = lever_lens(big, least + k * eps).expect_err("the lever margin is in band");
        assert_eq!(
            escalating_predicate(&err, "the lever lens"),
            "fillet_offset_lever"
        );
        carries_its_own_recourse(&err, FILLET_OFFSET_LEVER_RECOURSE, "the lever lens");
    }
    // The definite sibling below the band, carrying the same story.
    let short = lever_lens(big, least - 50.0 * eps).expect_err("below the least lever");
    assert!(
        matches!(short, PathError::FilletOffsetLeverTooShort { .. }),
        "expected the definite sibling, got {short:?}"
    );
    // The sentence's own request, followed at the site it fired at.
    builds_and_validates(
        lever_lens(big, big / 10.0),
        "the radius moved clearly away from the carrier radius",
    );
}
/// **`FILLET_TURN_INBAND_RECOURSE` names both of the situations its
/// band admits, and both are driven here.**
///
/// The gate's margin is LEVERED — `sin(angle between the legs) * the
/// shorter leg's extent` — so a margin in the band means one of two
/// unrelated things, and the sentence has to be true of each:
///
/// 1. **the angle is degenerate.** A bend of one epsilon at a lever arm
///    of 7 lands at `7*eps`. It escalates under the PATH family's own
///    key (`path_corner_turn`) rather than the fillet one, so what a
///    caller reads at this site is the shared coincidence recourse and
///    not this sentence — recorded, not asserted away.
/// 2. **the angle is real and the LEG is short.** A straight leg of
///    `500*eps` meeting a radius-2 circle at `asin(0.01)` puts
///    `sin(phi) * arm` at `5*eps` while the arm itself is far above the
///    band and the raw carriers cross transversally, so no length-shaped
///    gate and no `path_carrier_meet` verdict answers first. The fillet
///    gate escalates by name and the caller reads this sentence.
///
/// At (2) the corner is not degenerate at all, which is what the
/// sentence used to assert: the identical carriers with a leg long
/// enough to take the setback build and validate. That is the clause the
/// sentence now names — give the shorter leg a longer extent — and it is
/// executed here. The reviewer rows
/// `review_fillet_recourse_arm_r1_probes::the_turn_gate_is_reachable_in_band_with_a_short_leg_and_a_real_angle`
/// and
/// `review_fillet_recourse_arm_r2_probes::r2_a_definite_turn_on_a_short_leg_renders_the_turn_sentence`
/// pin the same mechanism at a 0.57-degree and a 64-degree corner.
#[test]
fn the_turn_in_band_recourse_names_both_of_its_situations() {
    let eps = tol().eps();

    // (1) the degenerate angle: a PATH key answers, so this sentence is
    // not what the caller reads there.
    let degenerate = bend(0.0, eps, 0.2).expect_err("a turn inside the band");
    match &degenerate {
        PathError::Escalated { source } => assert!(
            matches!(
                source.predicate,
                Some("path_corner_turn" | "path_junction_turn")
            ),
            "the degenerate turn escalates under a PATH key, not a fillet one, got {:?}",
            source.predicate
        ),
        other => panic!("expected the in-band escalation, got {other:?}"),
    }
    carries_no_fillet_recourse(&degenerate, "a degenerate turn inside the band");
    builds_and_validates(bend(0.0, 1.0, 0.2), "a real corner");

    // (2) the real angle on a short leg: the fillet gate answers and the
    // caller reads this sentence.
    let short = short_leg_at_angle(500.0 * eps, 1e-4, 0.05)
        .expect_err("the levered turn margin is in band");
    assert_eq!(
        escalating_predicate(&short, "a real angle on a short leg"),
        "fillet_corner_turn"
    );
    carries_its_own_recourse(
        &short,
        FILLET_TURN_INBAND_RECOURSE,
        "a real angle on a short leg",
    );
    // The lever the sentence names at THIS site, followed: the identical
    // carriers with a leg long enough to take the setback.
    builds_and_validates(
        short_leg_at_angle(1.0, 1e-4, 0.05),
        "the same corner with a longer leg",
    );
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
// The three STORED-FORM sentences
// ------------------------------------------------------------------
//
// These are the path door's stored-form read's own, not the
// construction gates': the door re-runs validation's segment and joint
// predicates on the loop it is about to emit, so their situation is a
// joint the door itself minted rather than a corner the caller
// authored, and their arm is BLEND-10's, one screen below the fillet
// one in `path.rs`.
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
        shown.starts_with("the fillet arc about to be stored is undecided"),
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
