//! **The delete cascade's wording**: what the delete button says, and
//! the census of what it says it about.
//!
//! A VOCABULARY. The affordance is composed from a document and a node
//! id; nothing here names the session, and the sentences are testable
//! without a window.

use pncad::document::{Doc, ProfileProgram, RecipeNodeId, cascade_delete_order};

use crate::tree;

/// **What the delete button says, and the list it says it about.**
///
/// A destructive action that understates itself is worse than one that
/// refuses: [`super::SessionOp::DeleteNode`] takes the target's whole
/// downstream cone, which in a chain-shaped model (a boolean per
/// feature) is most of the document, so the count belongs on the
/// button and not only in a tooltip.
///
/// The wording lives here rather than in the chrome for the reason
/// [`super::Refusal::exists_wording`] does: one composition, read by whatever
/// renders it, and testable without a window.
#[derive(Clone, Debug)]
pub struct DeleteAffordance {
    /// The button's label.
    pub label: String,
    /// The hover text; `None` when the label already says everything,
    /// which is exactly when nothing depends on the target.
    pub hover: Option<String>,
    /// The nodes the delete would remove, consumers first and the
    /// target last — the operation's own order.
    pub cascade: Vec<RecipeNodeId>,
}

impl DeleteAffordance {
    /// Compose the sentences for deleting `node` out of `doc`.
    ///
    /// The kind name is the node vocabulary's own (first-light finding
    /// #1097: reached from a face selection, a bare "Delete feature"
    /// read as deleting the *face* — an entity this vocabulary can
    /// never delete). The id-only arm is for a node the document does
    /// not hold: no button renders for one today, and if that changes
    /// the label stays honest rather than panicking.
    ///
    /// Neither sentence mentions the features that merely FED the
    /// target and survive as roots of their own, because this delete
    /// does not touch them. A delete that reconnected a target's
    /// consumers to its input instead — splice — is open as issue
    /// #1324.
    pub(super) fn of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Self {
        let cascade = cascade_delete_order(doc, node);
        let Some(target) = doc.node(node) else {
            return Self {
                label: format!("Delete feature {}", node.0),
                hover: None,
                cascade,
            };
        };
        let kind = tree::node_kind(target);
        // The cascade's last entry is the target itself; everything
        // before it is a dependent.
        let dependents = cascade.len().saturating_sub(1);
        if dependents == 0 {
            return Self {
                label: format!("Delete feature '{kind}'"),
                hover: None,
                cascade,
            };
        }
        let (plural, depend) = if dependents == 1 {
            ("", "depends")
        } else {
            ("s", "depend")
        };
        let census = kind_census(doc, &cascade[..dependents]);
        Self {
            label: format!("Delete feature '{kind}' and {dependents} dependent feature{plural}"),
            hover: Some(format!(
                "Also deletes {dependents} feature{plural} that {depend} on it: {census}"
            )),
            cascade,
        }
    }
}

/// A census of node kinds, most numerous first and ties broken by
/// name, as `20 × Boolean, 1 × Fillet` — the readable form of a list
/// whose LENGTH is the thing being warned about.
fn kind_census(doc: &Doc<ProfileProgram>, nodes: &[RecipeNodeId]) -> String {
    let mut counts: std::collections::BTreeMap<&'static str, usize> =
        std::collections::BTreeMap::new();
    for &id in nodes {
        if let Some(node) = doc.node(id) {
            *counts.entry(tree::node_kind(node)).or_default() += 1;
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
