//! **A family of boolean results is a family of bodies** (D10
//! `[repetition]`, D3): a per-`k` subtract's members are its results,
//! and a seat that reads the family whole, or one member by index,
//! reads them as it reads any family of bodies. A member that is the
//! empty result is the empty value where it is read, and no seat
//! absorbs it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on};
use crate::fixture::{self, insert};
use editor_core::{
    Bodies, BodyRead, BooleanValue, EntityKey, Entry, Evaluation, Formula, InterrogateError,
    MemberAt, Node, NodeErrorKind, NodeResult, Operand, PinRefusal, ProfileDoc, RecipeNodeId,
    RoleSeg, StableName, TargetAxis, ValuePayload,
};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &Default::default())
}

/// The body `id` holds, or a panic naming what it holds instead.
fn body(ev: &Evaluation<f64>, id: RecipeNodeId) -> &topo::Body<f64> {
    match ev.value(id).map(|v| &v.payload) {
        Some(ValuePayload::Body(b) | ValuePayload::Boolean(BooleanValue::Body { body: b, .. })) => {
            b.body()
        }
        _ => panic!("{id:?} is no body: {:?}", ev.result(id)),
    }
}

/// `body`'s extent along x.
fn span(body: &topo::Body<f64>) -> (f64, f64) {
    let xs: Vec<f64> = body.vertex_points().map(|(_, p)| p.x).collect();
    (
        xs.iter().copied().fold(f64::INFINITY, f64::min),
        xs.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    )
}

/// Whether `id` refuses `EmptyOperand` naming `input`.
fn refuses_empty(ev: &Evaluation<f64>, id: RecipeNodeId, input: RecipeNodeId) -> bool {
    matches!(
        ev.result(id),
        Some(NodeResult::Failed(e)) if matches!(e.kind, NodeErrorKind::EmptyOperand { input: at } if at == input)
    )
}

/// Whether `id` refuses `EmptyMember` naming `input` and the member at
/// `at`.
fn refuses_empty_member(
    ev: &Evaluation<f64>,
    id: RecipeNodeId,
    input: RecipeNodeId,
    at: &[(editor_core::VarId, i64)],
) -> bool {
    matches!(
        ev.result(id),
        Some(NodeResult::Failed(e)) if matches!(
            &e.kind,
            NodeErrorKind::EmptyMember { input: of, at: member } if *of == input && member == at
        )
    )
}

/// The member a family's name names, if it is a member's.
fn member_of_name(name: &StableName) -> Option<MemberAt> {
    match name.path.as_slice() {
        [RoleSeg::Member { at, .. }] => Some(at.clone()),
        _ => None,
    }
}

/// **A row of three unit blocks, pitch 0.75, each cut by the read
/// `tool` makes** (given the block and the row): the document, the row
/// and the per-`k` subtract.
fn cut_row(
    tool: impl FnOnce(ProfileDoc, RecipeNodeId, RecipeNodeId) -> (ProfileDoc, BodyRead<Formula>),
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("g-family-of-booleans", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = fixture::row(doc, b, ("k", 3), TargetAxis::X, 0.75);
    let (doc, tool) = tool(doc, b, row);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: BodyRead::plain(Operand::Node(row)),
            tool,
        },
    );
    (doc, row, cut)
}

/// Every seat that reads a family whole, over `family`: a union of it
/// as its argument, a union spelling it as one read, an intersect and
/// an assemble.
fn whole_seats(doc: ProfileDoc, family: RecipeNodeId) -> (ProfileDoc, [RecipeNodeId; 4]) {
    let whole = || Bodies::Family(BodyRead::plain(Operand::Node(family)));
    let (doc, union) = insert(doc, Node::Union { members: whole() });
    let (doc, spelled) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![BodyRead::plain(Operand::Node(family))]),
        },
    );
    let (doc, intersect) = insert(doc, Node::Intersect { members: whole() });
    let (doc, assemble) = insert(doc, Node::Assemble { members: whole() });
    (doc, [union, spelled, intersect, assemble])
}

/// **Every member a body**: a hole through member 0 alone leaves three
/// bodies, and the family reads as any family of bodies does — whole, a
/// union fusing the row into one solid 2.5 long and an assemble taking
/// it; by index, member 0 at a placement and member 1 at a subtract's
/// tool. Red if a family of boolean results refuses a seat a family of
/// copies takes.
#[test]
fn a_family_of_boolean_results_reads_whole_and_by_index() {
    let (doc, _, cut) = cut_row(|doc, b, _| {
        let (doc, hole) = block_on(doc, b, (0.25, 0.5), (0.25, 0.75), -1.0, 3.0);
        (doc, BodyRead::plain(Operand::Node(hole)))
    });
    let (doc, [union, spelled, _, assemble]) = whole_seats(doc, cut);
    let (doc, holed) = fixture::member(doc, cut, 0);
    let (doc, less) = insert(
        doc,
        Node::Subtract {
            from: BodyRead::plain(Operand::Node(union)),
            tool: BodyRead::indexed(cut, vec![Formula::count(1)]),
        },
    );
    let ev = run(&doc);
    assert!(
        matches!(ev.value(cut).map(|v| &v.payload), Some(ValuePayload::Instances(b)) if b.len() == 3),
        "the cut is a family of three bodies: {:?}",
        ev.result(cut)
    );
    for (what, id) in [("union", union), ("spelled union", spelled)] {
        let fused = body(&ev, id);
        assert_eq!(fused.solids().count(), 1, "the {what} fuses the row");
        assert_eq!(span(fused), (0.0, 2.5), "the {what} spans the row");
    }
    assert!(
        ev.value(assemble).is_some(),
        "the assemble takes the family: {:?}",
        ev.result(assemble)
    );
    assert_eq!(
        body(&ev, holed).faces().count(),
        10,
        "member 0 placed is the block with its hole"
    );
    assert_eq!(
        body(&ev, less).solids().count(),
        2,
        "the row less member 1 is its two ends"
    );
}

/// **A member that is the empty result** (`F[k] − F[1]`: member 1 is
/// itself less itself): every seat reading the family whole refuses
/// `EmptyMember` naming the family and member 1, as a seat reading one
/// empty result refuses; a body member read by index is that member,
/// under the family's names for it, at a placement and at a subtract;
/// the empty member read by index is the empty value, which a
/// placement keeps and a subtract refuses at either seat. The family
/// names its two bodies, each at its member's place. Red if the empty
/// member makes the family refuse as no family of bodies
/// (`WrongOperand`), reaches a sibling read by index, or shifts a later
/// member's place.
#[test]
fn a_family_with_an_empty_member_refuses_whole_and_reads_its_bodies_by_index() {
    let (doc, _, cut) =
        cut_row(|doc, _, row| (doc, BodyRead::indexed(row, vec![Formula::count(1)])));
    let (doc, whole) = whole_seats(doc, cut);
    let (doc, first) = fixture::member(doc, cut, 0);
    let (doc, empty) = fixture::member(doc, cut, 1);
    let indexed = |i| BodyRead::indexed(cut, vec![Formula::count(i)]);
    let (doc, apart) = insert(
        doc,
        Node::Subtract {
            from: indexed(0),
            tool: indexed(2),
        },
    );
    let (doc, less_empty) = insert(
        doc,
        Node::Subtract {
            from: indexed(2),
            tool: indexed(1),
        },
    );
    let (doc, empty_less) = insert(
        doc,
        Node::Subtract {
            from: indexed(1),
            tool: indexed(0),
        },
    );
    let ev = run(&doc);
    assert!(
        matches!(
            ev.value(cut).map(|v| &v.payload),
            Some(ValuePayload::Instances(b)) if matches!(b[..], [Some(_), None, Some(_)])
        ),
        "the cut is a family of bodies with member 1 empty: {:?}",
        ev.result(cut)
    );
    let k = doc.var_named("k").unwrap();
    for id in whole {
        assert!(
            refuses_empty_member(&ev, id, cut, &[(k, 1)]),
            "{:?}: a whole read of the family meets its empty member: {:?}",
            doc.node(id),
            ev.result(id)
        );
    }
    // Each name with the place of the body it names: member 2's body
    // stands at place 2, past the empty member's, not at 1.
    let mut named: Vec<(MemberAt, u32)> = fixture::table(&ev, cut)
        .iter()
        .filter_map(|(name, entry)| {
            let place = match entry {
                Entry::Unique(e) => e.body,
                Entry::Tied(es) => es[0].body,
            };
            Some((member_of_name(name)?, place))
        })
        .collect();
    named.sort_unstable();
    named.dedup();
    assert_eq!(
        named,
        vec![(vec![(k, 0)], 0), (vec![(k, 2)], 2)],
        "the family names its two bodies, each at its member's place, and nothing of its \
         empty member"
    );
    assert_eq!(span(body(&ev, first)), (0.0, 0.75), "member 0 placed");
    let held: Vec<MemberAt> = fixture::table(&ev, first)
        .iter()
        .filter_map(|(name, _)| member_of_name(name))
        .collect();
    assert!(
        !held.is_empty() && held.iter().all(|at| *at == vec![(k, 0)]),
        "member 0 placed keeps the family's names for it: {held:?}"
    );
    assert_eq!(
        span(body(&ev, apart)),
        (0.0, 0.75),
        "member 0 less member 2, which it does not meet"
    );
    assert!(
        matches!(
            ev.value(empty).map(|v| &v.payload),
            Some(ValuePayload::Boolean(BooleanValue::Empty))
        ),
        "member 1 placed is the empty value: {:?}",
        ev.result(empty)
    );
    for (seat, id) in [("tool", less_empty), ("from", empty_less)] {
        assert!(
            refuses_empty(&ev, id, cut),
            "a subtract reading the empty member at its {seat} refuses it: {:?}",
            ev.result(id)
        );
    }
}

/// **A family with an empty member is a family of bodies to every
/// reader past the evaluator** (`F[k] − F[1]`): its kind is a family
/// of bodies' word; an interrogator reads member 2's body at its place
/// and its face by the family's name for it, and finds no body at the
/// empty member's place; and the product of its world copies is its
/// two bodies. Red if the empty member makes the family read as no
/// body, shifts member 2 onto place 1, or makes the product refuse.
#[test]
fn a_family_with_an_empty_member_is_its_bodies_at_the_product_and_the_interrogators() {
    let (doc, _, cut) =
        cut_row(|doc, _, row| (doc, BodyRead::indexed(row, vec![Formula::count(1)])));
    let (doc, _) = fixture::place(doc, cut);
    let ev = run(&doc);
    let value = ev.value(cut).expect("the cut evaluated");
    assert_eq!(
        value.payload.kind_name(),
        ValuePayload::<f64>::Instances(vec![]).kind_name(),
        "a family with an empty member is spoken as a family of bodies"
    );
    let var = doc.output(cut, 0).expect("the cut defines its output");
    let read = |i| ev.read_body(var, i).expect("the cut stands");
    assert_eq!(
        read(0).map(|b| span(&fixture::in_space(b))),
        Ok((0.0, 0.75)),
        "member 0 at place 0"
    );
    assert!(
        matches!(read(1), Err(InterrogateError::NoSuchBody { index: 1 })),
        "no body at the empty member's place: {:?}",
        read(1).err()
    );
    assert_eq!(
        read(2).map(|b| span(&fixture::in_space(b))),
        Ok((1.75, 2.5)),
        "member 2 at place 2"
    );
    let face = fixture::table(&ev, cut)
        .iter()
        .find_map(|(name, entry)| match entry {
            Entry::Unique(e) if e.body == 2 && matches!(e.key, EntityKey::Face(_)) => {
                Some(name.clone())
            }
            _ => None,
        })
        .expect("the family names member 2's faces");
    assert_eq!(
        editor_core::face_carrier_kind(&ev, cut, &face),
        Ok(geom::SurfaceKind::Plane),
        "member 2's face, read by the family's name for it"
    );
    let product = editor_core::product(&doc, &ev, Tol::witness());
    assert_eq!(
        product.as_ref().ok().map(|p| p.solids().count()),
        Some(2),
        "the product is the family's two bodies: {:?}",
        product.err()
    );
}

/// **A frame read off a family whose first member is empty** (`F[k] −
/// F[0]`) is read off its first member that is a body, member 1 — the
/// block `[0.75, 1.75]` less `[0, 1]` — at a corner that member has.
/// Red if the corner walk reads member 0 alone and refuses the family
/// `NotABody`.
#[test]
fn a_corner_frame_over_a_family_reads_its_first_body_member() {
    let (doc, _, cut) =
        cut_row(|doc, _, row| (doc, BodyRead::indexed(row, vec![Formula::count(0)])));
    let ev = run(&doc);
    let frame: Result<Operand, PinRefusal> = editor_core::corner_frame_in(
        &doc,
        &ev,
        &Operand::Node(cut),
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 1.0],
        [1.75, 0.0, 0.0],
    );
    assert!(
        matches!(frame, Ok(Operand::Pose(_))),
        "the frame is read off member 1's corner: {frame:?}"
    );
}
