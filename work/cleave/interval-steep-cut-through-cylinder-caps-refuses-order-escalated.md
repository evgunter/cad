---
id: interval-steep-cut-through-cylinder-caps-refuses-order-escalated
kind: issue
title: plane_section at Interval refuses OrderEscalated (split_join_order_u, enclosure ~±7e-15) on a steep cut through a cylinder's caps that f64 answers
status: open
opened: 2026-10-02
priority: P3
cost: E
---


Seen by PR 3877's fix pass (section arcs), not chased. A steep cut
through a cylinder's caps (the first fixture tried for
`plane_section_area_of_an_uncancelled_arc_at_f64_and_interval`) answers
at f64; built and cut at `Interval`, `plane_section` refuses
`OrderEscalated` on `split_join_order_u` with an enclosure of about
±7e-15, also with the seams turned off the mirror plane. Measure first:
whether the two crossings are distinct in truth (an over-wide enclosure
on the join order's `u` key) or coincide (an honest refusal).
