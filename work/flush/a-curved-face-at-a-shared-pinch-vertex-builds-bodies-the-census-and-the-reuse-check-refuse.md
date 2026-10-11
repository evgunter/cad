---
id: a-curved-face-at-a-shared-pinch-vertex-builds-bodies-the-census-and-the-reuse-check-refuse
kind: issue
title: A curved face at a shared pinch vertex builds bodies with an exact volume that tier 3' escalates and a further union refuses VolumeUncertified (98 lines PR 4249 moved from refusal)
status: closed
opened: 2026-10-07
closed: 2026-10-11
branch: flush/curved-pinch-reread
pr: 4698
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

## Measured (2026-10-11, main `6e3985cb7`)

The probe is ported as `sweep`
`curved_notch_pinch::curved_notch_pinch` (ignored,
release, plain booleans). It names each refusal by its variant. On
`875e049a` it reproduces this row's numbers exactly (873 `OK BAD`, 461
`CensusUndecidable`, 33 mesh refusals).

| reading | `875e049a` | `6e3985cb7` |
|---|---|---|
| sound and meshes | 468 | 472 |
| t3′ ok, reuse `VolumeUncertified` | 402 | 469 |
| t3′ `CensusUndecidable`, reuse `VolumeUncertified` | 436 | 511 |
| t3′ `CensusUndecidable`, reuse ok | 0 | 2 |
| volume, t2 and cert ok, mesh refuses at δ 0.05 | 33 | 37 |
| volume off the oracle (one pose) | 2 | 6 |
| `ClassificationInvariant` | 106 | 0 |
| `SharedVertexCrossings` | 54 | 0 |
| `JoinDesync` | 11 | 15 |

No body line moved. CONTACTHOLD's PRs 4598 and 4604 changed none of
the 461. What moved is 160 refusals, 156 of which now build (PR
4249's hang and what followed); 4 went on to `JoinDesync`.

**The bodies are right.** Every one of the 1 497 has the oracle's
volume and passes tier 2 and the certificate. None has a definite
tier-3′ finding. Every one holds one vertex per cone at `v`
(`cone_finding`, read on every line). The 6
off-volume lines are the pose `0 4 4` the row already read: the
kernel's `U + I` sums to `vA + vB` at 9 decimals (130.945654093 +
0.919232683 = 131.864886776), so the oracle's `common` is off by
1.98e-7.

What fails is three lanes outside the boolean, each already a row:
- **Reuse (1 023 lines)**: `at_infinity_side`'s closed-form props
  refuses the bowed wall where a cube face cuts it along an ellipse
  arc (`NotIsoRectangle`, instrumented on three poses). RESTREAD's
  `at-infinity-probe-measures-in-closed-form-only`.
- **Census (542 lines)**: the result is two solids meeting only at
  the pinch, with one vertex–vertex record. The cross-solid backstop
  answers `CurvedWithinReach` for the faces through `v` (the
  cylinder wall, or the arc-bounded caps, against the other solid's
  planes). They touch there, so no gap separates them. RESTREAD's
  `census-cross-solid-curved-pairs-undecidable-on-shell-results`,
  whose JOIN-pinch section names this class.
- **Mesh (37 lines)**: thin planar slivers between the arc and a
  straight cut, two of them lenses cornered at `v`. TESS's
  `a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta`.

## Closed

Each population's evidence and replay command went onto its owning
row in the same PR. `a_curved_notch_at_a_corner_pinch_builds_bodies_two_lanes_cannot_read_yet`
pins one pose per population: the body's volume, tier 2, the
certificate and no definite tier-3′ finding, plus each lane's current
answer, which flips when that lane lands. The `JoinDesync` refusals,
which are the boolean's own, are
`a-curved-notch-at-a-corner-pinch-refuses-join-desync-on-a-section-vertexs-null-edge-copies`.
