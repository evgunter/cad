//! MSOLVE-10 acceptance — **a mate the coset table refuses on its own
//! is refused at the edit door**, through the solve's own per-mate
//! admission and the reach the door holds (the spec is
//! `docs/MSOLVE-10-SPEC.md`).
//!
//! The solve decides some things about a mate from its datum alone —
//! the walk's members, the class, each frame's direction, the table's
//! row for the primitive and rider, and the rider on a coincidence,
//! decided over the mate's own lever — and records them against the
//! mate whenever it reads the datum. The edit door asks that admission
//! of a mate being inserted and refuses `EditError::MateRefused`
//! carrying the solve's fault unaltered: THE DOORS DECIDE EDITS AND
//! THE SOLVE DECIDES STATES. What stays the solve's is every verdict
//! about a PAIR, and every state a mate comes to hold after insert —
//! a stranded head, a re-pointed `Part`, a doctored snapshot — which
//! the next evaluation refuses.
//!
//! These rows go through ordinary doors with a `PartStore`: the rider
//! beyond the band refuses at insert with the solve's lever, the rider
//! inside it is admitted and placed, the table's static gaps and a
//! degenerate frame refuse with no reach asked, a rider through the
//! refusing reach refuses `Unleverable`, a refused insert leaves no
//! entry, replay round-trips what the recording door admitted and
//! refuses what the table always refused, and the door and the solve
//! agree on every mate of a corpus.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::wire;
use editor_core::ExtrudeSide;
use editor_core::Formula;

use editor_core::{
    Alignment, AxisSense, CapEnd, Clash, ContactClass, DocEdit, DocumentId, EditError, EvalOptions,
    FacePoseRefusal, Lever, LeverRefusal, MateFault, MateFrame, MatePrimitive, MateReach, MateRef,
    MateSide, Node, PartFault, PersistError, ProfileDoc, ReachRefusal, RecipeNodeId, RefusingReach,
    SideRead, Space, load, mate_reach, save,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, at_the_door, insert, len, on_frame, scl, solve, step, step_with};
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
/// the part's body.
fn instances(label: &str, n: usize) -> (ProfileDoc, Vec<RecipeNodeId>, EvalOptions, RecipeNodeId) {
    let mut store = PartStore::new();
    let (doc_ref, body) =
        store.insert_part(box_part(&format!("{label}-part"), 0.5, 1.0), Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..n {
        // The first is pinned to the world; the rest sit where their
        // mates put them.
        let node = if i == 0 {
            Node::instantiate_part(doc_ref)
        } else {
            crate::fixture::mated_instance(doc_ref)
        };
        let (next, id) = insert(doc, node);
        doc = next;
        ids.push(id);
    }
    (doc, ids, with_resolver(store), body)
}

fn frame(origin: [f64; 3]) -> MateFrame<Formula> {
    MateFrame::authored(
        origin,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        geom_core::Tol::witness(),
    )
    .expect("a definite frame")
}

/// `a`'s top cap on `b`'s bottom cap, at `alignment`, both instances
/// of the part whose body is `body`.
fn mate(
    body: RecipeNodeId,
    a: RecipeNodeId,
    b: RecipeNodeId,
    alignment: Alignment<Formula>,
) -> editor_core::Mate<Formula> {
    mate_across((a, body), (b, body), alignment)
}

/// [`mate`] between instances of two parts, each given with its
/// part's body.
fn mate_across(
    (a, a_body): (RecipeNodeId, RecipeNodeId),
    (b, b_body): (RecipeNodeId, RecipeNodeId),
    alignment: Alignment<Formula>,
) -> editor_core::Mate<Formula> {
    fixture::mate(
        fixture::head(in_part(a, a_body, CapEnd::End)),
        fixture::head(in_part(b, b_body, CapEnd::Start)),
        ContactClass::Rest,
        alignment,
    )
}

/// `mate` with its `b` head replaced.
fn with_b(
    mut mate: editor_core::Mate<Formula>,
    head: editor_core::SitedFace,
) -> editor_core::Mate<Formula> {
    mate.b = SideRead::Face(head.into());
    mate
}

/// `mate` with its class replaced.
fn with_class(
    mut mate: editor_core::Mate<Formula>,
    class: ContactClass,
) -> editor_core::Mate<Formula> {
    mate.class = class;
    mate
}

/// **The constraint `at` names, as written**: its mate with every
/// read and value spelled as authored, ready to be added again.
fn written_mate(doc: &ProfileDoc, at: MateRef) -> editor_core::Mate<Formula> {
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
        .mate
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

/// A frame coincidence seating `b` a unit up `a`, with `clocking` as
/// the rider.
fn seat(clocking: Option<f64>) -> Alignment<Formula> {
    Alignment {
        a: frame([0.0, 0.0, 1.0]),
        b: frame([0.0; 3]),
        primitive: MatePrimitive::FrameCoincidence,
        sense: AxisSense::Aligned,
        clocking,
    }
}

/// A reach that counts its asks and answers through `inner` — the
/// reach asks and the face-pose asks counted apart, since the door
/// asks each where the solve asks it.
struct Counting<'a>(
    core::cell::Cell<usize>,
    &'a dyn MateReach,
    core::cell::Cell<usize>,
);

impl<'a> Counting<'a> {
    fn over(inner: &'a dyn MateReach) -> Self {
        Self(core::cell::Cell::new(0), inner, core::cell::Cell::new(0))
    }

    /// How many face poses were asked.
    fn faces(&self) -> usize {
        self.2.get()
    }
}

impl MateReach for Counting<'_> {
    fn reach(&self, part: &editor_core::DocRef) -> Result<f64, ReachRefusal> {
        self.0.set(self.0.get() + 1);
        self.1.reach(part)
    }

    fn face_pose(
        &self,
        part: &editor_core::DocRef,
        face: &editor_core::FaceName,
    ) -> Result<topo::readback::Pose<f64>, FacePoseRefusal> {
        self.2.set(self.2.get() + 1);
        self.1.face_pose(part, face)
    }
}

/// The lever the solve forms for a mate on `ids`: both parts' reach
/// plus the datum's own terms.
fn lever_of(
    doc: &editor_core::ProfileDoc,
    opts: &EvalOptions,
    ids: &[RecipeNodeId],
    a: &Alignment<Formula>,
) -> f64 {
    let reach = mate_reach::<f64>(opts, Tol::witness());
    let mut arm = fixture::datum_lever(a);
    for &id in ids {
        let Some(Node::InstantiatePart { doc_ref, .. }) = doc.node(id) else {
            panic!("an instance");
        };
        arm += reach.reach(doc_ref).expect("the part reaches");
    }
    arm
}

// ---- A1: the decided rider, at the door ----

/// **A rider beyond the band refuses at insert with the solve's own
/// fault and lever**: `MateRefused { Contradictory { predicate:
/// "mate_clocking_redundant", clash: Levered(Roll { radians, arm }) } }`
/// naming the constraint on both sides, with `arm` the lever the solve
/// forms for the pair, re-derived here — and the constraint the door
/// names is the one the add would have minted, in the bundle of the
/// copy it places.
#[test]
fn a1_a_rider_beyond_the_band_refuses_at_insert_with_the_solves_lever() {
    let (doc, ids, opts, body) = instances("msolve10-a1-beyond", 2);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let alignment = seat(Some(core::f64::consts::FRAC_PI_2));
    let (named, fault) = at_the_door(&doc, &reach, mate(body, ids[0], ids[1], alignment.clone()))
        .expect_err("refused");
    let MateFault::Contradictory {
        held,
        added,
        predicate,
        clash: Clash::Levered(Lever::Roll { radians, arm }),
    } = fault
    else {
        panic!("expected the decided rider, got {fault:?}");
    };
    assert_eq!(held, added, "one constraint, at fault on its own");
    assert_eq!(
        (named, held.placement),
        (ids[1], ids[1]),
        "the door names the placement the constraint places"
    );
    assert_eq!(predicate, "mate_clocking_redundant");
    assert_eq!(radians.to_bits(), core::f64::consts::FRAC_PI_2.to_bits());
    assert_eq!(
        arm.to_bits(),
        lever_of(&doc, &opts, &ids, &alignment).to_bits(),
        "the door's lever is the solve's, to the bit"
    );
    // The constraint the door named is the one the add would have
    // minted: drawn for this mate, and none the document holds.
    assert!(
        !doc.mint().has_constraint(held.id),
        "the named constraint is not the document's"
    );
    // The refusal's sentence names the placement and forwards the
    // fault's.
    let err = doc
        .apply(
            &add(ids[1], mate(body, ids[0], ids[1], alignment)),
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
    let own = "constraint 1 of this node contradicts itself";
    assert!(
        sentence.contains(&format!(
            "InstantiatePart {t} is refused by the solve on its own datum: {own}"
        )) && fault.to_string().starts_with(&format!(
            "constraint {} of node {t} contradicts itself",
            held.id
        )),
        "the door names the placement once, and the fault's own sentence names the \
         constraint: {sentence} / {fault}"
    );
    assert_eq!(sentence.matches(&t).count(), 1, "{sentence}");
}

/// **A rider inside the band is admitted, and the solve places the
/// pair as it does with no rider at all**: no verdict moves, and the
/// same document through the same road solves to the same poses.
#[test]
fn a1_a_rider_inside_the_band_is_admitted_and_the_pair_is_placed() {
    let (doc, ids, opts, body) = instances("msolve10-a1-inside", 2);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let (with_rider, mate_id) =
        at_the_door(&doc, &reach, mate(body, ids[0], ids[1], seat(Some(0.0)))).expect("admitted");
    let (without, plain) =
        at_the_door(&doc, &reach, mate(body, ids[0], ids[1], seat(None))).expect("admitted");
    let a = solve(&with_rider, &opts, Tol::witness());
    let b = solve(&without, &opts, Tol::witness());
    assert_eq!(a.fault(mate_id.placement), None);
    assert_eq!(a.space(mate_id.placement), Some(Space::World));
    assert_eq!(b.space(plain.placement), Some(Space::World));
    for &id in &ids {
        assert!(
            a.placement(&with_rider, id)
                .expect("placed")
                .bit_eq(&b.placement(&without, id).expect("placed")),
            "the redundant rider moves no pose"
        );
    }
}

// ---- A4 / C4: the static gaps and a degenerate frame, with no ask ----

/// **The table's static gaps refuse `TableLacks` at insert with no
/// reach asked**: a clocking rider on a planar rest and a standalone
/// clocking primitive, through a counting reach that answers the
/// refusing one — zero asks, and the fault's `what` is the table's
/// own words.
#[test]
fn a4_the_static_gaps_refuse_table_lacks_with_no_reach_asked() {
    let (doc, ids, _, body) = instances("msolve10-a4-static", 2);
    let counting = Counting::over(&RefusingReach);
    let rest = Alignment {
        primitive: MatePrimitive::PlanarRest { offset: 0.0 },
        ..seat(Some(0.3))
    };
    let (_, fault) =
        at_the_door(&doc, &counting, mate(body, ids[0], ids[1], rest)).expect_err("refused");
    assert!(
        matches!(fault, MateFault::TableLacks { what, .. } if what.contains("planar rest")),
        "{fault:?}"
    );
    let standalone = Alignment {
        primitive: MatePrimitive::Clocking,
        ..seat(Some(0.3))
    };
    let (_, fault) =
        at_the_door(&doc, &counting, mate(body, ids[0], ids[1], standalone)).expect_err("refused");
    assert!(
        matches!(fault, MateFault::TableLacks { what, .. } if what.contains("standalone clocking")),
        "{fault:?}"
    );
    assert_eq!(counting.0.get(), 0, "a static gap asks no lever");
}

/// **A frame with no definite direction refuses at its authoring, and
/// an offset step with none refuses `FrameUnevaluated` at insert**, with
/// no reach asked: the frame is read before any decision is levered.
#[test]
fn a4_a_degenerate_frame_refuses_frame_at_insert_with_no_ask() {
    assert!(matches!(
        MateFrame::<Formula>::authored(
            [0.0; 3],
            [0.0; 3],
            [1.0, 0.0, 0.0],
            geom_core::Tol::witness()
        ),
        Err(geom_core::FrameError::Degenerate { .. })
    ));
    let (doc, ids, _, body) = instances("msolve10-a4-frame", 2);
    let counting = Counting::over(&RefusingReach);
    let mut alignment = seat(Some(0.3));
    alignment.b = MateFrame::on_part(editor_core::Step::Rigid {
        translation: [len(0.0), len(0.0), len(0.0)],
        axis: [scl(0.0), scl(0.0), scl(0.0)],
        angle: ang(0.5),
    });
    let (_, fault) =
        at_the_door(&doc, &counting, mate(body, ids[0], ids[1], alignment)).expect_err("refused");
    assert!(
        matches!(
            fault,
            MateFault::FrameUnevaluated {
                side: MateSide::B,
                ..
            }
        ),
        "{fault:?}"
    );
    assert_eq!(counting.0.get(), 0, "a frame refusal precedes the lever");
}

/// **A rider through the refusing reach refuses `Unleverable`** in the
/// resolver's own voice — exactly as the solve answers it — and a
/// coincidence WITHOUT a rider through the same reach is admitted with
/// no ask at all: the door levers only what the table decides. The id
/// the refusal names is the one the same mate mints through the store's
/// reach, which admits it.
#[test]
fn a4_a_rider_needs_the_reach_and_a_plain_coincidence_asks_none() {
    let (doc, ids, opts, body) = instances("msolve10-a4-reach", 2);
    let counting = Counting::over(&RefusingReach);
    let (named, fault) = at_the_door(&doc, &counting, mate(body, ids[0], ids[1], seat(Some(0.0))))
        .expect_err("refused");
    let MateFault::Unleverable { mate: refused, .. } = &fault else {
        panic!("{fault:?}");
    };
    let refused = *refused;
    assert_eq!(refused.placement, named, "the constraint the door names");
    assert!(
        matches!(
            &fault,
            MateFault::Unleverable { refusal, .. }
                if matches!(
                    refusal.as_ref(),
                    LeverRefusal::Reach {
                        instance,
                        refusal: editor_core::ReachRefusal::PartUnresolved {
                            fault: PartFault::NoResolver,
                        },
                        ..
                    } if *instance == ids[0]
                )
        ),
        "{fault:?}"
    );
    assert_eq!(counting.0.get(), 1, "the first part is asked, and refuses");
    let (_, plain) =
        at_the_door(&doc, &counting, mate(body, ids[0], ids[1], seat(None))).expect("admitted");
    assert_eq!(counting.0.get(), 1, "no rider, no ask");
    assert_ne!(plain, refused, "another mate, another id");
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let (_, admitted) = at_the_door(&doc, &reach, mate(body, ids[0], ids[1], seat(Some(0.0))))
        .expect("the store's reach admits the rider");
    assert_eq!(admitted, refused, "the refusal named the id the mate mints");
}

// ---- A face-based side at the door, and on replay ----

/// **A face-based side asks `face_pose` once per such side at the
/// door, an `Authored` side asks nothing, and a rider still asks the
/// reach once per part**: the door asks each read exactly where the
/// solve asks it — the face before the table reads the frame, the
/// lever only for the rider.
#[test]
fn a_from_face_side_asks_face_pose_once_per_side_at_the_door() {
    let (doc, ids, opts, body) = instances("msolve10-from-face-asks", 2);
    let store_reach = mate_reach::<f64>(&opts, Tol::witness());
    let counting = Counting::over(&store_reach);
    // One face side, one authored side, no rider: one face ask, no
    // reach ask.
    let one_side = Alignment {
        a: MateFrame::from_face(),
        ..seat(None)
    };
    at_the_door(&doc, &counting, mate(body, ids[0], ids[1], one_side)).expect("admitted");
    assert_eq!(counting.faces(), 1, "one face side, one ask");
    assert_eq!(counting.0.get(), 0, "no rider, no reach");
    // Two face sides, no rider: two face asks, still no reach ask.
    let both = Alignment {
        a: MateFrame::from_face(),
        b: MateFrame::from_face(),
        ..seat(None)
    };
    at_the_door(&doc, &counting, mate(body, ids[0], ids[1], both)).expect("admitted");
    assert_eq!(counting.faces(), 3, "two face sides, two asks");
    assert_eq!(counting.0.get(), 0, "no rider, no reach");
    // Two authored sides: no face ask. With a rider: the reach once
    // per part.
    at_the_door(&doc, &counting, mate(body, ids[0], ids[1], seat(Some(0.0)))).expect("admitted");
    assert_eq!(counting.faces(), 3, "an authored side asks nothing");
    assert_eq!(counting.0.get(), 2, "a rider asks the reach once per part");
}

/// **A logged face-based insert replays with no store and loads**:
/// replay declines the face as it declines the rider — the datum
/// alone is decided, the face is not read, the next solve decides it
/// — so a file the door admitted loads without the parts it was
/// admitted against.
#[test]
fn a_logged_from_face_insert_replays_with_no_store_and_loads() {
    let (doc, ids, opts, body) = instances("msolve10-from-face-replay", 2);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let snapshot = doc.clone();
    let alignment = Alignment {
        a: MateFrame::from_face(),
        ..seat(Some(0.0))
    };
    let edit = add(ids[1], mate(body, ids[0], ids[1], alignment));
    let applied = doc.apply(&edit, Tol::witness(), &reach).expect("admitted");
    let log = vec![edit.clone()];
    let text = save(&snapshot, &log, Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("a face-based insert replays with no store");
    assert_eq!(loaded.doc.ids(), applied.doc.ids());
    assert_eq!(
        fixture::bundle(&loaded.doc, ids[1]),
        fixture::bundle(&applied.doc, ids[1])
    );
    assert_eq!(loaded.edits, log);
    // And the same entry through the door with no reach at all —
    // replay's own arm — is admitted, the face declined.
    let admitted = snapshot
        .apply(&edit, Tol::witness(), &RefusingReach)
        .map(|applied| fixture::bundle(&applied.doc, ids[1]).len());
    assert!(
        matches!(
            admitted,
            Err(EditError::MateRefused { ref fault, .. })
                if matches!(**fault, MateFault::FaceUnresolved { .. })
        ),
        "the LIVE door with the refusing reach refuses, since it holds a reach: {admitted:?}"
    );
}

// ---- The history and the log ----

/// **A refused add leaves no entry**: the document is the one it was,
/// and the next add mints one constraint id, not the refused one's.
#[test]
fn a1_a_refused_insert_leaves_no_entry_in_the_history() {
    let (doc, ids, _, body) = instances("msolve10-a1-history", 2);
    let before = doc.clone();
    let rest = Alignment {
        primitive: MatePrimitive::PlanarRest { offset: 0.0 },
        ..seat(Some(0.3))
    };
    let (named, fault) =
        at_the_door(&doc, &RefusingReach, mate(body, ids[0], ids[1], rest)).expect_err("refused");
    let MateFault::TableLacks { mate: refused, .. } = fault else {
        panic!("{fault:?}");
    };
    assert_eq!(named, ids[1]);
    assert_eq!(doc.ids(), before.ids());
    assert!(
        fixture::bundle(&doc, named).is_empty(),
        "the bundle holds nothing"
    );
    let (after, minted) = fixture::constrain(doc, mate(body, ids[0], ids[1], seat(None)));
    assert_ne!(minted, refused, "another mate, another id");
    assert!(
        !after.mint().has_constraint(refused.id),
        "nothing was minted for the refusal"
    );
    assert_eq!(after.ids(), before.ids());
    assert_eq!(fixture::bundle(&after, ids[1]).len(), 1);
    let (unasked, _) = fixture::constrain(before, mate(body, ids[0], ids[1], seat(None)));
    assert_eq!(
        after.mint(),
        unasked.mint(),
        "the refusal left the chain and the log as though it was never asked"
    );
}

/// **Replay re-applies what the recording door decided, and refuses
/// what the table always refused**: a log whose entry inserted a rider
/// the door admitted round-trips through `save` and `load` with no
/// store — replay never solves, so the decision that needed a lever is
/// not re-made — while a hand-edited entry carrying a rider on a
/// planar rest refuses at `load` as `EditReplay { index, MateRefused
/// { TableLacks } }`, naming the entry. A hand-edited CONTRADICTORY
/// rider replays too — the door that could have decided it was not
/// the one that wrote it — and the evaluation refuses it, as it
/// always did.
#[test]
fn a3_replay_round_trips_an_admitted_rider_and_refuses_a_table_gap_at_load() {
    let (doc, ids, opts, body) = instances("msolve10-a3-log", 2);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let snapshot = doc.clone();
    let edit = add(ids[1], mate(body, ids[0], ids[1], seat(Some(0.0))));
    let applied = doc.apply(&edit, Tol::witness(), &reach).expect("admitted");
    let log = vec![edit.clone()];
    let text = save(&snapshot, &log, Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("a log the door admitted loads with no store");
    assert_eq!(
        fixture::bundle(&loaded.doc, ids[1]),
        fixture::bundle(&applied.doc, ids[1])
    );
    assert_eq!(loaded.edits, log);

    // The hand-edited entry: a rider on a planar rest, spliced in as
    // a bare entry at index 1.
    let gap = add(
        ids[1],
        mate(
            body,
            ids[0],
            ids[1],
            Alignment {
                primitive: MatePrimitive::PlanarRest { offset: 0.0 },
                ..seat(Some(0.3))
            },
        ),
    );
    let gap_wire = serde_json::to_value(&gap).expect("an entry serializes");
    let doctored = wire::doctored(&text, |wire| {
        wire["edits"]
            .as_array_mut()
            .expect("the log is a list")
            .push(gap_wire);
    });
    let err = load(&doctored, Tol::witness()).expect_err("the table always refused this mate");
    let PersistError::EditReplay { index, error } = &err else {
        panic!("expected EditReplay, got {err:?}");
    };
    assert_eq!(*index, 1, "the entry is named");
    assert!(
        matches!(
            &**error,
            EditError::MateRefused { fault, .. }
                if matches!(**fault, MateFault::TableLacks { what, .. } if what.contains("planar rest"))
        ),
        "{error:?}"
    );
    assert!(
        err.to_string().contains("edit 1 refused on replay"),
        "{err}"
    );

    // A contradictory rider, hand-edited into a log of its own,
    // replays: no reach, no decision, and the evaluation's solve
    // refuses it as before.
    let contradictory = add(
        ids[1],
        mate(
            body,
            ids[0],
            ids[1],
            seat(Some(core::f64::consts::FRAC_PI_2)),
        ),
    );
    let wire_entry = serde_json::to_value(&contradictory).expect("serializes");
    let bare = save(&snapshot, &[], Tol::witness()).expect("saves");
    let doctored = wire::doctored(&bare, |wire| {
        wire["edits"]
            .as_array_mut()
            .expect("the log is a list")
            .push(wire_entry);
    });
    let loaded = load(&doctored, Tol::witness()).expect("replay re-decides nothing");
    let rider = ids[1];
    assert_eq!(fixture::bundle(&loaded.doc, rider).len(), 1);
    let poses = solve(&loaded.doc, &opts, Tol::witness());
    assert!(
        matches!(
            poses.fault(rider),
            Some(MateFault::Contradictory {
                predicate: "mate_clocking_redundant",
                ..
            })
        ),
        "{:?}",
        poses.fault(rider)
    );
}

/// **A snapshot is a state, and a state the doors did not decide is
/// the solve's**: the load door's snapshot walk asks only that a
/// mate's alignment be finite, so a doctored snapshot carrying a
/// planar rest WITH a rider — the table's gap, refused at every
/// insert — LOADS, and the next evaluation refuses it `TableLacks` at
/// the solve, where the door refuses its twin identically. The door
/// decides edits; it does not re-decide what a file says.
#[test]
fn a3_a_doctored_snapshot_carrying_a_table_gap_loads_and_the_solve_refuses_it() {
    let (doc, ids, opts, body) = instances("msolve10-a3-snapshot", 2);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let rest = Alignment {
        primitive: MatePrimitive::PlanarRest { offset: 0.0 },
        ..seat(None)
    };
    let (doc, mate_id) =
        at_the_door(&doc, &reach, mate(body, ids[0], ids[1], rest)).expect("admitted");
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    // `clocking` is skipped on the wire when `None`, so the rider is
    // ADDED to the snapshot's planar rest rather than replacing a
    // null: the object that carries a `planar_rest` primitive and
    // `sense` and no `clocking` yet (the first instance's world pin
    // is a frame coincidence, and keeps none).
    let mut added = 0_usize;
    fn add_rider(v: &mut serde_json::Value, added: &mut usize) {
        match v {
            serde_json::Value::Object(map) => {
                if map
                    .get("primitive")
                    .is_some_and(|p| p.to_string().contains("planar_rest"))
                    && map.contains_key("sense")
                    && !map.contains_key("clocking")
                {
                    map.insert("clocking".to_owned(), serde_json::json!(0.3));
                    *added += 1;
                }
                for value in map.values_mut() {
                    add_rider(value, added);
                }
            }
            serde_json::Value::Array(items) => {
                for item in items {
                    add_rider(item, added);
                }
            }
            _ => {}
        }
    }
    let doctored = wire::doctored(&text, |w| add_rider(w, &mut added));
    assert_eq!(added, 1, "the snapshot's planar rest gained the rider");
    let loaded = load(&doctored, Tol::witness()).expect("a snapshot is a state the load admits");
    assert_eq!(
        fixture::constraint(&loaded.doc, mate_id).alignment.clocking,
        Some(0.3)
    );
    let poses = solve(&loaded.doc, &opts, Tol::witness());
    let fault = poses
        .fault(mate_id.placement)
        .expect("the solve refuses the state");
    assert!(
        matches!(fault, MateFault::TableLacks { mate, what } if *mate == mate_id && what.contains("planar rest")),
        "{fault:?}"
    );
    let twin = written_mate(&loaded.doc, mate_id);
    let (named, door) =
        at_the_door(&loaded.doc, &reach, twin).expect_err("the door refuses the twin");
    assert_eq!(named, mate_id.placement);
    let subject = own_datum_subject(&door).expect("a fault about the twin's own datum");
    assert_eq!(
        renamed(door, subject, mate_id),
        *fault,
        "the door's word is the solve's"
    );
}

// ---- A2: the door and the solve agree over a corpus ----

/// The constraint a fault names as its SUBJECT, for the arms that are
/// a fact about one constraint's own datum — the arms the door refuses
/// on the datum alone — and `None` for every other: a verdict about a
/// pair or a bundle (a contradiction between two constraints, an
/// escalation on a fold, a pose past the format's range, a bundle
/// beyond the pin or across two targets), a fault about the document,
/// and `PlacerRefused`, which two sites raise — the per-reference check
/// the door asks, and the pair's derived offset the fold alone reads —
/// with nothing in the value to say which, so the corpus claims nothing
/// of it.
fn own_datum_subject(fault: &MateFault) -> Option<MateRef> {
    match fault {
        MateFault::Frame { mate, .. }
        | MateFault::ClassNotAdmitted { mate }
        | MateFault::TableLacks { mate, .. }
        | MateFault::SideUnresolved { mate, .. }
        | MateFault::DanglingHead { mate, .. }
        | MateFault::PartSelectsAnotherCopy { mate, .. }
        | MateFault::SelfMate { mate, .. }
        | MateFault::Unleverable { mate, .. }
        | MateFault::FaceUnresolved { mate, .. }
        | MateFault::FrameUnevaluated { mate, .. }
        | MateFault::NotOnTheCopy { mate }
        | MateFault::SideBase { mate, .. } => Some(*mate),
        MateFault::Contradictory {
            held,
            added,
            predicate: "mate_clocking_redundant",
            ..
        } if held == added => Some(*held),
        MateFault::Indeterminate { mate, diag }
            if diag.predicate == Some("mate_clocking_redundant") =>
        {
            Some(*mate)
        }
        MateFault::Contradictory { .. }
        | MateFault::Indeterminate { .. }
        | MateFault::PoseOutOfRange { .. }
        | MateFault::Band { .. }
        | MateFault::PosesOfAnotherDocument { .. }
        | MateFault::PlacerRefused { .. }
        | MateFault::Overconstrained { .. }
        | MateFault::TargetsDiffer { .. } => None,
    }
}

/// `fault` with every constraint equal to `from` renamed `to`: the
/// door's fault about a re-added TWIN of a constraint, spelled as the
/// solve's about the original. Exhaustive, so a new arm arrives here as
/// a compile error rather than as an address the rename missed.
fn renamed(fault: MateFault, from: MateRef, to: MateRef) -> MateFault {
    let r = |id: MateRef| if id == from { to } else { id };
    match fault {
        MateFault::PosesOfAnotherDocument { .. }
        | MateFault::Band { .. }
        | MateFault::Overconstrained { .. }
        | MateFault::TargetsDiffer { .. } => fault,
        MateFault::Frame { mate, side, error } => MateFault::Frame {
            mate: r(mate),
            side,
            error,
        },
        MateFault::FrameUnevaluated {
            mate,
            side,
            refusal,
        } => MateFault::FrameUnevaluated {
            mate: r(mate),
            side,
            refusal,
        },
        MateFault::ClassNotAdmitted { mate } => MateFault::ClassNotAdmitted { mate: r(mate) },
        MateFault::NotOnTheCopy { mate } => MateFault::NotOnTheCopy { mate: r(mate) },
        MateFault::SideBase { mate, side, base } => MateFault::SideBase {
            mate: r(mate),
            side,
            base,
        },
        MateFault::PoseOutOfRange { held, added } => MateFault::PoseOutOfRange {
            held: r(held),
            added: r(added),
        },
        MateFault::TableLacks { mate, what } => MateFault::TableLacks {
            mate: r(mate),
            what,
        },
        MateFault::Indeterminate { mate, diag } => MateFault::Indeterminate {
            mate: r(mate),
            diag,
        },
        MateFault::Contradictory {
            held,
            added,
            predicate,
            clash,
        } => MateFault::Contradictory {
            held: r(held),
            added: r(added),
            predicate,
            clash,
        },
        MateFault::SideUnresolved { mate, side, read } => MateFault::SideUnresolved {
            mate: r(mate),
            side,
            read,
        },
        MateFault::DanglingHead { mate, side, head } => MateFault::DanglingHead {
            mate: r(mate),
            side,
            head,
        },
        MateFault::PlacerRefused {
            mate,
            side,
            placer,
            error,
            placer_row,
        } => MateFault::PlacerRefused {
            mate: r(mate),
            side,
            placer,
            error,
            placer_row,
        },
        MateFault::PartSelectsAnotherCopy {
            mate,
            side,
            part,
            named,
            selected,
        } => MateFault::PartSelectsAnotherCopy {
            mate: r(mate),
            side,
            part,
            named,
            selected,
        },
        MateFault::SelfMate { mate, instance } => MateFault::SelfMate {
            mate: r(mate),
            instance,
        },
        MateFault::Unleverable { mate, refusal } => MateFault::Unleverable {
            mate: r(mate),
            refusal,
        },
        MateFault::FaceUnresolved {
            mate,
            side,
            refusal,
        } => MateFault::FaceUnresolved {
            mate: r(mate),
            side,
            refusal,
        },
    }
}

/// One corpus document: its label, the document, and the options its
/// solve and its door both read through.
type Row = (&'static str, ProfileDoc, EvalOptions);

/// **The corpus**: every shape of constraint a document can hold,
/// through the doors that admit it — sound pairs and chains, a
/// redundant rider, a rest and a coaxial that leave the copy loose, a
/// class that solves and mints nothing, and every own-datum fault a
/// constraint can COME to carry after it is added: a head stranded by
/// a rebind, one member on both sides by a rebind, a `Part` re-pointed
/// at another copy, a part the store no longer resolves, and a
/// contradictory rider a hand-edited log carried in.
fn corpus() -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    let pair = |label: &'static str, alignment: Alignment<Formula>| -> Row {
        let (doc, ids, opts, body) = instances(label, 2);
        let reach = mate_reach::<f64>(&opts, Tol::witness());
        let (doc, _) = fixture::constrain_with(doc, &reach, mate(body, ids[0], ids[1], alignment));
        (label, doc, opts)
    };
    rows.push(pair("msolve10-corpus-sound", seat(None)));
    rows.push(pair("msolve10-corpus-redundant", seat(Some(0.0))));
    rows.push(pair(
        "msolve10-corpus-rest",
        Alignment {
            primitive: MatePrimitive::PlanarRest { offset: 0.0 },
            ..seat(None)
        },
    ));
    rows.push(pair(
        "msolve10-corpus-coaxial-rider",
        Alignment {
            primitive: MatePrimitive::Coaxial,
            ..seat(Some(0.4))
        },
    ));
    rows.push({
        let (doc, ids, opts, body) = instances("msolve10-corpus-tangent", 2);
        let m = with_class(
            mate(body, ids[0], ids[1], seat(None)),
            ContactClass::Tangent,
        );
        let (doc, _) = fixture::constrain(doc, m);
        ("msolve10-corpus-tangent", doc, opts)
    });
    rows.push({
        let (doc, ids, opts, body) = instances("msolve10-corpus-chain", 3);
        let (doc, _) = fixture::constrain(doc, mate(body, ids[0], ids[1], seat(None)));
        let (doc, _) = fixture::constrain(doc, mate(body, ids[1], ids[2], seat(None)));
        ("msolve10-corpus-chain", doc, opts)
    });
    rows.push({
        // A head stranded past a pattern's count, after insert: the
        // copy the name says does not exist.
        let (doc, ids, opts, body) = instances("msolve10-corpus-stranded", 2);
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: ids[0].into(),
                count: editor_core::Formula::count(2),
                kind: editor_core::PatternKind::Linear {
                    direction: [fixture::scl(1.0), fixture::scl(0.0), fixture::scl(0.0)],
                    spacing: len(3.0),
                },
            },
        );
        let (doc, _) = fixture::insert_mate_with_stranded_head(
            doc,
            with_b(
                mate(body, ids[1], ids[0], seat(None)),
                fixture::head(fixture::in_copy(
                    pattern,
                    5,
                    in_part(ids[0], body, CapEnd::Start),
                )),
            ),
            MateSide::B,
            ids[0],
            body,
        );
        ("msolve10-corpus-stranded", doc, opts)
    });
    rows.push({
        // One member on both sides, by a re-point after insert: the
        // side's slot door writes `b`'s side onto `a`.
        let (doc, ids, opts, body) = instances("msolve10-corpus-self", 2);
        let reach = mate_reach::<f64>(&opts, Tol::witness());
        let (doc, at) =
            fixture::constrain_with(doc, &reach, mate(body, ids[0], ids[1], seat(None)));
        let (doc, _) = step_with(
            doc,
            DocEdit::SetParam {
                node: at.placement,
                slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Side {
                    constraint: at.id,
                    side: MateSide::A,
                }),
                value: editor_core::SlotValue::Read(
                    fixture::head(in_part(ids[1], body, CapEnd::End)).into(),
                ),
                fresh: Vec::new(),
            },
            &reach,
        );
        ("msolve10-corpus-self", doc, opts)
    });
    rows.push({
        // A `Part` re-pointed at another copy than the name says.
        let (doc, ids, opts, body) = instances("msolve10-corpus-part", 2);
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
        let (doc, part) = insert(
            doc,
            Node::Part {
                of: pattern.into(),
                select: editor_core::PartSelect::Instance(editor_core::Formula::count(0)),
            },
        );
        let m = editor_core::Mate {
            a: SideRead::Face(
                fixture::head_at(
                    part,
                    fixture::in_copy(pattern, 0, in_part(ids[0], body, CapEnd::End)),
                )
                .into(),
            ),
            ..mate(body, ids[0], ids[1], seat(None))
        };
        let (doc, _) = fixture::constrain_at(doc, ids[1], m);
        let (doc, _) = step(
            doc,
            DocEdit::SetStructuralParam {
                node: part,
                slot: editor_core::SlotId::Instance,
                expr: editor_core::Formula::count(2),
                fresh: Vec::new(),
            },
        );
        ("msolve10-corpus-part", doc, opts)
    });
    rows.push({
        // A part the store no longer resolves: authored where both
        // parts were in hand, read where one is not.
        let (doc, ids, opts, body) = instances("msolve10-corpus-lost", 1);
        let mut both = PartStore::new();
        both.insert_part(
            box_part("msolve10-corpus-lost-part", 0.5, 1.0),
            Tol::witness(),
        );
        let (lost_ref, lost_body) = both.insert_part(
            box_part("msolve10-corpus-lost-elsewhere", 0.5, 1.0),
            Tol::witness(),
        );
        let (doc, lost) = insert(doc, fixture::mated_instance(lost_ref));
        let both = with_resolver(both);
        let reach = mate_reach::<f64>(&both, Tol::witness());
        let (doc, _) = fixture::constrain_with(
            doc,
            &reach,
            mate_across((ids[0], body), (lost, lost_body), seat(Some(0.0))),
        );
        ("msolve10-corpus-lost", doc, opts)
    });
    rows.push({
        // A contradictory rider a hand-edited log carried in: replay
        // re-decides nothing, so the document holds it.
        let (doc, ids, opts, body) = instances("msolve10-corpus-hand-edited", 2);
        let text = save(&doc, &[], Tol::witness()).expect("saves");
        let entry = add(
            ids[1],
            mate(
                body,
                ids[0],
                ids[1],
                seat(Some(core::f64::consts::FRAC_PI_2)),
            ),
        );
        let entry = serde_json::to_value(&entry).expect("serializes");
        let doctored = wire::doctored(&text, |wire| {
            wire["edits"]
                .as_array_mut()
                .expect("the log is a list")
                .push(entry);
        });
        let loaded = load(&doctored, Tol::witness()).expect("replay re-decides nothing");
        assert_eq!(
            fixture::bundle(&loaded.doc, ids[1]).len(),
            1,
            "the replayed add lands in the bundle of the copy it places"
        );
        ("msolve10-corpus-hand-edited", loaded.doc, opts)
    });
    rows
}

/// **The door and the solve agree on every constraint of the corpus**:
/// a constraint the solve admits, the door admits again as a twin added
/// in its place; a constraint the solve faults for its OWN datum, the
/// door refuses with the same fault, renamed to the twin's address —
/// the same predicate, the same clash, the same lever, the same head. A
/// verdict about a pair or a bundle is the solve's and claimed of
/// nothing here. A world pin reads no face and decides nothing, so the
/// rows read the constraints that read one.
#[test]
fn a2_the_door_and_the_solve_agree_on_every_mate_of_the_corpus() {
    let (mut admitted, mut refused) = (0_usize, 0_usize);
    for (label, doc, opts) in corpus() {
        let reach = mate_reach::<f64>(&opts, Tol::witness());
        let poses = solve(&doc, &opts, Tol::witness());
        let mut mates = 0_usize;
        for placement in doc.placements() {
            for c in fixture::bundle(&doc, placement) {
                if !c
                    .mate
                    .sides()
                    .iter()
                    .any(|(_, read)| matches!(read, SideRead::Face(_)))
                {
                    continue;
                }
                mates += 1;
                let at = MateRef::new(placement, c.id);
                let tag = format!(
                    "{} of {}",
                    c.id,
                    test_utils::refusal::tag(placement.0.digest())
                );
                // The twin is added in the original's place: the bundle
                // without it, the same mate added back.
                let without = step(
                    doc.clone(),
                    DocEdit::RemoveConstraint {
                        placement,
                        constraint: c.id,
                    },
                )
                .0;
                let twin =
                    fixture::at_the_door_of(&without, &reach, placement, written_mate(&doc, at));
                match poses.fault(placement) {
                    None => {
                        assert!(
                            twin.is_ok(),
                            "{label}: the solve admits constraint {tag}; the door refused its \
                             twin: {:?}",
                            twin.err()
                        );
                        admitted += 1;
                    }
                    Some(fault) if own_datum_subject(fault) == Some(at) => {
                        let got = match twin {
                            Err((_, got)) => got,
                            Ok(_) => panic!(
                                "{label}: the solve refuses constraint {tag} on its own datum \
                                 ({fault}); the door admitted its twin"
                            ),
                        };
                        let subject = own_datum_subject(&got)
                            .unwrap_or_else(|| panic!("{label}: {tag}: {got:?}"));
                        assert_eq!(
                            renamed(got, subject, at),
                            *fault,
                            "{label}: constraint {tag} — the door's fault is the solve's"
                        );
                        refused += 1;
                    }
                    Some(_) => {}
                }
            }
        }
        assert!(mates > 0, "{label}: a corpus document holds a constraint");
    }
    // Floors, not counts: the corpus can grow, the claim cannot go
    // vacuous.
    assert!(
        admitted >= 7 && refused >= 5,
        "the corpus checked {admitted} admissions and {refused} own-datum refusals"
    );
}
