//! **A fillet and a chamfer in the document carry their operand's
//! contact records** (`sweep::blend::naming::BlendNaming::carry_contacts`):
//! the blended body holds the records of the cells the blend keeps, and
//! a reader of the blend reads them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{failure, run};
use crate::fixture::{self, insert, len, table};
use editor_core::{
    Bodies, EntityKey, Entry, Evaluation, Node, Operand, ProfileDoc, RecipeNodeId, StableName,
    ValuePayload,
};
use geom_core::{Point3, Tol};

/// The blend size, meters.
const R: f64 = 0.1;

/// The name of `node`'s edge between `a` and `b`.
fn edge_named(ev: &Evaluation<f64>, node: RecipeNodeId, a: [f64; 3], b: [f64; 3]) -> StableName {
    let body = match &ev
        .value(node)
        .unwrap_or_else(|| panic!("{:?}", failure(ev, node)))
        .payload
    {
        ValuePayload::Body(b) => fixture::in_space(b),
        ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) => {
            fixture::in_space(body)
        }
        other => panic!("expected a body, got {other:?}"),
    };
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let at = |p: Point3<f64>, q: [f64; 3]| (p - Point3::new(q[0], q[1], q[2])).norm() < 1e-12;
    table(ev, node)
        .iter()
        .find_map(|(n, entry)| match entry {
            Entry::Unique(r) => match r.key {
                EntityKey::Edge(e) => {
                    let he = body.get_edge(e).unwrap().he_plus;
                    let s = point(body.get_half_edge(he).unwrap().start);
                    let t = point(body.half_edge_end(he).unwrap());
                    ((at(s, a) && at(t, b)) || (at(s, b) && at(t, a))).then(|| n.clone())
                }
                _ => None,
            },
            Entry::Tied(_) => None,
        })
        .unwrap_or_else(|| panic!("an edge named between {a:?} and {b:?}"))
}

/// The records `id`'s value carries.
fn held(ev: &Evaluation<f64>, id: RecipeNodeId, what: &str) -> usize {
    ev.value(id)
        .unwrap_or_else(|| panic!("{what}: {:?}", failure(ev, id)))
        .carry
        .contacts
        .rows()
        .len()
}

/// `doc` with a fillet (or, where `chamfer`, a chamfer) of `of`'s edge
/// named `edge`, and a one-member union reading the blend.
fn blended(
    doc: ProfileDoc,
    of: RecipeNodeId,
    edge: StableName,
    chamfer: bool,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let selection = Operand::select(of, vec![edge]);
    let (doc, blend) = insert(
        doc,
        if chamfer {
            Node::Chamfer {
                distance: len(R),
                selection,
            }
        } else {
            Node::Fillet {
                radius: len(R),
                selection,
            }
        },
    );
    let (doc, reader) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![blend.into()]),
        },
    );
    (doc, blend, reader)
}

/// **A blend clear of a pinch carries the union's records**: the plate's
/// left bottom edge, far from the blocks' pinch on its top, is filleted
/// and chamfered, and the blend and a reader of it hold the union's two
/// records, each citing the union's own. Red where the blend's value
/// carries no records.
#[test]
fn a_blend_clear_of_a_pinch_carries_the_unions_records() {
    for chamfer in [false, true] {
        let what = if chamfer { "chamfer" } else { "fillet" };
        let (doc, union) = pinch_union("blend-carries-a-pinch");
        let edge = edge_named(&run(&doc), union, [0.0, 0.0, 0.0], [0.0, 2.0, 0.0]);
        let (doc, blend, reader) = blended(doc, union, edge, chamfer);
        let ev = run(&doc);
        assert_eq!(held(&ev, union, "the union"), 2, "the union's two records");
        assert_eq!(held(&ev, blend, what), 2, "{what}: the blend holds them");
        assert_eq!(
            held(&ev, reader, "the reader"),
            2,
            "{what}: a reader of the blend holds them"
        );
        let cited: Vec<_> = ev
            .value(blend)
            .unwrap()
            .carry
            .contacts
            .cites()
            .flat_map(|cites| cites.iter().collect::<Vec<_>>())
            .collect();
        assert_eq!(
            cited,
            [
                topo::Backing::Carried {
                    input: 0,
                    record: 0
                },
                topo::Backing::Carried {
                    input: 0,
                    record: 1
                }
            ],
            "{what}: each record cites the union's own"
        );
    }
}

/// [`crate::union_pinch_member_order::pinch`]'s union: two records,
/// the second block's top corner `(1.5, 1, 1.7)` on the first block's
/// edge along `x = 1.5, y = 1`, and that edge against the second
/// block's along it, `z ∈ [1, 1.7]`.
fn pinch_union(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, members) = crate::union_pinch_member_order::pinch(doc);
    insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(members.iter().map(|&m| m.into()).collect()),
        },
    )
}

/// The record cells `id`'s value carries.
fn records(ev: &Evaluation<f64>, id: RecipeNodeId) -> Vec<topo::Contact> {
    ev.value(id)
        .unwrap_or_else(|| panic!("{:?}", failure(ev, id)))
        .carry
        .contacts
        .rows()
        .iter()
        .map(|row| row.record)
        .collect()
}

/// **A recorded edge the rounded edge ends on keeps its record under
/// its key**: the first block's top edge along `y` ends at the top of
/// the recorded edge `x = 1.5, y = 1`, whose top the blend carves to
/// its foot at `z = 2 - R`. The edge keeps its key, and the touch it
/// records, `z ∈ [1, 1.7]`, lies on what is left, so both records ride
/// the blend on the union's own cells.
/// Red where the blend drops them, or moves them off the shortened edge.
#[test]
fn a_recorded_edge_the_blend_ends_on_keeps_its_record() {
    for chamfer in [false, true] {
        let what = if chamfer { "chamfer" } else { "fillet" };
        let (doc, union) = pinch_union("blend-ends-on-a-recorded-edge");
        let edge = edge_named(&run(&doc), union, [1.5, -1.0, 2.0], [1.5, 1.0, 2.0]);
        let (doc, blend, _) = blended(doc, union, edge, chamfer);
        let ev = run(&doc);
        let before = records(&ev, union);
        assert_eq!(before.len(), 2, "the union's two records");
        assert_eq!(
            records(&ev, blend),
            before,
            "{what}: the records keep the union's cells"
        );
    }
}

/// **A blend whose rounded edge is bounded by a recorded vertex is
/// refused before any carry**: the second block's top edges end at its
/// corner on the first block's edge, so the first block's face lies in
/// the strip the blend would carve, and the blend refuses that touch
/// itself (`FaceClearance`), never as a carry
/// (`work/bandrule/a-blend-refuses-a-touch-in-its-strip-so-no-real-touch-reaches-its-consumption.md`).
/// Red where such a blend builds and drops the record, or refuses it as
/// a `ContactCarry`.
#[test]
fn a_blend_bounded_by_a_recorded_vertex_refuses_the_touch() {
    for (along, far) in [("x", [2.0, 1.0, 1.7]), ("y", [1.5, 3.0, 1.7])] {
        for chamfer in [false, true] {
            let what = if chamfer { "chamfer" } else { "fillet" };
            let (doc, union) = pinch_union("blend-bounded-by-a-recorded-vertex");
            let edge = edge_named(&run(&doc), union, [1.5, 1.0, 1.7], far);
            let (doc, blend, _) = blended(doc, union, edge, chamfer);
            let ev = run(&doc);
            assert!(
                matches!(
                    failure(&ev, blend),
                    Some(editor_core::NodeErrorKind::Blend {
                        error: sweep::blend::BlendError::FaceClearance { .. },
                        ..
                    })
                ),
                "{what} along {along}: the blend refuses the touch in its strip: {:?}",
                failure(&ev, blend)
            );
        }
    }
}

/// A = [0, 2]² × [0, 1] with B = [0.5, 1.5]² × [1, 2] standing on it,
/// joined by `⊔`: the census keeps one face/face record, A's top
/// against B's bottom.
fn stacked() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("blend-a-census-face-record", Tol::witness());
    let (doc, a) = crate::blocks::block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, b) = crate::blocks::block_on(doc, a, (0.5, 1.5), (0.5, 1.5), 1.0, 1.0);
    insert(
        doc,
        Node::Assemble {
            members: Bodies::Spelled(vec![a.into(), b.into()]),
        },
    )
}

/// `stacked`'s blend of A's edge between `p` and `q`, evaluated.
fn stacked_blend(p: [f64; 3], q: [f64; 3], chamfer: bool) -> (Evaluation<f64>, RecipeNodeId) {
    let (doc, asm) = stacked();
    let edge = edge_named(&run(&doc), asm, p, q);
    let (doc, blend, _) = blended(doc, asm, edge, chamfer);
    (run(&doc), blend)
}

/// The vertical edge of A at the origin.
const VERTICAL: ([f64; 3], [f64; 3]) = ([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);

/// **A blend clear of a `⊔`'s face/face record carries it**: a chamfer
/// of A's vertical edge at the origin cuts A's top with a chord, and a
/// fillet of A's top front edge cuts it with a straight trimline. Each
/// keeps the census's record on A's top and B's bottom. Red where the
/// blend drops the record.
#[test]
fn a_blend_clear_of_a_census_face_record_carries_it() {
    for (what, (p, q), chamfer) in [
        ("chamfer of the vertical edge", VERTICAL, true),
        (
            "fillet of the top edge",
            ([0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
            false,
        ),
    ] {
        let (ev, blend) = stacked_blend(p, q, chamfer);
        let held = records(&ev, blend);
        assert!(
            matches!(held[..], [c] if matches!(c.cells(), [topo::Cell::Face(_), topo::Cell::Face(_)])),
            "{what}: the blend holds the census's one face/face record: {held:?}"
        );
    }
}

/// **A fillet that trims a census face record's face with an arc
/// refuses the carry, typed**: the fillet of A's vertical edge at the
/// origin leaves a quarter-circle arc in A's top face's trim. The
/// carry's confirm of A's top against B's bottom then has no region
/// reading for a planar face with a conic trim
/// (`work/charttail/a-planar-face-with-a-conic-trim-has-no-region-door.md`),
/// so the node refuses `ContactCarry { Unread }` on that pair rather
/// than building without the record. Red where the fillet builds
/// recordless, or refuses anything else.
#[test]
fn a_fillet_trimming_a_census_face_record_with_an_arc_refuses_unread() {
    let (chamfered, chamfer) = stacked_blend(VERTICAL.0, VERTICAL.1, true);
    let [record] = records(&chamfered, chamfer)[..] else {
        panic!("the chamfer carries the census's one record")
    };
    let (ev, blend) = stacked_blend(VERTICAL.0, VERTICAL.1, false);
    match failure(&ev, blend) {
        Some(editor_core::NodeErrorKind::ContactCarry {
            refusal: topo::CarryRefusal::Unread { record: 0, cells },
        }) => assert_eq!(
            *cells,
            record.cells(),
            "the refusal names the record's pair, A's top and B's bottom"
        ),
        other => panic!("the fillet refuses the carry Unread: {other:?}"),
    }
}
