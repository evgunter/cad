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
    let square = LoopProgram::polygon([
        (-half, -half),
        (half, -half),
        (half, half),
        (-half, half),
    ])
    .expect("finite corners");
    let mut plates = Vec::new();
    for (z, v) in [
        (0.0, [scl(0.0), scl(1.0), scl(0.0)]),
        (
            THICKNESS + SEPARATION,
            [scl(0.0), scl(c), scl(s)],
        ),
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
/// Two 20 mm plates 10 mm apart, tilted by 1e-8 rad. Across their own
/// extent the tilt induces a deviation of a few times 1e-10 m — under
/// this run's coincidence threshold — so the pair IS parallel at this
/// tolerance and the measure answers a number. Under
/// `max(separation, 1 m)` the same tilt was priced across a metre the
/// part does not span, gave 1e-8 m — exactly the escalation threshold —
/// and the measure refused.
#[test]
fn a_small_part_tilted_below_its_own_extent_reads_parallel() {
    let d = measures(10.0e-3, 1.0e-8).unwrap_or_else(|e| {
        panic!("a 1e-8 rad tilt across a 20 mm plate is a coincidence at this run's eps: {e}")
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
/// Two 200 mm plates 10 mm apart: the extent is more than twenty times
/// the separation, which is past the band's own width, so a tilt exists
/// that the extent decides and the separation calls coincident. At
/// 6e-8 rad the deviation across the plates is ~1.8e-8 m — past the
/// escalation threshold — while across the separation alone it is
/// ~6.6e-10 m, under the coincidence one. The measure refuses, which a
/// separation-levered arm would not.
#[test]
fn a_tilt_only_the_extent_can_decide_is_decided() {
    let e = measures(100.0e-3, 6.0e-8)
        .expect_err("a 6e-8 rad tilt across 200 mm plates is a definite non-parallelism");
    assert!(
        e.contains("bool_plane_parallel"),
        "the refusal names the predicate that decided it: {e}"
    );
}
