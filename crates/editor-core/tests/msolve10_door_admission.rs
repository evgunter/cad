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

use editor_core::{
    Alignment, AxisSense, CapEnd, Clash, ClusterMaintenance, ContactClass, DocEdit, DocumentId,
    EditError, EvalOptions, Lever, LeverRefusal, LoggedEdit, MateFault, MateFrame, MatePrimitive,
    MateReach, MateRole, MateSide, Node, PartFault, PersistError, ProfileDoc, ReachRefusal,
    RecipeNodeId, RefusingReach, gauge_of, load, mate_reach, save,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{at_the_door, insert, len, on_frame, solve, step, step_with};
use geom_core::Tol;

// ---- Substrate ----

/// A box part: a square of half-side `half` at the origin, extruded
/// `height` along +z.
fn box_part(label: &str, half: f64, height: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, half)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(height),
        },
    );
    doc
}

/// `n` instances of one box part, and the options that resolve them.
fn instances(label: &str, n: usize) -> (ProfileDoc, Vec<RecipeNodeId>, EvalOptions) {
    let mut store = PartStore::new();
    let doc_ref = store.insert(box_part(&format!("{label}-part"), 0.5, 1.0), Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..n {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    (doc, ids, with_resolver(store))
}

fn frame(origin: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

/// `a`'s top cap on `b`'s bottom cap, at `alignment`.
fn mate(
    a: RecipeNodeId,
    b: RecipeNodeId,
    alignment: Alignment,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: fixture::head(in_part(a, CapEnd::End)),
        b: fixture::head(in_part(b, CapEnd::Start)),
        class: ContactClass::Rest,
        alignment,
    }
}

/// `node` with its `b` head replaced.
fn with_b(
    mut node: Node<editor_core::ProfileProgram>,
    head: editor_core::SitedFace,
) -> Node<editor_core::ProfileProgram> {
    if let Node::Mate { b, .. } = &mut node {
        *b = head;
    }
    node
}

/// `node` with its `a` head replaced.
fn with_a(
    mut node: Node<editor_core::ProfileProgram>,
    head: editor_core::SitedFace,
) -> Node<editor_core::ProfileProgram> {
    if let Node::Mate { a, .. } = &mut node {
        *a = head;
    }
    node
}

/// `node` with its class replaced.
fn with_class(
    mut node: Node<editor_core::ProfileProgram>,
    class: ContactClass,
) -> Node<editor_core::ProfileProgram> {
    if let Node::Mate { class: c, .. } = &mut node {
        *c = class;
    }
    node
}

/// A frame coincidence seating `b` a unit up `a`, with `clocking` as
/// the rider.
fn seat(clocking: Option<f64>) -> Alignment {
    Alignment {
        a: frame([0.0, 0.0, 1.0]),
        b: frame([0.0; 3]),
        primitive: MatePrimitive::FrameCoincidence,
        sense: AxisSense::Aligned,
        clocking,
    }
}

/// A reach that counts its asks and answers through `inner`.
struct Counting<'a>(core::cell::Cell<usize>, &'a dyn MateReach);

impl MateReach for Counting<'_> {
    fn reach(&self, part: &editor_core::DocRef) -> Result<f64, ReachRefusal> {
        self.0.set(self.0.get() + 1);
        self.1.reach(part)
    }
}

/// The lever the solve forms for a mate on `ids`: both parts' reach
/// plus the datum's own terms.
fn lever_of(doc: &editor_core::ProfileDoc, opts: &EvalOptions, ids: &[RecipeNodeId], a: &Alignment) -> f64 {
    let reach = mate_reach::<f64>(opts, Tol::witness());
    let mut arm = a.lever_arm();
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
/// naming the mate on both sides, with `arm` the lever the solve forms
/// for the pair, re-derived here — and the id the door names is the
/// one the insert would have minted.
#[test]
fn a1_a_rider_beyond_the_band_refuses_at_insert_with_the_solves_lever() {
    let (doc, ids, opts) = instances("msolve10-a1-beyond", 2);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let alignment = seat(Some(core::f64::consts::FRAC_PI_2));
    let (named, fault) =
        at_the_door(&doc, &reach, mate(ids[0], ids[1], alignment)).expect_err("refused");
    let MateFault::Contradictory {
        held,
        added,
        predicate,
        clash: Clash::Levered(Lever::Roll { radians, arm }),
    } = fault
    else {
        panic!("expected the decided rider, got {fault:?}");
    };
    assert_eq!(
        (held, added),
        (named, named),
        "one mate, at fault on its own"
    );
    assert_eq!(predicate, "mate_clocking_redundant");
    assert_eq!(radians.to_bits(), core::f64::consts::FRAC_PI_2.to_bits());
    assert_eq!(
        arm.to_bits(),
        lever_of(&doc, &opts, &ids, &alignment).to_bits(),
        "the door's lever is the solve's, to the bit"
    );
    // The id the door named is the one a mate inserted next mints.
    let (_, minted) =
        at_the_door(&doc, &reach, mate(ids[0], ids[1], seat(None))).expect("admitted");
    assert_eq!(minted, named);
    // The refusal's sentence names the node and forwards the fault's.
    let err = doc
        .apply(
            &DocEdit::InsertNode {
                node: mate(ids[0], ids[1], alignment),
            },
            Tol::witness(),
            &reach,
        )
        .expect_err("refused");
    let EditError::MateRefused { fault, .. } = &err else {
        panic!("{err:?}");
    };
    let sentence = err.to_string();
    assert!(
        sentence.contains(&format!("node {}", named.0)) && sentence.contains(&fault.to_string()),
        "{sentence}"
    );
}

/// **A rider inside the band is admitted, and the solve places the
/// pair as it does with no rider at all**: no verdict moves, and the
/// same document through the same road solves to the same poses.
#[test]
fn a1_a_rider_inside_the_band_is_admitted_and_the_pair_is_placed() {
    let (doc, ids, opts) = instances("msolve10-a1-inside", 2);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let (with_rider, mate_id) =
        at_the_door(&doc, &reach, mate(ids[0], ids[1], seat(Some(0.0)))).expect("admitted");
    let (without, plain) =
        at_the_door(&doc, &reach, mate(ids[0], ids[1], seat(None))).expect("admitted");
    let a = solve(&with_rider, &opts, Tol::witness());
    let b = solve(&without, &opts, Tol::witness());
    assert_eq!(a.fault(mate_id), None);
    assert_eq!(a.role(mate_id), Some(MateRole::Determining));
    assert_eq!(b.role(plain), Some(MateRole::Determining));
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
    let (doc, ids, _) = instances("msolve10-a4-static", 2);
    let counting = Counting(core::cell::Cell::new(0), &RefusingReach);
    let rest = Alignment {
        primitive: MatePrimitive::PlanarRest { offset: 0.0 },
        ..seat(Some(0.3))
    };
    let (_, fault) = at_the_door(&doc, &counting, mate(ids[0], ids[1], rest)).expect_err("refused");
    assert!(
        matches!(fault, MateFault::TableLacks { what, .. } if what.contains("planar rest")),
        "{fault:?}"
    );
    let standalone = Alignment {
        primitive: MatePrimitive::Clocking,
        ..seat(Some(0.3))
    };
    let (_, fault) =
        at_the_door(&doc, &counting, mate(ids[0], ids[1], standalone)).expect_err("refused");
    assert!(
        matches!(fault, MateFault::TableLacks { what, .. } if what.contains("standalone clocking")),
        "{fault:?}"
    );
    assert_eq!(counting.0.get(), 0, "a static gap asks no lever");
}

/// **A frame with no definite direction refuses `Frame` at insert**,
/// with no reach asked: the frame is read before any decision is
/// levered.
#[test]
fn a4_a_degenerate_frame_refuses_frame_at_insert_with_no_ask() {
    let (doc, ids, _) = instances("msolve10-a4-frame", 2);
    let counting = Counting(core::cell::Cell::new(0), &RefusingReach);
    let mut alignment = seat(Some(0.3));
    alignment.b.axis = [0.0; 3];
    let (_, fault) =
        at_the_door(&doc, &counting, mate(ids[0], ids[1], alignment)).expect_err("refused");
    assert!(
        matches!(
            fault,
            MateFault::Frame {
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
/// no ask at all: the door levers only what the table decides.
#[test]
fn a4_a_rider_needs_the_reach_and_a_plain_coincidence_asks_none() {
    let (doc, ids, _) = instances("msolve10-a4-reach", 2);
    let counting = Counting(core::cell::Cell::new(0), &RefusingReach);
    let (named, fault) =
        at_the_door(&doc, &counting, mate(ids[0], ids[1], seat(Some(0.0)))).expect_err("refused");
    assert!(
        matches!(
            &fault,
            MateFault::Unleverable {
                mate,
                refusal: LeverRefusal::PartUnresolved {
                    instance,
                    fault: PartFault::NoResolver,
                },
            } if *mate == named && *instance == ids[0]
        ),
        "{fault:?}"
    );
    assert_eq!(counting.0.get(), 1, "the first part is asked, and refuses");
    let (_, plain) =
        at_the_door(&doc, &counting, mate(ids[0], ids[1], seat(None))).expect("admitted");
    assert_eq!(counting.0.get(), 1, "no rider, no ask");
    assert_eq!(plain, named);
}

// ---- The history and the log ----

/// **A refused insert leaves no entry**: the document is the one it
/// was, and the next insert mints the id the refused one was named
/// with.
#[test]
fn a1_a_refused_insert_leaves_no_entry_in_the_history() {
    let (doc, ids, _) = instances("msolve10-a1-history", 2);
    let before = doc.clone();
    let rest = Alignment {
        primitive: MatePrimitive::PlanarRest { offset: 0.0 },
        ..seat(Some(0.3))
    };
    let (named, _) =
        at_the_door(&doc, &RefusingReach, mate(ids[0], ids[1], rest)).expect_err("refused");
    assert_eq!(doc.order(), before.order());
    assert_eq!(doc.node(named), None);
    let (after, minted) = insert(doc, mate(ids[0], ids[1], seat(None)));
    assert_eq!(minted, named, "nothing was minted for the refusal");
    assert_eq!(after.order().len(), before.order().len() + 1);
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
    let (doc, ids, opts) = instances("msolve10-a3-log", 2);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let snapshot = doc.clone();
    let edit = DocEdit::InsertNode {
        node: mate(ids[0], ids[1], seat(Some(0.0))),
    };
    let applied = doc.apply(&edit, Tol::witness(), &reach).expect("admitted");
    let log = vec![LoggedEdit {
        edit: edit.clone(),
        maintenance: applied.cluster_rows(),
    }];
    let text = save(&snapshot, &log, Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("a log the door admitted loads with no store");
    assert_eq!(loaded.doc.order(), applied.doc.order());
    assert_eq!(loaded.edits, log);

    // The hand-edited entry: a rider on a planar rest, spliced in as
    // a bare entry at index 1.
    let gap = LoggedEdit::bare(DocEdit::InsertNode {
        node: mate(
            ids[0],
            ids[1],
            Alignment {
                primitive: MatePrimitive::PlanarRest { offset: 0.0 },
                ..seat(Some(0.3))
            },
        ),
    });
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
            error,
            EditError::MateRefused { fault, .. }
                if matches!(**fault, MateFault::TableLacks { what, .. } if what.contains("planar rest"))
        ),
        "{error:?}"
    );
    assert!(
        err.to_string().contains("edit 1 refused on replay"),
        "{err}"
    );

    // A contradictory rider, hand-edited in, replays: no reach, no
    // decision, and the evaluation's solve refuses it as before.
    let contradictory = LoggedEdit::bare(DocEdit::InsertNode {
        node: mate(ids[0], ids[1], seat(Some(core::f64::consts::FRAC_PI_2))),
    });
    let wire_entry = serde_json::to_value(&contradictory).expect("serializes");
    let doctored = wire::doctored(&text, |wire| {
        wire["edits"]
            .as_array_mut()
            .expect("the log is a list")
            .push(wire_entry);
    });
    let loaded = load(&doctored, Tol::witness()).expect("replay re-decides nothing");
    let rider = *loaded.doc.order().last().expect("the rider is last");
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
    let (doc, ids, opts) = instances("msolve10-a3-snapshot", 2);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let rest = Alignment {
        primitive: MatePrimitive::PlanarRest { offset: 0.0 },
        ..seat(None)
    };
    let (doc, mate_id) = at_the_door(&doc, &reach, mate(ids[0], ids[1], rest)).expect("admitted");
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    // `clocking` is skipped on the wire when `None`, so the rider is
    // ADDED to the snapshot's one alignment rather than replacing a
    // null: the object that carries `primitive` and `sense` and no
    // `clocking` yet.
    let mut added = 0_usize;
    fn add_rider(v: &mut serde_json::Value, added: &mut usize) {
        match v {
            serde_json::Value::Object(map) => {
                if map.contains_key("primitive")
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
    assert_eq!(added, 1, "the snapshot's one alignment gained the rider");
    let loaded = load(&doctored, Tol::witness()).expect("a snapshot is a state the load admits");
    let Some(Node::Mate { alignment, .. }) = loaded.doc.node(mate_id) else {
        panic!("the mate is in the snapshot");
    };
    assert_eq!(alignment.clocking, Some(0.3));
    let poses = solve(&loaded.doc, &opts, Tol::witness());
    let fault = poses.fault(mate_id).expect("the solve refuses the state");
    assert!(
        matches!(fault, MateFault::TableLacks { mate, what } if *mate == mate_id && what.contains("planar rest")),
        "{fault:?}"
    );
    assert_eq!(poses.role(mate_id), Some(MateRole::Refused));
    let twin = loaded.doc.node(mate_id).expect("live").clone();
    let (named, door) =
        at_the_door(&loaded.doc, &reach, twin).expect_err("the door refuses the twin");
    assert_eq!(
        renamed(door, named, mate_id),
        *fault,
        "the door's word is the solve's"
    );
}

/// **A mate on a pair the fold never reads is refused on the datum
/// alone**: two members over ONE instance — the instance and a copy
/// of it — form a pair `solve_cluster` never folds, so the solve
/// records NOTHING against such a mate whatever its datum says (it
/// declares, fault-free), while the insert door refuses a table gap
/// and a contradictory rider on it all the same. Which pairs the fold
/// reads is a cluster fact the door does not decide; the datum is
/// malformed by itself. The two documents are reached the only way
/// they can be, through a doctored snapshot.
#[test]
fn a2_a_mate_on_a_pair_the_fold_never_reads_is_refused_on_the_datum_alone() {
    for (label, rider_on, expect) in [
        (
            "msolve10-unread-gap",
            MatePrimitive::PlanarRest { offset: 0.0 },
            "table gap",
        ),
        (
            "msolve10-unread-rider",
            MatePrimitive::FrameCoincidence,
            "rider",
        ),
    ] {
        let (doc, ids, opts) = instances(label, 1);
        let (doc, copies) = insert(
            doc,
            Node::Pattern {
                input: ids[0],
                count: editor_core::Expr::count(2),
                kind: editor_core::PatternKind::Linear {
                    direction: [fixture::scl(1.0), fixture::scl(0.0), fixture::scl(0.0)],
                    spacing: len(3.0),
                },
            },
        );
        let reach = mate_reach::<f64>(&opts, Tol::witness());
        let (doc, mate_id) = at_the_door(
            &doc,
            &reach,
            with_a(
                mate(
                    ids[0],
                    ids[0],
                    Alignment {
                        primitive: rider_on,
                        ..seat(None)
                    },
                ),
                fixture::head(fixture::in_copy(copies, 1, in_part(ids[0], CapEnd::End))),
            ),
        )
        .expect("two members over one instance are a pair the door admits");
        let text = save(&doc, &[], Tol::witness()).expect("saves");
        let mut added = 0_usize;
        let doctored = wire::doctored(&text, |w| {
            fn add(v: &mut serde_json::Value, added: &mut usize) {
                match v {
                    serde_json::Value::Object(map) => {
                        if map.contains_key("primitive")
                            && map.contains_key("sense")
                            && !map.contains_key("clocking")
                        {
                            map.insert(
                                "clocking".to_owned(),
                                serde_json::json!(core::f64::consts::FRAC_PI_2),
                            );
                            *added += 1;
                        }
                        for value in map.values_mut() {
                            add(value, added);
                        }
                    }
                    serde_json::Value::Array(items) => {
                        for item in items {
                            add(item, added);
                        }
                    }
                    _ => {}
                }
            }
            add(w, &mut added);
        });
        assert_eq!(added, 1, "{label}");
        let loaded = load(&doctored, Tol::witness()).expect("a snapshot is a state");
        let poses = solve(&loaded.doc, &opts, Tol::witness());
        assert_eq!(
            poses.fault(mate_id),
            None,
            "{label}: the fold never reads this pair, so the solve records nothing"
        );
        assert_eq!(poses.role(mate_id), Some(MateRole::Declaring), "{label}");
        let twin = loaded.doc.node(mate_id).expect("live").clone();
        let (_, fault) =
            at_the_door(&loaded.doc, &reach, twin).expect_err("the door refuses the datum");
        match expect {
            "table gap" => assert!(
                matches!(fault, MateFault::TableLacks { .. }),
                "{label}: {fault:?}"
            ),
            _ => assert!(
                matches!(
                    fault,
                    MateFault::Contradictory {
                        predicate: "mate_clocking_redundant",
                        ..
                    }
                ),
                "{label}: {fault:?}"
            ),
        }
    }
}

// ---- A2: the door and the solve agree over a corpus ----

/// The mate a fault names as its SUBJECT, for the arms that are a
/// fact about one mate's own datum — the arms the door refuses — and
/// `None` for every other: a verdict about a pair (UNDER, a
/// contradiction between two mates, an escalation on a fold), a fault
/// about the document, and `PlacerRefused`, which two sites raise —
/// the per-reference check the door asks, and the pair's derived
/// offset the fold alone reads — with nothing in the value to say
/// which, so the corpus claims nothing of it.
fn own_datum_subject(fault: &MateFault) -> Option<RecipeNodeId> {
    match fault {
        MateFault::Frame { mate, .. }
        | MateFault::ClassNotAdmitted { mate }
        | MateFault::TableLacks { mate, .. }
        | MateFault::DanglingHead { mate, .. }
        | MateFault::PartSelectsAnotherCopy { mate, .. }
        | MateFault::SelfMate { mate, .. }
        | MateFault::Unleverable { mate, .. } => Some(*mate),
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
        | MateFault::Under { .. }
        | MateFault::Band { .. }
        | MateFault::PosesOfAnotherDocument { .. }
        | MateFault::PlacerRefused { .. } => None,
    }
}

/// `fault` with every mate id equal to `from` renamed `to`: the door's
/// fault about a re-inserted TWIN of a mate, spelled as the solve's
/// about the original. Exhaustive, so a new arm arrives here as a
/// compile error rather than as an id the rename missed.
fn renamed(fault: MateFault, from: RecipeNodeId, to: RecipeNodeId) -> MateFault {
    let r = |id: RecipeNodeId| if id == from { to } else { id };
    match fault {
        MateFault::PosesOfAnotherDocument { .. } | MateFault::Band { .. } => fault,
        MateFault::Frame { mate, side, error } => MateFault::Frame {
            mate: r(mate),
            side,
            error,
        },
        MateFault::ClassNotAdmitted { mate } => MateFault::ClassNotAdmitted { mate: r(mate) },
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
        MateFault::Under {
            mate,
            parent,
            child,
            residual,
        } => MateFault::Under {
            mate: r(mate),
            parent,
            child,
            residual,
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
        } => MateFault::PlacerRefused {
            mate: r(mate),
            side,
            placer,
            error,
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
    }
}

/// One corpus document: its label, the document, and the options its
/// solve and its door both read through.
type Row = (&'static str, ProfileDoc, EvalOptions);

/// **The corpus**: every shape of mate a document can hold, through
/// the doors that admit it — sound pairs and chains, a redundant
/// rider, the pair's own verdicts (a contradiction between two mates,
/// UNDER on a rest and on a coaxial), a declaring copy pair, a class
/// that solves and mints nothing, and every own-datum fault a mate
/// can COME to carry after insert: a head stranded by a rebind, one
/// member on both sides by a rebind, a `Part` re-pointed at another
/// copy, a part the store no longer resolves, and a contradictory
/// rider a hand-edited log carried in.
fn corpus() -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    let pair = |label: &'static str, alignment: Alignment| -> Row {
        let (doc, ids, opts) = instances(label, 2);
        let reach = mate_reach::<f64>(&opts, Tol::witness());
        let (doc, _) = step_with(
            doc,
            DocEdit::InsertNode {
                node: mate(ids[0], ids[1], alignment),
            },
            &reach,
        );
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
        let (doc, ids, opts) = instances("msolve10-corpus-tangent", 2);
        let node = with_class(mate(ids[0], ids[1], seat(None)), ContactClass::Tangent);
        let (doc, _) = insert(doc, node);
        ("msolve10-corpus-tangent", doc, opts)
    });
    rows.push({
        let (doc, ids, opts) = instances("msolve10-corpus-pair-contradiction", 2);
        let (doc, _) = insert(doc, mate(ids[0], ids[1], seat(None)));
        let (doc, _) = insert(
            doc,
            mate(
                ids[0],
                ids[1],
                Alignment {
                    a: frame([0.0, 0.0, 2.0]),
                    ..seat(None)
                },
            ),
        );
        ("msolve10-corpus-pair-contradiction", doc, opts)
    });
    rows.push({
        let (doc, ids, opts) = instances("msolve10-corpus-chain", 3);
        let (doc, _) = insert(doc, mate(ids[0], ids[1], seat(None)));
        let (doc, _) = insert(doc, mate(ids[1], ids[2], seat(None)));
        ("msolve10-corpus-chain", doc, opts)
    });
    rows.push({
        // Copies of `a` onto `b`: a declaring sibling pair.
        let (doc, ids, opts) = instances("msolve10-corpus-copies", 2);
        let (doc, copies) = insert(
            doc,
            Node::Pattern {
                input: ids[0],
                count: editor_core::Expr::count(2),
                kind: editor_core::PatternKind::Linear {
                    direction: [fixture::scl(1.0), fixture::scl(0.0), fixture::scl(0.0)],
                    spacing: len(3.0),
                },
            },
        );
        let (doc, _) = insert(doc, mate(ids[0], ids[1], seat(None)));
        let (doc, _) = insert(
            doc,
            with_a(
                mate(ids[0], ids[1], seat(None)),
                fixture::head(fixture::in_copy(copies, 1, in_part(ids[0], CapEnd::End))),
            ),
        );
        ("msolve10-corpus-copies", doc, opts)
    });
    rows.push({
        // A head stranded onto local geometry, after insert.
        let (doc, ids, opts) = instances("msolve10-corpus-stranded", 2);
        let (doc, profile) = on_frame(
            doc,
            [5.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![fixture::square(0.0, 0.0, 0.5)],
        );
        let (doc, local) = insert(
            doc,
            Node::Extrude {
                profile,
                distance: len(1.0),
            },
        );
        let local_cap = editor_core::StableName {
            kind: editor_core::EntityKind::Face,
            node: local,
            path: vec![editor_core::RoleSeg::Cap(CapEnd::Start)],
        };
        let (doc, _) = fixture::insert_mate_with_stranded_head(
            doc,
            with_b(mate(ids[0], ids[1], seat(None)), fixture::head(local_cap)),
            MateSide::B,
            ids[0],
        );
        ("msolve10-corpus-stranded", doc, opts)
    });
    rows.push({
        // One member on both sides, by a rebind after insert: the
        // name-repair door moves a head read at its own mint with its
        // name, so `b`'s head lands on `a`.
        let (doc, ids, opts) = instances("msolve10-corpus-self", 2);
        let reach = mate_reach::<f64>(&opts, Tol::witness());
        let (doc, _) = step_with(
            doc,
            DocEdit::InsertNode {
                node: mate(ids[0], ids[1], seat(None)),
            },
            &reach,
        );
        let (doc, _) = step_with(
            doc,
            DocEdit::Rebind {
                from: in_part(ids[1], CapEnd::Start),
                to: in_part(ids[0], CapEnd::Start),
            },
            &reach,
        );
        ("msolve10-corpus-self", doc, opts)
    });
    rows.push({
        // A `Part` re-pointed at another copy than the name says.
        let (doc, ids, opts) = instances("msolve10-corpus-part", 2);
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: ids[1],
                count: editor_core::Expr::count(3),
                kind: editor_core::PatternKind::Linear {
                    direction: [fixture::scl(1.0), fixture::scl(0.0), fixture::scl(0.0)],
                    spacing: len(3.0),
                },
            },
        );
        let (doc, part) = insert(
            doc,
            Node::Part {
                of: pattern,
                select: editor_core::PartSelect::Instance(editor_core::Expr::count(0)),
            },
        );
        let (doc, _) = insert(
            doc,
            with_b(
                mate(ids[0], ids[1], seat(None)),
                fixture::head_at(
                    part,
                    fixture::in_copy(pattern, 0, in_part(ids[1], CapEnd::Start)),
                ),
            ),
        );
        let (doc, _) = step(
            doc,
            DocEdit::SetStructuralParam {
                node: part,
                slot: editor_core::SlotId::Instance,
                expr: editor_core::Expr::count(2),
            },
        );
        ("msolve10-corpus-part", doc, opts)
    });
    rows.push({
        // A part the store no longer resolves: authored where both
        // parts were in hand, read where one is not.
        let (doc, ids, opts) = instances("msolve10-corpus-lost", 1);
        let mut both = PartStore::new();
        both.insert(
            box_part("msolve10-corpus-lost-part", 0.5, 1.0),
            Tol::witness(),
        );
        let lost_ref = both.insert(
            box_part("msolve10-corpus-lost-elsewhere", 0.5, 1.0),
            Tol::witness(),
        );
        let (doc, lost) = insert(doc, Node::instantiate_part(lost_ref));
        let both = with_resolver(both);
        let reach = mate_reach::<f64>(&both, Tol::witness());
        let (doc, _) = step_with(
            doc,
            DocEdit::InsertNode {
                node: mate(ids[0], lost, seat(Some(0.0))),
            },
            &reach,
        );
        ("msolve10-corpus-lost", doc, opts)
    });
    rows.push({
        // A contradictory rider a hand-edited log carried in: replay
        // re-decides nothing, so the document holds it.
        let (doc, ids, opts) = instances("msolve10-corpus-hand-edited", 2);
        let text = save(&doc, &[], Tol::witness()).expect("saves");
        // The mate joins the two instances' clusters, and a log entry
        // carries the rows its edit performs — so the hand-edited entry
        // records the join, as the save door would have.
        let entry = LoggedEdit {
            edit: DocEdit::InsertNode {
                node: mate(ids[0], ids[1], seat(Some(core::f64::consts::FRAC_PI_2))),
            },
            maintenance: vec![ClusterMaintenance::Join {
                survived: ids[0],
                absorbed: ids[1],
                absorbed_frame: doc.placements().get(&ids[1]).copied(),
            }],
        };
        let entry = serde_json::to_value(&entry).expect("serializes");
        let doctored = wire::doctored(&text, |wire| {
            wire["edits"]
                .as_array_mut()
                .expect("the log is a list")
                .push(entry);
        });
        let loaded = load(&doctored, Tol::witness()).expect("replay re-decides nothing");
        assert_eq!(
            gauge_of(&loaded.doc, ids[1]),
            ids[0],
            "the recorded join names the gauge the joined cluster keeps"
        );
        ("msolve10-corpus-hand-edited", loaded.doc, opts)
    });
    rows
}

/// **The door and the solve agree on every mate of the corpus**: a
/// mate the solve admits, the door admits again as a twin; a mate the
/// solve faults for its OWN datum, the door refuses with the same
/// fault, renamed to the twin's id — the same predicate, the same
/// clash, the same lever, the same head. A verdict about a pair is
/// the solve's and claimed of nothing here.
#[test]
fn a2_the_door_and_the_solve_agree_on_every_mate_of_the_corpus() {
    let (mut admitted, mut refused) = (0_usize, 0_usize);
    for (label, doc, opts) in corpus() {
        let reach = mate_reach::<f64>(&opts, Tol::witness());
        let poses = solve(&doc, &opts, Tol::witness());
        let mut mates = 0_usize;
        for &id in doc.order() {
            let Some(node) = doc.node(id) else {
                continue;
            };
            if !matches!(node, Node::Mate { .. }) {
                continue;
            }
            mates += 1;
            let twin = at_the_door(&doc, &reach, node.clone());
            match poses.fault(id) {
                None => {
                    assert!(
                        twin.is_ok(),
                        "{label}: the solve admits mate {}; the door refused its twin: {:?}",
                        id.0,
                        twin.err()
                    );
                    admitted += 1;
                }
                Some(fault) if own_datum_subject(fault) == Some(id) => {
                    let (named, got) = match twin {
                        Err(refusal) => refusal,
                        Ok(_) => panic!(
                            "{label}: the solve refuses mate {} on its own datum ({fault}); \
                             the door admitted its twin",
                            id.0
                        ),
                    };
                    assert_eq!(
                        renamed(got, named, id),
                        *fault,
                        "{label}: mate {} — the door's fault is the solve's",
                        id.0
                    );
                    refused += 1;
                }
                Some(_) => {}
            }
        }
        assert!(mates > 0, "{label}: a corpus document holds a mate");
    }
    // Floors, not counts: the corpus can grow, the claim cannot go
    // vacuous.
    assert!(
        admitted >= 7 && refused >= 5,
        "the corpus checked {admitted} admissions and {refused} own-datum refusals"
    );
}
