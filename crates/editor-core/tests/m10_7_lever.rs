//! **The extent lever** (ERROR-DESIGN E3's amendment, ratified at
//! revision E12): a parallelism verdict a measure consumes is levered by
//! an UPPER BOUND ON THE OPERANDS' EXTENT, with no floor.
//!
//! The rows here are the amendment's own falsifier, aimed three ways: a
//! small part's small tilt reads PARALLEL because the tilt is priced
//! across the faces it actually spans; a large tilt does not; and a tilt
//! that only the EXTENT can decide is decided, which is what tells the
//! shipped arm from a separation-only one. The arm this replaced —
//! `max(separation, 1 m)` — got the first wrong for every model smaller
//! than a metre, which is most of them.
//!
//! Everything goes through the public doors: `Datum`, `Profile`,
//! `Extrude`, the selection door for the faces, and a `Measure` node
//! carrying `Distance`. Nothing here reaches into `eval::measure`, and
//! no row asserts a margin — they assert what a CONSUMER sees, which is
//! a number or a typed refusal.
//!
//! # Why the tilts are derived from the run's epsilon, not written down
//!
//! Every claim here is a claim about which side of the BAND a levered
//! margin `sin(theta) * L` falls on, and the band is `[eps, K*eps]` — a
//! run-time global (`CAD_TOLERANCE_EPS`, read through `Tol::eps` and
//! `Tol::k`), with the same binary at every value. A fixed angle
//! therefore states its row at ONE epsilon only: the first version of
//! this file wrote `1e-8 rad`, which is true at the default 1e-9 and
//! false at 1e-12, where a 1e-8 rad tilt across 20 mm is no longer a
//! coincidence. Hosted CI draws one epsilon per run, so a row written
//! that way is red on some draws and green on others for a reason that
//! has nothing to do with the lever.
//!
//! So each row solves for its own angle against this run's band, and
//! against the arm it is reasoning about — computed from the fixture's
//! authored dimensions by [`arm_of`], re-deriving the shipped
//! `reach(a) + reach(b) + ||ref(b) - ref(a)||` here rather than reading
//! it out of the crate, so that a change to that formula reds these
//! rows instead of silently re-aiming them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, CapEnd, Datum, DocEdit, DocumentId, EntityKind, EvalOptions, Evaluation,
    LoopProgram, MeasureExpr, MeasurePrimitive, MeasureRef, NamePat, Node, NodeResult, ProfileDoc,
    ProfileProgram, RecipeNodeId, SegPat, SegTag, Selector, ValuePayload, apply, evaluate, select,
};
use fixture::{len, scl};
use geom_core::Tol;

/// The air between the two plates: 10 mm, the amendment's own example.
const SEPARATION: f64 = 10.0e-3;
/// Each plate's own thickness.
const THICKNESS: f64 = 1.0e-3;

/// **The lever `eval::measure` forms for this fixture**, re-derived from
/// the authored dimensions.
///
/// The shipped arm is `reach(a) + reach(b) + ||ref(b) - ref(a)||`. Here
/// each face is a square cap of half-width `half` whose plane origin is
/// its sketch frame's origin, so its reach is the distance to a corner,
/// `sqrt(2) * half`; and the two plane origins are the lower plate's TOP
/// cap (`z = THICKNESS`) and the upper plate's BOTTOM cap
/// (`z = THICKNESS + SEPARATION`), `SEPARATION` apart. The tilt does not
/// enter — it turns the upper frame about its own origin.
fn arm_of(half: f64) -> f64 {
    2.0 * std::f64::consts::SQRT_2 * half + SEPARATION
}

fn mint(doc: &ProfileDoc, node: Node<ProfileProgram>) -> (ProfileDoc, RecipeNodeId) {
    let applied =
        apply(doc, &DocEdit::InsertNode { node }, Tol::witness()).expect("the insert applies");
    let id = applied.record.minted.expect("an insert mints an id");
    (applied.doc, id)
}

/// A named cap of a prism, read at the node that owns it.
fn cap(ev: &Evaluation<f64>, node: RecipeNodeId, end: CapEnd) -> MeasureRef {
    let sel =
        Selector::of(NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(end)));
    let mut found = select(ev, node, &sel);
    assert_eq!(found.len(), 1, "one {end:?} cap on node {node:?}");
    MeasureRef::new(node, found.remove(0))
}

/// **Two square plates of half-width `half`, the upper one TILTED by
/// `theta` about the x axis**, `SEPARATION` of air between their facing
/// caps, and a `Measure` reading the distance between the lower plate's
/// TOP cap and the upper plate's BOTTOM cap.
///
/// The tilt is authored as the sketch frame's `v` direction, so the
/// DOCUMENT says "this plane is tilted" rather than the test saying
/// "these normals differ" — which is what makes each row a statement
/// about the measure and not about the predicate.
fn measures(half: f64, theta: f64) -> Result<f64, String> {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-7-lever"), Tol::witness());
    let (c, s) = (theta.cos(), theta.sin());
    let square = LoopProgram::polygon([(-half, -half), (half, -half), (half, half), (-half, half)])
        .expect("finite corners");
    let mut plates = Vec::new();
    for (z, v) in [
        (0.0, [scl(0.0), scl(1.0), scl(0.0)]),
        (THICKNESS + SEPARATION, [scl(0.0), scl(c), scl(s)]),
    ] {
        let (next, plane) = mint(
            &doc,
            Node::Datum(Datum::Frame {
                origin: [len(0.0), len(0.0), len(z)],
                u: [scl(1.0), scl(0.0), scl(0.0)],
                v,
            }),
        );
        doc = next;
        let (next, profile) = mint(
            &doc,
            Node::Profile(ProfileProgram {
                plane,
                loops: vec![square.clone()],
            }),
        );
        doc = next;
        let (next, prism) = mint(
            &doc,
            Node::Extrude {
                profile,
                distance: len(THICKNESS),
            },
        );
        doc = next;
        plates.push(prism);
    }

    let ev: Evaluation<f64> = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let refs = vec![
        cap(&ev, plates[0], CapEnd::Top),
        cap(&ev, plates[1], CapEnd::Bottom),
    ];
    let expr = MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 });
    let (doc, measure) = mint(&doc, Node::measure(expr, refs).expect("indices in range"));

    let ev: Evaluation<f64> = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev.result(measure) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => Ok(*value),
            other => Err(format!("not a measure: {}", other.kind_name())),
        },
        _ => Err(ev
            .node_error(measure)
            .map_or_else(|| "not evaluated".to_owned(), |e| e.kind.to_string())),
    }
}

/// **The amendment's own example, and the row the old arm failed.**
///
/// Two 20 mm plates 10 mm apart, tilted by an angle chosen so that the
/// SHIPPED lever calls it a coincidence and the OLD one called it a
/// definite non-parallelism:
///
/// * shipped: `theta * arm_of(10 mm) <= eps`, so the pair IS parallel at
///   this tolerance and the measure answers a number;
/// * old (`max(separation, 1 m)`, which is the constant 1 m for a part
///   this size): `theta * 1 m >= K*eps`, so the same tilt escalated past
///   the band and the measure refused — priced across a metre the part
///   does not span.
///
/// Both hold for any `theta` in `(K*eps, eps / arm_of(10 mm))`, a
/// nonempty interval because `K = 10` is well under
/// `1 / 0.0383 ~ 26`; the row takes the geometric middle, which keeps a
/// factor of ~1.6 of headroom on each side at every epsilon.
#[test]
fn a_small_part_tilted_below_its_own_extent_reads_parallel() {
    let tol = Tol::witness();
    let arm = arm_of(10.0e-3);
    let (lo, hi) = (tol.k() * tol.eps(), tol.eps() / arm);
    assert!(
        lo < hi,
        "this run's band leaves no tilt the shipped lever calls parallel and the old one \
         did not: K = {} has to be under 1/arm = {}",
        tol.k(),
        1.0 / arm
    );
    let theta = (lo * hi).sqrt();
    let d = measures(10.0e-3, theta).unwrap_or_else(|e| {
        panic!(
            "a {theta} rad tilt across a 20 mm plate is a coincidence at this run's eps \
             ({}): `distance` of a plane face against a plane face needs them parallel, \
             and the shipped lever prices this tilt at {} m, at or under that eps — so \
             the LEVER is what to fix if this reds, not the tilt: {e}",
            tol.eps(),
            theta * arm
        )
    });
    // The lower plate's TOP cap sits at `THICKNESS`, the upper plate's
    // BOTTOM cap at `THICKNESS + SEPARATION`: the air between them.
    assert!(
        (d - SEPARATION).abs() < 1.0e-6,
        "the distance is the authored air gap: {d}"
    );
}

/// **The other direction, which the lever must not lose.** The same pair
/// at 45° is not parallel by any lever, and the measure refuses typed
/// rather than reporting a number whose meaning depends on an undecided
/// fact. This is the amendment's "two planes crossing at 45° do NOT
/// certify parallel", with the crossing put where the old
/// separation-only reading was wrong.
#[test]
fn a_large_tilt_still_refuses_typed() {
    let e = measures(10.0e-3, std::f64::consts::FRAC_PI_4)
        .expect_err("two planes at 45 degrees are not parallel");
    assert!(
        e.contains("bool_plane_parallel"),
        "the refusal names the predicate that decided it: {e}"
    );
}

/// **The lever is the EXTENT, not the separation** — the falsifier that
/// tells the shipped arm from a separation-only one.
///
/// Two 200 mm plates 10 mm apart: the extent is nearly thirty times the
/// separation, which is past the band's own width, so a tilt exists that
/// the EXTENT decides and the SEPARATION calls coincident. The two
/// conditions, again solved against this run's band rather than written
/// down:
///
/// * shipped: `theta * arm_of(200 mm) >= K*eps`, past the escalation
///   threshold, so the measure refuses;
/// * a separation-only lever: `theta * SEPARATION <= eps`, under the
///   coincidence threshold, so it would have reported a number.
///
/// Both hold for any `theta` in
/// `(K*eps / arm_of(200 mm), eps / SEPARATION)`, nonempty because the
/// arm is `arm_of(200 mm) / SEPARATION ~ 29` times the separation and
/// `K` is 10; the row takes the geometric middle, ~1.7 of headroom on
/// each side at every epsilon.
#[test]
fn a_tilt_only_the_extent_can_decide_is_decided() {
    let tol = Tol::witness();
    let arm = arm_of(100.0e-3);
    let (lo, hi) = (tol.k() * tol.eps() / arm, tol.eps() / SEPARATION);
    assert!(
        lo < hi,
        "this run's band leaves no tilt that tells the two levers apart: the arm is only \
         {} times the separation and K is {}",
        arm / SEPARATION,
        tol.k()
    );
    let theta = (lo * hi).sqrt();
    let e = match measures(100.0e-3, theta) {
        Err(e) => e,
        Ok(d) => panic!(
            "a {theta} rad tilt across 200 mm plates is a definite non-parallelism at this \
             run's eps ({}): the shipped lever prices it at {} m, past K*eps = {}, so the \
             measure has to refuse rather than report {d}",
            tol.eps(),
            theta * arm,
            tol.k() * tol.eps()
        ),
    };
    assert!(
        e.contains("bool_plane_parallel"),
        "the refusal names the predicate that decided it: {e}"
    );
}
