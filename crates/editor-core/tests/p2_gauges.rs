//! **EDIT-PLACEMENT P2 — gauges, instance offsets and the unplaced
//! group** (ASSEMBLY.md A11 (2)–(5), A4, A9; the spec's `## P2`
//! rows).
//!
//! Placement lives on a gauge: an instance names its gauge (the world
//! by default) and may carry an offset in it; a gauge names its parent.
//! A placing mate joins the groups of two instances on one gauge and
//! places its FIRST operand's group on its SECOND's, clearing the
//! first's root offset in the same edit. No edit records a frame, so a
//! delete is never refused for the placement it removes and a log
//! replays without a solve; a group nothing places lives in its own
//! space. The world pose an evaluation composes is `A ∘ F ∘ B` — the
//! placers' offsets, the group's frame, the solved representatives —
//! and every lane composes the same.
//!
//! The rows build their scenes from two literal block parts: a wide
//! base slab and a tall top block, the top seated on the base by one
//! `Rest` frame coincidence, as `msolve1_transform_aware` seats them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::fixture;
use crate::wire::doctored;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, Dimension, DocEdit, DocParam, DocParamValue,
    DocRef, DocumentId, EditError, EvalOptions, Evaluation, Expr, Frame, Maintenance, MateFault,
    MateFrame, MatePrimitive, MateRole, MeasureExpr, MeasurePrimitive, Node, NodeErrorKind,
    ParamName, PartResolver, PersistError, Placement, ProfileDoc, RecipeNodeId, RefusingReach,
    SitedFace, SitedRef, StableName, Step, Unplaced, ValuePayload, apply, apply_replayed, evaluate,
    groups, load, product, regauge_then_mate, root_of, save,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::seat::{assert_seated, seat_map};
use fixture::{ang, head, head_at, insert, len, offset_of, on_frame, run, scl, solve, step, xform};
use geom_core::{Bounds, Interval, Tol};
use topo::Body;

// ---- substrate ----

pub(crate) const BASE_WIDTH: f64 = 3.0;
pub(crate) const BASE_HEIGHT: f64 = 1.0;
pub(crate) const TOP_HEIGHT: f64 = 3.0;

/// A `w x w x h` block, as a whole part document, and its body.
pub(crate) fn block(label: &str, w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
        },
    )
}

/// The two parts in one store: the store's options, each part's
/// reference and body.
pub(crate) struct Parts {
    pub(crate) store: PartStore,
    pub(crate) base: DocRef,
    pub(crate) base_body: RecipeNodeId,
    pub(crate) top: DocRef,
    pub(crate) top_body: RecipeNodeId,
}

pub(crate) fn parts(label: &str) -> Parts {
    let mut store = PartStore::new();
    let (base, base_body) = store.insert_part(
        block(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (top, top_body) = store.insert_part(
        block(&format!("{label}-top"), 1.0, TOP_HEIGHT),
        Tol::witness(),
    );
    Parts {
        store,
        base,
        base_body,
        top,
        top_body,
    }
}

impl Parts {
    pub(crate) fn opts(&self) -> EvalOptions {
        with_resolver(self.store.clone())
    }
    /// The base's top cap.
    pub(crate) fn base_cap(&self, base: RecipeNodeId) -> StableName {
        in_part(base, self.base_body, CapEnd::End)
    }
    /// The top block's bottom cap.
    pub(crate) fn top_cap(&self, top: RecipeNodeId) -> StableName {
        in_part(top, self.top_body, CapEnd::Start)
    }
    /// The top block's upper cap, which a second block seats on.
    pub(crate) fn top_upper_cap(&self, top: RecipeNodeId) -> StableName {
        in_part(top, self.top_body, CapEnd::End)
    }
}

pub(crate) fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame::authored(origin, axis, [1.0, 0.0, 0.0])
}

/// **"Mate the top to the base"**: the top block's bottom cap (the
/// FIRST operand, which moves) seated on the base's top cap at
/// `(1, 1)`, outward normals opposed.
pub(crate) fn seat(top: SitedFace, base: SitedFace) -> Node<editor_core::ProfileProgram> {
    seat_on(top, base, [1.0, 1.0, BASE_HEIGHT])
}

/// [`seat`] onto a cap whose seat point is `at`, in its own part's
/// coordinates.
pub(crate) fn seat_on(
    mover: SitedFace,
    onto: SitedFace,
    at: [f64; 3],
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: mover,
        b: onto,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
            b: frame(at, [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

pub(crate) fn lift() -> ParamName {
    ParamName::from_static("lift")
}

/// A gauge on `parent` at `[0, 0, lift]`, turned `angle` about z — a
/// placement the document's `lift` drives.
pub(crate) fn lifting_gauge(
    parent: Option<RecipeNodeId>,
    angle: f64,
) -> Node<editor_core::ProfileProgram> {
    Node::gauge(
        parent,
        Step::Rigid {
            translation: [len(0.0), len(0.0), Expr::param(lift(), Dimension::Length)],
            axis: [0.0, 0.0, 1.0].map(scl),
            angle: ang(angle),
        },
    )
}

pub(crate) fn literal(t: [f64; 3]) -> Placement {
    Placement::literal(&Frame::translation(t))
}

pub(crate) fn set_lift(doc: ProfileDoc, value: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetDocParamValue {
            name: lift(),
            value: DocParamValue::Continuous(value),
        },
    )
    .0
}

pub(crate) fn declare_lift(doc: ProfileDoc, value: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetDocParam {
            name: lift(),
            value: DocParam::continuous(Dimension::Length, value),
        },
    )
    .0
}

pub(crate) fn set_gauge(
    doc: ProfileDoc,
    node: RecipeNodeId,
    gauge: Option<RecipeNodeId>,
) -> ProfileDoc {
    step(doc, DocEdit::SetGauge { node, gauge }).0
}

pub(crate) fn set_offset(
    doc: ProfileDoc,
    instance: RecipeNodeId,
    offset: Option<Placement>,
) -> ProfileDoc {
    step(doc, DocEdit::SetOffset { instance, offset }).0
}

pub(crate) fn body_of<T: geom_core::Decide>(ev: &Evaluation<T>, id: RecipeNodeId) -> Arc<Body<T>> {
    match &ev
        .value(id)
        .unwrap_or_else(|| panic!("{id:?} evaluated: {:?}", ev.node_error(id)))
        .payload
    {
        ValuePayload::Body(b) => Arc::clone(b),
        other => panic!("{id:?} is a {}, not a body", other.kind_name()),
    }
}

/// A body's points, in arena order, as `f64` triples.
pub(crate) fn points(body: &Body<f64>) -> Vec<[f64; 3]> {
    body.points().map(|(_, p)| [p.x, p.y, p.z]).collect()
}

/// A body's lowest corner — where a block's own origin landed.
pub(crate) fn min_corner(body: &Body<f64>) -> [f64; 3] {
    points(body).into_iter().fold([f64::INFINITY; 3], |m, p| {
        [m[0].min(p[0]), m[1].min(p[1]), m[2].min(p[2])]
    })
}

pub(crate) fn close(a: [f64; 3], b: [f64; 3], what: &str) {
    assert!(
        a.iter().zip(b).all(|(x, y)| (x - y).abs() <= 1e-12),
        "{what}: {a:?} vs {b:?}"
    );
}

pub(crate) fn resolver(store: PartStore) -> Arc<dyn PartResolver> {
    Arc::new(store)
}

// ---- Row 1: the gauge chain ----

/// **An instance on a gauge under a gauge is placed by the composed
/// chain, and one document parameter moves both levels.** The outer
/// gauge sits at `[0, 0, lift]`, the inner one `[5, 0, 0]` on it, and
/// an instance on each; the inner instance carries `[0, 1, 0]` of its
/// own. The world pose door and the evaluated bodies agree, and a
/// value edit to `lift` moves both instances by the same amount.
#[test]
fn a_gauge_under_a_gauge_places_by_the_composed_chain_and_a_parameter_moves_both_levels() {
    let p = parts("p2-chain");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-chain"), Tol::witness());
    let doc = declare_lift(doc, 2.0);
    let (doc, outer) = insert(doc, lifting_gauge(None, 0.0));
    let (doc, inner) = insert(doc, Node::gauge(Some(outer), literal([5.0, 0.0, 0.0])));
    let (doc, on_outer) = insert(doc, Node::instantiate_part(p.base));
    let (doc, on_inner) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, on_outer, Some(outer));
    let doc = set_gauge(doc, on_inner, Some(inner));
    let doc = set_offset(doc, on_inner, Some(literal([0.0, 1.0, 0.0])));
    let o = p.opts();
    let check = |doc: &ProfileDoc, z: f64| {
        let poses = solve(doc, &o, Tol::witness());
        let at = |id| poses.placement(doc, id).expect("placed").translation;
        close(at(on_outer), [0.0, 0.0, z], "the outer gauge's instance");
        close(at(on_inner), [5.0, 1.0, z], "the inner gauge's instance");
        let ev = run(doc, &o);
        close(
            min_corner(&body_of(&ev, on_outer)),
            [0.0, 0.0, z],
            "the outer body",
        );
        close(
            min_corner(&body_of(&ev, on_inner)),
            [5.0, 1.0, z],
            "the inner body",
        );
    };
    check(&doc, 2.0);
    let doc = set_lift(doc, 7.0);
    check(&doc, 7.0);
}

// ---- Row 2: insert at the origin; the mate door ----

/// **An inserted instance stands on the world at the empty offset, and
/// mating it clears its offset in the same edit, which replays without
/// solving.** The mate names the top first, so the top's group is
/// placed on the base's: the edit reports the offset it cleared, a
/// replay with no reach produces the same document and report, and the
/// whole log saved from empty loads with no store.
#[test]
fn an_inserted_instance_sits_at_the_origin_and_its_mate_clears_its_offset_replayably() {
    let p = parts("p2-door");
    let empty = ProfileDoc::empty(DocumentId::derive("p2-door"), Tol::witness());
    let (doc, base) = insert(empty.clone(), Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    assert_eq!(offset_of(&doc, top), Some(Placement::IDENTITY));
    assert_eq!(doc.node(top).and_then(Node::gauge_ref), None);
    let o = p.opts();
    assert!(
        solve(&doc, &o, Tol::witness())
            .placement(&doc, top)
            .expect("a lone instance is placed")
            .bit_eq(&Frame::IDENTITY),
        "an inserted instance sits at the world's origin"
    );
    let mate = DocEdit::InsertNode {
        node: Box::new(seat(head(p.top_cap(top)), head(p.base_cap(base)))),
    };
    let applied = apply(&doc, &mate, Tol::witness(), &editor_core::RefusingReach)
        .expect("a mate with no rider asks no store");
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::OffsetCleared {
            instance: doc.spoken(top),
            offset: Placement::IDENTITY,
        }],
        "the first operand's root gave up its offset, and the edit says so"
    );
    assert_eq!(offset_of(&applied.doc, top), None);
    assert_eq!(offset_of(&applied.doc, base), Some(Placement::IDENTITY));
    assert_eq!(root_of(&applied.doc, top), base);
    let replayed = apply_replayed(&doc, &mate, Tol::witness()).expect("replay needs no reach");
    assert!(replayed.doc.bit_eq(&applied.doc), "the replay is the edit");
    assert_eq!(replayed.maintenance, applied.maintenance);
    let log = vec![
        DocEdit::InsertNode {
            node: Box::new(Node::instantiate_part(p.base)),
        },
        DocEdit::InsertNode {
            node: Box::new(Node::instantiate_part(p.top)),
        },
        mate,
    ];
    let text = save(&empty, &log, Tol::witness()).expect("the log saves");
    let loaded = load(&text, Tol::witness()).expect("the log loads with no store");
    assert!(
        loaded.doc.bit_eq(&applied.doc),
        "and replays to the same document"
    );
    // The mate places: the top seats on the base.
    let poses = solve(&loaded.doc, &o, Tol::witness());
    let mate_id = *loaded
        .doc
        .order()
        .last()
        .expect("the mate is the last node");
    assert_eq!(
        poses.role(mate_id),
        Some(MateRole::Determining),
        "the mate placed the top"
    );
}

// ---- Row 3: the compound door ----

/// **"Copy `b`'s gauge to `a`, then mate `a` to `b`" re-gauges `a`'s
/// whole group.** The base stands on a gauge; the top and a second
/// block mated onto it are a group on the world. A plain insert of the
/// top-to-base mate only DECLARES (the two sit on different gauges);
/// the compound door puts both world instances on the base's gauge
/// first, so its mate places and the three are one group.
#[test]
fn the_compound_door_regauges_the_first_operands_whole_group_then_places() {
    let p = parts("p2-regauge");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-regauge"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([10.0, 0.0, 0.0])));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, base, Some(g));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, upper) = insert(doc, Node::instantiate_part(p.top));
    let (doc, _) = insert(
        doc,
        seat_on(
            head(p.top_cap(upper)),
            head(p.top_upper_cap(top)),
            [0.0, 0.0, TOP_HEIGHT],
        ),
    );
    assert_eq!(groups(&doc).len(), 2, "the base, and the stacked pair");
    let o = p.opts();
    let mate = seat(head(p.top_cap(top)), head(p.base_cap(base)));

    // The plain insert: across gauges, the mate declares.
    let (plain, plain_mate) = insert(doc.clone(), mate.clone());
    assert_eq!(
        solve(&plain, &o, Tol::witness()).role(plain_mate),
        Some(MateRole::Declaring)
    );
    assert_eq!(groups(&plain).len(), 2, "a declaring mate joins nothing");

    // The compound door: the whole action applied, with its record.
    let out = regauge_then_mate(&doc, mate.clone(), Tol::witness(), &RefusingReach)
        .expect("no other mate starts placing");
    assert_eq!(
        out.edits,
        vec![
            DocEdit::SetGauge {
                node: top,
                gauge: Some(g),
            },
            DocEdit::SetGauge {
                node: upper,
                gauge: Some(g),
            },
            DocEdit::InsertNode {
                node: Box::new(mate),
            },
        ],
        "the record is the group's re-gauges in document order, then the insert"
    );
    assert!(
        replay(&doc, &out.edits).bit_eq(&out.doc),
        "the record replays from the input to the outcome's document"
    );
    assert_eq!(
        out.doc.order().last(),
        Some(&out.mate),
        "the outcome names the mate its insert minted"
    );
    let done = out.doc;
    let mate = out.mate;
    assert_eq!(
        out.maintenance
            .iter()
            .filter_map(|row| match row {
                Maintenance::OffsetCleared { instance, .. } => Some(instance.id()),
                _ => None,
            })
            .collect::<Vec<_>>(),
        vec![top],
        "the action reports the insert's offset clear: {:?}",
        out.maintenance
    );
    assert_eq!(groups(&done), vec![vec![base, top, upper]]);
    for id in [top, upper] {
        assert_eq!(done.node(id).and_then(Node::gauge_ref), Some(g));
    }
    assert_eq!(
        root_of(&done, upper),
        base,
        "the base's group placed the pair"
    );
    assert_eq!(
        offset_of(&done, top),
        None,
        "the mate cleared the moved root"
    );
    let poses = solve(&done, &o, Tol::witness());
    assert_eq!(poses.role(mate), Some(MateRole::Determining));
    close(
        poses.placement(&done, top).expect("placed").translation,
        [11.0, 1.0, BASE_HEIGHT],
        "the top seats on the base, on the base's gauge",
    );
}

/// **A refusal after a re-gauge applied leaves no partial outcome.**
/// The same group, mated onto the base by a mate the insert door
/// refuses on its own datum (a standalone clocking, `TableLacks`): the
/// two re-gauges apply, then the insert refuses, and the door answers
/// that refusal, not a document with the group re-gauged and no mate.
#[test]
fn a_refusal_at_the_insert_after_the_regauge_refuses_the_whole_action() {
    let p = parts("p2-regauge-refused");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-regauge-refused"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([10.0, 0.0, 0.0])));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, base, Some(g));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let mut mate = seat(head(p.top_cap(top)), head(p.base_cap(base)));
    let Node::Mate { alignment, .. } = &mut mate else {
        unreachable!("seat builds a mate")
    };
    alignment.primitive = MatePrimitive::Clocking;
    let before = doc.clone();
    match regauge_then_mate(&doc, mate, Tol::witness(), &RefusingReach) {
        Err(EditError::MateRefused { fault, .. }) => assert!(
            matches!(*fault, MateFault::TableLacks { .. }),
            "the insert refuses on the table's gap: {fault:?}"
        ),
        other => panic!("the insert step refuses the whole action: {other:?}"),
    }
    assert!(doc.bit_eq(&before), "the input document is untouched");
    assert_eq!(
        doc.node(top).and_then(Node::gauge_ref),
        None,
        "and the top was never re-gauged in it"
    );
}

// ---- Row 4: checked offsets ----

/// **A non-root member's offset is a checked statement.** Stated where
/// the solve puts the instance, it holds and nothing faults; stated
/// elsewhere, that instance faults typed, naming the offset, the root
/// and the measured clash, with the recourse — and the rest of the
/// group stands.
#[test]
fn a_checked_offset_that_disagrees_with_the_solve_faults_that_instance_typed() {
    let p = parts("p2-checked");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-checked"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let o = p.opts();
    let solved = solve(&doc, &o, Tol::witness())
        .placement(&doc, top)
        .expect("the top is placed");

    let agrees = set_offset(doc.clone(), top, Some(Placement::literal(&solved)));
    let poses = solve(&agrees, &o, Tol::witness());
    assert_eq!(poses.fault(top), None, "a true statement holds");
    assert_eq!(root_of(&agrees, top), base, "and the base still roots");

    let disagrees = set_offset(doc, top, Some(literal([0.0, 0.0, 5.0])));
    let poses = solve(&disagrees, &o, Tol::witness());
    let Some(MateFault::OffsetDisagrees { instance, root, .. }) = poses.fault(top) else {
        panic!("expected OffsetDisagrees, got {:?}", poses.fault(top))
    };
    assert_eq!((*instance, *root), (top, base));
    assert_eq!(poses.fault(base), None, "the root stands");
    assert_eq!(poses.fault(mate), None, "the mate placed what it placed");
    let ev = run(&disagrees, &o);
    let error = ev.node_error(top).expect("the top's row fails");
    assert!(
        matches!(&error.kind, NodeErrorKind::Mate(f) if matches!(**f, MateFault::OffsetDisagrees { .. })),
        "{error:?}"
    );
    assert!(
        error.to_string().contains(editor_core::OFFSET_RECOURSE),
        "{error}"
    );
    assert!(ev.value(base).is_some(), "the base evaluates");
}

// ---- Row 5: what is never refused, and the unplaced group ----

/// **Deleting a gauge, a placed member or a placing mate is not
/// refused, and what stays placed is recomputed.** Each leaves the
/// moved part's group unplaced: no offset, or a dead gauge — whose
/// reference is kept, dangling, so the group can name its cause.
#[test]
fn deleting_a_gauge_a_placed_member_or_a_placing_mate_unplaces_the_group() {
    let p = parts("p2-delete");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-delete"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 5.0, 0.0])));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, base, Some(g));
    let doc = set_gauge(doc, top, Some(g));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let o = p.opts();
    for (what, id, cause, unplaced) in [
        ("the gauge", g, Unplaced::DeadGauge { gauge: g }, base),
        ("the placed member", base, Unplaced::NoOffset, top),
        ("the placing mate", mate, Unplaced::NoOffset, top),
    ] {
        let applied = apply(
            &doc,
            &DocEdit::DeleteNode { id },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap_or_else(|e| panic!("deleting {what} is never refused: {e}"));
        assert!(
            applied
                .maintenance
                .iter()
                .all(|row| !matches!(row, Maintenance::OffsetCleared { .. })),
            "{what}: no edit records a frame"
        );
        let poses = solve(&applied.doc, &o, Tol::witness());
        assert_eq!(poses.unplaced(unplaced), Some(cause), "{what}");
        assert!(
            poses.placement(&applied.doc, unplaced).is_err(),
            "{what}: an unplaced group has no world pose"
        );
        // It keeps its shape: it evaluates, in its own space.
        let ev = run(&applied.doc, &o);
        assert!(ev.value(unplaced).is_some(), "{what}: it still evaluates");
        assert_eq!(
            ev.unplaced.get(&unplaced).map(|&(_, c)| c),
            Some(cause),
            "{what}: and the evaluation says which space"
        );
    }
    // The dead reference is KEPT: the instance still names the gauge.
    let (gone, _) = step(doc, DocEdit::DeleteNode { id: g });
    assert_eq!(gone.node(base).and_then(Node::gauge_ref), Some(g));
}

/// **Nothing outside an unplaced group is compared with it.** A world
/// base and a block on a deleted gauge, a mate between them (across
/// gauges, so declaring) and a measure between their caps: the product
/// gathers the world alone, the at-rest gate mints none of the
/// cross-space pair, and the measure refuses typed naming the unplaced
/// group, with the recourse — never a number across spaces.
#[test]
fn an_unplaced_group_is_gathered_minted_and_measured_against_nothing_outside_it() {
    let p = parts("p2-space");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-space"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 0.0, BASE_HEIGHT])));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, top, Some(g));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let measure = || {
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                SitedRef::at_mint(p.base_cap(base)),
                SitedRef::at_mint(p.top_cap(top)),
            ],
        )
        .expect("both indices address a reference")
    };
    let o = p.opts();
    // Placed, across gauges: the measure answers, the mate declares.
    let (measured, at) = insert(doc.clone(), measure());
    let placed = run(&measured, &o);
    assert!(placed.value(at).is_some(), "{:?}", placed.node_error(at));
    assert_eq!(
        solve(&doc, &o, Tol::witness()).role(mate),
        Some(MateRole::Declaring)
    );

    let (doc, _) = step(doc, DocEdit::DeleteNode { id: g });
    let ev = run(&doc, &o);
    let world = |id| {
        topo::mass_properties(body_of(&ev, id).as_ref(), Tol::witness())
            .expect("mass properties")
            .volume
    };
    let gathered = product(&doc, &ev, Tol::witness()).expect("the world gathers");
    let volume = topo::mass_properties(&gathered, Tol::witness())
        .expect("mass properties")
        .volume;
    assert_eq!(
        volume.to_bits(),
        world(base).to_bits(),
        "the product is the world's material alone"
    );
    let minted: Vec<RecipeNodeId> = editor_core::assemble(&doc, &ev, Tol::witness())
        .expect("the gate certifies the world and the top's own space")
        .minted
        .iter()
        .map(|d| d.mate)
        .collect();
    assert!(
        !minted.contains(&mate),
        "the gate mints no pair across spaces: {minted:?}"
    );
    let (doc, measure) = insert(doc, measure());
    let ev = run(&doc, &o);
    let error = ev.node_error(measure).expect("the measure refuses");
    assert!(
        matches!(
            error.kind,
            NodeErrorKind::Unplaced {
                group,
                cause: Unplaced::DeadGauge { gauge },
            } if group == top && gauge == g
        ),
        "{error:?}"
    );
    assert!(
        error.to_string().contains(editor_core::UNPLACED_RECOURSE),
        "{error}"
    );
}

// ---- Row 6: the file ----

/// **A file the registry schema wrote refuses typed, and a dangling
/// gauge reference saves and loads.** An old snapshot's `placements`
/// table and an old log entry's `maintenance` wrapper are each a field
/// this build does not read, so each file refuses `Unreadable` with the
/// regenerate recourse — it never loads with a different meaning. A
/// reference to a deleted gauge is legal state, and round-trips.
#[test]
fn an_old_file_refuses_and_a_dangling_gauge_reference_saves_and_loads() {
    let p = parts("p2-file");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-file"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 5.0, 0.0])));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, base, Some(g));
    let text = save(&doc, &[], Tol::witness()).expect("saves");

    let registry = doctored(&text, |wire| {
        wire["snapshot"]["placements"] = serde_json::json!({});
    });
    assert!(
        matches!(
            load(&registry, Tol::witness()),
            Err(PersistError::Unreadable { .. })
        ),
        "a registry table refuses"
    );
    let logged = doctored(&text, |wire| {
        wire["edits"] = serde_json::json!([{
            "edit": serde_json::to_value(DocEdit::<editor_core::ProfileProgram>::SetGauge {
                node: base,
                gauge: None,
            })
            .expect("serializes"),
            "maintenance": [],
        }]);
    });
    assert!(
        matches!(
            load(&logged, Tol::witness()),
            Err(PersistError::Unreadable { .. })
        ),
        "a maintenance-carrying log entry refuses"
    );

    let (dangling, _) = step(doc, DocEdit::DeleteNode { id: g });
    let text = save(&dangling, &[], Tol::witness()).expect("a dangling reference saves");
    let loaded = load(&text, Tol::witness()).expect("and loads");
    assert!(loaded.doc.bit_eq(&dangling));
    assert_eq!(
        solve(&loaded.doc, &p.opts(), Tol::witness()).unplaced(base),
        Some(Unplaced::DeadGauge { gauge: g })
    );
}

// ---- Row 7: A ∘ F ∘ B, in every lane ----

/// **A transform between a tree mate's operand and its instance,
/// under a parametric gauge, poses alike in every lane.** The base and
/// the top stand on a gauge turned a quarter of a radian about z and
/// lifted by `lift`; the mate reads the top through a `+10 z`
/// transform. The evaluated pose is `A ∘ F ∘ B` — the transform's
/// inverse offset `A`, the gauge's frame `F`, the solved seat `B`:
///
/// - the mated faces seat in the product exactly as on a document with
///   no gauge and no transform (dropping `A` lifts the top 10 off it);
/// - a value edit to `lift` moves both instances by the same amount
///   (dropping `F` moves neither);
/// - the `Interval` lane encloses every point of both bodies the `f64`
///   lane computes (dropping `F` in one lane parts the two).
#[test]
fn a_placer_between_a_tree_mates_operand_and_its_instance_under_a_parametric_gauge_poses_alike_in_every_lane()
 {
    let p = parts("p2-afb");
    let o = p.opts();

    // The control: no gauge, no transform.
    let control = ProfileDoc::empty(DocumentId::derive("p2-afb-control"), Tol::witness());
    let (control, cb) = insert(control, Node::instantiate_part(p.base));
    let (control, ct) = insert(control, Node::instantiate_part(p.top));
    let (control, _) = insert(control, seat(head(p.top_cap(ct)), head(p.base_cap(cb))));
    let seat_control = seat_map(
        &control,
        &run(&control, &o),
        &p.base_cap(cb),
        &p.top_cap(ct),
    );

    let doc = ProfileDoc::empty(DocumentId::derive("p2-afb"), Tol::witness());
    let doc = declare_lift(doc, 2.0);
    let (doc, g) = insert(doc, lifting_gauge(None, 0.25));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, base, Some(g));
    let doc = set_gauge(doc, top, Some(g));
    let (doc, lifted) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, mate) = insert(
        doc,
        seat(head_at(lifted, p.top_cap(top)), head(p.base_cap(base))),
    );
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(mate), None);
    assert_eq!(poses.role(mate), Some(MateRole::Determining));

    let ev = run(&doc, &o);
    assert_seated(
        &doc,
        &ev,
        &p.base_cap(base),
        &p.top_cap(top),
        &seat_control,
        "A ∘ F ∘ B",
    );

    // F: the gauge's parameter moves the whole group.
    let moved = set_lift(doc.clone(), 7.0);
    let ev_moved = run(&moved, &o);
    for id in [base, lifted] {
        let (before, after) = (points(&body_of(&ev, id)), points(&body_of(&ev_moved, id)));
        assert_eq!(before.len(), after.len());
        for (b, a) in before.iter().zip(&after) {
            close(*a, [b[0], b[1], b[2] + 5.0], "the gauge moved the group");
        }
    }

    // Every lane: the interval bodies enclose the f64 ones, tightly.
    let lane = evaluate::<Interval>(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &o,
        Tol::witness(),
    );
    for id in [base, lifted] {
        let (exact, enclosed) = (body_of(&ev, id), body_of(&lane, id));
        let lanes: Vec<_> = enclosed.points().map(|(_, p)| [p.x, p.y, p.z]).collect();
        let exact = points(&exact);
        assert_eq!(exact.len(), lanes.len(), "{id:?}: one body in both lanes");
        for (x, i) in exact.iter().zip(&lanes) {
            for axis in 0..3 {
                let (lo, hi) = (i[axis].lo(), i[axis].hi());
                assert!(
                    lo <= x[axis] && x[axis] <= hi && hi - lo <= 1e-9,
                    "{id:?}: the interval lane's [{lo}, {hi}] does not hold the f64 lane's {}",
                    x[axis]
                );
            }
        }
    }
}

/// **`A` is the identity, bit for bit, where no placer stands on the
/// path.** With no transform and no pattern between the mate and the
/// instance, the world pose is the group's frame composed onto the
/// solved pose and nothing else: `F ∘ B`, bits equal — so no
/// pre-existing pose moves (`msolve1_transform_aware::a7` pins `B`'s
/// own bits).
#[test]
fn with_no_placer_on_the_path_the_world_pose_is_the_frame_onto_the_solve_bit_for_bit() {
    let p = parts("p2-identity");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-identity"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let root =
        Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.5, [2.0, -3.0, 0.25], fixture::band())
            .expect("a literal axis");
    let doc = set_offset(doc, base, Some(Placement::literal(&root)));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, _) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let poses = solve(&doc, &p.opts(), Tol::witness());
    let relative = poses.relative(top).expect("the top solves");
    assert!(
        poses
            .placement(&doc, top)
            .expect("placed")
            .bit_eq(&root.compose(&relative)),
        "F ∘ B, nothing composed beside it"
    );
    assert!(
        poses.placement(&doc, base).expect("placed").bit_eq(&root),
        "the root's own pose is its offset's frame"
    );
}

// ---- Row 8: split and inline, as built in P2-core ----

/// A placed pair — the base at an offset, the top mated onto it — and
/// the store holding its parts.
pub(crate) fn placed_pair(label: &str) -> (Parts, ProfileDoc, [RecipeNodeId; 3]) {
    let p = parts(label);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(doc, base, Some(literal([4.0, 0.0, 0.0])));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    (p, doc, [base, top, mate])
}

pub(crate) fn cut(ids: &[RecipeNodeId]) -> BTreeSet<RecipeNodeId> {
    ids.iter().copied().collect()
}

/// **Gauge references leaving a cut must land on one anchor** (A4). A
/// cut holding a gauge is `p2_split`'s.
#[test]
fn a_cut_landing_on_two_anchors_refuses_typed() {
    let p = parts("p2-split-gauge");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-split-gauge"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 5.0, 0.0])));
    let (doc, on_g) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, on_g, Some(g));
    let (doc, on_world) = insert(doc, Node::instantiate_part(p.top));
    let o = p.opts();
    let split = |ids: &[RecipeNodeId]| {
        editor_core::split(
            &doc,
            &cut(ids),
            DocumentId::derive("p2-split-gauge-part"),
            Tol::witness(),
            o.resolver.as_ref(),
        )
    };
    let err = split(&[on_g, on_world]).expect_err("two anchors");
    assert!(
        matches!(
            &err,
            editor_core::SplitError::TwoAnchors { node, first, second }
                if node.id() == on_world && first.as_ref().map(|f| f.id()) == Some(g) && second.is_none()
        ),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse:"), "{err}");
    // One anchor: the instance left behind names it.
    let out = split(&[on_g]).expect("a cut on one kept gauge splits");
    assert_eq!(
        out.remainder.node(out.instance).and_then(Node::gauge_ref),
        Some(g)
    );
}

/// **A cut whose group is unplaced refuses**: a dead gauge reference has
/// no anchor to give the instance left behind, and a cut of unplaced
/// material alone has nothing to be placed as (A4).
#[test]
fn a_cut_reaching_a_dead_gauge_or_of_unplaced_material_alone_refuses_typed() {
    let p = parts("p2-split-unplaced");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-split-unplaced"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 5.0, 0.0])));
    let (doc, dead) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, dead, Some(g));
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: g });
    let (doc, bare) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_offset(doc, bare, None);
    let o = p.opts();
    let split = |ids: &[RecipeNodeId]| {
        editor_core::split(
            &doc,
            &cut(ids),
            DocumentId::derive("p2-split-unplaced-part"),
            Tol::witness(),
            o.resolver.as_ref(),
        )
    };
    let err = split(&[dead]).expect_err("a dead gauge reference");
    assert!(
        matches!(
            &err,
            editor_core::SplitError::DeadGaugeReference { node, gauge }
                if node.id() == dead && gauge.id() == g
        ),
        "{err:?}"
    );
    let err = split(&[bare]).expect_err("unplaced material alone");
    assert!(
        matches!(&err, editor_core::SplitError::UnplacedAlone { group } if group.id() == bare),
        "{err:?}"
    );
    for err in [split(&[dead]).unwrap_err(), split(&[bare]).unwrap_err()] {
        assert!(err.to_string().contains("Recourse:"), "{err}");
    }
}

/// **A cut of unplaced material alone names the group of its first node
/// in document order**, whatever the ids. A gauge ahead of the two bare
/// instances is moved until the later instance draws the lower id, so a
/// walk over the cut set in id order would name the later group.
#[test]
fn a_cut_of_unplaced_material_alone_names_its_first_group_in_document_order() {
    let p = parts("p2-split-unplaced-order");
    let o = p.opts();
    for k in 0..64u32 {
        let doc = ProfileDoc::empty(
            DocumentId::derive("p2-split-unplaced-order"),
            Tol::witness(),
        );
        let (doc, _) = insert(doc, Node::gauge(None, literal([f64::from(k), 0.0, 0.0])));
        let (doc, first) = insert(doc, Node::instantiate_part(p.base));
        let doc = set_offset(doc, first, None);
        let (doc, second) = insert(doc, Node::instantiate_part(p.top));
        let doc = set_offset(doc, second, None);
        if first < second {
            continue;
        }
        let err = editor_core::split(
            &doc,
            &cut(&[first, second]),
            DocumentId::derive("p2-split-unplaced-order-part"),
            Tol::witness(),
            o.resolver.as_ref(),
        )
        .expect_err("unplaced material alone");
        assert!(
            matches!(&err, editor_core::SplitError::UnplacedAlone { group } if group.id() == first),
            "the refusal names the first group the document holds: {err:?}"
        );
        return;
    }
    panic!("no gauge in 0..64 gave the later bare instance the lower id");
}

/// **The group hoist, and the frame rule at a split** (A4). A cut that
/// is exactly one placed group hoists its root's offset onto the
/// instance left behind; a member carrying a further offset refuses,
/// since the hoist moves the root it was stated against; and a kept
/// mate whose cut side reads an instance that will not be its part
/// group's root at the empty chain refuses naming the mate and side.
#[test]
fn the_group_hoist_and_the_frame_rule_at_a_split() {
    let (p, doc, [base, top, mate]) = placed_pair("p2-hoist");
    let o = p.opts();
    let split = |doc: &ProfileDoc, ids: &[RecipeNodeId]| {
        editor_core::split(
            doc,
            &cut(ids),
            DocumentId::derive("p2-hoist-part"),
            Tol::witness(),
            o.resolver.as_ref(),
        )
    };
    let out = split(&doc, &[base, top, mate]).expect("one whole group");
    assert_eq!(
        offset_of(&out.remainder, out.instance),
        Some(literal([4.0, 0.0, 0.0])),
        "the root's offset hoisted onto the instance"
    );
    assert_eq!(
        offset_of(&out.part, out.node_map[&base]),
        Some(Placement::IDENTITY),
        "and the root lands at the empty chain"
    );

    // A further offset on a member.
    let solved = solve(&doc, &o, Tol::witness())
        .placement(&doc, top)
        .expect("placed");
    let checked = set_offset(doc.clone(), top, Some(Placement::literal(&solved)));
    let err = split(&checked, &[base, top, mate]).expect_err("a further offset");
    assert!(
        matches!(&err, editor_core::SplitError::HoistedMemberOffset { instance } if instance.id() == top),
        "{err:?}"
    );

    // The frame rule: a kept block mated onto the cut top, whose cut
    // side reads the top — which in the part is NOT its group's root.
    let (with_kept, kept) = insert(doc.clone(), Node::instantiate_part(p.top));
    let (with_kept, kept_mate) = insert(
        with_kept,
        seat_on(
            head(p.top_cap(kept)),
            head(p.top_upper_cap(top)),
            [0.0, 0.0, TOP_HEIGHT],
        ),
    );
    let err = split(&with_kept, &[base, top, mate]).expect_err("the torn group, or the frame");
    // The kept block joined the group by its placing mate, so the cut
    // tears it — the frame rule is met on a DECLARING kept mate: put
    // the kept block on a gauge of its own.
    assert!(
        matches!(err, editor_core::SplitError::TornGroup { .. }),
        "{err:?}"
    );
    let (apart, own) = insert(with_kept, Node::gauge(None, literal([0.0, 0.0, 9.0])));
    let apart = set_gauge(apart, kept, Some(own));
    let err = split(&apart, &[base, top, mate]).expect_err("the frame rule");
    assert!(
        matches!(
            &err,
            editor_core::SplitError::MateFrameCrosses { mate: m, side }
                if m.id() == kept_mate && *side == editor_core::MateSide::B
        ),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse:"), "{err}");
}

/// **Inline's sugar and its empty offset** (A4): a root-placed instance
/// over a part that is one group rooted at the empty chain on its
/// world has that root take the instance's offset, and at the empty
/// offset the content lands verbatim. The round trip of the group
/// hoist through the sugar is equal in evaluation (the spec's D1); the
/// minted gauge and the mate-placed inline are `p2_split`'s.
#[test]
fn inline_admits_the_sugar_and_the_empty_offset_and_refuses_the_unplaced_typed() {
    let (p, doc, [base, top, mate]) = placed_pair("p2-inline");
    let o = p.opts();
    let out = editor_core::split(
        &doc,
        &cut(&[base, top, mate]),
        DocumentId::derive("p2-inline-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("one whole group");
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let r = resolver(store.clone());

    // The sugar: the round trip.
    let back = editor_core::inline(&out.remainder, out.instance, &r, Tol::witness())
        .expect("the sugar inlines");
    assert_eq!(groups(&back.doc).len(), 1, "one group again");
    let root = root_of(&back.doc, groups(&back.doc)[0][0]);
    assert_eq!(offset_of(&back.doc, root), Some(literal([4.0, 0.0, 0.0])));
    let volume = |d: &ProfileDoc| {
        topo::mass_properties(
            &product(d, &run(d, &o), Tol::witness()).expect("gathers"),
            Tol::witness(),
        )
        .expect("mass properties")
        .volume
    };
    assert_eq!(volume(&back.doc).to_bits(), volume(&doc).to_bits());

    // The empty offset: verbatim.
    let at_origin = set_offset(
        out.remainder.clone(),
        out.instance,
        Some(Placement::IDENTITY),
    );
    editor_core::inline(&at_origin, out.instance, &r, Tol::witness())
        .expect("the empty offset lands the content verbatim");

    // The refusal: an unplaced instance has no frame to splice at.
    let unplaced = set_offset(out.remainder.clone(), out.instance, None);
    let err = editor_core::inline(&unplaced, out.instance, &r, Tol::witness())
        .expect_err("an unplaced instance");
    assert!(
        matches!(&err, editor_core::InlineError::Unplaced { instance, cause: Unplaced::NoOffset } if instance.id() == out.instance),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse:"), "{err}");
}

/// **A declaring mate crossing a cut fills the instance's interface
/// record** (AQ8). The cut block sits on the world and the kept one on
/// a gauge of its own, so the mate between them declares; it survives
/// the split as one crossing on the instance left behind, and nothing
/// starts placing: the instance sits on the world, the kept block on
/// its gauge.
#[test]
fn a_declaring_mate_crossing_a_cut_fills_the_interface_record() {
    let p = parts("p2-crossing");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-crossing"), Tol::witness());
    let (doc, k) = insert(doc, Node::gauge(None, literal([0.0, 0.0, 0.0])));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, base, Some(k));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let o = p.opts();
    assert_eq!(
        solve(&doc, &o, Tol::witness()).role(mate),
        Some(MateRole::Declaring)
    );
    let out = editor_core::split(
        &doc,
        &cut(&[top]),
        DocumentId::derive("p2-crossing-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("a declaring mate crosses");
    let Some(Node::InstantiatePart { interface, .. }) = out.remainder.node(out.instance) else {
        panic!("the remainder's instance");
    };
    assert_eq!(interface.crossings.len(), 1, "{interface:?}");
    assert!(
        matches!(
            &interface.crossings[0],
            editor_core::InterfaceCrossing::Mate {
                class: ContactClass::Rest,
                ..
            }
        ),
        "{interface:?}"
    );
    assert_eq!(
        out.remainder.node(out.instance).and_then(Node::gauge_ref),
        None
    );
}

/// **A `FromFace` side crosses the seam with its head**: a kept
/// declaring mate whose side reading the cut is framed on its head
/// face crosses split exactly as its authored twin does — the head
/// re-anchors through the instance qualifier, and the face it names
/// in the new part is the same face — and inline carries it back, the
/// head unwrapped onto the inner instance. The mate solves on each
/// document, declaring, with no fault.
#[test]
fn a_from_face_side_crosses_split_and_inline_with_its_head() {
    let p = parts("p2-face-crossing");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("p2-face-crossing"), Tol::witness());
    let (doc, k) = insert(doc, Node::gauge(None, literal([0.0, 0.0, 0.0])));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_gauge(doc, base, Some(k));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let Node::Mate {
        a,
        b,
        class,
        mut alignment,
    } = seat(head(p.top_cap(top)), head(p.base_cap(base)))
    else {
        panic!("a mate");
    };
    alignment.a = MateFrame::FromFace;
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (faced, mate) = fixture::step_with(
        doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Mate {
                a,
                b,
                class,
                alignment,
            }),
        },
        &reach,
    );
    let mate = mate.expect("the mate");
    assert_eq!(
        solve(&faced, &o, Tol::witness()).role(mate),
        Some(MateRole::Declaring)
    );
    let out = editor_core::split(
        &faced,
        &cut(&[top]),
        DocumentId::derive("p2-face-crossing-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("the face side crosses with its head");
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let store_opts = with_resolver(store.clone());
    let Some(Node::Mate {
        a: crossed,
        alignment: crossed_alignment,
        ..
    }) = out.remainder.node(mate)
    else {
        panic!("the kept mate");
    };
    assert_eq!(crossed_alignment.a, MateFrame::FromFace);
    assert_eq!(
        crossed.name.node, out.instance,
        "the head re-anchors through the instance"
    );
    let ev = run(&out.remainder, &store_opts);
    assert!(ev.node_error(mate).is_none(), "{:?}", ev.node_error(mate));
    assert_eq!(
        solve(&out.remainder, &store_opts, Tol::witness()).role(mate),
        Some(MateRole::Declaring)
    );
    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &resolver(store),
        Tol::witness(),
    )
    .expect("and inlines back with its head");
    let Some(Node::Mate {
        alignment: back_alignment,
        ..
    }) = back.doc.node(mate)
    else {
        panic!("the host mate");
    };
    assert_eq!(back_alignment.a, MateFrame::FromFace);
    let ev = run(&back.doc, &o);
    assert!(
        ev.node_error(mate).is_none(),
        "the host mate evaluates clean: {:?}",
        ev.node_error(mate)
    );
}

/// **The fold rule at inline** (A4): two placing mates of one pair —
/// a host block and the instance — re-anchored onto two different
/// inner instances would be two pairs the solve folds apart, so inline
/// refuses naming both mates. The part is two blocks, each its own
/// group at the empty chain, inlined at the empty offset.
#[test]
fn inline_refuses_two_placing_mates_of_one_pair_that_would_read_two_pairs() {
    let p = parts("p2-fold");
    let part = ProfileDoc::empty(DocumentId::derive("p2-fold-part"), Tol::witness());
    let (part, first) = insert(part, Node::instantiate_part(p.base));
    let (part, second) = insert(part, Node::instantiate_part(p.base));
    let mut store = p.store.clone();
    let part_ref = store.insert(part, Tol::witness());

    let host = ProfileDoc::empty(DocumentId::derive("p2-fold"), Tol::witness());
    let (host, h) = insert(host, Node::instantiate_part(part_ref));
    let (host, block) = insert(host, fixture::mated_instance(p.top));
    let through = |inner: RecipeNodeId| StableName {
        kind: editor_core::EntityKind::Face,
        node: h,
        path: vec![editor_core::RoleSeg::InPart {
            of: p.base_cap(inner).into(),
        }],
    };
    let (host, one) = insert(host, seat(head(p.top_cap(block)), head(through(first))));
    let (host, two) = insert(host, seat(head(p.top_cap(block)), head(through(second))));
    let err =
        editor_core::inline(&host, h, &resolver(store), Tol::witness()).expect_err("two pairs");
    assert!(
        matches!(&err, editor_core::InlineError::MatePairSplits { first: f, second: s } if f.id() == one && s.id() == two),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse:"), "{err}");
}

/// **A declaring mate across gauges goes through the gate's existing
/// declared-contact path** (the spec's ruling 8). The top stands on a
/// gauge of its own, posed by its offset exactly where the seat puts
/// it; the mate between it and the world base declares, the gate mints
/// it and certifies the flush contact — what a loop-closing declaring
/// mate within one gauge gets. P2 neither widens nor narrows it.
#[test]
fn a_declaring_mate_across_gauges_is_minted_and_certified_at_rest() {
    let p = parts("p2-cross-gauge");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-cross-gauge"), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 0.0, 0.0])));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, top, Some(g));
    // Where the seat puts the top: the half turn of the opposed seat,
    // its corner on the base's top cap at (1, 1).
    let seated = Frame {
        columns: [[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [1.0, 1.0, BASE_HEIGHT],
    };
    let doc = set_offset(doc, top, Some(Placement::literal(&seated)));
    let (doc, mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let o = p.opts();
    assert_eq!(
        solve(&doc, &o, Tol::witness()).role(mate),
        Some(MateRole::Declaring)
    );
    let assembly = editor_core::assemble(&doc, &run(&doc, &o), Tol::witness())
        .expect("the gate certifies the declared flush contact");
    assert_eq!(
        assembly.minted.iter().map(|d| d.mate).collect::<Vec<_>>(),
        vec![mate]
    );
}

/// **Only a cut of exactly one placed group hoists** (A4). Two placed
/// groups cut together move verbatim: the instance left behind sits at
/// the empty offset and each root keeps its own offset in the part —
/// hoisting either would move the other group.
#[test]
fn a_cut_of_two_placed_groups_moves_verbatim_rather_than_hoisting() {
    let p = parts("p2-two-groups");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-two-groups"), Tol::witness());
    let (doc, first) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(doc, first, Some(literal([4.0, 0.0, 0.0])));
    let (doc, second) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(doc, second, Some(literal([0.0, 9.0, 0.0])));
    let o = p.opts();
    let out = editor_core::split(
        &doc,
        &cut(&[first, second]),
        DocumentId::derive("p2-two-groups-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("two whole groups");
    assert_eq!(
        offset_of(&out.remainder, out.instance),
        Some(Placement::IDENTITY),
        "nothing hoisted"
    );
    assert_eq!(
        offset_of(&out.part, out.node_map[&first]),
        Some(literal([4.0, 0.0, 0.0]))
    );
    assert_eq!(
        offset_of(&out.part, out.node_map[&second]),
        Some(literal([0.0, 9.0, 0.0]))
    );
}

// ---- P2-carry: the carry keeps offsets ----

/// The survey's probe: a placed pair whose top carries a checked
/// offset (its solved world pose), beside a second placed base, so a
/// cut of all four is two groups and moves verbatim. Returns the store,
/// the document, `[base, top, mate, second]` and the top's offset.
fn checked_pair_beside_a_base(
    label: &str,
) -> (Parts, ProfileDoc, [RecipeNodeId; 4], Option<Placement>) {
    let (p, doc, [base, top, mate]) = placed_pair(label);
    let solved = solve(&doc, &p.opts(), Tol::witness())
        .placement(&doc, top)
        .expect("the top is placed");
    let checked = Some(Placement::literal(&solved));
    let doc = set_offset(doc, top, checked.clone());
    let (doc, second) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(doc, second, Some(literal([0.0, 9.0, 0.0])));
    (p, doc, [base, top, mate, second], checked)
}

/// Split's verbatim move of [`checked_pair_beside_a_base`], whole.
fn split_all_four(
    p: &Parts,
    doc: &ProfileDoc,
    ids: [RecipeNodeId; 4],
    part_id: DocumentId,
) -> editor_core::SplitOutcome {
    editor_core::split(
        doc,
        &cut(&ids),
        part_id,
        Tol::witness(),
        p.opts().resolver.as_ref(),
    )
    .expect("two placed groups move verbatim")
}

/// Every instance's offset in `doc` beside the offset `onto` holds at
/// its image under `map`.
fn offsets_through(
    doc: &ProfileDoc,
    map: impl Fn(RecipeNodeId) -> RecipeNodeId,
    onto: &ProfileDoc,
) -> Vec<(Option<Placement>, Option<Placement>)> {
    doc.order()
        .iter()
        .filter(|id| matches!(doc.node(**id), Some(Node::InstantiatePart { .. })))
        .map(|&id| (offset_of(doc, id), offset_of(onto, map(id))))
        .collect()
}

fn cleared(rows: &[Maintenance]) -> Vec<&Maintenance> {
    rows.iter()
        .filter(|row| matches!(row, Maintenance::OffsetCleared { .. }))
        .collect()
}

/// Replays `edits` from `from` with no reach: no store, no solve.
fn replay(from: &ProfileDoc, edits: &[DocEdit<editor_core::ProfileProgram>]) -> ProfileDoc {
    edits.iter().fold(from.clone(), |doc, edit| {
        apply_replayed(&doc, edit, Tol::witness())
            .expect("a recorded edit replays")
            .doc
    })
}

/// **C1 at split: a verbatim move keeps a carried member's checked
/// offset.** The cut mate lands in the part through the mate door,
/// which clears the top's offset as it joins the top to the placed
/// base; the part still holds exactly the source's offsets, and no
/// `OffsetCleared` survives in `part_maintenance`.
#[test]
fn a_verbatim_split_keeps_a_carried_members_checked_offset() {
    let (p, doc, ids, checked) = checked_pair_beside_a_base("p2-carry-split");
    let out = split_all_four(&p, &doc, ids, DocumentId::derive("p2-carry-split-part"));
    assert_eq!(
        offset_of(&out.remainder, out.instance),
        Some(Placement::IDENTITY),
        "the move is verbatim, not a hoist"
    );
    assert_eq!(
        offset_of(&out.part, out.node_map[&ids[1]]),
        checked,
        "the top's checked offset survives the carry"
    );
    for (source, part) in offsets_through(&doc, |id| out.node_map[&id], &out.part) {
        assert_eq!(part, source, "the part holds exactly the source's offsets");
    }
    assert_eq!(
        cleared(&out.part_maintenance),
        Vec::<&Maintenance>::new(),
        "no OffsetCleared for a carried node survives"
    );
}

/// **C1 at inline: an empty-offset inline keeps a carried member's
/// checked offset.** The part is the probe's document itself, so this
/// row reads inline's carry alone, whatever split's keeps.
#[test]
fn an_empty_offset_inline_keeps_a_carried_members_checked_offset() {
    let (p, part, ids, checked) = checked_pair_beside_a_base("p2-carry-inline");
    let mut store = p.store.clone();
    let part_ref = store.insert(part.clone(), Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("p2-carry-inline-host"), Tol::witness());
    let (host, instance) = insert(host, Node::instantiate_part(part_ref));
    assert_eq!(offset_of(&host, instance), Some(Placement::IDENTITY));
    let back = editor_core::inline(&host, instance, &resolver(store), Tol::witness())
        .expect("the empty offset lands the content verbatim");
    let through = |id: RecipeNodeId| back.node_map[&id];
    assert_eq!(
        offset_of(&back.doc, through(ids[1])),
        checked,
        "the top's checked offset survives the splice"
    );
    for (source, spliced) in offsets_through(&part, through, &back.doc) {
        assert_eq!(
            spliced, source,
            "the host holds exactly the source's offsets"
        );
    }
    assert_eq!(
        cleared(&back.maintenance),
        Vec::<&Maintenance>::new(),
        "no OffsetCleared for a carried node survives"
    );
}

/// `inline(split(d))` over a verbatim cut of all of `doc`: the split
/// and the inline each keep every source offset through their node
/// maps (the inline's through the composed map), report no
/// `OffsetCleared`, and replay with no reach to their documents.
fn round_trip_keeps_every_offset(
    p: &Parts,
    doc: &ProfileDoc,
    label: &str,
) -> (editor_core::SplitOutcome, editor_core::InlineOutcome) {
    let part_id = DocumentId::derive(&format!("{label}-part"));
    let out = editor_core::split(
        doc,
        &doc.order().iter().copied().collect(),
        part_id,
        Tol::witness(),
        p.opts().resolver.as_ref(),
    )
    .expect("a cut of whole groups moves verbatim");
    assert_eq!(
        offset_of(&out.remainder, out.instance),
        Some(Placement::IDENTITY),
        "the move is verbatim, not a hoist"
    );
    for (source, part) in offsets_through(doc, |id| out.node_map[&id], &out.part) {
        assert_eq!(part, source, "the part holds exactly the source's offsets");
    }
    assert_eq!(
        cleared(&out.part_maintenance),
        Vec::<&Maintenance>::new(),
        "split: no OffsetCleared for a carried node survives"
    );
    let part = replay(&ProfileDoc::empty(part_id, Tol::witness()), &out.part_edits);
    assert!(part.bit_eq(&out.part), "the part's edit list is the part");

    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &resolver(store),
        Tol::witness(),
    )
    .expect("the empty offset lands the content verbatim");
    let through = |id: RecipeNodeId| back.node_map[&out.node_map[&id]];
    for (source, host) in offsets_through(doc, through, &back.doc) {
        assert_eq!(host, source, "inline(split(d)) holds exactly d's offsets");
    }
    assert_eq!(
        cleared(&back.maintenance),
        Vec::<&Maintenance>::new(),
        "inline: no OffsetCleared for a carried node survives"
    );
    let host = replay(&out.remainder, &back.edits);
    assert!(host.bit_eq(&back.doc), "the inline's edit list is the host");
    (out, back)
}

/// **The carry's edit lists replay to its documents without a solve**:
/// the part from the empty document and the host from the document
/// inlined into, each with no reach, and every source offset comes
/// back through the composed map.
#[test]
fn a_carry_keeping_a_checked_offset_replays_without_a_solve() {
    let (p, doc, ids, checked) = checked_pair_beside_a_base("p2-carry-replay");
    let (out, back) = round_trip_keeps_every_offset(&p, &doc, "p2-carry-replay");
    assert_eq!(
        offset_of(&back.doc, back.node_map[&out.node_map[&ids[1]]]),
        checked,
        "the round trip holds the checked offset"
    );
}

/// **The re-statement waits for the last carried node, covers roots,
/// and states nothing the source does not.** T is inserted first and X
/// seated on T (clearing X); T is then seated on a placed base B,
/// which clears T's whole group; T and X are given their solved poses,
/// so T roots the group. A second group, Y seated on a placed base G,
/// keeps Y at no offset. A carry that re-stated right after each
/// insert would lose X and T to the second mate's clear; one that
/// skipped each group's first member would lose T; one that wrote the
/// empty chain for a missing offset would give Y one.
#[test]
fn a_carry_re_states_after_every_mate_and_only_what_the_source_states() {
    let p = parts("p2-carry-chain");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-carry-chain"), Tol::witness());
    let (doc, t) = insert(doc, Node::instantiate_part(p.top));
    let (doc, x) = insert(doc, Node::instantiate_part(p.top));
    let (doc, _) = insert(
        doc,
        seat_on(
            head(p.top_cap(x)),
            head(p.top_upper_cap(t)),
            [1.0, 1.0, TOP_HEIGHT],
        ),
    );
    assert_eq!(offset_of(&doc, x), None, "the first mate cleared X");
    let (doc, b) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(doc, b, Some(literal([4.0, 0.0, 0.0])));
    let (doc, _) = insert(doc, seat(head(p.top_cap(t)), head(p.base_cap(b))));
    assert_eq!(offset_of(&doc, t), None, "the second mate cleared T");
    let poses = solve(&doc, &p.opts(), Tol::witness());
    let pose = |i| {
        Some(Placement::literal(
            &poses.placement(&doc, i).expect("placed"),
        ))
    };
    let (t_pose, x_pose) = (pose(t), pose(x));
    let doc = set_offset(doc, t, t_pose.clone());
    let doc = set_offset(doc, x, x_pose.clone());
    assert_eq!(root_of(&doc, x), t, "T roots its group");
    let (doc, g) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(doc, g, Some(literal([0.0, 9.0, 0.0])));
    let (doc, y) = insert(doc, Node::instantiate_part(p.top));
    let (doc, _) = insert(doc, seat(head(p.top_cap(y)), head(p.base_cap(g))));
    assert_eq!(offset_of(&doc, y), None, "Y sits at no offset");
    assert_eq!(doc.order().len(), 8, "eight nodes, all cut");

    let (out, back) = round_trip_keeps_every_offset(&p, &doc, "p2-carry-chain");
    let host = |i: RecipeNodeId| back.node_map[&out.node_map[&i]];
    for (what, i, want) in [("T", t, t_pose), ("X", x, x_pose), ("Y", y, None)] {
        assert_eq!(
            offset_of(&out.part, out.node_map[&i]),
            want,
            "{what} in the part"
        );
        assert_eq!(offset_of(&back.doc, host(i)), want, "{what} in the host");
    }
}
