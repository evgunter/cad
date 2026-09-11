---
id: two-datumkind-enums-name-the-same-four-datum-kinds
kind: issue
title: two DatumKind enums in the viewer crate name the same four datum kinds
status: open
opened: 2026-09-06
---


Found by the style review of PR 2046 (`view/const-all`).

## Two enums, one concept

- `crates/viewer/src/datums.rs:276` — `pub enum DatumKind { Plane,
  Frame, Axis, Point }`, re-exported at the crate root
  (`crates/viewer/src/lib.rs:128`, `pub use datums::{DatumDraw,
  DatumKind}`), so it is `viewer::DatumKind`. It tags what a datum
  draw IS.
- `crates/viewer/src/forms.rs:93` (`view/const-all`: `:74`) — `pub(crate) enum DatumKind {
  Plane, Frame, Axis, Point }` behind the `app` feature. It tags what
  the add-datum form is OFFERING.

Same name, same four members, same crate. Nothing maps one to the
other and nothing holds them together; `crates/viewer/tests/
datum_draw.rs:23` imports one and `crates/viewer/src/pane/create.rs:14`
imports the other, so a reader meeting `DatumKind` in this crate has to
work out which they have.

## What PR 2046 did to it, without saying so

That unit reordered `forms::DatumKind`'s declaration from
`Plane, Axis, Point, Frame` into `Plane, Frame, Axis, Point` so the
enum would carry its form order. The result is that the two enums are
now variant-for-variant identical in name, membership AND order — the
drift is at its smallest it has ever been and nothing records that or
holds it there. The PR body argues the reorder is inert (it is: no
`Ord`, no discriminant, no serde, and `ALL`'s order is unchanged) but
does not mention the twin.

## Also worth deciding

`datums::DatumKind` is a closed vocabulary with no `ALL`, so PR 2046's
`vocabulary!` did not touch it. If the two are ever made one type, the
survivor should be declared through `crates/viewer/src/vocab.rs` like
its neighbours.
