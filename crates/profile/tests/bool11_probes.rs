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

mod common;

use common::pinned;
use geom_core::{Point2, Tol};
use profile::{CloseSite, ClosedLoop, Open, PathError, Profile, ProfileLoop, SketchPlane, Start};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

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
    Open.at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(2.0, 0.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(2.0, 2.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(0.0, 2.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Start, t)
        .unwrap()
}

/// **End to end.** The subdivided square closes through the lattice,
/// carries its eight authored vertices in order, declares no tangency,
/// and passes the data gate.
///
/// `tangent_joints` empty is the load-bearing half: the four collinear
/// joints the continuations minted are carrier IDENTITY, and identity
/// is not tangency — nothing was declared, so nothing is claimed about
/// independently-typed numbers at the verify layer.
#[test]
fn the_subdivided_square_closes_and_validates() {
    let t = Tol::witness();
    let loop_ = pinned(subdivided_square(t));
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
        let got = loop_.vertices()[i].pos();
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
    assert!(loop_.tangent_joints().is_empty());
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
    let target = p2(2.0, off);
    let chain = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(target, t)
        .unwrap();
    let closed = chain
        .line_to(p2(2.0, 2.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap();
    let v = pinned(closed);
    let landed = v.vertices()[2].pos();
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
    let entry = p2(0.3, -0.7);
    let closed = Open
        .at(entry)
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .line_to(p2(1.3, 1.0), t)
        .unwrap()
        .line_to(p2(0.3, 1.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(0.85, t)
        .unwrap()
        .continue_to(Start, t)
        .unwrap();
    let loop_ = pinned(closed);
    assert_eq!(loop_.vertices().len(), 5);
    let first = loop_.vertices()[0].pos();
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
        Open.at(p2(0.0, 0.0))
            .angle(0.0, t)
            .unwrap()
            .line(1.0, t)
            .unwrap()
    };
    let inferred = run(t).line_to(p2(2.0, 0.0), t);
    assert!(
        matches!(inferred, Err(PathError::JunctionTangent { .. })),
        "the undeclared spelling must still refuse: {inferred:?}"
    );
    assert!(run(t).continue_to(p2(2.0, 0.0), t).is_ok());
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
        Open.at(p2(0.0, 0.0))
            .angle(0.0, t)
            .unwrap()
            .line(1.0, t)
            .unwrap()
            .continue_to(p2(2.0, dy), t)
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
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(2.0, 1000.0 * eps_input), t)
        .expect_err("definitely off the ray");
    let msg = err.to_string();
    assert!(msg.contains("declared straight continuation"), "{msg}");
    assert!(msg.contains("across the ray"), "{msg}");
    assert!(msg.contains("line_to"), "{msg}");
}

/// **A target BEHIND the departure is a non-positive leg**, not a miss:
/// it is on the ray's line and off its half-line, which is the same
/// fact `line(len)` gates on an authored length. One fact, one refusal
/// — the off-ray refusal stays about the lateral miss it meters.
#[test]
fn a_target_behind_the_departure_is_a_nonpositive_leg() {
    let t = Tol::witness();
    let behind = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(0.5, 0.0), t);
    match behind {
        Err(PathError::NonpositiveLeg { length }) => assert!(length < 0.0),
        other => panic!("a target behind the departure must refuse: {other:?}"),
    }
}

/// **Carrier-blind, as the §2c axiom requires**: the row reads the
/// tangent and nothing about the leg that produced it, so off an
/// ARC-carrier point the same spelling authors a line TANGENT to that
/// arc and declares nothing. Legal to write here; refused at the DATA
/// gate, where an undeclared tangency between distinct carriers is
/// exactly what is caught. Declaring it is `.tangent()`'s job, and the
/// declared spelling is a different verb.
///
/// The fixture makes the end tangent nameable: a quarter arc left off
/// `+x` ends heading `+y`, so a target straight above the arc's end IS
/// on the departing ray and the check passes — which is the point. The
/// refusal that follows is the data gate's, not the algebra's.
#[test]
fn a_continuation_off_an_arc_is_undeclared_tangency_at_the_data_gate() {
    use profile::{ArcSide, Sweep};
    use std::f64::consts::FRAC_PI_2;
    let t = Tol::witness();
    let closed = Open
        .at(p2(0.0, 0.0))
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
        .continue_to(p2(1.0, 3.0), t)
        .expect("a target on the arc's own end tangent passes the on-ray check")
        .line_to(p2(-1.0, 3.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap();
    let loop_ = pinned(closed);
    let gate = Profile::new(SketchPlane::xy(), vec![loop_])
        .validate(Tol::witness())
        .expect_err("an undeclared arc/line tangency must not pass the data gate");
    assert!(
        format!("{gate}").contains("tangen"),
        "the data gate must name the tangency: {gate}"
    );
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
    let right = p2(1.0, 0.0);
    let keel = p2(0.0, -1.0);
    let m3 = p2(0.5 * (keel.x + right.x), 0.5 * (keel.y + right.y));
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
/// `TangentLineClose { site: Seam }`, from the verb that ended the
/// other half — which is exactly the point of naming the site.
#[test]
fn a_seam_at_a_subdivision_vertex_still_refuses_as_a_mid_carrier_seam() {
    use std::f64::consts::FRAC_PI_2;
    let t = Tol::witness();
    // Ring from (2,0): corner (3,0), corner (3,3), corner (0,3),
    // corner (0,0), subdivision (1,0), close on (2,0).
    let attempt = Open
        .at(p2(2.0, 0.0))
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
        Err(PathError::TangentLineClose { site, .. }) => {
            assert_eq!(site, CloseSite::Seam);
        }
        other => panic!("a mid-carrier seam must refuse as the SEAM: {other:?}"),
    }
}
