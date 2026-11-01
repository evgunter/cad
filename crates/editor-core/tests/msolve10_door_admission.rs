//! MSOLVE-10 acceptance — **a constraint the recipe refuses on its own
//! is refused at the edit door**, through the solve's own per-constraint
//! admission (the spec is `docs/MSOLVE-10-SPEC.md`).
//!
//! The door decides of a constraint what its recipe alone decides: a
//! mate's `on` reads the copy its placement defines
//! (`MateFault::NotOnTheCopy`), its two poses are of one kind
//! (`MateFault::KindsDiffer`); a value's
//! freedom is one its bundle's mates leave, else it fixes an equation
//! they already fix (`MateFault::Overconstrained`). It refuses
//! `EditError::MateRefused` carrying the fault unaltered, and the load
//! door asks the same of every constraint a snapshot holds. THE DOORS
//! DECIDE EDITS AND THE EVALUATION DECIDES STATES: what needs the poses'
//! values — a contradiction, a face that stops resolving — is the
//! placement's own refusal where it evaluates, so the door reads no
//! face.
//!
//! These rows go through ordinary doors with a `PartStore`: each
//! refusal at insert, a refused insert leaves no entry, replay round-trips what the recording door
//! admitted and refuses what the door refuses, a doctored snapshot is
//! refused at load in the door's words, and the door admits again every
//! constraint of a corpus whatever its evaluation says.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::wire;
use editor_core::Dimension;
use editor_core::Formula;

use editor_core::{
    CapEnd, Coord, DocEdit, DocumentId, EditError, EvalOptions, MateFault, MateRef, Node,
    PersistError, ProfileDoc, RecipeNodeId, SnapshotError, TargetAxis, VarKind, load, save,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, at_the_door_of, insert, insert_pinned, len, on_frame};
use geom_core::Tol;

// ---- Substrate ----

/// A box part: a square of half-side `half` at the origin, extruded
/// `height` along +z; and its body.
fn box_part(label: &str, half: f64, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, half)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

/// `n` instances of one box part, the options that resolve them, and
/// the part's body. The first is pinned to the world; the rest sit
/// where their mates put them.
fn instances(label: &str, n: usize) -> (ProfileDoc, Vec<RecipeNodeId>, EvalOptions, RecipeNodeId) {
    let mut store = PartStore::new();
    let (doc_ref, body) =
        store.insert_part(box_part(&format!("{label}-part"), 0.5, 1.0), Tol::witness());
    let opts = with_resolver(store);
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..n {
        let (next, id) = if i == 0 {
            insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &opts)
        } else {
            insert(doc, fixture::mated_instance(doc_ref))
        };
        doc = next;
        ids.push(id);
    }
    (doc, ids, opts, body)
}

/// A face of instance `of`'s copy of the part whose body is `body`.
fn cap(of: RecipeNodeId, body: RecipeNodeId, end: CapEnd) -> editor_core::SitedFace {
    fixture::head_at(of, in_part(of, body, end))
}

/// `b`'s bottom cap resting on `a`'s top cap, both instances of the
/// part whose body is `body`: a mate `b`'s bundle holds.
fn mate(body: RecipeNodeId, a: RecipeNodeId, b: RecipeNodeId) -> editor_core::Mate<Formula> {
    fixture::rest(cap(b, body, CapEnd::Start), cap(a, body, CapEnd::End))
}

/// `b`'s bottom cap's plane equated with the point of `a`'s top cap: a
/// plane mated to a point, which the door refuses by kind.
fn kinds_differ(
    body: RecipeNodeId,
    a: RecipeNodeId,
    b: RecipeNodeId,
) -> editor_core::Mate<Formula> {
    fixture::mate(
        fixture::plane(cap(b, body, CapEnd::Start)),
        fixture::point_of(cap(a, body, CapEnd::End)),
    )
}

/// `mate` alone, as a bundle: what the door asks of a mate before it
/// asks the bundle to pin its copy.
fn lone(mate: editor_core::Mate<Formula>) -> Vec<editor_core::Constraint<Formula>> {
    vec![editor_core::Constraint::of_mate(
        editor_core::ConstraintId::placeholder(1),
        mate,
    )]
}

/// The edit that writes `mate` alone as `placement`'s bundle — one the
/// door refuses on the mate's own datum before it reads the bundle.
fn add(
    placement: RecipeNodeId,
    mate: editor_core::Mate<Formula>,
) -> DocEdit<editor_core::ProfileProgram> {
    DocEdit::SetConstraints {
        placement,
        constraints: lone(mate),
        fresh: Vec::new(),
    }
}

/// The edit that writes `mate` as `placement`'s bundle with each
/// freedom it leaves set to 0 ([`fixture::squared`]), so it pins.
fn seat(
    doc: &ProfileDoc,
    placement: RecipeNodeId,
    mate: editor_core::Mate<Formula>,
) -> DocEdit<editor_core::ProfileProgram> {
    DocEdit::SetConstraints {
        placement,
        constraints: fixture::squared(doc, mate, &[]).bundle(),
        fresh: Vec::new(),
    }
}

/// `constraints` written onto `placement` whole through the door: the
/// document and the constraints it minted, or the placement and fault
/// the door refused with.
#[allow(clippy::result_large_err)]
fn whole_door(
    doc: &ProfileDoc,
    placement: RecipeNodeId,
    constraints: Vec<editor_core::Constraint<Formula>>,
) -> Result<(ProfileDoc, Vec<MateRef>), (RecipeNodeId, MateFault)> {
    match doc.apply(
        &DocEdit::SetConstraints {
            placement,
            constraints,
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Ok(applied) => {
            let ids = applied
                .record
                .minted_constraints
                .iter()
                .map(|&id| MateRef::new(placement, id))
                .collect();
            Ok((applied.doc, ids))
        }
        Err(EditError::MateRefused { node, fault, .. }) => Err((node.id(), *fault)),
        Err(other) => panic!("the door refused otherwise: {other:?}"),
    }
}

/// A value of `coord`, by `by`, at placeholder `at` of a bundle written
/// whole.
fn value_at(at: u32, coord: Coord, by: Formula) -> editor_core::Constraint<Formula> {
    editor_core::Constraint::of_value(
        editor_core::ConstraintId::placeholder(at),
        editor_core::Value { coord, by },
    )
}

// ---- A1: the door's refusal, naming the placement ----

/// **A mate of two kinds refuses at insert with the admission's own
/// fault**: `MateRefused { KindsDiffer { on: Plane, to: Point } }`
/// naming the constraint the add would have minted, in the bundle of
/// the copy it places — and none the document holds. The refusal's
/// sentence names the placement once and forwards the fault's.
#[test]
fn a1_a_mate_of_two_kinds_refuses_at_insert_naming_the_placement() {
    let (doc, ids, _, body) = instances("msolve10-a1-kinds", 2);
    let (named, fault) =
        whole_door(&doc, ids[1], lone(kinds_differ(body, ids[0], ids[1]))).expect_err("refused");
    let MateFault::KindsDiffer { mate: held, on, to } = fault else {
        panic!("expected the kinds to differ, got {fault:?}");
    };
    assert_eq!(
        (named, held.placement),
        (ids[1], ids[1]),
        "the door names the placement the constraint places"
    );
    assert_eq!((on, to), (VarKind::Plane, VarKind::Point));
    assert!(
        !doc.mint().has_constraint(held.id),
        "the named constraint is not the document's"
    );
    let err = doc
        .apply(
            &add(ids[1], kinds_differ(body, ids[0], ids[1])),
            Tol::witness(),
        )
        .expect_err("refused");
    let EditError::MateRefused { fault, .. } = &err else {
        panic!("{err:?}");
    };
    let sentence = err.to_string();
    let t = test_utils::refusal::tag(named.0.digest());
    // The door speaks with the document at hand: the constraint by its
    // place in the bundle; the fault alone, by its ordinal.
    let own = "constraint 1 of this node equates an `on` pose of kind plane with a `to` pose of kind point";
    assert!(
        sentence.contains(&format!(
            "InstantiatePart {t} is refused by the solve on its own datum: {own}"
        )) && fault.to_string().starts_with(&format!(
            "constraint {} of node {t} equates an `on` pose of kind plane with a `to` pose of kind \
             point",
            held.id
        )),
        "the door names the placement once, and the fault's own sentence names the \
         constraint: {sentence} / {fault}"
    );
    assert_eq!(sentence.matches(&t).count(), 1, "{sentence}");
}

/// **A mate whose `on` reads another copy refuses `NotOnTheCopy`**: the
/// same rest, added to the bundle of the copy its `to` reads, pins
/// nothing of that placement.
#[test]
fn a1_a_mate_whose_on_reads_another_copy_refuses_not_on_the_copy() {
    let (doc, ids, _, body) = instances("msolve10-a1-not-on", 2);
    let (named, fault) =
        at_the_door_of(&doc, ids[0], mate(body, ids[0], ids[1])).expect_err("refused");
    assert_eq!(named, ids[0]);
    assert!(
        matches!(fault, MateFault::NotOnTheCopy { mate } if mate.placement == ids[0]),
        "{fault:?}"
    );
}

/// **A value is asked of its bundle, at the door** (row 190: a value
/// names no mate): a plane mate leaves two slides, so a third is a
/// freedom it fixes and the bundle refuses `Overconstrained`, naming the
/// three slides and the mate; a ball's rotation is a chain of spins about the
/// target space's axes, admitted; and a spin about an axis mate whose
/// target is the line two copies' caps meet in is admitted too, since
/// the chart is the target space's whatever bodies the target reads.
///
/// *Breaks if* a value's freedom is admitted beside a mate that fixes
/// it, or a value is charted on a body its mate's target reads.
#[test]
fn a1_a_value_is_admitted_by_its_bundle() {
    let (doc, ids, _, body) = instances("msolve10-a1-values", 3);
    let plane = mate(body, ids[0], ids[1]);
    let mut bundle = lone(plane);
    for (at, axis) in [(2, TargetAxis::X), (3, TargetAxis::Y), (4, TargetAxis::Z)] {
        bundle.push(value_at(at, Coord::Slide(axis), len(0.25)));
    }
    let (_, fault) = whole_door(&doc, ids[1], bundle).expect_err("refused");
    assert!(
        matches!(&fault, MateFault::Overconstrained { constraints, .. } if constraints.len() == 4),
        "{fault:?}"
    );

    // A ball's rotation, written beside it as a chain.
    let ball = fixture::mate(
        fixture::point_of(cap(ids[2], body, CapEnd::Start)),
        fixture::point_of(cap(ids[1], body, CapEnd::End)),
    );
    let mut bundle = lone(ball);
    for (at, axis) in (2..).zip(TargetAxis::ALL) {
        bundle.push(value_at(at, Coord::Spin(Some(axis)), ang(0.1)));
    }
    whole_door(&doc, ids[2], bundle).expect("a ball's chain of spins pins it");

    // An axis mate whose target is the line two copies' faces meet in.
    let (doc, lines, _, body) = instances("msolve10-a1-two-bodies", 3);
    let meet = editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Meet {
        a: editor_core::pose::RoleRead::plain(fixture::plane(cap(lines[0], body, CapEnd::End))),
        b: editor_core::pose::RoleRead::plain(fixture::plane(cap(lines[1], body, CapEnd::End))),
    }));
    let mut bundle = lone(fixture::mate(
        fixture::axis(cap(lines[2], body, CapEnd::Start)),
        meet,
    ));
    bundle.push(value_at(
        2,
        Coord::Slide(TargetAxis::Z),
        Formula::zero(Dimension::Length),
    ));
    bundle.push(value_at(3, Coord::SPIN, ang(0.1)));
    whole_door(&doc, lines[2], bundle).expect("a target over two bodies has one chart");
}

// ---- The history and the log ----

/// **A refused add leaves no entry**: the document is the one it was,
/// and the next add mints one constraint id, not the refused one's.
#[test]
fn a1_a_refused_insert_leaves_no_entry_in_the_history() {
    let (doc, ids, _, body) = instances("msolve10-a1-history", 2);
    let before = doc.clone();
    let (named, fault) =
        whole_door(&doc, ids[1], lone(kinds_differ(body, ids[0], ids[1]))).expect_err("refused");
    let MateFault::KindsDiffer { mate: refused, .. } = fault else {
        panic!("{fault:?}");
    };
    assert_eq!(named, ids[1]);
    assert_eq!(doc.ids(), before.ids());
    assert_eq!(
        fixture::bundle(&doc, named),
        fixture::bundle(&before, named),
        "the bundle holds what it held"
    );
    let (after, minted) = fixture::constrain(doc, mate(body, ids[0], ids[1]));
    assert_ne!(minted, refused, "another mate, another id");
    assert!(
        !after.mint().has_constraint(refused.id),
        "nothing was minted for the refusal"
    );
    assert_eq!(after.ids(), before.ids());
    assert_eq!(
        fixture::bundle(&after, ids[1]).len(),
        4,
        "the rest and the three values it leaves"
    );
    let (unasked, _) = fixture::constrain(before, mate(body, ids[0], ids[1]));
    assert_eq!(
        after.mint(),
        unasked.mint(),
        "the refusal left the chain and the log as though it was never asked"
    );
}

/// **Replay re-applies what the recording door admitted, and refuses
/// what the door refuses**: a log whose entry added a face-to-face rest
/// round-trips through `save` and `load` with no store, while a
/// hand-edited entry mating a plane to a point refuses at `load` as
/// `EditReplay { index, MateRefused { KindsDiffer } }`, naming the
/// entry.
#[test]
fn a3_replay_round_trips_an_admitted_mate_and_refuses_a_door_refusal_at_load() {
    let (doc, ids, _, body) = instances("msolve10-a3-log", 2);
    let snapshot = doc.clone();
    let edit = seat(&doc, ids[1], mate(body, ids[0], ids[1]));
    let applied = doc.apply(&edit, Tol::witness()).expect("admitted");
    let log = vec![edit.clone()];
    let text = save(&snapshot, &log, Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("a log the door admitted loads with no store");
    assert_eq!(loaded.doc.ids(), applied.doc.ids());
    assert_eq!(
        fixture::bundle(&loaded.doc, ids[1]),
        fixture::bundle(&applied.doc, ids[1])
    );
    assert_eq!(loaded.edits, log);

    // The hand-edited entry, spliced in as a bare entry at index 1.
    let refused = add(ids[1], kinds_differ(body, ids[0], ids[1]));
    let refused_wire = serde_json::to_value(&refused).expect("an entry serializes");
    let doctored = wire::doctored(&text, |wire| {
        wire["edits"]
            .as_array_mut()
            .expect("the log is a list")
            .push(refused_wire);
    });
    let err = load(&doctored, Tol::witness()).expect_err("the door refuses this mate");
    let PersistError::EditReplay { index, error } = &err else {
        panic!("expected EditReplay, got {err:?}");
    };
    assert_eq!(*index, 1, "the entry is named");
    assert!(
        matches!(
            &**error,
            EditError::MateRefused { fault, .. }
                if matches!(**fault, MateFault::KindsDiffer { on: VarKind::Plane, to: VarKind::Point, .. })
        ),
        "{error:?}"
    );
    assert!(
        err.to_string().contains("edit 1 refused on replay"),
        "{err}"
    );
}

/// **A second rest refuses at the door, and on replay**: two rests on
/// one copy — its bottom cap and its top cap each resting on the same
/// face — fix the normal twice, which the door reads off the two
/// mates' kinds, measuring nothing (FORK-S3O). A log carrying the
/// second refuses to save, its replay refusing in the same words,
/// naming the entry.
#[test]
fn a3_a_second_rest_refuses_at_the_door_and_on_replay() {
    let (doc, ids, _, body) = instances("msolve10-a3-overconstrained", 2);
    let snapshot = doc.clone();
    let bottom = seat(&doc, ids[1], mate(body, ids[0], ids[1]));
    // The second rest added to the pinned bundle beside the first.
    let top = DocEdit::AddConstraint {
        placement: ids[1],
        mate: Box::new(fixture::rest(
            cap(ids[1], body, CapEnd::End),
            cap(ids[0], body, CapEnd::End),
        )),
        fresh: Vec::new(),
    };
    let doc = doc
        .apply(&bottom, Tol::witness())
        .expect("the door admits the first rest")
        .doc;
    let held = fixture::bundle(&doc, ids[1])[0].id;
    let refused = doc
        .apply(&top, Tol::witness())
        .expect_err("the door refuses the second rest");
    let EditError::MateRefused { fault, .. } = &refused else {
        panic!("refused as a mate: {refused:?}");
    };
    assert!(
        matches!(
            &**fault,
            MateFault::Overconstrained { placement, constraints, free: 0 }
                if *placement == ids[1] && constraints.len() == 2 && constraints.contains(&held)
        ),
        "{fault:?}"
    );
    let err = save(&snapshot, &[bottom, top], Tol::witness())
        .expect_err("a log carrying the second rest does not replay");
    let PersistError::EditReplay { index, error } = &err else {
        panic!("expected EditReplay, got {err:?}");
    };
    assert_eq!(*index, 1, "the entry is named");
    assert_eq!(**error, refused, "replay refuses in the door's words");
}

/// **A snapshot is data, and the load door decides it as the edit door
/// does**: a doctored snapshot whose rest has its two poses swapped —
/// `on` now reading the pinned copy — refuses at load
/// `SnapshotError::MateRefused { NotOnTheCopy }` naming the placement
/// and the constraint, and the edit door refuses the same mate added
/// fresh in the same words.
#[test]
fn a3_a_doctored_snapshot_whose_mate_is_not_on_its_copy_refuses_at_load() {
    let (doc, ids, _, body) = instances("msolve10-a3-snapshot", 2);
    let (doc, mate_id) = fixture::constrain(doc, mate(body, ids[0], ids[1]));
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    // The one mate: its `on` and `to` reads exchanged, so every
    // variable keeps its one reader.
    fn swap(v: &mut serde_json::Value, swapped: &mut usize) {
        match v {
            serde_json::Value::Object(map) => {
                if map.contains_key("on") && map.contains_key("to") {
                    let on = map["on"].clone();
                    map["on"] = map["to"].clone();
                    map["to"] = on;
                    *swapped += 1;
                }
                for value in map.values_mut() {
                    swap(value, swapped);
                }
            }
            serde_json::Value::Array(items) => {
                for item in items {
                    swap(item, swapped);
                }
            }
            _ => {}
        }
    }
    let mut swapped = 0_usize;
    let doctored = wire::doctored(&text, |w| swap(w, &mut swapped));
    assert_eq!(swapped, 1, "the snapshot's one mate was swapped");
    match load(&doctored, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::MateRefused {
            node,
            constraint,
            fault,
        })) => {
            assert_eq!((node.id(), constraint), (ids[1], mate_id.id));
            assert_eq!(*fault, MateFault::NotOnTheCopy { mate: mate_id });
        }
        other => panic!("the load door refuses the swapped mate: {other:?}"),
    }
    let swapped_mate = fixture::mate(
        fixture::flip(fixture::plane(cap(ids[0], body, CapEnd::End))),
        fixture::plane(cap(ids[1], body, CapEnd::Start)),
    );
    let (named, door) = at_the_door_of(&doc, ids[1], swapped_mate).expect_err("refused");
    assert_eq!(named, ids[1]);
    assert!(
        matches!(door, MateFault::NotOnTheCopy { mate } if mate.placement == ids[1]),
        "the door's word is the load's: {door:?}"
    );
}

// ---- A2: the door re-admits every constraint of a corpus ----

/// Whether `fault` is one the door decides: a fact about one
/// constraint's recipe, which no evaluation raises.
fn a_door_fault(fault: &MateFault) -> bool {
    matches!(
        fault,
        MateFault::NotOnTheCopy { .. }
            | MateFault::KindsDiffer { .. }
            | MateFault::NoChart { .. }
            | MateFault::Overconstrained { .. }
    )
}

/// One corpus document: its label, the document, and the options its
/// evaluation reads through.
type Row = (&'static str, ProfileDoc, EvalOptions);

/// **The corpus**: shapes of constraint a document can hold, through
/// the doors that admit them — a sound pair, a chain, a value on a
/// mate, and a state the
/// evaluation refuses: a target stranded past a pattern's count after
/// the mate is in.
fn corpus() -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    rows.push({
        let (doc, ids, opts, body) = instances("msolve10-corpus-sound", 2);
        let (doc, _) = fixture::constrain(doc, mate(body, ids[0], ids[1]));
        ("msolve10-corpus-sound", doc, opts)
    });
    rows.push({
        let (doc, ids, opts, body) = instances("msolve10-corpus-chain", 3);
        let (doc, _) = fixture::constrain(doc, mate(body, ids[0], ids[1]));
        let (doc, _) = fixture::constrain(doc, mate(body, ids[1], ids[2]));
        ("msolve10-corpus-chain", doc, opts)
    });
    rows.push({
        let (doc, ids, opts, body) = instances("msolve10-corpus-value", 2);
        // A plane mate leaves its spin.
        let (doc, _) = fixture::mated(
            doc,
            ids[1],
            mate(body, ids[0], ids[1]),
            &[(Coord::SPIN, ang(0.3))],
        );
        ("msolve10-corpus-value", doc, opts)
    });
    rows.push({
        // A target stranded past a family's count, after insert: the
        // member the name says no longer exists.
        let (doc, ids, opts, body) = instances("msolve10-corpus-stranded", 2);
        let (doc, k, pattern) = crate::fixture::linear(doc, ids[0], [1.0, 0.0, 0.0], 3.0, 3);
        let (doc, _) = fixture::constrain_at(
            doc,
            ids[1],
            fixture::rest(
                cap(ids[1], body, CapEnd::Start),
                fixture::head(fixture::in_copy(
                    pattern,
                    k,
                    2,
                    in_part(ids[0], body, CapEnd::End),
                )),
            ),
        );
        let doc = fixture::recount(doc, k, editor_core::Formula::count(2));
        ("msolve10-corpus-stranded", doc, opts)
    });
    rows
}

/// **The door admits again every constraint of the corpus, whatever its
/// evaluation says**: each bundle, written again whole as written, is
/// admitted — the door decides only the recipe, and every
/// constraint of a document passed it. Every fault the evaluation
/// records is one about the poses' values, never a fact the door
/// decides.
#[test]
fn a2_the_door_admits_every_constraint_of_the_corpus_again() {
    let (mut admitted, mut refused_where_evaluated) = (0_usize, 0_usize);
    for (label, doc, opts) in corpus() {
        let poses = fixture::solve(&doc, &opts, Tol::witness());
        let mut constraints = 0_usize;
        for placement in doc.placements() {
            if let Some(fault) = poses.fault(placement) {
                assert!(
                    !a_door_fault(fault),
                    "{label}: the evaluation raised a fault the door decides: {fault:?}"
                );
                refused_where_evaluated += 1;
            }
            // Every bundle pins its copy, so it is re-admitted whole:
            // the bundle as written, written again over itself.
            let Some(
                Node::Place {
                    constraints: written,
                    ..
                }
                | Node::InstantiatePart {
                    constraints: written,
                    ..
                },
            ) = doc.node(placement).map(|node| node.written(&doc))
            else {
                panic!("{label}: {placement:?} is a placement");
            };
            constraints += written.len();
            let twin = whole_door(&doc, placement, written);
            assert!(
                twin.is_ok(),
                "{label}: the door refused the twin of {placement:?}'s bundle: {:?}",
                twin.err()
            );
            admitted += 1;
        }
        assert!(
            constraints > 0,
            "{label}: a corpus document holds a constraint"
        );
    }
    // Floors, not counts: the corpus can grow, the claim cannot go
    // vacuous.
    assert!(
        admitted >= 6 && refused_where_evaluated >= 1,
        "the corpus re-admitted {admitted} constraints over {refused_where_evaluated} \
         placements refused where they evaluate"
    );
}
