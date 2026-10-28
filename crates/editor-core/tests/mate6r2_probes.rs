//! MATE-6 R2 probes — the refusal-precedence and seam rows that
//! review left behind.
//!
//! Each prints a `P<n>:`-tagged line; the rows were written to compile
//! on the MATE-6 head AND on its merge base, so the review could diff
//! the tagged lines. **That property is spent** — the branch merged —
//! and four of the eight rows still assert nothing, so their printed
//! answers are unguarded;
//! `work/vacuity/mate6r1-shared-has-eleven-tests-and-no-assertions.md`
//! owns that. What the rows are FOR now is what each one says below.
//!
//! P1 — a bad reference fails its placement, and that failure is the
//!         gate's answer ahead of the at-rest census.
//! P3 — the checks resident over the ×3-stand seam document.
//! P4 — the checks resident over a single correctly-mated document.
//! P6 — carried PENETRATING geometry (claim 5): loud on some arm,
//!         never a silent pass.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Dimension;
use editor_core::Formula;

use editor_core::{
    CapEnd, ChecksConfig, Coord, DocRef, DocumentId, EntityKind, Frame, MateRef, Node, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, TargetAxis, run_checks,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, mated_instance, on_frame, run};
use geom_core::Tol;

fn block(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
        },
    )
}

fn cube_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    block(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        (0.0, 1.0),
        (0.0, 1.0),
        0.0,
        1.0,
    )
}

/// A reference whose inner name answers to nothing of the part —
/// `RecipeNodeId(99)` has no face — so a pose read off it does not
/// evaluate, and its placement refuses.
fn vanished(instance: RecipeNodeId) -> crate::fixture::resolver::PartName {
    crate::fixture::resolver::PartName {
        at: instance,
        name: StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, 99),
            path: vec![RoleSeg::Cap(CapEnd::End)],
        },
    }
}

/// A rest of `b`'s face on `a`'s, standing `b` `apart` beside `a`
/// ([`Seat`]).
fn mate_node(a: impl crate::fixture::Sited, b: impl crate::fixture::Sited, apart: f64) -> Seat {
    Seat {
        mate: fixture::rest(fixture::head(b), fixture::head(a)),
        apart,
    }
}

/// `b` sunk `depth` into `a`: `b`'s face on `a`'s plane slid `-depth`
/// along its own normal, facing it. A plane constructed off a face
/// declares no contact, so the overlap is geometry nothing declares.
fn sunk(a: impl crate::fixture::Sited, b: impl crate::fixture::Sited, depth: f64) -> Seat {
    let under = fixture::plane(fixture::head(a));
    let under = editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Slide {
        pose: editor_core::pose::RoleRead::plain(under.clone()),
        along: editor_core::pose::RoleRead::through(under, editor_core::pose::PoseRole::Normal),
        by: len(-depth),
    }));
    Seat {
        mate: fixture::mate(fixture::plane(fixture::head(b)), fixture::flip(under)),
        apart: 0.0,
    }
}

/// **A rest and how far beside its target it stands**: the copy's
/// own origin slid `apart` along x from the target's in the rested
/// plane. `0.0` stands the two unit cubes square, face to face; `2.0`
/// leaves the faces coplanar with a unit of clear air between their
/// trims, so the declaration is FALSE.
struct Seat {
    mate: editor_core::Mate<Formula>,
    apart: f64,
}

/// `seat`'s mate added to `placement`'s bundle with the three freedoms
/// a plane mate leaves set: a slide of `apart` along x, none along y,
/// and no spin, so it pins the copy.
fn seated_at(doc: ProfileDoc, placement: RecipeNodeId, seat: Seat) -> (ProfileDoc, MateRef) {
    let (doc, m) = crate::fixture::mated(
        doc,
        placement,
        seat.mate,
        &[
            (Coord::Slide(TargetAxis::X), len(seat.apart)),
            (
                Coord::Slide(TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    (doc, m)
}

/// [`seated_at`] on the placement the seat's `on` reads.
fn seated(doc: ProfileDoc, seat: Seat) -> (ProfileDoc, MateRef) {
    let placement = fixture::placed_by(&doc, &seat.mate);
    seated_at(doc, placement, seat)
}

/// Two instances of `part`, whose body is `body`: the first pinned to
/// the world, the second stood on it by `seat`'s mate.
fn stand_with(
    label: &str,
    part: DocRef,
    body: RecipeNodeId,
    store: &PartStore,
    seat: impl FnOnce(crate::fixture::resolver::PartName, crate::fixture::resolver::PartName) -> Seat,
) -> (ProfileDoc, Vec<RecipeNodeId>, MateRef) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, lower) = fixture::insert_pinned(
        doc,
        crate::fixture::mated_instance(part),
        &with_resolver(store.clone()),
    );
    let (doc, upper) = insert(doc, mated_instance(part));
    let (doc, mate) = seated(
        doc,
        seat(
            in_part(lower, body, CapEnd::End),
            in_part(upper, body, CapEnd::Start),
        ),
    );
    (doc, vec![lower, upper], mate)
}

/// [`stand_with`] a `Rest` seat, the upper cube `apart` beside the
/// lower.
fn stand(
    label: &str,
    part: DocRef,
    body: RecipeNodeId,
    apart: f64,
    store: &PartStore,
) -> (ProfileDoc, Vec<RecipeNodeId>, MateRef) {
    stand_with(label, part, body, store, |a, b| mate_node(a, b, apart))
}

/// `count` instances of `part`, each pinned `spacing * i` along +x.
fn row_of(
    label: &str,
    part: DocRef,
    count: usize,
    spacing: f64,
    store: &PartStore,
) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let o = with_resolver(store.clone());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..count {
        let (next, id) = insert(doc, mated_instance(part));
        #[allow(clippy::cast_precision_loss)]
        let dx = spacing * i as f64;
        doc = fixture::pin(next, id, &Frame::translation([dx, 0.0, 0.0]), &o);
        ids.push(id);
    }
    (crate::fixture::assemble_world_copies(doc), ids)
}

/// A one-line, cross-tree-comparable rendering of the at-rest check's
/// answer.
fn headline(result: &Result<Vec<editor_core::CheckFinding>, editor_core::ChecksError>) -> String {
    match result {
        Ok(findings) => format!(
            "at rest: findings={} could-not-look={}",
            findings.len(),
            crate::fixture::could_not_look(findings).len()
        ),
        Err(other) => format!("{other:?}").chars().take(80).collect(),
    }
}

/// A stand whose upper cube stands two units beside the lower, and two
/// more copies of the cube, each placed on the stand's lower cube: one
/// by a bad-reference mate, one by a mate two units beside it, their
/// placements in `order` (the bad reference first when `badref_first`).
/// Answers the document and the placement the bad reference is in.
fn stand_with_two_bad_mates(
    label: &str,
    store: &mut PartStore,
    badref_first: bool,
) -> (ProfileDoc, RecipeNodeId) {
    let (part, body) = store.insert_part(cube_part(&format!("{label}-cube")), Tol::witness());
    let (doc, ids, _) = stand(&format!("{label}-stand"), part, body, 2.0, store);
    let (doc, first) = insert(doc, mated_instance(part));
    let (doc, second) = insert(doc, mated_instance(part));
    let (badref, beside) = if badref_first {
        (first, second)
    } else {
        (second, first)
    };
    let (doc, _) = seated(
        doc,
        mate_node(vanished(ids[0]), in_part(badref, body, CapEnd::Start), 2.0),
    );
    let (doc, _) = seated(
        doc,
        mate_node(
            in_part(ids[0], body, CapEnd::End),
            in_part(beside, body, CapEnd::Start),
            2.0,
        ),
    );
    (doc, badref)
}

/// The gather's answer over a document whose bad reference failed its
/// placement: the gather refuses the product's `⊔`, poisoned through
/// that placement, typed, ahead of the at-rest census, and the
/// placement's own refusal is the mate's.
fn refuses_the_failed_placement(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    badref: RecipeNodeId,
) {
    let result = editor_core::product_recorded(doc, ev, Tol::witness()).map(|_| ());
    match &result {
        Err(refusal) => assert!(
            matches!(
                *refusal,
                editor_core::ProductError::Read(editor_core::NodeStanding::Poisoned {
                    through,
                    ..
                }) if through == badref
            ),
            "the failed placement is the gate's answer: {refusal:?}"
        ),
        Ok(()) => panic!("expected the failed placement, got a product"),
    }
    assert!(
        ev.node_error(badref).is_some(),
        "the bad reference's refusal is its placement's"
    );
}

/// P1: a stand, plus a bad-reference mate, plus a second mate, their
/// placements in that document order. The bad reference fails its
/// placement, and that failure is the gate's answer.
#[test]
fn p1_badref_first_refuses_its_failed_placement() {
    let mut store = PartStore::default();
    let (doc, badref) = stand_with_two_bad_mates("m6r2-p1", &mut store, true);
    let ev = run(&doc, &with_resolver(store));
    println!("P1: {}", headline(&crate::fixture::at_rest(&doc, &ev)));
    refuses_the_failed_placement(&doc, &ev, badref);
}

/// P3: the checks resident over the seam document (×3 stands).
#[test]
fn p3_checks_over_the_seam_document() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("m6r2-p3-cube"), Tol::witness());
    let (inner, _, _) = stand("m6r2-p3-stand", part, body, 0.0, &store);
    let inner = crate::fixture::assemble_world_copies(inner);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("m6r2-p3-row", inner_ref, 3, 4.0, &store);
    let ev = run(&outer, &with_resolver(store));
    let report =
        run_checks(&outer, &ev, &ChecksConfig::default(), Tol::witness()).expect("the checks run");
    println!(
        "P3: findings={} {:?}",
        report.findings.len(),
        report.findings
    );
}

/// P4: the checks resident over the single correctly-mated stand.
#[test]
fn p4_checks_over_a_correctly_mated_document() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("m6r2-p4-cube"), Tol::witness());
    let (doc, _, _) = stand("m6r2-p4-stand", part, body, 0.0, &store);
    let ev = run(&doc, &with_resolver(store));
    let report =
        run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness()).expect("the checks run");
    println!(
        "P4: findings={} {:?}",
        report.findings.len(),
        report.findings
    );
}

/// P6: carried PENETRATING geometry (the inner upper cube sunk half a
/// unit into the lower, by a standoff that declares nothing): whatever
/// arm fires, the outer document must not pass silently: it carries the
/// part's overlap, which the part's census decided off the sunk cube's
/// walls lying on the lower's, as a loud interference.
#[test]
fn p6_carried_penetration_is_loud() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("m6r2-p6-cube"), Tol::witness());
    let (inner, _, _) = stand_with("m6r2-p6-stand", part, body, &store, |a, b| sunk(a, b, 0.5));
    let inner = crate::fixture::assemble_world_copies(inner);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("m6r2-p6-row", inner_ref, 1, 4.0, &store);
    let ev = run(&outer, &with_resolver(store));
    let result = crate::fixture::at_rest(&outer, &ev);
    println!("P6: {}", headline(&result));
    let findings = result.expect("a carried overlap is a finding, not a refusal");
    let carried: Vec<_> = findings
        .iter()
        .filter_map(|f| match &f.evidence {
            editor_core::CheckEvidence::CarriedInterference { finding } => Some(&finding.finding),
            _ => None,
        })
        .collect();
    assert!(
        !carried.is_empty()
            && carried.iter().all(|f| f.is_loud()
                && f.evidence
                    .iter()
                    .any(|e| matches!(e, topo::ValidationError::SameSideInterference { .. }))),
        "penetrating carried geometry is the part's loud, decided overlap: {carried:?}"
    );
}

/// P7: the outer gate over the ×3-stand seam document, counted by arm.
/// It prints what the at-rest check could not look at, by arm.
#[test]
fn p7_seam_gate_by_arm() {
    let mut store = PartStore::default();
    let (part, body) = store.insert_part(cube_part("m6r2-p7-cube"), Tol::witness());
    let (inner, _, _) = stand("m6r2-p7-stand", part, body, 0.0, &store);
    let inner = crate::fixture::assemble_world_copies(inner);
    let inner_ref = store.insert(inner, Tol::witness());
    let (outer, _) = row_of("m6r2-p7-row", inner_ref, 3, 4.0, &store);
    let ev = run(&outer, &with_resolver(store));
    match crate::fixture::at_rest(&outer, &ev) {
        Ok(findings) => {
            let blind = crate::fixture::could_not_look(&findings);
            let undeclared = blind
                .iter()
                .filter(|f| format!("{f:?}").contains("UnrecordedContact"))
                .count();
            println!(
                "P7: could-not-look={} undeclared={}",
                blind.len(),
                undeclared
            );
        }
        Err(other) => println!("P7: {other}"),
    }
}
