//! **A takeover** (A10): copies of a body re-pointed to a result that
//! replaces it, with every reference read in those copies that the
//! result still carries re-seated on the result's name for it, in one
//! action.
//!
//! A re-pointed copy reads the result's table, where a name of the body
//! it copied is carried in by the result (`From { read, of }`, or a
//! member of a `Merged` set): the old name no longer resolves there.
//! Resolution does not follow descent on its own (N4, `[identity]`), so
//! the takeover writes the repair as the person would, an explicit
//! [`DocEdit::Rebind`] per reference, for each name that has a
//! successor ([`crate::names::successors`]). A name the result splits,
//! ties or consumes has none, and its reference stays stale with the
//! result's own diagnosis.
//!
//! **Not a refactoring.** A refactoring (`crate::refactor`) rewrites a
//! document into one that evaluates to the same shapes; a takeover
//! changes what the copies show, so it is its own door.
//!
//! **Why it does not read like [`crate::flatten`].** Flatten deletes a
//! copy, so every read of it moves to the shape it copied: each reader's
//! body read is re-authored, and a pose leaf or a pick re-anchored. A
//! takeover keeps the copy, its identity, its bundle and every read of
//! it; only what it carries changes, so only the names inside its
//! readers move. A pick of a copy is never re-pointed (D10
//! `[placement.placement]`), and a caller that would strand one
//! refuses before it gets here (the viewer's `Refusal::StrandedPick`).
//!
//! **What it evaluates.** Reading where a name went needs the result's
//! table, an evaluation of the staged document. The door runs one only
//! when the document reads a copy (a selection of it, or one that
//! embeds its read at any depth); otherwise it writes the re-point
//! alone. A reader that holds names outside the document reads where
//! they went off any later evaluation that holds the result
//! ([`TakenOver::reseated`]), the run its caller makes anyway; the
//! door's own evaluation, when it made one, rides out with the answer
//! ([`TakenOver::evaluation`]) for that run to start from.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::edit::{DocEdit, EditError, Recorded, Recording};
use crate::names::{Entry, StableName};
use crate::node::RecipeNodeId;
use crate::program::ProfileProgram;
use crate::var::VarId;
use crate::{Doc, EvalOptions, Evaluation, Maintenance, Operand};
use geom_core::Tol;

/// **One copy a takeover re-pointed**: the read it copied, and the one
/// it copies now.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Taken {
    /// The body read the copy copied before the takeover.
    pub was: VarId,
    /// The body read it copies now: the result's.
    pub now: VarId,
}

/// **What a takeover did**: each copy it re-pointed, and the
/// evaluation the door read the result's table off, where it made one.
///
/// Every reference the document holds is already re-seated by the
/// action's own edits. A reader that holds a name outside the document
/// (a viewer's pick of a face of the copy) reads where it went with
/// [`Self::reseated`].
#[derive(Clone, Debug, Default)]
pub struct TakenOver {
    /// Each copy re-pointed, and what it copied and copies now.
    pub copies: BTreeMap<RecipeNodeId, Taken>,
    /// The staged document's evaluation, before the rebinds, where the
    /// door evaluated (the document read a copy): a run of the
    /// finished document can start from it, since only what reads the
    /// rebound selections differs.
    pub evaluation: Option<Arc<Evaluation<f64>>>,
}

impl TakenOver {
    /// Whether the takeover re-pointed no copy.
    pub fn is_empty(&self) -> bool {
        self.copies.is_empty()
    }

    /// **Where each name of each copy went**, read off `ev`, an
    /// evaluation of a document that holds the result: a copy carries
    /// its source's names as they are, so the names the copy holds now
    /// are the result's, and each of the old body's names that has a
    /// successor there moved to it.
    pub fn reseated(&self, ev: &Evaluation<f64>) -> Reseated {
        Reseated(
            self.copies
                .iter()
                .filter_map(|(&copy, taken)| {
                    let table = ev.table(taken.now)?;
                    Some((copy, crate::names::successors(table, taken.was)))
                })
                .collect(),
        )
    }
}

/// **Where the names of the copies a takeover re-pointed went**: per
/// copy, each name the copy's old source held that the result carries
/// as one entity, and the result's name for it ([`TakenOver::reseated`]).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Reseated(pub BTreeMap<RecipeNodeId, BTreeMap<StableName, StableName>>);

impl Reseated {
    /// The name `name`, read in `copy` before the takeover, has in it
    /// now; `None` where the takeover did not move it.
    pub fn reseat(&self, copy: RecipeNodeId, name: &StableName) -> Option<&StableName> {
        self.0.get(&copy)?.get(name)
    }
}

impl Recording<'_, ProfileProgram> {
    /// **Re-point each of `copies` to `result`, and re-seat what read
    /// them** ([module docs](self)): each copy's source slot set to
    /// `result`, then, where the document reads a copy, one
    /// [`DocEdit::Rebind`] per (selection body, name) whose name has a
    /// successor in the result, at whatever depth the selection embeds
    /// the copy's read. The staged document is evaluated under `opts`
    /// to read the result's table, `prior` its memo.
    ///
    /// Which copies a takeover reaches is the caller's: each is
    /// re-pointed as given, and one that is no placement refuses at the
    /// slot door ([`EditError::UnknownSlot`]: only a `Place` carries a
    /// source).
    ///
    /// A name held where no selection holds it (an instance record's
    /// `outer`, an appearance key) is not rebound: [`DocEdit::Rebind`]
    /// reaches those only all together, whatever read each is keyed at,
    /// and the copy's old names are its source's, so the repair would
    /// also move a key the source's own read holds under the name (issue
    /// `a-takeover-leaves-payload-names-and-appearance-read-in-the-copy-stale`).
    ///
    /// # Errors
    ///
    /// As [`Self::apply`]'s: the first door that refuses ends the action.
    pub fn take_over(
        &mut self,
        copies: &[RecipeNodeId],
        result: &Operand,
        opts: &EvalOptions,
        prior: Option<&Evaluation<f64>>,
    ) -> Result<TakenOver, EditError> {
        let mut taken = TakenOver::default();
        for &copy in copies {
            let was = self.doc().copied_read(copy);
            self.apply(DocEdit::SetParam {
                node: copy,
                slot: crate::SlotId::Operand(crate::OperandSlot::Source),
                value: crate::SlotValue::Read(result.clone()),
                fresh: Vec::new(),
            })?;
            // The slot door admitted the source, so `copy` is a `Place`
            // and both reads are there.
            if let (Some(was), Some(now)) = (was, self.doc().copied_read(copy)) {
                taken.copies.insert(copy, Taken { was, now });
            }
        }
        let doc = self.doc();
        // The copies' bodies, by the read a selection of one names.
        let bodies: BTreeMap<VarId, RecipeNodeId> = taken
            .copies
            .keys()
            .flat_map(|&copy| doc.outputs(copy).into_iter().map(move |var| (var, copy)))
            .collect();
        let read = doc.name_carriers().any(|carrier| match carrier {
            crate::doc::NameCarrier::Select { var, name } => {
                doc.selection(var)
                    .is_some_and(|select| bodies.contains_key(&select.body))
                    || crate::resolve::derivation_reads(name)
                        .iter()
                        .any(|r| bodies.contains_key(r))
            }
            // Not rebound (above): nothing here to evaluate for.
            crate::doc::NameCarrier::Payload { .. } | crate::doc::NameCarrier::Store { .. } => {
                false
            }
        });
        if !read {
            return Ok(taken);
        }
        let ev: Evaluation<f64> =
            crate::evaluate(doc, prior, &crate::CancelToken::new(), opts, self.tol());
        let reseated = taken.reseated(&ev);
        // A name read in a copy's body is that copy's; one read
        // downstream carries such names inside its own, keyed by the
        // copy's read.
        let moved: BTreeMap<VarId, &BTreeMap<StableName, StableName>> = bodies
            .iter()
            .filter_map(|(&var, copy)| Some((var, reseated.0.get(copy)?)))
            .collect();
        let edits = crate::refactor::rebinds_at_sites(doc, |site, name| {
            let body = site?;
            match moved.get(&body) {
                // A successor is a unique row of the copy's table.
                Some(map) => map.get(name).cloned(),
                // A downstream body re-keys its own rows: its new name
                // holds only where its table has it as one entity.
                None => crate::names::reseated(name, &moved).filter(|to| {
                    ev.table(body)
                        .and_then(|table| table.lookup(to))
                        .is_some_and(|entry| matches!(entry, Entry::Unique(_)))
                }),
            }
        });
        for edit in edits {
            self.apply(edit)?;
        }
        taken.evaluation = Some(Arc::new(ev));
        Ok(taken)
    }
}

/// [`fn@take_over`]'s outcome: the action applied, and what it
/// re-pointed.
#[derive(Debug)]
pub struct TakeOverOutcome {
    /// The document the action produced.
    pub doc: Doc<ProfileProgram>,
    /// The edits that produce `doc` from the start.
    pub edits: Vec<DocEdit<ProfileProgram>>,
    /// The maintenance they reported.
    pub maintenance: Vec<Maintenance>,
    /// What the takeover re-pointed.
    pub taken: TakenOver,
}

/// **A takeover, as one action** ([`Recording::take_over`]).
///
/// # Errors
///
/// The first door's refusal; the document is untouched.
pub fn take_over(
    doc: &Doc<ProfileProgram>,
    copies: &[RecipeNodeId],
    result: &Operand,
    opts: &EvalOptions,
    tol: Tol,
) -> Result<TakeOverOutcome, EditError> {
    let mut action = Recording::start(doc, tol);
    let taken = action.take_over(copies, result, opts, None)?;
    let Recorded {
        doc,
        edits,
        maintenance,
        ..
    } = action.finish()?;
    Ok(TakeOverOutcome {
        doc,
        edits,
        maintenance,
        taken,
    })
}
