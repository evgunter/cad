---
id: ctrl-wheel-reaches-no-zoom
kind: issue
title: A ctrl+wheel reaches no zoom: the toolkit routes it into zoom_factor_delta and the viewport reads only smooth_scroll_delta
status: open
opened: 2026-09-15
priority: P0
cost: E
---



Filed by the VIEW unit that closed
`work/view/viewport-adapter-drops-part-of-two-toolkit-values`. That
row asked for a STATEMENT at two narrowings and got one; this is the
product decision the statement could not make, and the one place where
re-deriving the row's premise against the tree changed the answer.

## What is actually true

**A ctrl+wheel over the viewport does nothing at all.** Not "the same
thing as a plain wheel" — nothing.

`InputState`'s per-frame pass (`egui-0.36.1/src/input_state/mod.rs`,
the `is_zoom` arm of `begin_pass`) asks whether the wheel's modifiers
match `InputOptions::zoom_modifier`, which is `Modifiers::COMMAND` by
default and matches ctrl, ⌘ and `command` alike
(`Modifiers::matches_any`). When they do, the delta is multiplied into
`zoom_factor_delta` and `smooth_scroll_delta` is left at `Vec2::ZERO`.
`ViewerBehavior::viewport_ui` reads `smooth_scroll_delta` and nothing
else, so no `ViewportEvent` is produced and no `CameraOp` follows.

Held by `pane::viewport::tests::
a_ctrl_wheel_is_spent_by_the_toolkit_before_the_adapter_sees_it`, which
also asserts the delta arrived in `zoom_delta()` — the value a binding
would have to read.

**This corrects the parent row's premise.** It said `ctrl` being unread
at the modifier site "means a ctrl+scroll and a plain scroll are the
same event". They are not the same event, and the cause is not the
unread `ctrl` field: the toolkit spends the modifier before the adapter
runs, so reading `ctrl` in `viewer_modifiers` would recover nothing.

## Why it is worth a row

Ctrl+wheel is the zoom gesture in every browser, and in every
mainstream CAD package this viewer's bindings follow
(`crates/viewer/src/input.rs`, module docs). `crates/viewer/README.md`'s
mouse-bindings table says `scroll | zoom`; a user who holds ctrl —
because that is what zooming is everywhere else — gets a viewport that
ignores them, with no refusal and no status line.

It is also the gesture a **trackpad pinch** arrives as on the web
backend and on several native ones, since the browser reports a pinch
as a ctrl+wheel.

## The shape an answer has

Reading a THIRD toolkit value: `InputState::zoom_delta()`, a
multiplicative factor rather than a notch count, folded into a
`ViewportEvent` the vocabulary can carry. Three questions it opens,
none of which the adapter can answer alone:

1. **Vocabulary.** `ViewportEvent::Scroll { units }` is notches and
   `InputMap::zoom_rate_per_notch` turns notches into a factor. A zoom
   factor is already a factor; converting it to notches to convert it
   back is the lossy shape. Either a second variant, or `CameraOp::
   Dolly` reached by a second route.
2. **Rate.** `zoom_factor_delta` is `exp(scroll_zoom_speed * points)`
   with the toolkit's own constant in it, so a ctrl+wheel and a plain
   wheel would zoom at different rates unless one of them is rescaled.
   A pinch has no notches at all.
3. **Whether the two should agree.** A pinch and a wheel are different
   gestures; making both mean the same `Dolly` is a decision, not a
   translation.

Not this unit's, and not cheap: it is a binding, a vocabulary change
and a rate decision, where the row that found it was a statement.
