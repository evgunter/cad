//! The Features pane: the feature tree, one row per recipe node.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::document::{AssertionVerdict, RecipeNodeId, SpokenNode, UnevaluatedReason};

use crate::app::{GLYPH_ROOT, ViewerBehavior, toned};
use crate::frame;
use crate::props::Notation;
use crate::session::{Refusal, Selection, SessionOp, VersionOffer};
use crate::theme::Theme;
use crate::tree::{self, Readout, RowStatus, TreeRow};

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

/// **What one feature-tree row reads as, drawn**: the node as the
/// document speaks it (kind noun and tag), which one of its kind it
/// is, and the root glyph.
///
/// **The kind alone is not a name.** A tree of rows reading `Datum
/// frame` twice asks a person to tell two frames apart by clicking;
/// the tag tells them apart, and the pose the node itself states is
/// what tells a person WHICH frame ([`crate::tree::frame_pose`]) — the
/// same words the creation forms' picker reads ([`crate::tree::node_label`]).
pub(crate) fn row_label(ui: &mut egui::Ui, row: &TreeRow, selected: bool) -> egui::Response {
    let named = tree::named(row.spoken, row.pose.as_deref());
    let label = if row.root {
        format!("{named} {GLYPH_ROOT}")
    } else {
        named
    };
    ui.selectable_label(selected, label)
}

/// **One feature-tree row, drawn**: its line — the indent, the label,
/// the instance toggle, what the run said — and every line under it.
/// Answers what a click asked for.
///
/// A free function over the `Ui` because that is the only shape a
/// headless drive can reach (`crate::pane::headless`): the caller is
/// a method on `ViewerBehavior`, which borrows the whole application,
/// and all it does with the answer is push the ops it names.
///
/// `hidden` is whether the display hides this row's node. Only an
/// instance row draws the hide toggle: a hidden instance stays IN this
/// tree (that is the point — the tree is the document, the viewport is
/// the display), and the checkbox is the display op's chrome.
pub(crate) fn feature_row_ui(
    ui: &mut egui::Ui,
    row: &TreeRow,
    selected: bool,
    hidden: bool,
    theme: &Theme,
    notation: Notation,
) -> RowClicks {
    let mut clicks = RowClicks::default();
    ui.horizontal(|ui| {
        ui.add_space(indent(row.depth));
        if row_label(ui, row, selected).clicked() {
            clicks.select = Some(row.id);
        }
        if row.spoken.kind() == Some("InstantiatePart") {
            let mut shown = !hidden;
            if ui.checkbox(&mut shown, "shown").changed() {
                clicks.hide = Some(!shown);
            }
        }
        row_result(ui, row, theme, notation);
    });
    if let Some(to) = lines_under(ui, row, theme) {
        clicks.select = Some(to);
    }
    if let Some(offer) = &row.version_offer
        && version_offer_lines(ui, row.depth, offer, theme)
    {
        clicks.accept = Some(offer.accept());
    }
    clicks
}

/// **The accept a pin-mismatched instance row offers, drawn** under
/// its failure: the question, and the button that is the edit the
/// failure's recourse names. Whether the button was clicked.
fn version_offer_lines(
    ui: &mut egui::Ui,
    depth: usize,
    offer: &VersionOffer,
    theme: &Theme,
) -> bool {
    advisory_line(ui, depth, &Refusal::version_question(offer), theme);
    ui.horizontal(|ui| {
        ui.add_space(message_indent(ui, depth));
        ui.button(VersionOffer::LABEL).clicked()
    })
    .inner
}

/// What a click on one drawn feature row asked for.
#[derive(Debug, Default)]
pub(crate) struct RowClicks {
    /// The node to select: the row's own, from its label, or the one a
    /// line under it points at.
    pub(crate) select: Option<RecipeNodeId>,
    /// Whether the instance should now be hidden, when its toggle was
    /// clicked.
    pub(crate) hide: Option<bool>,
    /// The operation accepting the row's [`TreeRow::version_offer`],
    /// when its button was clicked.
    pub(crate) accept: Option<SessionOp>,
}

/// **The lines under a row that a failure writes, drawn** — and the
/// node a click on one of them selects, when one was clicked.
///
/// The payload's own words where the row failed, and where it did not,
/// the pointer at the row that has them — which is a CLICK, so "that
/// row" is one gesture away rather than an id to hunt for. A failed
/// row whose words name ANOTHER node to repair
/// ([`TreeRow::repair_at`]) keeps its words as they are and gets a
/// line that is that click, under the refusals its words carry
/// ([`crate::tree::carried_lines`]), one line per level.
pub(crate) fn failure_lines(
    ui: &mut egui::Ui,
    row: &TreeRow,
    theme: &Theme,
) -> Option<RecipeNodeId> {
    let message = row.status.message()?;
    let mut clicked = None;
    match row.status.jump() {
        Some(to) => {
            if link_line(ui, row.depth, message) {
                clicked = Some(to);
            }
        }
        // Advisory whatever the status: how loud a row is, is its
        // badge's, read off `RowStatus::tone` on the row above.
        // The line under it is that verdict's words, and a failed
        // row drawn loud twice would be the one loud row twice.
        None => advisory_line(ui, row.depth, message, theme),
    }
    // The refusals the row's words point at, each one step further in:
    // the traceback reads down the way the failure reaches in. Each is
    // headed by the document its node is in, a label of its own, so
    // the refusal is drawn as its own tree draws it.
    if let RowStatus::Failed { carried, .. } = &row.status {
        for (level, carried) in carried.iter().enumerate() {
            let depth = row.depth + 1 + level;
            ui.horizontal(|ui| {
                ui.add_space(message_indent(ui, depth));
                ui.weak(&carried.document);
            });
            advisory_line(ui, depth, &carried.line, theme);
        }
    }
    if let Some(to) = row.repair_at {
        clicked = link_to(ui, row.depth, to).or(clicked);
    }
    clicked
}

/// **What the run said about a row, drawn beside it**: the badge of a
/// row that is not `Ok`, and on one that is, what its value says — a
/// measure's value, an assertion's verdict (its state in the kernel's
/// word, at the verdict's own tone, and the comparison it decided), or
/// which part of the value is empty.
///
/// Whether a row draws a badge at all is this pane's decision. How
/// LOUD a drawn badge or verdict is, is not decided here — that is
/// `TreeRow::tone()`.
fn row_result(ui: &mut egui::Ui, row: &TreeRow, theme: &Theme, notation: Notation) {
    match &row.status {
        // A healthy row draws no badge: a tree of unmarked rows is
        // what makes the marked ones carry. The status still has one,
        // which `examples/r1_e2e.rs` prints.
        RowStatus::Ok => match &row.readout {
            Some(Readout::Value(value)) => {
                ui.label(value.spelled(notation));
            }
            Some(Readout::Empty(emptiness)) => {
                ui.label(emptiness.to_string());
            }
            Some(Readout::Asserted(asserted)) => {
                ui.label(toned(asserted.verdict.label(), theme, row.tone()));
                if let Some(comparison) = asserted.comparison(notation) {
                    ui.label(comparison);
                }
            }
            // Its reason is a sentence, drawn under the row.
            Some(Readout::Unavailable(_)) | None => {}
        },
        RowStatus::Unevaluated | RowStatus::Poisoned { .. } | RowStatus::Failed { .. } => {
            ui.label(toned(row.status.badge(), theme, row.tone()));
        }
    }
}

/// **Every line drawn under a row**, in order: a failure's
/// ([`failure_lines`]), why a measure has no value or an assertion no
/// verdict, and the node's standing caveat. Answers the node a click
/// selects, as [`failure_lines`] does.
fn lines_under(ui: &mut egui::Ui, row: &TreeRow, theme: &Theme) -> Option<RecipeNodeId> {
    let mut clicked = failure_lines(ui, row, theme);
    match &row.readout {
        Some(Readout::Unavailable(reason)) => {
            advisory_line(ui, row.depth, &reason.to_string(), theme);
        }
        Some(Readout::Asserted(asserted)) => match &asserted.verdict {
            AssertionVerdict::Unevaluated {
                reason: UnevaluatedReason::MeasureUnavailable(_),
            } => clicked = link_to(ui, row.depth, asserted.measure).or(clicked),
            AssertionVerdict::Unevaluated {
                reason:
                    reason @ (UnevaluatedReason::Indeterminate { .. }
                    | UnevaluatedReason::WindowSuperset { .. }),
            } => advisory_line(ui, row.depth, &reason.to_string(), theme),
            AssertionVerdict::Holds { .. } | AssertionVerdict::Violated { .. } => {}
        },
        Some(Readout::Value(_) | Readout::Empty(_)) | None => {}
    }
    if let Some(note) = &row.note {
        advisory_line(ui, row.depth, note, theme);
    }
    clicked
}

/// A line under a row at `depth` that reports and links nowhere. A
/// payload's own words are a sentence, so `widgets::message`, not
/// `ui.weak`.
fn advisory_line(ui: &mut egui::Ui, depth: usize, text: &str, theme: &Theme) {
    ui.horizontal(|ui| {
        ui.add_space(message_indent(ui, depth));
        crate::widgets::message_toned(ui, text, theme, frame::Tone::Advisory);
    });
}

/// **A line under a row at `depth` that links to the node `to`**, in
/// [`tree::link_wording`]'s words: `to`'s id when it was clicked. The
/// wording and the target are one argument, so they cannot name two
/// different nodes.
fn link_to(ui: &mut egui::Ui, depth: usize, to: SpokenNode) -> Option<RecipeNodeId> {
    link_line(ui, depth, &tree::link_wording(to)).then_some(to.id())
}

/// A line under a row at `depth` that is a click; whether it was
/// clicked.
fn link_line(ui: &mut egui::Ui, depth: usize, text: &str) -> bool {
    ui.horizontal(|ui| {
        ui.add_space(message_indent(ui, depth));
        crate::widgets::message_link(ui, text).clicked()
    })
    .inner
}

impl ViewerBehavior<'_> {
    /// The feature tree: one row per recipe node, with what the landed
    /// run said about it ([`feature_row_ui`]).
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

    /// One feature-tree row ([`feature_row_ui`]), and the ops its
    /// clicks name.
    pub(crate) fn feature_row(&mut self, ui: &mut egui::Ui, row: &TreeRow, selected: bool) {
        let hidden = self.display.hidden.contains(&row.id);
        let clicks = feature_row_ui(ui, row, selected, hidden, &self.theme, *self.notation);
        if let Some(to) = clicks.select {
            self.ops.push(SessionOp::Select(Selection::Node(to)));
        }
        if let Some(hidden) = clicks.hide {
            self.ops.push(SessionOp::SetInstanceHidden {
                instance: row.id,
                hidden,
            });
        }
        if let Some(accept) = clicks.accept {
            self.ops.push(accept);
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

    use pncad::document::{RecipeNodeId, SpokenNode};

    use eframe::egui;

    use super::{
        INDENT_MAX_DEPTH, INDENT_STEP, failure_lines, feature_row_ui, indent, message_indent,
        row_label,
    };
    use crate::app::GLYPH_ROOT;
    use crate::pane::headless::SLACK;
    use crate::pane::headless::{
        assert_under, find, landed, landed_voiced, painted, painted_after_clicking, painted_text,
    };
    use crate::props::Notation;
    use crate::test_support::spoken;
    use crate::theme::Theme;
    use crate::tree;
    use crate::tree::{CarriedLine, Readout, RowStatus, TreeRow};
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
            id: RecipeNodeId(test_utils::refusal::tagged(id)),
            spoken: spoken(
                RecipeNodeId(test_utils::refusal::tagged(id)),
                Some("Datum frame"),
            ),
            pose: Some(pose.to_owned()),
            depth: 0,
            root: false,
            status: RowStatus::Ok,
            note: None,
            repair_at: None,
            readout: None,
            version_offer: None,
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
            id: RecipeNodeId(test_utils::refusal::tagged(1)),
            spoken: spoken(
                RecipeNodeId(test_utils::refusal::tagged(1)),
                Some("Extrude"),
            ),
            pose: None,
            depth: 0,
            root: true,
            status: RowStatus::Ok,
            note: None,
            repair_at: None,
            readout: None,
            version_offer: None,
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
            drawn.contains(&format!(
                "Datum frame 000000000002 — yz at (0, 0, 0) m {GLYPH_ROOT}"
            )),
            "{drawn}"
        );
    }

    /// A mate row refused because its placer (node 3) did not
    /// derive, as `tree::rows` builds one: `Failed`, with the link.
    fn placer_refused_row(repair_at: Option<SpokenNode>) -> TreeRow {
        TreeRow {
            id: RecipeNodeId(test_utils::refusal::tagged(7)),
            spoken: spoken(RecipeNodeId(test_utils::refusal::tagged(7)), Some("Mate")),
            pose: None,
            depth: 0,
            root: false,
            status: RowStatus::Failed {
                message: FAILURE.to_owned(),
                carried: Vec::new(),
            },
            note: None,
            repair_at,
            readout: None,
            version_offer: None,
        }
    }

    /// **A failed row draws each refusal it carries on a line of its
    /// own**, headed by the document it is in, under its own words and
    /// one step further in per level: the traceback a part inside a
    /// part reads as.
    #[test]
    fn a_failed_rows_carried_refusals_draw_under_it_one_step_in_per_level() {
        let carried = [
            ("bracket.pncad", "node 000000000007 failed: the first level"),
            ("boss.pncad", "node 000000000003 failed: the second level"),
        ];
        let row = TreeRow {
            status: RowStatus::Failed {
                message: FAILURE.to_owned(),
                carried: carried
                    .map(|(document, line)| CarriedLine {
                        document: document.to_owned(),
                        line: line.to_owned(),
                    })
                    .to_vec(),
            },
            ..placer_refused_row(None)
        };
        let painted = landed(|ui| {
            failure_lines(ui, &row, &Theme::DEFAULT);
        });
        let at = |text: &str| {
            painted
                .iter()
                .find(|landed| landed.text == text)
                .and_then(|landed| landed.rows.first().copied())
                .unwrap_or_else(|| panic!("{text:?} was not painted"))
        };
        let own = at(FAILURE);
        let (first_doc, first) = (at(carried[0].0), at(carried[0].1));
        let (second_doc, second) = (at(carried[1].0), at(carried[1].1));
        assert!(
            own.bottom() <= first_doc.top()
                && first_doc.bottom() <= first.top()
                && first.bottom() <= second_doc.top()
                && second_doc.bottom() <= second.top(),
            "each level's document heads its line, under the level before:              {own:?} {first_doc:?} {first:?} {second_doc:?} {second:?}"
        );
        assert!(
            own.left() < first.left() && first.left() < second.left(),
            "each level is one step further in: {own:?} {first:?} {second:?}"
        );
        assert!(
            first_doc.left() == first.left() && second_doc.left() == second.left(),
            "a level's document label stands at its line's indent"
        );
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
        let placer = RecipeNodeId(test_utils::refusal::tagged(3));
        let row = placer_refused_row(Some(spoken(placer, Some("Datum frame"))));
        let link = tree::link_wording(spoken(placer, Some("Datum frame")));
        assert_eq!(
            link, "see Datum frame 000000000003",
            "the node as the document speaks it"
        );
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

    /// **A failed row's words are said quietly**: the row is loud once,
    /// at its badge, and the line under it is that verdict's words —
    /// egui's weak text, not the theme's actionable colour.
    #[test]
    fn a_failed_rows_words_are_weak_under_its_loud_badge() {
        let (painted, voices) = landed_voiced(&Theme::DEFAULT, |ui, theme| {
            failure_lines(ui, &placer_refused_row(None), theme);
        });
        assert_eq!(find(&painted, FAILURE).ink, Some(voices.weak));
    }

    /// A poisoned row's pointer is still the click to `through`.
    #[test]
    fn a_poisoned_rows_pointer_selects_the_row_it_names() {
        let through = RecipeNodeId(test_utils::refusal::tagged(7));
        let pointer = tree::downstream_wording(spoken(through, Some("Fillet")));
        let row = TreeRow {
            status: RowStatus::Poisoned {
                through,
                message: Some(pointer.clone()),
            },
            ..placer_refused_row(None)
        };
        assert_eq!(clicking(&row, &pointer), Some(through));
    }

    /// **A box's height, measured three ways over its two caps**, as
    /// `tree::rows` builds the rows off a real evaluation: a
    /// `distance`, which has a value; a `min_clearance`, which has none
    /// at `f64`; and a distance over zero, whose node fails. And
    /// assertions: the height, measured a second time, at least 0.01 m
    /// (holds), at least 0.02 m (violated), and at least itself and 2ε,
    /// in the band between ε and K·ε (indeterminate); the clearance and
    /// the failed measure at least 0.01 m; and the angle at most 1 rad.
    /// The first `distance` is consumed by none, so its row is a root.
    struct MeasureFixture {
        doc: pncad::document::Doc<pncad::document::ProfileProgram>,
        evaluation: pncad::document::Evaluation<f64>,
        distance: RecipeNodeId,
        clearance: RecipeNodeId,
        failed: RecipeNodeId,
        angle: RecipeNodeId,
        holds: RecipeNodeId,
        violated: RecipeNodeId,
        indeterminate: RecipeNodeId,
        unavailable: RecipeNodeId,
        poisoned: RecipeNodeId,
        angle_holds: RecipeNodeId,
    }

    /// The box's height, which the `distance` measure reads back.
    const HEIGHT: f64 = 0.0125;

    fn measure_fixture() -> MeasureFixture {
        use pncad::document::{
            AssertionDir, CancelToken, Doc, EvalOptions, Expr, MeasureExpr, MeasurePrimitive, Node,
            SitedRef, evaluate,
        };
        use pncad::geom_core::Tol;
        use pncad::select::{CapEnd, EntityKind, NamePat, SegPat, SegTag, Selector, select};

        use crate::test_support::{ang, framed_square, inserted, len, scl};

        let tol = Tol::witness();
        let run = |doc: &Doc<_>| {
            evaluate(
                doc,
                None,
                &CancelToken::default(),
                &EvalOptions::default(),
                tol,
            )
        };
        let (doc, profile) = framed_square(&Doc::empty_derived("measure-rows", tol), 0.02, tol);
        let (doc, solid) = inserted(
            &doc,
            Node::Extrude {
                profile,
                distance: len(HEIGHT),
            },
            tol,
        );
        let box_run = run(&doc);
        let cap = |end: CapEnd| {
            let sel = Selector::of(
                NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(end)),
            );
            let found = select(&box_run, solid, &sel);
            assert_eq!(found.len(), 1, "one {end:?} cap: {found:?}");
            SitedRef::new(solid, found[0].clone())
        };
        let caps = vec![cap(CapEnd::Start), cap(CapEnd::End)];
        let across = || MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 });
        let measure = |doc: &Doc<_>, expr: MeasureExpr| {
            inserted(
                doc,
                Node::measure(expr, caps.clone()).expect("both caps are referenced"),
                tol,
            )
        };
        let (doc, distance) = measure(&doc, across());
        let (doc, clearance) = measure(
            &doc,
            MeasureExpr::primitive(MeasurePrimitive::MinClearance { a: 0, b: 1 }),
        );
        let (doc, failed) = measure(
            &doc,
            MeasureExpr::div(across(), MeasureExpr::value(scl(0.0)))
                .expect("a length over a scalar is a length"),
        );
        let (doc, angle) = measure(&doc, MeasureExpr::value(ang(0.5)));
        let assertion = |doc: &Doc<_>, measure: RecipeNodeId, bound: Expr, dir: AssertionDir| {
            inserted(
                doc,
                Node::Assertion {
                    measure,
                    bound,
                    dir,
                },
                tol,
            )
        };
        let (doc, height) = measure(&doc, across());
        let (doc, holds) = assertion(&doc, height, len(0.01), AssertionDir::AtLeast);
        let (doc, violated) = assertion(&doc, height, len(0.02), AssertionDir::AtLeast);
        let (doc, indeterminate) = assertion(
            &doc,
            height,
            len(HEIGHT + 2.0 * tol.eps()),
            AssertionDir::AtLeast,
        );
        let (doc, unavailable) = assertion(&doc, clearance, len(0.01), AssertionDir::AtLeast);
        let (doc, poisoned) = assertion(&doc, failed, len(0.01), AssertionDir::AtLeast);
        let (doc, angle_holds) = assertion(&doc, angle, ang(1.0), AssertionDir::AtMost);
        let evaluation = run(&doc);
        MeasureFixture {
            doc,
            evaluation,
            distance,
            clearance,
            failed,
            angle,
            holds,
            violated,
            indeterminate,
            unavailable,
            poisoned,
            angle_holds,
        }
    }

    impl MeasureFixture {
        /// What `id` evaluated to.
        fn payload(&self, id: RecipeNodeId) -> Option<&pncad::document::ValuePayload<f64>> {
            self.evaluation.usable(id).ok().map(|value| &value.payload)
        }

        fn row(&self, id: RecipeNodeId) -> TreeRow {
            tree::rows(
                &self.doc,
                Some(&self.evaluation),
                &crate::parts::PartFiles::Unscanned,
            )
            .into_iter()
            .find(|row| row.id == id)
            .expect("every node has a row")
        }
    }

    /// What `painted` says, in paint order.
    fn texts(painted: &[crate::pane::headless::Landed]) -> Vec<&str> {
        painted.iter().map(|landed| landed.text.as_str()).collect()
    }

    /// `row`, drawn by the pane's own row function, unselected and not
    /// hidden.
    fn feature_row_drawn(ui: &mut egui::Ui, row: &TreeRow, theme: &Theme) {
        feature_row_drawn_in(ui, row, theme, Notation::DEFAULT);
    }

    /// [`feature_row_drawn`], read in `notation`.
    fn feature_row_drawn_in(ui: &mut egui::Ui, row: &TreeRow, theme: &Theme, notation: Notation) {
        feature_row_ui(ui, row, false, false, theme, notation);
    }

    /// **A measure with a value paints it on its own row**, in the
    /// working notation the chrome writes any computed value in — here
    /// the default's metres — with the unit's symbol.
    ///
    /// Red if `row_result` stops drawing `Readout::Value` or is drawn
    /// off the row's line, or if `readout_of` spells the value any
    /// other way.
    #[test]
    fn a_measure_with_a_value_paints_it_beside_its_row() {
        let fixture = measure_fixture();
        let painted =
            landed(|ui| feature_row_drawn(ui, &fixture.row(fixture.distance), &Theme::DEFAULT));
        let value = find(&painted, "0.0125 m");
        let kind = find(
            &painted,
            &format!(
                "Measure {} {GLYPH_ROOT}",
                test_utils::refusal::tag(fixture.distance.0)
            ),
        );
        assert!(
            (value.rows[0].center().y - kind.rows[0].center().y).abs() <= SLACK
                && value.rows[0].left() > kind.rows[0].right(),
            "the value stands beside the row's kind, on its line: {:?} / {:?}",
            kind.rows,
            value.rows
        );
        assert_eq!(
            painted.len(),
            2,
            "and nothing else is painted: {:?}",
            texts(&painted)
        );
    }

    /// **The value is written in the kind its dimension is**: an
    /// angle in the notation's angle unit, not a length.
    ///
    /// Red if `readout_of` renders every measure as a length.
    #[test]
    fn a_measured_angle_paints_in_the_notations_angle_unit() {
        let fixture = measure_fixture();
        let drawn = painted_text(|ui| {
            feature_row_drawn_in(
                ui,
                &fixture.row(fixture.angle),
                &Theme::DEFAULT,
                Notation::CANONICAL,
            );
        });
        assert!(drawn.contains("0.5 rad"), "{drawn}");
    }

    /// **An empty value says so on its own row, beside its kind**, and
    /// draws no badge: the row is `Ok`.
    ///
    /// Red if `row_result` stops drawing `Readout::Empty`, draws it off
    /// the row's line, or re-spells `tree::Emptiness`'s phrase.
    #[test]
    fn an_empty_value_paints_its_phrase_beside_its_row() {
        use pncad::select::SplitHalf;

        let row = |readout| TreeRow {
            id: RecipeNodeId(test_utils::refusal::tagged(4)),
            spoken: spoken(RecipeNodeId(test_utils::refusal::tagged(4)), Some("Split")),
            pose: None,
            depth: 0,
            root: false,
            status: RowStatus::Ok,
            note: None,
            repair_at: None,
            readout: Some(Readout::Empty(readout)),
            version_offer: None,
        };
        for (emptiness, phrase) in [
            (tree::Emptiness::Whole, "empty"),
            (tree::Emptiness::Half(SplitHalf::Above), "above half empty"),
            (tree::Emptiness::Half(SplitHalf::Below), "below half empty"),
        ] {
            let painted = landed(|ui| feature_row_drawn(ui, &row(emptiness), &Theme::DEFAULT));
            let said = find(&painted, phrase);
            let kind = find(&painted, "Split 000000000004");
            assert!(
                (said.rows[0].center().y - kind.rows[0].center().y).abs() <= SLACK
                    && said.rows[0].left() > kind.rows[0].right(),
                "{phrase:?} stands beside the row's kind, on its line: {:?} / {:?}",
                kind.rows,
                said.rows
            );
            assert_eq!(
                texts(&painted),
                vec!["Split 000000000004", phrase],
                "the kind and the phrase, and no badge"
            );
        }
    }

    /// **A measure with no value at this scalar paints the kernel's
    /// reason under its row, byte for byte**, quietly: the node
    /// evaluated, so there is no badge, and the line is a report.
    ///
    /// Red if `lines_under` drops `Readout::Unavailable`, if the
    /// reason is re-spelled, or if the line is drawn loud.
    #[test]
    fn a_measure_with_no_value_paints_the_kernels_reason_under_its_row() {
        use pncad::document::{MeasureUnavailableAt, ValuePayload};

        let fixture = measure_fixture();
        let unavailable = match fixture.payload(fixture.clearance) {
            Some(ValuePayload::MeasureUnavailable { reason, .. }) => *reason,
            other => panic!("the premise: `min_clearance` has no value at f64: {other:?}"),
        };
        let MeasureUnavailableAt::NeedsEnclosure { door, .. } = unavailable;
        let reason = unavailable.to_string();
        assert!(
            reason.contains(door),
            "the premise: the kernel's words name the door that answers: {reason}"
        );
        let row = fixture.row(fixture.clearance);
        let (painted, voices) = landed_voiced(&Theme::DEFAULT, |ui, theme| {
            feature_row_drawn(ui, &row, theme)
        });
        let line = find(&painted, &reason);
        assert_under(
            find(
                &painted,
                &format!("Measure {}", test_utils::refusal::tag(fixture.clearance.0)),
            ),
            line,
        );
        assert_eq!(line.ink, Some(voices.weak), "said quietly");
        assert_eq!(
            painted.len(),
            2,
            "no badge and no value beside the row: {:?}",
            texts(&painted)
        );
    }

    /// **A measure whose node failed paints its failure and no value**:
    /// the badge and the kernel's words, as any failed row.
    #[test]
    fn a_failed_measure_paints_its_failure_and_no_value() {
        let fixture = measure_fixture();
        let row = fixture.row(fixture.failed);
        let RowStatus::Failed { message, .. } = &row.status else {
            panic!("the premise: a distance over zero fails: {:?}", row.status)
        };
        let drawn = painted(|ui| feature_row_drawn(ui, &row, &Theme::DEFAULT));
        assert_eq!(
            drawn,
            vec![
                format!("Measure {}", test_utils::refusal::tag(fixture.failed.0)),
                "FAILED".to_owned(),
                message.clone()
            ],
            "the kind, the badge, the words, and nothing measured"
        );
    }

    /// **A row's standing note is drawn under it, quietly** — here a
    /// mate whose class has no at-rest record, in the kernel's words.
    ///
    /// Red if `lines_under` drops the note, or draws it loud.
    #[test]
    fn a_mate_rows_standing_note_paints_under_it() {
        use pncad::document::{ClassAdmission, class_admission};
        use pncad::select::ContactClass;

        let admission = class_admission(ContactClass::Tangent);
        assert!(
            matches!(admission, ClassAdmission::NoAtRestRecord { .. }),
            "the premise: a Tangent mate has no at-rest record: {admission:?}"
        );
        let note = admission.no_record_reason();
        let row = TreeRow {
            status: RowStatus::Ok,
            note: Some(note.to_owned()),
            ..placer_refused_row(None)
        };
        let (painted, voices) = landed_voiced(&Theme::DEFAULT, |ui, theme| {
            feature_row_drawn(ui, &row, theme)
        });
        let line = find(&painted, note);
        assert_under(find(&painted, "Mate 000000000007"), line);
        assert_eq!(line.ink, Some(voices.weak), "said quietly");
    }

    /// The verdict `id` landed, from the evaluation itself.
    fn verdict_of(
        fixture: &MeasureFixture,
        id: RecipeNodeId,
    ) -> pncad::document::AssertionVerdict<f64> {
        match fixture.payload(id) {
            Some(pncad::document::ValuePayload::Assertion(verdict)) => verdict.clone(),
            other => panic!("the premise: {id:?} evaluated to a verdict: {other:?}"),
        }
    }

    /// The kernel's word for the state of `id`'s verdict.
    fn state_of(fixture: &MeasureFixture, id: RecipeNodeId) -> &'static str {
        verdict_of(fixture, id).label()
    }

    /// `measured` and `bound` — the chrome's spelling, written out —
    /// with the kernel's relation for the side assertion `id` records.
    fn compared(fixture: &MeasureFixture, id: RecipeNodeId, measured: &str, bound: &str) -> String {
        match fixture.doc.node(id) {
            Some(pncad::document::Node::Assertion { dir, .. }) => {
                format!("{measured} {} {bound}", dir.symbol())
            }
            other => panic!("the premise: {id:?} is an assertion: {other:?}"),
        }
    }

    /// **An assertion that holds paints its verdict and both numbers on
    /// its own line**: the kernel's word, quietly, then the comparison
    /// in the measure's notation with the kernel's relation between.
    ///
    /// Red if `row_result` stops drawing `Readout::Asserted`, draws it
    /// off the row's line, draws a holding verdict loud, or if
    /// `Asserted::comparison` drops a number or the relation.
    #[test]
    fn an_assertion_that_holds_paints_its_verdict_and_both_numbers_beside_its_row() {
        let fixture = measure_fixture();
        assert!(
            matches!(
                verdict_of(&fixture, fixture.holds),
                pncad::document::AssertionVerdict::Holds { .. }
            ),
            "the premise: 0.0125 m is at least 0.01 m"
        );
        let (painted, voices) = landed_voiced(&Theme::DEFAULT, |ui, theme| {
            feature_row_drawn(ui, &fixture.row(fixture.holds), theme)
        });
        let kind = find(
            &painted,
            &format!(
                "Assertion {} {GLYPH_ROOT}",
                test_utils::refusal::tag(fixture.holds.0)
            ),
        );
        let state = find(&painted, state_of(&fixture, fixture.holds));
        let comparison = find(
            &painted,
            &compared(&fixture, fixture.holds, "0.0125 m", "0.01 m"),
        );
        for beside in [state, comparison] {
            assert!(
                (beside.rows[0].center().y - kind.rows[0].center().y).abs() <= SLACK
                    && beside.rows[0].left() > kind.rows[0].right(),
                "`{}` stands beside the row's kind, on its line: {:?} / {:?}",
                beside.text,
                kind.rows,
                beside.rows
            );
        }
        assert_eq!(
            state.ink,
            Some(voices.weak),
            "a holding verdict is a report"
        );
        assert_eq!(
            painted.len(),
            3,
            "and nothing else is painted: {:?}",
            texts(&painted)
        );
    }

    /// **A violated assertion's verdict is drawn loud**, beside both
    /// numbers.
    ///
    /// Red if `Asserted::tone` reads a violated verdict as advisory, or
    /// if the numbers are dropped from a violated row.
    #[test]
    fn a_violated_assertion_paints_its_verdict_loud_beside_both_numbers() {
        let fixture = measure_fixture();
        let state = state_of(&fixture, fixture.violated);
        assert!(
            matches!(
                verdict_of(&fixture, fixture.violated),
                pncad::document::AssertionVerdict::Violated { .. }
            ),
            "the premise: 0.0125 m is not at least 0.02 m"
        );
        let (painted, voices) = landed_voiced(&Theme::DEFAULT, |ui, theme| {
            feature_row_drawn(ui, &fixture.row(fixture.violated), theme)
        });
        assert_eq!(
            texts(&painted),
            vec![
                format!(
                    "Assertion {} {GLYPH_ROOT}",
                    test_utils::refusal::tag(fixture.violated.0)
                )
                .as_str(),
                state,
                &compared(&fixture, fixture.violated, "0.0125 m", "0.02 m"),
            ],
        );
        assert_eq!(find(&painted, state).ink, Some(voices.actionable));
    }

    /// **An assertion's numbers are spelled in its measure's
    /// dimension**, and its relation is the one it records.
    ///
    /// Red if `asserted` spells a verdict's numbers in any dimension
    /// but its measure's, or `comparison` any relation but `dir`'s.
    #[test]
    fn an_assertion_over_an_angle_paints_both_numbers_in_its_angle_unit() {
        let fixture = measure_fixture();
        let drawn = painted(|ui| {
            feature_row_drawn_in(
                ui,
                &fixture.row(fixture.angle_holds),
                &Theme::DEFAULT,
                Notation::CANONICAL,
            );
        });
        let comparison = compared(&fixture, fixture.angle_holds, "0.5 rad", "1 rad");
        assert!(
            drawn.contains(&comparison),
            "`{comparison}` among {drawn:?}"
        );
    }

    /// **One change of the working notation re-spells a driven slot, a
    /// measure's row and an assertion's two numbers together**, from
    /// rows built ONCE: the tree rows carry quantities and are spelled
    /// when drawn, so nothing has to be rebuilt for the new notation to
    /// reach them.
    ///
    /// Red if a measure's row or an assertion's verdict is spelled when
    /// the row is built (it would keep metres), or if any of the three
    /// reads a notation of its own.
    #[test]
    fn a_changed_notation_re_spells_a_driven_slot_a_measure_and_an_assertion_together() {
        use pncad::document::{Dimension, SlotId};

        use crate::props::{SlotDriver, SlotRow, SlotValue, field_text};

        let fixture = measure_fixture();
        let measure = fixture.row(fixture.distance);
        let assertion = fixture.row(fixture.holds);
        let driven = SlotRow {
            slot: SlotId::Distance,
            dimension: Dimension::Length,
            structural: false,
            driver: SlotDriver::Expression { params: Vec::new() },
            value: Ok(SlotValue::Continuous(HEIGHT)),
            unit: None,
            source: Some("height".to_owned()),
        };
        let millimetres = Notation {
            length: pncad::quantity::MM,
            ..Notation::DEFAULT
        };
        for (notation, height, bound) in [
            (Notation::DEFAULT, "0.0125 m", "0.01 m"),
            (millimetres, "12.5 mm", "10 mm"),
        ] {
            let drawn = painted(|ui| {
                feature_row_drawn_in(ui, &measure, &Theme::DEFAULT, notation);
                feature_row_drawn_in(ui, &assertion, &Theme::DEFAULT, notation);
            });
            assert!(
                drawn.iter().any(|text| text == height),
                "the measure reads {height}: {drawn:?}"
            );
            let comparison = compared(&fixture, fixture.holds, height, bound);
            assert!(
                drawn.contains(&comparison),
                "the assertion reads `{comparison}`: {drawn:?}"
            );
            assert_eq!(
                field_text(&driven, notation),
                format!("{} {height}", crate::props::DRIVEN),
                "and the driven slot reads in the same notation"
            );
        }
    }

    /// **An indeterminate assertion paints the kernel's reason under
    /// its row, byte for byte**, quietly, and no numbers.
    ///
    /// Red if `lines_under` drops the reason, re-spells it, or draws it
    /// loud.
    #[test]
    fn an_indeterminate_assertion_paints_the_kernels_reason_under_its_row() {
        use pncad::document::{AssertionVerdict, UnevaluatedReason};

        let fixture = measure_fixture();
        let reason = match verdict_of(&fixture, fixture.indeterminate) {
            AssertionVerdict::Unevaluated {
                reason: reason @ UnevaluatedReason::Indeterminate { .. },
            } => reason.to_string(),
            other => panic!("the premise: a margin of 2ε is in the sliver band: {other:?}"),
        };
        let state = state_of(&fixture, fixture.indeterminate);
        let (painted, voices) = landed_voiced(&Theme::DEFAULT, |ui, theme| {
            feature_row_drawn(ui, &fixture.row(fixture.indeterminate), theme)
        });
        assert_eq!(
            texts(&painted),
            vec![
                format!(
                    "Assertion {} {GLYPH_ROOT}",
                    test_utils::refusal::tag(fixture.indeterminate.0)
                )
                .as_str(),
                state,
                reason.as_str()
            ],
        );
        let line = find(&painted, &reason);
        assert_under(find(&painted, state), line);
        assert_eq!(line.ink, Some(voices.weak), "said quietly");
        assert_eq!(find(&painted, state).ink, Some(voices.weak));
    }

    /// **An assertion whose measure has no value points at the
    /// measure's row** — which draws the kernel's reason — and does not
    /// draw that reason a second time. The pointer selects the measure.
    ///
    /// Red if `lines_under` draws the kernel's `UnevaluatedReason`
    /// `Display` for this arm, drops the pointer, or points elsewhere.
    #[test]
    fn an_assertion_whose_measure_has_no_value_points_at_the_measures_row() {
        use pncad::document::{AssertionVerdict, UnevaluatedReason};

        let fixture = measure_fixture();
        let unavailable = match verdict_of(&fixture, fixture.unavailable) {
            AssertionVerdict::Unevaluated {
                reason: UnevaluatedReason::MeasureUnavailable(unavailable),
            } => unavailable.to_string(),
            other => panic!("the premise: `min_clearance` has no value at f64: {other:?}"),
        };
        let row = fixture.row(fixture.unavailable);
        let pointer = format!(
            "see Measure {}",
            test_utils::refusal::tag(fixture.clearance.0)
        );
        let drawn = painted(|ui| feature_row_drawn(ui, &row, &Theme::DEFAULT));
        assert_eq!(
            drawn,
            vec![
                format!(
                    "Assertion {} {GLYPH_ROOT}",
                    test_utils::refusal::tag(fixture.unavailable.0)
                ),
                state_of(&fixture, fixture.unavailable).to_owned(),
                pointer.clone()
            ],
        );
        assert!(
            drawn.iter().all(|text| !text.contains(&unavailable)),
            "the measure's row owns the kernel's reason: {drawn:?}"
        );
        assert_eq!(
            row_clicked(&row, &pointer, false).select,
            Some(fixture.clearance)
        );
    }

    /// **An assertion over a failed measure is poisoned**: the badge
    /// and the pointer at the measure's row, and no verdict.
    #[test]
    fn an_assertion_over_a_failed_measure_paints_its_pointer_and_no_verdict() {
        let fixture = measure_fixture();
        let row = fixture.row(fixture.poisoned);
        assert!(
            matches!(row.status, RowStatus::Poisoned { through, .. } if through == fixture.failed),
            "the premise: a failed measure poisons its assertion: {:?}",
            row.status
        );
        let drawn = painted(|ui| feature_row_drawn(ui, &row, &Theme::DEFAULT));
        assert_eq!(
            drawn,
            vec![
                format!(
                    "Assertion {} {GLYPH_ROOT}",
                    test_utils::refusal::tag(fixture.poisoned.0)
                ),
                "POISONED".to_owned(),
                format!(
                    "upstream failure at Measure {} — that row carries the cause",
                    test_utils::refusal::tag(fixture.failed.0)
                )
            ],
        );
    }

    /// **What `feature_row_ui` answers once the text `target` has been
    /// clicked**, on a row the display does or does not hide.
    fn row_clicked(row: &TreeRow, target: &str, hidden: bool) -> super::RowClicks {
        let select = core::cell::Cell::new(None);
        let hide = core::cell::Cell::new(None);
        painted_after_clicking(target, |ui| {
            let clicks = feature_row_ui(ui, row, false, hidden, &Theme::DEFAULT, Notation::DEFAULT);
            select.set(select.get().or(clicks.select));
            hide.set(hide.get().or(clicks.hide));
        });
        super::RowClicks {
            select: select.get(),
            hide: hide.get(),
            accept: None,
        }
    }

    /// An instance row, as `tree::rows` builds one.
    fn instance_row() -> TreeRow {
        TreeRow {
            id: RecipeNodeId(test_utils::refusal::tagged(4)),
            spoken: spoken(
                RecipeNodeId(test_utils::refusal::tagged(4)),
                Some("InstantiatePart"),
            ),
            pose: Some(crate::test_support::PART_FILE.to_owned()),
            depth: 0,
            root: false,
            status: RowStatus::Ok,
            note: None,
            repair_at: None,
            readout: None,
            version_offer: None,
        }
    }

    /// **A click on a row's label selects that row's node.**
    ///
    /// Red if the label click stops setting `select`.
    #[test]
    fn clicking_a_rows_label_selects_its_node() {
        let row = instance_row();
        let label = format!(
            "InstantiatePart 000000000004 — {}",
            crate::test_support::PART_FILE
        );
        let clicks = row_clicked(&row, &label, false);
        assert_eq!(clicks.select, Some(row.id));
        assert_eq!(clicks.hide, None, "a label click toggles nothing");
    }

    /// **An instance row's toggle asks for the opposite of what the
    /// display shows**, both ways round, and selects nothing.
    ///
    /// Red if a toggle click sets no `hide`, or sets the state the row
    /// already has.
    #[test]
    fn clicking_an_instance_rows_toggle_flips_whether_it_is_hidden() {
        let row = instance_row();
        for hidden in [false, true] {
            let clicks = row_clicked(&row, "shown", hidden);
            assert_eq!(clicks.hide, Some(!hidden), "drawn hidden: {hidden}");
            assert_eq!(clicks.select, None, "the toggle selects nothing");
        }
    }

    /// **Only an instance row has a toggle.**
    ///
    /// Red if the toggle is drawn on every row.
    #[test]
    fn a_row_that_is_no_instance_draws_no_toggle() {
        let row = TreeRow {
            spoken: spoken(
                RecipeNodeId(test_utils::refusal::tagged(4)),
                Some("Extrude"),
            ),
            pose: None,
            ..instance_row()
        };
        let drawn = painted(|ui| feature_row_drawn(ui, &row, &Theme::DEFAULT));
        assert_eq!(drawn, vec!["Extrude 000000000004".to_owned()], "{drawn:?}");
        let with = painted(|ui| feature_row_drawn(ui, &instance_row(), &Theme::DEFAULT));
        assert!(
            with.iter().any(|text| text == "shown"),
            "the premise: an instance row draws one: {with:?}"
        );
    }

    /// **An instance row whose pin no longer holds draws the accept its
    /// failure offers, and the button is that offer's operation.**
    ///
    /// The row is assembled here with the two fields `tree::rows` fills
    /// from such a failure: its words, and the offer the real
    /// `frame::version_offer` reads off it. The session's own rows are
    /// `tests/instance_authoring.rs`'s. Red if
    /// the question or the button is not drawn, if clicking the button
    /// answers anything but the offer's op or also selects, or if a row
    /// with no offer draws either.
    #[test]
    fn a_pin_mismatched_instance_row_draws_the_accept_and_its_button_is_the_offer() {
        use pncad::document::{PartFault, ResolveFault};

        use crate::session::{Refusal, SessionOp, VersionOffer};
        use crate::test_support::part_refused;

        let (kind, files) = part_refused(PartFault::Unresolved {
            fault: ResolveFault::PinMismatch,
            message: "the store's own words".to_owned(),
        });
        let offer = crate::frame::version_offer(&kind, &files).expect("a pin mismatch offers");
        let row = TreeRow {
            status: RowStatus::Failed {
                message: kind.to_string(),
                carried: Vec::new(),
            },
            version_offer: Some(offer.clone()),
            ..instance_row()
        };

        let drawn = painted_text(|ui| feature_row_drawn(ui, &row, &Theme::DEFAULT));
        assert!(
            drawn.contains(&Refusal::version_question(&offer))
                && drawn.contains(VersionOffer::LABEL),
            "{drawn}"
        );

        let clicked = core::cell::RefCell::new(None);
        let mut selected = None;
        painted_after_clicking(VersionOffer::LABEL, |ui| {
            let clicks = feature_row_ui(ui, &row, false, false, &Theme::DEFAULT, Notation::DEFAULT);
            selected = selected.or(clicks.select);
            if let Some(op) = clicks.accept {
                *clicked.borrow_mut() = Some(op);
            }
        });
        let clicked = clicked.into_inner();
        assert!(
            matches!(
                (&clicked, offer.accept()),
                (
                    Some(SessionOp::AcceptPartVersion { id }),
                    SessionOp::AcceptPartVersion { id: offered },
                ) if *id == offered
            ),
            "{clicked:?}"
        );
        assert_eq!(selected, None, "the button selects nothing");

        let unoffered = TreeRow {
            version_offer: None,
            ..row
        };
        let drawn = painted_text(|ui| feature_row_drawn(ui, &unoffered, &Theme::DEFAULT));
        assert!(
            !drawn.contains(VersionOffer::LABEL)
                && !drawn.contains(&Refusal::version_question(&offer)),
            "{drawn}"
        );
    }
}
