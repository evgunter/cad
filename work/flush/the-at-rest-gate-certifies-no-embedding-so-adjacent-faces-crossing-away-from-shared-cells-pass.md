---
id: the-at-rest-gate-certifies-no-embedding-so-adjacent-faces-crossing-away-from-shared-cells-pass
kind: issue
title: The at-rest gate certifies no embedding, so adjacent faces that cross away from the cells they share pass
status: open
opened: 2026-10-21
refs: [a-fin-crown-at-rest-holds-a-region-winding-minus-one-that-check-7-and-the-census-pass, a-seam-region-whose-boundary-carries-both-poles-is-unread, SHELL-3]
priority: P2
cost: M
design: true
---


Found by FLUSH's fin-crown unit (PR 5534). The every-aim check 7 that
unit tried (PR 5520) sampled this class and missed most of it, and it
was dropped: on an embedded boundary whose faces are oriented
coherently, one aimed ray reads what every aim reads.

## What passes

The at-rest gate certifies no embedding. The census leaves out the
pairs of a body's own faces that share cells, away from the cells they
share, so two adjacent faces that cross elsewhere are never compared.
Tier 3 names the gap itself: `validate.rs`'s "What tier 3 does NOT yet
check" list opens with **global self-intersection / minimum clearance**,
which is deferred to an interval whole-body check. The clearance
engine's body half is still in `editor-core` (`work/clear/SHELL-3.md`),
and the gate does not run it.

Witnesses, each passing the gate:
- **The fin crown on its old bottom.**
  `test_support_meeting::crown_body(&fin(), [0.0, 0.2, -0.6], pose, tol)`.
  The bottom fan's face on the long edge `c0 c1` passes through the far
  fin face and the top face beyond it. The exact segment–triangle test
  in `crates/topo/tests/the_fin_crown_winds_as_a_solid.rs` finds the
  crossing at every pose, and `posed_crown_kept` passes the crown.
- **FLUX's seam-region witness A.** Both halves of a split lune are
  flipped, so the faces of one sphere overlap. It passes tier 3 and
  tier 3′ with no contacts, and measures 48 − 2π against the true
  48 + 2π/3
  (`crates/sweep/tests/a_seam_region_carrying_both_poles.rs`).
- **FLUX's witness B, the 1e-12 dimple.** The same overlap in band. It
  passed even the every-aim reading.

## What to find

Where an embedding check lives, and what it reads. The home is open:
- a census lane over a solid's own adjacent face pairs away from their
  shared cells;
- or a CLEAR/SHELL embedding check once the clearance engine's body
  half is in `topo` (SHELL-3).

Both must stay sound at the band, deciding a crossing away from shared
cells and refusing in band.
