//! **The delete cascade's wording**: what the delete button says, and
//! the census of what it says it about.
//!
//! A VOCABULARY. The affordance is composed from a document and a node
//! id; nothing here names the session, and the sentences are testable
//! without a window.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{Doc, Maintenance, ProfileProgram, RecipeNodeId};

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
    /// delete, are not in it. A name one carrier holds twice is one
    /// name: a rebind repairs every site of it at once.
    pub stranded: usize,
}

impl DeleteAffordance {
    /// Compose the sentences for deleting `node` out of `doc`, from the
    /// delete rehearsed: `cascade`, the nodes it removes in the
    /// operation's order, and `report`, the maintenance rows its
    /// accepted action carries. A delete that would be refused removes
    /// nothing and reports nothing; its button is drawn disabled with
    /// the refusal (`DocSession::delete_refusal`).
    ///
    /// The node is named as the document speaks it, kind noun and tag
    /// (first-light finding #1097: reached from a face selection, a
    /// bare "Delete feature" read as deleting the *face* — an entity
    /// this vocabulary can never delete). A node the document does not
    /// hold reads `node <tag>`, which a face or edge selection can
    /// still name while the landed run holds its feature.
    ///
    /// Neither sentence mentions the features that merely FED the
    /// target, or the readers handed to what it continues: the delete
    /// does not remove them.
    pub(super) fn of(
        doc: &Doc<ProfileProgram>,
        node: RecipeNodeId,
        cascade: Vec<RecipeNodeId>,
        report: &[Maintenance],
    ) -> Self {
        let stranded = stranded_names(report);
        let mut label = format!("Delete {}", crate::tree::spelled(&doc.spoken(node)));
        let mut hover = Vec::new();
        // The cascade's last entry is the target itself; everything
        // before it is a dependent.
        let dependents = cascade.len().saturating_sub(1);
        if dependents > 0 {
            let (plural, depend) = if dependents == 1 {
                ("", "depends")
            } else {
                ("s", "depend")
            };
            let census = kind_census(doc, &cascade[..dependents]);
            label.push_str(&format!(" and {dependents} dependent feature{plural}"));
            hover.push(format!(
                "Also deletes {dependents} feature{plural} that {depend} on it: {census}"
            ));
        }
        if stranded > 0 {
            let (plural, resolve) = if stranded == 1 {
                ("", "resolves")
            } else {
                ("s", "resolve")
            };
            label.push_str(&format!(", stranding {stranded} name{plural}"));
            hover.push(format!(
                "Strands {stranded} name{plural} that the document keeps and that {resolve} to \
                 nothing until rebound"
            ));
        }
        Self {
            label,
            hover: (!hover.is_empty()).then(|| hover.join("\n")),
            cascade,
            stranded,
        }
    }
}

/// **How many names `report` says the document keeps with their
/// referent gone**: the distinct answers of
/// [`Maintenance::stranded_name`], because two rows of one action can
/// report one name of one carrier.
fn stranded_names(report: &[Maintenance]) -> usize {
    let mut held = Vec::new();
    for name in report.iter().filter_map(Maintenance::stranded_name) {
        if !held.contains(&name) {
            held.push(name);
        }
    }
    held.len()
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
