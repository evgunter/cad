//! **The delete cascade's wording**: what the delete button says, and
//! the census of what it says it about.
//!
//! A VOCABULARY. The affordance is composed from a document and a node
//! id; nothing here names the session, and the sentences are testable
//! without a window.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use editor_core::continuation::{Splice, splice};
use pncad::document::{Doc, Maintenance, ProfileProgram, RecipeNodeId, Recording, VarId};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use std::collections::BTreeSet;

/// **What the delete button says, and the list it says it about.**
///
/// A destructive action that understates itself is worse than one that
/// refuses: [`super::SessionOp::DeleteNode`] hands each reader of the
/// target to the body it continues and takes the rest of its downstream
/// cone (`editor_core::continuation::splice`), which can be most of the document,
/// so the count belongs on the button and not only in a tooltip.
///
/// The wording lives here rather than in the chrome for the reason
/// [`super::Refusal::exists_wording`] does: one composition, read by whatever
/// renders it, and testable without a window.
#[derive(Clone, Debug)]
pub struct DeleteAffordance {
    /// The button's label.
    pub label: String,
    /// The hover text; `None` when the label already says everything,
    /// which is exactly when nothing depends on the target and the
    /// delete strands no name.
    pub hover: Option<String>,
    /// The nodes the delete would remove, consumers first and the
    /// target last — the operation's own order.
    pub cascade: Vec<RecipeNodeId>,
    /// **The names the delete would leave resolving to nothing** (DM7):
    /// the references the stranded-name rows of
    /// [`super::OpOutcome::maintenance`] are about once the click is
    /// accepted, each counted once.
    ///
    /// The END state's count: a name a hand-over rebinds to the body the
    /// target continues, and one whose carrier the cascade goes on to
    /// delete, are not in it.
    pub stranded: usize,
}

impl DeleteAffordance {
    /// Compose the sentences for deleting `node` out of `doc`, each
    /// edit of the delete rehearsed at `tol`.
    ///
    /// The node is named as the document speaks it, kind noun and tag
    /// (first-light finding #1097: reached from a face selection, a
    /// bare "Delete feature" read as deleting the *face* — an entity
    /// this vocabulary can never delete). A node the document does not
    /// hold reads `node <tag>`, which a face or edge selection can
    /// still name while the landed run holds its feature: that button
    /// is drawn disabled, with the op's refusal on its hover
    /// (`DocSession::delete_refusal`).
    ///
    /// Neither sentence mentions the features that merely FED the
    /// target, or the readers handed to what it continues: the delete
    /// does not remove them.
    pub(super) fn of(doc: &Doc<ProfileProgram>, node: RecipeNodeId, tol: Tol) -> Self {
        // A delete that cannot splice deletes nothing: its button is drawn
        // disabled with the refusal (`DocSession::delete_refusal`).
        let (cascade, stranded) = match splice(doc, node) {
            Ok(Some(plan)) => (plan.deleted.clone(), stranded_names(doc, plan, tol)),
            Ok(None) | Err(_) => (Vec::new(), 0),
        };
        let spoken = doc.spoken(node);
        // The cascade's last entry is the target itself; everything
        // before it is a dependent.
        let dependents = cascade.len().saturating_sub(1);
        let also = (dependents > 0).then(|| {
            let (plural, depend) = if dependents == 1 {
                ("", "depends")
            } else {
                ("s", "depend")
            };
            let census = kind_census(doc, &cascade[..dependents]);
            (
                format!(" and {dependents} dependent feature{plural}"),
                format!("Also deletes {dependents} feature{plural} that {depend} on it: {census}"),
            )
        });
        let strands = (stranded > 0).then(|| {
            let (plural, resolve) = if stranded == 1 {
                ("", "resolves")
            } else {
                ("s", "resolve")
            };
            (
                format!(", stranding {stranded} name{plural}"),
                format!(
                    "Strands {stranded} name{plural} that the document keeps and that \
                     {resolve} to nothing until rebound"
                ),
            )
        });
        let (labels, hovers): (Vec<_>, Vec<_>) = also.into_iter().chain(strands).unzip();
        Self {
            label: format!("Delete {spoken}{}", labels.concat()),
            hover: (!hovers.is_empty()).then(|| hovers.join("\n")),
            cascade,
            stranded,
        }
    }
}

/// **How many names the planned delete leaves resolving to nothing**:
/// `plan` applied to a scratch recording of `doc`, whose net
/// (`editor_core::MaintenanceNet`) is the rows the click's outcome
/// carries, and the references those rows are about counted.
///
/// References, not rows: a hand-over that re-points a reader past the
/// target and the delete of the target each report the one name the
/// reader holds, and the document holds it once.
///
/// A plan a door refuses deletes nothing, so it strands nothing.
fn stranded_names(doc: &Doc<ProfileProgram>, plan: Splice<ProfileProgram>, tol: Tol) -> usize {
    let mut run = Recording::start(doc, tol);
    let rows = match plan.apply(&mut run).and_then(|()| run.finish()) {
        Ok(recorded) => recorded.maintenance,
        Err(_) => Vec::new(),
    };
    rows.iter()
        .filter_map(stranded_reference)
        .collect::<BTreeSet<_>>()
        .len()
}

/// Where the document holds a stranded name.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Holder {
    /// A node's payload.
    Payload(RecipeNodeId),
    /// A selection.
    Selection(VarId),
    /// The appearance store, keyed under a body read.
    Appearance(VarId),
}

/// **The reference `row` reports stranded**: the name the document
/// still holds whose referent the action removed, and where it holds
/// it. `None` for a row about anything else.
fn stranded_reference(row: &Maintenance) -> Option<(Holder, &StableName)> {
    match row {
        Maintenance::Strand { node, name, .. } => Some((Holder::Payload(node.id()), name.name())),
        Maintenance::StrandedSelection { var, name, .. } => {
            Some((Holder::Selection(var.id()), name.name()))
        }
        Maintenance::StrandedAppearance { body, name, .. } => {
            Some((Holder::Appearance(body.id()), name.name()))
        }
        Maintenance::StrandedRead { .. }
        | Maintenance::StrandedSpace { .. }
        | Maintenance::ProductOutsideWorld { .. }
        | Maintenance::ValueDropped { .. }
        | Maintenance::AnonymousVarRemoved { .. } => None,
    }
}

/// A census of node kinds, most numerous first and ties broken by
/// name, as `20 × Subtract, 1 × Fillet` — the readable form of a list
/// whose LENGTH is the thing being warned about.
fn kind_census(doc: &Doc<ProfileProgram>, nodes: &[RecipeNodeId]) -> String {
    let mut counts: std::collections::BTreeMap<&'static str, usize> =
        std::collections::BTreeMap::new();
    for &id in nodes {
        if let Some(kind) = doc.spoken(id).kind() {
            *counts.entry(kind).or_default() += 1;
        }
    }
    let mut census: Vec<(&'static str, usize)> = counts.into_iter().collect();
    census.sort_by_key(|&(kind, count)| (core::cmp::Reverse(count), kind));
    census
        .into_iter()
        .map(|(kind, count)| format!("{count} × {kind}"))
        .collect::<Vec<_>>()
        .join(", ")
}
