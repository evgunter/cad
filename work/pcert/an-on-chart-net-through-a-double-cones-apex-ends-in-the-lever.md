---
id: an-on-chart-net-through-a-double-cones-apex-ends-in-the-lever
kind: issue
title: A net on a double cone that crosses its apex ends in the lever, not certified, at the bounded read
status: open
opened: 2026-11-11
priority: P3
cost: M
---


Filed by `pcert/analytic-net-bounded` (PR 6826) from its review's
probe (`review/6826-full` at `cf7c0cc1`,
`pcurve_cache/review_6826_probes.rs`, `review_6826_a_ruling_through_the_apex`).

## What

A straight degree-1 net along a ruling of a cone through its apex, read
over a span that crosses from one nappe to the other (`[-1, 1]`,
`[-0.2, 1]`, `[-1, 0.2]` in the rows), lies on the double cone exactly.
`pcurve_cache::read_net_bound` does not show it on. It ends
`Incidence(Undecided)`, the lever, at `f64` and `Interval`
(`analytic_net_rows::a_ruling_through_a_cones_apex_is_never_a_defect_*`).
So a stated net through a cone's apex gets no edge certificate.

## Why

Limb 2's reader (`limb2_offset_sup`) reads one nappe, the one
`projected::face_locus` picks at the window's middle. The parts on the
other nappe have no positive lever, and the apex reading
(`Meters::apex_part`) does not bound them. The composite either refuses
the bound (`NoOffsetBound`) or stalls (`KernelLimit` at a point scalar).
Both now end as a logged straddle in the lever. Before PR 6826's fix
OO1, `NoOffsetBound` ended as a poisoned margin, a defect.

That ending is honest: no certificate is issued for a net the read does
not bound. Here it is a completeness hole. The apex reading is not the
face's own bound, though: `√|f|` bounds the distance from the DOUBLE
cone, so a part on the other nappe whose lever refuses can read small
and clear, which is a soundness question for the face reading
(`a-face-limb-two-reading-clears-a-carrier-on-the-other-nappe`). The same net
cut at the apex reads on, nappe by nappe
(`analytic_net_rows::a_net_beside_a_cones_apex_ends_off_*`, the on
cases).

## Fix

Read a window that crosses the apex as two windows, cut at the
parameter where the carrier meets the apex's plane (or at the nearest
knot beside it). Each nappe is then read by its own lever, and the two
are joined as the larger bound. Alternatively, bound the double cone's
`|ρ cos α − |z| sin α|` across the nappes in one composite.

No product path is known to state a net through a cone's apex: a
revolve's apex edges are lines.


## 2026-11-21 — the double-cone reading exists (SECTCONE)

`sectcone/one-limb-two-reader` gave the reader a nappe-free cone arm:
`projected::Meters::Cone` at `projected::Locus::Carrier`, the lever
`ρ cos α + |z|_min sin α` with the lesser of it and `√|f|` taken
(`Meters::certified`), read by the search through
`pcurve_cache::limb2_offset_sup` at `Limb2Locus::Carrier`
(`ssi::certify::one_reader_rows::a_search_reads_a_cone_as_the_double_cone`
reads a ruling through the apex on). The edge door still reads the
face's nappe (`limb2_offset_sup`), so this row's net still ends in the
lever there; the fix's second alternative is now that arm.
