//! **One copy of a body placed in the world where it stands**
//! ([`place_in_world`]): a loose `Node::place` of the body pinned by
//! the named interim's world pin (stage 3 C), which reads nothing, so
//! the copy follows every variable its body reads.
//!
//! The placement is two edits applied in order, the insert and the
//! pin; [`Placed::edits`] is that sequence, so a scene that exports its
//! edit log replays exactly what it applied.

use pncad::document::{BodyRead, Doc, DocEdit, Formula, Node, ProfileProgram, RecipeNodeId, apply};
use pncad::geom_core::Tol;

/// What the placement left: the document, the placement it minted, and
/// the edits it applied, in order.
pub struct Placed {
    pub doc: Doc<ProfileProgram>,
    pub placement: RecipeNodeId,
    pub edits: Vec<DocEdit<ProfileProgram>>,
}

/// One copy of `body` placed in the world where it stands: a loose
/// `Node::place` of the body pinned by a world pin
/// (`DocEdit::PinWhereItStands`, stage 3 C's named interim), which
/// reads nothing and holds no number, so the copy follows every
/// variable its body reads.
///
/// # Panics
///
/// On any refusal of its two edits, naming it.
pub fn place_in_world(
    doc: &Doc<ProfileProgram>,
    body: impl Into<BodyRead<Formula>>,
    tol: Tol,
) -> Placed {
    let mut edits = Vec::new();
    let mut step = |doc: &Doc<ProfileProgram>, edit: DocEdit<ProfileProgram>| {
        let applied = apply(doc, &edit, tol)
            .unwrap_or_else(|err| panic!("the placement's edit applies: {err}"));
        edits.push(edit);
        applied
    };
    let inserted = step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(Node::place(vec![body.into()])),
            fresh: Vec::new(),
        },
    );
    let placement = inserted.record.minted.expect("an insert mints an id");
    let doc = step(&inserted.doc, DocEdit::PinWhereItStands { placement }).doc;
    Placed {
        doc,
        placement,
        edits,
    }
}
