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

/// **What one feature-tree row reads as, drawn**: the node's kind,
/// which one of its kind it is, and the root glyph.
///
/// **The kind alone is not a name.** A tree of rows reading `Datum
/// frame` twice asks a person to tell two frames apart by clicking;
/// the pose the node itself states is what separates them
/// ([`crate::tree::frame_pose`]), and it is the same sentence the
/// creation forms' picker puts after that node's number.
///
/// A free function over the `Ui` because that is the only shape a
/// headless drive can reach ([`crate::pane::headless`]): the caller is
/// a method on `ViewerBehavior`, which borrows the whole application.
pub(crate) fn row_label(ui: &mut egui::Ui, row: &TreeRow, selected: bool) -> egui::Response {
    let named = match &row.pose {
        Some(pose) => format!("{} — {pose}", row.kind),
        None => row.kind.to_owned(),
    };
    let label = if row.root {
        format!("{named} {GLYPH_ROOT}")
    } else {
        named
    };
    ui.selectable_label(selected, label)
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
            if row_label(ui, row, selected).clicked() {
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

/// **The feature tree's rows, driven** — [`crate::pane::headless`]
/// carries the harness and what it can and cannot reach.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use pncad::document::RecipeNodeId;

    use super::row_label;
    use crate::app::GLYPH_ROOT;
    use crate::pane::headless::painted_text;
    use crate::tree::{RowStatus, TreeRow};

    /// A row as `tree::rows` builds one for a `Datum::Frame` node.
    fn frame_row(id: u64, pose: &str) -> TreeRow {
        TreeRow {
            id: RecipeNodeId(id),
            kind: "Datum frame",
            pose: Some(pose.to_owned()),
            depth: 0,
            root: false,
            status: RowStatus::Ok,
            note: None,
        }
    }

    /// **The pane draws which frame the row is**, not the kind alone.
    ///
    /// The tree's half of this unit. A `TreeRow` that carries a pose
    /// the pane drops is the same defect the row was filed against
    /// with a field added, and until this row existed deleting the
    /// composition reddened nothing in either suite.
    #[test]
    fn a_frame_row_says_which_frame_it_is() {
        let drawn = painted_text(|ui| {
            row_label(ui, &frame_row(3, "xy at (0, 0, 0) m"), false);
        });
        assert!(drawn.contains("Datum frame"), "{drawn}");
        assert!(
            drawn.contains("xy at (0, 0, 0) m"),
            "the pose the node states reaches the row a person reads: {drawn}"
        );
    }

    /// Two frames a centimetre apart are two different rows.
    #[test]
    fn two_frame_rows_a_centimetre_apart_read_differently() {
        let one = painted_text(|ui| {
            row_label(ui, &frame_row(3, "xy at (0, 0, 0) m"), false);
        });
        let other = painted_text(|ui| {
            row_label(ui, &frame_row(7, "xy at (0, 0, 0.01) m"), false);
        });
        assert_ne!(one, other, "{one} / {other}");
    }

    /// A node with no pose reads as its kind, with no dangling
    /// separator where the sentence would have been — and a root
    /// still carries its glyph.
    #[test]
    fn a_row_with_nothing_more_to_say_reads_as_its_kind() {
        let row = TreeRow {
            id: RecipeNodeId(1),
            kind: "Extrude",
            pose: None,
            depth: 0,
            root: true,
            status: RowStatus::Ok,
            note: None,
        };
        let drawn = painted_text(|ui| {
            row_label(ui, &row, false);
        });
        assert!(drawn.contains("Extrude"), "{drawn}");
        assert!(
            !drawn.contains('—'),
            "no separator with nothing after it: {drawn}"
        );
        assert!(
            drawn.contains(GLYPH_ROOT),
            "a root keeps its glyph: {drawn}"
        );
    }

    /// A root frame carries both: the pose AND the glyph, in that
    /// order — the composition the glyph arm is written around.
    #[test]
    fn a_root_frame_row_carries_the_pose_and_the_glyph() {
        let mut row = frame_row(2, "yz at (0, 0, 0) m");
        row.root = true;
        let drawn = painted_text(|ui| {
            row_label(ui, &row, false);
        });
        assert!(
            drawn.contains(&format!("Datum frame — yz at (0, 0, 0) m {GLYPH_ROOT}")),
            "{drawn}"
        );
    }
}
