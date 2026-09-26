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
use crate::forms::{SHAPE_LOCKED, ShapeEdits};
use crate::frame;
use crate::session::SessionOp;
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
    /// that agree.
    ///
    /// What differs is what the editor is opened ON and what it
    /// commits as: the node's program, loaded by
    /// [`crate::sketch::held_loops`], committed as slot writes by
    /// [`SessionOp::EditProfile`] — which is why the shape controls
    /// are locked ([`ShapeEdits::Locked`]).
    ///
    /// Returns `false`, having said why in the unresolved colour, when
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
            &mut self.drafts.length_unit,
            &mut self.drafts.angle_unit,
        );
        let units = (self.drafts.length_unit.def(), self.drafts.angle_unit.def());
        let notation = self.drafts.notation();
        let Some(edit) = self.drafts.profile_edit.as_mut() else {
            unreachable!("`Drafts::profile_edit` answered Ok, so the draft is held")
        };
        ui.label(format!("profile on frame {}", edit.plane().0));
        crate::widgets::message_toned(ui, SHAPE_LOCKED, &self.theme, frame::Tone::Advisory);
        let loops = edit.loops.len();
        for (index, steps) in edit.loops.iter_mut().enumerate() {
            // A loop label only where there is more than one: "loop 0"
            // over the only loop is a number with nothing to tell
            // apart.
            if loops > 1 {
                ui.label(format!("loop {index}"));
            }
            path_steps_ui(
                ui,
                &format!("edit_{}_{index}", node.0),
                session.tol(),
                units,
                ShapeEdits::Locked,
                steps,
            );
        }
        let refused = preview_verdict(ui, self.theme, self.profile_previews.edit.as_ref());
        let moved = edit.moved();
        ui.horizontal(|ui| {
            let (apply, revert) = apply_and_revert(ui, moved, refused);
            if apply {
                match edit.programs(notation) {
                    Ok(loops) => self.ops.push(SessionOp::EditProfile {
                        node,
                        base: edit.base().clone(),
                        loops,
                    }),
                    Err(error) => self.notices.push(frame::tool_news(
                        format!("edit profile: {error}"),
                        frame::Retold::Again,
                    )),
                }
            }
            if revert && let Err(error) = edit.revert(doc) {
                self.notices.push(frame::tool_news(
                    format!("revert profile: {error}"),
                    frame::Retold::Again,
                ));
            }
        });
        true
    }
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
/// Every verdict that is a SENTENCE goes through
/// [`crate::widgets::message_toned`]; the loop count is a number and
/// stays a plain label.
pub(crate) fn preview_verdict(
    ui: &mut egui::Ui,
    theme: Theme,
    preview: Option<&Result<ProfilePreview, PreviewError>>,
) -> bool {
    match preview {
        // The first frame this form is on screen: the latch has
        // not asked for a preview yet, so there is nothing
        // honest to say about one. The commit door is still the
        // judge, so the button is not held for a frame either.
        None => false,
        Some(Ok(drawn)) if drawn.has_open_chain() => {
            // **Drawn, and still not committable.** The chain is
            // in the viewport (`sketch::preview` walks it under a
            // provisional close) so the shape can be looked at
            // while it is written; what it is not yet is a loop,
            // and the commit door refuses a program that does not
            // close. Saying which of the two this is beats a
            // disabled button with a lattice refusal beside it.
            crate::widgets::message_toned(
                ui,
                "the chain does not close yet — its last step has to target the start",
                &theme,
                frame::Tone::Advisory,
            );
            true
        }
        Some(Ok(drawn)) => {
            if let Some(invalid) = &drawn.invalid {
                crate::widgets::message_toned(
                    ui,
                    format!("does not validate: {invalid}"),
                    &theme,
                    frame::Tone::Actionable,
                );
                true
            } else {
                ui.weak(format!(
                    "{} loop(s), drawn in the viewport",
                    drawn.loops.len()
                ));
                false
            }
        }
        Some(Err(error)) => {
            // **Unfinished is not wrong.** The end-of-program arm
            // says only that the chain has no closing verb yet,
            // which is the state every chain passes through while
            // it is being written — a one-point chain reaches the
            // form this way, because there is no leg for the
            // provisional close to be walked over. Every OTHER
            // refusal blames a step somebody actually wrote, and
            // keeps the colour that says so.
            if matches!(error, PreviewError::Transition { verb: None, .. }) {
                crate::widgets::message_toned(ui, error.to_string(), &theme, frame::Tone::Advisory);
            } else {
                crate::widgets::message_toned(
                    ui,
                    error.to_string(),
                    &theme,
                    frame::Tone::Actionable,
                );
            }
            true
        }
    }
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
/// profile editor draw: the add-profile form over its own draft
/// ([`ShapeEdits::Free`]), and the same form opened on a committed
/// profile, one list per loop ([`ShapeEdits::Locked`], whose controls
/// that would change the program's shape are drawn and not taken).
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
pub(crate) fn path_steps_ui(
    ui: &mut egui::Ui,
    salt: &str,
    tol: Tol,
    (length_unit, angle_unit): (UnitDef, UnitDef),
    shape: ShapeEdits,
    steps: &mut Vec<Step<f64>>,
) {
    // The row edits are COLLECTED and applied after the loop: a
    // list cannot be reordered or shortened while it is being
    // iterated, and one edit per frame is what keeps two buttons
    // clicked in one frame from compounding into a move nobody
    // asked for.
    let mut remove: Option<usize> = None;
    let mut swap: Option<(usize, usize)> = None;
    // A verb chosen in a row's combo, applied after the loop for
    // the same reason the moves are: the probe that decides which
    // verbs a combo may offer reads the WHOLE list, and it cannot
    // borrow it while a row holds a mutable slice of it.
    let mut rebind: Option<(usize, Verb, Option<TipState>)> = None;
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
            let free = shape.free();
            // Why each control is stopped, if it is — the lock first:
            // a locked list offers no move at all, whichever row it
            // is.
            let lock_reason = (!free).then_some(SHAPE_LOCKED);
            if step_control(ui, GLYPH_REMOVE, "remove this step", lock_reason) {
                remove = Some(index);
            }
            let earlier_reason = lock_reason.or((index == 0).then_some(STEP_IS_FIRST));
            if step_control(ui, GLYPH_UP, "move this step earlier", earlier_reason) {
                swap = Some((index, index - 1));
            }
            let later_reason = lock_reason.or((index == last).then_some(STEP_IS_LAST));
            if step_control(ui, GLYPH_DOWN, "move this step later", later_reason) {
                swap = Some((index, index + 1));
            }
            // **Insert after this row.** A chain is written in the
            // middle as often as at the end — a leg forgotten
            // between two that exist used to mean appending it and
            // walking it up with the arrows — so every row carries
            // the control, and the last row's is the append.
            //
            // In the row's own control cluster rather than at the
            // far end of it, which is where this first went: a
            // row's width is its verb's, so at the end the `+`
            // sits at a different place on every row and, on the
            // widest, past the edge of a pane that does not scroll
            // sideways. A control that moves under the cursor is
            // worse than one that is not where a reader first
            // looks for it.
            if step_control(ui, "+", "insert a step after this one", lock_reason) {
                insert = Some(index + 1);
            }
            let verb = steps[index].verb();
            ui.add_enabled_ui(free, |ui| {
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
                                    rebind = Some((index, option, state));
                                }
                                None => {}
                            }
                        }
                    });
            });
            let step = &mut steps[index];
            path_step_fields(ui, &row_salt, length_unit, angle_unit, state, shape, step);
        });
    }
    if let Some((index, verb, state)) = rebind {
        steps[index] = sketch::fresh_step_at(verb, state);
    }
    if let Some(index) = remove {
        steps.remove(index);
    }
    if let Some(at) = insert {
        steps.insert(at, new_row_step(at));
    }
    if let Some((from, to)) = swap {
        steps.swap(from, to);
    }
    // A locked list has no whole-list controls to offer: both
    // change what the program IS.
    if !shape.free() {
        return;
    }
    ui.horizontal(|ui| {
        // **"Add step" only when there is no row to insert after.**
        // Once the list has rows, every one of them carries a `+`
        // that inserts after it — including the last, which is the
        // append — so a second control at the bottom would be the
        // same move spelled twice.
        //
        // There is no verb picker beside it either. It duplicated
        // the row combo one row down: whatever the new step is,
        // the way to change it is the same control either way, and
        // a second one only asked the question a frame earlier.
        if steps.is_empty() {
            if ui.button("Add step").clicked() {
                steps.push(new_row_step(0));
            }
        } else if ui.button("Clear").clicked() {
            steps.clear();
        }
    });
}

/// Why a row's move-earlier control is not live in a free list.
const STEP_IS_FIRST: &str = "it is already the first step";

/// Why a row's move-later control is not live in a free list.
const STEP_IS_LAST: &str = "it is already the last step";

/// **One of a step row's glyph controls**, live unless `blocked`
/// names why not.
///
/// The glyph is the control's only label, so `action` — what a click
/// does — is its hover in both states, and a blocked control adds the
/// reason on the line under it. Every reason is a draft gate's (a
/// locked list, or a row with nothing past it to move over): no
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
const UNTOUCHED: &str = "the numbers are the committed profile's";

/// **The edit door's Apply and Revert**, both live only once the held
/// numbers have `moved` off the committed profile's; Apply also waits
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
/// Answers whether each was clicked — Apply, then Revert — which a
/// disabled button never is.
fn apply_and_revert(ui: &mut egui::Ui, moved: bool, refused: bool) -> (bool, bool) {
    let apply = ui.add_enabled(moved && !refused, egui::Button::new("Apply"));
    let apply = if moved {
        apply
    } else {
        apply.on_disabled_hover_text(format!("nothing to apply: {UNTOUCHED}"))
    };
    let revert = ui.add_enabled(moved, egui::Button::new("Revert"));
    let revert = if moved {
        revert.on_hover_text("put the numbers back to the committed profile's")
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
    use pncad::document::{Dimension, Doc, DocEdit, Expr, Node, ProfileProgram, apply};
    use pncad::geom_core::{Point2, Tol};
    use pncad::profile::Step;

    use crate::app::{GLYPH_DOWN, GLYPH_REMOVE, GLYPH_UP};
    use crate::drafts::Drafts;
    use crate::pane::headless::painted_while_hovering;
    use crate::session::author::datum_node;
    use crate::sketch;

    /// **Drawing the editor never rewrites a document value.** A
    /// committed `circle_split` above the form's count cap (the
    /// document admits any count) is loaded into the edit door and
    /// DRAWN once, locked and untouched: its count is the committed
    /// one, and nothing reads as moved.
    #[test]
    fn drawing_a_locked_split_circle_above_the_cap_leaves_it_alone() {
        use crate::forms::{MAX_CIRCLE_SPLIT, ShapeEdits};
        let len = |m: f64| Expr::literal(m, Dimension::Length).expect("finite");
        let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite");
        let doc = Doc::empty_derived("probe", Tol::witness());
        let frame = datum_node(crate::session::DatumSpec::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        });
        let doc = apply(
            &doc,
            &DocEdit::InsertNode { node: frame },
            Tol::witness(),
            &pncad::document::RefusingReach,
        )
        .expect("frame")
        .doc;
        let plane = *doc.order().last().expect("the frame");
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
                sketch::Notation::CANONICAL,
            )
            .expect("finite"),
        ];
        let node = Node::Profile(ProfileProgram {
            plane,
            loops,
            ids: Vec::new(),
        });
        let doc = apply(
            &doc,
            &DocEdit::InsertNode { node },
            Tol::witness(),
            &pncad::document::RefusingReach,
        )
        .expect("the document admits a split circle above the form's cap")
        .doc;
        let profile = *doc.order().last().expect("the profile");
        let mut drafts = Drafts::default();
        let edit = drafts.profile_edit(&doc, profile).expect("held");
        assert!(!edit.moved(), "fresh load");
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            super::path_steps_ui(
                ui,
                "probe",
                Tol::witness(),
                (pncad::quantity::M.def(), pncad::quantity::RAD.def()),
                ShapeEdits::Locked,
                &mut edit.loops[0],
            );
        });
        output.textures_delta.clear();
        let held_n = match edit.loops[0][0] {
            Step::CircleSplit { n, .. } => n,
            _ => panic!("a split circle"),
        };
        assert_eq!(held_n, n, "drawing the locked editor rewrote the count");
        assert!(!edit.moved(), "drawing alone made Apply live");
    }

    /// What a pointer resting on the `nth` painting of `glyph` reads,
    /// over a list of `rows` fresh steps drawn under `shape`.
    fn hovering_step_control(
        shape: crate::forms::ShapeEdits,
        rows: usize,
        glyph: &str,
        nth: usize,
    ) -> String {
        let mut steps: Vec<Step<f64>> = (0..rows).map(crate::widgets::new_row_step).collect();
        painted_while_hovering(glyph, nth, |ui| {
            super::path_steps_ui(
                ui,
                "probe",
                Tol::witness(),
                (pncad::quantity::M.def(), pncad::quantity::RAD.def()),
                shape,
                &mut steps,
            );
        })
    }

    /// **A locked list's step controls say why while disabled**: each
    /// names what it would do, and under that the lock notice's own
    /// value — the one reason, on the control the pointer is on. The
    /// lock is read first, so a lone row's arrows, which the index
    /// would also stop, still say the list is locked.
    #[test]
    fn a_locked_lists_step_controls_each_say_the_list_is_locked() {
        use crate::forms::{SHAPE_LOCKED, ShapeEdits};
        for (glyph, action) in [
            (GLYPH_REMOVE, "remove this step"),
            (GLYPH_UP, "move this step earlier"),
            (GLYPH_DOWN, "move this step later"),
            ("+", "insert a step after this one"),
        ] {
            let hovered = hovering_step_control(ShapeEdits::Locked, 1, glyph, 0);
            assert!(
                hovered.contains(&format!("{action}\n{SHAPE_LOCKED}")),
                "{glyph}: {hovered}"
            );
            assert!(!hovered.contains("already"), "{glyph}: {hovered}");
        }
    }

    /// **A free list's end rows say which end they are at** — a lone
    /// row is both, and neither arrow has a row to move past.
    #[test]
    fn a_lone_free_rows_arrows_say_it_is_first_and_last() {
        use crate::forms::ShapeEdits;
        let up = hovering_step_control(ShapeEdits::Free, 1, GLYPH_UP, 0);
        assert!(
            up.contains("move this step earlier\nit is already the first step"),
            "{up}"
        );
        let down = hovering_step_control(ShapeEdits::Free, 1, GLYPH_DOWN, 0);
        assert!(
            down.contains("move this step later\nit is already the last step"),
            "{down}"
        );
    }

    /// **Live, each control's hover is what a click does, and nothing
    /// else** — on a two-row list, where row 0's down arrow and row
    /// 1's up arrow both move.
    #[test]
    fn a_free_lists_live_step_controls_say_what_a_click_does() {
        use crate::forms::ShapeEdits;
        for (glyph, nth, action) in [
            (GLYPH_REMOVE, 0, "remove this step"),
            (GLYPH_UP, 1, "move this step earlier"),
            (GLYPH_DOWN, 0, "move this step later"),
            ("+", 0, "insert a step after this one"),
        ] {
            let hovered = hovering_step_control(ShapeEdits::Free, 2, glyph, nth);
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
    fn a_two_row_free_lists_end_arrows_say_which_end() {
        use crate::forms::ShapeEdits;
        let up = hovering_step_control(ShapeEdits::Free, 2, GLYPH_UP, 0);
        assert!(
            up.contains("move this step earlier\nit is already the first step"),
            "{up}"
        );
        let down = hovering_step_control(ShapeEdits::Free, 2, GLYPH_DOWN, 1);
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
            super::apply_and_revert(ui, false, false);
        });
        assert!(
            untouched.contains("nothing to apply: the numbers are the committed profile's"),
            "{untouched}"
        );
        let refused = painted_while_hovering("Apply", 0, |ui| {
            super::apply_and_revert(ui, true, true);
        });
        assert!(!refused.contains("nothing to apply"), "{refused}");
    }

    /// **Revert says why while it is disabled**, and live says what it
    /// puts back.
    #[test]
    fn revert_says_there_is_nothing_to_revert_until_a_number_moves() {
        let untouched = painted_while_hovering("Revert", 0, |ui| {
            super::apply_and_revert(ui, false, false);
        });
        assert!(
            untouched.contains("nothing to revert: the numbers are the committed profile's"),
            "{untouched}"
        );
        assert!(!untouched.contains("put the numbers back"), "{untouched}");
        let moved = painted_while_hovering("Revert", 0, |ui| {
            super::apply_and_revert(ui, true, false);
        });
        assert!(
            moved.contains("put the numbers back to the committed profile's"),
            "{moved}"
        );
        assert!(!moved.contains("nothing to revert"), "{moved}");
    }
}
