---
id: derivation-ceiling-asserts-print-percent-at-five-places
kind: issue
title: tess-meter derivations.rs: bare <= ceilings print both sides as {:.4}%/{:.5}%, so a last-bit red prints two equal percentages
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

Blind spot of the sweep that found these: see
`work/tint/enclosure-asserts-print-too-few-digits-to-show-a-last-bit-red.md`,
"What the sweep could not see" — in particular, outside `crates/mesh` only
asserts carrying an explicit sub-17-digit format spec were read.
