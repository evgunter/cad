---
id: the-per-frame-badge-reads-are-three-hand-copied-fields
kind: issue
title: The per-frame badge reads are three hand-copied fields, each wired five times in app.rs
status: open
opened: 2026-09-30
priority: P1
cost: M
---


Found by the review of AUTH-14 (`author/edge-name-fault`), which added
the third field.

## The finding

The toolbar's per-frame, unlatched badge reads are one mechanism: the
viewport writes the read while it draws, the frame entry point
(`<ViewerApp as eframe::App>::ui`) zeroes a local before the panes
draw and assigns it back afterwards whether or not the viewport drew,
and a `frame::*_badge` reads the field next frame. In
`crates/viewer/src/app.rs` that mechanism is spelled out by hand once
per read, five times each:

| read | struct field | init (`ViewerApp::assemble`) | zeroed local | `ViewerBehavior` field | assign-back |
|---|---|---|---|---|---|
| `datums_vanished` | ~:396 | ~:839 | ~:1888 | ~:2089 | ~:1950 |
| `profiles_undrawn` | ~:402 | ~:840 | ~:1889 | ~:2093 | ~:1951 |
| `held_edges_refused` | ~:408 | ~:841 | ~:1890 | ~:2097 | ~:1952 |

The toolbar draws each separately (~:1762, ~:1768, ~:1772). Each field
doc says it is zeroed and assigned back "as `datums_vanished` is". The
rule is stated three times and kept by copying.

Nothing checks that a new read is zeroed. A copy that seeds its local
from the field instead of a zero latches a badge. AUTH-14's
`a_blend_target_whose_edges_cannot_be_named_says_so` guards the third
read against that by hiding the viewport tile for a frame. The first
two have no such guard.

## The shape of a fix

One per-frame value (a struct of the reads, `Default` being "the
viewport drew nothing"), zeroed and assigned back once, lent to the
viewport as one field. That makes the zeroing structural for every
member. The badge doors in `crate::frame` stay one per read.

## Fence

`crates/viewer/src/app.rs` (CHROME, VSEAM, AUTHOR), and
`crates/viewer/src/pane/viewport.rs` for the writes.
