//! **EDIT-PLACEMENT P2 over placements — world pins, seated copies
//! and loose copies** (ASSEMBLY.md A4, A9, A11; D10, Spaces).
//!
//! An instance is a placement: its bundle pins its copy, and an empty
//! bundle leaves it loose, in a space of its own. One world pin puts it
//! in the world at the pin's pose, which may read a document parameter;
//! a seat puts it where its target's copy says. No edit records a
//! solved frame, so removing what pins a copy is never refused and a
//! log replays without a solve; the product is exactly the copies whose
//! space reaches the world, and nothing outside a loose copy's space is
//! compared with it. Every lane poses alike. Split and inline carry
//! every bundle verbatim.
//!
//! The rows build their scenes from two literal block parts: a wide
//! base slab and a tall top block, the top seated on the base by one
//! `Rest` and the values that square it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ExtrudeSide;
use std::collections::BTreeSet;
use std::sync::Arc;

use crate::fixture;
use crate::wire::doctored;

use editor_core::pose::PoseRole;
use editor_core::{
    CapEnd, ContactClass, Coord, Dimension, DocEdit, DocRef, DocumentId, EvalOptions, Evaluation,
    Formula, Frame, FreeValue, FreeVar, Maintenance, MateRef, MeasurePrimitive, Node,
    NodeErrorKind, Operand, PartResolver, PersistError, PoseRefusal, ProfileDoc, ProfileProgram,
    RecipeNodeId, RefusingReach, SitedFace, SitedRef, SlideAxis, Space, StableName, Target, Value,
    ValuePayload, VarName, WorldRead, apply, apply_replayed, evaluate, load, product, save,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, head, insert, len, offset_of, on_frame, run, solve, step, xform};
use geom_core::linalg::Affine3;
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
            profile: profile.into(),
            distance: len(h),
            side: ExtrudeSide::Along,
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

/// **`name`'s world copy by `by`**: the one name in the gathered
/// product's table that is `by`'s copy of `name` — `name` itself where
/// `by` minted it, or `name` under the `Placed` qualifier of the
/// placement `by` ([`StableName::copy_of`]).
fn copy_name(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    name: &StableName,
    by: RecipeNodeId,
) -> StableName {
    let (_, names) =
        editor_core::product_named(doc, ev, Tol::witness()).expect("the product gathers");
    let copies: Vec<StableName> = names
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n.copy_of() == Some((by, name)) || (n == name && name.node == by))
        .collect();
    let [copy] = copies.as_slice() else {
        panic!("{name:?} has one world copy by {by:?}, not {copies:?}");
    };
    copy.clone()
}

/// [`fixture::seat::assert_seated`] over the world copies of `a` by
/// `a_by` and of `b` by `b_by` ([`copy_name`]).
fn assert_seated(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    (a, a_by): (&StableName, RecipeNodeId),
    (b, b_by): (&StableName, RecipeNodeId),
    control: &Affine3<f64>,
    what: &str,
) {
    let (a, b) = (copy_name(doc, ev, a, a_by), copy_name(doc, ev, b, b_by));
    fixture::seat::assert_seated(doc, ev, &a, &b, control, what);
}

/// [`fixture::seat::seat_map`] over the world copies of `a` and `b`,
/// each by the instance that minted it.
fn seat_map(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    a: &StableName,
    b: &StableName,
) -> Affine3<f64> {
    fixture::seat::seat_map(
        doc,
        ev,
        &copy_name(doc, ev, a, a.node),
        &copy_name(doc, ev, b, b.node),
    )
}

/// **"Mate the top to the base"**: the top block's bottom cap seated on
/// the base's top cap with the top's origin at `(1, 1)` of the base's
/// ([`seat_on`]).
pub(crate) fn seat(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    top: SitedFace,
    base: SitedFace,
) -> (ProfileDoc, MateRef) {
    seat_on(doc, placement, top, base, [1.0, 1.0])
}

/// **`mover`'s face seated on `onto`'s at `at`**: the two planes resting
/// against each other, outward normals opposed, and a value on each
/// freedom the rest leaves — the mover's origin `at` from the target's
/// along the plane, and no spin.
pub(crate) fn seat_on(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    mover: SitedFace,
    onto: SitedFace,
    at: [f64; 2],
) -> (ProfileDoc, MateRef) {
    let (doc, mate, _) = seat_edits(doc, placement, mover, onto, at);
    (doc, mate)
}

/// [`seat_on`], with the edits that wrote it, in order: the rest, then
/// its three values.
pub(crate) fn seat_edits(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    mover: SitedFace,
    onto: SitedFace,
    at: [f64; 2],
) -> (ProfileDoc, MateRef, Vec<DocEdit<ProfileProgram>>) {
    let rest = DocEdit::AddConstraint {
        placement,
        mate: Box::new(fixture::rest(mover, onto, ContactClass::Rest)),
        fresh: Vec::new(),
    };
    let applied = apply(&doc, &rest, Tol::witness(), &RefusingReach)
        .unwrap_or_else(|e| panic!("the rest is admitted: {e}"));
    let mate = MateRef::new(placement, applied.record.minted_constraints[0]);
    let mut edits = vec![rest];
    let doc = [
        (Coord::Slide(SlideAxis::X), len(at[0])),
        (Coord::Slide(SlideAxis::Y), len(at[1])),
        (Coord::Spin, ang(0.0)),
    ]
    .into_iter()
    .fold(applied.doc, |doc, (coord, by)| {
        let edit = DocEdit::AddValue {
            placement,
            value: Box::new(Value {
                mate: mate.id,
                coord,
                by,
            }),
            fresh: Vec::new(),
        };
        let doc = step(doc, edit.clone()).0;
        edits.push(edit);
        doc
    });
    (doc, mate, edits)
}

pub(crate) fn lift() -> VarName {
    VarName::from_static("lift")
}

/// The pose a translation by `t` is.
pub(crate) fn literal(t: [f64; 3]) -> Frame {
    Frame::translation(t)
}

pub(crate) fn set_lift(doc: ProfileDoc, value: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetVarValue {
            var: lift().into(),
            value: FreeValue::Continuous(value),
        },
    )
    .0
}

pub(crate) fn declare_lift(doc: ProfileDoc, value: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::DeclareVar {
            name: lift(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, value)),
        },
    )
    .0
}

/// **An instance of `part` pinned to the world at `pose`** over its own
/// geometry ([`fixture::pin`]), `o` resolving the part.
pub(crate) fn pinned(
    doc: ProfileDoc,
    part: DocRef,
    pose: Frame,
    o: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, id) = insert(doc, fixture::mated_instance(part));
    (fixture::pin(doc, id, &pose, o), id)
}

/// **An instance of `part` pinned to the world at `pose` lifted by the
/// document's `lift`**: the pin at `pose` over its geometry, its world
/// read slid along the world's z by `lift`, so one document parameter
/// moves it.
pub(crate) fn pinned_lifting(
    doc: ProfileDoc,
    part: DocRef,
    pose: Frame,
    o: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, id) = insert(doc, fixture::mated_instance(part));
    let copy = Operand::Var(doc.output(id, 0).expect("an instance defines its copies"));
    let mut pin = editor_core::pin_at(&doc, &copy, &pose, o, Tol::witness())
        .unwrap_or_else(|refusal| panic!("the copy pins over its geometry: {refusal}"));
    let Target::World(world) = pin.to else {
        unreachable!("a pin reads the world")
    };
    pin.to = Target::World(world.slide(
        WorldRead::through(PoseRole::Z),
        Formula::named(lift(), Dimension::Length),
    ));
    (fixture::constrain_at(doc, id, pin).0, id)
}

/// **`placement`'s bundle replaced** by one world pin at `pose`, or
/// emptied (a loose copy) for `None`; `o` resolves its part.
pub(crate) fn set_pin(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    pose: Option<Frame>,
    o: &EvalOptions,
) -> ProfileDoc {
    // Values first: a mate goes once no value names it.
    let doc = fixture::bundle(&doc, placement)
        .into_iter()
        .rev()
        .fold(doc, |doc, c| {
            step(
                doc,
                DocEdit::RemoveConstraint {
                    placement,
                    constraint: c.id,
                },
            )
            .0
        });
    match pose {
        Some(pose) => fixture::pin(doc, placement, &pose, o),
        None => doc,
    }
}

/// The one body a node's value holds: a body, or a placement's copies
/// gathered as one.
pub(crate) fn body_of<T: geom_core::Decide>(ev: &Evaluation<T>, id: RecipeNodeId) -> Arc<Body<T>> {
    let one = |bodies: &[Arc<Body<T>>]| {
        let [b] = bodies else {
            panic!("{id:?} holds {} bodies, not one", bodies.len());
        };
        Arc::clone(b)
    };
    match &ev
        .value(id)
        .unwrap_or_else(|| panic!("{id:?} evaluated: {:?}", ev.node_error(id)))
        .payload
    {
        ValuePayload::Body(b) => Arc::clone(b),
        ValuePayload::Copies(c) => one(&c.bodies),
        ValuePayload::Instances(bodies) => one(bodies),
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

// ---- Row 1: a world pin at a parametric pose ----

/// **An instance pinned to the world at a composed parametric pose sits
/// there, and one document parameter moves every pose reading it.** One
/// instance is pinned at `[0, 0, lift]`, another at `[5, 0, 0]` composed
/// with `[0, 1, 0]` and lifted the same. The world pose door and the
/// evaluated bodies agree, and a value edit to `lift` moves both
/// instances by the same amount.
#[test]
fn a_world_pin_at_a_composed_parametric_pose_places_its_instance_and_a_parameter_moves_it() {
    let p = parts("p2-chain");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-chain"), Tol::witness());
    let doc = declare_lift(doc, 2.0);
    let o = p.opts();
    let (doc, outer) = pinned_lifting(doc, p.base, Frame::IDENTITY, &o);
    let (doc, inner) = pinned_lifting(
        doc,
        p.base,
        literal([5.0, 0.0, 0.0]).compose(&literal([0.0, 1.0, 0.0])),
        &o,
    );
    let check = |doc: &ProfileDoc, z: f64| {
        let poses = solve(doc, &o, Tol::witness());
        let at = |id| poses.placement(doc, id).expect("placed").translation;
        close(at(outer), [0.0, 0.0, z], "the lifted instance");
        close(at(inner), [5.0, 1.0, z], "the composed instance");
        let ev = run(doc, &o);
        close(
            min_corner(&body_of(&ev, outer)),
            [0.0, 0.0, z],
            "the lifted body",
        );
        close(
            min_corner(&body_of(&ev, inner)),
            [5.0, 1.0, z],
            "the composed body",
        );
    };
    check(&doc, 2.0);
    let doc = set_lift(doc, 7.0);
    check(&doc, 7.0);
}

// ---- Row 2: insert loose; pin; the constraint door ----

/// **An inserted instance is loose until a constraint pins it, and a
/// constraint added to it places it in an edit that replays without
/// solving.** Pinned at the identity it sits at the world's origin. The
/// seat's edits record no frame, a replay with no reach produces the
/// same document and report, and the whole log saved from empty loads
/// with no store; the seat then places the top in the world.
#[test]
fn an_inserted_instance_sits_at_the_origin_and_a_constraint_places_a_loose_one_replayably() {
    let p = parts("p2-door");
    let o = p.opts();
    let empty = ProfileDoc::empty(DocumentId::derive("p2-door"), Tol::witness());
    let (doc, base) = insert(empty.clone(), Node::instantiate_part(p.base));
    assert!(
        fixture::bundle(&doc, base).is_empty(),
        "an inserted instance is loose"
    );
    let pin = DocEdit::AddConstraint {
        placement: base,
        mate: Box::new(
            editor_core::pin_at(
                &doc,
                &Operand::Var(doc.output(base, 0).expect("an instance defines its copies")),
                &Frame::IDENTITY,
                &o,
                Tol::witness(),
            )
            .expect("the copy pins over its geometry"),
        ),
        fresh: Vec::new(),
    };
    let doc = step(doc, pin.clone()).0;
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    assert!(offset_of(&doc, base).is_some(), "the base is pinned");
    assert_eq!(offset_of(&doc, top), None);
    let poses = solve(&doc, &o, Tol::witness());
    assert!(
        poses.at_identity(base, fixture::band()),
        "a copy pinned at the identity sits at the world's origin"
    );
    assert_eq!(
        poses.space(top),
        Some(Space::Loose { root: top }),
        "an empty bundle is a loose copy"
    );
    let (seated, _, seat) = seat_edits(
        doc.clone(),
        top,
        head(p.top_cap(top)),
        head(p.base_cap(base)),
        [1.0, 1.0],
    );
    let applied = seat.iter().fold(doc.clone(), |doc, edit| {
        let applied = apply(&doc, edit, Tol::witness(), &RefusingReach)
            .expect("a constraint asks no store");
        assert_eq!(
            applied.maintenance,
            Vec::<Maintenance>::new(),
            "the edit records no frame"
        );
        applied.doc
    });
    assert!(applied.bit_eq(&seated));
    assert!(fixture::same_offset(&applied, base, &doc, base));
    let replayed = seat.iter().fold(doc.clone(), |doc, edit| {
        apply_replayed(&doc, edit, Tol::witness())
            .expect("replay needs no reach")
            .doc
    });
    assert!(replayed.bit_eq(&applied), "the replay is the edit");
    let log: Vec<DocEdit<ProfileProgram>> = [
        DocEdit::InsertNode {
            node: Box::new(Node::instantiate_part(p.base)),
            fresh: Vec::new(),
        },
        pin,
        DocEdit::InsertNode {
            node: Box::new(fixture::mated_instance(p.top)),
            fresh: Vec::new(),
        },
    ]
    .into_iter()
    .chain(seat)
    .collect();
    let text = save(&empty, &log, Tol::witness()).expect("the log saves");
    let loaded = load(&text, Tol::witness()).expect("the log loads with no store");
    assert!(
        loaded.doc.bit_eq(&applied),
        "and replays to the same document"
    );
    // The seat places: the top on the base, in the world.
    let poses = solve(&loaded.doc, &o, Tol::witness());
    assert_eq!(poses.fault(top), None);
    assert_eq!(
        poses.space(top),
        Some(Space::World),
        "the seat placed the top"
    );
    close(
        poses
            .placement(&loaded.doc, top)
            .expect("placed")
            .translation,
        [1.0, 1.0, BASE_HEIGHT],
        "the top seats on the base",
    );
}

// ---- Row 3: what is never refused, and the loose copy ----

/// **Removing a world pin or a value of a placing seat is not refused,
/// and what stays placed is recomputed.** Removing the base's world pin
/// leaves the base loose and the top, pinned to it, in the base's
/// space; removing the seat's spin leaves the top loose by itself. Each
/// loose copy still evaluates, in its own space, and has no world pose.
#[test]
fn removing_what_pins_a_copy_is_never_refused_and_leaves_it_loose() {
    let p = parts("p2-delete");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("p2-delete"), Tol::witness());
    let (doc, base) = pinned(doc, p.base, literal([0.0, 5.0, 0.0]), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let pin = fixture::bundle(&doc, base)[0].id;
    let spin = fixture::bundle(&doc, top)
        .iter()
        .find(|c| c.value().is_some_and(|v| v.coord == Coord::Spin))
        .expect("the seat's spin")
        .id;
    for (what, edit, loose) in [
        (
            "the world pin",
            DocEdit::RemoveConstraint {
                placement: base,
                constraint: pin,
            },
            vec![(base, base), (top, base)],
        ),
        (
            "the seat's spin",
            DocEdit::RemoveConstraint {
                placement: top,
                constraint: spin,
            },
            vec![(top, top)],
        ),
    ] {
        let applied = apply(&doc, &edit, Tol::witness(), &RefusingReach)
            .unwrap_or_else(|e| panic!("removing {what} is never refused: {e}"));
        // The only maintenance is the silent retirement of the
        // anonymous variables the removed constraint's numbers read.
        assert!(
            applied
                .maintenance
                .iter()
                .all(Maintenance::is_silent_retirement),
            "{what}: no edit records a frame: {:?}",
            applied.maintenance
        );
        let poses = solve(&applied.doc, &o, Tol::witness());
        let ev = run(&applied.doc, &o);
        for (id, root) in loose {
            assert_eq!(poses.space(id), Some(Space::Loose { root }), "{what}");
            assert!(
                matches!(
                    poses.placement(&applied.doc, id),
                    Err(PoseRefusal::Loose { placement, root: r }) if placement == id && r == root
                ),
                "{what}: a loose copy has no world pose"
            );
            // It keeps its shape: it evaluates, in its own space.
            assert!(ev.value(id).is_some(), "{what}: it still evaluates");
            assert_eq!(
                ev.space(id),
                Space::Loose { root },
                "{what}: and the evaluation says which space"
            );
        }
    }
}

/// **Nothing outside a loose copy's space is compared with it.** A world
/// base and a top: pinned to the world, a measure between their caps
/// answers. With its pin removed the top is loose: the product gathers
/// the world alone, the at-rest gate certifies, and the measure refuses
/// typed naming both spaces, with the recourse — never a number across
/// spaces.
#[test]
fn a_loose_copy_is_gathered_and_measured_against_nothing_outside_it() {
    let p = parts("p2-space");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-space"), Tol::witness());
    let o = p.opts();
    let (doc, base) = fixture::insert_pinned(doc, Node::instantiate_part(p.base), &o);
    let (doc, top) = pinned(doc, p.top, literal([0.0, 0.0, BASE_HEIGHT]), &o);
    let measure = || Node::Measure {
        primitive: MeasurePrimitive::Distance {
            a: SitedRef::at_mint(p.base_cap(base)).into(),
            b: SitedRef::at_mint(p.top_cap(top)).into(),
        },
    };
    // Both in the world: the measure answers.
    let (measured, at) = insert(doc.clone(), measure());
    let placed = run(&measured, &o);
    assert!(placed.value(at).is_some(), "{:?}", placed.node_error(at));

    let doc = set_pin(doc, top, None, &o);
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
    editor_core::assemble(&doc, &ev, Tol::witness())
        .expect("the gate certifies the world and the top's own space");
    let (doc, measure) = insert(doc, measure());
    let ev = run(&doc, &o);
    let error = ev.node_error(measure).expect("the measure refuses");
    let spaces = [Space::World, Space::Loose { root: top }];
    assert!(
        matches!(
            error.kind,
            NodeErrorKind::SpaceMismatch { a, b }
                if (a, b) == (spaces[0], spaces[1]) || (b, a) == (spaces[0], spaces[1])
        ),
        "{error:?}"
    );
    assert!(error.to_string().contains("Recourse:"), "{error}");
}

// ---- Row 4: the file ----

/// **A file the registry schema wrote refuses typed, and a loose
/// instance saves and loads.** An old snapshot's `placements` table and
/// an old log entry's `maintenance` wrapper are each a field this build
/// does not read, so each file refuses `Unreadable` with the regenerate
/// recourse — it never loads with a different meaning. An instance
/// whose bundle is empty is legal state, and round-trips loose.
#[test]
fn an_old_file_refuses_and_a_loose_instance_saves_and_loads() {
    let p = parts("p2-file");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("p2-file"), Tol::witness());
    let (doc, base) = pinned(doc, p.base, literal([0.0, 5.0, 0.0]), &o);
    let pin = fixture::bundle(&doc, base)[0].id;
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
            "edit": serde_json::to_value(
                DocEdit::<editor_core::ProfileProgram>::RemoveConstraint {
                    placement: base,
                    constraint: pin,
                }
            )
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

    let loose = set_pin(doc, base, None, &o);
    let text = save(&loose, &[], Tol::witness()).expect("a loose instance saves");
    let loaded = load(&text, Tol::witness()).expect("and loads");
    assert!(loaded.doc.bit_eq(&loose));
    assert_eq!(
        solve(&loaded.doc, &o, Tol::witness()).space(base),
        Some(Space::Loose { root: base })
    );
}

// ---- Row 5: a parametric pin, in every lane ----

/// **A seat under a parametric world pin poses alike in every lane.**
/// The base is pinned at a pose turned a quarter of a radian about z and
/// lifted by `lift`; the top is seated on it, and a `+10 z` transform of
/// the top is placed in the world beside it:
///
/// - the mated faces seat in the product exactly as on a document whose
///   base is pinned at the identity;
/// - a value edit to `lift` moves every copy by the same amount;
/// - the `Interval` lane encloses every point of each body the `f64`
///   lane computes.
#[test]
fn a_seat_under_a_parametric_pin_poses_alike_in_every_lane() {
    let p = parts("p2-afb");
    let o = p.opts();

    // The control: the base at the identity.
    let control = ProfileDoc::empty(DocumentId::derive("p2-afb-control"), Tol::witness());
    let (control, cb) = fixture::insert_pinned(control, Node::instantiate_part(p.base), &o);
    let (control, ct) = insert(control, fixture::mated_instance(p.top));
    let (control, _) = seat(control, ct, head(p.top_cap(ct)), head(p.base_cap(cb)));
    let seat_control = seat_map(
        &control,
        &run(&control, &o),
        &p.base_cap(cb),
        &p.top_cap(ct),
    );

    let doc = ProfileDoc::empty(DocumentId::derive("p2-afb"), Tol::witness());
    let doc = declare_lift(doc, 2.0);
    let turned = Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.25, [0.0; 3], fixture::band())
        .expect("a literal axis");
    let (doc, base) = pinned_lifting(doc, p.base, turned, &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, mate) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let (doc, lifted) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, lifted_copy) = insert(doc, Node::place(vec![lifted.into()]));
    let doc = fixture::pinned(doc, lifted_copy, &o);
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(mate.placement), None);
    assert_eq!(poses.space(top), Some(Space::World));

    let ev = run(&doc, &o);
    assert_seated(
        &doc,
        &ev,
        (&p.base_cap(base), base),
        (&p.top_cap(top), top),
        &seat_control,
        "the seat under the pin",
    );

    // The pin's parameter moves every copy.
    let moved = set_lift(doc.clone(), 7.0);
    let ev_moved = run(&moved, &o);
    for id in [base, top, lifted_copy] {
        let (before, after) = (points(&body_of(&ev, id)), points(&body_of(&ev_moved, id)));
        assert_eq!(before.len(), after.len());
        for (b, a) in before.iter().zip(&after) {
            close(*a, [b[0], b[1], b[2] + 5.0], "the pin moved the copy");
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
    for id in [base, top, lifted_copy] {
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

// ---- Row 6: split and inline ----

/// A placed pair — the base pinned to the world at `[4, 0, 0]`, the top
/// seated onto it — the store holding its parts, `[base, top]` and the
/// seat's rest.
pub(crate) fn placed_pair(label: &str) -> (Parts, ProfileDoc, [RecipeNodeId; 2], MateRef) {
    let p = parts(label);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = pinned(doc, p.base, literal([4.0, 0.0, 0.0]), &p.opts());
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, mate) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    (p, doc, [base, top], mate)
}

/// **A cut of `ids` in `doc`, with the world copies of what it
/// moves**: each `Part` selecting a copy of a cut pattern, then every
/// world placement of a cut body ([`fixture::with_placements`]) — a
/// part delivers only its world, so a split moves a body with its
/// placements.
pub(crate) fn cut(doc: &ProfileDoc, ids: &[RecipeNodeId]) -> BTreeSet<RecipeNodeId> {
    let mut out: BTreeSet<RecipeNodeId> = ids.iter().copied().collect();
    for id in doc.ids() {
        if let Some(Node::Part { of, .. }) = doc.node(id)
            && doc.operation_of(*of).is_some_and(|at| out.contains(&at))
        {
            out.insert(id);
        }
    }
    fixture::with_placements(doc, &out)
}

/// **Whether the instance `out` leaves behind sits on the world at the
/// identity**, its part resolved beside `p`'s.
fn left_at_identity(p: &Parts, out: &editor_core::SplitOutcome) -> bool {
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    solve(&out.remainder, &with_resolver(store), Tol::witness())
        .at_identity(out.instance, fixture::band())
}

/// **Each of `pattern`'s `count` copies placed in the world**, in
/// instance order: a `Part` per copy, each placed at the identity (A10).
pub(crate) fn place_copies(doc: ProfileDoc, pattern: RecipeNodeId, count: i64) -> ProfileDoc {
    (0..count).fold(doc, |doc, i| {
        let (doc, copy) = insert(
            doc,
            Node::Part {
                of: pattern.into(),
                select: editor_core::PartSelect::Instance(Formula::count(i)),
            },
        );
        fixture::place(doc, copy).0
    })
}

/// **A cut of one placed pair moves as selected** (A4). The instance
/// left behind sits on the world at the identity, the base keeps its
/// world pin in the part, and the top keeps its seat.
#[test]
fn a_cut_of_one_placed_pair_moves_its_bundles_as_they_are() {
    let (p, doc, [base, top], _) = placed_pair("p2-one-group");
    let o = p.opts();
    let out = editor_core::split(
        &doc,
        &cut(&doc, &[base, top]),
        DocumentId::derive("p2-one-group-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("one placed pair");
    assert!(
        left_at_identity(&p, &out),
        "the instance sits on the world at the identity"
    );
    assert!(
        offset_of(&doc, base).is_some()
            && fixture::same_offset(&out.part, out.node_map[&base], &doc, base),
        "and the base keeps its world pin in the part"
    );
    let top_bundle = fixture::bundle(&out.part, out.node_map[&top]);
    assert_eq!(
        top_bundle.len(),
        fixture::bundle(&doc, top).len(),
        "the top keeps its seat"
    );
    assert_eq!(offset_of(&out.part, out.node_map[&top]), None);
}

/// **Inline at the identity, and at any other pose** (A4): at the
/// identity the content lands verbatim, so the round trip of a cut
/// returns the pair with the base's pin; at another pose the part's
/// world pins move by the instance's solved motion and nothing in the
/// part moves relative to it. An instance with an empty bundle, over a
/// part that is not one instance at the identity, has no placement to
/// hand its bundle to and refuses typed.
#[test]
fn inline_lands_verbatim_at_the_identity_and_composes_the_pin_at_any_other_pose() {
    let (p, doc, [base, top], _) = placed_pair("p2-inline");
    let o = p.opts();
    let out = editor_core::split(
        &doc,
        &cut(&doc, &[base, top]),
        DocumentId::derive("p2-inline-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("one placed pair");
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let r = resolver(store.clone());
    let through =
        |back: &editor_core::InlineOutcome, id: RecipeNodeId| back.node_map[&out.node_map[&id]];

    // The identity: the round trip.
    let back = editor_core::inline(&out.remainder, out.instance, &r, Tol::witness())
        .expect("the identity inlines");
    assert!(
        fixture::same_offset(&back.doc, through(&back, base), &doc, base),
        "the base's pin comes back as it was"
    );
    let volume = |d: &ProfileDoc| {
        topo::mass_properties(
            &product(d, &run(d, &o), Tol::witness()).expect("gathers"),
            Tol::witness(),
        )
        .expect("mass properties")
        .volume
    };
    assert_eq!(volume(&back.doc).to_bits(), volume(&doc).to_bits());

    // Another pose: the instance's pin composes into the base's.
    let store_opts = with_resolver(store.clone());
    let shifted = set_pin(
        out.remainder.clone(),
        out.instance,
        Some(literal([0.0, 5.0, 0.0])),
        &store_opts,
    );
    let moved = editor_core::inline(&shifted, out.instance, &r, Tol::witness())
        .expect("a pinned instance inlines at any pose");
    let poses = solve(&moved.doc, &o, Tol::witness());
    close(
        poses
            .placement(&moved.doc, through(&moved, base))
            .expect("placed")
            .translation,
        [4.0, 5.0, 0.0],
        "the base sits at the instance's pose composed with its own",
    );
    assert_eq!(
        poses.space(through(&moved, top)),
        Some(Space::World),
        "and the top still seats on it"
    );

    // The refusal: a loose instance over a pair.
    let loose = set_pin(out.remainder.clone(), out.instance, None, &store_opts);
    let err = editor_core::inline(&loose, out.instance, &r, Tol::witness())
        .expect_err("a loose instance over a pair");
    assert!(
        matches!(&err, editor_core::InlineError::UnplaceablePart { instance, .. } if instance.id() == out.instance),
        "{err:?}"
    );
    assert!(err.to_string().contains("Recourse:"), "{err}");
}

/// **A face a seat reads crosses the seam with its head**: a kept top
/// seated on a cut base, its target reading the base's top face, crosses
/// split — the head re-anchors through the instance qualifier, and the
/// face it names in the new part is the same face — and inline carries
/// it back, the head unwrapped onto the inner instance. The seat solves
/// on each document with no fault.
#[test]
fn a_face_target_crosses_split_and_inline_with_its_head() {
    let p = parts("p2-face-crossing");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("p2-face-crossing"), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, Node::instantiate_part(p.base), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, mate) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    assert_eq!(mate.placement, top, "the kept top holds the constraint");
    assert_eq!(solve(&doc, &o, Tol::witness()).fault(top), None);
    let out = editor_core::split(
        &doc,
        &cut(&doc, &[base]),
        DocumentId::derive("p2-face-crossing-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("the face crosses with its head");
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let store_opts = with_resolver(store.clone());
    let crossed = fixture::constraint(&out.remainder, mate);
    let Target::Pose(target) = crossed.to else {
        panic!("the crossed seat reads a pose: {crossed:?}");
    };
    let [side] = fixture::face_reads(&out.remainder, target)[..] else {
        panic!("the crossed target reads one face: {crossed:?}");
    };
    assert_eq!(
        fixture::side_head(&out.remainder, side).name.node,
        out.instance,
        "the head re-anchors through the instance"
    );
    let ev = run(&out.remainder, &store_opts);
    assert!(ev.node_error(top).is_none(), "{:?}", ev.node_error(top));
    assert_eq!(
        solve(&out.remainder, &store_opts, Tol::witness()).fault(top),
        None
    );
    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &resolver(store),
        Tol::witness(),
    )
    .expect("and inlines back with its head");
    let host = fixture::constraint(&back.doc, mate);
    let Target::Pose(target) = host.to else {
        panic!("the host seat reads a pose: {host:?}");
    };
    let [side] = fixture::face_reads(&back.doc, target)[..] else {
        panic!("the host target reads one face: {host:?}");
    };
    assert_eq!(
        fixture::side_head(&back.doc, side).name.node,
        back.node_map[&out.node_map[&base]],
        "the head unwraps onto the inner instance"
    );
    let ev = run(&back.doc, &o);
    assert!(
        ev.node_error(top).is_none(),
        "the host constraint evaluates clean: {:?}",
        ev.node_error(top)
    );
}

/// **A cut of two pinned instances moves verbatim** (A4): the instance
/// left behind sits on the world at the identity and each keeps its own
/// world pin in the part.
#[test]
fn a_cut_of_two_pinned_instances_moves_verbatim() {
    let p = parts("p2-two-groups");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("p2-two-groups"), Tol::witness());
    let (doc, first) = pinned(doc, p.base, literal([4.0, 0.0, 0.0]), &o);
    let (doc, second) = pinned(doc, p.base, literal([0.0, 9.0, 0.0]), &o);
    let out = editor_core::split(
        &doc,
        &cut(&doc, &[first, second]),
        DocumentId::derive("p2-two-groups-part"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("two pinned instances");
    assert!(
        left_at_identity(&p, &out),
        "the instance on the world at the identity"
    );
    for id in [first, second] {
        assert!(
            offset_of(&doc, id).is_some()
                && fixture::same_offset(&out.part, out.node_map[&id], &doc, id),
            "each keeps its own world pin"
        );
    }
}

// ---- P2-carry: the carry keeps every bundle ----

/// The survey's probe: a placed pair beside a second pinned base; a cut
/// of all three moves verbatim. Returns the store, the document and
/// `[base, top, second]`.
fn pair_beside_a_base(label: &str) -> (Parts, ProfileDoc, [RecipeNodeId; 3]) {
    let (p, doc, [base, top], _) = placed_pair(label);
    let (doc, second) = pinned(doc, p.base, literal([0.0, 9.0, 0.0]), &p.opts());
    (p, doc, [base, top, second])
}

/// Every instance's world pin in `doc` beside the one `onto` holds at
/// its image under `map`.
fn pins_through(
    doc: &ProfileDoc,
    map: impl Fn(RecipeNodeId) -> RecipeNodeId,
    onto: &ProfileDoc,
) -> Vec<(Option<WorldRead<Formula>>, Option<WorldRead<Formula>>)> {
    doc.ids()
        .iter()
        .filter(|id| matches!(doc.node(**id), Some(Node::InstantiatePart { .. })))
        .map(|&id| (offset_of(doc, id), offset_of(onto, map(id))))
        .collect()
}

/// Every placement's bundle size in `doc` beside its image's in `onto`.
fn bundles_through(
    doc: &ProfileDoc,
    map: impl Fn(RecipeNodeId) -> RecipeNodeId,
    onto: &ProfileDoc,
) -> Vec<(usize, usize)> {
    doc.placements()
        .into_iter()
        .map(|id| {
            (
                fixture::bundle(doc, id).len(),
                fixture::bundle(onto, map(id)).len(),
            )
        })
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

/// **C1 at split: a verbatim move keeps every bundle.** The part holds
/// exactly the source's world pins and each placement's constraints, and
/// the split reports no maintenance for a carried node.
#[test]
fn a_verbatim_split_keeps_every_bundle() {
    let (p, doc, ids) = pair_beside_a_base("p2-carry-split");
    let out = editor_core::split(
        &doc,
        &cut(&doc, &ids),
        DocumentId::derive("p2-carry-split-part"),
        Tol::witness(),
        p.opts().resolver.as_ref(),
    )
    .expect("the placements move verbatim");
    assert!(left_at_identity(&p, &out), "the move is verbatim");
    for (source, part) in pins_through(&doc, |id| out.node_map[&id], &out.part) {
        assert_eq!(part, source, "the part holds exactly the source's pins");
    }
    for (source, part) in bundles_through(&doc, |id| out.node_map[&id], &out.part) {
        assert_eq!(part, source, "and each placement's constraints");
    }
    assert_eq!(out.part_maintenance, Vec::<Maintenance>::new());
}

/// **C1 at inline: an inline at the identity keeps every bundle.** The
/// part is the probe's document itself, so this row reads inline's
/// carry alone, whatever split's keeps.
#[test]
fn an_inline_at_the_identity_keeps_every_bundle() {
    let (p, part, _) = pair_beside_a_base("p2-carry-inline");
    let mut store = p.store.clone();
    let part_ref = store.insert(part.clone(), Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("p2-carry-inline-host"), Tol::witness());
    let (host, instance) = fixture::insert_pinned(
        host,
        Node::instantiate_part(part_ref),
        &with_resolver(store.clone()),
    );
    let back = editor_core::inline(&host, instance, &resolver(store), Tol::witness())
        .expect("the identity lands the content verbatim");
    let through = |id: RecipeNodeId| back.node_map[&id];
    for (source, spliced) in pins_through(&part, through, &back.doc) {
        assert_eq!(spliced, source, "the host holds exactly the source's pins");
    }
    for (source, spliced) in bundles_through(&part, through, &back.doc) {
        assert_eq!(spliced, source, "and each placement's constraints");
    }
    assert_eq!(back.maintenance, Vec::<Maintenance>::new());
}

/// **The carry's edit lists replay to its documents without a solve**:
/// `inline(split(d))` over a cut of all of `d`, the part replayed from
/// the empty document and the host from the document inlined into,
/// each with no reach, and every source pin comes back through the
/// composed map.
#[test]
fn a_carry_replays_without_a_solve() {
    let (p, doc, _) = pair_beside_a_base("p2-carry-replay");
    let part_id = DocumentId::derive("p2-carry-replay-part");
    let all: BTreeSet<RecipeNodeId> = doc
        .ids()
        .iter()
        .copied()
        .filter(|&id| id != RecipeNodeId::WORLD)
        .collect();
    let out = editor_core::split(
        &doc,
        &all,
        part_id,
        Tol::witness(),
        p.opts().resolver.as_ref(),
    )
    .expect("a cut of every placement moves verbatim");
    assert!(left_at_identity(&p, &out), "the move is verbatim");
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
    .expect("the identity lands the content verbatim");
    let through = |id: RecipeNodeId| back.node_map[&out.node_map[&id]];
    for (source, host) in pins_through(&doc, through, &back.doc) {
        assert_eq!(host, source, "inline(split(d)) holds exactly d's pins");
    }
    let host = replay(&out.remainder, &back.edits);
    assert!(host.bit_eq(&back.doc), "the inline's edit list is the host");
}
