---
id: contribution-bounds-via-dual-interval
kind: issue
title: E5 contribution bounds from Dual<Interval> derivative enclosures (M10-4 deviation 3)
status: open
opened: 2026-09-03
refs: [M10-4, M10-5]
---

`work/m10/plan.md`'s M10-4 entry names "`Dual<Interval>` enclosures
consumed for contribution bounds and E7 pruning only"; M10-4 shipped the
seed+box composition at `Dual<Interval>` (pinned at the door:
`m10_4_seed::seed_and_box_compose_exactly_at_dual_interval`) but no
report column consumes the tangent enclosure — `PerParam::contribution`
is the `Dual64` linearization at the nominal, and `PerParam::chamber_span`
the same product over the certified leaf's own half-width. M10-5's
entry names only E7 pruning, so the contribution-BOUND deliverable has
no home; this item is it.

What is owed: a per-entry bound `sup |∂m/∂pᵢ|` over the nominal's
certified leaf, read off a `Dual<Interval>` pass over that leaf (value
channel the leaf box, tangent channel the seed — the door M10-4 opened),
multiplied by the leaf's half-width — a bound, where today's column is
a point value. Consumed for contribution only, never for refusal (E9;
DL1 unmoved). Blocked in practice by the certification widths
(`crates/editor-core/src/stackup.rs`, module docs; M10-3's headline): a
certified leaf is ε-scale today, so the bound would be over an ε-wide
span. Dispatch with M10-5's pruning consumer, which needs the same pass.
