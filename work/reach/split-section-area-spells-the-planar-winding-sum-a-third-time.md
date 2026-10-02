---
id: split-section-area-spells-the-planar-winding-sum-a-third-time
kind: issue
title: the splitter's certify_section_area spells the Newell + conic-bulge area sum a third time, apart from crate::loop_winding
status: open
opened: 2026-10-02
priority: P3
cost: E
refs: [unclaimed-half-edge-read-as-a-minus-half-in-reach]
---

Found by JOIN's `ring-run-winding-is-a-second-spelling-of-the-loop-winding-sum`
sweep (branch `join/ring-run-winding`), which gave the loop winding and
the boolean join's open-run winding one home in
`crates/topo/src/loop_winding.rs` (`Body::planar_run_winding_decided`,
`Body::planar_loop_winding_decided`, both over one private
`winding_of_halves`).

## What

`crates/topo/src/splitting/join.rs`, `certify_section_area` (the
`split_section_area` predicate), accumulates the same functional over
a completed section polygon's below loop — the chord Newell sum about
the first vertex, plus a per-conic-edge correction, metered by the
perimeter — in its own arithmetic:

- **The bulge** is spelled from the carrier's centre,
  `[(c − O)×(B − A)]·n̂ + s_a·s_b·(axis·n̂)·Δt − [(A − O)×(B − O)]·n̂`,
  where `loop_winding::conic_segment_term` spells it
  `axis · s_a·s_b·(Δ − sin Δ)`. Algebraically the same area; not the
  same statement, so a fix to one (e.g. the ellipse lever, which
  `conic_segment_term` reads as the larger semi-axis MAGNITUDE and
  this site reads as `s_a·|Δt|`, a signed `major`) does not reach the
  other.
- **The carrier set**: a spiric or NURBS edge is skipped (`continue`),
  i.e. wound by its chord. Unreachable today — the split operand gate
  (`splitting/classify.rs`) refuses both kinds — but it is the drift
  the join row closed.
- **The claim**: `forward = edge.he_plus == he`, already listed in
  `unclaimed-half-edge-read-as-a-minus-half-in-reach`.

The predicate differs (it certifies `|2A|/P` rather than deciding a
sign), so the repair is to read the signed sum from the one home and
take its magnitude — not to delete the predicate.
