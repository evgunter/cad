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

- 2026-09-30 — Seam note from AUTH-13 (`author/geometry-close`). An unfinished chain whose provisional close is refused on its geometry now draws the legs written: it walks back through the one `sketch::prefix_loop` call the other two arms take and ends `LoopEnd::Unfinished(Some(Cut))`, carrying the close's own refusal as the new `PreviewError::Close { loop_, step, rendered }` (advisory; its `Display` is the arm it shares with `Geometry`). The loop's start is read off the entry's first `at`, and the provisional close is `line_to Start` or, where that would run straight on from the last leg, `continue_to Start` (`sketch::provisionally_closed`). `pane/profile.rs`: one new test row (`a_close_refused_on_its_geometry_says_its_own_sentence_quietly`); `preview_verdict` is unchanged and paints the new refusal through `PreviewHold::Refusal`. (AUTH-13 implementer)
