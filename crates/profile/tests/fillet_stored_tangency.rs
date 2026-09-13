//! **A fillet arc the profile cannot store is refused at the door, not
//! at validation.**
//!
//! A profile stores an arc as its chord and a bulge, and a reader
//! classifies that pair back into a carrier through the same predicates
//! validation runs. A fillet whose sagitta `r(1 − cos(θ/2)) ≈ r·θ²/8`
//! sits at or below the run's ε is stored as a segment read as a
//! straight one: the carrier the door computed exactly is simply not in
//! what the profile holds, and the tangency the fillet declares against
//! it is contradicted.
//!
//! So the path door reads its own output the way validation will and
//! refuses `FilletArcCannotCarryTangency` — carrying the predicate and
//! the margin that classification stopped at — or escalates through the
//! validator's own predicate name when the margin is in band.
//!
//! **The rows ride the run's ε.** The stored arc's sagitta goes as θ²,
//! so the window where a fillet builds but cannot be stored sits at
//! `θ ∝ √ε` and moves with the row CI draws. Every turn below is
//! written as a multiple of `√ε`, and the multiples are the ones that
//! land inside the window at 1e-6, 1e-9 and 1e-12 alike.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{p2, tol};
use geom_core::{Point2, Tol};
use profile::{
    ArcSweep, Center, Open, PathError, Profile, ProfileError, ProfileLoop, SketchPlane, Start,
};

/// The fillet radius every corner here is rounded with.
const R: f64 = 0.2;

/// The turn scale the window sits on: the sagitta `r·θ²/8` crosses ε at
/// `θ = √(8ε/r)`, so `√ε` is the unit every row below counts in.
fn scale() -> f64 {
    tol().eps().sqrt()
}

/// The multiples of `√ε` at which the door builds a fillet the stored
/// form cannot carry, at every ε row CI gates.
const INSIDE: [f64; 4] = [0.1, 0.3, 1.0, 2.0];

/// A multiple of `√ε` well clear of the window on both sides: the
/// fillet's sagitta is definitely above the band and the stored arc is
/// an arc.
const CLEAR: f64 = 32.0;

// ------------------------------------------------------------------
// The three corners, each authored through its own fillet door.
// ------------------------------------------------------------------

/// The item's **line × line** bend: the incoming ray runs east from the
/// origin, the corner sits at `(4, 0)`, the arrival leaves it at
/// `theta`, anchored three units along.
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

/// The centre of the **line × arc** corner's arrival circle: radius 2,
/// counterclockwise tangent `(cos θ, sin θ)` at the corner `(4, 0)`.
fn line_arc_centre(theta: f64) -> Point2<f64> {
    p2(4.0 - 2.0 * theta.sin(), 2.0 * theta.cos())
}

/// A **line × arc** corner turning by `theta`: the east ray from the
/// origin meets that circle at `(4, 0)`, and the fillet closes along it.
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

/// An **arc × arc** corner turning by `theta`: the two radius-2 circles
/// about `(∓θ, 0)` cross at `(0, √(4 − θ²))`, where their tangents are
/// an angle `theta` apart — the arc × arc fixtures' vesica with its
/// corner opened out to a shallow turn.
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

/// A corner kind: its name for the messages, and the door that builds
/// it at a given turn and radius.
type Corner = (
    &'static str,
    fn(f64, f64) -> Result<ProfileLoop<f64>, PathError<f64>>,
);

/// Every corner kind.
fn corners() -> [Corner; 3] {
    [
        ("line x line", line_line),
        ("line x arc", line_arc),
        ("arc x arc", arc_arc),
    ]
}

/// The refusal's payload, or a panic naming what came instead.
fn stored_form_refusal(err: &PathError<f64>, what: &str) -> (&'static str, f64, f64, f64, f64) {
    match err {
        PathError::FilletArcCannotCarryTangency {
            turn,
            radius,
            arc_length,
            predicate,
            margin,
        } => (*predicate, *margin, *turn, *radius, *arc_length),
        other => panic!("{what}: expected the stored-form refusal, got {other}"),
    }
}

/// Validation's verdict on a loop, as a `Result` a row can read.
fn validates(lp: ProfileLoop<f64>, tol: Tol) -> Result<(), ProfileError> {
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .map(|_| ())
}

// ------------------------------------------------------------------
// The rows
// ------------------------------------------------------------------

/// **The witness refuses at the door, typed, at every ε row.** The
/// refusal names the predicate that read the stored form and the margin
/// it classified, and both describe the fillet the author asked for:
/// the arc is `radius · |turn|` long and the joint's own margin is what
/// the classification stopped at.
#[test]
fn the_door_refuses_a_fillet_its_stored_form_cannot_carry() {
    for c in INSIDE {
        let theta = c * scale();
        let err = line_line(theta, R)
            .err()
            .unwrap_or_else(|| panic!("c = {c}, theta = {theta:e}: the door must refuse"));
        let (predicate, margin, turn, radius, arc_length) =
            stored_form_refusal(&err, &format!("c = {c}, theta = {theta:e}"));
        // The stored fillet is read as a straight segment and the
        // outgoing leg is straight too, so the classification that
        // refuses the declaration is the line/line carrier-identity
        // question the validator asks: `chord_side` on the far
        // endpoint.
        assert_eq!(
            predicate, "chord_side",
            "c = {c}: the refusal names the classification that read the stored form, got {err}"
        );
        assert!(
            margin.abs() >= tol().eps() * tol().k(),
            "c = {c}: a definite margin is what makes the declaration contradicted, got {margin:e}"
        );
        assert_eq!(radius, R, "c = {c}: the radius the author asked for");
        assert!(
            (turn.abs() - theta).abs() <= 1e-3 * theta,
            "c = {c}: the turn read back from the stored bulge is the corner's, got {turn:e}"
        );
        assert!(
            (arc_length - radius * turn.abs()).abs() <= f64::EPSILON * arc_length,
            "c = {c}: the arc length is radius * |turn|, got {arc_length:e}"
        );
    }
}

/// **Every corner kind, the same contract.** At a turn inside the
/// window the door either refuses — typed, through one of the
/// validator's own classifications — or builds a loop whose declared
/// tangency validation accepts. What it never does is mint a
/// declaration validation contradicts.
///
/// Both outcomes are live, and which one arrives is a fact about the
/// stored form rather than about the corner kind: an arc × arc corner
/// at a coarse ε stores its flattened fillet as a chord whose own
/// clearance from both leg circles is still under ε, so the
/// classification reads `Tangent` and the declaration holds. The door
/// asks the same question and gets the same answer, which is the point.
#[test]
fn every_corner_kind_either_refuses_or_builds_a_declaration_that_holds() {
    for (name, build) in corners() {
        for c in INSIDE {
            let theta = c * scale();
            match build(theta, R) {
                Err(err) => assert!(
                    matches!(
                        err,
                        PathError::FilletArcCannotCarryTangency { .. }
                            | PathError::Escalated { .. }
                            | PathError::NoCornerForFillet { .. }
                            | PathError::NoCornerOfPair { .. }
                            | PathError::FilletOffsetLeverTooShort { .. }
                    ),
                    "{name}, c = {c}: the refusal is typed, got {err}"
                ),
                Ok(lp) => {
                    if let Err(e) = validates(lp, tol()) {
                        assert!(
                            !matches!(
                                e,
                                ProfileError::TangencyContradicted { .. }
                                    | ProfileError::UndeclaredTangency { .. }
                            ),
                            "{name}, c = {c}, theta = {theta:e}: the door built a declaration \
                             validation contradicts: {e}"
                        );
                    }
                }
            }
        }
    }
}

/// **Well clear of the window the two sides agree, unchanged**: the
/// stored arc is an arc, both joints classify tangent, and the loop
/// validates.
///
/// The straight-legged corners are read at `32·√ε`, three decades above
/// the window at whatever ε the run committed. The arc × arc corner is
/// read at an ABSOLUTE turn instead: its two leg circles sit `2θ` apart,
/// so a shallow turn puts them within a whisker of each other and the
/// offset-lever gate that places the fillet's tangent points refuses on
/// conditioning grounds long before the stored form has anything to say
/// — a fact about deriving that corner, not about storing its fillet.
#[test]
fn a_fillet_the_stored_form_carries_builds_and_validates() {
    let wide: [(Corner, f64); 3] = [
        (("line x line", line_line), CLEAR * scale()),
        (("line x arc", line_arc), CLEAR * scale()),
        (("arc x arc", arc_arc), 1.0),
    ];
    for ((name, build), theta) in wide {
        let lp = build(theta, R)
            .unwrap_or_else(|e| panic!("{name}, theta = {theta:e}: the door builds it, got {e}"));
        validates(lp, tol())
            .unwrap_or_else(|e| panic!("{name}, theta = {theta:e}: and it validates, got {e}"));
    }
}

/// **The refusal's recourse is followable.** The sentence names two
/// levers — turn the corner further, or round it with a larger radius —
/// and both are real: the stored arc's sagitta is `r(1 − cos(θ/2))`, so
/// either one raises it past the band. The row follows each and asserts
/// the result builds AND validates.
#[test]
fn the_recourse_the_refusal_names_builds_and_validates() {
    let theta = scale();
    let refused = line_line(theta, R).expect_err("the window turn refuses");
    let shown = refused.to_string();
    assert!(
        shown.contains("turn the corner further") && shown.contains("LARGER radius"),
        "the refusal names both levers, got {shown}"
    );

    // Lever 1: the larger turn, at the same radius.
    let wider = CLEAR * scale();
    let lp = line_line(wider, R).expect("the larger turn builds");
    validates(lp, tol()).expect("and validates");

    // Lever 2: the larger radius, at the same turn. The sagitta scales
    // with r, so the radius the recourse needs is the one that lifts
    // r(1 - cos(theta/2)) clear of the band.
    let needed = tol().eps() * tol().k() / (1.0 - (theta * 0.5).cos());
    let bigger = needed * 4.0;
    // The leg the fillet is cut into has to hold the setback, which for
    // a turn this shallow is r*tan(theta/2) — vanishing beside r.
    let lp = line_line(theta, bigger)
        .unwrap_or_else(|e| panic!("the larger radius {bigger:e} m builds, got {e}"));
    validates(lp, tol())
        .unwrap_or_else(|e| panic!("the larger radius {bigger:e} m validates, got {e}"));

    // Lever 3: drop the fillet. The corner stays sharp and validates.
    let anchor = p2(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin());
    let sharp = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(4.0, 0.0), tol())
        .and_then(|p| p.line_to(anchor, tol()))
        .and_then(|p| p.line_to(Start, tol()))
        .expect("the sharp corner builds")
        .loop_;
    validates(sharp, tol()).expect("and the sharp corner validates");
}

/// **No loop the fillet doors build carries a declared tangency
/// validation refuses** — swept over the whole corpus of door shapes
/// this crate can author, at whatever ε the run committed.
///
/// The claim is about the declaration, not about everything a profile
/// can be refused for: a corpus loop whose legs or self-intersections
/// validation objects to is objected to on those terms, and this row
/// says nothing about them. What it does say is that
/// `TangencyContradicted` and `UndeclaredTangency` — the two refusals
/// that are ABOUT a declaration the door minted — never arrive.
#[test]
fn no_door_output_is_refused_for_its_declared_tangency() {
    for (name, lp) in corpus() {
        if let Err(e) = validates(lp, tol()) {
            assert!(
                !matches!(
                    e,
                    ProfileError::TangencyContradicted { .. }
                        | ProfileError::UndeclaredTangency { .. }
                ),
                "{name}: the door built a loop validation refuses for its declared tangency: {e}"
            );
        }
    }
}

/// Every fillet-authored loop this crate can build through a public
/// door, named — the corpus the row above sweeps and the differential
/// dumps.
fn corpus() -> Vec<(String, ProfileLoop<f64>)> {
    let mut out: Vec<(String, ProfileLoop<f64>)> = Vec::new();
    let mut keep = |name: String, lp: Result<ProfileLoop<f64>, PathError<f64>>| {
        if let Ok(lp) = lp {
            out.push((name, lp));
        }
    };
    // The three swept corners, over five decades of turn and three
    // radii — every regime of the window, both sides of it.
    for (name, build) in corners() {
        for c in [0.1, 0.3, 1.0, 2.0, 8.0, 32.0, 128.0, 1024.0] {
            for radius in [0.05, 0.2, 0.5] {
                keep(
                    format!("{name} c={c} r={radius}"),
                    build(c * scale(), radius),
                );
            }
        }
    }
    let swept = out.len();
    assert!(
        swept > 0,
        "the swept turns contributed nothing: every corner kind refused at every turn"
    );
    // The named shapes of each fillet door, at ordinary turns — these
    // are inside no window at any epsilon CI gates, so each one MUST
    // build, and `keep` silently dropping one would hide it.
    let mut must = |name: String, lp: Result<ProfileLoop<f64>, PathError<f64>>| {
        out.push((
            name.clone(),
            lp.unwrap_or_else(|e| panic!("{name}: this corpus shape must build, got {e}")),
        ));
    };
    for radius in [0.1, 0.25, 0.5] {
        must(format!("rounded square with a seam fillet r={radius}"), {
            let m = [p2(0.0, -1.0), p2(1.0, 0.0), p2(0.0, 1.0), p2(-1.0, 0.0)];
            let north = std::f64::consts::FRAC_PI_2;
            let th = [0.0, north, std::f64::consts::PI, -north];
            Open.at(m[0])
                .angle(th[0], tol())
                .and_then(|p| p.fillet(radius, tol()))
                .and_then(|p| p.at(m[1], tol()))
                .and_then(|p| p.angle(th[1], tol()))
                .and_then(|p| p.fillet(radius, tol()))
                .and_then(|p| p.at(m[2], tol()))
                .and_then(|p| p.angle(th[2], tol()))
                .and_then(|p| p.fillet(radius, tol()))
                .and_then(|p| p.at(m[3], tol()))
                .and_then(|p| p.angle(th[3], tol()))
                .and_then(|p| p.fillet(radius, tol()))
                .and_then(|p| p.to(Start, tol()))
                .map(|c| c.loop_)
        });
        must(format!("line x arc internal r={radius}"), {
            Open.at(p2(0.0, 2.0))
                .line_to(p2(0.0, 0.0), tol())
                .and_then(|p| p.toward(2.0, 0.0, tol()))
                .and_then(|p| {
                    p.fillet_arc(
                        radius,
                        Center {
                            c: p2(0.0, 0.0),
                            winding: ArcSweep::Ccw,
                            p: Start,
                        },
                        tol(),
                    )
                })
                .map(|c| c.loop_)
        });
        must(format!("arc x line r={radius}"), {
            Open.arc_fillet(
                Center {
                    c: p2(0.0, 0.0),
                    winding: ArcSweep::Cw,
                    p: p2(0.0, 2.0),
                },
                radius,
                tol(),
            )
            .and_then(|p| p.toward(1.0, 0.0, tol()))
            .and_then(|p| p.to(p2(4.0, 0.0), tol()))
            .and_then(|p| p.line_to(p2(4.0, 3.0), tol()))
            .and_then(|p| p.line_to(p2(-1.0, 3.0), tol()))
            .and_then(|p| p.line_to(Start, tol()))
            .map(|c| c.loop_)
        });
        must(format!("arc x arc vesica r={radius}"), {
            Open.arc_fillet_arc(
                Center {
                    c: p2(-1.0, 0.0),
                    winding: ArcSweep::Ccw,
                    p: p2(1.0, 0.0),
                },
                radius,
                Center {
                    c: p2(1.0, 0.0),
                    winding: ArcSweep::Ccw,
                    p: p2(-1.0, 0.0),
                },
                tol(),
            )
            .and_then(|p| p.line_to(Start, tol()))
            .map(|c| c.loop_)
        });
    }
    assert_eq!(
        out.len() - swept,
        12,
        "every named door shape is in the corpus, at every radius"
    );
    out
}

/// **The stored loops of the corpus, to the bit.** `CAD_DUMP_FILLETS=1`
/// prints every corpus loop's vertices and bulges as their raw IEEE
/// bit patterns, which is what a differential across two revisions
/// diffs. Off by that switch the row still runs the dump, so the
/// formatting cannot rot unnoticed.
#[test]
fn the_corpus_stored_loops_dump_to_the_bit() {
    let dump: Vec<String> = corpus()
        .into_iter()
        .map(|(name, lp)| {
            let verdict = match validates(lp.clone(), tol()) {
                Ok(()) => "validates",
                Err(_) => "refused",
            };
            let verts: Vec<String> = lp
                .vertices()
                .iter()
                .map(|v| {
                    format!(
                        "{:016x},{:016x},{:016x}",
                        v.pos().x.to_bits(),
                        v.pos().y.to_bits(),
                        v.bulge().to_bits()
                    )
                })
                .collect();
            format!(
                "{name} | {verdict} | {:?} | {}",
                lp.tangent_joints(),
                verts.join(" ")
            )
        })
        .collect();
    if std::env::var("CAD_DUMP_FILLETS").is_ok() {
        for line in &dump {
            println!("DUMP {line}");
        }
    }
    assert!(
        dump.iter().all(|line| line.contains(" | ")),
        "every dumped loop names itself and its stored table"
    );
}
