---
id: the-torus-square-lever-row-reads-one-tilt-one-order
kind: issue
title: section_end_poses' torus-square lever row reads the clearly tilted face at one tilt, in one order, under ∪ and ∩ only
status: open
opened: 2026-11-24
priority: P3
cost: E
refs: [a-torus-near-a-tilted-cut-stops-at-the-extent-scan]
---


Filed by `orbit/tilted-torus-rim` (PR 7617).

`crates/sweep/tests/section_end_poses.rs`
`a_torus_section_read_square_refuses_as_the_inputs_and_its_lever_builds`
pinned the clearly tilted face (`CLEARLY_TILTED = 1e-2`) to
`GermSectionSpiric` in every op, both orders, at `±CLEARLY_TILTED`.
Once the section value marches that face, the 24 runs took 1023 s
(debug), so the row now reads, at each height `h ∈ {0, 0.2}`:

- kept: `+CLEARLY_TILTED`, order torus-first, `∪` and `∩`, each held to
  tier 3 and to its volume (the block's `64(h + 4)` and
  `germ_plane_torus_frame::torus_past_a_plane`);
- dropped: `−CLEARLY_TILTED` (every op), the block-first order (every
  op), and `torus ∖ block`, `block ∖ torus` in either order.

The square pose (`t = 0`) still runs every op in both orders.

Settles when the dropped runs are held to their volumes in the slow set
(the march is cached per operation, so a cheaper march or a split row
would carry them), or a row argues the kept ones stand for them.
