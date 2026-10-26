---
id: a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop
kind: issue
title: A union of a block resting a Zero-band gap above another refuses ResultVolumeImplausible: the glue closes the gap and the backstop has no slack for it
status: dispatched
opened: 2026-10-13
priority: P0
cost: M
branch: cleave/backstop-admits-the-glues-zero-band
pr: 5757
---


Found by FLUSH's given-gate lane (`flush/given-gate`, 2026-10-13) while
measuring a pair boolean's inputs. Measured on main `3ad2c1bf`.

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

The backstop's bound admits the displacement the glue's own Zero
verdicts allow (the band's zero width times the glued area, or the
same read off the verdict margins), or the glue keeps the volume it
closes; a pin over the table above.

## Evidence from FLUSH (PR 5008's review, 2026-10-14)

The same backstop refuses a pose whose pair glues with a Zero-band *tilt*, not only a Zero-band gap. Measured on `e0fed17b` (recorded in `work/flush/an-in-band-glue-verdict-is-refused-as-the-corner-arms-unglued-coincidence.md`, "Found beside it"):
- Standing wedge, block `[0,3] × [-2,2.5] × [0,1]`, wedge length 1.
- At ε 1e-9, θ 2e-12, every φ ∈ {2, 5, 20, 60, 90}° refuses `ResultVolumeImplausible` in both orders. For example, got 14.366025403784464 against bound 14.36602540378444 at 60°, and 14.500000000000034 against 14.5 at 90°.
- At ε 2e-11, θ 5e-12 it refuses the same way, with got minus bound 5e-15 to 8e-14.
- The glue closes the wedge face's Zero-band tilt onto the block's top, and `volume_backstop` (`ops.rs`) has no slack for the volume that closes. The fix the row owes covers both: the bound admits the displacement the glue's own Zero verdicts allow.

## Measured on main `bc060f24` (2026-10-25)

Measured at ε default, 1e-6 and 1e-12. Before the fix, only the union
refuses, and only in two footprints. In both orders it refuses
`vol(A ∪ B) ≤ vol(A) + vol(B)`:

- on `[0,1]²`, every gap from 2 ulp up to `0.99·zero`;
- on `[-0.5,0.5]×[0.25,0.75]`, every gap from 16 ulp (or `1e-13`) up
  to `0.99·zero`.

At the default ε and 1e-6 that includes the row's 1e-12 … 5e-10.
At 1e-12 it means the cells inside that band's zero.

The smaller gaps on that footprint build: 1, 2 and 4 ulp, and
`1e-15`. So do the overlaps and a zero gap.

The inside footprint `[0.25,0.75]²` now builds every gap. Its union's
walk reads `1.25 − g/6`, which is under the bound.

The other ops (∩ in both orders, both ∖) build or come back empty at
every cell.

FLUSH's tilt poses also build on this main: θ 2e-12 and 5e-12 at
ε 1e-9, and θ ≤ ¼·zero at every ε.

Two kinds of cell escalate:

- At 1e-12, a gap of `1e-12` (one zero) escalates
  `Coincidence(VertexOnFace)`.
- θ = ½·zero escalates `Coincidence(Sectors)`, or `(OnPlanes)`.

Both are in-band readings of the input, so they are not this row's.

## Fix (`cleave/backstop-admits-the-glues-zero-band`)

`volume_backstop` (`crates/topo/src/boolean/ops.rs`) takes the pairs
the op glued, and widens every set bound by what the glue admits
(`glue_admits`). Each `Rest` or continuation pair was decided Zero
over both faces: every point of each stands within the band's zero
of the other's carrier (`coincident_as_declared`'s reach,
`carrier_eq.rs`). The pairs join faces into groups, and each group is
merged onto one description, which may be its smallest face's. So
the widening is the zero times each group's area less its smallest
face's, at the interval area's lower end, each face counted once. A
`Tangent` pair moves no carrier and widens nothing, and a body with
no one-carrier pair is held to the exact bound as before. Every arm
reads the one widened margin: the sign arm, the refinement, and the
magnitude arm.

Pinned by `crates/topo/tests/a_block_resting_a_zero_band_gap.rs`.
The pin covers the three footprints, the row's gaps and overlaps,
the same as fractions of the zero, 1/2/4/16 ulp, and FLUSH's tilt
poses, every op in both orders. Where the glue door reads the pair
Zero, every op builds within the admitted volume of the box
arithmetic. Otherwise an op builds within rounding or escalates. No
cell refuses at the backstop.

`volume_backstop_admits_what_the_glue_moves_and_no_more` (`ops.rs`)
pins the bound itself. A union result standing ½·zero past the bound
passes with a one-carrier pair and refuses with none. One standing
2·zero past refuses with the pair, however often it is listed. A
`Tangent` pair admits nothing.
`the_glue_admits_each_group_less_its_smallest_face` pins the groups.
