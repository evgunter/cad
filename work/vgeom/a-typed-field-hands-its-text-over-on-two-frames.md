---
id: a-typed-field-hands-its-text-over-on-two-frames
kind: issue
title: a DragValue parses its buffered text on two consecutive frames, so one typed number reaches the chrome twice
status: closed
opened: 2026-09-21
priority: P2
cost: D
branch: vgeom/field-product
pr: 3067
closed: 2026-09-22
---


(AUTH-2) Found while re-shaping the property panel's no-op guard;
measured, not inferred.

## The finding

`egui::DragValue` keeps the text of an open keyboard edit in
`ui.data` under the widget's id and parses it in TWO places
(`egui-0.36.1/src/widgets/drag_value.rs`): once inside the
keyboard-editing branch, when the text box reports `lost_focus`, and
again at the top of the next frame, where `mem.lost_focus(id)` is
STILL true and the buffered text is removed and parsed a second time.
`Response::lost_focus()` therefore answers true on both frames, so it
cannot be used to tell them apart either.

A chrome that turns a parse into a session operation emits that
operation twice for one typed number. Measured at the property
panel's parameter row with a real `egui::Context` and a real
`DocSession`: typing `1002` over a field showing `1000.0` emits
`SetParam` on two consecutive frames, with identical payloads.

## What holds it today, and where it does not reach

The two panel value fields are covered, at the document rather than at
the widget: `DocSession::writes_nothing` refuses to submit an edit
that writes what the document already holds, so the second hand-over
costs no undo step. `widgets::value_field_tests::
the_second_hand_over_of_one_typed_text_changes_nothing` is the row,
and it fails loudly if the toolkit ever stops doing this.

Two things that rule does not reach:

- **A door with no "already" to compare.** `writes_nothing` answers
  `false` for every edit that is not a slot literal or a declaration's
  value or notation, which is the conservative direction — the second
  hand-over would cost an undo step rather than losing an edit. A
  future field whose door mints something (an insert) would be
  charged twice.
- **The field's own guard sees it only when the render round-trips.**
  `props::echoed` compares the typed text against the field's render,
  and the formatter runs before BOTH parse sites, so the render is
  populated both times. What lets a second hand-over through is
  narrower than "the document has moved": `widgets::number_text`
  spells at least one decimal, so a user who typed `1002` is compared
  against `1002.0`. Where the round trip IS exact the field stops the
  second parse by itself — `widgets::value_field_tests::
  the_field_swallows_the_second_hand_over_when_its_render_round_trips`
  is that row, and its sibling is the case that reaches the document.
  So the two rules are genuinely two, and a reader who collapses them
  re-opens this.

## The toolkit DOES separate the two frames — considered and declined

`egui::Memory::had_focus_last_frame(id)` is public and is exactly the
discriminator `lost_focus` is not. Both read the same bookkeeping
(`egui-0.36.1/src/memory/mod.rs`): `lost_focus` is
`(id_previous_frame == Some(id) || id_two_frames_ago == Some(id)) &&
!has_focus(id)`, and it spans both frames BY DESIGN — the doc comment
says so. `had_focus_last_frame` is `id_previous_frame == Some(id)`
alone, with no `two_frames_ago` disjunct, so it is true on the frame
that carries the first parse and false on the one that carries the
second. The earlier claim here — that the toolkit cannot tell them
apart — was true of `lost_focus` and wrong about the toolkit.

It was declined anyway, for two reasons and neither is "it cannot be
done":

- **Reaching it costs a bet.** The parser is a closure inside
  `DragValue::custom_parser`, so using it means capturing the
  `egui::Context` AND the widget's id into that closure — and the id
  is `ui.next_auto_id()` (`drag_value.rs`), read before the widget is
  added. A caller re-deriving it is betting that `DragValue` consumes
  exactly one auto-id, which is an implementation detail of the
  toolkit rather than anything it promises.
- **It does not buy the other half.** Suppressing the repeat at the
  field would not answer the case `DocSession::writes_nothing` also
  answers: a person re-typing, in different characters, a number or an
  expression that already stands (`50 mm` over a parameter declared
  `50 mm`, `base_r*2.0` over a slot driven by `base_r * 2.0`). That is
  a document question, and the document door would still have to ask
  it.

## What a fix would answer

Whether the chrome should recognise the repeat AT the field — either
through `had_focus_last_frame` above, or by remembering, per widget
id, the text a field last turned into an operation and clearing it on
`gained_focus` so a deliberate re-type is still an act. Either is
state the panel does not otherwise keep, and either would let the
field emit at most one operation per keyboard edit rather than relying
on every door below it being idempotent.
