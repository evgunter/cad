---
id: a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop
kind: issue
title: A union of a block resting a Zero-band gap above another refuses ResultVolumeImplausible: the glue closes the gap and the backstop has no slack for it
status: dispatched
opened: 2026-10-13
priority: P0
cost: M
branch: cleave/volume-relations-a-test-oracle
---


Found by FLUSH's given-gate lane (`flush/given-gate`, 2026-10-13) while
measuring a pair boolean's inputs. Measured on main `3ad2c1bf`.

## Ruled (Ev, `[ev]` PR 5780, 2026-10-27)

The boolean's volume relations (vol(A ∪ B) ≤ vol(A) + vol(B) and its
siblings) become a test oracle, checked on the suite's fixtures, not a
check every operation runs. Ev: "(b) sounds good!", after the record
showed one recorded catch, which an operand gate caught better, against
about a dozen open rows refusing correct bodies. That removes this
refusal; no B′ is built and the glued boundary is not re-derived
("tidier but strays more from the original"). DESIGN.md
`[simple.checks]`: no representation, stage or decision exists only so
a check can read it. PR 5757's volume allowance is closed unmerged.

## What

`topo::union` of `A = [0,1]³` and a brick whose bottom rests a positive
gap `g` above `A`'s top, `g` inside the Zero band, refuses:

```
ResultVolumeImplausible { which: "vol(A ∪ B) ≤ vol(A) + vol(B)", got: "2.0", bound: "1.9999999999999" }
```

| `g` | B's footprint `[0.25,0.75]²` | `[0,1]²` | `[-0.5,0.5]×[0.25,0.75]` |
|---|---|---|---|
| 0 | Ok | Ok | Ok |
| 1e-13 … 5e-10 | refuses | refuses | refuses |
| −1e-12, −1e-10 (overlap) | Ok | Ok | Ok |

PR 5002's review reproduced the table on the `[0,1]²` footprint and
added finer cells there (B's bottom placed at `1.0` plus `n` ulp, or
at `1.0 + g`): 1 ulp builds; 4 ulp and `1e-15` refuse
(`cleave_zero_band_gap_review_probe.rs` on `flush/given-gate-review`
@ `e1abf0a4dd`). The review agrees the band is P0.

(`Tol::witness()`, band zero 1e-9; `brick` and `finished` from
`topo::test_support`.)

The rest glue decides the two planes one carrier at Zero and keeps
`A`'s description for the merged face, so `B` grows by `g × area`. The
result is then larger than `vol(A) + vol(B)` by exactly that, and the
union backstop (`volume_backstop` in `crates/topo/src/boolean/ops.rs`)
has no allowance for a Zero-decided displacement. A gap that small
comes from ordinary float placement, so this is a plain union of two
resting blocks refusing.

## Owed

The volume backstop leaves the boolean's operation path, and its
relations move into the test suite as an oracle over fixtures.
`BooleanError::ResultVolumeImplausible` and the backstop's other
refusal arms go with it, if nothing else raises them. The table above
then builds. Pin it over the table, and close or re-point every row
whose only defect is a backstop refusal.

## Evidence from FLUSH (PR 5008's review, 2026-10-14)

The same backstop refuses a pose whose pair glues with a Zero-band *tilt*, not only a Zero-band gap. Measured on `e0fed17b` (recorded in `work/flush/an-in-band-glue-verdict-is-refused-as-the-corner-arms-unglued-coincidence.md`, "Found beside it"):
- Standing wedge, block `[0,3] × [-2,2.5] × [0,1]`, wedge length 1.
- At ε 1e-9, θ 2e-12, every φ ∈ {2, 5, 20, 60, 90}° refuses `ResultVolumeImplausible` in both orders. For example, got 14.366025403784464 against bound 14.36602540378444 at 60°, and 14.500000000000034 against 14.5 at 90°.
- At ε 2e-11, θ 5e-12 it refuses the same way, with got minus bound 5e-15 to 8e-14.
- The glue closes the wedge face's Zero-band tilt onto the block's top, and `volume_backstop` (`ops.rs`) has no slack for the volume that closes. The fix the row owes covers both: the bound admits the displacement the glue's own Zero verdicts allow.
