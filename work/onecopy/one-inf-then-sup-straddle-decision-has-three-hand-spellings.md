---
id: one-inf-then-sup-straddle-decision-has-three-hand-spellings
kind: issue
title: One inf-then-sup straddle decision has three hand spellings
status: open
opened: 2026-11-16
priority: P2
cost: M
---


Found by #6780's review (the headline). Not fixed there.

## The finding

One decision is spelled by hand three times: read a quantity at a lower
bound (an inf-metered margin, or a flagged raw value), act on a decided
side there, read a Zero again at the upper bound (the sup), and escalate
on both bounds through `k_stats::straddle` where the two disagree:
- `topo/src/boolean/slit.rs:2835` `parting_order` (#6780): the order at
  the inf, or flagged raw (F45) where a spline's floor is not decided
  positive, then one point at the sup, then `bool_parting_straddle`;
- `geom-brep/src/pair_section.rs:2993` `within_span` (#6767): the
  overshoot past the span at `GapMeter::Clear`, the desync at
  `GapMeter::Within`, then `section_site_in_span_past_straddle`;
- `topo/src/validate.rs:534` `decide_agreed`.

**Its home** is `geom_core::k_stats`, beside `straddle`: one door that
takes the two readings and the acted-on sides, with an enum for the
lower bound that covers a `Margin` and a flagged raw value (row id
carried). The three sites would then read it.

**The wider class to sort.** These are straddle sites that may or may
not be the same shape. Each wants a look: either it reads the door, or
it states why it does not.
- `geom-brep/src/curve_pair/span.rs:537`, `:556` and `:621` (the span's
  past, end and run straddles);
- `geom-brep/src/pcurve_cache/spline_incidence.rs:613`;
- `geom-brep/src/pair_section/gap.rs:546`;
- `geom-brep/src/pair_section/ruling.rs:841`;
- `sweep/src/blend/mod.rs:688` (blend's `straddle` wrapper).

## Notes from the same review, not fixed in #6780

- **(4) `cuts` may now refuse where it skipped.** `slit.rs:1194` `cuts`
  calls `slot_from_far_end` and drops only `EdgeInFace`. A
  `parting_order` straddle comes back as `Escalated`, which `cuts`
  propagates where the old sup-only reading ordered. No ci reading
  reaches it, because all 112 probed readings are on lines. It wants a
  look at whether `cuts` should treat an undecided order as it treats an
  in-band one.
- **(6) Mixed units in the F45 straddle's margin.** On the flagged arm,
  the lower bound is the raw parameter difference and the upper is
  metres (the sup-metered reading). `straddle` hulls their two margins,
  so the reported margin mixes a parameter with a length. The door
  above should carry each bound's unit, or report them apart.
- **(7) `replace_face.rs:2762` reads Zero as forward.** `against`
  matches only `Sign::Negative` (F44), so an in-band span along the row
  reads as running forward. The row's span is certified forward on the
  old carrier, so a Zero there is a degenerate span the certification
  should already have refused. It wants an explicit arm, an invariant or
  a refusal, rather than a silent forward.
