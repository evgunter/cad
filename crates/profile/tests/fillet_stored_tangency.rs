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

/// The FLATTENING refusal's payload, or a panic naming what came
/// instead — including its sibling, which is a different situation with
/// different levers and must not be mistaken for it.
fn flattened_refusal(err: &PathError<f64>, what: &str) -> (&'static str, f64, f64, f64, f64) {
    match err {
        PathError::FilletArcFlattenedInStorage {
            turn,
            radius,
            arc_length,
            predicate,
            margin,
        } => (*predicate, *margin, *turn, *radius, *arc_length),
        other => panic!("{what}: expected the flattening refusal, got {other}"),
    }
}

/// The refusals the stored-form read itself produces: the two losses
/// and the undecided twin that stands for either.
fn is_stored_form_refusal(err: &PathError<f64>) -> bool {
    match err {
        PathError::FilletArcFlattenedInStorage { .. }
        | PathError::FilletCarrierBelowSceneResolution { .. } => true,
        PathError::Escalated { source } => matches!(
            source.predicate,
            Some(
                "vertex_separation"
                    | "segment_straightness"
                    | "arc_diameter_clearance"
                    | "chord_side"
                    | "carrier_line_circle"
                    | "carrier_circles_identity"
                    | "carrier_circles_external"
                    | "carrier_circles_internal"
            )
        ),
        _ => false,
    }
}

/// Every way the door can refuse a corner inside the window, as one
/// closed set: a row asserting "typed" says WHICH types, so a refusal
/// arriving from somewhere else is a finding rather than a pass.
fn is_typed_door_refusal(err: &PathError<f64>) -> bool {
    matches!(
        err,
        PathError::FilletArcFlattenedInStorage { .. }
            | PathError::FilletCarrierBelowSceneResolution { .. }
            | PathError::Escalated { .. }
            | PathError::NoCornerForFillet { .. }
            | PathError::NoCornerOfPair { .. }
            | PathError::FilletOffsetLeverTooShort { .. }
    )
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
            flattened_refusal(&err, &format!("c = {c}, theta = {theta:e}"));
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
                    is_typed_door_refusal(&err),
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
    // A door call that refuses is not silently dropped: it is read,
    // and the ONLY refusals this corpus admits are the stored-form ones
    // — the two losses and their undecided twin. A corner refusing for
    // some other reason would mean the corpus had wandered off the
    // subject, which is exactly what a silent `keep` would hide.
    let mut refused = 0usize;
    let mut stored_form = 0usize;
    let mut keep = |name: String, lp: Result<ProfileLoop<f64>, PathError<f64>>| match lp {
        Ok(lp) => out.push((name, lp)),
        Err(e) => {
            assert!(
                is_typed_door_refusal(&e),
                "{name}: every refusal the corpus meets is one of the door's own, got {e}"
            );
            refused += 1;
            if is_stored_form_refusal(&e) {
                stored_form += 1;
            }
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
    // The swept counts, per ε row, as a number that MOVES when the door
    // does: a corpus that quietly lost half its loops would otherwise
    // still satisfy "more than none". The three ε rows CI gates give
    // three counts because the window they read moves with ε; a row
    // outside them says so rather than guessing.
    let expected = match format!("{:e}", tol().eps()).as_str() {
        "1e-9" => Some(24),
        "1e-6" => Some(32),
        "1e-12" => Some(16),
        _ => None,
    };
    if let Some(expected) = expected {
        assert_eq!(
            swept,
            expected,
            "the swept corners built {swept} loops at eps = {:e} and {refused} refused; the \
             count is pinned per ε row so a door that refuses more makes this row harder, \
             not easier",
            tol().eps()
        );
    }
    assert_eq!(
        swept + refused,
        72,
        "every swept call is accounted for: three corner kinds x eight turns x three radii"
    );
    assert!(
        stored_form > 0,
        "at eps = {:e} the sweep met {refused} refusals and none of them was the stored \
         form's — the corpus has wandered off this unit's subject",
        tol().eps()
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
    let named = out.len() - swept;
    assert_eq!(
        named, 12,
        "every named door shape is in the corpus, at every radius"
    );
    // **The suites' own fillet fixtures**, not only the ones this file
    // authors. The aggregated binary makes a sibling suite's private
    // fixture unreachable by name, but `common::coverage_corpus` is the
    // shared corpus those suites replay, and every fillet-bearing
    // program in it lands here — so the differential is taken over
    // geometry this unit did not choose as well as geometry it did.
    let shared = common::coverage_corpus();
    assert!(
        shared.len() >= 10,
        "the shared coverage corpus has {} loops, too few to be adding anything",
        shared.len()
    );
    let mut carried = 0usize;
    for (i, closed) in shared.into_iter().enumerate() {
        if closed.loop_.tangent_joints().is_empty() {
            continue;
        }
        carried += 1;
        out.push((format!("shared coverage corpus {i}"), closed.loop_));
    }
    assert!(
        carried >= 5,
        "only {carried} of the shared corpus's loops carry a declared joint — the \
         differential would be reading this file's own fixtures and little else"
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
                .zip(lp.bulges())
                .map(|(v, b)| {
                    format!(
                        "{:016x},{:016x},{:016x}",
                        v.x.to_bits(),
                        v.y.to_bits(),
                        b.to_bits()
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
    // **The golden.** FNV-1a over the whole dump — every coordinate and
    // every bulge of every corpus loop, as the bits they are stored as.
    // One ulp anywhere reds this row, which is what makes it an
    // instrument rather than a description of its own `format!`.
    //
    // To re-derive after a change that MOVED the stored form: run with
    // `CAD_DUMP_FILLETS=1` at each ε row, read the printed hash off the
    // failure, and say in the PR which loops moved and why.
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in dump.join("\n").bytes() {
        hash = (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3);
    }
    let expected = match format!("{:e}", tol().eps()).as_str() {
        "1e-9" => Some(GOLDEN_DEFAULT),
        "1e-6" => Some(GOLDEN_1E6),
        "1e-12" => Some(GOLDEN_1E12),
        _ => None,
    };
    if let Some(expected) = expected {
        assert_eq!(
            hash,
            expected,
            "the corpus's stored tables moved at eps = {:e} (got {hash:#018x}); every \
             coordinate and bulge is in this hash, so re-derive it and say what moved",
            tol().eps()
        );
    }
}

/// The corpus dump's hash at the default ε row (see
/// [`the_corpus_stored_loops_dump_to_the_bit`]). Re-derived when
/// `coverage_corpus` gained the RADIUS-ARRIVAL fused chain (row 16):
/// the dump grows that chain's own loop and the two carrier forms
/// after it renumber, so the label lines move. No loop that was in
/// the dump before changed a coordinate or a bulge — the new chain is
/// an addition, not an edit, at every ε.
const GOLDEN_DEFAULT: u64 = 0xb07a_e8ba_6ecd_7b4e;
/// The same at `CAD_TOLERANCE_EPS=1e-6`.
const GOLDEN_1E6: u64 = 0x1b29_8cb7_03c2_4bc2;
/// The same at `CAD_TOLERANCE_EPS=1e-12`.
const GOLDEN_1E12: u64 = 0xb986_07f6_554a_e6ba;

/// **The transition, bracketed.** Every other row here reads a turn a
/// long way from the crossing; this one reads both sides of it at the
/// run's own ε. `θ* = √(8ε/r)` is where the stored sagitta crosses ε,
/// and the band puts the escalating window just above it at `√K·θ*`, so
/// a hair below `θ*` must refuse and a short way above `√K·θ*` must
/// build and validate — with nothing assumed about the band's inside.
#[test]
fn the_transition_is_bracketed_on_both_sides_at_this_eps() {
    let star = (8.0 * tol().eps() / R).sqrt();
    let err =
        line_line(0.9 * star, R).expect_err("just below the crossing the stored arc is not an arc");
    assert!(
        is_stored_form_refusal(&err),
        "below the crossing the refusal is the stored form's, got {err}"
    );
    let above = 4.0 * tol().k().sqrt() * star;
    let lp = line_line(above, R)
        .unwrap_or_else(|e| panic!("clear above the band the door builds, got {e}"));
    validates(lp, tol()).unwrap_or_else(|e| panic!("and the loop validates, got {e}"));
}

/// **The reach's other edge, exhibited.** The check reads only the
/// joints the door DECLARED, and the natural question is whether an
/// undeclared one can come back `Tangent` and draw
/// `UndeclaredTangency` from validation — the door minting a refusal a
/// different way.
///
/// It cannot, and the shape that would do it is the one this row
/// builds: the exact outgoing fit, where the fillet arc consumes its
/// arrival side entirely and ends at the anchor. That is the only door
/// path that emits a fillet arc with `declare = false`
/// (`emit_fillet_arc(&trims, trims.fit_out == Sign::Positive)`), and
/// the reason it declares nothing is that nothing follows it on the
/// arrival carrier: the direction leaving that vertex is free, so there
/// is no second carrier for the joint to be tangent TO. The row pins
/// both halves — the door leaves the joint undeclared, and what it
/// built validates — so a future door that started declaring there, or
/// a validator that started calling that joint tangent, reds it.
///
/// An author who then continues tangentially owns that declaration
/// themselves; `UndeclaredTangency` is what tells them so, and it is a
/// claim about the declaration set rather than about the stored form.
#[test]
fn an_exact_outgoing_fit_leaves_its_joint_undeclared_and_still_validates() {
    // r = 1 consumes the line × arc corner's outgoing side exactly.
    let lp = Open
        .at(p2(0.0, 2.0))
        .line_to(p2(0.0, 0.0), tol())
        .and_then(|p| p.toward(2.0, 0.0, tol()))
        .and_then(|p| {
            p.fillet_arc(
                1.0,
                Center {
                    c: p2(0.0, 0.0),
                    winding: ArcSweep::Ccw,
                    p: Start,
                },
                tol(),
            )
        })
        .expect("the exact-fit radius constructs")
        .loop_;
    let joints = lp.vertices().len();
    assert!(
        lp.tangent_joints().len() < joints,
        "the exact fit declares fewer joints than the loop has: {:?} of {joints}",
        lp.tangent_joints()
    );
    validates(lp, tol()).expect("and the loop the door built validates");
}
