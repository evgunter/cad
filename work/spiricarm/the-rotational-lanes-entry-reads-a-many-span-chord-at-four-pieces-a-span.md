---
id: the-rotational-lanes-entry-reads-a-many-span-chord-at-four-pieces-a-span
kind: issue
title: The rotational lane reads a projected chord's length and turn at four pieces a span before its rounds, now the larger part of a face's measurement
status: open
opened: 2026-11-10
priority: P3
cost: E
refs: [a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses]
---

## What

Filed by the cylinder-pair cost lane (branch
`spiricarm/cylinder-pair-cost`, 2026-11-10; rewritten in its fix pass,
2026-11-11). The lane's rounds now read each span of a projected
carrier in the pieces its own turn and length need
(`geom_brep::props::quad::rotational::span_schedule`), so a 625-span
chord's round 0 is one piece a span. Two entry reads remain before the
rounds (`edge_entry`):

- **The metric length** (`projected_metric_length`) still reads
  `PROJECTED_ENTRY_PIECES` = 4 pieces a span, whatever the schedule: an
  upper bound at any count, but it pads every round
  (`RotationalChartQ::residual`, `chart_term_pad`), so a coarser read
  widens every enclosure. On a 625-span chord it is 2,500 rate reads.
- **The turn** (`entry_turn`) starts at the schedule and multiplies
  every span's pieces by four while the turn's enclosure is an eighth of
  a turn wide or more, to at most `MOST_SPAN_PIECES` = 1024 a span. A
  chord that passes by the axis can therefore read up to 1024 pieces on
  every span, the short ones included, since the factor is the edge's.

The question for each: read it per span (the length to the schedule's
pieces, with a row that the pads do not widen past what the rounds
remove; the turn refining only the spans whose own turn is wide), and
measure what that saves on the fenced cylinder pair, where these reads
were 0.2–0.7 s of a face's call before the schedule.
