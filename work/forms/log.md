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
