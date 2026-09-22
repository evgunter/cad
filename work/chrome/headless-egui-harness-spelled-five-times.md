---
id: headless-egui-harness-spelled-five-times
kind: issue
title: Five modules hand-roll the same headless egui drive, each with its own texture-delta discharge
status: open
opened: 2026-09-21
priority: P4
cost: E
---

## What

Driving a widget against a real `egui::Context` with no window is
spelled separately in each module that does it:
`crates/viewer/src/pane/view.rs` (twice), `widgets.rs` (three times),
`pane/viewport.rs`, `pane/profile.rs`, and — as of AUTH-3 —
`crates/viewer/src/pane.rs`'s `headless`, which the create and
features pane suites share.

Every one of them is the same four lines: a `Context::default()`, a
`run_ui` with a `RawInput`, and a `textures_delta.clear()` to stop
`TexturesDelta`'s drop panic. The last of those is the tell — it is a
detail of epaint, not of any pane, and nine sites each remember it
separately.

## Why it is filed and not fixed

Found by AUTH-3, which added the ninth and routed only its own two
suites through one home; sweeping the other seven is a change in five
modules for no behaviour, which is a sitting of its own rather than a
drive-by inside a unit about frames.

Note what a merge would and would not buy. `pane::headless` reads back
the TEXT a frame painted, which the seven existing sites do not need —
they drive a widget and then assert over the state it left behind. So
the shared thing is the drive, not the read: the home wants a
`painted`/`drive` pair rather than one function, and deciding that
shape is most of the work.

`pane::headless` is `#[cfg(test)]` in a `pane` submodule, which is the
wrong home for a harness `widgets.rs` would also use.

## A second READ, and the wrong home showing (2026-09-22)

The layout half of `error-and-check-text-overflows-its-region` needed
to read WHERE a frame painted its text, not only what it said, so
`pane::headless` grew `landed` beside `painted` — the galley's rows,
each rect translated into the caller's own coordinates.

**The first version of that change minted a fresh instance of this
row's own defect, inside the PR that closes a duplication.** It added
`collect_landed` — a THIRD verbatim copy of the `Shape::Text` /
`Shape::Vec` recursive walker already spelled twice in this module, as
`hit` and as `collect` — and a second driver `landed`, byte-identical
to `painted` apart from which collector it called, `textures_delta`
discharge and all. `landed` also strictly subsumed `painted`:
`painted(d) == landed(d).into_iter().map(|l| l.text).collect()`, with
`hit` being `landed`'s galley rect centred. The PR's own fresh-instance
check looked only at `pane/profile.rs`'s deleted helpers, and its
amendment to this row argued in advance that *"`landed` is a second
READ, not a seventh drive"* — which is a textual justification standing
where a grep would have done.

**Folded in the fix pass.** The module now has ONE walker
(`landed_in`), ONE drive (`landed`), and `painted` and `hit` derived
from them; `painted_after_clicking` walks once per frame instead of
twice. The two coordinate conventions that were incidental before are
now fields with names and a reason: `Landed::allocated` is `pos` plus
`Galley::size` — leading space INCLUDED, which is the box a click has
to land in, and why `hit` centres on it — and `Landed::rows` is one
rect per row with the leading space EXCLUDED, which is where a
reader's eye finds the first glyph and what a layout row measures.

So the count above is unchanged, and `landed_in` is now also callable
on shapes a caller drove itself: `app::tests::toolbar_with` uses it to
read the status line out of the REAL toolbar, which needs two frames
and a `screen_rect` and so cannot go through `landed`.

What this does not fix is the home. `widgets::message_tests` — in
`widgets.rs`, which this row already names as a module the harness's
home is wrong for — says `crate::pane::headless::landed` to measure a
widget that has nothing to do with any pane, and `app::tests` now says
`crate::pane::headless::landed_in`. A `painted`/`drive` pair is still
the shape; `landed_in`, `landed`, `painted` and `hit` are what
whichever home takes them inherits.
