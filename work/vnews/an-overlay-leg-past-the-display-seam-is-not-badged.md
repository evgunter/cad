---
id: an-overlay-leg-past-the-display-seam-is-not-badged
kind: issue
title: An overlay leg the display seam refused is counted and nothing says so
status: open
opened: 2026-09-22
priority: P3
cost: D
---



Filed by `vgeom/seam-refusals`, which built the state and could not
write the sentence: the badge family is `crate::frame`'s.

## What now exists

`crate::marks::LegLane` is the display seam's per-leg disposition, in
one place: a leg with an end whose narrowing is not a finite `f32` is
not drawn, the rest of the lane is, and the drop is COUNTED —
`LegLane::undrawn`. Both producers reach it: the drawn edges of an
indexed body (`marks::edge_id_lane`, under `marks::edge_id_segments`)
and the three lanes `pane::viewport`'s `viewport_ui` composes itself
(`push_segment`, `push_loop`, and the datum marks).

So the VGEOM half of
`work/vgeom/the-display-seams-refusal-is-drawn-and-never-said` is
closed: the state is held and has a door. Nothing reads it. The lanes
are dropped into `EdgeOverlay`'s `Vec`s at the end of `viewport_ui`
and the count goes with them.

## Why it is worth a sentence — the evidence the row was decided on

The argument the VGEOM row recorded AGAINST a badge was that at the
magnitudes which reach this arm — a coordinate past `f32::MAX`, about
`3.40e38` — the picture is already nowhere, so a badge would name a
leg nobody could have seen.

**That is false for the authored producer, and the row's own witness
shows it.** The seam refuses PER LEG, not per lane. A profile whose
corners are `[0,0]`, `[1,0]`, `[7e307,0]`, `[0,1]` — and `7e307` is a
number the add-profile form takes, because it is a number — draws as
the two legs between its ordinary corners and a gap where the other
two were. At a camera framed on the ordinary corners that is a picture
a person is looking at, with a leg missing from it, and a missing leg
in an outline reads as an authoring mistake rather than as a number
too large to show. The case is executed at
`crates/viewer/src/marks.rs`'s
`a_lane_draws_the_legs_it_can_and_counts_the_ones_it_cannot` and at
`pane::viewport`'s `an_authored_loop_with_one_far_corner_draws_its_other_legs`.

The datum-lane rule already has the shape to copy: `datums_vanished`
is a count recomputed every frame, written back by the frame entry
point whether or not the viewport drew, and badged.

## What this half has to decide and build

- The field. `ViewerApp` needs the count with `datums_vanished`'
  discipline — zeroed by the frame entry point before the panes draw
  and assigned back unconditionally after — so a viewport tabbed away
  reports none rather than leaving the last frame's count standing.
  `work/vseam/projection-fault-has-no-sweeper.md` is the record of
  what happens to app-gated display state without that.
- The word. A count, not a name: the lanes do not know which edge or
  which corner, and `LegLane::undrawn` is a sum over a frame.
- Whether the EDGE lanes are in it. `marks::edge_id_segments` drops
  the count on the floor for the selected and hovered marks; the
  viewport's own three lanes keep it. Badging one and not the other
  would be a sentence that is true of some of the picture.

## Not in scope here

The pane's own extent refusing at the same seam. `viewport_ui` returns
with no paint callback when `[width_px, height_px]` or
`pixels_per_point` does not narrow, and says nothing — deliberately,
and the reason is written at the site: `aspect()` has already declined
an extent that is not positive and finite, and what is left is a
window above `3.4e38` physical pixels, which no person can produce. A
badge there would name a state nobody can reach.

## Fence

`crates/viewer/src/frame.rs` and `crates/viewer/src/app.rs` — the word
and the field. The state and its door
(`crates/viewer/src/marks.rs`, `crates/viewer/src/pane/viewport.rs`)
are VGEOM's and are already built.
