//! **One box of a disjoint union, blended by name.** The union of two
//! boxes a unit apart is two solids; a `Node::Fillet` or `Node::Chamfer`
//! over one box's twelve edges carves that box at its closed form, and
//! every name the other box carries resolves, after the blend, to the
//! entity it named before it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use editor_core::ExtrudeSide;
use editor_core::{
    EntityKey, Entry, EvalOptions, NameRef, NameTable, Node, NodeResult, ProfileDoc, RecipeNodeId,
    RoleSeg, StableName,
};

use crate::corpus;
use crate::fixture;
use fixture::{len, table, tol};
use topo::Body;

const R: f64 = 0.1;

/// A unit box on `[x0, x0 + 1] × [0, 1] × [0, 1]`.
fn unit_box(doc: ProfileDoc, x0: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(x0 + 0.5, 0.5, 0.5)],
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

/// The union of the box at `x ∈ [0, 1]` and the one at `x ∈ [2, 3]`.
fn two_boxes(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(label, tol());
    let (doc, a) = unit_box(doc, 0.0);
    let (doc, b) = unit_box(doc, 2.0);
    fixture::insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
            declare: Vec::new(),
        },
    )
}

/// Whether the entity `key` names lies wholly at `x < 1.5` (box `a`)
/// or wholly beyond it (box `b`), read off its vertices.
fn in_box(body: &Body<f64>, key: EntityKey, a_side: bool) -> bool {
    let x = |v| {
        body.get_vertex(v)
            .and_then(|x| body.get_point(x.point))
            .expect("a live vertex has a point")
            .x
    };
    let verts = match key {
        EntityKey::Vertex(v) => vec![v],
        EntityKey::Edge(e) => fixture::ends(body, e).to_vec(),
        EntityKey::Face(f) => fixture::face_vertices(body, f).into_iter().collect(),
        _ => return false,
    };
    !verts.is_empty() && verts.into_iter().all(|v| (x(v) < 1.5) == a_side)
}

/// Every uniquely named entity of one box, with its name.
fn named(t: &NameTable, body: &Body<f64>, a_side: bool) -> Vec<(StableName, EntityKey)> {
    t.iter()
        .filter_map(|(n, entry)| match entry {
            Entry::Unique(r) if in_box(body, r.key, a_side) => Some((n.clone(), r.key)),
            _ => None,
        })
        .collect()
}

fn blend_one_box(
    label: &str,
    node: fn(RecipeNodeId, Vec<StableName>) -> editor_core::AuthoredNode,
    want: f64,
) {
    let (doc, union) = two_boxes(label);
    let ev = fixture::run(&doc, &EvalOptions::default());
    let (t, body) = (table(&ev, union), corpus::body_of(&ev, union));
    assert_eq!(
        body.solids().count(),
        2,
        "{label}: a disjoint union is two solids"
    );
    let selection: Vec<StableName> = named(t, body, true)
        .into_iter()
        .filter(|(_, k)| matches!(k, EntityKey::Edge(_)))
        .map(|(n, _)| n)
        .collect();
    assert_eq!(
        selection.len(),
        12,
        "{label}: box a's twelve edges, by name"
    );
    let carried = named(t, body, false);
    assert_eq!(
        carried.len(),
        6 + 12 + 8,
        "{label}: box b's faces, edges and vertices are each named"
    );

    let (doc, blended) = fixture::insert(doc, node(union, selection));
    let ev = fixture::run(&doc, &EvalOptions::default());
    assert!(
        matches!(ev.nodes.get(&blended), Some(NodeResult::Ok(_))),
        "{label}: the blend evaluates, got {:?}",
        ev.nodes.get(&blended)
    );
    let (t, body) = (table(&ev, blended), corpus::body_of(&ev, blended));
    assert_eq!(body.solids().count(), 2, "{label}: still two solids");
    let v = topo::mass_properties(body, tol())
        .expect("closed-form mass properties")
        .volume;
    assert!(
        (v - want).abs() <= 1e-12 * want,
        "{label}: V = {v}, closed form {want}"
    );
    // The blend re-roots each name it carries under its own node, and
    // the entity it resolves to is the one it named before.
    let carried: Vec<(StableName, EntityKey)> = carried
        .into_iter()
        .map(|(n, k)| {
            (
                fixture::minted(
                    n.kind,
                    blended,
                    RoleSeg::From {
                        read: fixture::out(&doc, union),
                        of: NameRef::new(n),
                    },
                ),
                k,
            )
        })
        .collect();
    let mut after = named(t, body, false);
    after.sort_by(|x, y| x.0.cmp(&y.0));
    let mut want = carried;
    want.sort_by(|x, y| x.0.cmp(&y.0));
    assert_eq!(
        after, want,
        "{label}: every name of box b resolves to the entity it named before"
    );
}

#[test]
fn one_box_of_a_disjoint_union_fillets_by_name_and_the_other_keeps_its_names() {
    // The filleted unit box is the Steiner form of its shrunk core.
    let l = 1.0 - 2.0 * R;
    let rounded = l.powi(3) + 6.0 * l * l * R + 3.0 * PI * l * R * R + (4.0 / 3.0) * PI * R.powi(3);
    blend_one_box(
        "one_box_of_two_fillet",
        |target, selection| Node::Fillet {
            target: target.into(),
            radius: len(R),
            selection,
        },
        rounded + 1.0,
    );
}

#[test]
fn one_box_of_a_disjoint_union_chamfers_by_name_and_the_other_keeps_its_names() {
    // Twelve prisms of section d²/2, less the 2d³ inclusion–exclusion
    // over-counts at each of eight corners, plus eight 4d³/3 tetrahedra.
    let chamfered = 1.0 - 6.0 * R * R + (16.0 / 3.0) * R.powi(3);
    blend_one_box(
        "one_box_of_two_chamfer",
        |target, selection| Node::Chamfer {
            target: target.into(),
            distance: len(R),
            selection,
        },
        chamfered + 1.0,
    );
}
