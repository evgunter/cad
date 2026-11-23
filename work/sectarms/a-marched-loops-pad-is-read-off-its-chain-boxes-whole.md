---
id: a-marched-loops-pad-is-read-off-its-chain-boxes-whole
kind: issue
title: A marched loop's pad is read off its chain boxes whole, each against its own span's carrier points
status: open
opened: 2026-11-23
priority: P3
cost: M
---


Filed by the `sectarms/rim-pad-cells` lane: the later step Ev's
decision on PR 7045 names ("the marched-loop door's reader is a later
step", fork-log row 280).

## What

A marched loop's pad (`pair_section::Tube::of`, `pad_and_reach`) is the
farthest its certificate's chain boxes, each padded by the tube radius,
stand from the carrier, each box read against nine carrier points over
its own span. It consults neither partner, so it is at least the tube's
radius, and a box read against its own span's points reads wider on a
short span. A fitted spline × implicit rim arc was read the same way
until its pad moved to its cells
(`pair_section::spline_implicit::cells`): the tube's pieces less what
the partner's form clears of the band, each read against the whole
carrier, refined until the section's arcs read apart.

No refusal is on record at this door, and it holds no pair of loops
apart: two marched loops whose pads meet are read by whichever a site
falls within, and a site within both desyncs
(`CarriedSection::site`).

## Remedy

The same reader on the marched loop's chain: cells of the ℝ³ boxes that
both surfaces' forms do not clear, read against the whole carrier, and
the loops of one section held apart on them.

## Done when

Two marched loops closer than their tube radii are carried, each point
reading on its own loop, or the pair refuses by name.
