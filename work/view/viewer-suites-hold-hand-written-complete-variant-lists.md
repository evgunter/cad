---
id: viewer-suites-hold-hand-written-complete-variant-lists
kind: issue
title: the viewer suites hold hand-written complete variant lists nothing forces
status: open
opened: 2026-09-06
---


Found by the style review of PR 2046 (`view/const-all`), which
projected nine `const ALL` tables in `crates/viewer/src` from their
enums' own declarations. That unit's census swept
`crates/viewer/src` only. `crates/viewer/tests/*` is VIEW's territory
too (Ev, in-chat, 2026-09-04), and the same shape lives there.

## The class

**A hand-written array of every variant of an enum, in a suite, whose
completeness the row's claim depends on.** PR 2046's own PR body
rejects exactly this shape as an answer for `ToolKind::ALL`: a
suite-local list "would be hand-written, unforced and invisible to the
compiler — the same defect one directory over". It is already there.

## The instances

- `crates/viewer/tests/chrome_labels.rs:64` —
  `for pane in [Pane::Viewport, Pane::Features, Pane::Properties,
  Pane::View]`, asserting each has a tile. `Pane`
  (`crates/viewer/src/app.rs:276`) has exactly those four variants. A
  fifth pane added with no tile leaves this row green.
- `crates/viewer/tests/review_gui0_r2.rs:303` — `buttons = [Primary,
  Secondary, Middle]`, the whole of `PointerButton`
  (`crates/viewer/src/input.rs:63`), driving the `map_stream`
  consistency fuzz. A fourth button is silently never fuzzed.
- `crates/viewer/tests/frame_policy.rs:506-508` — one
  `frame::cursor_status` call per `IdStep`
  (`crates/viewer/src/frame.rs:1812`), all three, written out. The
  weakest of the three: it is a list of calls rather than a list of
  variants, but the row's claim is still about the whole vocabulary and
  a fourth step would not be asked.

**A fourth instance was here and is not fixed — it is gone.** It was
`frame_policy.rs`'s three-`ChooserBackend` loop under *"a chosen path
is never this policy's business"*, and it went with
`an_empty_dialog_is_loud_only_under_a_confidently_absent_backend` when
Ev's ruling on
`was-the-status-route-supposed-to-fire-for-an-absent-chooser` deleted
`frame::dialog_status`. `ChooserBackend` still has three variants and
still has no `ALL`; nothing in this class was repaired, the row that
depended on the hand-written list simply stopped existing. **The count
above is three because the population is three, not because one was
answered.**

None of these enums carries an `ALL`, so `vocabulary!` does not reach
them as they stand: taking this means deciding, per enum, whether the
vocabulary earns a projected `ALL` on the type or whether the row
should be re-expressed so completeness is not what it rests on.

## Not the same as, and adjacent to

`work/view/hand-maintained-mirrors-of-a-kernel-enum-are-unforced.md`
is about a viewer table mirroring ANOTHER crate's enum. This is the
viewer's own enums, listed by hand in the viewer's own suites.

## The sweep that found these, and its blind spot

A structural scan over `crates/viewer/**/*.rs` for array literals
holding two or more distinct `Type::Variant` entries of one type, then
each hit read against that enum's variant count. It cannot match: a
list built with `vec!`, an iterator chain or a `matches!` ladder; a
list written after `use Enum::*` so the type prefix is absent; a list
containing a nested bracket; and an enumeration spelled as match arms
rather than as an array.
