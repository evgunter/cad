//! **Review probes (R1) — the fillet door's stored-form check across the
//! door shapes the unit's own corpus reads only at ordinary turns.**
//!
//! The unit's claim is that no loop a fillet door emits carries a
//! declaration `Profile::validate` refuses. Its corpus sweeps the three
//! corner kinds through the window but reads the SEAM fillet, the fused
//! `.tangent()`-incoming fillet and a fillet whose incoming leg is
//! another fillet's arc only at ordinary turns. These rows put each of
//! those shapes inside the window, and add a shape where the stored
//! form could lose the tangency for a reason that is NOT flattening.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{p2, tol};
use geom_core::Tol;
use profile::{
    ArcSweep, Center, Open, PathError, Profile, ProfileError, ProfileLoop, SketchPlane, Start,
};

const R: f64 = 0.2;

fn scale() -> f64 {
    tol().eps().sqrt()
}

const INSIDE: [f64; 5] = [0.1, 0.3, 1.0, 2.0, 4.0];

fn validates(lp: ProfileLoop<f64>, t: Tol) -> Result<(), ProfileError> {
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(t)
        .map(|_| ())
}

/// The one contract: the door refuses, or what it built carries no
/// declaration validation objects to.
fn door_output_is_honest(what: &str, built: Result<ProfileLoop<f64>, PathError<f64>>) {
    match built {
        Err(_) => {}
        Ok(lp) => {
            if let Err(e) = validates(lp, tol()) {
                assert!(
                    !matches!(
                        e,
                        ProfileError::TangencyContradicted { .. }
                            | ProfileError::UndeclaredTangency { .. }
                    ),
                    "{what}: the door built a loop validation refuses for its declaration: {e}"
                );
            }
        }
    }
}

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

// ------------------------------------------------------------------
// The seam fillet, inside the window.
// ------------------------------------------------------------------

/// A loop whose CLOSING corner is the shallow one: the entry ray runs
/// east from the origin, the chain comes back along a carrier `theta`
/// off it, and `.fillet(r).to(Start)` rounds the seam — the emission
/// where the arc IS the closing segment and joint 0 is the declared
/// seam tangency.
fn seam_bend(theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    // The incoming carrier crosses the entry ray 2 m BEHIND the entry
    // point, so the derived corner lies behind the arrival anchor (the
    // seam side's own fit gate) and the fillet has a corner to round.
    let ux = theta.cos();
    let uy = theta.sin();
    let a = p2(-2.0 - ux, -uy);
    let b = p2(-2.0 - 5.0 * ux, -5.0 * uy);
    Open.at(p2(0.0, 0.0))
        .angle(0.0, tol())?
        .line(6.0, tol())?
        .line_to(p2(6.0, 4.0), tol())?
        .line_to(b, tol())?
        .line_to(a, tol())?
        .tangent()
        .fillet(radius, tol())?
        .to(Start, tol())
        .map(|c| c.loop_)
}

#[test]
fn the_seam_fillet_inside_the_window_never_mints_a_refused_declaration() {
    for c in INSIDE {
        let theta = c * scale();
        door_output_is_honest(&format!("seam fillet c={c}"), seam_bend(theta, R));
    }
}

#[test]
fn report_the_seam_fillet_window() {
    for c in [0.1, 0.3, 1.0, 2.0, 4.0, 32.0, 1024.0] {
        let theta = c * scale();
        println!(
            "R1 seam c={c} theta={theta:e}: {}",
            verdict(seam_bend(theta, R))
        );
    }
    for theta in [1e-3, 1e-2, 0.1, 0.4] {
        println!(
            "R1 seam abs theta={theta:e}: {}",
            verdict(seam_bend(theta, R))
        );
    }
}

// ------------------------------------------------------------------
// The fused `.tangent()`-incoming fillet, inside the window.
// ------------------------------------------------------------------

/// The incoming side is a `.tangent()` ray extension of the leg before
/// it, so the fillet MERGES into that leg (`extend_leg_to`) instead of
/// pushing its own straight piece — the emission whose incoming joint
/// is the leg's own already-declared end.
fn fused_bend(theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let anchor = p2(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin());
    Open.at(p2(0.0, 0.0))
        .angle(0.0, tol())?
        .line(2.0, tol())?
        .tangent()
        .fillet(radius, tol())?
        .at(anchor, tol())?
        .angle(theta, tol())?
        .line(1.0, tol())?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

#[test]
fn a_fused_incoming_fillet_inside_the_window_never_mints_a_refused_declaration() {
    for c in INSIDE {
        let theta = c * scale();
        door_output_is_honest(&format!("fused fillet c={c}"), fused_bend(theta, R));
    }
}

#[test]
fn report_the_fused_fillet_window() {
    for c in [0.1, 0.3, 1.0, 2.0, 4.0, 32.0, 1024.0] {
        let theta = c * scale();
        println!(
            "R1 fused c={c} theta={theta:e}: {}",
            verdict(fused_bend(theta, R))
        );
    }
}

// ------------------------------------------------------------------
// Two fillets in a row.
// ------------------------------------------------------------------

fn two_fillets(theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let a1 = p2(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin());
    let d2 = 2.0 * theta;
    // The second corner sits 3 m along the first fillet's outgoing ray,
    // and its arrival anchor another 3 m beyond it.
    let k = p2(a1.x + 3.0 * theta.cos(), a1.y + 3.0 * theta.sin());
    let a2 = p2(k.x + 3.0 * d2.cos(), k.y + 3.0 * d2.sin());
    Open.at(p2(0.0, 0.0))
        .angle(0.0, tol())?
        .fillet(radius, tol())?
        .at(a1, tol())?
        .angle(theta, tol())?
        .fillet(radius, tol())?
        .at(a2, tol())?
        .angle(d2, tol())?
        .line(1.0, tol())?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

#[test]
fn back_to_back_fillets_inside_the_window_never_mint_a_refused_declaration() {
    for c in INSIDE {
        let theta = c * scale();
        door_output_is_honest(&format!("two fillets c={c}"), two_fillets(theta, R));
    }
}

#[test]
fn report_back_to_back_fillets() {
    for c in [0.1, 0.3, 1.0, 2.0, 4.0, 32.0, 1024.0] {
        let theta = c * scale();
        println!(
            "R1 two c={c} theta={theta:e}: {}",
            verdict(two_fillets(theta, R))
        );
    }
    for theta in [1e-2, 0.1, 0.4] {
        println!(
            "R1 two abs theta={theta:e}: {}",
            verdict(two_fillets(theta, R))
        );
    }
}

// ------------------------------------------------------------------
// The arc-incoming door (`arc_fillet`) inside the window.
// ------------------------------------------------------------------

/// An authored arc incoming, a straight arrival `theta` off its tangent
/// at the meeting point — the `arc_fillet` door, which the unit's
/// corpus reads only at right angles.
fn arc_line(theta: f64, radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let arr = -std::f64::consts::FRAC_PI_2 + theta;
    let far = p2(2.0 + 4.0 * arr.cos(), 4.0 * arr.sin());
    Open.arc_fillet(
        Center {
            c: p2(0.0, 0.0),
            winding: ArcSweep::Cw,
            p: p2(0.0, 2.0),
        },
        radius,
        tol(),
    )?
    .toward(arr.cos(), arr.sin(), tol())?
    .to(far, tol())?
    .line_to(p2(-6.0, -6.0), tol())?
    .line_to(Start, tol())
    .map(|c| c.loop_)
}

#[test]
fn the_arc_incoming_door_inside_the_window_never_mints_a_refused_declaration() {
    for c in INSIDE {
        let theta = c * scale();
        door_output_is_honest(&format!("arc x line c={c}"), arc_line(theta, R));
    }
}

#[test]
fn report_the_arc_incoming_door() {
    for c in [0.1, 0.3, 1.0, 2.0, 4.0, 32.0, 1024.0] {
        let theta = c * scale();
        println!(
            "R1 arcline c={c} theta={theta:e}: {}",
            verdict(arc_line(theta, R))
        );
    }
    for theta in [1e-2, 0.1, 0.5] {
        println!(
            "R1 arcline abs theta={theta:e}: {}",
            verdict(arc_line(theta, R))
        );
    }
}

// ------------------------------------------------------------------
// A loss that is NOT flattening: the stored arc is definitely an arc,
// but the joint's classification is a cancellation at the scene's own
// magnitude.
// ------------------------------------------------------------------

fn far_bend(
    shift: f64,
    leg: f64,
    theta: f64,
    radius: f64,
) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let anchor = p2(shift + 4.0 + leg * theta.cos(), shift + leg * theta.sin());
    Open.at(p2(shift, shift))
        .angle(0.0, tol())?
        .fillet(radius, tol())?
        .at(anchor, tol())?
        .angle(theta, tol())?
        .line(leg, tol())?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

#[test]
fn a_fillet_whose_stored_arc_is_an_arc_still_never_mints_a_refused_declaration() {
    for shift in [0.0, 1e4, 1e6, 1e8, 1e10] {
        for leg in [3.0, 1e4, 1e7] {
            let what = format!("far bend shift={shift:e} leg={leg:e}");
            door_output_is_honest(&what, far_bend(shift, leg, 0.5, 0.2));
        }
    }
}

#[test]
fn report_the_far_scene_verdicts() {
    for shift in [0.0, 1e4, 1e6, 1e8, 1e10] {
        for leg in [3.0, 1e4, 1e7] {
            println!(
                "R1 far shift={shift:e} leg={leg:e}: {}",
                verdict(far_bend(shift, leg, 0.5, 0.2))
            );
        }
    }
}

// ------------------------------------------------------------------
// The window moves with epsilon.
// ------------------------------------------------------------------

#[test]
fn report_fixed_turns_against_the_committed_band() {
    for theta in [1e-6, 1e-5, 1e-4, 1e-3, 1e-2] {
        println!(
            "R1 fixed eps={:e} K={} theta={theta:e}: {}",
            tol().eps(),
            tol().k(),
            verdict(line_line(theta, R))
        );
    }
}

fn verdict(built: Result<ProfileLoop<f64>, PathError<f64>>) -> String {
    match built {
        Err(e) => format!("door refused [{:?}]: {}", e.kind(), short(&e.to_string())),
        Ok(lp) => match validates(lp, tol()) {
            Ok(()) => "built+validates".to_string(),
            Err(e) => format!("built, validate refused: {}", short(&e.to_string())),
        },
    }
}

fn short(s: &str) -> String {
    s.chars().take(400).collect()
}

// ------------------------------------------------------------------
// The K count of one fillet through each door.
// ------------------------------------------------------------------

/// **What the stored-form read costs the K stream, guarded.**
///
/// The read fires only names the verify layer already fires on the same
/// loop — no new predicate — and it fires a FIXED number of them per
/// fillet, which this row pins per corner kind so a read that grew a
/// classification cannot slip in unpriced.
///
/// The line × arc count has two legitimate values and the row takes
/// either: the arc/arc funnel stops at `carrier_circles_external` when
/// that clearance answers `Zero` (externally tangent carriers) and goes
/// on to `carrier_circles_internal` when it does not, so the door's
/// share is 11 or 12 decisions depending on which side of its leg the
/// fillet sits. Both are the same three calls — `build_seg` on the
/// fillet and on each declared joint's neighbour, `joint_tangency` on
/// each joint — and the row says so by branch rather than by pinning
/// the one it happens to meet.
#[test]
fn the_stored_form_read_costs_a_fixed_k_count_per_fillet() {
    use geom_core::k_stats::Bracket;
    use std::collections::BTreeMap;
    type Door = fn() -> Result<ProfileLoop<f64>, PathError<f64>>;
    let cases: [(&str, Door); 3] = [
        ("line x line", || line_line(0.4, R)),
        ("line x arc", || {
            let theta: f64 = 0.4;
            let c = p2(4.0 - 2.0 * theta.sin(), 2.0 * theta.cos());
            let start = p2(c.x + 2.0 * theta.cos(), c.y + 2.0 * theta.sin());
            Open.at(start)
                .line_to(p2(0.0, 0.0), tol())?
                .toward(1.0, 0.0, tol())?
                .fillet_arc(
                    R,
                    Center {
                        c,
                        winding: ArcSweep::Ccw,
                        p: Start,
                    },
                    tol(),
                )
                .map(|c| c.loop_)
        }),
        ("arc x arc", || {
            Open.arc_fillet_arc(
                Center {
                    c: p2(-1.0, 0.0),
                    winding: ArcSweep::Ccw,
                    p: p2(1.0, 0.0),
                },
                R,
                Center {
                    c: p2(1.0, 0.0),
                    winding: ArcSweep::Ccw,
                    p: p2(-1.0, 0.0),
                },
                tol(),
            )?
            .line_to(Start, tol())
            .map(|c| c.loop_)
        }),
    ];
    for (name, build) in cases {
        let bracket = Bracket::open();
        let out = build();
        let rec = bracket.finish();
        let mut by: BTreeMap<&str, usize> = BTreeMap::new();
        for v in &rec.verdicts {
            *by.entry(v.predicate).or_default() += 1;
        }
        println!(
            "R1 K {name}: total {} ok={} :: {by:?}",
            rec.verdicts.len(),
            out.is_ok()
        );
        assert!(
            out.is_ok(),
            "{name}: the K measurement reads a door that BUILDS"
        );
        // The read's own share: `build_seg`'s three gates on the fillet
        // and on each declared joint's neighbour, plus the joint
        // classification itself. Every name below is one the verify
        // layer fires on this loop already.
        let share: usize = [
            "vertex_separation",
            "segment_straightness",
            "arc_diameter_clearance",
            "chord_side",
            "carrier_line_circle",
            "carrier_circles_identity",
            "carrier_circles_external",
            "carrier_circles_internal",
        ]
        .iter()
        .map(|n| by.get(n).copied().unwrap_or(0))
        .sum();
        let expected: &[usize] = match name {
            "line x line" => &[9],
            // Two values, one situation: see this row's docs.
            "line x arc" => &[11, 12],
            "arc x arc" => &[15],
            other => panic!("unpriced corner kind {other}"),
        };
        assert!(
            expected.contains(&share),
            "{name}: the stored-form read fired {share} classifications, not {expected:?} \
             — the K stream's price per fillet moved: {by:?}"
        );
        // Three segments read, always: the fillet and its two declared
        // neighbours. A read that stopped looking at one would show here
        // before it showed anywhere else.
        assert_eq!(
            by.get("vertex_separation").copied().unwrap_or(0),
            3,
            "{name}: the read builds the fillet's stored segment and both neighbours"
        );
    }
}

/// **Is the refusal's recourse followable when the loss is NOT the
/// sagitta?** At a scene magnitude of 1e10 m the stored fillet is a
/// genuine arc — its sagitta is seven decades above the band — yet the
/// joint's margin `radius − |perp_dot(u, centre − a)|` is a
/// cancellation at the scene's own magnitude, so the door refuses with
/// the same arm and the same sentence. The sentence names a larger turn
/// and a LARGER radius; this row reports whether either one is a lever
/// there.
#[test]
fn report_the_recourse_at_a_far_scene() {
    for (turn, radius) in [
        (0.5, 0.2),
        (1.0, 0.2),
        (2.0, 0.2),
        (0.5, 2.0),
        (0.5, 20.0),
        (0.5, 200.0),
        (2.0, 200.0),
    ] {
        println!(
            "R1 recourse shift=1e10 turn={turn} r={radius}: {}",
            verdict(far_bend(1e10, 3.0, turn, radius))
        );
    }
}

/// **Deviation (h), checked with the door's read suppressed.** The PR
/// says both interval-lane loops whose rows moved are refused by
/// `Profile::validate` at that scalar and ε with the same predicate and
/// the same enclosure. Under the mutant (`Core::build` calling `finish`
/// without the stored-form read) these two doors build again, and this
/// row prints what validation then says about each.
#[cfg(feature = "interval")]
#[test]
fn report_the_interval_loops_with_the_door_read_suppressed() {
    use common::coverage_corpus;
    use geom_core::{Interval, Point2, Real};
    use profile::{
        ArcData, ArcSweep, Center, FilletDecision, ReplayStructure, Step, Target, replay,
        replay_guided, replay_recording,
    };
    fn pt(p: Point2<f64>) -> Point2<Interval> {
        Point2::new(Interval::from_f64(p.x), Interval::from_f64(p.y))
    }
    fn tgt(t: Target<f64>) -> Target<Interval> {
        match t {
            Target::Start => Target::Start,
            Target::StartArriving => Target::StartArriving,
            Target::Point(p) => Target::Point(pt(p)),
        }
    }
    fn spec(s: ArcData<f64>) -> ArcData<Interval> {
        match s {
            ArcData::Center { c, winding, target } => ArcData::Center {
                c: pt(c),
                winding,
                target: tgt(target),
            },
            other => panic!("unexpected arc spec {other:?}"),
        }
    }
    fn embed(step: &Step<f64>) -> Step<Interval> {
        match *step {
            Step::ArcFilletArc {
                spec: s,
                radius,
                spec2,
            } => Step::ArcFilletArc {
                spec: spec(s),
                radius: Interval::from_f64(radius),
                spec2: spec(spec2),
            },
            ref other => panic!("unexpected step {other:?}"),
        }
    }

    // (1) the generic corpus, restricted to the fused ArcFilletArc
    // rows (the relayed row the PR names is one of these).
    for (i, closed) in coverage_corpus().into_iter().enumerate() {
        if closed.program.len() != 1 || !matches!(closed.program[0], Step::ArcFilletArc { .. }) {
            continue;
        }
        let embedded: Vec<Step<Interval>> = closed.program.iter().map(embed).collect();
        match replay::<Interval>(&embedded, tol()) {
            Err(e) => println!(
                "R1 h corpus row {i}: replay refused: {}",
                short(&e.to_string())
            ),
            Ok(lp) => match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
                Ok(_) => println!("R1 h corpus row {i}: built and VALIDATES"),
                Err(e) => println!(
                    "R1 h corpus row {i}: built, VALIDATE refused: {}",
                    short(&e.to_string())
                ),
            },
        }
    }

    // (2) the guided hairline lens, told the other index.
    let s3 = 3.0_f64.sqrt();
    let program = Open
        .arc_fillet_arc(
            Center {
                c: p2(-1.0 + f64::EPSILON, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -s3),
            },
            0.5,
            Center {
                c: p2(1.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            tol(),
        )
        .expect("the lens constructs")
        .program;
    let (_, structure) = replay_recording(&program, tol()).expect("the lens replays at f64");
    let lifted: Vec<Step<Interval>> = program.iter().map(embed).collect();
    let d = &structure.fillets[0];
    let other = ReplayStructure {
        fillets: vec![FilletDecision {
            candidate: 1 - d.candidate,
            ..d.clone()
        }],
        ..structure.clone()
    };
    for (name, st) in [("recorded", &structure), ("other", &other)] {
        match replay_guided(&lifted, st, tol()) {
            Err(e) => println!(
                "R1 h lens {name}: replay refused: {}",
                short(&e.to_string())
            ),
            Ok(lp) => match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
                Ok(_) => println!("R1 h lens {name}: built and VALIDATES"),
                Err(e) => println!(
                    "R1 h lens {name}: built, VALIDATE refused: {}",
                    short(&e.to_string())
                ),
            },
        }
    }
}
