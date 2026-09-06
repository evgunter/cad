//! **The per-part window, checked against a second implementation of
//! itself.**
//!
//! `PickIndex` answers eight questions off one idea: the parts are laid
//! out in a fixed order, each part owns a contiguous run of the flat
//! entity list, and the flat names are parallel to it. The rows below
//! rebuild that idea INDEPENDENTLY — from `index.parts()` and the
//! evaluation, through `NodePick`'s own public inversion, with the
//! windows walked by hand — and assert the index answers exactly what
//! the hand-walked windows do, for every entity of every part.
//!
//! **Why a second implementation and not more assertions.** Every
//! window door here is total: it answers a name for anything asked. So
//! a wrong window does not crash, it answers the NEXT body's name —
//! plausible, confident, wrong (#1098). Assertions written against the
//! index's own output cannot see that; an independent walk can, and it
//! keeps seeing it as the index is refactored underneath it.
//!
//! The fixture is chosen for the places a window breaks: a first part,
//! a last part, one node with THREE bodies, and one name drawn TWICE
//! (two `Transform` roots over one extrude carry the same names), plus
//! addresses one past the end of a body that IS drawn — the refusal
//! that must not read the next body.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::collections::BTreeMap;

use crate::common;

use pncad::document::{Dimension, Doc, Evaluation, Expr, Node, ProfileProgram, RecipeNodeId};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::{HitTestError, NodePick};
use viewer::pick::{EdgeId, EdgeNameFault, PickIndex};
use viewer::scene;
use viewer::session::{DocSession, EdgeSelection, FaceSelection};

/// The display δ these rows tessellate at: coarse (the fixture is all
/// planes) and cheap.
fn delta() -> scene::DisplayTolerance {
    scene::DisplayTolerance::new(1.0e-3).expect("a positive delta")
}

/// The document every row here indexes: three roots, six drawn bodies.
///
/// - a linear pattern of three blocks — ONE node, THREE bodies;
/// - two `Transform`s over one extrude — one set of names, drawn
///   TWICE, which is the case `ids_of` answers two ids for and the
///   (node, body) narrowing exists to separate;
/// - a bare extrude — the last part.
fn fixture(tol: Tol) -> Doc<ProfileProgram> {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("chrome-pick-windows", tol);
    let (doc, profile) = common::framed_square(&doc, 0.02, tol);
    let (doc, block) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.01),
        },
        tol,
    );
    let (doc, _pattern) = common::inserted(
        &doc,
        Node::Pattern {
            input: block,
            count: Expr::count(3),
            kind: pncad::document::PatternKind::Linear {
                direction: [common::scl(1.0), common::scl(0.0), common::scl(0.0)],
                spacing: common::len(0.05),
            },
        },
        tol,
    );
    let (doc, twinned) = common::framed_square(&doc, 0.015, tol);
    let (doc, twinned) = common::inserted(
        &doc,
        Node::Extrude {
            profile: twinned,
            distance: common::len(0.008),
        },
        tol,
    );
    let placed = |at: f64| Node::Transform {
        input: twinned,
        translation: [common::len(at), common::len(0.2), common::len(0.0)],
        rotation_axis: [common::scl(0.0), common::scl(0.0), common::scl(1.0)],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("a finite angle"),
    };
    let (doc, _first) = common::inserted(&doc, placed(0.0), tol);
    let (doc, _second) = common::inserted(&doc, placed(0.1), tol);
    let (doc, last) = common::framed_square(&doc, 0.01, tol);
    let (doc, _last) = common::inserted(
        &doc,
        Node::Extrude {
            profile: last,
            distance: common::len(0.004),
        },
        tol,
    );
    doc
}

/// The index and the evaluation it was built from.
fn indexed(tol: Tol) -> (DocSession, PickIndex) {
    let mut session = DocSession::inline(fixture(tol), tol);
    session.pump();
    let (doc, eval) = session.landed_pair().expect("the inline seam lands");
    let generation = session
        .landed_generation()
        .expect("a landed evaluation has a generation");
    let index = PickIndex::build(doc, eval, generation, delta(), tol).expect("the fixture indexes");
    (session, index)
}

/// The per-part windows, walked BY HAND from the parts and the
/// evaluation — the second implementation the rows compare against.
///
/// This is the hand-parallel layout `PickIndex` used to carry as seven
/// fields: a flat patch-name list, its name→ids inverse, a
/// (node, body)→window map, the flat edge list, its names, and its own
/// window map. It is kept here on purpose: it is derived from the
/// public doors alone (`NodePick::patch_names`, `NodePick::boundary_names`),
/// so it stays an INDEPENDENT statement of what the layout means no
/// matter how the index comes to hold it.
struct HandWalked {
    names: Vec<Result<StableName, HitTestError>>,
    by_name: BTreeMap<StableName, Vec<u32>>,
    by_target: BTreeMap<(RecipeNodeId, u32), (usize, usize)>,
    id_slice: Vec<u32>,
    edges: Vec<EdgeId>,
    edge_names: Vec<Result<StableName, HitTestError>>,
    edges_by_target: BTreeMap<(RecipeNodeId, u32), (usize, usize)>,
}

impl HandWalked {
    fn of(parts: &[NodePick], eval: &Evaluation<f64>) -> Self {
        let mut names: Vec<Result<StableName, HitTestError>> = Vec::new();
        for part in parts {
            names.extend(part.patch_names(eval));
        }
        let mut by_name: BTreeMap<StableName, Vec<u32>> = BTreeMap::new();
        for (index, name) in names.iter().enumerate() {
            if let Ok(name) = name {
                by_name
                    .entry(name.clone())
                    .or_default()
                    .push(index as u32 + 1);
            }
        }
        let id_slice: Vec<u32> = (1..=names.len()).map(|i| i as u32).collect();
        let mut by_target: BTreeMap<(RecipeNodeId, u32), (usize, usize)> = BTreeMap::new();
        let mut next = 0usize;
        for part in parts {
            let patches = part.mesh().patches.len();
            by_target.insert((part.node(), part.body()), (next, patches));
            next += patches;
        }
        let mut edges: Vec<EdgeId> = Vec::new();
        let mut edge_names: Vec<Result<StableName, HitTestError>> = Vec::new();
        let mut edges_by_target: BTreeMap<(RecipeNodeId, u32), (usize, usize)> = BTreeMap::new();
        for part in parts {
            let start = edges.len();
            for (boundary, name) in part.boundary_names(eval).into_iter().enumerate() {
                edges.push(EdgeId {
                    node: part.node(),
                    body: part.body(),
                    boundary,
                });
                edge_names.push(name);
            }
            edges_by_target.insert((part.node(), part.body()), (start, edges.len() - start));
        }
        Self {
            names,
            by_name,
            by_target,
            id_slice,
            edges,
            edge_names,
            edges_by_target,
        }
    }

    fn name_of(&self, id: u32) -> Option<&Result<StableName, HitTestError>> {
        self.names.get(usize::try_from(id.checked_sub(1)?).ok()?)
    }

    fn ids_of(&self, name: &StableName) -> &[u32] {
        self.by_name.get(name).map_or(&[], Vec::as_slice)
    }

    fn ids_in(&self, node: RecipeNodeId, body: u32) -> &[u32] {
        let Some(&(start, len)) = self.by_target.get(&(node, body)) else {
            return &[];
        };
        self.id_slice.get(start..start + len).unwrap_or_default()
    }

    fn ids_of_node(&self, node: RecipeNodeId) -> Vec<u32> {
        self.by_target
            .iter()
            .filter(|((drawn, _), _)| *drawn == node)
            .flat_map(|(_, &(start, len))| {
                self.id_slice.get(start..start + len).unwrap_or_default()
            })
            .copied()
            .collect()
    }

    fn ids_of_target(&self, face: &FaceSelection) -> Vec<u32> {
        let scope = self.ids_in(face.node, face.body);
        self.ids_of(&face.name)
            .iter()
            .copied()
            .filter(|id| scope.contains(id))
            .collect()
    }

    fn edges_in(&self, node: RecipeNodeId, body: u32) -> &[EdgeId] {
        let Some(&(start, len)) = self.edges_by_target.get(&(node, body)) else {
            return &[];
        };
        self.edges.get(start..start + len).unwrap_or_default()
    }

    fn edge_name_of(&self, id: EdgeId) -> Result<&StableName, EdgeNameFault> {
        let &(start, len) =
            self.edges_by_target
                .get(&(id.node, id.body))
                .ok_or(EdgeNameFault::NotDrawn {
                    node: id.node,
                    body: id.body,
                })?;
        if id.boundary >= len {
            return Err(EdgeNameFault::OutOfRange {
                node: id.node,
                body: id.body,
                boundary: id.boundary,
                drawn: len,
            });
        }
        match self.edge_names.get(start + id.boundary) {
            Some(Ok(name)) => Ok(name),
            Some(Err(error)) => Err(EdgeNameFault::Unnamed(*error)),
            None => Err(EdgeNameFault::OutOfRange {
                node: id.node,
                body: id.body,
                boundary: id.boundary,
                drawn: len,
            }),
        }
    }

    fn edges_of_target(&self, edge: &EdgeSelection) -> Vec<EdgeId> {
        self.edges_in(edge.node, edge.body)
            .iter()
            .copied()
            .filter(|id| matches!(self.edge_name_of(*id), Ok(name) if *name == edge.name))
            .collect()
    }
}

/// The fixture is the fixture the rows below claim to walk. A
/// differential over a degenerate document proves nothing, and a
/// document is easy to degenerate by accident.
#[test]
fn the_fixture_has_the_shape_the_differential_needs() {
    let tol = Tol::witness();
    let (session, index) = indexed(tol);
    let eval = session.evaluation().expect("an evaluation has landed");
    let hand = HandWalked::of(index.parts(), eval);

    let mut bodies_per_node: BTreeMap<RecipeNodeId, usize> = BTreeMap::new();
    for part in index.parts() {
        *bodies_per_node.entry(part.node()).or_default() += 1;
    }
    assert!(
        bodies_per_node.len() >= 3,
        "three drawn roots: {bodies_per_node:?}"
    );
    assert!(
        bodies_per_node.values().any(|&n| n >= 3),
        "one node with several bodies: {bodies_per_node:?}"
    );
    assert!(index.parts().len() >= 6, "six drawn parts");
    assert!(
        hand.by_name.values().any(|ids| ids.len() > 1),
        "one name drawn twice — the narrowing has something to narrow"
    );
    assert!(
        hand.by_target.values().all(|&(_, len)| len > 0),
        "every window is non-empty; a ZERO-length window is not \
         constructible through this door and is checked at the \
         structure instead (pick.rs's own unit rows)"
    );
}

/// Every window door, over every entity of every part, against the
/// hand-walked layout — plus the addresses that are NOT entities: an
/// id past the end, `IdMap::NOTHING`, a boundary one past a drawn
/// body, and a (node, body) this index does not draw at all.
#[test]
fn every_window_door_answers_what_the_hand_walk_does() {
    let tol = Tol::witness();
    let (session, index) = indexed(tol);
    let eval = session.evaluation().expect("an evaluation has landed");
    let hand = HandWalked::of(index.parts(), eval);

    // Every id the index assigned, plus 0 (nothing) and two past the
    // end — the flat address has no window to leave, and both doors
    // must agree about where it stops.
    for id in 0..=(hand.names.len() as u32 + 2) {
        assert_eq!(index.name_of(id), hand.name_of(id), "name_of({id})");
    }

    // Every drawn (node, body), and one that is not drawn.
    let mut targets: Vec<(RecipeNodeId, u32)> = hand.by_target.keys().copied().collect();
    let absent = RecipeNodeId(u64::MAX);
    targets.push((absent, 0));
    targets.push((index.parts()[0].node(), 99));
    for (node, body) in targets {
        assert_eq!(index.ids_in(node, body), hand.ids_in(node, body));
        assert_eq!(index.edges_in(node, body), hand.edges_in(node, body));
        assert_eq!(index.ids_of_node(node), hand.ids_of_node(node));

        // Every boundary of the body, and two past its end: the
        // OutOfRange refusal that must not read the next body's name.
        let drawn = hand.edges_in(node, body).len();
        for boundary in 0..drawn + 2 {
            let id = EdgeId {
                node,
                body,
                boundary,
            };
            assert_eq!(
                index.edge_name_of(id).ok(),
                hand.edge_name_of(id).ok(),
                "edge_name_of({id:?})"
            );
            assert_eq!(
                index.edge_name_of(id).err(),
                hand.edge_name_of(id).err(),
                "edge_name_of({id:?}) refusal"
            );
        }

        // The narrowing, for every name drawn ANYWHERE — so each
        // target is asked about its own names and about every other
        // part's, which is what a window that reaches too far answers
        // wrongly.
        for name in hand.by_name.keys() {
            let face = FaceSelection {
                name: name.clone(),
                node,
                body,
            };
            assert_eq!(
                index.ids_of_target(&face),
                hand.ids_of_target(&face),
                "ids_of_target({name:?}, {node:?}, {body})"
            );
            let edge = EdgeSelection {
                name: name.clone(),
                node,
                body,
            };
            assert_eq!(
                index.edges_of_target(&edge),
                hand.edges_of_target(&edge),
                "edges_of_target({name:?}, {node:?}, {body})"
            );
        }
    }

    // The cross-body inverse, for every name and one name that is not
    // drawn at all.
    for name in hand.by_name.keys() {
        assert_eq!(index.ids_of(name), hand.ids_of(name), "ids_of({name:?})");
    }

    // Every drawn EDGE, addressed the way `edges_in` hands it out.
    for &id in &hand.edges {
        assert_eq!(index.edge_name_of(id).ok(), hand.edge_name_of(id).ok());
    }
}

/// The first part and the last part specifically: an off-by-one in the
/// window walk shows at the ends first, and a row that only checks
/// "some part" can miss both.
#[test]
fn the_first_and_last_parts_answer_their_own_entities() {
    let tol = Tol::witness();
    let (session, index) = indexed(tol);
    let eval = session.evaluation().expect("an evaluation has landed");
    let hand = HandWalked::of(index.parts(), eval);
    let parts = index.parts();
    let ends = [
        parts.first().expect("a first part"),
        parts.last().expect("a last part"),
    ];
    for part in ends {
        let (node, body) = (part.node(), part.body());
        let ids = index.ids_in(node, body);
        assert_eq!(ids.len(), part.mesh().patches.len(), "the whole run");
        assert_eq!(ids, hand.ids_in(node, body));
        for (patch, id) in ids.iter().enumerate() {
            assert_eq!(
                index
                    .ids()
                    .key_of(*id)
                    .map(|key| (key.node, key.body, key.patch)),
                Some((node, body, patch)),
                "id {id} is this part's patch {patch}"
            );
        }
        let edges = index.edges_in(node, body);
        assert_eq!(edges.len(), part.mesh().boundaries.len(), "the whole run");
        for (boundary, id) in edges.iter().enumerate() {
            assert_eq!(id.node, node);
            assert_eq!(id.body, body);
            assert_eq!(id.boundary, boundary);
        }
        // One past the end refuses rather than reading the next part.
        let past = EdgeId {
            node,
            body,
            boundary: edges.len(),
        };
        assert_eq!(
            index.edge_name_of(past),
            Err(EdgeNameFault::OutOfRange {
                node,
                body,
                boundary: edges.len(),
                drawn: edges.len(),
            })
        );
    }
}
