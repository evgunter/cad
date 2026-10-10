---
id: a-curved-face-at-a-shared-pinch-vertex-builds-bodies-the-census-and-the-reuse-check-refuse
kind: issue
title: A curved face at a shared pinch vertex builds bodies with an exact volume that tier 3' escalates and a further union refuses VolumeUncertified (98 lines PR 4249 moved from refusal)
status: open
opened: 2026-10-07
priority: P2
cost: M
refs: [4249]
---


## What

Found by PR 4249's review r1 (MINOR-2), measured in its fix pass.

Probe: r1's `r1_curved_notch_pinch`
(`crates/sweep/tests/join_pierce_runs_sweep/review_r1_4249.rs` on
`join/pinch-cones-split-at-insertion-review-r1`):
- `notch343` with its face `v → (2, 1.15)` bowed into an arc, against
  the two-cube corner pinch of `pinch_runs_battery`;
- 84 directions × 3 turns, every op in both orders;
- the oracle is Richardson-extrapolated over 48 and 96 chords.

**On main `875e049a`**, 873 of its 1 512 lines are already `OK BAD`.
Volume is exact, and t2 and the certificate pass. Every one fails the
reuse check, which unites the result with a far brick: that union
refuses `Containment(VolumeUncertified)`. 461 of them also fail tier 3′
with `CensusUndecidable` "a curved face of one is within reach of the
other", and 33 refuse to mesh at δ = 0.05 (`Triangulation`, or one
`MissingEntity`, a degenerate trimmed boundary).

**PR 4249 adds 98 refusals to that population.** Its hang
(`insert::hang_in_turned`) builds the shared corner where main refused
`ClassificationInvariant`. All 98 have:
- volume matching the oracle (4 lines, `i=0 j=4 k=4 ab U/S, ba U/S`,
  off by 1.98e-7: the oracle's, as main's `ab I` there is off by as
  much and `U + I` sums to `vA + vB` at 9 decimals), and t2 and the
  certificate passing;
- one vertex per cone at `v`, on one point key;
- no face through two vertices at `v`.

What fails, with no definite tier-3′ failure among them:

| lines | tier 3′ | reuse (far-brick union) | mesh |
|---|---|---|---|
| 52 | passes | `VolumeUncertified` | meshes |
| 42 | `CensusUndecidable` | `VolumeUncertified` | meshes |
| 2 | `CensusUndecidable` | passes | meshes |
| 1 | `CensusUndecidable` | `VolumeUncertified` | `Triangulation` |
| 1 | passes | `VolumeUncertified` | `Triangulation` |

The two census-only lines, `i=9 j=4 k=4 ab S` and `i=9 j=5 k=4 ab S`,
are a pattern main's unmoved lines do not show. Elsewhere in the same
probe, 4 lines go to `SOUND` and 4 to `JoinDesync` "null-edge copies
have not exactly one kept end".

## Open

Whether these bodies are right is the census's and the containment
lane's question on curved bodies, not the hang's. The planar batteries'
shared vertices build `SOUND` with the hang. Read the population with
the curved census and `VolumeUncertified` lanes, the moved 98 alongside.
