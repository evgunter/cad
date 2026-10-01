---
id: arc-window-membership-has-three-spellings
kind: issue
title: geom/sweep/topo: 'does this circle window hold angle phi' is spelled three times, and whole-circle ring metering has a home beside the per-piece one
status: open
opened: 2026-09-26
priority: P1
cost: D
---


## Finding

**One question, three spellings.** "Does this circle window hold the
angle `φ`" is answered in three places, each with its own argument for
its own soundness direction:

- `sweep`'s `CircleFrame::misses` (`crates/sweep/src/blend/surgery.rs`)
  — a bracket read on the angle of `q` past the window's start, read in
  `(0, τ]` (`CircleFrame::past`), selecting between an arc's end values
  and the whole circle's extreme;
- `geom`'s `angle_interval_in_span` (`crates/geom/src/curves/boxes.rs`)
  — an `f64` interval-translate test with `ANGLE_SLOP` widening,
  conservative-inclusive, for the curve boxes;
- `topo`'s `point_on_arc` (`crates/topo/src/boolean/contain.rs`) — the
  cosine-window construction, metered (`bool_contact_arc_span`,
  `bool_contact_arc`).

Three spellings of one geometric fact means three places a periodicity
or branch-cut slip can hide, each checked by its own rows only.

**Two homes for metering a ring against a circle.** In
`ring_clearance_pass` a support ring is read as a WHOLE circle
(`ring_circle` → `circle_margins` → `CircleMargins`), while a boundary
edge — on a ladder host's outer cycle or on a ruled cut-off's cap — is
read as its carrier over its stored window (`stored_piece` →
`piece_distance`). A ring is a cycle of pieces too; the whole-circle
home is a second answer to the question the per-piece home already
answers, kept because a ring happened to be one closed circle edge
when it was written.

**`point_at` has gone** (this row's third item): the piece meters read
a circle's point through `CircleFrame::at` (`Curve3::circle_at`, the
door `Curve3::eval`'s circle arm calls) and a line's as
`origin + dir·t` inline. Calling `Curve3::eval` itself is not a
one-liner there: it needs `SpanLocate`, which the meters' sole
`Bounds` bound does not carry, so it would put the compound bound on
three more signatures.

Found by review of the ruled cut-off cap meter (PR 3271).

## What the taker owes

Pick one home for arc-window membership and route the other two
through it — or state, at each, why its soundness direction cannot be
the shared one's. Retire `CircleMargins`/`ring_circle` in favour of
metering a ring's pieces through `piece_distance`, if the containment
reading (`other_inside_trim`) can be put as a per-piece question; if it
cannot, say so beside `CircleMargins`.

## Findings (band/arc-window-one-home)

**The three sites are two questions, not one.**

- *May this angle bracket lie in this periodic window?* — read by a
  caller that SELECTS between two sound bounds (an arc's end values,
  or the whole circle's extreme), where an uncertain answer must
  loosen. `sweep`'s `CircleFrame::misses` and `geom`'s curve boxes
  (`circle_arc_aabb`, `ellipse_arc_aabb`, through `axis_extremum`) ask
  it. One home now: `geom::angle_window_may_hold`
  (`crates/geom/src/param.rs`); the private `angle_interval_in_span`
  is gone and `misses` calls the home. The boxes' `ANGLE_SLOP` stays
  the boxes' own widening of the window (their arithmetic is not
  outward through `libm::atan2`); `misses` asks with its brackets as
  they stand, because its two candidates agree at the switch.
- *Is this point inside, outside or in band of the window?* — a
  DECIDED membership, metered in metres, escalating in band. `topo`
  holds it, and the row's citation was stale: there is no
  `point_on_arc` in `contain.rs` (that file has `point_on_circle`, the
  carrier half). The angular half is `splitting::containment::arc_trim`
  (chordal distances, the pre-pass's `bool_contact_arc_{end,trim}`
  rows) and `boolean::solid_contain::chart_azimuth_margin` (the cosine
  window, with its own site roster). Each now says, at its doc, why it
  is not the selection home's question.

**`CircleMargins`/`ring_circle` stay**, and say why beside
`CircleMargins`: `other_inside_trim` is per-piece (`si` less the
largest piece `far`), but `external` is not — a piece's `near` is
unsigned, so `near − si` reads a ring that ENCLOSES the trim circle as
clear where `external` reads the overlap. The trim's side of a ring is
a property of the closed cycle.
