//! The Features pane: the feature tree, one row per recipe node.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;

use crate::app::{GLYPH_ROOT, ViewerBehavior, toned};
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
            // **The kind, and which one of its kind it is.** A tree
            // of rows reading `Datum frame` twice asks a person to
            // tell two frames apart by clicking; the pose the node
            // itself states is what separates them, and it is the
            // same string the creation forms' picker offers that node
            // by (`tree::node_label`).
            let named = match &row.pose {
                Some(pose) => format!("{} — {pose}", row.kind),
                None => row.kind.to_owned(),
            };
            let label = if row.root {
                format!("{named} {GLYPH_ROOT}")
            } else {
                named
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
            // **Exhaustive on purpose**: whether a row draws a badge
            // at all is this pane's decision, so a status the kernel
            // grows has to answer it here rather than fall into a
            // wildcard and draw.
            //
            // How LOUD a drawn badge is, is not decided here — that is
            // `RowStatus::tone()`, read below.
            match &row.status {
                // Silent: a healthy row's own line is the whole of
                // what it has to say, and a tree of unmarked rows is
                // what makes the marked ones carry. The status still
                // has a badge, which `examples/r1_e2e.rs` prints.
                RowStatus::Ok => {}
                RowStatus::Unevaluated | RowStatus::Poisoned { .. } | RowStatus::Failed { .. } => {
                    ui.label(toned(row.status.badge(), &self.theme, row.status.tone()));
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
