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
//! [`DocEdit::Rebind`] per reference, for each name whose carrier is
//! unique ([`crate::names::carriers`]). A name the result splits, ties
//! or consumes has no carrier and its reference stays stale, with the
//! result's own diagnosis.

use std::collections::{BTreeMap, BTreeSet};

use crate::edit::{DocEdit, EditError, Recorded, Recording};
use crate::names::{Entry, StableName};
use crate::node::{Node, RecipeNodeId};
use crate::program::ProfileProgram;
use crate::var::VarId;
use crate::{Doc, EvalOptions, Evaluation, Maintenance, Operand};
use geom_core::Tol;

/// **What a takeover re-seated**: for each copy it re-pointed, each name
/// the copy's old source held that the result carries uniquely, and the
/// result's name for it.
///
/// Every reference the document holds is already re-seated by the
/// action's own edits; this is for a reader that holds a name outside
/// the document (a viewer's pick of a face of the copy).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TakenOver {
    /// Per re-pointed copy, its old name → its name now.
    pub reseated: BTreeMap<RecipeNodeId, BTreeMap<StableName, StableName>>,
}

impl TakenOver {
    /// The name `name`, read in `copy` before the takeover, has in it
    /// now; `None` where the takeover did not move it.
    pub fn reseat(&self, copy: RecipeNodeId, name: &StableName) -> Option<&StableName> {
        self.reseated.get(&copy)?.get(name)
    }
}

impl Recording<'_, ProfileProgram> {
    /// **Re-point each of `copies` to `result`, and re-seat what read
    /// them** ([module docs](self)): the copy's source slot set to
    /// `result`, then one [`DocEdit::Rebind`] per (body, name) whose
    /// name the result carries uniquely, at whatever depth the
    /// selection embeds the copy's read. The staged document is
    /// evaluated under `opts` to read the result's table; `prior` is
    /// that evaluation's memo.
    ///
    /// Which copies a takeover reaches is the caller's: each is
    /// re-pointed as given.
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
        let copied: Vec<(RecipeNodeId, VarId)> = copies
            .iter()
            .filter_map(|&copy| match self.doc().node(copy) {
                Some(Node::Place { source, .. }) => Some((copy, source.read)),
                _ => None,
            })
            .collect();
        for &copy in copies {
            self.apply(DocEdit::SetParam {
                node: copy,
                slot: crate::SlotId::Operand(crate::OperandSlot::Source),
                value: crate::SlotValue::Read(result.clone()),
                fresh: Vec::new(),
            })?;
        }
        let doc = self.doc();
        let ev: Evaluation<f64> =
            crate::evaluate(doc, prior, &crate::CancelToken::new(), opts, self.tol());
        let mut taken = TakenOver::default();
        let mut moved: BTreeMap<VarId, BTreeMap<StableName, StableName>> = BTreeMap::new();
        for (copy, read) in copied {
            let seated = taken.reseated.entry(copy).or_default();
            for var in doc.outputs(copy) {
                if let Some(table) = ev.table(var) {
                    let map = crate::names::carriers(table, read);
                    seated.extend(map.clone());
                    moved.insert(var, map);
                }
            }
        }
        let mut rebinds: BTreeSet<(VarId, StableName, StableName)> = BTreeSet::new();
        for select in doc.vars().values().filter_map(|var| var.def().select()) {
            for name in &select.names {
                let to = match moved.get(&select.body) {
                    Some(map) => map.get(name).cloned(),
                    None => crate::names::reseated(name, &moved),
                };
                let Some(to) = to else { continue };
                let held = ev.table(select.body).and_then(|table| table.lookup(&to));
                if matches!(held, Some(Entry::Unique(_))) {
                    rebinds.insert((select.body, name.clone(), to));
                }
            }
        }
        for (body, from, to) in rebinds {
            self.apply(DocEdit::Rebind {
                body: Some(body),
                from,
                to,
            })?;
        }
        Ok(taken)
    }
}

/// [`fn@take_over`]'s outcome: the action applied, and what it
/// re-seated.
#[derive(Debug)]
pub struct TakeOverOutcome {
    /// The document the action produced.
    pub doc: Doc<ProfileProgram>,
    /// The edits that produce `doc` from the start.
    pub edits: Vec<DocEdit<ProfileProgram>>,
    /// The maintenance they reported.
    pub maintenance: Vec<Maintenance>,
    /// What the takeover re-seated.
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
