# FORMS — the log

## 2026-09-22 — opened

Cut out of CHROME, which was carrying 88 budget points against a
30-point ceiling, along its priority seam per `work/README.md` "Track
size". CHROME's P0 and P1 rows alone came to 55, so the seam had to cut
inside P1 too; it was cut where the slate already divided — the text
CHROME shows and the badges it draws stay there, the creation forms'
vocabulary comes here, and what the viewer offers a person to do went
to OFFER, opened in the same commit.

Eleven rows arrived by `git mv` with their ids, bodies and history
unchanged. Band 10000-10099 is claimed for this program in the same
commit (`docs/MODEL-AB-LOG.md`); CHROME keeps 1600-1699. Nothing
dispatched.

One row arrived with live evidence being written onto it elsewhere:
`body-seat-reads-through-the-placer-chain` gains an AUTH-4 section on
`author/part-and-duplicate` (#3052), which edits it at its old path. A
rename against a modify merges cleanly under rename detection, so
nothing was held back for it; if #3052's merge of `main` recreates the
old path instead, the section belongs here.

Signed (CHROME orchestrator).

## Seam note from AUTHOR (AUTH-4 fix pass, PR 3052, 2026-09-24)

**What a seated tool takes from a viewport pick changed, on this
program's ground** (`crates/viewer/src/tools.rs`,
`crates/viewer/src/session/select.rs`).

`tools::on_node_pick` — the one route every seated tool's pick takes —
now reads a new accessor, `Selection::seat_node`: the tree's node for a
tree click, and for a viewport pick the node whose DRAWN body the ray
met (`FaceSelection::node` / `EdgeSelection::node`). It used to read
`Selection::node`, which answers the feature that MINTED the face.
**`Selection::node` is unchanged** and still serves the feature tree's
highlight, the property panel's rows and the extrude form — it is the
right answer there.

Tools whose behaviour changes, all for the better and all held by
`combine_ops::a_viewport_pick_seats_the_drawn_body_in_every_body_seat`:
the **boolean, split, transform and pattern** tools (a face on a moved
copy or a filleted body now seats that body, not the upstream extrude),
and the two new ones, **projection** and **duplicate**. Unchanged: the
**revolve** tool (its seats are a profile and an in-sketch axis, which
no ray meets), and the **mate** and **blend** tools, which never took
this route — they read the face and the edge whole.

FORMS: `PartSelectChoice` and `split_half_label` (`forms.rs`) are the projection form's vocabulary and landed here with AUTH-4; the README's forms row and closed-vocabulary census now name them.

## 2026-09-29 — seam note from AUTH-6 (`author/profile-reshape`)

AUTH-6 removed `ShapeEdits` (both arms) and `SHAPE_LOCKED` from
`forms.rs`. A committed profile's program is now written whole by one
`DocEdit::SetProgram`, so the edit door has no controls to lock and the
sentence was false. With one arm left the enum said nothing, and the
`shape` parameter went from `widgets::path_step_fields`, `arc_fields`
and `target_fields`, and from `pane::profile::path_steps_ui`. The seven
`shape.free()` gates (target form, arc mode, three side pickers, winding,
split count) now take input in both doors. `path_steps_ui` now returns a
`drafts::RowEdit` for its caller to apply instead of reshaping the list
itself. (AUTH-6 implementer)

- 2026-09-30 — Seam note from AUTH-9 (`author/declared-union`). `combine.rs` gains `UndeclaredContact` (the boolean door's contact refusal) and `DeclareOffer`, and `BooleanTool::op` authors `declare: Vec::new()`; `session/probe.rs`'s `evaluate_with` is `pub(crate)`; `pane/create.rs` draws the offer under the boolean tool (`declare_offer_rows`). `forms.rs` is untouched. (AUTH-9 implementer)

- 2026-09-30 — Seam note from AUTH-9's fix pass (`author/declared-union`, PR #3543). `combine.rs` no longer holds the refusal or the offer; they moved to `session/refuse.rs`. Its module doc points at `tools.rs` for the actions that take more than one edit. `session/probe.rs`'s `evaluate_with` is gone, and the probe calls `evalseam::evaluate_beside`. (AUTH-9 implementer)

- 2026-09-30 — Seam note from AUTH-10 (`author/held-face-mark`, PR 3556). A pick a form or tool HOLDS has a mark of its own: the selection's colour (`Theme::held`), told from the live selection by shape (stripes on a face, a hollow line on an edge). `pane/create.rs`: the add-datum gate is `session::face_frame_seat_drawn` over the on-screen index, refusing a held face the picture does not draw (`FaceFrameFault::NotDrawn`); the form's face line reads `held_face`; the add-profile form's withholding reasons are renamed `Withheld` (`withheld_for`, `withheld_line`, `bore_withholds`) so "held" means a held pick. `pane/profile.rs`: one comment says "withheld". `work/forms/a-creation-forms-held-pick-survives-a-document-swap` gains a section: `datum_face` is now dropped on a document replacement; `datum_frame` is not, and the row's shape choice stands. (AUTH-10 implementer)

- 2026-09-30 — Seam note from AUTH-11 (`author/binder-prefix`, PR 3563). An unfinished chain whose tip is unclosable (no `line_to` leaves it, so the provisional close is ill-typed) now draws the prefix `sketch::prefix_loop` walks back to, and the form says that tip's end-of-program refusal, advisory. `sketch::LoopEnd` is now `Closed | Unfinished(Option<Cut>) | Refused(Cut)`, where `Cut { refusal, closes }` is shared, and `LoopEnd::unclosable()` reads an unfinished chain's cut; `PreviewHold::Refused` is renamed `PreviewHold::Refusal` and also carries an unclosable tip's refusal. `pane/profile.rs`: a new row, `an_unclosable_tip_draws_its_legs_and_says_its_own_sentence_quietly`, and the planted open chain spells `LoopEnd::Unfinished(None)`; `preview_verdict` is unchanged. (AUTH-11 implementer)

- 2026-09-30 — Seam note from AUTH-12 (`author/tool-census`, PR 3573). The nine tool activation buttons in `pane/create.rs` take their words from `ToolKind::button()` (`tools.rs`: `label()` capitalised, with the ellipsis) instead of literals, and a whole-app row (`app::properties_pane_tests::every_tool_opens_from_its_activation_button`) clicks each kind open by those words. A new tool panel places its kind in that row's section match, and a panel call dropped from `create_ui` or `properties_ui` reddens it. `pane::create::EXTRUDE` and `ADD_PART` are the extrude form's and the part chooser's button words, held by `the_extrude_form_and_the_part_chooser_are_reachable`. Painted text is unchanged.

- 2026-09-30 — Seam note from AUTH-12's fix pass (`author/tool-census`, PR 3573). `ToolKind::label()` (`tools.rs`) now returns the BARE noun ("mate", …, "projection"). `says` adds " tool: ", `button` capitalises and adds " tool…", and the new `ToolKind::commit()` gives "Commit <noun>". A new reader of `label()` gets the noun, not "<noun> tool". `tool_commit_row` lost its `label` parameter and reads `kind.commit()`; the mate and blend commit buttons read `ToolKind::{Mate,Blend}.commit()`. Painted text is unchanged. `pane/create.rs` gains consts for the three section headings (`ADD_FEATURE`, `COMBINE_BODIES`, `BLEND_EDGES`) and the form buttons (`ADD_DATUM`, `ADD_PROFILE`). `pane.rs`'s `headless` doc now sends a row that must drive a pane METHOD to `app::properties_pane_tests`' whole-app harness.

- 2026-09-30 — Seam note from AUTH-13 (`author/geometry-close`, PR 3579). An unfinished chain whose provisional close is refused now draws the legs written: it walks back through the one `sketch::prefix_loop` call the other two arms take and ends `LoopEnd::Unfinished(Some(Cut))`, carrying the close's own refusal (`PreviewError::Geometry`, which now carries a typed `kind: PathErrorKind`), or the end-of-program refusal where the close is ill-typed or the last leg itself closes the loop. `PreviewHold::Unfinished` is the new advisory hold for such a cut, and `LoopEnd::unclosable()` is renamed `LoopEnd::unfinished_refusal()`. The loop's start is read off the entry (`sketch::loop_start`), and the provisional close is `line_to Start`, re-spelled where the lattice refuses it as tangent (`sketch::replay_provisionally_closed`). `pane/profile.rs`: one new test row (`a_close_refused_on_its_geometry_says_its_own_sentence_quietly`); `preview_verdict` is unchanged and paints the cut's refusal through `PreviewHold::Unfinished`. (AUTH-13 implementer)

- 2026-09-30 — Seam note from AUTH-15 (`author/accept-part-version`, PR #3591). A new `SessionOp::AcceptPartVersion { id }` commits `pncad::workspace::update_to_store`'s edits (one `DocEdit::UpdateReference` per site whose pin moves) as one action. The store is read through a new `DocSession::read_store`, which `add_instance` and `part_catalogue` now share; `parts::catalogue` takes the scanned `&Workspace` and is infallible. `frame::version_offer(kind, files)` sits beside `declare_offer` and reads a `session::VersionOffer` off an instance's own `PartFault::Unresolved { fault: PinMismatch }`. `tree::TreeRow` gains `version_offer: Option<VersionOffer>` (every `TreeRow` literal needs the field). `DocSession::tree_rows` withholds every offer while `busy()`. `pane::features::feature_row_ui` draws `Refusal::version_question` and a `VersionOffer::LABEL` button under the row (`RowClicks::accept`). `DeclareOffer::ACCEPT_LABEL` and `DECLINE_LABEL` replace `pane::create`'s "Declare"/"Decline" literals. Every exhaustive `SessionOp` table gains the arm (`tools.rs`, `frame::acts`, the three in `session/op.rs`, and `tests/gesture_table.rs`, whose `OP_COUNT` is 47). `test_support::{PART_FILE, part_refused}` are new fixtures. Rows: `tests/instance_authoring.rs` (accept, one undo, the store's refusals, the committed-document read), `frame`'s `PartFault` census, the label-versus-recourse row in `session::refuse`, and a `pane::features` unit test.
- **2026-10-30, crossing announced by (INTENT orchestrator):** INTENT stage 6 E (`work/intent/the-sketch-tool-authors-a-tangent-continuation.md`, spec `docs/INTENT-STAGE6-SPEC.md` §6) edits `crates/viewer/src/sketch.rs` and `crates/viewer/src/pane/profile.rs`. The sketch tool draws a picked face's trace and writes C's `OnTrace`/`AlongTrace` step, and `sketch::is_committed` is fixed for a profile drawn on a frame. Branch `intent/s6-e-sketch-tool`. It takes none of this program's rows.
- 2026-11-02 — **Picked up; split; re-chartered** (FORMS orchestrator). The track stood at 52 of 30 once Ev's create/edit reports landed on it. Cut by question:
  - FORMS keeps the rows Ev reported, which ask whether creating a thing and editing it are one implementation.
  - The eleven vocabulary and seat-gate rows moved by `git mv` to a new program, FORMWORDS (`work/formwords/`, `ready`), with ids and bodies unchanged.
  - Citations of the moved paths were re-pointed in five other programs' rows and in one `forms.rs` doc comment.
  - `a-profile-step-cannot-refer-to-its-own-earlier-points` is closed as a duplicate of VSEAM's ruled self-reference build (`a-profile-cannot-refer-to-its-own-earlier-parts`, ruled on `[ev]` 6346: U3 reads a vertex, and Q10's `Slide` is "vertex plus an offset").
  - `the-add-profile-form-takes-numbers-and-no-formula` now rides with the profile unit, which subsumes it (priced E).
  - Review posture answered in the plan: full single review per unit, no duals.
- 2026-11-02 — **Designers weighed the create/edit rule** (fork row 246). Two designers, labels only. They agreed on every load-bearing point, so no reconciliation round ran.
  - The agreed rule went to Ev as `[ev]` PR 6388, with three confirmations: undo at Apply grain in the two draft editors; an empty profile as the empty region whose sweep refuses; and DESIGN.md's "Kind N" sentence.
  - Acted on from their orchestrator sections:
    - VSEAM's `an-expression-typed-over-held-profile-edits-is-refused` is a prerequisite of the profile unit. Without it, "variables can't be entered in the profile" survives the form's deletion, because every step of a new profile is unapplied. It is claimed here by `git mv`, re-banded P3 → P1 (a blocker inherits its parked row's band), and dispatched on `forms/held-expression`. Review tier: a single full review, because the parse and its refusal at the field carry correctness risk. VSEAM has no orchestrator; noted on its log.
    - The split plane's and the pattern's values may live in variables the insert mints rather than in node slots. Both designers flagged this as unverified. It is checked when those units are cut, not now.
    - Sequencing: the profile unit's "New profile on a face" waits on VSEAM's sketch-on-face units (doctail and lib), or takes the drawn-on-frame op as it stands at dispatch.
- 2026-11-02 — Ev on 6388: edits land live in the draft editors (README revised in place, 90d29b06a0); empty profiles are ok; the label sentence was explained and is awaiting Ev's reply. Dispatched `an-empty-profile-is-a-document-state` (P1/M, full single review), the first piece of the profile unit, which does not touch 6390's files. Its fix pass on 6390 is in flight after an APPROVE-WITH-FIXES review (four MINORs, no MAJOR; the class finding "refused text held in its field has two homes" is fixed or filed by that pass). (FORMS orchestrator)
- 2026-11-02 — **PR 6390 lands** (`an-expression-typed-over-held-profile-edits-is-refused`, P1/M). Full single review at 74eeec0af: APPROVE-WITH-FIXES, no MAJOR, four MINORs (a wrong-dimension formula previewed as a false length; the draft-side driven refusal untested, so mutants M4/M5 survived; the range probe judged the draft and acted on the document; an unknown name lost the add-variable offer) plus style items. Fix pass at 1f4c29537: all eleven fixed. Each carries a test that goes red without its fix, except S5's single-document read, which no test can red. editor-core's `slot_admits` is now the one dimension check, public, shared by the edit door's three single-slot sites and the viewer's draft. The class finding (refused text held in its field had two homes) is closed by construction, with one home: `Drafts::expr_target/expr_text`. Orchestrator's read of the fix-pass delta, no second review. Filed: `the-profile-editors-naming-and-offer-doors-wait-on-held-edits` (forms), `the-placement-editors-typed-value-says-a-parse-refusal-in-its-own-words` (placing). (FORMS orchestrator)
- 2026-11-02 — **Red main from 6390, fixed by #6397**: `slot_admits` was exported through pncad without a binding-census entry, and 6390's run skipped the python suite. It reddened #6393. The class (a pncad surface change skips the census) is filed on GREEN as `a-pncad-surface-change-skips-the-python-census` (P3). #6393 (empty profile) is in review, full single review on frozen head c7826ab30. (FORMS orchestrator)
- 2026-11-02 — Dispatched in parallel after #6393: the profile unit (`forms/profile-unit`, H: live commits in the one editor, New profile, the add-profile form deleted; full single review) and extrude/revolve (`forms/extrude-revolve`, M: insert from the selection, values as slot rows; full single review). Both are on create.rs and drafts.rs in separate functions, so logically independent; a selection hook either may need is reconciled by the orchestrator. Ev's item 3 (the label sentence) is still open on 6388. Both units store "Kind N" at insert, which is that sentence's consequence; if Ev rules otherwise, it is a one-line change in each. (FORMS orchestrator)
- 2026-11-02 — #6401 (extrude/revolve) and #6403 (profile unit) are both up, green, with no textual conflict reported. Each wrote a general "a creation selects the node it made" hook (`app::selects_what_it_made` vs `SessionOp::selects_what_it_makes`). Two homes for one rule, so whichever merges second adopts the first's hook; recorded so the fix pass cannot miss it. 6401 is in review now. 6403's review waits for disk, about 19 GB free with one review build live. (FORMS orchestrator)
