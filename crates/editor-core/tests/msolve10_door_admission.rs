//! MSOLVE-10 acceptance — **a constraint the recipe refuses on its own
//! is refused at the edit door**, through the solve's own per-constraint
//! admission (the spec is `docs/MSOLVE-10-SPEC.md`).
//!
//! The door decides of a constraint what its recipe alone decides: a
//! mate's `on` reads the copy its placement defines
//! (`MateFault::NotOnTheCopy`), its world read is a pose
//! (`MateFault::WorldRead`), its two poses are of one kind
//! (`MateFault::KindsDiffer`), and its class is admitted; a value names
//! a mate of its bundle (`MateFault::ValueNamesNoMate`), its mate's kind
//! leaves the freedom it sets (`MateFault::NotAFreedom`), and the
//! freedom has a chart (`MateFault::NoChart`). It refuses
//! `EditError::MateRefused` carrying the fault unaltered, and the load
//! door asks the same of every constraint a snapshot holds. THE DOORS
//! DECIDE EDITS AND THE EVALUATION DECIDES STATES: what needs the poses'
//! values — a contradiction, a face that stops resolving — is the
//! placement's own refusal where it evaluates, so the door asks no reach
//! and reads no face.
//!
//! These rows go through ordinary doors with a `PartStore`: each
//! refusal at insert, a refused insert leaves no entry, the door reads
//! nothing of the parts, replay round-trips what the recording door
//! admitted and refuses what the door refuses, a doctored snapshot is
//! refused at load in the door's words, and the door admits again every
//! constraint of a corpus whatever its evaluation says.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::wire;
use editor_core::ExtrudeSide;
use editor_core::Formula;

use editor_core::{
    CapEnd, ContactClass, Coord, DocEdit, DocumentId, EditError, EvalOptions, MateFault, MateReach,
    MateRef, Node, NodeErrorKind, NodeResult, PersistError, ProfileDoc, ReachRefusal, RecipeNodeId,
    RefusingReach, SlideAxis, SnapshotError, Target, VarKind, WorldRead, WorldReadFault, load,
    mate_reach, save,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, at_the_door, at_the_door_of, insert, insert_pinned, len, on_frame, step};
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
            side: ExtrudeSide::Along,
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
            insert_pinned(doc, Node::instantiate_part(doc_ref), &opts)
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
    fixture::head(in_part(of, body, end))
}

/// `b`'s bottom cap resting on `a`'s top cap, both instances of the
/// part whose body is `body`: a mate `b`'s bundle holds.
fn mate(body: RecipeNodeId, a: RecipeNodeId, b: RecipeNodeId) -> editor_core::Mate<Formula> {
    fixture::rest(
        cap(b, body, CapEnd::Start),
        cap(a, body, CapEnd::End),
        ContactClass::Rest,
    )
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
        ContactClass::Rest,
    )
}

/// **The constraint `at` names, as written**: every read and value
/// spelled as authored — an anonymous pose as its definition — ready to
/// be added again.
fn written_constraint(doc: &ProfileDoc, at: MateRef) -> editor_core::Constraint<Formula> {
    let node = doc
        .node(at.placement)
        .expect("a live placement")
        .written(doc);
    let (Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. }) = node else {
        panic!("{at:?} names a placement");
    };
    constraints
        .into_iter()
        .find(|c| c.id == at.id)
        .unwrap_or_else(|| panic!("{at:?} is a constraint of its placement"))
}

/// The edit that adds `mate` to `placement`'s bundle.
fn add(
    placement: RecipeNodeId,
    mate: editor_core::Mate<Formula>,
) -> DocEdit<editor_core::ProfileProgram> {
    DocEdit::AddConstraint {
        placement,
        mate: Box::new(mate),
        fresh: Vec::new(),
    }
}

/// The edit that adds a value of `coord` on `mate` to `placement`'s
/// bundle.
fn add_value(
    placement: RecipeNodeId,
    mate: editor_core::ConstraintId,
    coord: Coord,
    by: Formula,
) -> DocEdit<editor_core::ProfileProgram> {
    DocEdit::AddValue {
        placement,
        value: Box::new(editor_core::Value { mate, coord, by }),
        fresh: Vec::new(),
    }
}

/// `edit` through the door with the refusing reach: the document and
/// the constraint it minted, or the fault the door refused with.
#[allow(clippy::result_large_err)]
fn through_door(
    doc: &ProfileDoc,
    placement: RecipeNodeId,
    edit: &DocEdit<editor_core::ProfileProgram>,
) -> Result<(ProfileDoc, MateRef), MateFault> {
    match doc.apply(edit, Tol::witness(), &RefusingReach) {
        Ok(applied) => {
            let id = applied.record.minted_constraints[0];
            Ok((applied.doc, MateRef::new(placement, id)))
        }
        Err(EditError::MateRefused { fault, .. }) => Err(*fault),
        Err(other) => panic!("the door refused otherwise: {other:?}"),
    }
}

/// A reach that counts its asks and answers through `inner`.
struct Counting<'a>(core::cell::Cell<usize>, &'a dyn MateReach);

impl<'a> Counting<'a> {
    fn over(inner: &'a dyn MateReach) -> Self {
        Self(core::cell::Cell::new(0), inner)
    }

    /// How many asks were made.
    fn asks(&self) -> usize {
        self.0.get()
    }
}

impl MateReach for Counting<'_> {
    fn reach(&self, part: &editor_core::DocRef) -> Result<f64, ReachRefusal> {
        self.0.set(self.0.get() + 1);
        self.1.reach(part)
    }
}

// ---- A1: the door's refusal, naming the placement ----

/// **A mate of two kinds refuses at insert with the admission's own
/// fault**: `MateRefused { KindsDiffer { on: Plane, to: Point } }`
/// naming the constraint the add would have minted, in the bundle of
/// the copy it places — and none the document holds. The refusal's
/// sentence names the placement once and forwards the fault's.
#[test]
fn a1_a_mate_of_two_kinds_refuses_at_insert_naming_the_placement() {
    let (doc, ids, opts, body) = instances("msolve10-a1-kinds", 2);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let (named, fault) =
        at_the_door(&doc, &reach, kinds_differ(body, ids[0], ids[1])).expect_err("refused");
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
            &reach,
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
    let (named, fault) = at_the_door_of(&doc, &RefusingReach, ids[0], mate(body, ids[0], ids[1]))
        .expect_err("refused");
    assert_eq!(named, ids[0]);
    assert!(
        matches!(fault, MateFault::NotOnTheCopy { mate } if mate.placement == ids[0]),
        "{fault:?}"
    );
}

/// **A world read that is no pose refuses `WorldRead`** naming the
/// construction its kinds refuse: a flip of the world's origin, which
/// as a point has no sense.
#[test]
fn a1_a_world_read_that_is_no_pose_refuses_world_read() {
    let (doc, ids, _, body) = instances("msolve10-a1-world", 2);
    let flipped_origin = editor_core::Mate {
        on: fixture::point_of(cap(ids[1], body, CapEnd::Start)),
        to: Target::World(WorldRead::through(editor_core::pose::PoseRole::Origin).flip()),
        class: ContactClass::Rest,
    };
    let fault = fixture::door_refusal(&doc, flipped_origin);
    assert!(
        matches!(
            fault,
            MateFault::WorldRead {
                mate,
                fault: WorldReadFault::FlippedPoint,
            } if mate.placement == ids[1]
        ),
        "{fault:?}"
    );
}

/// **A world read nested past its bound refuses `TooDeep`** before its
/// kinds are read: the world's frame spun about z one turn more than
/// [`editor_core::MAX_WORLD_READ_DEPTH`] allows, read at its origin.
/// The read is written inside its mate, so the door's bound is what
/// holds a saved mate within the load door's nesting limit.
///
/// *Breaks if* the door admits a read deeper than the bound (a save
/// could then write a file the load door refuses), or reads the kinds
/// of one first.
#[test]
fn a1_a_world_read_past_its_bound_refuses_too_deep() {
    let (doc, ids, _, body) = instances("msolve10-a1-world-deep", 2);
    let mut world = WorldRead::Frame;
    // Each spin about z (two deep) adds one; the role adds one more.
    for _ in 0..editor_core::MAX_WORLD_READ_DEPTH - 2 {
        world = world.spin(
            WorldRead::through(editor_core::pose::PoseRole::Z),
            fixture::ang(0.0),
        );
    }
    let to = WorldRead::Role {
        pose: Box::new(world),
        role: editor_core::pose::PoseRole::Origin,
    };
    let depth = to.depth();
    assert_eq!(depth, editor_core::MAX_WORLD_READ_DEPTH + 1);
    let too_deep = editor_core::Mate {
        on: fixture::point_of(cap(ids[1], body, CapEnd::Start)),
        to: Target::World(to),
        class: ContactClass::Rest,
    };
    let fault = fixture::door_refusal(&doc, too_deep);
    assert!(
        matches!(
            fault,
            MateFault::WorldRead {
                mate,
                fault: WorldReadFault::TooDeep { depth: d },
            } if mate.placement == ids[1] && d == depth
        ),
        "{fault:?}"
    );
}

/// **A value's own admission refuses at insert**: a value naming no
/// constraint of its bundle, or a value rather than a mate
/// (`ValueNamesNoMate`); a value on a freedom its mate's kind fixes — a
/// plane mate's slide along its normal (`NotAFreedom { kind:
/// Some(Plane) }`); a point mate's rotation (`NoChart {
/// PointRotation }`); and a mate whose `to` reads two bodies, whose
/// frames do not agree on which reference to chart from (`NoChart {
/// TwoBodies }`). The freedom the kind leaves is admitted beside each,
/// so each refusal is the value's and not the fixture's.
#[test]
fn a1_a_values_own_admission_refuses_at_insert() {
    let (doc, ids, _, body) = instances("msolve10-a1-values", 3);
    let (doc, rest) = fixture::constrain(doc, mate(body, ids[0], ids[1]));
    let (doc, slid) = through_door(
        &doc,
        ids[1],
        &add_value(ids[1], rest.id, Coord::Slide(SlideAxis::X), len(0.25)),
    )
    .expect("a plane mate leaves its in-plane slide");

    let unheld = editor_core::ConstraintId(editor_core::MintId::new(0, 0x5eed));
    for named in [unheld, slid.id] {
        let fault = through_door(
            &doc,
            ids[1],
            &add_value(ids[1], named, Coord::Spin, ang(0.1)),
        )
        .expect_err("refused");
        assert!(
            matches!(fault, MateFault::ValueNamesNoMate { value, mate }
                if value.placement == ids[1] && mate == named),
            "{named}: {fault:?}"
        );
    }

    let fault = through_door(
        &doc,
        ids[1],
        &add_value(ids[1], rest.id, Coord::Slide(SlideAxis::Z), len(0.25)),
    )
    .expect_err("refused");
    assert!(
        matches!(
            fault,
            MateFault::NotAFreedom {
                coord: Coord::Slide(SlideAxis::Z),
                kind: Some(VarKind::Plane),
                ..
            }
        ),
        "{fault:?}"
    );

    let (doc, ball) = fixture::constrain(
        doc,
        fixture::mate(
            fixture::point_of(cap(ids[2], body, CapEnd::Start)),
            fixture::point_of(cap(ids[1], body, CapEnd::End)),
            ContactClass::Rest,
        ),
    );
    let fault = through_door(
        &doc,
        ids[2],
        &add_value(ids[2], ball.id, Coord::Spin, ang(0.1)),
    )
    .expect_err("refused");
    assert!(
        matches!(
            fault,
            MateFault::NoChart {
                why: editor_core::NoChartWhy::PointRotation,
                ..
            }
        ),
        "{fault:?}"
    );

    // An axis mate whose target is the line two copies' faces meet in.
    let (doc, lines, _, body) = instances("msolve10-a1-two-bodies", 3);
    let meet = editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Meet {
        a: editor_core::pose::RoleRead::plain(fixture::plane(cap(lines[0], body, CapEnd::End))),
        b: editor_core::pose::RoleRead::plain(fixture::plane(cap(lines[1], body, CapEnd::End))),
    }));
    let (doc, coaxial) = fixture::constrain_at(
        doc,
        lines[2],
        fixture::mate(
            fixture::axis(cap(lines[2], body, CapEnd::Start)),
            meet,
            ContactClass::Rest,
        ),
    );
    let fault = through_door(
        &doc,
        lines[2],
        &add_value(lines[2], coaxial.id, Coord::Spin, ang(0.1)),
    )
    .expect_err("refused");
    assert!(
        matches!(
            fault,
            MateFault::NoChart {
                why: editor_core::NoChartWhy::TwoBodies,
                ..
            }
        ),
        "{fault:?}"
    );
}

// ---- A4: the door reads nothing of the parts ----

/// **The door asks no reach and reads no face**: an admitted rest and
/// every structural refusal, through a counting reach over the store's
/// — zero asks. The same rest through the refusing reach is admitted
/// too, to the same bundle: nothing the door decides needs the parts.
#[test]
fn a4_the_door_asks_no_reach_and_reads_no_face() {
    let (doc, ids, opts, body) = instances("msolve10-a4-asks", 2);
    let store_reach = mate_reach::<f64>(&opts, Tol::witness());
    let counting = Counting::over(&store_reach);
    let (with_store, _) =
        at_the_door(&doc, &counting, mate(body, ids[0], ids[1])).expect("admitted");
    at_the_door(&doc, &counting, kinds_differ(body, ids[0], ids[1])).expect_err("refused");
    at_the_door_of(&doc, &counting, ids[0], mate(body, ids[0], ids[1])).expect_err("refused");
    assert_eq!(counting.asks(), 0, "the door asks the parts nothing");
    let (refusing, _) =
        at_the_door(&doc, &RefusingReach, mate(body, ids[0], ids[1])).expect("admitted");
    assert_eq!(
        fixture::bundle(&refusing, ids[1]),
        fixture::bundle(&with_store, ids[1]),
        "the refusing reach admits the same constraint"
    );
}

// ---- The history and the log ----

/// **A refused add leaves no entry**: the document is the one it was,
/// and the next add mints one constraint id, not the refused one's.
#[test]
fn a1_a_refused_insert_leaves_no_entry_in_the_history() {
    let (doc, ids, _, body) = instances("msolve10-a1-history", 2);
    let before = doc.clone();
    let (named, fault) =
        at_the_door(&doc, &RefusingReach, kinds_differ(body, ids[0], ids[1])).expect_err("refused");
    let MateFault::KindsDiffer { mate: refused, .. } = fault else {
        panic!("{fault:?}");
    };
    assert_eq!(named, ids[1]);
    assert_eq!(doc.ids(), before.ids());
    assert!(
        fixture::bundle(&doc, named).is_empty(),
        "the bundle holds nothing"
    );
    let (after, minted) = fixture::constrain(doc, mate(body, ids[0], ids[1]));
    assert_ne!(minted, refused, "another mate, another id");
    assert!(
        !after.mint().has_constraint(refused.id),
        "nothing was minted for the refusal"
    );
    assert_eq!(after.ids(), before.ids());
    assert_eq!(fixture::bundle(&after, ids[1]).len(), 1);
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
    let (doc, ids, opts, body) = instances("msolve10-a3-log", 2);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let snapshot = doc.clone();
    let edit = add(ids[1], mate(body, ids[0], ids[1]));
    let applied = doc.apply(&edit, Tol::witness(), &reach).expect("admitted");
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

/// **What needs the poses' values is the placement's at evaluation**:
/// two rests a copy cannot hold at once — its bottom cap and its top
/// cap each resting on the same face — are each admitted at the door,
/// replay with no store, and the placement's evaluation refuses them
/// `Contradictory`, naming the two.
#[test]
fn a3_a_contradictory_pair_is_admitted_and_refused_where_it_evaluates() {
    let (doc, ids, opts, body) = instances("msolve10-a3-contradictory", 2);
    let snapshot = doc.clone();
    let bottom = add(ids[1], mate(body, ids[0], ids[1]));
    let top = add(
        ids[1],
        fixture::rest(
            cap(ids[1], body, CapEnd::End),
            cap(ids[0], body, CapEnd::End),
            ContactClass::Rest,
        ),
    );
    let doc = doc
        .apply(&bottom, Tol::witness(), &RefusingReach)
        .expect("the door admits the first rest")
        .doc;
    let doc = doc
        .apply(&top, Tol::witness(), &RefusingReach)
        .expect("the door admits the second rest: it decides no pose")
        .doc;
    let log = vec![bottom, top];
    let text = save(&snapshot, &log, Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("replay re-decides no pose");
    assert_eq!(
        fixture::bundle(&loaded.doc, ids[1]),
        fixture::bundle(&doc, ids[1])
    );
    let held: Vec<editor_core::ConstraintId> =
        fixture::bundle(&doc, ids[1]).iter().map(|c| c.id).collect();
    let ev = fixture::run(&loaded.doc, &opts);
    match ev.result(ids[1]) {
        Some(NodeResult::Failed(err)) => assert!(
            matches!(
                &err.kind,
                NodeErrorKind::Mate(fault) if matches!(
                    **fault,
                    MateFault::Contradictory { held: h, added: a, .. }
                        if h != a && held.contains(&h.id) && held.contains(&a.id)
                )
            ),
            "{:?}",
            err.kind
        ),
        other => panic!("the placement refuses its own bundle: {other:?}"),
    }
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
    // The one mate whose target is a pose (the first instance's world
    // mate reads the world): its `on` and `to` reads exchanged, so
    // every variable keeps its one reader.
    fn swap(v: &mut serde_json::Value, swapped: &mut usize) {
        match v {
            serde_json::Value::Object(map) => {
                if map.contains_key("on")
                    && map.get("to").is_some_and(|to| to.get("pose").is_some())
                {
                    let on = map["on"].clone();
                    map["on"] = map["to"]["pose"].clone();
                    map["to"]["pose"] = on;
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
    assert_eq!(
        swapped, 1,
        "the snapshot's one pose-to-pose mate was swapped"
    );
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
        ContactClass::Rest,
    );
    let (named, door) =
        at_the_door_of(&doc, &RefusingReach, ids[1], swapped_mate).expect_err("refused");
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
        MateFault::ClassNotAdmitted { .. }
            | MateFault::NotOnTheCopy { .. }
            | MateFault::KindsDiffer { .. }
            | MateFault::WorldRead { .. }
            | MateFault::ValueNamesNoMate { .. }
            | MateFault::NotAFreedom { kind: Some(_), .. }
            | MateFault::NoChart { .. }
    )
}

/// One corpus document: its label, the document, and the options its
/// evaluation reads through.
type Row = (&'static str, ProfileDoc, EvalOptions);

/// **The corpus**: shapes of constraint a document can hold, through
/// the doors that admit them — a sound pair, a chain, a class that
/// solves and mints nothing, a value on a mate, and states the
/// evaluation refuses: a contradictory pair, and a target stranded past
/// a pattern's count after the mate is in.
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
        let (doc, ids, opts, body) = instances("msolve10-corpus-tangent", 2);
        let m = editor_core::Mate {
            class: ContactClass::Tangent,
            ..mate(body, ids[0], ids[1])
        };
        let (doc, _) = fixture::constrain(doc, m);
        ("msolve10-corpus-tangent", doc, opts)
    });
    rows.push({
        let (doc, ids, opts, body) = instances("msolve10-corpus-value", 2);
        let (doc, rest) = fixture::constrain(doc, mate(body, ids[0], ids[1]));
        let (doc, _) = through_door(
            &doc,
            ids[1],
            &add_value(ids[1], rest.id, Coord::Spin, ang(0.3)),
        )
        .expect("a plane mate leaves its spin");
        ("msolve10-corpus-value", doc, opts)
    });
    rows.push({
        let (doc, ids, opts, body) = instances("msolve10-corpus-contradictory", 2);
        let (doc, _) = fixture::constrain(doc, mate(body, ids[0], ids[1]));
        let (doc, _) = fixture::constrain(
            doc,
            fixture::rest(
                cap(ids[1], body, CapEnd::End),
                cap(ids[0], body, CapEnd::End),
                ContactClass::Rest,
            ),
        );
        ("msolve10-corpus-contradictory", doc, opts)
    });
    rows.push({
        // A target stranded past a pattern's count, after insert: the
        // copy the name says no longer exists.
        let (doc, ids, opts, body) = instances("msolve10-corpus-stranded", 2);
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: ids[0].into(),
                count: editor_core::Formula::count(3),
                kind: editor_core::PatternKind::Linear {
                    direction: [fixture::scl(1.0), fixture::scl(0.0), fixture::scl(0.0)],
                    spacing: len(3.0),
                },
            },
        );
        let (doc, _) = fixture::constrain_at(
            doc,
            ids[1],
            fixture::rest(
                cap(ids[1], body, CapEnd::Start),
                fixture::head(fixture::in_copy(
                    pattern,
                    2,
                    in_part(ids[0], body, CapEnd::End),
                )),
                ContactClass::Rest,
            ),
        );
        let (doc, _) = step(
            doc,
            DocEdit::SetStructuralParam {
                node: pattern,
                slot: editor_core::SlotId::Count,
                expr: editor_core::Formula::count(2),
                fresh: Vec::new(),
            },
        );
        ("msolve10-corpus-stranded", doc, opts)
    });
    rows
}

/// **The door admits again every constraint of the corpus, whatever its
/// evaluation says**: each constraint, removed and added back as
/// written, is admitted — the door decides only the recipe, and every
/// constraint of a document passed it. Every fault the evaluation
/// records is one about the poses' values, never a fact the door
/// decides.
#[test]
fn a2_the_door_admits_every_constraint_of_the_corpus_again() {
    let (mut admitted, mut refused_where_evaluated) = (0_usize, 0_usize);
    for (label, doc, opts) in corpus() {
        let reach = mate_reach::<f64>(&opts, Tol::witness());
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
            let bundle = fixture::bundle(&doc, placement);
            for c in &bundle {
                // A world mate is the pin the fixture wrote; a mate a
                // value names is re-added with its value below.
                let Some(m) = c.mate() else { continue };
                if matches!(m.to, Target::World(_))
                    || bundle
                        .iter()
                        .any(|v| v.value().is_some_and(|v| v.mate == c.id))
                {
                    continue;
                }
                constraints += 1;
                let at = MateRef::new(placement, c.id);
                let written = written_constraint(&doc, at);
                let without = step(
                    doc.clone(),
                    DocEdit::RemoveConstraint {
                        placement,
                        constraint: c.id,
                    },
                )
                .0;
                let twin = at_the_door_of(
                    &without,
                    &reach,
                    placement,
                    written.mate().cloned().expect("a mate"),
                );
                assert!(
                    twin.is_ok(),
                    "{label}: the door refused the twin of constraint {} of {placement:?}: {:?}",
                    c.id,
                    twin.err()
                );
                admitted += 1;
            }
            for c in bundle.iter().filter(|c| c.value().is_some()) {
                constraints += 1;
                let written = written_constraint(&doc, MateRef::new(placement, c.id));
                let without = step(
                    doc.clone(),
                    DocEdit::RemoveConstraint {
                        placement,
                        constraint: c.id,
                    },
                )
                .0;
                let value = written.value().cloned().expect("a value");
                let twin = through_door(
                    &without,
                    placement,
                    &DocEdit::AddValue {
                        placement,
                        value: Box::new(value),
                        fresh: Vec::new(),
                    },
                );
                assert!(
                    twin.is_ok(),
                    "{label}: the door refused the twin of value {}: {:?}",
                    c.id,
                    twin.err()
                );
                admitted += 1;
            }
        }
        assert!(
            constraints > 0,
            "{label}: a corpus document holds a constraint"
        );
    }
    // Floors, not counts: the corpus can grow, the claim cannot go
    // vacuous.
    assert!(
        admitted >= 6 && refused_where_evaluated >= 2,
        "the corpus re-admitted {admitted} constraints over {refused_where_evaluated} \
         placements refused where they evaluate"
    );
}
