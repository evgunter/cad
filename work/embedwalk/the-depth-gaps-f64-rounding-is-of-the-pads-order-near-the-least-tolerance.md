---
id: the-depth-gaps-f64-rounding-is-of-the-pads-order-near-the-least-tolerance
kind: issue
title: The depth gap is read at f64 with no rounding charged, and near the least tolerance its rounding is of the same order as the pads that stand between it and the verdict: measured safe-sided, not proved, for the census and the certificate alike
status: open
opened: 2026-11-18
priority: P2
cost: M
design: true
---


Measured by the review of PR 7119 (EMBED, 2026-11-18) and filed by
lane `embed/nested-tori`.

## What reads it

The depth gap (`topo::census::depth_gap`): one face's least distance
from an axis, a centre or a core circle, less the other's greatest,
off the two faces' covers, less both faces' pads. Two readers grant on
it: the census's arm 1 clears a cross-solid pair, and since PR 7119
the clearance engine discharges a face pair's root, so the embedding
certificate and every metric clearance bound rest on it.

## The design, as it stands

A cover's bound at `f64` is the real-arithmetic value computed as if
exact, with no rounding charged (`[one-way.readings]`: "as real
numbers at `f64`, with no rounding charged"; `[simple.conditioned]`
forbids a site charging it). So reading the gap at the lane scalar is
the design, not a deviation of either reader.

## What was measured

- To part two faces nearer than `K·ε`, the `f64` gap must round UP by
  more than the pads, `2·sweep_pad = 2.4·K·ε`: `2.4e-8` at the default
  ε, and `4.8e-12` near the least tolerance (`ε_min ≈ 2e-13`).
- The `f64` gap of two nested tori against the true gap, over 80,000
  frames (`geom`'s
  `depth_gap_rounding::a_nested_tori_depth_gap_at_f64_is_never_over_the_true_gap`,
  the review's probe kept as a row at 20,000): never over the truth,
  and up to `1.1e-14` under it at unit size, `2.7e-12` under at the
  session box's edge (`R = 400`), `1.8e-9` under at `R = 1e6`.
- 600 trimmed cylinder wall pairs in two bodies
  (`topo`'s `clearance::depth_rows::no_wall_pair_is_parted_in_depth_past_its_sampled_distance`):
  none parted past its sampled least distance.
- `census::depth_gap_fuzz`'s tolerance is `1e-11` at the box's edge,
  larger than the pad near `ε_min`, so the standing register would not
  show a rounding of that size.

So at the default ε there are four orders of room. Near `ε_min` the
reading's rounding is of the pad's own order; it fell on the safe side
in the family measured, and that is a measurement, not a proof.

## The question

Whether a reader that grants on the depth gap (the census, the
certificate, a clearance bound) is to be held to more than the design
gives it: an enclosure of the gap at the interval scalar, a rounding
charge at this one site, or a floor on the tolerance below which the
reading refuses. It is one question for both readers, and it changes
`[simple.conditioned]`'s rule at a site, so it is Ev's. Not a defect
of PR 7119.
