---
id: the-at-rest-gate-certifies-no-embedding-so-adjacent-faces-crossing-away-from-shared-cells-pass
kind: issue
title: The census passes a body whose adjacent faces of one sphere overlap (FLUX's witness A), so tier 3′ certifies no embedding there
status: open
opened: 2026-10-21
refs: [a-finished-body-is-gated-at-tier-3-alone-so-a-body-the-census-refuses-finishes, a-seam-region-whose-boundary-carries-both-poles-is-unread, SHELL-3]
priority: P2
cost: M
design: true
---


Found by FLUSH's fin-crown unit (PR 5534) and its review. Filed here
because the census is RESTREAD's.

## What passes

FLUX's seam-region witness A
(`crates/sweep/tests/a_seam_region_carrying_both_poles.rs`,
`two_faces_whose_region_carries_both_poles_flip_silently`): a y-poled
ball unioned with a brick keeps one 180° lune, which `mef` splits along
a meridian. Both halves are then flipped, so the two faces of one sphere
each claim their complement and overlap. The body passes tier 3, and,
by the PR 5534 review (`flush/fin-crown-fixture-review` @ `ee3e585c3`),
tier 3′ with no contacts too. It measures 48 − 2π against the true
48 + 2π/3.

The census does catch the other self-crossing witnesses:
- the fin crown on its old bottom, as two `EdgeFacePierce`;
- FLUX's witness B (the 1e-12 dimple), as `CensusEscalated` on
  `curve_carrier_gap`.

Those pass only because the at-rest gate runs no census
(`a-finished-body-is-gated-at-tier-3-alone-so-a-body-the-census-refuses-finishes`).
Witness A's two faces share a whole boundary arc and overlap away from
it, on one carrier. Tier 3's own list of what it does not check opens
with **global self-intersection / minimum clearance**
(`validate.rs`, "What tier 3 does NOT yet check"), deferred to the
interval clearance engine, whose body half is still in `editor-core`
(`work/clear/SHELL-3.md`).

## What to find

Where the overlap of two faces on one carrier, away from the cells they
share, is decided, and so refused:
- a census lane;
- a CLEAR/SHELL embedding check once the clearance engine is in
  `topo`;
- or FLUX's seam-region read (check 6), which would refuse witness A's
  flipped region directly.

The home is open.
