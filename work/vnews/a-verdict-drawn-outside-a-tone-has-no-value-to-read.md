---
id: a-verdict-drawn-outside-a-tone-has-no-value-to-read
kind: issue
title: "Four verdict sites draw their salience outside a Tone: two by omission, two as a bare ui.weak"
status: open
opened: 2026-09-25
priority: P3
cost: E
---

Found by the sweep of `resolution-and-standing-pick-their-tone-by-hand`
(its rule: *every site under `crates/viewer/src` that renders a typed
refusal or verdict and chooses its salience at the call*), run on
merge base `1c39d2402`. These are the hits that sweep's fix did not
take, because each needs a decision the row did not own.

**By omission** — `crate::widgets::message(ui, …)` over a typed value,
neither weak nor coloured, so the body colour, and nothing at the site
or on the value says it was chosen:

1. **The status line**, `app.rs` (the toolbar's `status.text()`, after
   `frame::prefs_badge`) and `pane/view.rs` (`view_ui`'s own
   `status.text()`). A `frame::Message` carries no `Tone` — only a
   `frame::Badge` does — and no doc says whether that is a decision
   (an outcome is read in the body voice) or an absence. If it is a
   decision, `frame::Message`'s doc should say so and this item
   closes as that sentence; if not, `Message` grows a tone the way
   `Badge` has one.
2. **The checks window's findings**, `app.rs`
   (`crate::widgets::message(ui, finding.to_string())`). Every finding
   is behind a badge that is `Tone::Actionable`
   (`frame::checks_badge`), and the window draws each in the body
   colour. Plausibly right — the window is where the sentences are
   READ, per `frame::Affordance::Opens`'s doc — but unstated.

**As a bare `ui.weak`** — the `Advisory` look spelled without a
`Tone`, which `widgets::message_toned`'s doc calls a third answer to
what `Advisory` looks like:

3. `widgets.rs`, the number field's `Err(unit)` arm:
   `ui.weak(props::no_reading(unit))`.
4. `widgets.rs`, the unit field's `props::written` refusal:
   `ui.weak(…marker…)` over the same `props::no_reading`.

`props::no_reading`'s own doc says the reader's next move is to write
the row in a coarser notation — which reads as `Actionable` by
`frame::Tone`'s definition. Whether it is, or whether a field that
cannot show its number is a report, is the decision.

**Not members**, recorded so the sweep can be re-run: the pick-state
lines in `pane/create.rs` (`"pick a: …"`, the blend target, the edge
count), the loop count and index in `pane/profile.rs`, the dimension
and axis labels in `pane/properties.rs`, and every `ui.label(format!…)`
header — state, counts and names, not verdicts.

**Territory.** `app.rs` is CHROME's and VSEAM's, `pane/view.rs`
CHROME's and VGEOM's, `widgets.rs` AUTHOR's, CHROME's and VGEOM's;
`frame::Message` is this program's vocabulary, which is why the row is
filed here.

## Evidence 2026-09-25 (VNEWS, from VNEWS #3281's style review)

`widgets::message_toned`'s doc (`widgets.rs`) says a call site that
spells `.weak()` itself is *"a third answer to what `Advisory` looks
like"*. Two held-pick lines in `pane/create.rs` are that spelling,
beside the seated and mate panels' lines, which since PR 3281 draw
through `message_toned(…, Tone::Advisory)` (`seats_row`,
`mate_picks_row`):

- the blend panel's held-picks line, `ViewerBehavior::blend_tool_ui`:
  `ui.weak(match (target, landed) { … "{count} edges picked on {}"
  (the target said from the landed document) … "no edges picked yet" })`;
- the face-frame datum form's face line,
  `ViewerBehavior::datum_face_frame_rows` (near lines 858-859):
  `ui.weak(…)` of `BlendTarget::of_face(face)` said from the landed
  document, and
  `ui.weak("none picked")`.

This sits against the row's own "Not members" paragraph, which files
the `pane/create.rs` pick-state lines as state rather than verdicts.
The two readings answer different questions: the row's test is
whether a line is a VERDICT, the doc's is whether a line draws the
`Advisory` look without the `Tone`. Two held-pick lines in one
file now draw one look two ways; which test governs is part of this
row's decision.
