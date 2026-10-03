---
id: derivation-ceiling-asserts-print-percentages-to-five-decimals
kind: issue
title: tess-meter derivations.rs: bare <= ceilings print both sides as percentages to 4-5 decimal places, which cannot show a last-bit red
status: open
opened: 2026-09-18
---


## Finding

Found by the TESS lane that made `crates/mesh`'s domination asserts legible
(`tess/domination-assert-messages`), sweeping for the same shape outside its
fence: a **bare** `<=` between a derived quantity and its ceiling, whose
message prints both sides at a handful of digits, so a red decided by the
last bits prints two equal numbers. `work/tess/nurbs-face-bound-unsound-on-a-random-rational.md`'s
"CORRECTION" section records what that cost on a `{:.3e}` message.

Hits, in `tools/tess-meter/tests/derivations.rs`:

- the "Claim 2, the family-free half" assert, `closed <= unfloored_ceiling()`
  (`:691`) — prints `{:.5}%` / `{:.5}%`;
- `the_shipped_sample_count_is_the_smallest_whose_envelope_fits`,
  `here <= growth_margin()` (`:830`) — prints `{:.4}%` / `{:.4}%`; its
  sibling `coarser > growth_margin()` (`:839`) is the same shape in the other
  direction.

These compare closed-form figures, not a fuzzed sample, so a last-bit red is
far less likely than on the mesh row — it would take a libm or constant
change landing a figure ON its ceiling. The cost of the repair is one format
spec per row (`{:.17e}` beside the percentage, which is worth keeping for the
reader), which is why it is filed rather than argued.

"Five decimals" is the format spec (`{:.4}%` / `{:.5}%`), not a site count:
three asserts are named above.

The sweep that found these read only one class of assert outside
`crates/mesh`; the unread remainder and the extraction's blind spot are
`work/tint/ordering-asserts-outside-mesh-unswept-for-illegible-domination-messages.md`.
