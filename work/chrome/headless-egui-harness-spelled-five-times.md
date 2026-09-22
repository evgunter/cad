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
