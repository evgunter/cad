//! **R2 review probes for M10-P** — an independent reviewer's own
//! derivations, not a re-reading of the unit's suites.
//!
//! Five questions, each attacking one of the review's claims from a
//! direction the shipped suites do not take:
//!
//! 1. Guided ≡ plain bitwise on ADVERSARIAL programs of the
//!    reviewer's own authoring (extreme fillet radii, a reflex arc
//!    carrier, a near-tangent junction) rather than on the unit's
//!    verb-coverage corpus.
//! 2. Does guided validation still refuse a SLIVER loop? The pinned
//!    permutation means `loop_orientation` does not run under
//!    guidance — and `loop_orientation` is the ONLY producer of
//!    `SliverLoop`.
//! 3. Is the consumed candidate index really unverified — i.e. does a
//!    record naming the other pocket pass every check the guided pass
//!    makes?
//! 4. When re-verification goes indeterminate somewhere OTHER than a
//!    corner gate, is the refusal still the typed `Structure` one that
//!    names the decision?
//! 5. When the carrier-MEET gate flips definitely at the lane, is that
//!    a `Structure::Flipped`?
//!
//! Rows 3–5 are EVIDENCE-ONLY: they assert the behaviour the reviewer
//! observed so the finding has a reproducible receipt, and they are
//! not proposed as pins for the tree.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{chain, p2, profile, tol};
use geom_core::{Point2, Real, Tol};
use profile::{
    ArcSide, ArcSweep, Center, Open, PathError, Profile, ProfileLoop, ProfileVertex, Radius,
    RawLoop, ReplayErrorKind, Start, Step, Sweep, replay, replay_guided, replay_recording,
};

// ------------------------------------------------------------------
// helpers
// ------------------------------------------------------------------

fn same_bits(a: &ProfileLoop<f64>, b: &ProfileLoop<f64>, what: &str) {
    assert_eq!(a.vertices().len(), b.vertices().len(), "{what}: arity");
    for (i, (u, v)) in a.vertices().iter().zip(b.vertices()).enumerate() {
        assert_eq!(u.pos().x.to_bits(), v.pos().x.to_bits(), "{what} v{i}.x");
        assert_eq!(u.pos().y.to_bits(), v.pos().y.to_bits(), "{what} v{i}.y");
        assert_eq!(u.bulge().to_bits(), v.bulge().to_bits(), "{what} v{i}.b");
    }
    assert_eq!(a.tangent_joints(), b.tangent_joints(), "{what}: joints");
}

/// Exact re-typing of a resolved step at another scalar (the same
/// `from_f64` embedding `anchor::embed_profile` performs).
fn embed_step<T: Real>(step: &Step<f64>) -> Step<T> {
    use profile::{ArcData, Target};
    fn pt<T: Real>(p: Point2<f64>) -> Point2<T> {
        Point2::new(T::from_f64(p.x), T::from_f64(p.y))
    }
    fn tgt<T: Real>(t: Target<f64>) -> Target<T> {
        match t {
            Target::Start => Target::Start,
            Target::Point(p) => Target::Point(pt(p)),
        }
    }
    fn spec<T: Real>(s: ArcData<f64>) -> ArcData<T> {
        match s {
            ArcData::Radius { r, side } => ArcData::Radius {
                r: T::from_f64(r),
                side,
            },
            ArcData::Bulge { target, b } => ArcData::Bulge {
                target: tgt(target),
                b: T::from_f64(b),
            },
            ArcData::Via { q, target } => ArcData::Via {
                q: pt(q),
                target: tgt(target),
            },
            ArcData::Center { c, winding, target } => ArcData::Center {
                c: pt(c),
                winding,
                target: tgt(target),
            },
            ArcData::Sweep { r, side, angle } => ArcData::Sweep {
                r: T::from_f64(r),
                side,
                angle: T::from_f64(angle),
            },
            ArcData::ArcLen { r, side, len } => ArcData::ArcLen {
                r: T::from_f64(r),
                side,
                len: T::from_f64(len),
            },
        }
    }
    match *step {
        Step::At(p) => Step::At(pt(p)),
        Step::ArcContinue(p) => Step::ArcContinue(pt(p)),
        Step::FarEndTo(p) => Step::FarEndTo(pt(p)),
        Step::Angle(v) => Step::Angle(T::from_f64(v)),
        Step::Turn(v) => Step::Turn(T::from_f64(v)),
        Step::Line(v) => Step::Line(T::from_f64(v)),
        Step::Toward { dx, dy } => Step::Toward {
            dx: T::from_f64(dx),
            dy: T::from_f64(dy),
        },
        Step::Tangent => Step::Tangent,
        Step::CloseTo => Step::CloseTo,
        Step::LineTo(t) => Step::LineTo(tgt(t)),
        Step::TangentArcTo(t) => Step::TangentArcTo(tgt(t)),
        Step::ArcTo(s) => Step::ArcTo(spec(s)),
        Step::Fillet { radius } => Step::Fillet {
            radius: T::from_f64(radius),
        },
        Step::FilletArc { radius, spec: s } => Step::FilletArc {
            radius: T::from_f64(radius),
            spec: spec(s),
        },
        Step::ArcFillet { spec: s, radius } => Step::ArcFillet {
            spec: spec(s),
            radius: T::from_f64(radius),
        },
        Step::ArcFilletArc {
            spec: s,
            radius,
            spec2,
        } => Step::ArcFilletArc {
            spec: spec(s),
            radius: T::from_f64(radius),
            spec2: spec(spec2),
        },
        Step::Circle { centre, radius } => Step::Circle {
            centre: pt(centre),
            radius: T::from_f64(radius),
        },
        Step::CircleSplit {
            centre,
            radius,
            n,
            phase,
        } => Step::CircleSplit {
            centre: pt(centre),
            radius: T::from_f64(radius),
            n,
            phase: T::from_f64(phase),
        },
    }
}

// ------------------------------------------------------------------
// 1. Guided ≡ plain on the reviewer's own adversarial programs
// ------------------------------------------------------------------

/// The reviewer's adversarial chains, generated as PARAMETER FAMILIES
/// swept to their extremes rather than hand-placed: a chain the
/// algebra refuses to construct is dropped, so the sweep finds the
/// edge of the admissible range instead of the author guessing it.
/// The surviving count is asserted by every caller, so a family that
/// silently emptied cannot pass as coverage.
fn adversarial() -> Vec<(String, Vec<Step<f64>>)> {
    use std::f64::consts::{FRAC_PI_2, PI};
    let t = Tol::witness();
    let mut rows: Vec<(String, Vec<Step<f64>>)> = Vec::new();

    // (a) A four-corner seam-closed diamond with EVERY corner
    //     filleted, across the whole radius range a 1.5 setback budget
    //     admits — hairline arcs at one end, leg-eating ones at the
    //     other. `.to(Start)` re-trims the corner under the entry
    //     vertex, so the seam resolution is in the sweep too.
    for r in [1e-6_f64, 1e-3, 0.1, 0.7, 1.2, 1.4, 1.49, 1.5, 1.6] {
        let built = (|| {
            Ok::<_, profile::PathError<f64>>(
                Open.at(p2(1.5, 0.0))
                    .angle(0.0, t)?
                    .fillet(r, t)?
                    .at(p2(3.0, 1.5), t)?
                    .angle(FRAC_PI_2, t)?
                    .fillet(r, t)?
                    .at(p2(1.5, 3.0), t)?
                    .angle(PI, t)?
                    .fillet(r, t)?
                    .at(p2(0.0, 1.5), t)?
                    .angle(-FRAC_PI_2, t)?
                    .fillet(r, t)?
                    .to(Start, t)?
                    .program,
            )
        })();
        match built {
            Ok(p) => rows.push((format!("seam diamond, line×line fillets r={r:e}"), p)),
            Err(e) => println!("R2-ADV dropped: seam diamond r={r:e}: {e}"),
        }
    }

    // (b) An ARC-CARRIER fillet family: a straight leg meeting a
    //     circular carrier, swept over the fillet radius. The corner
    //     is derived by ray×circle, gated on both sides, and the
    //     surviving pairs are ranked — the whole `arc_fillet::resolve`
    //     ladder, which the editor-core corpus never reaches.
    for r in [1e-4_f64, 0.05, 0.25, 0.5, 0.75, 1.0, 1.5] {
        let built = (|| {
            Ok::<_, profile::PathError<f64>>(
                Open.at(p2(0.0, 0.0))
                    .angle(0.0, t)?
                    .line(4.0, t)?
                    .fillet_arc(
                        r,
                        Center {
                            c: p2(4.0, 3.0),
                            winding: ArcSweep::Ccw,
                            p: p2(4.0, 6.0),
                        },
                        t,
                    )?
                    .arc_fillet(
                        Radius {
                            r: 3.0,
                            side: ArcSide::Left,
                        },
                        r,
                        t,
                    )?
                    .at(p2(1.0, 4.0), t)?
                    .toward(0.0, -1.0, t)?
                    .line(3.0, t)?
                    .line_to(Start, t)?
                    .program,
            )
        })();
        match built {
            Ok(p) => rows.push((format!("arc-carrier fillets r={r:e}"), p)),
            Err(e) => println!("R2-ADV dropped: arc-carrier r={r:e}: {e}"),
        }
    }

    // (c) A REFLEX arc carrier: the swept-angle helpers are exactly
    //     what the unit's own census found widening at `Interval`, so
    //     a sweep past π is the adversarial argument for them. Swept
    //     over the arc angle from acute to nearly a full turn.
    for a in [0.6_f64, 1.5, 2.9, 3.4, 4.2, 5.5, 6.0] {
        let built = (|| {
            Ok::<_, profile::PathError<f64>>(
                Open.at(p2(0.0, 0.0))
                    .angle(0.0, t)?
                    .arc_to(
                        Sweep {
                            r: 2.0,
                            side: ArcSide::Left,
                            angle: a,
                        },
                        t,
                    )?
                    .line_to(Start, t)?
                    .program,
            )
        })();
        match built {
            Ok(p) => rows.push((format!("reflex arc carrier, sweep={a}"), p)),
            Err(e) => println!("R2-ADV dropped: reflex sweep={a}: {e}"),
        }
    }

    // (d) NEAR-TANGENT junctions: two straight legs meeting at a turn
    //     swept down toward the declared-tangency band. Below the band
    //     the algebra refuses (`.tangent()` is the recourse), so the
    //     sweep walks right up to the wall.
    for turn in [1e-2_f64, 1e-4, 1e-6, 1e-8, 1e-10, 1e-12] {
        let built = (|| {
            Ok::<_, profile::PathError<f64>>(
                Open.at(p2(0.0, 0.0))
                    .angle(0.0, t)?
                    .line(3.0, t)?
                    .turn(turn, t)?
                    .line(3.0, t)?
                    .turn(FRAC_PI_2, t)?
                    .line(2.0, t)?
                    .turn(FRAC_PI_2, t)?
                    .line(6.0, t)?
                    .line_to(Start, t)?
                    .program,
            )
        })();
        match built {
            Ok(p) => rows.push((format!("near-tangent junction, turn={turn:e}"), p)),
            Err(e) => println!("R2-ADV dropped: near-tangent turn={turn:e}: {e}"),
        }
    }

    // (e) The hairline-asymmetric vesica lens — `fillet_select`'s own
    //     different-pockets hazard — swept over the asymmetry.
    for dx in [0.0_f64, 1e-15, 1e-12, 1e-9, 1e-6, 1e-3] {
        let built = std::panic::catch_unwind(|| lens_program_checked(dx));
        match built {
            Ok(Some(p)) => rows.push((format!("vesica lens, dx={dx:e}"), p)),
            Ok(None) | Err(_) => println!("R2-ADV dropped: vesica dx={dx:e}"),
        }
    }

    rows
}

/// **Claim 2, independently.** Guided replay at `f64` reproduces plain
/// replay bit for bit on adversarial programs the unit never wrote.
#[test]
fn r2_guided_equals_plain_bitwise_on_adversarial_programs() {
    let rows = adversarial();
    assert!(
        rows.len() >= 12,
        "the adversarial sweep emptied: {} rows",
        rows.len()
    );
    for (name, program) in rows {
        let plain = replay(&program, tol()).unwrap_or_else(|e| panic!("{name}: plain replay: {e}"));
        let (rec, structure) = replay_recording(&program, tol())
            .unwrap_or_else(|e| panic!("{name}: recording replay: {e}"));
        same_bits(&plain, &rec, &format!("{name}: recording"));
        let guided = replay_guided(&program, &structure, tol())
            .unwrap_or_else(|e| panic!("{name}: guided replay: {e}"));
        same_bits(&plain, &guided, &format!("{name}: guided"));
        println!(
            "R2-ADV ok: {name} ({} fillet decisions)",
            structure.fillets.len()
        );
    }
}

// ------------------------------------------------------------------
// 2. The sliver gate under guidance
// ------------------------------------------------------------------

/// **The `SliverLoop` gate is the only consumer of
/// `loop_orientation`** — and `loop_orientation` is exactly what
/// `validate_guided` does not run.
///
/// Two halves. The first is at `f64` and shows the gate is
/// structurally absent: a loop plain validation refuses `SliverLoop`
/// certifies under a record of the same shape. The second is the
/// honest lane case: the SAME loop widened to an `Interval` box whose
/// enclosure straddles zero area.
///
/// EVIDENCE-ONLY.
#[test]
fn r2_guided_validation_no_longer_refuses_a_sliver() {
    // A triangle of base 2 and apex height 1: healthy, CCW, outer.
    let healthy = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
    ])]);
    let (_, canonical) = healthy
        .validate_recording(tol())
        .expect("the healthy triangle validates and records");

    // The same triangle with the apex at 1e-12: 2A/perimeter is ~1e-12,
    // inside the (eps, K·eps) band at the witness tolerance.
    let sliver = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 0.0),
        (1.0, 1e-12, 0.0),
    ])]);
    let plain = sliver.validate(tol());
    println!("R2-SLIVER plain = {plain:?}");
    let guided = sliver.validate_guided(tol(), &canonical);
    println!(
        "R2-SLIVER guided = {}",
        match &guided {
            Ok(_) => "CERTIFIED".to_string(),
            Err(e) => format!("refused: {e}"),
        }
    );
    assert!(
        plain.is_err(),
        "the sliver must be refused by plain validation for this row to mean anything"
    );
    // The observation this row exists to record: whichever way it
    // goes, the printout above is the receipt.
    println!(
        "R2-SLIVER verdict: plain refuses = {}, guided certifies = {}",
        plain.is_err(),
        guided.is_ok()
    );
}

/// The lane version: one interval box, one guided validation.
///
/// The apex height is an `Interval` spanning zero, so the box CONTAINS
/// degenerate and inverted triangles. Plain `Interval` validation
/// cannot classify the orientation and escalates; the guided one does
/// not ask.
///
/// EVIDENCE-ONLY.
#[cfg(feature = "interval")]
#[test]
fn r2_a_zero_straddling_area_box_is_not_refused_under_guidance() {
    use geom_core::Interval;
    let healthy = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
    ])]);
    let (_, canonical) = healthy.validate_recording(tol()).expect("records");

    let v = |x: f64, y: Interval| {
        ProfileVertex::new(
            Point2::new(Interval::from_f64(x), y),
            Interval::from_f64(0.0),
        )
    };
    let wide: Profile<Interval> = Profile::new(
        profile::SketchPlane::xy(),
        vec![ProfileLoop::new(vec![
            v(0.0, Interval::from_f64(0.0)),
            v(2.0, Interval::from_f64(0.0)),
            v(1.0, Interval::from_bounds(-1.0, 1.0)),
        ])],
    );
    let plain = wide.validate(tol());
    println!(
        "R2-WIDEAREA plain@Interval = {}",
        match &plain {
            Ok(_) => "CERTIFIED".to_string(),
            Err(e) => format!("refused: {e}"),
        }
    );
    let guided = wide.validate_guided(tol(), &canonical);
    println!(
        "R2-WIDEAREA guided@Interval = {}",
        match &guided {
            Ok(_) => "CERTIFIED".to_string(),
            Err(e) => format!("refused: {e}"),
        }
    );
}

// ------------------------------------------------------------------
// 3. The candidate index: consumed, and unverified how far?
// ------------------------------------------------------------------

/// The vesica lens, the unit's own hairline-asymmetric fixture,
/// authored here independently so the row does not depend on theirs.
fn lens_program_checked(dx: f64) -> Option<Vec<Step<f64>>> {
    let s3 = 3.0_f64.sqrt();
    Open.arc_fillet_arc(
        Center {
            c: p2(-1.0 + dx, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.0, -s3),
        },
        0.5,
        Center {
            c: p2(1.0, 0.0),
            winding: ArcSweep::Ccw,
            p: Start,
        },
        Tol::witness(),
    )
    .ok()
    .map(|c| c.program)
}

fn lens_program(dx: f64) -> Vec<Step<f64>> {
    let s3 = 3.0_f64.sqrt();
    Open.arc_fillet_arc(
        Center {
            c: p2(-1.0 + dx, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.0, -s3),
        },
        0.5,
        Center {
            c: p2(1.0, 0.0),
            winding: ArcSweep::Ccw,
            p: Start,
        },
        Tol::witness(),
    )
    .expect("the lens constructs")
    .program
}

/// **The index is consumed with NO check of its own.** Handing the
/// guided pass the other pocket's index builds the other pocket, and
/// every surviving check — joint-space size, both fit signs, both
/// corner gates — passes, because none of them is about WHICH pocket.
///
/// EVIDENCE-ONLY: this is the design's stated posture, recorded here
/// with a receipt so the review can weigh it.
#[test]
fn r2_the_candidate_index_is_the_one_decision_nothing_checks() {
    let program = lens_program(1e-9);
    let (nominal, structure) = replay_recording(&program, tol()).expect("the lens replays");
    assert_eq!(structure.fillets.len(), 1, "one fused resolution");
    let d = &structure.fillets[0];
    println!(
        "R2-PICK survivors={} candidate={} fit_in={:?} fit_out={:?} corners={:?}",
        d.survivors, d.candidate, d.fit_in, d.fit_out, d.corners
    );
    if d.survivors < 2 {
        println!("R2-PICK: only one survivor — the swap is vacuous on this fixture");
        return;
    }
    let mut other = structure.clone();
    other.fillets[0].candidate = 1 - d.candidate;
    match replay_guided(&program, &other, tol()) {
        Ok(swapped) => {
            let moved = nominal.vertices().len() != swapped.vertices().len()
                || nominal
                    .vertices()
                    .iter()
                    .zip(swapped.vertices())
                    .any(|(a, b)| {
                        a.pos().x.to_bits() != b.pos().x.to_bits()
                            || a.pos().y.to_bits() != b.pos().y.to_bits()
                            || a.bulge().to_bits() != b.bulge().to_bits()
                    });
            println!(
                "R2-PICK: the swapped index was honoured with NO refusal; geometry moved = {moved}"
            );
            for (i, (a, b)) in nominal
                .vertices()
                .iter()
                .zip(swapped.vertices())
                .enumerate()
            {
                println!(
                    "  v{i}: nominal ({:?}, {:?}, b={:?})  swapped ({:?}, {:?}, b={:?})",
                    a.pos().x,
                    a.pos().y,
                    a.bulge(),
                    b.pos().x,
                    b.pos().y,
                    b.bulge()
                );
            }
        }
        Err(e) => println!("R2-PICK: the swapped index refused: {e}"),
    }
}

// ------------------------------------------------------------------
// 4/5. The refusal VOCABULARY away from the corner gate
// ------------------------------------------------------------------

/// **Where does the typed `Structure` vocabulary actually reach?**
///
/// The claim under review is that an indeterminate re-verification
/// aborts typed with `PathError::Structure` NAMING the decision. This
/// row walks the adversarial programs at `Interval` under guidance and
/// reports, per row, which error family the wall belongs to — so the
/// review can say how much of the ladder the new vocabulary covers.
///
/// EVIDENCE-ONLY.
#[cfg(feature = "interval")]
#[test]
fn r2_which_refusal_family_the_lane_actually_hits() {
    use geom_core::Interval;
    let rows = adversarial();
    assert!(
        rows.len() >= 12,
        "the adversarial sweep emptied: {} rows",
        rows.len()
    );
    for (name, program) in rows {
        let (_, structure) = replay_recording(&program, tol()).expect("records at f64");
        let lane: Vec<Step<Interval>> = program.iter().map(embed_step::<Interval>).collect();
        // Exact embedding first (the pinned case): this must agree.
        match replay_guided(&lane, &structure, tol()) {
            Ok(_) => println!("R2-VOCAB [{name}] exact embed: certified"),
            Err(e) => println!("R2-VOCAB [{name}] exact embed: {}", family(&e.kind)),
        }
        // Now widen every scalar in the program by a relative 1e-6 —
        // a genuinely wide parameter box, not an embedding.
        let widened: Vec<Step<Interval>> = program.iter().map(widen_step).collect();
        match replay_guided(&widened, &structure, tol()) {
            Ok(_) => println!("R2-VOCAB [{name}] widened 1e-6: CERTIFIED"),
            Err(e) => println!("R2-VOCAB [{name}] widened 1e-6: {}", family(&e.kind)),
        }
    }
}

#[cfg(feature = "interval")]
fn family(k: &ReplayErrorKind<geom_core::Interval>) -> String {
    match k {
        ReplayErrorKind::Path(PathError::Structure(r)) => {
            let arm = match &r.kind {
                profile::StructureRefusalKind::Indeterminate(_) => "Indeterminate",
                profile::StructureRefusalKind::Flipped { .. } => "Flipped",
            };
            format!("Structure/{arm} naming [{}]", r.decision)
        }
        ReplayErrorKind::Path(PathError::Escalated { source }) => {
            format!("Escalated (BARE, names no decision): {source}")
        }
        ReplayErrorKind::Path(p) => format!("Path::{}", short(p)),
        other => format!("{other:?}"),
    }
}

#[cfg(feature = "interval")]
fn short(p: &PathError<geom_core::Interval>) -> String {
    let s = format!("{p}");
    s.chars().take(90).collect()
}

/// Widens every scalar of a step into a relative-1e-6 box about its
/// value — the reviewer's stand-in for a genuinely wide parameter.
#[cfg(feature = "interval")]
fn widen_step(step: &Step<f64>) -> Step<geom_core::Interval> {
    use geom_core::Interval;
    fn w(v: f64) -> Interval {
        let h = v.abs().max(1.0) * 1e-6;
        Interval::from_bounds(v - h, v + h)
    }
    unwiden(*step, &w)
}

/// Re-typing with a widening embedding. Written as an explicit walk
/// rather than a generic one because the widening is not a `from_f64`.
#[cfg(feature = "interval")]
fn unwiden(step: Step<f64>, w: &dyn Fn(f64) -> geom_core::Interval) -> Step<geom_core::Interval> {
    use geom_core::Interval;
    use profile::{ArcData, Target};
    let pt = |p: Point2<f64>| Point2::new(w(p.x), w(p.y));
    let tgt = |t: Target<f64>| match t {
        Target::Start => Target::Start,
        Target::Point(p) => Target::Point(pt(p)),
    };
    let spec = |s: ArcData<f64>| match s {
        ArcData::Radius { r, side } => ArcData::Radius { r: w(r), side },
        ArcData::Bulge { target, b } => ArcData::Bulge {
            target: tgt(target),
            b: w(b),
        },
        ArcData::Via { q, target } => ArcData::Via {
            q: pt(q),
            target: tgt(target),
        },
        ArcData::Center { c, winding, target } => ArcData::Center {
            c: pt(c),
            winding,
            target: tgt(target),
        },
        ArcData::Sweep { r, side, angle } => ArcData::Sweep {
            r: w(r),
            side,
            angle: w(angle),
        },
        ArcData::ArcLen { r, side, len } => ArcData::ArcLen {
            r: w(r),
            side,
            len: w(len),
        },
    };
    match step {
        Step::At(p) => Step::At(pt(p)),
        Step::ArcContinue(p) => Step::ArcContinue(pt(p)),
        Step::FarEndTo(p) => Step::FarEndTo(pt(p)),
        Step::Angle(v) => Step::Angle(w(v)),
        Step::Turn(v) => Step::Turn(w(v)),
        Step::Line(v) => Step::Line(w(v)),
        Step::Toward { dx, dy } => Step::Toward {
            dx: w(dx),
            dy: w(dy),
        },
        Step::Tangent => Step::Tangent,
        Step::CloseTo => Step::CloseTo,
        Step::LineTo(t) => Step::LineTo(tgt(t)),
        Step::TangentArcTo(t) => Step::TangentArcTo(tgt(t)),
        Step::ArcTo(s) => Step::ArcTo(spec(s)),
        Step::Fillet { radius } => Step::Fillet { radius: w(radius) },
        Step::FilletArc { radius, spec: s } => Step::FilletArc {
            radius: w(radius),
            spec: spec(s),
        },
        Step::ArcFillet { spec: s, radius } => Step::ArcFillet {
            spec: spec(s),
            radius: w(radius),
        },
        Step::ArcFilletArc {
            spec: s,
            radius,
            spec2,
        } => Step::ArcFilletArc {
            spec: spec(s),
            radius: w(radius),
            spec2: spec(spec2),
        },
        Step::Circle { centre, radius } => Step::Circle {
            centre: pt(centre),
            radius: w(radius),
        },
        Step::CircleSplit {
            centre,
            radius,
            n,
            phase,
        } => Step::CircleSplit {
            centre: pt(centre),
            radius: w(radius),
            n,
            phase: w(phase),
        },
    }
}
