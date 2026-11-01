//! **The declared point-target continuation and the structural
//! closer** (issue 433's lattice half, ruled 2026-09-01 fourth round).
//!
//! What the rows here pin, in the order the ruling states it:
//!
//! - the leg is DECLARED, not inferred — the same collinear target
//!   refuses through `line_to` (which computes a direction and
//!   classifies it) and is accepted through `continue_to` (which takes
//!   the ray and checks the target against it);
//! - the check is BANDED, with a row on BOTH sides of the boundary and
//!   one inside the escalation band, all stated in units of the run's
//!   own ε so they mean the same thing on every tolerance leg;
//! - the emitted vertex is the AUTHORED target, bit-for-bit, and for
//!   `Start` that is what closing means;
//! - the miss BOOL-8 measured on lily's corner is accepted, and by a
//!   margin rather than on the boundary.
//!
//! The seam wall's own flip — the closer ending the departure half and
//! PQ4 standing at the seam half — is pinned where its anticipating
//! doc lives, in `path_property.rs`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{pinned, pinned_constructed};
use geom_core::{Point2, Tol};
use profile::{ClosedLoop, Open, PathError, Profile, ProfileLoop, SketchPlane, Start};

fn validate_ok(l: &ProfileLoop<f64>) {
    Profile::new(SketchPlane::xy(), vec![l.clone()])
        .validate(Tol::witness())
        .expect("algebra-lowered loop passes the junction verifier");
}

/// The run's classification band, read from the witness rather than
/// written as a literal: every row below states its miss as a multiple
/// of ε, so the same source means the same thing on the default leg, on
/// `CAD_TOLERANCE_EPS=1e-6` and on `1e-12`.
fn band() -> (f64, f64) {
    let t = Tol::witness();
    (t.eps(), t.k() * t.eps())
}

/// A unit square with one interior subdivision per side, seam cut at a
/// corner, authored through the lattice: three `turn`s, four `line`
/// legs, three point-target continuations and the closer. The shape the
/// ruling was for — four corners said on eight vertices — and the only
/// one in which a straight run crosses the seam.
fn subdivided_square(t: Tol) -> ClosedLoop<f64> {
    use std::f64::consts::FRAC_PI_2;
    Open.at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Point2::new(2.0, 0.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Point2::new(2.0, 2.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Point2::new(0.0, 2.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Start, t)
        .unwrap()
}

/// **End to end.** The subdivided square closes through the lattice,
/// carries its eight authored vertices in order, constructs the four
/// collinear joints its continuations minted, and passes the data gate.
#[test]
fn the_subdivided_square_closes_and_validates() {
    let t = Tol::witness();
    let loop_ = pinned_constructed(subdivided_square(t));
    // EIGHT vertices, not nine: the closer's target is the entry, which
    // the loop already carries, so the closing leg mints nothing.
    assert_eq!(loop_.vertices().len(), 8);
    let want = [
        (0.0, 0.0),
        (1.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (2.0, 2.0),
        (1.0, 2.0),
        (0.0, 2.0),
        (0.0, 1.0),
    ];
    for (i, (wx, wy)) in want.iter().enumerate() {
        let got = loop_.vertices()[i];
        // The three point-target vertices land EXACTLY where they were
        // authored; the four `line(len)` vertices are `at + û·len`, and
        // û comes from `turn`'s round trip through the angle, so their
        // off-axis component is the round trip's own residue rather
        // than zero. Measured, not assumed: it is ~1.8e-16 here.
        assert!(
            (got.x - wx).abs() < 1e-15 && (got.y - wy).abs() < 1e-15,
            "vertex {i}: got ({}, {}), want ({wx}, {wy})",
            got.x,
            got.y
        );
    }
    // The four `continue_to` joints are constructed tangent joints
    // (every zero-turn joint is one, 2026-09-02); the four corners turn.
    assert_eq!(loop_.constructed_joints(), &[1, 3, 5, 7]);
    validate_ok(&loop_);
}

/// **The authored target IS the vertex** (§4 item 3), not its
/// projection onto the ray — pinned where the two differ: a target a
/// hair off the ray, inside the band, lands exactly where it was
/// authored.
///
/// This is what makes the closer close. Projecting would put the loop's
/// last vertex a hair off the entry vertex, and a loop that closes to
/// within a band is not closed.
#[test]
fn an_accepted_target_lands_where_it_was_authored() {
    let t = Tol::witness();
    let (eps, _) = band();
    let off = 0.01 * eps;
    let target = Point2::new(2.0, off);
    let chain = Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(target, t)
        .unwrap();
    let closed = chain
        .line_to(Point2::new(2.0, 2.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap();
    let v = pinned(closed);
    let landed = v.vertices()[2];
    assert_eq!(
        (landed.x.to_bits(), landed.y.to_bits()),
        (target.x.to_bits(), target.y.to_bits()),
        "the accepted target must be the emitted vertex, bit for bit"
    );
}

/// **The closer mints NO vertex.** `Start` is the entry, and the entry
/// is already a vertex of the loop; a closing leg is the segment back
/// to it. Authoring it again would put the entry on the path twice
/// (§4 item 3 says once) and leave a zero-length segment behind, which
/// the data gate calls `DegenerateSegment` — measured, before this was
/// fixed.
///
/// So the row pins the count and the entry's bits: the rectangle below
/// has five authored vertices (four corners and one subdivision on the
/// closing side), and the loop that comes out has exactly those five,
/// the first of them bit-for-bit the authored entry.
#[test]
fn the_closer_mints_no_vertex_at_the_entry() {
    use std::f64::consts::FRAC_PI_2;
    let t = Tol::witness();
    let entry = Point2::new(0.3, -0.7);
    let closed = Open
        .at(entry)
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .line_to(Point2::new(1.3, 1.0), t)
        .unwrap()
        .line_to(Point2::new(0.3, 1.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(0.85, t)
        .unwrap()
        .continue_to(Start, t)
        .unwrap();
    let loop_ = pinned(closed);
    assert_eq!(loop_.vertices().len(), 5);
    let first = loop_.vertices()[0];
    assert_eq!(
        (first.x.to_bits(), first.y.to_bits()),
        (entry.x.to_bits(), entry.y.to_bits()),
        "the entry vertex must survive the close bit for bit"
    );
    validate_ok(&loop_);
}

/// **DECLARED, not inferred** — the contrast row. The same three
/// collinear points, the same lowered geometry, two spellings:
///
/// - `line_to(q)` computes a direction toward `q`, classifies it
///   against the incoming tangent, finds it in band and refuses
///   `JunctionTangent`. That refusal is correct and stays: the verb
///   never said the leg was straight, so accepting would be reading
///   intent off a coincidence;
/// - `continue_to(q)` says the leg is the continuation, so there is no
///   direction to classify — only the target to check.
///
/// The difference is the declaration and nothing else.
#[test]
fn the_declaration_is_what_separates_the_two_spellings() {
    let t = Tol::witness();
    let run = |t: Tol| {
        Open.at(Point2::new(0.0, 0.0))
            .angle(0.0, t)
            .unwrap()
            .line(1.0, t)
            .unwrap()
    };
    let inferred = run(t).line_to(Point2::new(2.0, 0.0), t);
    assert!(
        matches!(inferred, Err(PathError::JunctionTangent { .. })),
        "the undeclared spelling must still refuse: {inferred:?}"
    );
    assert!(run(t).continue_to(Point2::new(2.0, 0.0), t).is_ok());
}

/// **The band, on both sides and in the middle.** One fixture, one
/// authored miss, swept across the run's own boundary:
///
/// - `0.01·ε` — coincident with the ray at the precision anything here
///   represents: ACCEPTED;
/// - `3·ε` — inside the escalation band (ε, K·ε): ESCALATED, because
///   the numbers cannot decide and this kernel never guesses;
/// - `100·K·ε` — definitely off the ray: REFUSED typed, the miss and
///   the along-extent riding the payload.
///
/// The boundary is ε_input (= K·ε, the two-tolerance principle's
/// input-quality role) because the question is about authored INPUT —
/// does the authored point agree with the authored intent — and it is
/// reached through the predicate funnel rather than by comparing
/// against K·ε directly, which is what leaves the escalation band
/// escalating instead of silently accepted.
#[test]
fn the_on_ray_check_is_banded_on_both_sides() {
    let t = Tol::witness();
    let (eps, eps_input) = band();
    let attempt = |dy: f64| {
        Open.at(Point2::new(0.0, 0.0))
            .angle(0.0, t)
            .unwrap()
            .line(1.0, t)
            .unwrap()
            .continue_to(Point2::new(2.0, dy), t)
    };
    assert!(attempt(0.01 * eps).is_ok(), "inside ε must be accepted");
    assert!(
        matches!(attempt(3.0 * eps), Err(PathError::Escalated { .. })),
        "the escalation band must escalate, not decide"
    );
    let refused = attempt(100.0 * eps_input);
    match refused {
        Err(PathError::ContinuationTargetOffRay { across, along }) => {
            assert!((across - 100.0 * eps_input).abs() <= f64::EPSILON * across.abs().max(1.0));
            assert!(
                (along - 1.0).abs() < 1e-6,
                "the along extent rides too: {along}"
            );
        }
        other => panic!("a definitely-off target must refuse typed, got {other:?}"),
    }
}

/// The refusal's message names the declared intent and the measured
/// miss — a build cannot see a message, so a row does.
#[test]
fn the_off_ray_message_names_the_intent_and_the_miss() {
    let t = Tol::witness();
    let (_, eps_input) = band();
    let err = Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Point2::new(2.0, 1000.0 * eps_input), t)
        .expect_err("definitely off the ray");
    let msg = err.to_string();
    assert!(msg.contains("declared straight continuation"), "{msg}");
    assert!(msg.contains("not on its run's line"), "{msg}");
    assert!(msg.contains("across the line"), "{msg}");
    assert!(msg.contains("line_to"), "{msg}");
    // A definite arm names no tolerance lever (D4 ¶1 (i)).
    assert!(!msg.contains("tolerance"), "{msg}");
}

/// **The two in-band escalations name the geometry, never a tolerance.**
/// A target `1.5ε` off the run's line escalates on
/// `path_continuation_target_offset`, and a `line(3ε)` leg on
/// `path_leg_length`. Each recourse is the authored data's (move the
/// target onto the run's line, author a longer leg), and neither
/// advises loosening ε (D4 ¶1 (i)): both have a geometry lever, so the
/// last resort does not apply.
#[test]
fn the_in_band_continuation_escalations_name_no_tolerance() {
    let t = Tol::witness();
    let (eps, _) = band();
    let started = || {
        Open.at(Point2::new(0.0, 0.0))
            .angle(0.0, t)
            .unwrap()
            .line(1.0, t)
            .unwrap()
    };
    let off = started()
        .continue_to(Point2::new(2.0, 1.5 * eps), t)
        .expect_err("in band of the run's line");
    let short = started()
        .line(3.0 * eps, t)
        .expect_err("a leg in band of zero");
    for (want, recourse, err) in [
        (
            "path_continuation_target_offset",
            "move the target onto the run's line",
            off,
        ),
        ("path_leg_length", "author a longer leg", short),
    ] {
        let msg = err.to_string();
        assert!(format!("{err:?}").contains(want), "{err:?}");
        assert!(msg.contains(recourse), "{msg}");
        assert!(!msg.contains("tolerance"), "{msg}");
    }
}

/// **A target BEHIND the departure is a non-positive leg**, not a miss:
/// it is on the ray's line and off its half-line, which is the same
/// fact `line(len)` gates on an authored length. One fact, one refusal
/// — the off-ray refusal stays about the lateral miss it meters.
#[test]
fn a_target_behind_the_departure_is_a_nonpositive_leg() {
    let t = Tol::witness();
    let behind = Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Point2::new(0.5, 0.0), t);
    match behind {
        Err(PathError::NonpositiveLeg { length }) => assert!(length < 0.0),
        other => panic!("a target behind the departure must refuse: {other:?}"),
    }
}

/// **Carrier-blind, as the §2c axiom requires**: the row reads the
/// tangent and nothing about the leg that produced it, so off an
/// ARC-carrier point the same spelling authors a line TANGENT to that
/// arc, and constructs that joint as it constructs a collinear one.
///
/// The fixture makes the end tangent nameable: a quarter arc left off
/// `+x` ends heading `+y`, so a target straight above the arc's end IS
/// on the departing ray and the check passes — which is the point.
#[test]
fn a_continuation_off_an_arc_is_a_constructed_tangency() {
    use profile::{ArcSide, Sweep};
    use std::f64::consts::FRAC_PI_2;
    let t = Tol::witness();
    let closed = Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .arc_to(
            Sweep {
                r: 1.0,
                side: ArcSide::Left,
                angle: FRAC_PI_2,
            },
            t,
        )
        .unwrap()
        .continue_to(Point2::new(1.0, 3.0), t)
        .expect("a target on the arc's own end tangent passes the on-ray check")
        .line_to(Point2::new(-1.0, 3.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap();
    let loop_ = pinned_constructed(closed);
    // A continuation verb CONSTRUCTS the zero-turn joint it mints, so
    // the arc/line tangency this row was written about is verified,
    // not decided from values. BOOL-8's recorded carrier-blindness
    // hole — the door could not see the arc carrier, so the gate had to
    // catch it — is closed by the construction rather than by a lookup.
    assert!(loop_.constructed_joints().contains(&1));
    let vp = profile::ConstructedProfile::new(SketchPlane::xy(), vec![loop_])
        .validate(Tol::witness())
        .expect("the continuation constructed the joint, so the gate verifies it");
    assert!(vp.loops()[0].decided_joints().is_empty());
}

/// **BOOL-8's measured residual, accepted.** The corner of lily's kite
/// section sits 7.85e-17 off the closing ray — a midpoint of two
/// authored points, exact as geometry and inexact as arithmetic. The
/// closer must take it, and this row measures the miss it takes rather
/// than asserting that it did.
///
/// The number is the fixture's, not the kernel's: it is what `0.5·(a+b)`
/// leaves behind at these coordinates. What the row pins is that a miss
/// of that size is DEEP inside the accepting side of the band — six
/// orders below ε on the tightest leg the suite runs — so no plausible
/// band choice could have excluded it, and the acceptance is not
/// balanced on the boundary.
#[test]
fn lilys_measured_corner_miss_is_deep_inside_the_band() {
    let t = Tol::witness();
    let (eps, _) = band();
    let right = Point2::new(1.0, 0.0);
    let keel = Point2::new(0.0, -1.0);
    let m3 = Point2::new(0.5 * (keel.x + right.x), 0.5 * (keel.y + right.y));
    // The ray the closer departs on: the run from `keel` through `m3`,
    // as `toward` binds it.
    let d = right - keel;
    let n = d.norm_squared().sqrt();
    let u = (d.x / n, d.y / n);
    let to_start = right - m3;
    let across = u.0 * to_start.y - u.1 * to_start.x;
    assert!(
        across.abs() < 1e-15,
        "the fixture's own miss, measured: {across}"
    );
    assert!(
        across.abs() < 0.001 * eps,
        "the measured miss {across} must sit far inside ε = {eps}, not near it"
    );
    // And the kernel takes it.
    assert!(
        Open.at(keel)
            .toward(d.x, d.y, t)
            .unwrap()
            .line(0.5 * n, t)
            .unwrap()
            .continue_to(right, t)
            .is_ok()
    );
}

/// **PQ4 still refuses a mid-carrier seam, said by THIS verb.** The
/// declared closer removes the DEPARTURE half of the old wall; it does
/// not touch the seam half, and this row is the fixture that reaches
/// it.
///
/// It takes two consecutive subdivisions on one side, which the strict
/// corner/subdivision alternation of "exactly one per side" forbids —
/// so the square below subdivides its bottom side TWICE and cuts the
/// seam at the second of them. The closing leg then departs the first
/// subdivision (a continuation, so the verb applies and the on-ray
/// check passes) and lands on a seam whose own junction is straight.
/// `SeamTangent`, from the verb that ended the
/// other half — which is exactly the point of naming the site.
#[test]
fn a_seam_at_a_subdivision_vertex_still_refuses_as_a_mid_carrier_seam() {
    use std::f64::consts::FRAC_PI_2;
    let t = Tol::witness();
    // Ring from (2,0): corner (3,0), corner (3,3), corner (0,3),
    // corner (0,0), subdivision (1,0), close on (2,0).
    let attempt = Open
        .at(Point2::new(2.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(3.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(3.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(3.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Start, t);
    match attempt {
        Err(PathError::SeamTangent { .. }) => {}
        other => panic!("a mid-carrier seam must refuse as the SEAM: {other:?}"),
    }
}

/// A run along x from the origin: `line(1)` to `(1, 0)`, then a
/// `continue_to` through each of `ys`, the `k`-th to `(1 + k, ys[k])`.
/// The first refusal, with its leg, or the run's tip.
#[allow(clippy::type_complexity)]
fn run_through(
    ys: &[f64],
    t: Tol,
) -> Result<
    profile::PartialPath<
        f64,
        profile::path::HasPos<profile::path::WithIncoming>,
        profile::path::NoAng,
    >,
    (usize, PathError<f64>),
> {
    let mut chain = Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap();
    for (k, &y) in ys.iter().enumerate() {
        chain = chain
            .continue_to(Point2::new(2.0 + k as f64, y), t)
            .map_err(|e| (k, e))?;
    }
    Ok(chain)
}

/// **The band's guarantee is PER RUN: a run's misses never sum.**
/// (PATHS-DESIGN §4, as ruled on #5498.)
///
/// Every `continue_to` of a run asks its departing point `lands_on_ray`,
/// which reads the line bound at the run's first vertex, never one
/// re-anchored at an accepted target. So an author who biases every
/// target `0.5·ε` further to one side (the targets `(1 + k, k·ε/2)`, R1's
/// chain) is read against `y = 0` throughout:
/// - the first leg's `0.5·ε` passes;
/// - the run refuses by the third leg, `1.5·ε` off, in band:
///   `Escalated` on `path_continuation_target_offset`, not a silent
///   accept (the second, `1.0·ε`, sits on the zero edge, where rounding
///   lands either side, so it is not pinned);
/// - the target the twenty-first leg would aim at, `10.5·ε` off at
///   `K = 10`, is read definitely off the run's line
///   (`ContinuationTargetOffRay`) from the run's own tip.
///
/// Before the ruling, each leg was read against its own vertex, every
/// leg passed, and forty of them put the run's end `20·ε` off its line.
#[test]
fn a_runs_misses_are_read_against_its_own_line_and_never_sum() {
    let t = Tol::witness();
    let eps = t.eps();
    let bias = 0.5 * eps;
    let ys: Vec<f64> = (1..=40).map(|k| k as f64 * bias).collect();
    match run_through(&ys, t) {
        Err((leg, PathError::Escalated { source }))
            if (1..=2).contains(&leg)
                && source.predicate == Some("path_continuation_target_offset") => {}
        Err((leg, e)) => panic!("leg {leg} refused, but not as the in-band run: {e:?}"),
        Ok(_) => panic!("a run biased past its band was accepted"),
    }
    let first = run_through(&ys[..1], t).expect("the first leg's 0.5·ε passes");
    let off = (2.0 * t.k()).ceil() + 1.0;
    match first.continue_to(Point2::new(1.0 + off, off * bias), t) {
        Err(PathError::ContinuationTargetOffRay { across, .. }) => {
            assert!(
                (across - off * bias).abs() < eps,
                "read off the run's line: {across:e}"
            );
        }
        other => panic!("the {off}th leg's target must be off the run's line: {other:?}"),
    }
}

/// **A run whose misses alternate sides passes.** Forty targets at
/// `±0.5·ε` from the run's line, alternating: each is read against the
/// line, so every one passes, and every vertex of the run lies within
/// `ε` of it.
#[test]
fn a_run_whose_misses_alternate_sides_passes() {
    let t = Tol::witness();
    let eps = t.eps();
    let ys: Vec<f64> = (0..40)
        .map(|k| if k % 2 == 0 { 0.5 * eps } else { -0.5 * eps })
        .collect();
    let closed = run_through(&ys, t)
        .unwrap_or_else(|(leg, e)| panic!("leg {leg} refused: {e:?}"))
        .line_to(Point2::new(41.0, 50.0), t)
        .unwrap()
        .line_to(Point2::new(0.0, 50.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap();
    let loop_ = pinned(closed);
    for v in &loop_.vertices()[..=41] {
        assert!(v.y.abs() <= eps, "a run vertex {v:?} off its line");
    }
}

/// **A turn starts a new run.** Two runs along x joined by a step up
/// (`turn(π/2)`, `line(1)`, `turn(−π/2)`): a turn binds the direction, so
/// the second run's line is bound at its own first vertex. A target
/// `0.9·ε` above that line passes, read against it. (Read against the
/// first run's line, the same target would stand about a metre off it
/// and refuse `ContinuationTargetOffRay`: the first run's line is not
/// what the second reads.)
#[test]
fn a_turn_starts_a_new_run() {
    let t = Tol::witness();
    let eps = t.eps();
    let miss = 0.9 * eps;
    let up = run_through(&[miss], t)
        .unwrap_or_else(|(leg, e)| panic!("leg {leg} refused: {e:?}"))
        .turn(core::f64::consts::FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .turn(-core::f64::consts::FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap();
    // The second run starts at (2, 1 + 0.9·ε) and its first leg ends at
    // (3, 1 + 0.9·ε); a target 0.9·ε above that line passes.
    up.continue_to(Point2::new(4.0, 1.0 + 2.0 * miss), t)
        .expect("the second run reads its own line, not the first run's");
}

/// **`.tangent()`, `.cusp()` and `line(len)` keep the run.** The verbs
/// that inherit the direction bitwise hand the run on (PATHS-DESIGN §2c),
/// and so does the straight continuation `line(len)` off a directed
/// point: a straight leg off the inherited direction continues it, so
/// its next `continue_to` is still read against the run's first line.
/// After one accepted `0.6·ε` miss, `line(1)`, `.tangent().line(1)` and
/// `.cusp().line(1)` land on the run, and a target `1.2·ε` off the run's
/// line (only `0.6·ε` off the leg's own end) escalates as the plain
/// continuation's does.
#[test]
fn straight_legs_off_a_run_keep_it() {
    let t = Tol::witness();
    let eps = t.eps();
    let miss = 0.6 * eps;
    let in_band = |r: Result<_, PathError<f64>>, what: &str| match r {
        Err(PathError::Escalated { source })
            if source.predicate == Some("path_continuation_target_offset") => {}
        Err(e) => panic!("{what}: refused, but not in band of the run's line: {e:?}"),
        Ok(_) => panic!("{what}: read against the leg's own end, not the run's line"),
    };
    let tip = || run_through(&[miss], t).unwrap_or_else(|(leg, e)| panic!("leg {leg}: {e:?}"));
    in_band(
        tip()
            .continue_to(Point2::new(3.0, 2.0 * miss), t)
            .map(|_| ()),
        "continue_to",
    );
    in_band(
        tip()
            .line(1.0, t)
            .unwrap()
            .continue_to(Point2::new(4.0, 2.0 * miss), t)
            .map(|_| ()),
        "line",
    );
    in_band(
        tip()
            .tangent()
            .line(1.0, t)
            .unwrap()
            .continue_to(Point2::new(4.0, 2.0 * miss), t)
            .map(|_| ()),
        ".tangent().line",
    );
    in_band(
        tip()
            .cusp()
            .line(1.0, t)
            .unwrap()
            .continue_to(Point2::new(0.5, 2.0 * miss), t)
            .map(|_| ()),
        ".cusp().line",
    );
}
