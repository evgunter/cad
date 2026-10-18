//! **A mate holds no number; an offset is a value or a standoff**
//! (FORK-S3M): an in-plane offset or a roll is a value — one freedom
//! the mate leaves, set to a number and charted on the two bodies' own
//! coordinates — and a standoff is the target plane slid along its own
//! normal.
//!
//! The scenes are `p2_gauges`'s two literal blocks: a 3 x 3 x 1 base
//! grounded in the world at the identity and a 1 x 1 x 3 top the seat
//! places, each with a corner at its own origin. The seat rests the
//! top's lower cap on the base's upper cap and sets the plane mate's
//! three freedoms by values: a slide along the base's x and y, which
//! measure the top's own origin from the base's, and a spin of zero.
//! Every value below is a dyadic length, so the corners compared are
//! exact.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::p2_gauges::{Parts, block, body_of, cut, min_corner, parts};

use editor_core::pose::PoseRole;
use editor_core::{
    CapEnd, ContactClass, Coord, Dimension, DocEdit, DocumentId, EditError, EvalOptions, Formula,
    FreeValue, FreeVar, Mate, MateRef, Node, ProfileDoc, ProfileProgram, RecipeNodeId, SlideAxis,
    SlotId, SplitError, Target, Value, VarName, WorldRead, apply,
};
use fixture::resolver::with_resolver;
use fixture::round_trip::{composed, same_up_to_ids};
use fixture::{
    ang, constrain_at, flip, gate, head, insert, insert_pinned, len, plane, rest, run, solve, step,
    step_with,
};
use geom_core::Tol;

// ---- substrate ----

fn slide() -> VarName {
    VarName::from_static("slide")
}

/// **`edit` applied through the refusing reach**, panicking on a
/// refusal: the document and the constraint ids it minted.
fn applied(
    doc: &ProfileDoc,
    edit: &DocEdit<ProfileProgram>,
) -> (ProfileDoc, Vec<editor_core::ConstraintId>) {
    let out = apply(doc, edit, Tol::witness(), &editor_core::RefusingReach)
        .unwrap_or_else(|e| panic!("the edit is admitted: {e}"));
    (out.doc, out.record.minted_constraints)
}

/// **A value setting `coord` of `mate` to `by`**, as an edit.
fn add_value(mate: MateRef, coord: Coord, by: Formula) -> DocEdit<ProfileProgram> {
    DocEdit::AddValue {
        placement: mate.placement,
        value: Box::new(Value {
            mate: mate.id,
            coord,
            by,
        }),
        fresh: Vec::new(),
    }
}

/// [`add_value`] applied: the value's own address.
fn valued(doc: ProfileDoc, mate: MateRef, coord: Coord, by: Formula) -> (ProfileDoc, MateRef) {
    let (doc, ids) = applied(&doc, &add_value(mate, coord, by));
    (doc, MateRef::new(mate.placement, ids[0]))
}

/// **`mate`'s three plane freedoms set**: a slide of `x` along the
/// target's x, one of `y` along its y, and a spin of zero. Answers the
/// x slide's address.
fn plane_values(doc: ProfileDoc, mate: MateRef, x: Formula, y: Formula) -> (ProfileDoc, MateRef) {
    let (doc, vx) = valued(doc, mate, Coord::Slide(SlideAxis::X), x);
    let (doc, _) = valued(doc, mate, Coord::Slide(SlideAxis::Y), y);
    let (doc, _) = valued(doc, mate, Coord::Spin, ang(0.0));
    (doc, vx)
}

/// **An instance of `part` grounded in the world at the identity**:
/// its lower cap resting on the world's xy plane, its three plane
/// freedoms set to zero — so its own coordinates are the world's, read
/// off its lower cap alone, whatever its height.
fn grounded(
    doc: ProfileDoc,
    part: editor_core::DocRef,
    body: RecipeNodeId,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, id) = insert(doc, fixture::mated_instance(part));
    let (doc, ground) = constrain_at(
        doc,
        id,
        Mate {
            on: plane(head(fixture::resolver::in_part(id, body, CapEnd::Start))),
            to: Target::World(WorldRead::through(PoseRole::Xy).flip()),
            class: ContactClass::Rest,
        },
    );
    (plane_values(doc, ground, len(0.0), len(0.0)).0, id)
}

/// The two-block scene: the base grounded, the top, and the top's lower
/// cap resting on the base's upper cap at `(x, y)` — the rest and its
/// values in the top's bundle. `prelude` runs on the empty document
/// first (a parameter's declaration). Answers the rest and the x
/// slide's address.
fn seated(
    label: &str,
    x: Formula,
    y: Formula,
    prelude: impl FnOnce(ProfileDoc) -> ProfileDoc,
) -> (Parts, ProfileDoc, [RecipeNodeId; 2], MateRef, MateRef) {
    let p = parts(label);
    let doc = prelude(ProfileDoc::empty(DocumentId::derive(label), Tol::witness()));
    let (doc, base) = grounded(doc, p.base, p.base_body);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, m) = constrain_at(
        doc,
        top,
        rest(
            head(p.top_cap(top)),
            head(p.base_cap(base)),
            ContactClass::Rest,
        ),
    );
    let (doc, vx) = plane_values(doc, m, x, y);
    (p, doc, [base, top], m, vx)
}

/// The seat places the top: the solve refuses nothing for it.
fn assert_placed(doc: &ProfileDoc, o: &EvalOptions, top: RecipeNodeId) {
    let poses = solve(doc, o, Tol::witness());
    assert_eq!(poses.fault(top), None, "the seat places the top");
    assert!(
        poses.placement(doc, top).is_ok(),
        "the top has a world pose"
    );
}

fn declare_slide(doc: ProfileDoc, value: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::DeclareVar {
            name: slide(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, value)),
        },
    )
    .0
}

/// Where the top's own origin landed.
fn top_corner(doc: &ProfileDoc, o: &EvalOptions, top: RecipeNodeId) -> [f64; 3] {
    min_corner(&body_of(&run(doc, o), top))
}

// ---- Row 1: an in-plane value follows the face ----

/// **A rest with an in-plane value follows the face through a part
/// edit.** Slid to `x = 1.5` along the base's x, the top sits at
/// `[1.5, 1, 1]`; the base grown to height 2 lifts it to `[1.5, 1, 2]`
/// — the values kept, the face followed — and the gate certifies the
/// seat both times.
///
/// *Breaks if* a value is measured off anything but the two bodies' own
/// origins, or the rest stops reading the base's face as it now stands.
#[test]
fn a_rest_with_an_in_plane_value_follows_the_face_through_a_part_edit() {
    let label = "place-mfo-follow";
    let (mut p, doc, [base, top], _, _) = seated(label, len(1.5), len(1.0), std::convert::identity);
    let o = p.opts();
    assert_placed(&doc, &o, top);
    assert_eq!(
        top_corner(&doc, &o, top),
        [1.5, 1.0, 1.0],
        "slid along the face"
    );
    let ev = run(&doc, &o);
    assert!(gate(&doc, &ev).is_ok(), "{:?}", gate(&doc, &ev).err());

    // The base part grows on disk; its names hold, its pin moves.
    let (base_doc, base_body) = block(&format!("{label}-base"), 3.0, 1.0);
    assert_eq!(base_body, p.base_body, "the same base document");
    // As stored: its body placed in its world.
    let base_doc = crate::fixture::place(base_doc, base_body).0;
    let (grown, _) = step(
        base_doc,
        DocEdit::SetParam {
            node: base_body,
            slot: SlotId::Distance,
            value: len(2.0).into(),
            fresh: Vec::new(),
        },
    );
    let new_ref = p.store.insert(grown, Tol::witness());
    let o = with_resolver(p.store.clone());
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (doc, _) = step_with(
        doc,
        DocEdit::UpdateReference {
            node: base,
            new_pin: new_ref.pin,
        },
        &reach,
    );
    assert_eq!(
        top_corner(&doc, &o, top),
        [1.5, 1.0, 2.0],
        "the top follows the grown face, still slid"
    );
    let ev = run(&doc, &o);
    assert!(gate(&doc, &ev).is_ok(), "{:?}", gate(&doc, &ev).err());
}

// ---- Row 2: a parameter drives a value ----

/// **A parameter drives a value, and the solved pose moves.** The x
/// slide reads the document's `slide`: at 1.5 the top sits at
/// x = 1.5, and one value edit to 2.0 moves it to 2.0 — the evaluation
/// fed the first as its memo re-keys the top, whose bundle holds the
/// value, and does not serve it from the memo. The parameter is read
/// by the value at both doors that check one: a value naming an
/// undeclared parameter refuses as it is added, and one reading an
/// angle parameter as a length refuses, each naming the value's own
/// slot.
///
/// *Breaks if* the value's number is not a slot of the placement, the
/// top is served from the memo, or a door lets a bad read through.
#[test]
fn a_parameter_drives_a_value_and_the_solved_pose_moves() {
    let label = "place-mfo-param";
    let (p, doc, [base, top], m, vx) = seated(
        label,
        Formula::named(slide(), Dimension::Length),
        len(1.0),
        |d| declare_slide(d, 1.5),
    );
    let o = p.opts();
    assert!(
        doc.node(m.placement)
            .expect("the placement")
            .slots()
            .contains(&SlotId::ValueSlide { constraint: vx.id }),
        "the value's number is the placement's slot"
    );
    let first = run(&doc, &o);
    assert_eq!(min_corner(&body_of(&first, top)), [1.5, 1.0, 1.0]);
    let (moved, _) = step(
        doc.clone(),
        DocEdit::SetVarValue {
            var: slide().into(),
            value: FreeValue::Continuous(2.0),
        },
    );
    let second = editor_core::evaluate::<f64>(
        &moved,
        Some(&first),
        &editor_core::CancelToken::new(),
        &o,
        Tol::witness(),
    );
    assert_eq!(
        min_corner(&body_of(&second, top)),
        [2.0, 1.0, 1.0],
        "the pose moved"
    );
    assert_ne!(
        first.value(top).map(|v| v.content_key),
        second.value(top).map(|v| v.content_key),
        "the top re-keys on the parameter its value reads"
    );
    assert_eq!(
        first.value(base).map(|v| v.content_key),
        second.value(base).map(|v| v.content_key),
        "the base reads nothing that moved"
    );

    // The two parameter doors, as the value is added.
    let (unvalued, _) = step(
        doc.clone(),
        DocEdit::RemoveConstraint {
            placement: top,
            constraint: vx.id,
        },
    );
    let (unvalued, _) = step(
        unvalued,
        DocEdit::DeclareVar {
            name: VarName::from_static("tilt"),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Angle, 0.0)),
        },
    );
    let refused = |name: &'static str| {
        apply(
            &unvalued,
            &add_value(
                m,
                Coord::Slide(SlideAxis::X),
                Formula::named(VarName::from_static(name), Dimension::Length),
            ),
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("the value refuses")
    };
    let is_the_slide_slot = |slot: &SlotId| matches!(slot, SlotId::ValueSlide { .. });
    let err = refused("nowhere");
    assert!(
        matches!(&err, EditError::SlotUnknownVarName { slot: s, .. } if is_the_slide_slot(s)),
        "{err:?}"
    );
    let err = refused("tilt");
    assert!(
        matches!(&err, EditError::SlotVarKind { slot: s, .. } if is_the_slide_slot(s)),
        "{err:?}"
    );
}

// ---- Row 3: a standoff ----

/// **A standoff places and declares no contact.** The base's upper cap
/// slid a quarter unit along its own normal is no face, so the rest
/// onto it places the top a quarter unit above the base, as asked, and
/// states no contact: the gather mints nothing for it, and the at-rest
/// gate has no declared rest to refute.
///
/// *Breaks if* the standoff is dropped (the top sits on the base), or a
/// mate whose target is a construction mints a contact.
#[test]
fn a_standoff_places_the_copy_and_declares_no_contact() {
    let label = "place-mfo-setback";
    let p = parts(label);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = grounded(doc, p.base, p.base_body);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let cap = plane(head(p.base_cap(base)));
    let standoff = editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Slide {
        pose: editor_core::pose::RoleRead::plain(cap.clone()),
        along: editor_core::pose::RoleRead::through(cap, PoseRole::Normal),
        by: len(0.25),
    }));
    let (doc, m) = constrain_at(
        doc,
        top,
        fixture::mate(
            plane(head(p.top_cap(top))),
            flip(standoff),
            ContactClass::Rest,
        ),
    );
    let (doc, _) = plane_values(doc, m, len(1.0), len(1.0));
    let o = p.opts();
    assert_placed(&doc, &o, top);
    assert_eq!(
        top_corner(&doc, &o, top),
        [1.0, 1.0, 1.25],
        "placed as asked"
    );
    let ev = run(&doc, &o);
    fixture::assert_mints_nothing(&doc, &ev, m);
    assert!(gate(&doc, &ev).is_ok(), "{:?}", gate(&doc, &ev).err());
}

// ---- Row 4: a value's parameter across the seam ----

/// **A value and its parameter cross split and inline.** A cut seat
/// whose value reads `slide` carries the parameter into the part,
/// inline merges it back, and `inline(split(d))` is `d` up to node ids
/// — the value included — with the top where it was. A kept seat
/// reading `slide` beside a cut one that does not keeps the parameter
/// in the remainder alone; both reading it is the uncut reference split
/// refuses.
///
/// *Breaks if* the parameter is left behind by the cut value or carried
/// off by the kept one, or the round trip moves the top.
#[test]
fn a_value_and_its_parameter_cross_split_and_inline() {
    let label = "place-mfo-seam";
    let p = parts(label);
    let o = p.opts();
    let doc = declare_slide(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        1.5,
    );
    let (doc, base) = insert_pinned(doc, Node::instantiate_part(p.base), &o);
    let seat_on = |doc: ProfileDoc, base: RecipeNodeId, x: Formula| {
        let (doc, top) = insert(doc, fixture::mated_instance(p.top));
        let (doc, m) = constrain_at(
            doc,
            top,
            rest(
                head(p.top_cap(top)),
                head(p.base_cap(base)),
                ContactClass::Rest,
            ),
        );
        let (doc, vx) = plane_values(doc, m, x, len(1.0));
        (doc, top, vx)
    };
    let (doc, top, vx) = seat_on(doc, base, Formula::named(slide(), Dimension::Length));
    let before = top_corner(&doc, &o, top);
    let out = editor_core::split(
        &doc,
        &cut(&doc, &[base, top]),
        DocumentId::derive(&format!("{label}-part")),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .unwrap_or_else(|e| panic!("split refused: {e}"));
    assert!(
        out.part.var_named(slide().as_str()).is_some(),
        "the cut value's parameter goes with it"
    );
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &crate::p2_gauges::resolver(store),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("inline refused: {e}"));
    let (map, steps) = composed(&doc, &out, &back);
    same_up_to_ids(&doc, &back.doc, &map, &steps)
        .unwrap_or_else(|e| panic!("inline(split(d)) is d up to node ids:\n{e}"));
    assert_eq!(
        top_corner(&back.doc, &o, top_image(&map, top)),
        before,
        "the top where it was"
    );

    // A kept seat reading `slide`: a second base pinned to the world
    // at `[0, 0, 8]` and a third block seated on it, slid by `slide`,
    // both left out of the cut.
    let (doc, kept_base) = insert(doc, fixture::mated_instance(p.base));
    let doc = fixture::pin(
        doc,
        kept_base,
        &editor_core::Frame::translation([0.0, 0.0, 8.0]),
        &o,
    );
    let (doc, _, _) = seat_on(doc, kept_base, Formula::named(slide(), Dimension::Length));
    let err = editor_core::split(
        &doc,
        &cut(&doc, &[base, top]),
        DocumentId::derive(&format!("{label}-part2")),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect_err("both seats read `slide`");
    assert!(
        matches!(&err, SplitError::UncutVarReference { var, .. } if var.name() == Some(&slide())),
        "{err:?}"
    );
    // The cut seat's value re-authored to read nothing: the parameter
    // stays.
    let (doc, _) = step(
        doc,
        DocEdit::RemoveConstraint {
            placement: top,
            constraint: vx.id,
        },
    );
    let rest_of_top = MateRef::new(
        top,
        fixture::bundle(&doc, top)
            .iter()
            .find(|c| c.mate().is_some())
            .expect("the top's rest")
            .id,
    );
    let (doc, _) = valued(doc, rest_of_top, Coord::Slide(SlideAxis::X), len(1.5));
    let out = editor_core::split(
        &doc,
        &cut(&doc, &[base, top]),
        DocumentId::derive(&format!("{label}-part3")),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .unwrap_or_else(|e| panic!("split refused: {e}"));
    assert!(
        out.part.var_named(slide().as_str()).is_none(),
        "nothing cut reads it"
    );
    assert!(
        out.remainder.var_named(slide().as_str()).is_some(),
        "the kept value's parameter stays"
    );
}

fn top_image(map: &editor_core::NodeMap, top: RecipeNodeId) -> RecipeNodeId {
    *map.get(&top).expect("the top is carried")
}

// ---- Row 5: a constraint compares by bits ----

/// **Two constraints differing only in a signed zero are two
/// constraints to D7.** A world read slid by `+0.0` and by `-0.0`, and
/// a value of `+0.0` and of `-0.0`: each pair is equal by value and
/// different by bits, and `bit_eq` says so.
///
/// *Breaks if* `bit_eq` compares a scalar by value.
#[test]
fn a_constraint_compares_by_bits() {
    let cap = |node, end| {
        head(editor_core::StableName {
            kind: editor_core::EntityKind::Face,
            node: RecipeNodeId::new(0, node),
            path: vec![editor_core::RoleSeg::Cap(end)],
        })
    };
    let world = |by: f64| Mate {
        on: plane(cap(1, CapEnd::Start)),
        to: Target::World(
            WorldRead::through(PoseRole::Xy).slide(WorldRead::through(PoseRole::Z), len(by)),
        ),
        class: ContactClass::Rest,
    };
    let (x, y) = (world(0.0), world(-0.0));
    assert_eq!(x, y, "world read: equal by value");
    assert!(!x.bit_eq(&y), "world read: different by bits");
    assert!(x.bit_eq(&x.clone()), "world read: equal to itself");

    let id = editor_core::ConstraintId(editor_core::MintId::new(0, 1));
    let value = |by: f64| {
        editor_core::Constraint::of_value(
            id,
            Value {
                mate: editor_core::ConstraintId(editor_core::MintId::new(0, 0)),
                coord: Coord::Slide(SlideAxis::X),
                by: len(by),
            },
        )
    };
    let (x, y) = (value(0.0), value(-0.0));
    assert_eq!(x, y, "value: equal by value");
    assert!(!x.bit_eq(&y), "value: different by bits");
    assert!(x.bit_eq(&x.clone()), "value: equal to itself");
}
