//! **A parent's names of a child document's pieces, across the child's
//! reshaping and the parent's pin update.**
//!
//! An assembly holds names of a part's entities as `InPart { of }` at
//! the instantiate node: `of` is the name the part document mints, so
//! a swept wall's is spelled by the piece its profile step drew — the
//! step's minted id and its role (`names/README.md`, "N1, the profile
//! pieces"). A reshaping that keeps the step keeps the id, so the
//! parent's name needs nothing from the part's edit door, which never
//! sees it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Attr, CancelToken, ContentPin, DocEdit, DocRef, DocumentId, EvalOptions, Evaluation,
    LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId, ResolveFailure, ResolveFault,
    Rgba8, RoleSeg, StableName, apply, content_pin, evaluate,
};
use fixture::{insert, len, point, table, tol};
use geom_core::Tol;

/// A resolver over an in-memory shelf keyed by the full reference.
#[derive(Debug, Default)]
struct VersionShelf {
    docs: BTreeMap<(DocumentId, ContentPin), ProfileDoc>,
}

impl VersionShelf {
    fn shelve(&mut self, doc: ProfileDoc) -> DocRef {
        let pin = content_pin(&doc, Tol::witness()).expect("the pin computes");
        let id = doc.id();
        self.docs.insert((id, pin), doc);
        DocRef { id, pin }
    }
}

impl editor_core::PartResolver for VersionShelf {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        self.docs
            .get(&(doc_ref.id, doc_ref.pin))
            .cloned()
            .ok_or_else(|| ResolveFailure {
                fault: ResolveFault::Unresolved,
                message: "no such version on the shelf".to_string(),
            })
    }
}

fn run(doc: &editor_core::ProfileDoc, shelf: &Arc<VersionShelf>) -> Evaluation<f64> {
    let opts = EvalOptions {
        resolver: Some(shelf.clone() as Arc<dyn editor_core::PartResolver>),
        ..EvalOptions::default()
    };
    evaluate::<f64>(doc, None, &CancelToken::new(), &opts, Tol::witness())
}

/// The part: a 2 × 2 square extruded 1 tall; `(doc, profile, extrude)`.
fn part() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive("held-names-part"), Tol::witness());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![
                LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]).unwrap(),
            ],
            ids: Vec::new(),
        }),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    (doc, profile, ext)
}

/// The part's wall at canonical segment `k` of its one loop, as the
/// part names it: the piece its profile draws there.
fn part_wall(doc: &editor_core::ProfileDoc, ext: RecipeNodeId, k: u32) -> StableName {
    fixture::fname(ext, fixture::wall(doc, ext, k))
}

/// The same wall as the parent names it: `InPart { of }` at the
/// instantiate node.
fn held(instance: RecipeNodeId, of: &StableName) -> StableName {
    fixture::fname(
        instance,
        RoleSeg::InPart {
            of: of.clone().into(),
        },
    )
}

/// The corners of the face a parent-held name denotes, sorted.
fn corners(
    ev: &Evaluation<f64>,
    instance: RecipeNodeId,
    name: &StableName,
) -> Vec<(f64, f64, f64)> {
    let body = corpus::body_of(ev, instance);
    let face = fixture::face_of(table(ev, instance), "held wall", name);
    let mut out: Vec<(f64, f64, f64)> = fixture::face_vertices(body, face)
        .into_iter()
        .map(|v| {
            let p = point(body, v);
            (p.x, p.y, p.z)
        })
        .collect();
    out.sort_by(|a, b| a.partial_cmp(b).unwrap());
    out
}

/// **A part reshaped under a parent that holds one of its walls.** The
/// part inserts a leg before the step that draws its wall 1 (the leg
/// to `(2,2)`, drawn from `(2,0)`), keeping every step it had. The
/// parent paints that wall through its instance, then moves its pin
/// to the reshaped version.
///
/// The held name spells the step that draws the leg to `(2,2)`, by the
/// id it was minted with, and the reshaping kept the step, so after
/// the pin update the name denotes the leg that step draws now —
/// `(3,1)→(2,2)`, still ending at the corner its step targets — and
/// never the inserted leg `(2,0)→(3,1)`, which is a new step's piece.
/// Neither door has anything to report: nothing the name denotes was
/// removed.
#[test]
fn a_parents_held_name_follows_its_step_across_a_pin_update() {
    let (v1, profile, ext) = part();
    let wall = part_wall(&v1, ext, 1);
    let v1_painted = apply(
        &v1,
        &DocEdit::SetAppearance {
            name: wall.clone(),
            attr: Attr::Color(Rgba8::opaque(1, 2, 3)),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .unwrap()
    .doc;
    let kept: Vec<Option<editor_core::StepId>> = match v1_painted.node(profile) {
        Some(Node::Profile(p)) => p.ids[0].iter().copied().map(Some).collect(),
        other => panic!("the part's profile: {other:?}"),
    };
    // The new leg is step 2 of the six: `at`, `line_to(2,0)`, the new
    // `line_to(3,1)`, then the four old ones' remainder.
    let mut ids = kept;
    ids.insert(2, None);
    let reshaped = apply(
        &v1_painted,
        &DocEdit::SetProgram {
            node: profile,
            loops: vec![
                LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (3.0, 1.0), (2.0, 2.0), (0.0, 2.0)])
                    .unwrap(),
            ],
            ids: vec![ids],
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .unwrap();
    assert_eq!(
        reshaped.maintenance,
        Vec::new(),
        "a reshaping that keeps every step has nothing to report"
    );
    let mut shelf = VersionShelf::default();
    let r1 = shelf.shelve(v1_painted);
    let r2 = shelf.shelve(reshaped.doc);
    let shelf = Arc::new(shelf);

    let parent = ProfileDoc::empty(DocumentId::derive("held-names-parent"), Tol::witness());
    let (parent, instance) = insert(parent, Node::instantiate_part(r1));
    let name = held(instance, &wall);
    let parent = apply(
        &parent,
        &DocEdit::SetAppearance {
            name: name.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .unwrap()
    .doc;
    let before = corners(&run(&parent, &shelf), instance, &name);
    assert!(
        before.contains(&(2.0, 0.0, 0.0)) && before.contains(&(2.0, 2.0, 0.0)),
        "the held name is the part's wall (2,0)→(2,2): {before:?}"
    );

    let updated = apply(
        &parent,
        &DocEdit::UpdateReference {
            node: instance,
            new_pin: r2.pin,
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .unwrap();
    let after_ev = run(&updated.doc, &shelf);
    let after = corners(&after_ev, instance, &name);
    assert!(
        after.contains(&(3.0, 1.0, 0.0)) && after.contains(&(2.0, 2.0, 0.0)),
        "the held name denotes the leg its step draws now, (3,1)→(2,2): {after:?}"
    );
    assert!(
        !after.contains(&(2.0, 0.0, 0.0)),
        "and never the inserted leg (2,0)→(3,1): {after:?}"
    );
    assert_eq!(updated.maintenance, Vec::new(), "nothing is reported");
}

/// The part's profile step ids, loop 0.
fn step_ids(doc: &ProfileDoc, profile: RecipeNodeId) -> Vec<editor_core::StepId> {
    match doc.node(profile) {
        Some(Node::Profile(p)) => p.ids[0].clone(),
        other => panic!("the part's profile: {other:?}"),
    }
}

/// `base` with one leg inserted into its square at program position
/// `at`, the new step minted by the door: the square's corners with
/// `corner` spliced in as corner `at`.
fn with_leg(base: &ProfileDoc, profile: RecipeNodeId, at: usize, corner: (f64, f64)) -> ProfileDoc {
    let mut corners = vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    corners.insert(at, corner);
    let mut ids: Vec<Option<editor_core::StepId>> =
        step_ids(base, profile).into_iter().map(Some).collect();
    ids.insert(at, None);
    apply(
        base,
        &DocEdit::SetProgram {
            node: profile,
            loops: vec![LoopProgram::polygon(corners).unwrap()],
            ids: vec![ids],
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .unwrap()
    .doc
}

/// **Two versions of one part that branch from one value.** Version A
/// inserts a leg to `(3,1)` after the corner `(2,0)`; version B, made
/// from the same base (an undo and a different edit, or a second
/// `apply` on the base), inserts a leg to `(1,3)` after `(2,2)`
/// instead. The step counter is part of the document value, so both
/// mint the same id for their different legs. The parent pins A and
/// paints A's new leg, then moves its pin to B — the version a store
/// holds once B is saved over A (`pncad::workspace::update_to_store`).
///
/// This row pins the defect the tracker row
/// `sibling-branches-mint-one-step-id-for-different-steps` records: the
/// held name silently re-denotes B's leg and nothing is reported. It is
/// the row that turns when that is fixed.
#[test]
fn sibling_versions_mint_one_step_id_and_a_held_name_crosses_between_them() {
    let (base, profile, ext) = part();
    let a = with_leg(&base, profile, 2, (3.0, 1.0));
    let b = with_leg(&base, profile, 3, (1.0, 3.0));
    let a_new = step_ids(&a, profile)[2];
    let b_new = step_ids(&b, profile)[3];
    assert_eq!(
        a_new, b_new,
        "each branch mints its new step from the base's counter"
    );

    let mut shelf = VersionShelf::default();
    let ra = shelf.shelve(a);
    let rb = shelf.shelve(b);
    let shelf = Arc::new(shelf);

    let wall = fixture::fname(
        ext,
        RoleSeg::Lateral(editor_core::ProfileEdgeRef::Piece {
            step: a_new,
            role: editor_core::PieceRole::Leg,
        }),
    );
    let parent = ProfileDoc::empty(DocumentId::derive("held-names-parent"), Tol::witness());
    let (parent, instance) = insert(parent, Node::instantiate_part(ra));
    let name = held(instance, &wall);
    let parent = apply(
        &parent,
        &DocEdit::SetAppearance {
            name: name.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .unwrap()
    .doc;
    let before = corners(&run(&parent, &shelf), instance, &name);
    assert!(
        before.contains(&(2.0, 0.0, 0.0)) && before.contains(&(3.0, 1.0, 0.0)),
        "at A the held name is A's leg (2,0)→(3,1): {before:?}"
    );

    let updated = apply(
        &parent,
        &DocEdit::UpdateReference {
            node: instance,
            new_pin: rb.pin,
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .unwrap();
    assert_eq!(
        updated.maintenance,
        Vec::new(),
        "the update reports nothing"
    );
    let after = corners(&run(&updated.doc, &shelf), instance, &name);
    assert!(
        after.contains(&(2.0, 2.0, 0.0)) && after.contains(&(1.0, 3.0, 0.0)),
        "at B the same spelling is B's leg (2,2)→(1,3), a step A never had: {after:?}"
    );
}

/// **Node ids branch the same way.** Two inserts applied to one base
/// mint one `RecipeNodeId` for two different nodes, so a name minted by
/// either node carries across to the other branch as the other node's.
#[test]
fn sibling_versions_mint_one_node_id_for_different_nodes() {
    let (base, profile, _) = part();
    let (_, tall) = insert(
        base.clone(),
        Node::Extrude {
            profile,
            distance: len(3.0),
        },
    );
    let (_, taller) = insert(
        base,
        Node::Extrude {
            profile,
            distance: len(5.0),
        },
    );
    assert_eq!(
        tall, taller,
        "each branch mints its new node from the base's counter"
    );
}
