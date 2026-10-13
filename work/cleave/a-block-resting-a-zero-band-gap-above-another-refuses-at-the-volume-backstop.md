---
id: a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop
kind: issue
title: A union of a block resting a Zero-band gap above another refuses ResultVolumeImplausible: the glue closes the gap and the backstop has no slack for it
status: open
opened: 2026-10-13
priority: P0
cost: M
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
