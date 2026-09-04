//! The Features pane: the feature tree, one row per recipe node.

use eframe::egui;

use crate::app::{GLYPH_ROOT, ViewerBehavior, chrome};
use crate::session::{Selection, SessionOp};
use crate::tree::{RowStatus, TreeRow};

/// Points of indent per level of the feature tree.
pub(crate) const INDENT_STEP: f32 = 12.0;

/// The deepest level the tree indents for.
///
/// A backstop, not the working limit: `tree`'s depth counts BRANCHES
/// off a node's primary input, so a chained document sits a handful
/// of levels deep however long its chain is. A document that does
/// nest genuinely deeper than this stops moving right here; the rows
/// stay in evaluation order, so the tree is still readable as a
/// sequence, and what is lost is depth information that had already
/// stopped fitting the pane.
pub(crate) const INDENT_MAX_DEPTH: usize = 8;

/// The indent a row at `depth` draws at.
pub(crate) fn indent(depth: usize) -> f32 {
    depth.min(INDENT_MAX_DEPTH) as f32 * INDENT_STEP
}

impl ViewerBehavior<'_> {
    /// The feature tree: one row per recipe node, with its status
    /// badge from the evaluation's typed result.
    pub(crate) fn features_ui(&mut self, ui: &mut egui::Ui) {
        let rows = self.session.tree_rows();
        ui.horizontal(|ui| {
            ui.heading("Features");
            ui.label(format!("{} nodes", rows.len()));
        });
        ui.separator();
        // A face picked in the viewport highlights its owning
        // feature here — one selection value, one inversion. Overflow
        // is `pane_ui`'s scroll container's job, the same one every
        // chrome pane sits in; a second ScrollArea here would nest.
        let selected = self.session.selection().node();
        for row in &rows {
            self.feature_row(ui, row, selected == Some(row.id));
        }
    }

    /// One feature-tree row.
    pub(crate) fn feature_row(&mut self, ui: &mut egui::Ui, row: &TreeRow, selected: bool) {
        ui.horizontal(|ui| {
            ui.add_space(indent(row.depth));
            let label = if row.root {
                format!("{} {GLYPH_ROOT}", row.kind)
            } else {
                row.kind.to_owned()
            };
            if ui.selectable_label(selected, label).clicked() {
                self.ops.push(SessionOp::Select(Selection::Node(row.id)));
            }
            // The hide toggle, on instance rows only: a hidden
            // instance stays IN this tree (that is the point — the
            // tree is the document, the viewport is the display), and
            // the checkbox is the display op's chrome.
            if row.kind == "InstantiatePart" {
                let mut shown = !self.display.hidden.contains(&row.id);
                if ui.checkbox(&mut shown, "shown").changed() {
                    self.ops.push(SessionOp::SetInstanceHidden {
                        instance: row.id,
                        hidden: !shown,
                    });
                }
            }
            match &row.status {
                RowStatus::Ok => {}
                // Nothing to act on HERE: the row was never run, or it
                // shows someone else's failure and points at the row
                // that owns it. Quiet, so the eye passes over it.
                RowStatus::Unevaluated | RowStatus::Poisoned { .. } => {
                    ui.weak(row.status.badge());
                }
                // The ACTIONABLE rows — the nodes whose own operation
                // refused — are the ones that take the colour, so a
                // document with six rows downstream of one broken
                // feature sends the eye to the one. There can be more
                // than one: a `MateFault::Contradictory` naming two
                // different mates blames both, and both go red
                // (`tree::blamed_mates`).
                RowStatus::Failed { .. } => {
                    ui.colored_label(chrome(self.theme.unresolved), row.status.badge());
                }
            }
        });
        // The line under the row: the payload's own words where the
        // row failed, and where it did not, the pointer at the row
        // that has them — which is a CLICK, so "that row" is one
        // gesture away rather than an id to hunt for.
        if let Some(message) = row.status.message() {
            let through = match &row.status {
                RowStatus::Poisoned { through, .. } => Some(*through),
                _ => None,
            };
            ui.horizontal(|ui| {
                ui.add_space(indent(row.depth) + INDENT_STEP);
                match through {
                    Some(through) => {
                        if ui.link(message).clicked() {
                            self.ops.push(SessionOp::Select(Selection::Node(through)));
                        }
                    }
                    None => {
                        ui.weak(message);
                    }
                }
            });
        }
        // The node's standing caveat (a mate class with no at-rest
        // record) — the admission verdict, outliving the commit.
        if let Some(note) = &row.note {
            ui.horizontal(|ui| {
                ui.add_space(indent(row.depth) + INDENT_STEP);
                ui.weak(note);
            });
        }
    }
}
