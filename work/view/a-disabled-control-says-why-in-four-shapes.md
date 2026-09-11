---
id: a-disabled-control-says-why-in-four-shapes
kind: issue
title: A control a reader cannot use says why in four shapes, and two of them are not on_disabled_hover_text at all
status: open
opened: 2026-09-11
refs: [environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere, gesture-drags-have-no-cancel-door]
---


Reported by the review of the cancel-door unit (2026-09-11, #2320),
which added the fourth spelling. Filed rather than taken there: the
choice between the four is a decision about what a reason IS, and
`CancelDoor` had to pick one to exist.

## The sweep rule, and why it is not `on_disabled_hover_text`

The claim is about **a control a reader cannot use that says why**, so
the population is that disposition and not the egui call that usually
carries it. Two members do not call `on_disabled_hover_text` at all, and
a rule shaped like the call cannot see them — the proxy failure this
program has now paid for repeatedly. The rule that produces the list:
every site under `crates/viewer/src` that draws a control as unusable
(`add_enabled(false, …)`, or a branch that draws a sentence where the
control would be) **together with** the text saying why. Run as
`add_enabled` (18 sites) ∪ `on_disabled_hover_text` (10 production
sites), then each read for what it hands the reader.

## The four shapes

1. **A typed refusal, rendered by the control** — the reason lives on
   the value and the control asks it for its words.
   `pane/create.rs:248-259` (a catalogue entry's `refusal()`, *"read off
   the entry, not minted here"*) and `app.rs:1256-1263` (`CancelDoor`,
   the new one).
2. **A literal composed at the button** — `app.rs:1176`,
   `pane/create.rs:830`, `pane/create.rs:1132`,
   `pane/properties.rs:205`, `pane/properties.rs:715`,
   `pane/create.rs:589-592`.
3. **A `&'static str` composed AWAY from the value that knows** —
   `frame::NO_CHOOSER_BACKEND` at `app.rs:1198` and `:1217`. Already
   filed, one facility over, as
   `environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere`.
4. **A prose sentence built from a typed state** —
   `pane/create.rs:740-750`, which formats a row's label and
   `sketch::tip_state_words(state)` into one sentence.

And the two the call-shaped rule misses, both in `pane/create.rs`:

- **`blocked: Option<&'static str>`** (`:445`, set at `:449-520`, drawn at `:524-526`, and the gate on the
  button at `:591`) —
  a field with the SAME NAME as `CancelDoor::blocked` and the opposite
  typing: a string, not a refusal, rendered through `ui.weak` beside
  the control rather than as its disabled reason.
- `pane/properties.rs:347-352`, the free-move probe's ineligibility:
  `ui.weak(fault.to_string())` **where the control would be**, with a
  typed `DisplayFault` — *"the same sentence the op would refuse with"*.
  The typed discipline of shape 1, rendered as shape 3's neighbour.

## What is actually at stake

Not tidiness. Shapes 1 and the `properties.rs` case put the sentence on
the value that knows it; shapes 2 and 3 put it at the button, where it
is free to drift from what the operation would actually answer. The
cancel-door unit holds its own pair together with a test
(`a_closed_door_says_what_its_own_operation_refuses`) precisely because
nothing structural does.

Whether shape 2 is wrong at all is the real question: several of its
sites have no refusal to read, because the condition they report is not
an operation's to refuse (*"pick a dimension first"* gates a draft, not
a door). If that is the dividing line then the class is smaller than it
looks and the rule is *a control whose condition IS an operation's
refusal reads it from the operation* — which would make the
`blocked: Option<&'static str>` field and `NO_CHOOSER_BACKEND` the only
genuine hits.

## Home

VIEW's: `crates/viewer/src/app.rs`, `crates/viewer/src/pane/create.rs`,
`crates/viewer/src/pane/properties.rs`.
