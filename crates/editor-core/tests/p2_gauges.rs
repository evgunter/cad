//! **EDIT-PLACEMENT P2 over placements — world mates, seated copies
//! and spaces** (ASSEMBLY.md A4, A9, A11; D10, Spaces).
//!
//! An instance is a placement: its bundle pins its copy into the space
//! it names. One world mate puts it in the world at the pin's pose,
//! which may read a document parameter; a seat puts it where its
//! target's copy says. No edit records a solved frame, so a log replays
//! without a solve; every bundle pins its copy, so removing what pins
//! one refuses; the product is exactly the copies placed into the
//! world, and nothing outside a space is compared with it. Every lane
//! poses alike. Split and inline carry every bundle verbatim.
//!
//! The rows build their scenes from two literal block parts: a wide
//! base slab and a tall top block, the top seated on the base by one
//! `Rest` and the values that square it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::fixture;
use crate::wire::doctored;

use editor_core::{
    CapEnd, Coord, Dimension, DocEdit, DocRef, DocumentId, EvalOptions, Evaluation, Formula, Frame,
    FreeValue, FreeVar, Maintenance, MateRef, MeasurePrimitive, Node, PartResolver, PersistError,
    ProfileDoc, ProfileProgram, RecipeNodeId, SitedFace, StableName, TargetAxis, ValuePayload,
    VarId, VarName, apply, apply_replayed, evaluate, load, product, save,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, head, head_at, insert, len, offset_of, on_frame, run, solve, step};
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
    pub(crate) fn base_cap(&self, base: RecipeNodeId) -> crate::fixture::resolver::PartName {
        in_part(base, self.base_body, CapEnd::End)
    }
    /// The top block's bottom cap.
    pub(crate) fn top_cap(&self, top: RecipeNodeId) -> crate::fixture::resolver::PartName {
        in_part(top, self.top_body, CapEnd::Start)
    }
    /// The top block's upper cap, which a second block seats on.
    pub(crate) fn top_upper_cap(&self, top: RecipeNodeId) -> crate::fixture::resolver::PartName {
        in_part(top, self.top_body, CapEnd::End)
    }
}

/// **`name`'s product name, read at `by`** ([`fixture::product_name`]).
fn copy_name(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    name: &StableName,
    by: RecipeNodeId,
) -> StableName {
    fixture::product_name(doc, ev, (by, name.clone()))
}

/// [`fixture::seat::assert_seated`] over the product names of `a` read
/// at `a_by` and of `b` read at `b_by` ([`copy_name`]).
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

/// [`fixture::seat::seat_map`] over the product names of `a` and `b`,
/// each read where it is read.
fn seat_map(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    a: &(impl fixture::Sited + Clone),
    b: &(impl fixture::Sited + Clone),
) -> Affine3<f64> {
    fixture::seat::seat_map(
        doc,
        ev,
        &fixture::product_name(doc, ev, a.clone()),
        &fixture::product_name(doc, ev, b.clone()),
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

/// [`seat_on`], with the edits that wrote it, in order: the rest and
/// its three values as one bundle, after a re-target where the rest
/// reads another space ([`fixture::bundle_edits`]).
pub(crate) fn seat_edits(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    mover: SitedFace,
    onto: SitedFace,
    at: [f64; 2],
) -> (ProfileDoc, MateRef, Vec<DocEdit<ProfileProgram>>) {
    let placed = editor_core::Placed {
        mates: vec![fixture::rest(mover, onto)],
        values: vec![
            (Coord::Slide(TargetAxis::X), len(at[0])),
            (Coord::Slide(TargetAxis::Y), len(at[1])),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    };
    let edits = fixture::bundle_edits(&doc, placement, placed.bundle());
    let mut doc = doc;
    let mut mate = None;
    for edit in &edits {
        let applied = apply(&doc, edit, Tol::witness())
            .unwrap_or_else(|e| panic!("the seat is admitted: {e}"));
        if let DocEdit::SetConstraints { .. } = edit {
            mate = Some(MateRef::new(
                placement,
                applied.record.minted_constraints[0],
            ));
        }
        doc = applied.doc;
    }
    (doc, mate.expect("the seat writes its rest"), edits)
}

/// [`seat_on`] with the spin it means too: the rest, the slides `at`
/// and the spin `turn` (radians) about the seat's normal, each on the
/// target space's axes (D′2: a value charts on the space, not on the
/// body it reads).
pub(crate) fn seat_turned(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    mover: SitedFace,
    onto: SitedFace,
    at: [f64; 2],
    turn: f64,
) -> (ProfileDoc, MateRef) {
    let placed = editor_core::Placed {
        mates: vec![fixture::rest(mover, onto)],
        values: vec![
            (Coord::Slide(TargetAxis::X), len(at[0])),
            (Coord::Slide(TargetAxis::Y), len(at[1])),
            (Coord::SPIN, ang(turn)),
        ],
    };
    let (doc, ids) = fixture::write_bundle(doc, placement, placed.bundle());
    (doc, ids[0])
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
    let _ = o;
    // The pose's moves on the world's axes, its slide up the world's
    // `z` lifted by the variable.
    let moves = editor_core::test_support::moves_at(&pose)
        .into_iter()
        .filter(|(coord, _)| *coord != Coord::Slide(TargetAxis::Z))
        .chain(std::iter::once((
            Coord::Slide(TargetAxis::Z),
            Formula::add(
                len(pose.translation[2]),
                Formula::named(lift(), Dimension::Length),
            )
            .expect("two lengths add"),
        )))
        .collect();
    let placed = editor_core::Placed::of(&doc, Vec::new(), moves);
    (fixture::written(doc, id, placed), id)
}

/// **`placement`'s bundle replaced** by one world mate at `pose`, or
/// for `None` the copy moved into a space of its own: a `new_space()`
/// it stands in at the identity ([`fixture::into_new_space`]); `o`
/// resolves its part.
pub(crate) fn set_pin(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    pose: Option<Frame>,
    o: &EvalOptions,
) -> ProfileDoc {
    match pose {
        Some(pose) => fixture::pin(doc, placement, &pose, o),
        None => fixture::into_new_space(doc, placement).0,
    }
}

/// **Whether the interval lane may escalate where a placement maps a
/// copy pinned over its geometry** (the retired pin-at helper at a pose off the
/// identity): at ε = 1e-12 the copy carries its geometry's enclosure, a
/// few ε wide, into the map, and certifying the placed copy's moved edges
/// escalates at some values of the pin's parameters
/// (`work/intent/a-pin-read-off-a-copys-geometry-carries-its-width-into-the-copy.md`).
pub(crate) fn a_pinned_copy_escalates_its_transform() -> bool {
    geom_core::Tol::witness().eps().to_bits() == 1e-12f64.to_bits()
}

/// **`check` where `id` evaluates, else the run's escalation**: at the
/// ε where a copy pinned over its geometry may escalate
/// ([`a_pinned_copy_escalates_its_transform`]), a node that holds no
/// value is held to an escalation the run made (a refusal of its own
/// or of a node it is poisoned through), never another refusal; at
/// every other ε, and wherever it evaluates, `check` runs.
pub(crate) fn encloses_or_escalates<T: geom_core::Decide>(
    ev: &Evaluation<T>,
    id: RecipeNodeId,
    check: impl FnOnce(),
) {
    if a_pinned_copy_escalates_its_transform() && ev.value(id).is_none() {
        assert!(
            ev.order
                .iter()
                .filter_map(|&at| ev.node_error(at))
                .any(|error| !error.escalations.is_empty()),
            "{id:?} holds no value, and the run escalated nowhere: {:?}",
            ev.node_error(id)
        );
    } else {
        check();
    }
}

/// The one body a node's value holds: a body, or a placement's copies
/// gathered as one.
pub(crate) fn body_of<T: geom_core::Decide + topo::AtRestPolicy>(
    ev: &Evaluation<T>,
    id: RecipeNodeId,
) -> Arc<Body<T>> {
    let one = |bodies: &[editor_core::Framed<T>]| {
        let [b] = bodies else {
            panic!("{id:?} holds {} bodies, not one", bodies.len());
        };
        Arc::new(crate::fixture::in_space(b))
    };
    match &ev
        .value(id)
        .unwrap_or_else(|| panic!("{id:?} evaluated: {:?}", ev.node_error(id)))
        .payload
    {
        ValuePayload::Body(b) => Arc::new(crate::fixture::in_space(b)),
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

// ---- Row 1: a world mate at a parametric pose ----

/// **An instance pinned to the world at a composed parametric pose sits
/// there, and one document parameter moves every pose reading it.** One
/// instance is pinned at `[0, 0, lift]`, another at `[5, 0, 0]` composed
/// with `[0, 1, 0]` and lifted the same. The world pose door and the
/// evaluated bodies agree, and a value edit to `lift` moves both
/// instances by the same amount.
#[test]
fn values_at_a_composed_parametric_pose_places_its_instance_and_a_parameter_moves_it() {
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

/// **An inserted instance stands in the world by the identity bundle
/// until its bundle is written, and the seat written to it places it in
/// an edit that replays without solving.** Pinned at the identity it sits at the world's origin. The
/// seat's edits record no frame, a replay with no reach produces the
/// same document and report, and the whole log saved from empty loads
/// with no store; the seat then places the top in the world.
#[test]
fn an_inserted_instance_sits_at_the_origin_and_a_constraint_places_a_loose_one_replayably() {
    let p = parts("p2-door");
    let o = p.opts();
    let empty = ProfileDoc::empty(DocumentId::derive("p2-door"), Tol::witness());
    // Inserted through the door alone: the log below is these edits,
    // and a fixture insert would write the product beside them.
    let insert = |doc: ProfileDoc, node| {
        let applied = apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect("an instance inserts");
        let id = applied.record.minted.expect("an insert mints its node");
        (applied.doc, id)
    };
    let (doc, base) = insert(empty.clone(), crate::fixture::mated_instance(p.base));
    assert_eq!(
        fixture::bundle(&doc, base).len(),
        6,
        "an inserted instance stands by the identity bundle"
    );
    let typed: Vec<(Coord, Formula)> = TargetAxis::ALL
        .into_iter()
        .map(|axis| (Coord::Slide(axis), Formula::zero(Dimension::Length)))
        .chain(
            TargetAxis::ALL
                .into_iter()
                .map(|axis| (Coord::Spin(Some(axis)), Formula::zero(Dimension::Angle))),
        )
        .collect();
    let pin = editor_core::Placed::of(&doc, Vec::new(), typed).write(base);
    let doc = step(doc, pin.clone()).0;
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    assert!(offset_of(&doc, base).is_some(), "the base is pinned");
    assert!(
        fixture::bundle(&doc, top)
            .iter()
            .all(|c| c.mate().is_none()),
        "the top stands by the identity bundle"
    );
    let poses = solve(&doc, &o, Tol::witness());
    let at = poses.relative(base).expect("the base is placed");
    let off = at
        .columns
        .iter()
        .flatten()
        .zip([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
        .map(|(a, b)| (a - b).abs())
        .chain(at.translation.iter().map(|t| t.abs()))
        .fold(0.0_f64, f64::max);
    assert!(
        off < 1e-12,
        "a copy pinned at the identity sits at the world's origin: {at:?}"
    );
    assert_eq!(
        poses.space(top),
        Some(VarId::WORLD),
        "an inserted copy stands in the world"
    );
    let (seated, _, seat) = seat_edits(
        doc.clone(),
        top,
        head(p.top_cap(top)),
        head(p.base_cap(base)),
        [1.0, 1.0],
    );
    let applied = seat.iter().fold(doc.clone(), |doc, edit| {
        let applied = apply(&doc, edit, Tol::witness()).expect("a constraint asks no store");
        assert!(
            applied
                .maintenance
                .iter()
                .all(Maintenance::is_silent_retirement),
            "the edit records no frame: {:?}",
            applied.maintenance
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
            node: Box::new(crate::fixture::mated_instance(p.base)),
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
        Some(VarId::WORLD),
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

// ---- Row 3: every bundle pins its copy ----

/// **Removing a value of a pin or of a placing seat writes its freedom
/// back as `0`** (D10 `[placement.placement]`: every bundle pins its
/// copy; D′2: the door rewrites the bundle whole). Taking out the base's
/// slide along `y` puts it back on the world's `x` axis, and taking out
/// the seat's spin, already `0`, moves nothing: each bundle still pins
/// its copy, the removed freedom written as the constant `0`.
///
/// *Breaks if* the removal leaves a freedom unwritten, or writes it as
/// anything but `0`.
#[test]
fn removing_a_value_writes_its_freedom_zero() {
    let p = parts("p2-delete");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("p2-delete"), Tol::witness());
    let (doc, base) = pinned(doc, p.base, literal([0.0, 5.0, 0.0]), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = seat(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let value = |placement, coord| {
        fixture::bundle(&doc, placement)
            .iter()
            .find(|c| c.value().is_some_and(|v| v.coord == coord))
            .expect("the value")
            .id
    };
    for (what, placement, coord, landed) in [
        (
            "the pin's slide along y",
            base,
            Coord::Slide(TargetAxis::Y),
            [0.0, 0.0, 0.0],
        ),
        ("the seat's spin", top, Coord::SPIN, [1.0, 1.0, BASE_HEIGHT]),
    ] {
        let removed = apply(
            &doc,
            &DocEdit::RemoveConstraint {
                placement,
                constraint: value(placement, coord),
            },
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("{what}: removing it is admitted: {e}"))
        .doc;
        let written = fixture::bundle(&removed, placement)
            .iter()
            .find_map(|c| c.value().filter(|v| v.coord == coord).map(|v| v.by))
            .expect("the freedom is written again");
        assert!(
            removed
                .var(written)
                .and_then(|v| v.def().defined())
                .is_some_and(|e| e.is_zero()),
            "{what}: written as 0"
        );
        let poses = solve(&removed, &o, Tol::witness());
        assert_eq!(
            poses.fault(placement),
            None,
            "{what}: the copy stays pinned"
        );
        assert_eq!(
            poses
                .placement(&removed, placement)
                .expect("placed")
                .translation,
            landed,
            "{what}: where the 0 puts it"
        );
    }
}

/// **Nothing outside a space is compared with it.** A world base and a
/// top: pinned to the world, a measure between their caps answers.
/// With the top moved into a space of its own, the product gathers the
/// world alone, the at-rest gate certifies, and the edit door refuses
/// the measure typed naming both spaces, with the recourse — never a
/// number across spaces.
#[test]
fn a_loose_copy_is_gathered_and_measured_against_nothing_outside_it() {
    let p = parts("p2-space");
    let doc = ProfileDoc::empty(DocumentId::derive("p2-space"), Tol::witness());
    let o = p.opts();
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(p.base), &o);
    let (doc, top) = pinned(doc, p.top, literal([0.0, 0.0, BASE_HEIGHT]), &o);
    let measure = || Node::Measure {
        primitive: MeasurePrimitive::Distance {
            a: crate::fixture::sited_ref(p.base_cap(base)).into(),
            b: crate::fixture::sited_ref(p.top_cap(top)).into(),
        },
    };
    // Both in the world: the measure answers.
    let (measured, at) = insert(doc.clone(), measure());
    let placed = run(&measured, &o);
    assert!(placed.value(at).is_some(), "{:?}", placed.node_error(at));

    let doc = set_pin(doc, top, None, &o);
    let ev = run(&doc, &o);
    let world = |id| {
        topo::test_support::pinned_volume(
            "mass properties",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(body_of(&ev, id).as_ref()),
                Tol::witness(),
            ),
            Tol::witness(),
        )
        .volume
    };
    let gathered = topo::test_support::finished(
        "gathered",
        product(&doc, &ev, Tol::witness()).expect("the world gathers"),
        Tol::witness(),
    );
    let volume =
        topo::test_support::pinned_volume("mass properties", &gathered, Tol::witness()).volume;
    assert_eq!(
        volume.to_bits(),
        world(base).to_bits(),
        "the product is the world's material alone"
    );
    crate::fixture::could_look(&doc, &ev).expect("the world and the top's own space are at rest");
    // The measure across the two spaces is refused at the edit door,
    // before any evaluation (D′1).
    let error = fixture::insert_refused(&doc, measure());
    let spaces = [
        VarId::WORLD,
        doc.node_space(top).expect("the top is in a space"),
    ];
    assert!(
        matches!(
            &error,
            editor_core::EditError::SpaceMismatch { a, b, .. }
                if (a.id(), b.id()) == (spaces[0], spaces[1])
                    || (b.id(), a.id()) == (spaces[0], spaces[1])
        ),
        "{error:?}"
    );
    assert!(error.to_string().contains("Recourse:"), "{error}");
}

// ---- Row 4: the file ----

/// **A file the registry schema wrote refuses typed, and an instance in
/// a space of its own saves and loads.** An old snapshot's `placements`
/// table and an old log entry's `maintenance` wrapper are each a field
/// this build does not read, so each file refuses `Unreadable` with the
/// regenerate recourse — it never loads with a different meaning. An
/// instance placed into a `new_space()` round-trips in that space.
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

    let own = set_pin(doc, base, None, &o);
    let text = save(&own, &[], Tol::witness()).expect("an instance in its own space saves");
    let loaded = load(&text, Tol::witness()).expect("and loads");
    assert!(loaded.doc.bit_eq(&own));
    assert_eq!(
        solve(&loaded.doc, &o, Tol::witness()).space(base),
        own.node_space(base)
    );
    assert_ne!(own.node_space(base), Some(VarId::WORLD));
}

// ---- Row 5: a parametric pin, in every lane ----

/// **A seat under a parametric world mate poses alike in every lane.**
/// The base is pinned at a pose turned a quarter of a radian about z and
/// lifted by `lift`; the top is seated on it, and a second top is
/// stacked on the seated one:
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
    let (control, cb) = fixture::insert_pinned(control, crate::fixture::mated_instance(p.base), &o);
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
    // The control's seat, `(1, 1)` on the base and square to it, on the
    // world's axes: the base turned a quarter radian, so the slides are
    // `(1, 1)` turned with it and the spin is the base's.
    let (s, c) = 0.25_f64.sin_cos();
    let (doc, mate) = seat_turned(
        doc,
        top,
        head(p.top_cap(top)),
        head(p.base_cap(base)),
        [c - s, s + c],
        0.25,
    );
    let (doc, lifted) = stacked(doc, p.top, p.top_body, head(p.top_upper_cap(top)));
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(mate.placement), None);
    assert_eq!(poses.space(top), Some(VarId::WORLD));

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
    for id in [base, top, lifted] {
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
    for id in [base, top, lifted] {
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

/// **A second instance of `part` stacked on `onto`**: its body's bottom
/// cap seated on `onto` square at the origin ([`seat_on`] at `[0, 0]`).
/// Returns the document and the instance.
pub(crate) fn stacked(
    doc: ProfileDoc,
    part: DocRef,
    part_body: RecipeNodeId,
    onto: SitedFace,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, copy) = insert(doc, fixture::mated_instance(part));
    let mover = head_at(copy, in_part(copy, part_body, CapEnd::Start));
    let (doc, _) = seat_on(doc, copy, mover, onto, [0.0, 0.0]);
    (doc, copy)
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
/// moves**: every world placement of a cut body or of a member of a cut
/// family ([`fixture::with_placements`]) — a part delivers only its
/// world, so a split moves a body with its placements.
pub(crate) fn cut(doc: &ProfileDoc, ids: &[RecipeNodeId]) -> BTreeSet<RecipeNodeId> {
    let out: BTreeSet<RecipeNodeId> = ids.iter().copied().collect();
    fixture::with_placements(doc, &out)
}

/// **Whether the instance `out` leaves behind sits on the world at the
/// identity**, its part resolved beside `p`'s.
fn left_at_identity(p: &Parts, out: &editor_core::SplitOutcome) -> bool {
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    solve(&out.remainder, &with_resolver(store), Tol::witness()).at_identity(out.instance)
}

/// **A cut of one placed pair moves as selected** (A4). The instance
/// left behind sits on the world at the identity, the base keeps its
/// world mate in the part, and the top keeps its seat.
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
        "and the base keeps its world mate in the part"
    );
    let top_bundle = fixture::bundle(&out.part, out.node_map[&top]);
    assert_eq!(
        top_bundle.len(),
        fixture::bundle(&doc, top).len(),
        "the top keeps its seat"
    );
    assert_eq!(offset_of(&out.part, out.node_map[&top]), None);
}

/// **Inline at the identity, and at any other pose** (A4, one rule):
/// the part's placements land verbatim in a new space, and a placement
/// of their copies takes the instance's bundle and target. At the
/// identity the round trip of a cut returns the pair with the base's
/// pin; at another pose the new placement takes the instance's pin and
/// nothing moves in the world; an instance in a space of its own inlines
/// into that space.
#[test]
fn inline_lands_verbatim_at_the_identity_and_composes_the_values_at_any_other_pose() {
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
    let back_doc = fixture::round_trip::flat(&back);
    let volume = |d: &ProfileDoc| {
        topo::mass_properties(
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(&product(d, &run(d, &o), Tol::witness()).expect("gathers")),
                Tol::witness(),
            ),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .expect("mass properties")
        .met()
        .expect("the pinned resolution is met")
        .volume
    };
    assert_eq!(volume(&back_doc).to_bits(), volume(&doc).to_bits());

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
    assert!(
        fixture::same_offset(&moved.doc, through(&moved, base), &doc, base),
        "the base's pin lands verbatim in the new space"
    );
    let poses = solve(&moved.doc, &o, Tol::witness());
    assert_eq!(
        poses.space(through(&moved, top)),
        Some(moved.space),
        "and the top still seats on it there"
    );
    close(
        poses
            .placement(&moved.doc, moved.placement)
            .expect("placed")
            .translation,
        [0.0, 5.0, 0.0],
        "the new placement takes the instance's pin",
    );
    let corner = |d: &ProfileDoc| {
        let body =
            product(d, &run(d, &with_resolver(store.clone())), Tol::witness()).expect("gathers");
        points(&body).into_iter().fold([f64::INFINITY; 3], |m, p| {
            [m[0].min(p[0]), m[1].min(p[1]), m[2].min(p[2])]
        })
    };
    close(
        corner(&moved.doc),
        corner(&shifted),
        "nothing moves in the world",
    );

    // An instance moved into a space of its own inlines there: the new
    // placement takes the instance's target.
    let loose = set_pin(out.remainder.clone(), out.instance, None, &store_opts);
    let inlined = editor_core::inline(&loose, out.instance, &r, Tol::witness())
        .expect("an instance in a space of its own inlines");
    assert_eq!(
        inlined.doc.node_space(inlined.placement),
        loose.node_space(out.instance),
        "into the instance's own space"
    );
}

/// **A face a seat reads crosses the seam with its head**: a kept top
/// seated on a cut base, its target reading the base's top face, crosses
/// split — the head is read at the instance, under the base part's own
/// name, which every copy of it carries — and inline carries it back
/// onto the new placement's copy, which flatten moves onto the inner
/// instance. The seat solves on each document with no fault.
#[test]
fn a_face_target_crosses_split_and_inline_with_its_head() {
    let p = parts("p2-face-crossing");
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("p2-face-crossing"), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(p.base), &o);
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
    let target = crossed.to;
    let [side] = fixture::face_reads(&out.remainder, target)[..] else {
        panic!("the crossed target reads one face: {crossed:?}");
    };
    let face = p.base_cap(base).name;
    let head = fixture::side_head(&out.remainder, side);
    assert_eq!(
        (head.at, head.name.into_name()),
        (out.instance, face.clone()),
        "the head is read at the instance, under the base's own name"
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
    let [side] = fixture::face_reads(&back.doc, host.to)[..] else {
        panic!("the host target reads one face: {host:?}");
    };
    let head = fixture::side_head(&back.doc, side);
    assert_eq!(
        (head.at, head.name.into_name()),
        (back.placement, face.clone()),
        "the head is read at the new placement's copy"
    );
    let ev = run(&back.doc, &o);
    assert!(
        ev.node_error(top).is_none(),
        "the host constraint evaluates clean: {:?}",
        ev.node_error(top)
    );
    let flat = fixture::round_trip::flat(&back);
    let host = fixture::constraint(&flat, mate);
    let [side] = fixture::face_reads(&flat, host.to)[..] else {
        panic!("the flattened target reads one face: {host:?}");
    };
    let head = fixture::side_head(&flat, side);
    assert_eq!(
        (head.at, head.name.into_name()),
        (back.node_map[&out.node_map[&base]], face),
        "flattened, the head is read at the inner instance"
    );
    let ev = run(&flat, &o);
    assert!(
        ev.node_error(top).is_none(),
        "the host constraint evaluates clean: {:?}",
        ev.node_error(top)
    );
}

/// **A cut of two pinned instances moves verbatim** (A4): the instance
/// left behind sits on the world at the identity and each keeps its own
/// world mate in the part.
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
            "each keeps its own world mate"
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

/// One instance's world mate beside its image's.
type PinPair = (Option<Vec<(Coord, Formula)>>, Option<Vec<(Coord, Formula)>>);

/// Every instance's world mate in `doc` beside the one `onto` holds at
/// its image under `map`.
fn pins_through(
    doc: &ProfileDoc,
    map: impl Fn(RecipeNodeId) -> RecipeNodeId,
    onto: &ProfileDoc,
) -> Vec<PinPair> {
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
/// exactly the source's world mates and each placement's constraints, and
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
    let part = crate::fixture::assemble_world_copies(part);
    let part_ref = store.insert(part.clone(), Tol::witness());
    let host = ProfileDoc::empty(DocumentId::derive("p2-carry-inline-host"), Tol::witness());
    let (host, instance) = fixture::insert_pinned(
        host,
        crate::fixture::mated_instance(part_ref),
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
    // The instance stands where its body stands by a mate to the world
    // and constant values; its going retires those anonymous values and
    // nothing else, so nothing carried from the part is reported.
    for row in &back.maintenance {
        assert!(
            matches!(row, Maintenance::AnonymousVarRemoved { var, .. }
                if host.var(var.id()).is_some() && back.doc.var(var.id()).is_none()
                    && part.var(var.id()).is_none()),
            "only the instance's own pin retires: {row:?}"
        );
    }
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
