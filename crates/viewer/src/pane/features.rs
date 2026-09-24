//! The Features pane: the feature tree, one row per recipe node.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::document::RecipeNodeId;

use crate::app::{GLYPH_ROOT, ViewerBehavior, toned};
use crate::frame;
use crate::session::{Selection, SessionOp};
use crate::theme::Theme;
use crate::tree::{self, RowStatus, TreeRow};

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

/// **The indent a line UNDER a row draws at** — a failure's own
/// words, the pointer at the row that has them, a standing note.
///
/// One step past the row's own [`indent`], so the line reads as the
/// row's — for as long as that leaves the line the width
/// [`crate::widgets::message_floor`] names. Past that the indent gives
/// way first: a deep row in a narrow pane draws its line further left
/// rather than wrapping it into a ribbon or pushing it past the pane's
/// edge, since the depth the indent shows is already on the row above
/// it and the sentence's width is not on screen anywhere else.
///
/// `ui` is the line's own row, before anything is placed in it.
pub(crate) fn message_indent(ui: &egui::Ui, depth: usize) -> f32 {
    let spare = (ui.available_width() - crate::widgets::message_floor(ui)).max(0.0);
    (indent(depth) + INDENT_STEP).min(spare)
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
/// headless drive can reach (`crate::pane::headless`): the caller is
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

/// **The lines under a row that a failure writes, drawn** — and the
/// node a click on one of them selects, when one was clicked.
///
/// The payload's own words where the row failed, and where it did not,
/// the pointer at the row that has them — which is a CLICK, so "that
/// row" is one gesture away rather than an id to hunt for. A failed
/// row whose words name ANOTHER node to repair
/// ([`TreeRow::repair_at`]) keeps its words as they are and gets a
/// second line that is that click.
///
/// A free function over the `Ui` for the reason [`row_label`] is one.
pub(crate) fn failure_lines(
    ui: &mut egui::Ui,
    row: &TreeRow,
    theme: &Theme,
) -> Option<RecipeNodeId> {
    let message = row.status.message()?;
    // Where the WORDS go, and where a line under them goes: which row
    // a status links to is decided per state.
    let (words_to, then_to) = match &row.status {
        RowStatus::Poisoned { through, .. } => (Some(*through), None),
        // The words are this row's own cause; the link, when the tree
        // drew one, is to the node those words say to repair.
        RowStatus::Failed { .. } => (None, row.repair_at),
        // No line to link ([`RowStatus::message`]).
        RowStatus::Ok | RowStatus::Unevaluated => (None, None),
    };
    let mut clicked = None;
    ui.horizontal(|ui| {
        ui.add_space(message_indent(ui, row.depth));
        // A payload's own words are a sentence, so
        // `widgets::message`, not `ui.link`/`ui.weak`.
        match words_to {
            Some(to) => {
                if crate::widgets::message_link(ui, message).clicked() {
                    clicked = Some(to);
                }
            }
            None => {
                crate::widgets::message_toned(ui, message, theme, frame::Tone::Advisory);
            }
        }
    });
    if let Some(to) = then_to {
        ui.horizontal(|ui| {
            ui.add_space(message_indent(ui, row.depth));
            if crate::widgets::message_link(ui, tree::repair_wording(to)).clicked() {
                clicked = Some(to);
            }
        });
    }
    clicked
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
            // Whether a row draws a badge at all is this pane's
            // decision.
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
        if let Some(to) = failure_lines(ui, row, &self.theme) {
            self.ops.push(SessionOp::Select(Selection::Node(to)));
        }
        // The node's standing caveat (a mate class with no at-rest
        // record) — the admission verdict, outliving the commit.
        if let Some(note) = &row.note {
            ui.horizontal(|ui| {
                ui.add_space(message_indent(ui, row.depth));
                crate::widgets::message_toned(
                    ui,
                    note.as_str(),
                    &self.theme,
                    frame::Tone::Advisory,
                );
            });
        }
    }
}

/// **The feature tree's rows, driven** — `crate::pane::headless`
/// carries the harness and what it can and cannot reach.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use pncad::document::RecipeNodeId;

    use eframe::egui;

    use super::{INDENT_MAX_DEPTH, INDENT_STEP, failure_lines, indent, message_indent, row_label};
    use crate::app::GLYPH_ROOT;
    use crate::pane::headless::{landed, painted_after_clicking, painted_text};
    use crate::theme::Theme;
    use crate::tree;
    use crate::tree::{RowStatus, TreeRow};
    use crate::widgets::message_tests::SLACK;
    use crate::widgets::{message, message_floor};

    /// A failure line of the length and shape a refusal has, quoting a
    /// number.
    const FAILURE: &str = "the offset is 0.30000000000000004 mm, which the solver refused";

    /// One headless frame of a line under a row at `depth`, drawn the
    /// way `feature_row` draws one, in a region `spare` points wider
    /// than [`message_floor`]. Answers with the region, the floor, and
    /// the rows [`FAILURE`] landed in.
    fn line_under_a_row(depth: usize, spare: f32) -> (egui::Rect, f32, Vec<egui::Rect>) {
        let region = core::cell::Cell::new(egui::Rect::NOTHING);
        let floor = core::cell::Cell::new(f32::NAN);
        let painted = landed(|ui| {
            floor.set(message_floor(ui));
            ui.allocate_ui(egui::vec2(floor.get() + spare, 400.0), |ui| {
                region.set(ui.max_rect());
                ui.horizontal(|ui| {
                    ui.add_space(message_indent(ui, depth));
                    message(ui, FAILURE);
                });
            });
        });
        let rows = painted
            .into_iter()
            .find(|landed| landed.text == FAILURE)
            .expect("the failure line was painted")
            .rows;
        (region.get(), floor.get(), rows)
    }

    /// **A deep row's line gives up its indent before its width**, in
    /// a pane with room for the floor and not for the indent too.
    ///
    /// The measured site of the ribbon: at [`INDENT_MAX_DEPTH`] the
    /// line's indent is `8 * 12 + 12` points, and a pane that leaves a
    /// sentence less than the floor after it would, with the indent
    /// taken whole, lay the line out at the floor from there and run
    /// it past the pane's right edge by the difference.
    #[test]
    fn a_deep_rows_line_gives_up_its_indent_before_its_width() {
        let wanted = indent(INDENT_MAX_DEPTH) + INDENT_STEP;
        let spare = wanted / 2.0;
        let (region, floor, rows) = line_under_a_row(INDENT_MAX_DEPTH, spare);
        let past = rows
            .iter()
            .map(|row| row.right() - region.right())
            .fold(f32::NEG_INFINITY, f32::max);
        assert!(
            past <= SLACK,
            "the line stays inside a pane {spare} points wider than the \
             {floor}-point floor ({past} points past, region {region:?}, rows {rows:?})"
        );
        assert!(
            rows[0].left() - region.left() > SLACK,
            "and it keeps the indent the pane has room for, rather than \
             dropping to the pane's edge ({:?} in {region:?})",
            rows[0]
        );
    }

    /// **And where there is room, the line keeps the row's indent
    /// whole** — the depth a person reads the tree by does not move
    /// for a pane that has space for it.
    #[test]
    fn a_line_under_a_row_in_a_wide_pane_keeps_its_whole_indent() {
        let wanted = indent(INDENT_MAX_DEPTH) + INDENT_STEP;
        let (region, _, rows) = line_under_a_row(INDENT_MAX_DEPTH, wanted * 4.0);
        let at = rows[0].left() - region.left();
        assert!(
            (at - wanted).abs() <= SLACK,
            "the line begins {at} points in, where the row's indent and \
             one step put it at {wanted}"
        );
    }

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
            repair_at: None,
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
            repair_at: None,
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

    /// A mate row refused because its placer (`feature 3`) did not
    /// derive, as `tree::rows` builds one: `Failed`, with the link.
    fn placer_refused_row(repair_at: Option<RecipeNodeId>) -> TreeRow {
        TreeRow {
            id: RecipeNodeId(7),
            kind: "Mate",
            pose: None,
            depth: 0,
            root: false,
            status: RowStatus::Failed {
                message: FAILURE.to_owned(),
            },
            note: None,
            repair_at,
        }
    }

    /// **What [`failure_lines`] answers when the text `target` is
    /// clicked**, through `pane::headless`'s one click drive.
    fn clicking(row: &TreeRow, target: &str) -> Option<RecipeNodeId> {
        let clicked = core::cell::Cell::new(None);
        painted_after_clicking(target, |ui| {
            if let Some(to) = failure_lines(ui, row, &Theme::DEFAULT) {
                clicked.set(Some(to));
            }
        });
        clicked.get()
    }

    /// **A failed row whose words name another node to repair links to
    /// it, and the click selects that node** — the words themselves
    /// stay this row's own and go nowhere.
    #[test]
    fn a_failed_rows_link_to_the_node_to_repair_selects_it() {
        let placer = RecipeNodeId(3);
        let row = placer_refused_row(Some(placer));
        let link = tree::repair_wording(placer);
        assert_eq!(link, "see feature 3", "the chrome's one spelling of a node");
        assert_eq!(clicking(&row, &link), Some(placer));
        assert_eq!(
            clicking(&row, FAILURE),
            None,
            "the words are the cause itself"
        );
    }

    /// And a failed row with nothing to repair elsewhere draws its
    /// words alone.
    #[test]
    fn a_failed_row_with_no_node_to_repair_draws_no_link() {
        let drawn = painted_text(|ui| {
            failure_lines(ui, &placer_refused_row(None), &Theme::DEFAULT);
        });
        assert!(drawn.contains(FAILURE), "{drawn}");
        assert!(!drawn.contains("see "), "no link line: {drawn}");
    }

    /// A poisoned row's pointer is still the click to `through`.
    #[test]
    fn a_poisoned_rows_pointer_selects_the_row_it_names() {
        let through = RecipeNodeId(7);
        let pointer = tree::downstream_wording(through);
        let row = TreeRow {
            status: RowStatus::Poisoned {
                through,
                message: Some(pointer.clone()),
            },
            ..placer_refused_row(None)
        };
        assert_eq!(clicking(&row, &pointer), Some(through));
    }
}
