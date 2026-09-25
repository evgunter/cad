---
id: three-spellings-say-a-parameter-is-not-declared
kind: issue
title: Three spellings say a parameter is not declared, and one of them is Refusal::NoSuchParam's own
status: open
opened: 2026-09-19
refs: [a-disabled-control-says-why-in-four-shapes]
priority: P3
cost: E
---

Found by the census in `a-disabled-control-says-why-in-four-shapes`,
at merge base `2654cc111417da806d9786c40136106469096fec`, and found
**only because that census was made to enumerate its judged pass
instead of asserting its size**. Both sites below sit in the pass whose
members the earlier draft never listed.

A genuine hit of the census's rule: *a control a reader cannot use owes
the sentence a click would have been answered with — when there is such
a sentence.*

## The refusal, and its one home

`crates/viewer/src/session/refuse.rs`'s `Refusal::NoSuchParam` renders:

> no document parameter named {name} — declare it first

Two doors raise it: `Session::begin_param_gesture`
(`crates/viewer/src/session.rs`, the `ok_or_else` in the `start`
closure) and `crates/viewer/src/session/probe.rs`'s parameter probe.
The first is the one that matters here — it is the door a drag on a
parameter's number field reaches.

## The two chrome spellings

`crates/viewer/src/pane/properties.rs`, both in the `else` of a branch
whose other arm draws the control:

1. `Panel::standing_ui`'s `Standing::Param { present: false }` arm
   (the `colored_label` at `:297`) draws
   `format!("parameter {} is no longer declared", name.0)` where the
   present arm draws the parameter's row.
2. The parameter panel's own `else` (`:119`) draws
   `"that parameter is gone"` where the `Some` arm draws the draggable
   value field and `Panel::param_bounds_ui`.

So a reader who lost a parameter out from under a selection is told the
fact in the chrome's words, and a reader who drags before the frame
catches up is told it in the session's — three compositions of one
fact, held in step by nothing. `Refusal::exists_wording` and
`Refusal::affordance` are the shape the repair takes: one composition
in `refuse.rs`, called by the panel.

## What a fix has to decide

`NoSuchParam`'s wording ends *"— declare it first"*, which is a status
line's instruction and is the wrong half for a panel that is describing
a selection rather than answering an attempt. This is the same fork
`the-new-document-button-states-its-refusal-twice` hit and
`Refusal::exists_wording` answered: a wording helper whose words serve
both surfaces, rather than two literals. It is a `refuse.rs` change,
not a panel change.

## Home

VNEWS's: `crates/viewer/src/pane/properties.rs` and
`crates/viewer/src/session/refuse.rs` are both in this program's
`paths`. `crates/viewer/src/session.rs` is read, not edited — the door
already raises the right variant.

## The tone half is settled (2026-09-25)

The two chrome spellings were also drawn in two TONES in one frame —
`Actionable` for `standing_ui`'s, `Advisory` for the panel's
`"that parameter is gone"`. Both now read `Standing::tone()`
(`resolution-and-standing-pick-their-tone-by-hand`), so whichever
spelling survives this row keeps the one tone. The words are
untouched and remain this row's.
