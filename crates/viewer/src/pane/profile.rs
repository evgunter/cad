//! **The profile editor**: the one step-list editor both of a
//! profile's doors draw — the add-profile form over its own draft
//! (`pane::create`), and the same editor opened on a committed profile
//! from the Properties pane ([`ViewerBehavior::edit_profile_ui`]).
//!
//! The step list ([`path_steps_ui`]), the notation row
//! ([`notation_row`]) and the preview's verdict ([`preview_verdict`])
//! are written once here, which is what makes "the same interface" a
//! fact about the code rather than an agreement between two copies.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::document::RecipeNodeId;
use pncad::geom_core::Tol;
use pncad::profile::{Step, TipState, Verb};
use pncad::quantity::{AngleUnit, LengthUnit, UnitDef};

use crate::app::{GLYPH_DOWN, GLYPH_REMOVE, GLYPH_UP, ViewerBehavior};
use crate::drafts::{ProfileEdit, RowEdit};
use crate::frame;
use crate::props::Notation;
use crate::session::{DocSession, SessionOp};
use crate::sketch::{self, PreviewError, ProfilePreview};
use crate::theme::Theme;
use crate::widgets::{angle_picker, length_picker, new_row_step, path_step_fields};

impl ViewerBehavior<'_> {
    /// **The add-profile form's editor, opened on a committed
    /// profile** — the edit door. The step list, its fields, its
    /// pickers and the preview under it are the create door's own
    /// ([`path_steps_ui`], [`notation_row`], [`preview_verdict`]),
    /// held in the same currency ([`crate::drafts::ProfileEdit`]), so
    /// the two doors are one editor by construction rather than two
    /// that agree. What differs is what the editor is opened ON and
    /// what it commits as: the node's program, loaded by
    /// [`crate::sketch::held_loops`], committed whole by
    /// [`SessionOp::EditProfile`] ([`edit_door_ui`]).
    ///
    /// Returns `false`, having said why in the actionable colour, when
    /// the editor cannot hold this profile (an argument an expression
    /// drives): the caller shows the slot rows instead, which can. When
    /// it returns `true` the caller still offers the slot rows, folded
    /// beneath the editor — they are where an argument is driven by an
    /// expression, re-noted in a unit, or probed for its range, which
    /// the editor's number fields do not do.
    pub(crate) fn edit_profile_ui(&mut self, ui: &mut egui::Ui, node: RecipeNodeId) -> bool {
        let session = self.session;
        let doc = session.committed_doc();
        if let Err(refusal) = self.drafts.profile_edit(doc, node) {
            crate::widgets::message_toned(
                ui,
                format!("the profile editor cannot hold this profile: {refusal}"),
                &self.theme,
                frame::Tone::Actionable,
            );
            return false;
        }
        self.profile_drawn.edit = true;
        notation_row(
            ui,
            "edit_path",
            &mut self.notation.length,
            &mut self.notation.angle,
        );
        let written = (
            (self.notation.length.def(), self.notation.angle.def()),
            *self.notation,
        );
        let Some(edit) = self.drafts.profile_edit.as_mut() else {
            unreachable!("`Drafts::profile_edit` answered Ok, so the draft is held")
        };
        match edit_door_ui(
            ui,
            session,
            self.theme,
            written,
            edit,
            self.profile_previews.edit.as_ref(),
        ) {
            Ok(Some(op)) => self.ops.push(op),
            Ok(None) => {}
            Err(notice) => self.notices.push(notice),
        }
        true
    }
}

/// **The edit door's body, once its draft is held**: one step list
/// per loop, the preview's verdict, and Apply and Revert — a free
/// function over the `Ui`, so a row can drive the door a person uses
/// rather than a copy of it.
///
/// Every control of the create door is live here: a committed
/// profile's steps are inserted, removed, reordered and re-verbed, and
/// its arcs, targets and split counts changed, exactly as a new one's
/// are, and Apply commits the whole program as ONE
/// [`SessionOp::EditProfile`] saying which committed step each held
/// step is ([`ProfileEdit::ids`]). A name on a step the program drops
/// is stranded; Apply says how many before it is clicked
/// ([`apply_and_revert`], from [`DocSession::edit_profile_report`]),
/// and the op's outcome reports each after.
///
/// Answers the op Apply formed, or the notice for what could not form
/// one (a field that does not lower, a revert that could not reload).
pub(crate) fn edit_door_ui(
    ui: &mut egui::Ui,
    session: &DocSession,
    theme: Theme,
    (units, notation): ((UnitDef, UnitDef), Notation),
    edit: &mut ProfileEdit,
    preview: Option<&Result<ProfilePreview, PreviewError>>,
) -> Result<Option<SessionOp>, frame::Message> {
    let node = edit.node;
    ui.label(format!("profile on frame {}", edit.plane().0));
    let loops = edit.loops().len();
    let mut rows = Vec::new();
    for index in 0..loops {
        // A loop label only where there is more than one: "loop 0"
        // over the only loop is a number with nothing to tell apart.
        if loops > 1 {
            ui.label(format!("loop {index}"));
        }
        let salt = format!("edit_{}_{index}", node.0);
        if let Some(row) = path_steps_ui(ui, &salt, session.tol(), units, edit.steps_mut(index)) {
            rows.push((index, row));
        }
    }
    for (index, row) in rows {
        edit.edit_row(index, row);
    }
    let refused = preview_verdict(ui, theme, preview);
    let moved = edit.moved();
    let stranded = if moved {
        let base = edit.base().clone();
        edit.report(session.history().current(), |loops, ids| {
            // A program the door refuses strands nothing yet; its
            // refusal is the preview's to show, or Apply's to get.
            session
                .edit_profile_report(node, &base, loops, ids)
                .unwrap_or_default()
        })
        .iter()
        .filter_map(frame::maintenance_notice)
        .map(|notice| notice.text().to_owned())
        .collect()
    } else {
        Vec::new()
    };
    let (apply, revert) = ui
        .horizontal(|ui| apply_and_revert(ui, moved, refused, &stranded))
        .inner;
    if apply {
        return match edit.programs(notation) {
            Ok(loops) => Ok(Some(SessionOp::EditProfile {
                node,
                base: edit.base().clone(),
                loops,
                ids: edit.ids(),
            })),
            Err(error) => Err(frame::tool_news(
                format!("edit profile: {error}"),
                frame::Retold::Again,
            )),
        };
    }
    if revert && let Err(error) = edit.revert(session.committed_doc()) {
        return Err(frame::tool_news(
            format!("revert profile: {error}"),
            frame::Retold::Again,
        ));
    }
    Ok(None)
}

/// **What the editor's preview says about its loops**, said under the
/// step list — and whether it holds the commit.
///
/// One reading for both of the editor's doors: the add-profile form
/// and the same editor opened on a committed profile show the same
/// sentences for the same loops, because the preview ran the commit
/// door's own ladder and a refusal here is the refusal the button
/// would get. `None` is "no preview was taken" (the first frame a
/// form is on screen, or a form at rest), and holds nothing.
///
/// Every verdict is a sentence the VALUE says and a tone the value
/// states, both read off one partition of it ([`ProfilePreview::hold`]
/// for a drawn preview, [`PreviewError`] for a refusal) and drawn
/// through [`crate::widgets::message_toned`] — so what the editor says
/// and how loud it says it are decided once, on the value, for both
/// doors. A drawn, valid preview holds nothing and has no verdict; the
/// loop count under it is state, and stays a weak label.
pub(crate) fn preview_verdict(
    ui: &mut egui::Ui,
    theme: Theme,
    preview: Option<&Result<ProfilePreview, PreviewError>>,
) -> bool {
    // The first frame this form is on screen: the latch has not asked
    // for a preview yet, so there is nothing honest to say about one.
    // The commit door is still the judge, so the button is not withheld
    // for a frame either.
    let Some(preview) = preview else {
        return false;
    };
    let (sentence, tone) = match preview {
        Ok(drawn) => match drawn.hold() {
            Some(hold) => (hold.to_string(), hold.tone()),
            None => {
                ui.weak(format!(
                    "{} loop(s), drawn in the viewport",
                    drawn.loops.len()
                ));
                return false;
            }
        },
        Err(error) => (error.to_string(), error.tone()),
    };
    crate::widgets::message_toned(ui, sentence, &theme, tone);
    true
}

/// **The notation row**: the two pickers every length and angle field
/// of a path editor is written in.
pub(crate) fn notation_row(
    ui: &mut egui::Ui,
    salt: &str,
    length_unit: &mut LengthUnit,
    angle_unit: &mut AngleUnit,
) {
    ui.horizontal(|ui| {
        ui.weak("written in");
        length_picker(ui, &format!("{salt}_length"), length_unit);
        angle_picker(ui, &format!("{salt}_angle"), angle_unit);
    });
}

/// **The path editor's step list**: one row per verb, plus the
/// control that appends another — the ONE list both doors of the
/// profile editor draw: the add-profile form over its own draft, and
/// the same form opened on a committed profile, one list per loop.
///
/// **The list's shape is the caller's to change.** Fields write their
/// numbers in place; a row inserted, removed, moved or re-verbed, or
/// the list cleared, is answered as a [`RowEdit`] for the caller to
/// apply — the edit door carries which committed step each row is
/// through it ([`ProfileEdit::edit_row`]). One per frame: two buttons
/// clicked in one frame do not compound into a move nobody asked for.
///
/// **Drawing it never writes a value.** A field writes back only on
/// its own edit, and a bounded field does not pull a document value it
/// was handed into its authoring range (`widgets::path_step_fields`'s
/// split-circle count).
///
/// Each row is `N [x] [up] [down] <verb> <the verb's fields>` — a
/// list a person edits in place, because a chain IS a list and its
/// order is the whole content. Changing a row's verb replaces the
/// step with a fresh one of that verb rather than carrying
/// numbers across: two verbs' fields mean different things (a
/// `line`'s length is not a `turn`'s angle), so a carried number
/// would be a guess the form cannot check.
///
/// **The row number is the one the refusals use.** A preview
/// refusal reads "loop 0 step 2", and a list with nothing written
/// on it left a reader counting rows to find which one that was.
/// It is therefore zero-based, matching the sentence rather than
/// matching what a list of things usually looks like.
///
/// **The verb combo offers only what the lattice admits at that
/// tip**, and shows the rest greyed with the tip's state as their
/// hover text. The tip comes from replaying the rows before it
/// ([`sketch::tip_state_at`]); what it admits is read off the
/// kernel's own transition table ([`sketch::admits_at`]) and, for
/// an arc spec, off its dispatcher's forms — both projected from
/// the declarations the replay runs, so the form keeps no copy of
/// the lattice. A verb picked lands in the first arc form its row
/// takes ([`sketch::fresh_step_at`]).
#[must_use = "a row edit the caller drops is a click that did nothing"]
pub(crate) fn path_steps_ui(
    ui: &mut egui::Ui,
    salt: &str,
    tol: Tol,
    (length_unit, angle_unit): (UnitDef, UnitDef),
    steps: &mut [Step<f64>],
) -> Option<RowEdit> {
    // The row edits are COLLECTED during the loop and answered after
    // it: a list cannot be reordered or shortened while it is being
    // iterated.
    let mut remove: Option<usize> = None;
    let mut swap: Option<(usize, usize)> = None;
    // A verb chosen in a row's combo, applied after the loop for
    // the same reason the moves are: the probe that decides which
    // verbs a combo may offer reads the WHOLE list, and it cannot
    // borrow it while a row holds a mutable slice of it.
    let mut reverb: Option<(usize, Verb, Option<TipState>)> = None;
    let mut insert: Option<usize> = None;
    // The tip each row's step lands on, read off the replay of the
    // rows before it. Every frame rather than only while a combo is
    // open, because the arc pickers grey their refused modes from
    // it too; a replay per row of a hand-authored path is cheap.
    let states: Vec<Option<TipState>> = (0..steps.len())
        .map(|index| sketch::tip_state_at(steps, index, tol))
        .collect();
    let last = steps.len().saturating_sub(1);
    for (index, &state) in states.iter().enumerate() {
        let row_salt = format!("{salt}_step_{index}");
        ui.horizontal(|ui| {
            // Zero-based, because "loop 0 step 2" is.
            ui.weak(format!("{index}"));
            if step_control(ui, GLYPH_REMOVE, "remove this step", None) {
                remove = Some(index);
            }
            let earlier_reason = (index == 0).then_some(STEP_IS_FIRST);
            if step_control(ui, GLYPH_UP, "move this step earlier", earlier_reason) {
                swap = Some((index, index - 1));
            }
            let later_reason = (index == last).then_some(STEP_IS_LAST);
            if step_control(ui, GLYPH_DOWN, "move this step later", later_reason) {
                swap = Some((index, index + 1));
            }
            // **Insert after this row.** A chain is written in the
            // middle as often as at the end, so every row carries
            // the control, and the last row's is the append. It sits
            // in the row's own control cluster: a row's width is its
            // verb's, so at the far end the `+` would move from row
            // to row and, on the widest, sit past the edge of a pane
            // that does not scroll sideways.
            if step_control(ui, "+", "insert a step after this one", None) {
                insert = Some(index + 1);
            }
            let verb = steps[index].verb();
            egui::ComboBox::from_id_salt((salt, "verb", index))
                .selected_text(verb.to_string())
                .width(120.0)
                .show_ui(ui, |ui| {
                    for &option in Verb::ALL {
                        let refusal = sketch::admits_at(state, option).err();
                        // `add_enabled` on the widget itself, not
                        // an `add_enabled_ui` around it: the
                        // reason a choice is greyed out is told
                        // through `on_disabled_hover_text`, and
                        // that is a `Response`'s door — a region's
                        // response shows nothing.
                        let row = ui.add_enabled(
                            refusal.is_none(),
                            egui::Button::selectable(option == verb, option.to_string()),
                        );
                        match refusal {
                            Some(state) => {
                                // The verb's `Display`, which is
                                // the word the combo shows, not
                                // its `Debug`: the sentence is
                                // about the row a reader is
                                // looking at.
                                row.on_disabled_hover_text(format!(
                                    "{option} is not well-typed here — the tip is {}",
                                    sketch::tip_state_words(state),
                                ));
                            }
                            None if row.clicked() && option != verb => {
                                reverb = Some((index, option, state));
                            }
                            None => {}
                        }
                    }
                });
            let step = &mut steps[index];
            path_step_fields(ui, &row_salt, length_unit, angle_unit, state, step);
        });
    }
    let whole = ui
        .horizontal(|ui| {
            // **"Add step" only when there is no row to insert after.**
            // Once the list has rows, every one of them carries a `+`
            // that inserts after it — including the last, which is the
            // append — so a second control at the bottom would be the
            // same move spelled twice.
            if steps.is_empty() {
                ui.button("Add step").clicked().then(|| RowEdit::Insert {
                    at: 0,
                    step: new_row_step(0),
                })
            } else {
                ui.button("Clear").clicked().then_some(RowEdit::Clear)
            }
        })
        .inner;
    let replace = reverb.map(|(index, verb, state)| RowEdit::Replace {
        index,
        step: sketch::fresh_step_at(verb, state),
    });
    let insert = insert.map(|at| RowEdit::Insert {
        at,
        step: new_row_step(at),
    });
    replace
        .or(remove.map(RowEdit::Remove))
        .or(insert)
        .or(swap.map(|(from, to)| RowEdit::Swap(from, to)))
        .or(whole)
}

/// Why a row's move-earlier control is not live.
const STEP_IS_FIRST: &str = "it is already the first step";

/// Why a row's move-later control is not live.
const STEP_IS_LAST: &str = "it is already the last step";

/// **One of a step row's glyph controls**, live unless `blocked`
/// names why not.
///
/// The glyph is the control's only label, so `action` — what a click
/// does — is its hover in both states, and a blocked control adds the
/// reason on the line under it. Every reason is a draft gate's (a row
/// with nothing past it to move over): no
/// operation is formed to be refused, so the words are the caller's.
/// Each state's words ride the hook egui shows in that state.
///
/// Answers whether it was clicked, which a blocked control never is.
fn step_control(ui: &mut egui::Ui, glyph: &str, action: &str, blocked: Option<&str>) -> bool {
    let button = ui.add_enabled(blocked.is_none(), egui::Button::new(glyph).small());
    match blocked {
        None => button.on_hover_text(action).clicked(),
        Some(why) => button
            .on_disabled_hover_text(format!("{action}\n{why}"))
            .clicked(),
    }
}

/// Why the edit door's Apply and Revert are not live on an untouched
/// draft.
const UNTOUCHED: &str = "the steps and numbers are the committed profile's";

/// **The edit door's Apply and Revert**, both live only once the held
/// program has `moved` off the committed profile's; Apply also waits
/// on the preview not having `refused`.
///
/// **Apply only what moved.** Untouched, there is nothing to write,
/// and the door itself also writes nothing for an untouched program,
/// so the button and the door agree rather than one trusting the
/// other. That is a draft gate on both buttons — no operation is
/// formed to be refused — so each says [`UNTOUCHED`] on its disabled
/// hover. A refused preview has its sentence already, drawn under the
/// step list by [`preview_verdict`], so Apply adds none for it.
///
/// **Apply says what it strands before it is clicked** — a fillet on a
/// dropped step's edge is the fillet the person is about to lose — so
/// the count is on the button, as a delete's cascade is, and each
/// `stranded` row's own sentence ([`frame::maintenance_notice`]) is on
/// its hover.
///
/// Answers whether each was clicked — Apply, then Revert — which a
/// disabled button never is.
fn apply_and_revert(
    ui: &mut egui::Ui,
    moved: bool,
    refused: bool,
    stranded: &[String],
) -> (bool, bool) {
    let label = match stranded.len() {
        0 => "Apply".to_owned(),
        1 => "Apply, stranding 1 name".to_owned(),
        n => format!("Apply, stranding {n} names"),
    };
    let apply = ui.add_enabled(moved && !refused, egui::Button::new(label));
    let apply = if !moved {
        apply.on_disabled_hover_text(format!("nothing to apply: {UNTOUCHED}"))
    } else if stranded.is_empty() {
        apply
    } else {
        let said = stranded.join("\n");
        apply.on_hover_text(&said).on_disabled_hover_text(said)
    };
    let revert = ui.add_enabled(moved, egui::Button::new("Revert"));
    let revert = if moved {
        revert.on_hover_text("put the steps and numbers back to the committed profile's")
    } else {
        revert.on_disabled_hover_text(format!("nothing to revert: {UNTOUCHED}"))
    };
    (apply.clicked(), revert.clicked())
}

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use eframe::egui;
    use pncad::document::{Doc, Node, ProfileProgram};
    use pncad::geom_core::{Point2, Tol};
    use pncad::profile::{PathErrorKind, ProfileError, SketchPlane, Step, Target, TipState, Verb};

    use super::preview_verdict;
    use crate::app::{GLYPH_DOWN, GLYPH_REMOVE, GLYPH_UP};
    use crate::drafts::Drafts;
    use crate::pane::headless::{
        Landed, Voices, find, find_opening, landed_voiced, painted_while_hovering,
    };
    use crate::props::Notation;
    use crate::sketch::{
        self, Cut, LoopEnd, PreviewError, PreviewHold, PreviewLoop, ProfilePreview, ProfileShape,
    };
    use crate::test_support::{self, inserted, line_to, try_inserted, two_legs, xy_frame};
    use crate::theme::Theme;

    /// **Drawing the editor never rewrites a document value.** A
    /// committed `circle_split` above the form's count cap (the
    /// document admits any count) is loaded into the edit door and
    /// DRAWN once, untouched: its count is the committed one, and
    /// nothing reads as moved.
    #[test]
    fn drawing_a_split_circle_above_the_cap_leaves_it_alone() {
        use crate::forms::MAX_CIRCLE_SPLIT;
        let (doc, plane) = inserted(
            &Doc::empty_derived("probe", Tol::witness()),
            xy_frame(),
            Tol::witness(),
        );
        let n = MAX_CIRCLE_SPLIT + 1;
        // **The figure's scale is the run's ε times a constant, and
        // that is forced.** A circle split n ways is conditioned
        // purely relatively: the validator reads the sagitta
        // s = r·(1 − cos(π/n)) to call each piece an arc rather than a
        // chord, and reads the carrier-identity margin — nought in the
        // reals, in floating point the residue of rebuilding each
        // arc's centre from a chord of length 2r·sin(π/n) — to call two
        // pieces one circle. Both are proportional to r and both are
        // read against the band (ε, K·ε), so the radii that never
        // escalate are an interval in ε, and both its walls are walls
        // in r/ε:
        //
        //   r > K·ε/(1 − cos(π/n)) — closed form; at n = 1025, K = 10
        //     that is 2.129e6·ε. Below it the sagitta is in band;
        //     below ε it reads as a chord, and the chord ladder
        //     escalates instead (the ladder paragraph below).
        //   r < ~1.5e12·ε at n = 1025 — MEASURED, not closed form:
        //     the f64 identity residue reaches ε there and "same
        //     carrier" stops being decidable. **This wall carries n
        //     even though its spelling does not**, and it falls as
        //     ~1/n: measured at ε = 1e-9, centre origin, n = 256 →
        //     1.08e13·ε, 512 → 5.07e12, 1024 → 2.57e12, 2048 →
        //     1.24e12, 4096 → 6.17e11, with odd counts paying about
        //     another 1.7× (n = 1025 reads 1.49e12, not 2.6e12).
        //     Raising MAX_CIRCLE_SPLIT moves BOTH walls — the lower
        //     up as n², this one down as 1/n.
        //
        // **No constant radius sits in that window, and only just
        // not.** At its narrowest it is 5.84 decades wide at n = 1025
        // (2.129e6·ε to 1.467e12·ε, the lowest upper wall measured
        // over ε ∈ [1e-13, 1e-5]); a constant across the gated
        // ε = 1e-6 … 1e-12 needs 6.00. ε = 1e-6 wants r > 2.129 m and ε = 1e-12 wants
        // r < 1.467 m: disjoint by a factor of 1.45. The miss hangs
        // entirely on the SOFTER wall — the lower one is exact and
        // ε-independent (measured = closed form to five digits at five
        // ε), the upper is an empirical residue that moves a few per
        // cent with ε and 1.7× with the parity of n. Had it come out
        // at 3.2e12·ε instead, a constant radius would have worked.
        //
        // **1e8 is chosen for the model envelope, not for numeric
        // symmetry**, and the trade is worth naming. The window's
        // geometric centre is 1.77e9, which stands ~700× clear of both
        // walls but makes this fixture a 1.5 km circle at ε = 1e-6 and
        // a 15 km one at the 1e-5 the sweep reaches — outside D4 ¶1's
        // ratified micron-to-kilometre coverage. 1e8·ε holds the
        // figure inside that envelope at every ε the suite runs at —
        // 1 km at 1e-5, the envelope's very top, down to 10 µm at
        // 1e-13, and 0.1 m at the default — and it
        // spends margin on the EXACT wall — 47× clear — to buy ~1.5e4×
        // on the empirical one, which is the wall the paragraph above
        // rests on. An ε-scaled fixture literal of this MAGNITUDE is
        // ordinary house style — `crates/sweep/tests` spells figure
        // scales `1.0e9 * eps` and `1.0e12 * eps` — but in those the
        // multiplier is the figure's size in metres at the default ε
        // and reads off the line, and in the small multiples (0.5,
        // 3.0, 100.0) the multiplier IS the margin. Here it is a
        // window placement, which reads off nothing, and that is why
        // the derivation above it is this long.
        //
        // **The chord ladder closes the "pick another rung" escape,
        // for any regular polygon and not just this one.** Where the
        // sagitta falls below ε the pieces read as chords, and the
        // `chord_side` ladder over the polygon's own vertices is
        // dⱼ = r(cos(π/n) − cos((2j+1)π/n)): it starts at d₁ = 8s
        // (exactly 8 in the small-angle limit, 7.99994 at n = 1025),
        // climbs with consecutive ratio (j + 2)/j ≤ 3, and tops out
        // near 2r. A bottom under the band and a top over it therefore
        // force some rung INTO (ε, K·ε), because no ≤3× step clears a
        // factor-K window. Two of those inequalities are facts about
        // the ratified K = 10 and not about the structure —
        // `CAD_AMBIGUITY_K` admits any finite K > 1, and at K ≤ 8 the
        // "nearest rung is 8s, still under K·ε" step fails outright
        // while at K ≤ 3 the ladder can step over the band. Below
        // r ≈ 1.6e3·ε the chord 2r·sin(π/n) is itself in band or under
        // ε and `vertex_separation` answers before the ladder does.
        //
        // **What guards this.** The claim the literal makes — that the
        // radius clears both walls — is guarded by the row itself: a
        // multiplier under the lower wall or over the upper one reds
        // this test at every ε it runs. Nothing computes with the
        // clearance FIGURES above, and no register re-measures them;
        // they are unguardable in that narrow sense and that is the
        // whole of the debt. One consequence to carry: this is the
        // viewer's only ε-relative fixture literal, so it is invisible
        // to the instrument that catches an absolutely-scaled fixture
        // drifting towards a wall — running the population at
        // neighbouring ε — and its own expect is what catches it
        // instead.
        let radius = 1e8 * Tol::witness().get().eps;
        let loops = vec![
            sketch::loop_program(
                &crate::session::ProfileShape::Path {
                    steps: vec![Step::CircleSplit {
                        centre: Point2::origin(),
                        radius,
                        n,
                        phase: 0.0,
                    }],
                },
                Notation::CANONICAL,
            )
            .expect("finite"),
        ];
        let node = Node::Profile(ProfileProgram {
            plane,
            loops,
            ids: Vec::new(),
        });
        let (doc, profile) = try_inserted(&doc, node, Tol::witness())
            .expect("the document admits a split circle above the form's cap");
        let mut drafts = Drafts::default();
        let edit = drafts.profile_edit(&doc, profile).expect("held");
        assert!(!edit.moved(), "fresh load");
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            let row = super::path_steps_ui(
                ui,
                "probe",
                Tol::witness(),
                (pncad::quantity::M.def(), pncad::quantity::RAD.def()),
                edit.steps_mut(0),
            );
            assert!(row.is_none(), "nothing was clicked");
        });
        output.textures_delta.clear();
        let held_n = match edit.loops()[0][0] {
            Step::CircleSplit { n, .. } => n,
            _ => panic!("a split circle"),
        };
        assert_eq!(held_n, n, "drawing the editor rewrote the count");
        assert!(!edit.moved(), "drawing alone made Apply live");
    }

    /// What a pointer resting on the `nth` painting of `glyph` reads,
    /// over a list of `rows` fresh steps.
    fn hovering_step_control(rows: usize, glyph: &str, nth: usize) -> String {
        let mut steps: Vec<Step<f64>> = (0..rows).map(crate::widgets::new_row_step).collect();
        painted_while_hovering(glyph, nth, |ui| {
            let _hovered_only = super::path_steps_ui(
                ui,
                "probe",
                Tol::witness(),
                (pncad::quantity::M.def(), pncad::quantity::RAD.def()),
                &mut steps,
            );
        })
    }

    /// **A list's end rows say which end they are at** — a lone
    /// row is both, and neither arrow has a row to move past.
    #[test]
    fn a_lone_rows_arrows_say_it_is_first_and_last() {
        let up = hovering_step_control(1, GLYPH_UP, 0);
        assert!(
            up.contains("move this step earlier\nit is already the first step"),
            "{up}"
        );
        let down = hovering_step_control(1, GLYPH_DOWN, 0);
        assert!(
            down.contains("move this step later\nit is already the last step"),
            "{down}"
        );
    }

    /// **Live, each control's hover is what a click does, and nothing
    /// else** — on a two-row list, where row 0's down arrow and row
    /// 1's up arrow both move.
    #[test]
    fn a_lists_live_step_controls_say_what_a_click_does() {
        for (glyph, nth, action) in [
            (GLYPH_REMOVE, 0, "remove this step"),
            (GLYPH_UP, 1, "move this step earlier"),
            (GLYPH_DOWN, 0, "move this step later"),
            ("+", 0, "insert a step after this one"),
        ] {
            let hovered = hovering_step_control(2, glyph, nth);
            assert!(hovered.contains(action), "{glyph}: {hovered}");
            assert!(
                !hovered.contains(&format!("{action}\n")),
                "a live control carried a reason: {hovered}"
            );
        }
    }

    /// **On a list of more than one row, only the ends are stopped** —
    /// row 0's up arrow and row 1's down arrow on two rows, each
    /// saying which end it is at. A lone row is both ends, so it
    /// cannot tell an end-of-list gate from a one-row gate.
    #[test]
    fn a_two_row_lists_end_arrows_say_which_end() {
        let up = hovering_step_control(2, GLYPH_UP, 0);
        assert!(
            up.contains("move this step earlier\nit is already the first step"),
            "{up}"
        );
        let down = hovering_step_control(2, GLYPH_DOWN, 1);
        assert!(
            down.contains("move this step later\nit is already the last step"),
            "{down}"
        );
    }

    /// **Apply says why while an untouched draft disables it**, and
    /// says nothing of its own for a refused preview, whose sentence
    /// [`super::preview_verdict`] draws.
    #[test]
    fn apply_says_there_is_nothing_to_apply_until_a_number_moves() {
        let untouched = painted_while_hovering("Apply", 0, |ui| {
            super::apply_and_revert(ui, false, false, &[]);
        });
        assert!(
            untouched
                .contains("nothing to apply: the steps and numbers are the committed profile's"),
            "{untouched}"
        );
        let refused = painted_while_hovering("Apply", 0, |ui| {
            super::apply_and_revert(ui, true, true, &[]);
        });
        assert!(!refused.contains("nothing to apply"), "{refused}");
    }

    /// **Revert says why while it is disabled**, and live says what it
    /// puts back.
    #[test]
    fn revert_says_there_is_nothing_to_revert_until_a_number_moves() {
        let untouched = painted_while_hovering("Revert", 0, |ui| {
            super::apply_and_revert(ui, false, false, &[]);
        });
        assert!(
            untouched
                .contains("nothing to revert: the steps and numbers are the committed profile's"),
            "{untouched}"
        );
        assert!(
            !untouched.contains("put the steps and numbers back"),
            "{untouched}"
        );
        let moved = painted_while_hovering("Revert", 0, |ui| {
            super::apply_and_revert(ui, true, false, &[]);
        });
        assert!(
            moved.contains("put the steps and numbers back to the committed profile's"),
            "{moved}"
        );
        assert!(!moved.contains("nothing to revert"), "{moved}");
    }

    /// A chord fine enough that nothing here is short of points.
    const CHORD: f64 = 1.0e-4;

    /// What [`preview_verdict`] painted for `preview`, and whether it
    /// held the commit.
    fn drawn(preview: &Result<ProfilePreview, PreviewError>) -> (Vec<Landed>, Voices, bool) {
        let mut held = None;
        let (painted, voices) = landed_voiced(&Theme::DEFAULT, |ui, theme| {
            held = Some(preview_verdict(ui, *theme, Some(preview)));
        });
        (painted, voices, held.expect("the verdict was drawn"))
    }

    /// The editor's own preview of `shapes`.
    fn previewed(shapes: &[ProfileShape]) -> Result<ProfilePreview, PreviewError> {
        sketch::preview(SketchPlane::xy(), shapes, Tol::witness(), CHORD)
    }

    /// The editor's own preview of one path loop.
    fn path(steps: Vec<Step<f64>>) -> Result<ProfilePreview, PreviewError> {
        previewed(&[ProfileShape::Path { steps }])
    }

    fn at(x: f64, y: f64) -> Step<f64> {
        Step::At(Point2::new(x, y))
    }

    /// **A chain being written is quiet, drawn or not.** The drawn
    /// open chain and the one-point chain that ended before anything
    /// could be drawn are one state, so they are one voice — and both
    /// hold the commit.
    #[test]
    fn an_unfinished_chain_is_quiet_and_holds_the_commit() {
        let open = path(two_legs(0.0, 0.0));
        assert!(
            matches!(&open, Ok(drawn) if drawn.has_unfinished_chain()),
            "a fixture that draws an open chain: {open:?}"
        );
        let (painted, voices, held) = drawn(&open);
        assert_eq!(
            find_opening(&painted, "the chain does not close yet").ink,
            Some(voices.weak)
        );
        assert!(held, "an open chain holds the commit");

        let ended = path(vec![at(0.0, 0.0)]);
        let Err(error) = &ended else {
            panic!("a one-point chain does not replay: {ended:?}")
        };
        assert!(
            matches!(error, PreviewError::Transition { verb: None, .. }),
            "{error}"
        );
        let (painted, voices, held) = drawn(&ended);
        assert_eq!(find(&painted, &error.to_string()).ink, Some(voices.weak));
        assert!(held, "a chain that never closes holds the commit");
    }

    /// **A refusal that blames a written step is loud**: an ill-typed
    /// verb, and a leg whose geometry refused.
    #[test]
    fn a_refusal_of_a_written_step_is_loud() {
        let ill_typed = path(vec![at(0.0, 0.0), Step::Tangent]);
        assert!(
            matches!(
                &ill_typed,
                Err(PreviewError::Transition {
                    verb: Some(Verb::Tangent),
                    ..
                })
            ),
            "{ill_typed:?}"
        );
        let geometry = Err(PreviewError::Geometry {
            loop_: 0,
            step: 1,
            kind: PathErrorKind::JunctionTangent,
            rendered: "the leg has no answer".to_owned(),
        });
        for refused in [ill_typed, geometry] {
            let Err(error) = &refused else {
                panic!("both fixtures are refusals: {refused:?}")
            };
            let (painted, voices, held) = drawn(&refused);
            assert_eq!(
                find(&painted, &error.to_string()).ink,
                Some(voices.actionable),
                "{error}"
            );
            assert!(held, "{error} holds the commit");
        }
    }

    /// **A refused step's loop draws, and the form says the refusal
    /// loud.** The `arc_fillet_arc` the form hands an author who picks
    /// it after two legs is refused on arrival; what is said under the
    /// step list is that refusal in the actionable voice, holding the
    /// commit — never the unfinished chain's quiet sentence, though the
    /// loop drawn does not close.
    ///
    /// Red if `ProfilePreview::hold` asks the unfinished chain before
    /// the refusal, or if the refusal's hold takes the advisory tone.
    #[test]
    fn a_refused_step_draws_and_is_said_loud() {
        let mut steps = two_legs(0.0, 0.0);
        let state = sketch::tip_state_at(&steps, steps.len(), Tol::witness());
        steps.push(sketch::fresh_step_at(Verb::ArcFilletArc, state));
        steps.push(Step::LineTo(Target::Start));
        let cut = path(steps);
        let Ok(cut_short) = &cut else {
            panic!("the steps before the refused one draw: {cut:?}")
        };
        let refused = cut_short.loops[0]
            .end
            .refusal()
            .expect("the loop carries its refusal");
        let (painted, voices, held) = drawn(&cut);
        assert_eq!(
            find(&painted, &refused.to_string()).ink,
            Some(voices.actionable),
            "{refused}"
        );
        assert!(
            !painted
                .iter()
                .any(|landed| landed.text.starts_with("the chain does not close yet")),
            "{:?}",
            painted.iter().map(|l| &l.text).collect::<Vec<_>>()
        );
        assert!(held, "a refused step holds the commit");
    }

    /// **A chain with an unclosable tip draws the legs before it, and
    /// the form says that tip's own sentence, quietly** — at every
    /// state no `line_to` leaves, as the lattice table lists them. What
    /// is painted under the step list is the end-of-program refusal
    /// naming the tip, in the weak voice, holding the commit; never the
    /// open chain's sentence, which asks for a close the tip cannot
    /// take.
    ///
    /// Red if the preview draws nothing for such a tip, if it drops the
    /// tip's refusal or says the open chain's sentence, or if that
    /// refusal is said loud.
    #[test]
    fn an_unclosable_tip_draws_its_legs_and_says_its_own_sentence_quietly() {
        let census = test_support::unclosable_tips();
        assert!(
            census.contains(&TipState::RadiusArrival),
            "the census reads the table: {census:?}"
        );
        let legs = two_legs(0.0, 0.0);
        let Ok(open) = path(legs.clone()) else {
            panic!("the legs alone draw")
        };
        let open_chain = PreviewHold::OpenChain.to_string();
        for state in census {
            let Some(way_in) = ::profile::test_support::way_in(state) else {
                assert_eq!(state, TipState::Entry, "only the entry has no way in");
                continue;
            };
            let steps = [legs.clone(), way_in].concat();
            let unclosable = PreviewError::Transition {
                loop_: 0,
                step: steps.len(),
                state,
                verb: None,
            };
            let preview = path(steps);
            let Ok(prefix) = &preview else {
                panic!("{state:?}: the legs before the tip draw: {preview:?}")
            };
            assert_eq!(
                prefix.loops[0].points, open.loops[0].points,
                "{state:?}: the legs, and only those"
            );
            assert_eq!(
                prefix.loops[0].end,
                LoopEnd::Unfinished(Some(Cut {
                    refusal: unclosable.clone(),
                    closes: false,
                })),
                "{state:?}"
            );
            let (painted, voices, held) = drawn(&preview);
            assert_eq!(
                find(&painted, &unclosable.to_string()).ink,
                Some(voices.weak),
                "{state:?}"
            );
            assert!(
                !painted.iter().any(|landed| landed.text == open_chain),
                "{state:?}: {:?}",
                painted.iter().map(|l| &l.text).collect::<Vec<_>>()
            );
            assert!(held, "{state:?}: an unfinished chain holds the commit");
        }
    }

    /// **A chain whose close is refused on its geometry draws the legs
    /// written, and the form says why, quietly** — at every shape of
    /// such a close ([`test_support::geometry_refused_closes`]). What is
    /// painted under the step list is the refusal the drawn loop
    /// carries, in the weak voice, holding the commit: the close's own,
    /// in the driver's words, or the end-of-program refusal where the
    /// last leg is the close in all but spelling. Never the open
    /// chain's sentence, which would hide what stops the close.
    ///
    /// Red if the preview draws nothing for such a chain, if the
    /// refusal is not painted or is painted loud, or if the open
    /// chain's sentence is said instead.
    #[test]
    fn a_close_refused_on_its_geometry_says_its_own_sentence_quietly() {
        let open_chain = PreviewHold::OpenChain.to_string();
        for fixture in test_support::geometry_refused_closes() {
            let preview = path(fixture.steps.clone());
            let Ok(drawn_legs) = &preview else {
                panic!("{:?}: the legs written draw: {preview:?}", fixture.steps)
            };
            let refused = drawn_legs.loops[0]
                .end
                .unfinished_refusal()
                .expect("the loop carries why it is drawn short");
            if let PreviewError::Geometry { rendered, .. } = refused {
                assert!(
                    refused.to_string().contains(rendered.as_str()),
                    "{:?}: in the driver's own words: {refused}",
                    fixture.steps
                );
            }
            let (painted, voices, held) = drawn(&preview);
            assert_eq!(
                find(&painted, &refused.to_string()).ink,
                Some(voices.weak),
                "{:?}",
                fixture.steps
            );
            assert!(
                !painted.iter().any(|landed| landed.text == open_chain),
                "{:?}: {:?}",
                fixture.steps,
                painted.iter().map(|l| &l.text).collect::<Vec<_>>()
            );
            assert!(
                held,
                "{:?}: an unfinished chain holds the commit",
                fixture.steps
            );
        }
    }

    /// **A drawn preview that does not validate is loud**, and a valid
    /// one is a quiet count that holds nothing.
    #[test]
    fn an_invalid_preview_is_loud_and_a_valid_one_is_a_quiet_count() {
        let crossing = previewed(&[
            ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius: 0.01,
            },
            ProfileShape::Circle {
                centre: [0.015, 0.0],
                radius: 0.01,
            },
        ]);
        assert!(
            matches!(&crossing, Ok(drawn) if drawn.invalid.is_some()),
            "{crossing:?}"
        );
        let (painted, voices, held) = drawn(&crossing);
        assert_eq!(
            find_opening(&painted, "does not validate: ").ink,
            Some(voices.actionable)
        );
        assert!(held, "an invalid profile holds the commit");

        let square = path(vec![
            at(0.0, 0.0),
            line_to(0.01, 0.0),
            line_to(0.01, 0.01),
            Step::LineTo(Target::Start),
        ]);
        let (painted, voices, held) = drawn(&square);
        assert_eq!(
            find(&painted, "1 loop(s), drawn in the viewport").ink,
            Some(voices.weak)
        );
        assert!(!held, "a valid preview holds nothing");
    }

    /// **A preview that is open AND invalid says the open chain,
    /// quietly** — one sentence and one tone off one partition of the
    /// value. `sketch::preview` never builds this value; a sentence
    /// and a tone read off two partitions would draw the open chain's
    /// words in the invalid profile's colour.
    #[test]
    fn an_open_and_invalid_preview_says_the_open_chain_quietly() {
        let planted = Ok(ProfilePreview {
            plane: SketchPlane::xy(),
            loops: vec![PreviewLoop {
                points: vec![[0.0, 0.0], [0.01, 0.0], [0.01, 0.01]],
                vertices: vec![0, 1, 2],
                end: LoopEnd::Unfinished(None),
            }],
            invalid: Some(ProfileError::EmptyProfile),
        });
        let (painted, voices, held) = drawn(&planted);
        assert_eq!(
            find_opening(&painted, "the chain does not close yet").ink,
            Some(voices.weak)
        );
        assert!(
            !painted
                .iter()
                .any(|landed| landed.text.starts_with("does not validate")),
            "{:?}",
            painted.iter().map(|l| &l.text).collect::<Vec<_>>()
        );
        assert!(held, "an open chain holds the commit");
    }

    // -------------------------------------------------------------- //
    // The edit door, driven through the panel
    // -------------------------------------------------------------- //

    /// A session over a 10 mm square profile authored as the form's
    /// default chain, a unit extrude of it, and a derived frame on the
    /// wall step `named` draws — a payload carrier of one profile
    /// piece's name. Answers the session, the profile, the carrier and
    /// the name.
    fn named_square(
        named: usize,
    ) -> (
        crate::session::DocSession,
        pncad::document::RecipeNodeId,
        pncad::document::RecipeNodeId,
        pncad::prelude::StableName,
    ) {
        use pncad::document::Datum;
        use pncad::prelude::{EntityKind, ProfileEdgeRef, RoleSeg, StableName};
        use pncad::select::PieceRole;

        let tol = Tol::witness();
        let (doc, plane) = inserted(&Doc::empty_derived("edit-door", tol), xy_frame(), tol);
        let loops = vec![
            sketch::loop_program(
                &crate::session::ProfileShape::Path {
                    steps: Drafts::default().profile_path,
                },
                Notation::CANONICAL,
            )
            .expect("the form's default chain lowers"),
        ];
        let (doc, profile) = inserted(
            &doc,
            Node::Profile(ProfileProgram {
                plane,
                loops,
                ids: Vec::new(),
            }),
            tol,
        );
        let (doc, extrude) = inserted(
            &doc,
            Node::Extrude {
                profile,
                distance: crate::test_support::len(0.01),
            },
            tol,
        );
        let Some(Node::Profile(program)) = doc.node(profile) else {
            panic!("a profile")
        };
        let wall = StableName {
            kind: EntityKind::Face,
            node: extrude,
            path: vec![RoleSeg::Lateral(ProfileEdgeRef::Piece {
                step: program.ids[0][named],
                role: PieceRole::Leg,
            })],
        };
        let (doc, carrier) = inserted(
            &doc,
            Node::Datum(Datum::FaceFrame {
                at: extrude,
                face: wall.clone(),
                spin: crate::test_support::ang(0.0),
            }),
            tol,
        );
        (
            crate::session::DocSession::inline(doc, tol),
            profile,
            carrier,
            wall,
        )
    }

    /// **The edit door, clicked** — the `nth` painting of `target`,
    /// over the draft `drafts` holds of `profile`, through
    /// [`super::edit_door_ui`]. Answers what the door painted once
    /// clicked and the op its Apply formed, if it formed one.
    fn click_door(
        session: &crate::session::DocSession,
        drafts: &mut Drafts,
        profile: pncad::document::RecipeNodeId,
        target: &str,
        nth: usize,
    ) -> (String, Option<crate::session::SessionOp>) {
        let mut formed = None;
        let painted = crate::pane::headless::painted_after_clicking_nth(target, nth, |ui| {
            let edit = drafts
                .profile_edit(session.committed_doc(), profile)
                .expect("the editor holds the profile");
            let written = (
                (pncad::quantity::M.def(), pncad::quantity::RAD.def()),
                Notation::CANONICAL,
            );
            match super::edit_door_ui(ui, session, Theme::DEFAULT, written, edit, None) {
                Ok(Some(op)) => formed = Some(op),
                Ok(None) => {}
                Err(notice) => panic!("the door could not form its op: {notice:?}"),
            }
        });
        (painted, formed)
    }

    /// **A committed profile reshaped in the panel lands as one edit,
    /// one undo, and its names follow.** The door's `+` on the first
    /// row inserts a leg; the new corner is typed into it; Apply —
    /// which strands nothing, and says nothing about stranding — forms
    /// one op, which lands as one `SetProgram` and one history state.
    /// The frame on the right wall still spells that wall's step, the
    /// reshaped program still draws the step's leg, nothing is
    /// reported, and one undo restores the square.
    #[test]
    fn a_reshaped_profile_lands_as_one_edit_and_its_names_follow() {
        use pncad::document::{Datum, DocEdit};
        use pncad::prelude::{ProfileEdgeRef, RoleSeg};
        use pncad::select::PieceRole;

        let (mut session, profile, carrier, wall) = named_square(2);
        let before = session.committed_doc().clone();
        let mut drafts = Drafts::default();
        let (_, formed) = click_door(&session, &mut drafts, profile, "+", 0);
        assert!(formed.is_none(), "an insert forms no op");
        let edit = drafts
            .profile_edit(session.committed_doc(), profile)
            .expect("held");
        assert_eq!(edit.loops()[0].len(), 6, "the + inserted a row");
        edit.steps_mut(0)[1] = Step::LineTo(Target::Point(Point2::new(0.005, -0.005)));
        let (painted, formed) = click_door(&session, &mut drafts, profile, "Apply", 0);
        assert!(!painted.contains("stranding"), "{painted}");
        let op = formed.expect("Apply formed the op");
        let state = session.history().current();
        let out = session.perform(op);
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
        assert!(
            matches!(out.committed.as_slice(), [DocEdit::SetProgram { .. }]),
            "one whole-program edit: {:?}",
            out.committed
        );
        assert!(out.maintenance.is_empty(), "{:?}", out.maintenance);
        let Some(Node::Profile(program)) = session.committed_doc().node(profile) else {
            panic!("a profile")
        };
        assert_eq!(program.ids[0].len(), 6);
        let RoleSeg::Lateral(piece) = wall.path[0].clone() else {
            unreachable!("built as a lateral wall")
        };
        let ProfileEdgeRef::Piece { step, role } = piece else {
            unreachable!("built as a piece")
        };
        assert_eq!(role, PieceRole::Leg);
        assert_eq!(
            program.ids[0][3], step,
            "the named step moved down a row, id and all"
        );
        let drawn = program
            .pieces(&session.committed_doc().param_env::<f64>(), Tol::witness())
            .expect("the reshaped program replays");
        assert!(
            drawn.edges.iter().flatten().any(|edge| *edge == piece),
            "the reshaped program still draws the named leg: {:?}",
            drawn.edges
        );
        assert!(
            matches!(
                session.committed_doc().node(carrier),
                Some(Node::Datum(Datum::FaceFrame { face, .. })) if *face == wall
            ),
            "the carrier's name is untouched"
        );
        assert!(
            session
                .perform(crate::session::SessionOp::Undo)
                .refusal
                .is_none()
        );
        assert_eq!(session.history().current(), state, "one undo");
        assert!(
            session.committed_doc().bit_eq(&before),
            "the square is back"
        );
    }

    /// **A step removed in the panel strands the names on it, and the
    /// author is told before and after.** The door's `×` on the right
    /// wall's row drops that step; Apply then counts the name it will
    /// strand on its own label and names the carrier on its hover;
    /// clicked, the op's outcome reports the strand, and the status
    /// line words it.
    #[test]
    fn a_removed_step_strands_its_names_and_says_so_before_and_after() {
        use pncad::document::Maintenance;

        let (mut session, profile, carrier, wall) = named_square(2);
        let mut drafts = Drafts::default();
        let (_, formed) = click_door(&session, &mut drafts, profile, GLYPH_REMOVE, 2);
        assert!(formed.is_none(), "a removal forms no op");
        let label = "Apply, stranding 1 name";
        let hovered = painted_while_hovering(label, 0, |ui| {
            let edit = drafts
                .profile_edit(session.committed_doc(), profile)
                .expect("held");
            let written = (
                (pncad::quantity::M.def(), pncad::quantity::RAD.def()),
                Notation::CANONICAL,
            );
            let _hovered_only =
                super::edit_door_ui(ui, &session, Theme::DEFAULT, written, edit, None);
        });
        assert!(
            hovered.contains(&format!("node {} carries a {wall}", carrier.0)),
            "the hover names the carrier and the name: {hovered}"
        );
        // Another step dropped instead strands nothing — what Apply
        // says is asked again of every held state, not kept from the
        // last one ...
        let (_, formed) = click_door(&session, &mut drafts, profile, "Revert", 0);
        assert!(formed.is_none());
        let (painted, _) = click_door(&session, &mut drafts, profile, GLYPH_REMOVE, 3);
        assert!(!painted.contains("stranding"), "{painted}");
        // ... and the named step re-verbed, from there, strands its
        // name again: a step drawn by another verb is another step.
        drafts
            .profile_edit(session.committed_doc(), profile)
            .expect("held")
            .steps_mut(0)[2] = Step::ArcTo(pncad::profile::ArcData::Bulge {
            target: Target::Point(Point2::new(0.01, 0.01)),
            b: 0.3,
        });
        let painted = crate::pane::headless::painted_text(|ui| {
            let edit = drafts
                .profile_edit(session.committed_doc(), profile)
                .expect("held");
            let written = (
                (pncad::quantity::M.def(), pncad::quantity::RAD.def()),
                Notation::CANONICAL,
            );
            let _read_only = super::edit_door_ui(ui, &session, Theme::DEFAULT, written, edit, None);
        });
        assert!(painted.contains(label), "{painted}");
        let (_, formed) = click_door(&session, &mut drafts, profile, "Revert", 0);
        assert!(formed.is_none());
        let (painted, _) = click_door(&session, &mut drafts, profile, GLYPH_REMOVE, 2);
        assert!(painted.contains(label), "{painted}");
        let (_, formed) = click_door(&session, &mut drafts, profile, label, 0);
        let op = formed.expect("Apply formed the op");
        let out = session.perform(op.clone());
        assert!(out.refusal.is_none(), "{:?}", out.refusal);
        let expected = vec![Maintenance::Strand {
            node: carrier,
            name: wall,
        }];
        assert_eq!(out.maintenance, expected, "the door reports the strand");
        let line: Vec<String> = crate::frame::outcome_notices(&out)
            .map(|notice| notice.text().to_owned())
            .collect();
        assert_eq!(
            line,
            vec![expected[0].to_string()],
            "the status line carries it in its own words"
        );
    }
}
