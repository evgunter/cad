//! **A parent's names spelled in a child document's numbering, across
//! the child's reshaping and the parent's pin update.**
//!
//! An assembly holds names of a part's entities as `InPart { of }` at
//! the instantiate node: `of` is the name the part document mints, so
//! it is spelled in the part's own numbering — for a swept profile,
//! that profile's canonical segment order. When the part is reshaped
//! its own names are carried and reported (DM7), and the parent is a
//! different document the part's edit door never sees.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Attr, CancelToken, ContentPin, DocEdit, DocRef, DocumentId, EvalOptions, Evaluation,
    LoopProgram, LoopProvenance, Node, ProfileDoc, ProfileProgram, RecipeNodeId, ResolveFailure,
    ResolveFault, Rgba8, RoleSeg, StableName, apply, content_pin, evaluate,
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

fn run(doc: &ProfileDoc, shelf: &Arc<VersionShelf>) -> Evaluation<f64> {
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

/// The part's wall `k` of its one loop, as the part names it.
fn part_wall(ext: RecipeNodeId, k: u32) -> StableName {
    fixture::fname(ext, fixture::wall(k))
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
/// part inserts a leg before its wall 1 (`(2,0)→(2,2)`), so its wall 1
/// is its wall 2 now — the part's own edit rebinds every name it holds
/// and reports it. The parent paints the part's wall 1 through its
/// instance, then moves its pin to the reshaped version.
///
/// Measured: `UpdateReference` reports nothing and rewrites nothing,
/// and the parent's held spelling `InPart { part wall 1 }` now denotes
/// the leg `(2,0)→(3,1)` — a DIFFERENT wall, silently. This row PINS
/// that defect, so it is the row that turns when it is fixed
/// (`work/emit/a-child-documents-rebind-leaves-the-parents-held-names-in-the-old-numbering.md`,
/// P0, which says why the translation does not survive to the pin
/// update).
#[test]
fn a_parents_held_name_silently_renumbers_across_a_pin_update() {
    let (v1, profile, ext) = part();
    // The part's own reshaping, and what its door reports for its own
    // names (a paint on its wall 1, so the report has a row to show).
    let v1_painted = apply(
        &v1,
        &DocEdit::SetAppearance {
            name: part_wall(ext, 1),
            attr: Attr::Color(Rgba8::opaque(1, 2, 3)),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .unwrap()
    .doc;
    let reshaped = apply(
        &v1_painted,
        &DocEdit::SetProgram {
            node: profile,
            loops: vec![
                LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (3.0, 1.0), (2.0, 2.0), (0.0, 2.0)])
                    .unwrap(),
            ],
            provenance: vec![LoopProvenance {
                from: Some(0),
                steps: vec![Some(0), Some(1), None, Some(2), Some(3), Some(4)],
            }],
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .unwrap();
    assert_eq!(
        reshaped.maintenance,
        vec![editor_core::Maintenance::Rebound {
            from: part_wall(ext, 1),
            to: part_wall(ext, 2),
        }],
        "the part's own door carries its own name"
    );
    let mut shelf = VersionShelf::default();
    let r1 = shelf.shelve(v1_painted);
    let r2 = shelf.shelve(reshaped.doc);
    let shelf = Arc::new(shelf);

    let parent = ProfileDoc::empty(DocumentId::derive("held-names-parent"), Tol::witness());
    let (parent, instance) = insert(parent, Node::instantiate_part(r1));
    let name = held(instance, &part_wall(ext, 1));
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
    let spelling = corners(&after_ev, instance, &name);
    assert!(
        spelling.contains(&(3.0, 1.0, 0.0)),
        "measured: the old spelling now denotes the leg (2,0)→(3,1): {spelling:?}"
    );
    assert_eq!(
        updated.maintenance,
        Vec::new(),
        "measured: nothing is reported"
    );
}
