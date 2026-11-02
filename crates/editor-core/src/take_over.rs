//! **A takeover** (A10): copies of a body re-pointed to a result that
//! replaces it, with every reference read in those copies that the
//! result still carries rebound to the result's name for it, in one
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
//! when something reads a copy: a selection of the document (of the
//! copy, of a copy of it — a copy carries its source's names as they
//! are, so a placement chain from the copy reads them too — or one that
//! embeds any of their reads), or a reader outside the document that
//! holds a name on one of them (the caller's `held`, a viewer's picks).
//! Otherwise it writes the re-point alone. Its evaluation rides out
//! with the answer ([`TakenOver::evaluation`]) for the run of the
//! finished document to start from.

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

/// **What a takeover did**: each copy it re-pointed, where the names
/// of each copy that reads them went, and the evaluation the door read
/// that off, where it made one.
///
/// Every reference the document holds is already rebound by the
/// action's own edits. A reader that holds a name outside the document
/// (a viewer's pick of a face of a copy) follows [`Self::moved`].
#[derive(Clone, Debug, Default)]
pub struct TakenOver {
    /// Each copy re-pointed, and what it copied and copies now.
    pub copies: BTreeMap<RecipeNodeId, Taken>,
    /// Where each name moved, per copy that reads it: a re-pointed
    /// copy, and each copy of a copy down its placement chain. Empty
    /// where the door did not evaluate (nothing read a copy).
    pub moved: Moved,
    /// The staged document's evaluation, before the rebinds, where the
    /// door evaluated: a run of the finished document can start from
    /// it, since only what reads the rebound selections differs.
    pub evaluation: Option<Arc<Evaluation<f64>>>,
}

impl TakenOver {
    /// Whether the takeover re-pointed no copy.
    pub fn is_empty(&self) -> bool {
        self.copies.is_empty()
    }
}

/// **Where a takeover moved the names a copy holds**: per copy, each
/// name of the old source the result carries as one entity, and the
/// result's name for it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Moved(pub BTreeMap<RecipeNodeId, BTreeMap<StableName, StableName>>);

impl Moved {
    /// The name `name`, read in `copy` before the takeover, has now;
    /// `None` where the takeover did not move it.
    pub fn of(&self, copy: RecipeNodeId, name: &StableName) -> Option<&StableName> {
        self.0.get(&copy)?.get(name)
    }

    /// Whether no name moved.
    pub fn is_empty(&self) -> bool {
        self.0.values().all(BTreeMap::is_empty)
    }
}

impl Recording<'_, ProfileProgram> {
    /// **Re-point each of `copies` to `result`, and rebind what read
    /// them** ([module docs](self)): each copy's source slot set to
    /// `result`, then, where something reads a copy, one
    /// [`DocEdit::Rebind`] per (selection body, name) whose name has a
    /// successor in the result, at whatever depth the selection embeds
    /// a copy's read. `held` says which drawn copies a reader outside
    /// the document holds names on. The staged document is evaluated
    /// under `opts` to read the result's table, `prior` its memo.
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
    /// [`EditError::TakeOverRepeatsCopy`] for a copy given twice, and
    /// as [`Self::apply`]'s: the first door that refuses ends the action.
    pub fn take_over(
        &mut self,
        copies: &[RecipeNodeId],
        result: &Operand,
        opts: &EvalOptions,
        prior: Option<&Evaluation<f64>>,
        held: &dyn Fn(RecipeNodeId) -> bool,
    ) -> Result<TakenOver, EditError> {
        let mut taken = TakenOver::default();
        for &copy in copies {
            if taken.copies.contains_key(&copy) {
                return Err(EditError::TakeOverRepeatsCopy {
                    node: self.doc().spoken(copy),
                });
            }
            let was = self.doc().copied_read(copy);
            self.apply(DocEdit::SetParam {
                node: copy,
                slot: crate::SlotId::Operand(crate::OperandSlot::Source),
                value: crate::SlotValue::Read(result.clone()),
                fresh: Vec::new(),
            })?;
            #[allow(clippy::expect_used)] // the slot door's own guarantee
            let (was, now) = (
                was.expect("the slot door admitted a source, so `copy` is a placement"),
                self.doc()
                    .copied_read(copy)
                    .expect("the slot door admitted a source, so `copy` is a placement"),
            );
            taken.copies.insert(copy, Taken { was, now });
        }
        let doc = self.doc();
        let chains = chains(doc, &taken);
        // Each body that carries a copy's names as they are, by the
        // re-pointed copy whose names they are.
        let bodies: BTreeMap<VarId, RecipeNodeId> = chains
            .iter()
            .flat_map(|(&node, &root)| doc.outputs(node).into_iter().map(move |var| (var, root)))
            .collect();
        let read_inside = doc.name_carriers().any(|carrier| match carrier {
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
        if !read_inside && !chains.keys().any(|&node| held(node)) {
            return Ok(taken);
        }
        // A link down a chain may itself read the copy (a mirror's plane
        // on one of its faces): it evaluates only once that read is
        // rebound. So the door evaluates, rebinds, and evaluates again
        // from that run until a round rebinds nothing; each round takes
        // the names it moved out of the document, so it ends within the
        // chain's depth.
        let mut ev: Evaluation<f64> =
            crate::evaluate(doc, prior, &crate::CancelToken::new(), opts, self.tol());
        loop {
            let doc = self.doc();
            let (edits, moved) = rebinds_of(doc, &ev, &taken, &chains, &bodies);
            if edits.is_empty() {
                taken.moved = moved;
                break;
            }
            for edit in edits {
                self.apply(edit)?;
            }
            ev = crate::evaluate(
                self.doc(),
                Some(&ev),
                &crate::CancelToken::new(),
                opts,
                self.tol(),
            );
        }
        taken.evaluation = Some(Arc::new(ev));
        Ok(taken)
    }
}

/// **One round of a takeover's rebinds, read off `ev`**, and where the
/// names of each chain node moved ([`Moved`]).
///
/// A name read in a chain body (`bodies`, by the re-pointed copy whose
/// names it carries) moved where that copy's successor did
/// ([`crate::names::successors`]) and the body's own table holds the new
/// name as one entity: a mirror and a copy carry every name of the
/// copy, a split those the geometry leaves intact, so the body's table
/// is the one that says. A name read downstream carries such names
/// inside its own, keyed by the body's read, and holds only where its
/// own body's table has the new name as one entity.
fn rebinds_of(
    doc: &Doc<ProfileProgram>,
    ev: &Evaluation<f64>,
    taken: &TakenOver,
    chains: &BTreeMap<RecipeNodeId, RecipeNodeId>,
    bodies: &BTreeMap<VarId, RecipeNodeId>,
) -> (Vec<DocEdit<ProfileProgram>>, Moved) {
    let one = |var: VarId, to: &StableName| {
        ev.table(var)
            .and_then(|table| table.lookup(to))
            .is_some_and(|entry| matches!(entry, Entry::Unique(_)))
    };
    let by_copy: BTreeMap<RecipeNodeId, BTreeMap<StableName, StableName>> = taken
        .copies
        .iter()
        .map(|(&copy, taken)| {
            let moved = ev
                .table(taken.now)
                .map(|table| crate::names::successors(table, taken.was))
                .unwrap_or_default();
            (copy, moved)
        })
        .collect();
    let in_body: BTreeMap<VarId, BTreeMap<StableName, StableName>> = bodies
        .iter()
        .filter_map(|(&var, root)| {
            let map = by_copy.get(root)?;
            Some((
                var,
                map.iter()
                    .filter(|(_, to)| one(var, to))
                    .map(|(from, to)| (from.clone(), to.clone()))
                    .collect(),
            ))
        })
        .collect();
    let edits = crate::refactor::rebinds_at_sites(doc, |site, name| {
        let body = site?;
        match in_body.get(&body) {
            Some(map) => map.get(name).cloned(),
            None => crate::names::recarried(name, &in_body).filter(|to| one(body, to)),
        }
    });
    let moved = Moved(
        chains
            .keys()
            .map(|&node| {
                let map = doc
                    .outputs(node)
                    .iter()
                    .filter_map(|var| in_body.get(var))
                    .flatten()
                    .map(|(from, to)| (from.clone(), to.clone()))
                    .collect();
                (node, map)
            })
            .collect(),
    );
    (edits, moved)
}

/// **Every node that carries a re-pointed copy's names as they are**,
/// by the re-pointed copy: each re-pointed copy itself, and each node
/// down a chain of name-carrying edges from it — a copy of the copy, a
/// mirror of it, a split of it — walked over the one census of those
/// edges (`names::verbatim_edge`), so a carrying node added there is
/// followed here without being remembered. An instance's copy carries
/// its part's names, read in no body of this document, and starts no
/// chain.
fn chains(doc: &Doc<ProfileProgram>, taken: &TakenOver) -> BTreeMap<RecipeNodeId, RecipeNodeId> {
    let mut chains: BTreeMap<RecipeNodeId, RecipeNodeId> =
        taken.copies.keys().map(|&copy| (copy, copy)).collect();
    let mut frontier: Vec<RecipeNodeId> = taken.copies.keys().copied().collect();
    while let Some(at) = frontier.pop() {
        let Some(&root) = chains.get(&at) else {
            continue;
        };
        let outs = doc.outputs(at);
        for node in doc.ids() {
            if !chains.contains_key(&node)
                && doc
                    .node(node)
                    .and_then(crate::names::verbatim_edge)
                    .and_then(|edge| edge.carried_read())
                    .is_some_and(|read| outs.contains(&read))
            {
                chains.insert(node, root);
                frontier.push(node);
            }
        }
    }
    chains
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

/// **A takeover, as one action** ([`Recording::take_over`]), its caller
/// holding a name on every copy: it always answers where they moved.
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
    // A caller of the action holds every name it may be handed back.
    let taken = action.take_over(copies, result, opts, None, &|_| true)?;
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
