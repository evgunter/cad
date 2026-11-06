//! **Every gate this suite can drive in band ends as its behaviour says
//! it must, and the tighten it offers is one that works.**
//!
//! A `Gate`'s kind is pinned by what its refusal DOES, not by restating
//! the table. Each driven gate is put in band through a public door, at
//! geometry fixed at the run's ε (`ε₀`), and its refusal is read:
//!
//! - it names this gate, and renders this gate's own lever, written here
//!   as the sentence it must be (so a lever swapped between two gates in
//!   the table reds);
//! - a gate whose margin a smaller ε decides for the request offers the
//!   conditional tighten, and the row FOLLOWS it: the same request, at
//!   the same geometry, is run again in a child process at a tolerance
//!   below the stated value, and this gate must no longer escalate there
//!   (whatever the request does next);
//! - every other gate offers no tighten at all.
//!
//! The gates no door here can drive in band are listed in
//! [`NOT_DRIVEN`], each with its reason, and the two lists together are
//! held equal to `Gate::ALL`.
//!
//! The doc pairing beside it ([`each_fillet_lever_serves_the_gates_its_docs_name`])
//! holds the fillet levers no door drives: each sentence's doc in
//! `src/fillet_recourse.rs` names its gates on a `Gates:` line, and two
//! gates share a lever in the table exactly when they share one there.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::f64::consts::FRAC_PI_2;

use geom_core::{Point2, Tol};
use profile::{
    ArcSweep, Center, EscalatedAt, FILLET_ENCLOSING_RECOURSE, FILLET_NO_CORNER_RECOURSE,
    FILLET_OFFSET_LEVER_RECOURSE, FILLET_STORED_FORM_INBAND_RECOURSE, FILLET_TURN_INBAND_RECOURSE,
    Gate, Open, PathError, ProfileError, Start, Step, StructureRefusal, StructureRefusalKind,
    replay_guided, replay_recording,
};

use crate::fillet_recourse_followability::{
    bend, lever_lens, lever_lens_least_lever, line_arc_internal, mixed_corner, short_leg_at_angle,
    two_lobes,
};

fn tol() -> Tol {
    Tol::witness()
}

/// What a door did with a request.
#[derive(Debug)]
enum Door {
    /// It built.
    Built,
    /// It escalated on `gate` at `site`, rendering `text`.
    Escalated {
        gate: Gate,
        site: Option<EscalatedAt>,
        text: String,
    },
    /// It refused otherwise.
    Refused(String),
}

fn guided(refusal: &StructureRefusal) -> Option<Door> {
    match &refusal.kind {
        StructureRefusalKind::Indeterminate { gate, .. } => Some(Door::Escalated {
            gate: *gate,
            site: None,
            text: refusal.to_string(),
        }),
        StructureRefusalKind::Flipped { .. } => None,
    }
}

fn path<V>(result: Result<V, PathError<f64>>) -> Door {
    match result {
        Ok(_) => Door::Built,
        Err(e @ PathError::Escalated { site, gate, .. }) => Door::Escalated {
            gate,
            site: Some(site),
            text: e.to_string(),
        },
        Err(PathError::Structure(r)) => guided(&r).unwrap_or_else(|| Door::Refused(r.to_string())),
        Err(other) => Door::Refused(format!("{other:?}")),
    }
}

/// A raw table loop through the validator.
fn table(vertices: &[(f64, f64, f64)]) -> Door {
    match crate::common::profile(vec![crate::common::chain(vertices)]).validate(tol()) {
        Ok(_) => Door::Built,
        Err(e @ ProfileError::Escalated { gate, .. }) => Door::Escalated {
            gate,
            site: None,
            text: e.to_string(),
        },
        Err(other) => Door::Refused(format!("{other:?}")),
    }
}

/// The declared seam arrival `off` beside the entry's direction, over a
/// closing leg of `arm`.
fn tilted_close(off: f64, arm: f64) -> Door {
    let t = tol();
    path(
        Open.at(Point2::new(0.0, 0.0))
            .angle(FRAC_PI_2, t)
            .and_then(|p| p.line(2.0 + arm, t))
            .and_then(|p| p.turn(FRAC_PI_2, t))
            .and_then(|p| p.line(1.0, t))
            .and_then(|p| p.turn(FRAC_PI_2, t))
            .and_then(|p| p.line(2.0 + 2.0 * arm, t))
            .and_then(|p| p.turn(FRAC_PI_2, t))
            .and_then(|p| p.line(1.0 + off, t))
            .and_then(|p| p.line_to(Start.arrives_tangent(), t)),
    )
}

/// The line × arc corner of [`line_arc_internal`] at radius `0.75`,
/// recorded, then replayed under guidance at `1.5 − 2.5ε₀`.
fn guided_line_circle(eps0: f64) -> Door {
    let closed = Open
        .at(Point2::new(0.0, 3.0))
        .line_to(Point2::new(0.0, 0.0), tol())
        .and_then(|p| p.toward(1.0, 0.0, tol()))
        .and_then(|p| {
            p.fillet_arc(
                0.75,
                Center {
                    c: Point2::new(0.0, 0.0),
                    winding: ArcSweep::Ccw,
                    p: Start,
                },
                tol(),
            )
        })
        .expect("the recorded program closes");
    let (_, structure) = replay_recording(&closed.program, tol()).expect("records");
    let mut steps = closed.program.clone();
    for s in &mut steps {
        if let Step::FilletArc { radius, .. } = s {
            *radius = 1.5 - 2.5 * eps0;
        }
    }
    match replay_guided(&steps, &structure, tol()) {
        Ok(_) => Door::Built,
        Err(e) => match &e.kind {
            profile::ReplayErrorKind::Path(PathError::Structure(r)) => {
                guided(r).unwrap_or_else(|| Door::Refused(e.to_string()))
            }
            profile::ReplayErrorKind::Path(PathError::Escalated { gate, site, .. }) => {
                Door::Escalated {
                    gate: *gate,
                    site: Some(*site),
                    text: e.to_string(),
                }
            }
            _ => Door::Refused(e.to_string()),
        },
    }
}

/// The fillet radius at which a straight bend's stored arc sits in the
/// band of `segment_straightness`: the first of the turns above
/// `√(8ε₀/r)` whose stored form the door cannot read.
fn stored_bend(eps0: f64) -> Door {
    let radius = 0.2;
    let flattening = (8.0 * eps0 / radius).sqrt();
    for k in 1..=64 {
        let door = path(bend(0.0, flattening * (1.0 + f64::from(k) * 0.05), radius));
        if matches!(
            door,
            Door::Escalated {
                site: Some(EscalatedAt::StoredForm),
                ..
            }
        ) {
            return door;
        }
    }
    Door::Refused("no turn across the band's width reads the stored form undecided".into())
}

/// A loop whose joint at `Start` runs straight within the band, its
/// last head `0.9ε₀` off the first leg's line, at validation.
fn straight_joint_at_start(eps0: f64) -> Door {
    let t = tol();
    let closed = Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .and_then(|p| p.line(1.0, t))
        .and_then(|p| p.line_to(Point2::new(0.0, 2.0), t))
        .and_then(|p| p.line_to(Point2::new(-2.0, 0.9 * eps0), t))
        .and_then(|p| p.angle(0.0, t))
        .and_then(|p| p.line(1.0, t))
        .and_then(|p| p.continue_to(Point2::new(-0.5, 0.9 * eps0), t))
        .and_then(|p| p.continue_to(Start.arrives_tangent(), t));
    match closed {
        Ok(closed) => match crate::common::profile(vec![crate::common::pinned(closed)]).validate(t)
        {
            Ok(_) => Door::Built,
            Err(e @ ProfileError::Escalated { gate, .. }) => Door::Escalated {
                gate,
                site: None,
                text: e.to_string(),
            },
            Err(other) => Door::Refused(format!("{other:?}")),
        },
        Err(e) => path::<()>(Err(e)),
    }
}

/// One gate driven in band at a door.
struct Driven {
    /// What the row drives.
    id: &'static str,
    /// The gate that must escalate.
    gate: Gate,
    /// The lever the refusal must render, as the sentence it must be.
    lever: &'static str,
    /// Whether the refusal offers the conditional tighten (and the row
    /// follows it).
    offers: bool,
    /// The request, its geometry fixed at `ε₀`.
    run: fn(f64) -> Door,
}

const MOVE: &str = "move the geometry";

const DRIVEN: &[Driven] = &[
    Driven {
        id: "line-circle clearance",
        gate: Gate::FilletOffsetLineCircle,
        lever: FILLET_NO_CORNER_RECOURSE,
        offers: true,
        run: |e| path(line_arc_internal(1.0 - 2.5 * e)),
    },
    Driven {
        id: "circle-circle external clearance",
        gate: Gate::FilletOffsetCirclesExternal,
        lever: FILLET_NO_CORNER_RECOURSE,
        offers: true,
        run: |e| path(two_lobes(0.5 - 2.5 * e)),
    },
    Driven {
        id: "circle-circle internal clearance",
        gate: Gate::FilletOffsetCirclesInternal,
        lever: FILLET_NO_CORNER_RECOURSE,
        offers: true,
        run: |e| path(mixed_corner(1.0 - 2.5 * e)),
    },
    Driven {
        id: "the enclosing gate",
        gate: Gate::FilletEnclosingCarrier,
        lever: FILLET_ENCLOSING_RECOURSE,
        offers: true,
        run: |e| path(two_lobes(1.0 + 5.0 * e)),
    },
    Driven {
        id: "a real angle on a short leg",
        gate: Gate::FilletCornerTurn,
        lever: FILLET_TURN_INBAND_RECOURSE,
        offers: true,
        run: |e| path(short_leg_at_angle(500.0 * e, 1e-4, 0.05)),
    },
    Driven {
        id: "the offset lever",
        gate: Gate::FilletOffsetLever,
        lever: FILLET_OFFSET_LEVER_RECOURSE,
        offers: false,
        run: |e| {
            let big = 3e10 * e;
            path(lever_lens(big, lever_lens_least_lever(big, e) + 5.0 * e))
        },
    },
    Driven {
        id: "a straight pair's degenerate corner turn",
        gate: Gate::PathCornerTurn,
        lever: profile::CORNER_ANCHOR_SEPARATION_LEVER,
        offers: true,
        run: |e| path(bend(0.0, e, 0.2)),
    },
    Driven {
        id: "a declared continuation's target",
        gate: Gate::PathContinuationTargetOffset,
        lever: "move the target onto the run's line",
        offers: true,
        run: |e| {
            path(
                Open.at(Point2::new(0.0, 0.0))
                    .angle(0.0, tol())
                    .and_then(|p| p.line(1.0, tol()))
                    .and_then(|p| p.continue_to(Point2::new(2.0, 1.5 * e), tol())),
            )
        },
    },
    Driven {
        id: "an authored leg's extent",
        gate: Gate::PathLegLength,
        lever: "author a longer leg",
        offers: true,
        run: |e| {
            path(
                Open.at(Point2::new(0.0, 0.0))
                    .angle(0.0, tol())
                    .and_then(|p| p.line(1.0, tol()))
                    .and_then(|p| p.line(3.0 * e, tol())),
            )
        },
    },
    Driven {
        id: "a declared seam arrival",
        gate: Gate::PathSeamArrivalTurn,
        lever: "move the geometry so the two directions agree (the declaration is the target)",
        offers: false,
        run: |e| tilted_close(3.0 * e, 1.0),
    },
    Driven {
        id: "a junction's turn",
        gate: Gate::PathJunctionTurn,
        lever: "for tangency onto a new carrier use .tangent()",
        offers: false,
        run: |e| {
            path(
                Open.at(Point2::new(0.0, 0.0))
                    .angle(0.0, tol())
                    .and_then(|p| p.line(1.0, tol()))
                    .and_then(|p| p.angle(3.0 * e, tol()))
                    .and_then(|p| p.line(1.0, tol())),
            )
        },
    },
    Driven {
        id: "the stored form's read",
        gate: Gate::SegmentStraightness,
        lever: FILLET_STORED_FORM_INBAND_RECOURSE,
        offers: false,
        run: stored_bend,
    },
    Driven {
        id: "near-coincident vertices at validation",
        gate: Gate::VertexSeparation,
        lever: MOVE,
        offers: true,
        run: |e| {
            table(&[
                (0.0, 0.0, 0.0),
                (1.0, 0.0, 0.0),
                (1.0, 5.0 * e, 0.0),
                (0.0, 1.0, 0.0),
            ])
        },
    },
    Driven {
        id: "a joint straight within the band at validation",
        gate: Gate::ChordSide,
        lever: MOVE,
        offers: true,
        run: straight_joint_at_start,
    },
    Driven {
        id: "a sliver bulge at validation",
        gate: Gate::SegmentStraightness,
        lever: MOVE,
        offers: false,
        run: |e| {
            table(&[
                (0.0, 0.0, 10.0 * e),
                (1.0, 0.0, 0.0),
                (1.0, 1.0, 0.0),
                (0.0, 1.0, 0.0),
            ])
        },
    },
    Driven {
        id: "a guided replay's fillet gate",
        gate: Gate::FilletOffsetLineCircle,
        lever: FILLET_NO_CORNER_RECOURSE,
        offers: true,
        run: guided_line_circle,
    },
];

/// The gates no door in this suite drives in band, each with why.
const NOT_DRIVEN: &[(Gate, &str)] = &[
    (Gate::FilletLegFit, EXACT),
    (Gate::FilletLegReach, EXACT),
    (Gate::CanonicalOrderX, EXACT),
    (Gate::CanonicalOrderY, EXACT),
    (
        Gate::FilletCornerArm,
        "shadowed by magnitude: its margin IS the lever arm, so a length gate on the same \
         request reads it first (`review_fillet_recourse_arm_r1_probes`)",
    ),
    (
        Gate::PathFilletJointTurn,
        "the door decides each declared joint by its turn over the fillet's own short arm, \
         where its construction's rounding stays far inside ε (`generic_replay`); its site \
         is held by the unit row in `path.rs`",
    ),
    (Gate::ArcApexIdentity, NO_FIXTURE),
    (Gate::ArcCarrierResolution, NO_FIXTURE),
    (Gate::ArcDiameterClearance, NO_FIXTURE),
    (Gate::ArcLanding, NO_FIXTURE),
    (Gate::ArcSpan, NO_FIXTURE),
    (Gate::ArcStartOnCarrier, NO_FIXTURE),
    (Gate::ArcSweepLength, NO_FIXTURE),
    (Gate::ArcSweepRange, NO_FIXTURE),
    (Gate::CarrierCirclesChord, NO_FIXTURE),
    (Gate::CarrierCirclesExternal, NO_FIXTURE),
    (Gate::CarrierCirclesIdentity, NO_FIXTURE),
    (Gate::CarrierCirclesInternal, NO_FIXTURE),
    (Gate::CarrierLineCircle, NO_FIXTURE),
    (Gate::CircleEndClear, NO_FIXTURE),
    (Gate::CircleExtremePastArc, NO_FIXTURE),
    (Gate::CircleFootPastSegment, NO_FIXTURE),
    (Gate::CircleSide, NO_FIXTURE),
    (Gate::CollinearOverlap, NO_FIXTURE),
    (Gate::ContactAtSharedVertex, NO_FIXTURE),
    (Gate::FullTurnReach, NO_FIXTURE),
    (Gate::FullTurnSense, NO_FIXTURE),
    (Gate::LineSpan, NO_FIXTURE),
    (Gate::LoopOrientation, NO_FIXTURE),
    (Gate::NearerEnd, NO_FIXTURE),
    (Gate::OwnCandidate, NO_FIXTURE),
    (Gate::PathArcBranch, READ_BACK),
    (Gate::PathArcBranchResolution, READ_BACK),
    (Gate::PathArcBulge, NO_FIXTURE),
    (Gate::PathArcCenterEquidistant, NO_FIXTURE),
    (Gate::PathArcCenterRadius, NO_FIXTURE),
    (Gate::PathArcChord, NO_FIXTURE),
    (Gate::PathArcLanding, READ_BACK),
    (Gate::PathArcLandingResolution, READ_BACK),
    (Gate::PathArcStartOnCarrier, READ_BACK),
    (Gate::PathArcStartOnCarrierResolution, READ_BACK),
    (Gate::PathArcSweep, NO_FIXTURE),
    (Gate::PathArcSweepFull, NO_FIXTURE),
    (Gate::PathArcThroughPoint, READ_BACK),
    (Gate::PathArcThroughPointResolution, READ_BACK),
    (Gate::PathArcViaOffset, NO_FIXTURE),
    (Gate::PathCarrierIdentity, NO_FIXTURE),
    (Gate::PathCarrierMeet, NO_FIXTURE),
    (Gate::PathCircleRadius, NO_FIXTURE),
    (Gate::PathCollinearTarget, NO_FIXTURE),
    (Gate::PathCornerAdvance, NO_FIXTURE),
    (Gate::PathCornerAdvanceArc, NO_FIXTURE),
    (Gate::PathCornerReachArc, NO_FIXTURE),
    (Gate::PathDirectorNorm, NO_FIXTURE),
    (Gate::PathFilletRadius, NO_FIXTURE),
    (Gate::PathJunctionSide, NO_FIXTURE),
    (Gate::PathRectangleSide, NO_FIXTURE),
    (Gate::PathRunOutCarrier, NO_FIXTURE),
    (Gate::PathSeamArrivalLever, NO_FIXTURE),
    (Gate::PathSeamArrivalSide, NO_FIXTURE),
    (Gate::RayAdvance, NO_FIXTURE),
    (Gate::RaySide, NO_FIXTURE),
    (Gate::SegmentGap, NO_FIXTURE),
];

const EXACT: &str = "classified against the exact-order band, which no f64 margin lies inside";
const READ_BACK: &str = "the read-back reads a sound construction well inside ε, so only a \
                         construction defect puts it in band";
const NO_FIXTURE: &str = "no fixture in this suite puts it in band at a door yet";

/// The words the conditional tighten opens with, and its value.
const TIGHTEN: &str = "tighten the tolerance below ";

fn offered_value(text: &str) -> Option<f64> {
    let (_, rest) = text.split_once(TIGHTEN)?;
    let value = rest.split_once(" m")?.0;
    Some(value.parse().expect("the offered tolerance is a number"))
}

/// **The driven rows and the listed ones are `Gate::ALL`, once each.**
#[test]
fn every_gate_is_driven_or_listed_with_its_reason() {
    let driven: BTreeSet<Gate> = DRIVEN.iter().map(|d| d.gate).collect();
    let listed: BTreeSet<Gate> = NOT_DRIVEN.iter().map(|(g, _)| *g).collect();
    assert_eq!(listed.len(), NOT_DRIVEN.len(), "a gate is listed twice");
    assert!(
        driven.is_disjoint(&listed),
        "driven and listed: {:?}",
        driven.intersection(&listed).collect::<Vec<_>>()
    );
    let all: BTreeSet<Gate> = Gate::ALL.iter().copied().collect();
    let both: BTreeSet<Gate> = driven.union(&listed).copied().collect();
    assert_eq!(both, all, "the driven and listed gates are not the table");
}

/// The environment the child reads.
const CHILD_ID: &str = "PROFILE_GATE_FOLLOW_ID";
const CHILD_EPS0: &str = "PROFILE_GATE_FOLLOW_EPS0";
const CHILD_OUT: &str = "PROFILE_GATE_FOLLOW_OUT";

/// **Each driven gate escalates, renders its own lever, and offers the
/// tighten exactly where following it decides the gate.**
#[test]
fn each_driven_gate_ends_as_following_it_shows() {
    let eps0 = tol().eps();
    for d in DRIVEN {
        let door = (d.run)(eps0);
        let Door::Escalated { gate, text, .. } = door else {
            panic!("{}: expected an escalation in band, got {door:?}", d.id);
        };
        assert_eq!(gate, d.gate, "{}: the gate that escalates: {text}", d.id);
        assert!(
            text.contains(&format!("Recourse: {}", d.lever))
                || text.contains(&format!("; Recourse: {}", d.lever))
                || text.contains(d.lever),
            "{}: renders its own lever {:?}: {text}",
            d.id,
            d.lever
        );
        match offered_value(&text) {
            None => assert!(!d.offers, "{}: a tighten was expected: {text}", d.id),
            Some(value) => {
                assert!(d.offers, "{}: offers a tighten it should not: {text}", d.id);
                let next = follow(d.id, eps0, value);
                assert_ne!(
                    next,
                    format!("escalated:{}", d.gate.predicate()),
                    "{}: at a tolerance below {value} m the same request still escalates on \
                     this gate: {text}",
                    d.id
                );
            }
        }
    }
}

/// The request `id`, re-run at a tolerance just below `value` with its
/// geometry at `eps0`, in a child process: what the door did.
fn follow(id: &str, eps0: f64, value: f64) -> String {
    let below = (value * 0.9).max((value + geom_core::tolerance::EPS_MIN) / 2.0);
    let out = std::env::temp_dir().join(format!(
        "profile_gate_follow_{}_{}",
        std::process::id(),
        id.replace(' ', "_")
    ));
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::child_gate_follow"),
        None => "child_gate_follow".to_string(),
    };
    let status = std::process::Command::new(std::env::current_exe().expect("test exe path"))
        .args([probe.as_str(), "--exact", "--nocapture"])
        .env("CAD_TOLERANCE_EPS", format!("{below:e}"))
        .env("CAD_AMBIGUITY_K", format!("{}", tol().k()))
        .env(CHILD_ID, id)
        .env(CHILD_EPS0, format!("{eps0:e}"))
        .env(CHILD_OUT, &out)
        .status()
        .expect("the child spawns");
    assert!(status.success(), "{id}: the child at ε = {below:e} failed");
    let verdict = std::fs::read_to_string(&out).expect("the child wrote");
    std::fs::remove_file(&out).expect("the child's file is removed");
    verdict
}

/// CHILD MODE (no-op unless [`CHILD_OUT`] is set): runs one driven
/// request at THIS process's committed ε and writes what the door did.
#[test]
fn child_gate_follow() {
    let Ok(out) = std::env::var(CHILD_OUT) else {
        return;
    };
    let id = std::env::var(CHILD_ID).expect("the child is told its row");
    let eps0: f64 = std::env::var(CHILD_EPS0)
        .expect("the child is told ε₀")
        .parse()
        .expect("ε₀ is a number");
    let d = DRIVEN.iter().find(|d| d.id == id).expect("the row exists");
    let verdict = match (d.run)(eps0) {
        Door::Built => "built".to_string(),
        Door::Escalated { gate, .. } => format!("escalated:{}", gate.predicate()),
        Door::Refused(why) => format!("refused: {why}"),
    };
    std::fs::write(out, verdict).expect("the child's file is writable");
}

/// **Each fillet lever serves the gates its docs name.** No door drives
/// `fillet_leg_fit`, `fillet_leg_reach` or `fillet_corner_arm` in band,
/// so their pairing with a lever is held here, against the `Gates:`
/// line on each sentence's doc in `src/fillet_recourse.rs`: two fillet
/// gates share a lever in the table exactly when one doc names both, and
/// the lever a doc's gates take is that doc's sentence. The two live in
/// different files, so a lone edit to either reds.
#[test]
fn each_fillet_lever_serves_the_gates_its_docs_name() {
    let text = std::fs::read_to_string(
        test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src/fillet_recourse.rs"),
    )
    .expect("the fillet recourse module is readable");
    let code = test_utils::source::code_and_literals(&text);
    let prose = test_utils::source::comments_only(&text);
    // Each `Gates:` line, and the const its doc sits above.
    let mut declared: Vec<(Vec<String>, String)> = Vec::new();
    let mut pending: Option<Vec<String>> = None;
    for (line, code_line) in prose.lines().zip(code.lines()) {
        if let Some(rest) = line.trim_start().strip_prefix("/// Gates:") {
            pending = Some(
                rest.split('`')
                    .filter(|piece| piece.starts_with("fillet_") || piece.starts_with("path_"))
                    .map(str::to_string)
                    .collect(),
            );
        }
        if let Some(name) = code_line
            .trim_start()
            .strip_prefix("pub const ")
            .and_then(|r| r.split(':').next())
            && let Some(group) = pending.take()
        {
            declared.push((group, name.to_string()));
        }
    }
    let fillet: Vec<Gate> = Gate::ALL
        .iter()
        .copied()
        .filter(|g| g.predicate().starts_with("fillet_"))
        .collect();
    for gate in &fillet {
        let (_, sentence) = declared
            .iter()
            .find(|(group, _)| group.iter().any(|n| n == gate.predicate()))
            .unwrap_or_else(|| panic!("{gate:?} is named on no `Gates:` line"));
        let lever = gate.kind().lever();
        assert!(
            text.contains(&format!("pub const {sentence}: &str"))
                && FILLET_LEVERS
                    .iter()
                    .any(|(name, value)| name == sentence && *value == lever),
            "{gate:?}'s doc names `{sentence}`, and the table gives it {lever:?}"
        );
    }
}

/// The fillet levers by name, for [`each_fillet_lever_serves_the_gates_its_docs_name`].
const FILLET_LEVERS: &[(&str, &str)] = &[
    (
        "FILLET_TURN_INBAND_RECOURSE",
        profile::FILLET_TURN_INBAND_RECOURSE,
    ),
    (
        "FILLET_NO_CORNER_RECOURSE",
        profile::FILLET_NO_CORNER_RECOURSE,
    ),
    (
        "FILLET_OFFSET_LEVER_RECOURSE",
        profile::FILLET_OFFSET_LEVER_RECOURSE,
    ),
    (
        "FILLET_ENCLOSING_RECOURSE",
        profile::FILLET_ENCLOSING_RECOURSE,
    ),
    ("FILLET_FIT_RECOURSE", profile::FILLET_FIT_RECOURSE),
    (
        "FILLET_LEG_EXTENT_RECOURSE",
        profile::FILLET_LEG_EXTENT_RECOURSE,
    ),
];
