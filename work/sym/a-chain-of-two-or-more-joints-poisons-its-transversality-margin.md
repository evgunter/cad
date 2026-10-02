---
id: a-chain-of-two-or-more-joints-poisons-its-transversality-margin
kind: issue
title: the wedge's transversality margin POISONS at two joints and at three, and it is what bounds the chain's certifiable box: just above the wall it is the first refusal at every link count
status: dispatched
opened: 2026-09-22
priority: P1
cost: D
refs: [SYM-14]
---



## What

**Specifically requested by Ev, 2026-09-22** — found by SYM-14, the
chain demo Ev asked for, measuring the certified lane on it
(`demos/tour/src/chaintol.rs`, the cell's own CI rows).

One leaf over the declared box, `Sym<Interval>`, on the chain of
`demos/tour/src/chain.rs`. At TWO links over the whole study:

```
node 14 — the transform op refused: transform: mapped edge EdgeKey(1v1)
failed re-certification: certification: the transversality margin at
sample 4 escalated: predicate 'dihedral_wedge' indeterminate: margin is
invalid (NaN or a poisoned enclosure)
```

A straddle is the tier working and running out of width. A POISONED
margin is not: a NaN says some upstream quantity went to poison, and
the refusal reports the branch it could not take rather than the thing
that poisoned.

**This is what bounds `chain::CERTIFIABLE_FRACTION`.** MEASURED at
`1.02×` and `1.10×` of each link count's certifiable fraction, at the
default ε and at `1e-6`: the first refusal is this wedge, poisoned, at
two links (node 14), three (node 21) and four (node 30) alike —
`EdgeKey(1v1)`, sample 4, every time. Pinned by
`chaintol.rs`'s `the_wall_is_the_wedge_not_the_arm`.

**The certifiable box moves with ε** (`1.110e-1` at the default,
`1.083e-1` at `1e-6`, both measured) while the arm's straddling bracket
does not move at all. WHY it moves is an open question and belongs to
this row, because this is the refusal at the boundary — and it is not
answered by "the margin is compared against the band", which is what
this row said for one cycle: a NaN is not classified against a band at
any ε. Something upstream of the poison is ε-dependent; naming it is
part of what answers this row.

**It is not exclusive to two joints.** The three-link leaf carries the
SAME poison, on the same edge and the same sample (`EdgeKey(1v1)`,
sample 4, nodes 18 and 20) — it is simply not the FIRST refusal
reported there, because `dihedral_arm`'s straddle at node 12 comes
earlier in evaluation order. An earlier reading of this said "the
shorter chain is the poisoned one"; that was evaluation order read as
a fact about the chain, and it is wrong.

## What answers it

The upstream quantity that goes to NaN, named —
`memories/refusal-text-is-not-cause.md` applies, so the wedge
predicate's message is where the branch was refused and not where the
poison was minted. The reviewer's reading, not yet confirmed by
execution and recorded here as the first thing to check: the pin's
cylinder gradient straddles zero once the positional box exceeds the
pin radius (`crates/topo/src/dihedral.rs`, the transversality arm).
That would also explain why the certified tip half-width sits at half
`chain::PIN_RADIUS` at every link count and doubles when the radius is
doubled (`chain::CERTIFIED_TIP_OVER_PIN_RADIUS`) — but it is a
hypothesis about the mechanism until someone reads the margin's own
inputs at the wall. Then either the poison is a real defect and is
fixed, or it is a legitimate empty enclosure and the refusal says
which.

## Home

SYM — the tier's reach on a construction Ev asked for; if the poison
turns out to be minted in the certification lane rather than in the
tier, it re-homes there.

## What Phase 1 found (SYM-15)

Measured with an env-gated probe in `geom_brep::dihedral::wedge_decided`
that prints every input of the wedge margin (the patch is in the SYM-15
PR body, not committed), driven by a one-leaf `Sym<Interval>` row on the
chain at a chosen fraction, link count and ε.

**1. The margin's inputs at the wall** (two links, 1.02× of `3.702e-1`,
default ε, `EdgeKey(1v1)`, sample 4). The edge is the tip pin's bottom
cap circle: a plane × cylinder `Intersection`, re-certified after the
outer joint's transform. Enclosures:

| input | enclosure | |
|---|---|---|
| `p` (carrier at sample 4) | x `[2.39780e-2, 2.40197e-2]`, y `[3.9189e-4, 1.20801e-3]`, z `0` | Com |
| cylinder `origin` | x `[2.39962e-2, 2.40015e-2]`, y `[-4.0780e-4, 4.0780e-4]`, z `0` | Com |
| `axis` | `(0, 0, [0.99987, 1.00013])` | Com |
| `radius` | `8e-4` | Com |
| `w = (p − origin)⊥` | x `[-2.352e-5, 2.352e-5]`, y `[-1.591e-5, 1.6158e-3]` | Com |
| `‖n_cyl‖ = ‖w‖/r` | `[0, 2.01997]` | Com |
| `n_plane`, `‖n_plane‖` | `(0, 0, −1)`, `[0.99987, 1.00013]` | Com |
| `‖n1 × n2‖` | `[0, 2.02023]` | Com |
| `‖n1‖·‖n2‖` | `[0, 2.02023]` | Com |
| **`sin θ`** | **`[−∞, +∞]`** | **Trv** |
| `extent`, `arm` | `[1.5945e-3, 1.8006e-3]`, `8e-4` | Com |
| margin `sin θ · arm` | `[−∞, +∞]` | Trv |

**The first to poison is `sin θ`.** Everything before it is a sound
`Com` enclosure; `‖n_cyl‖`'s merely reaches zero. It is minted by the
division `n1.cross(n2).norm() / (n1.norm() * n2.norm())` in
`geom_brep::dihedral::wedge_decided` (`crates/geom-brep/src/dihedral.rs:232`):
an interval quotient by a denominator containing zero is `Entire` at
decoration `Trv` (`impl Div for Interval`, `crates/geom-core/src/interval.rs:406`),
which `Interval::sign_within` reads as `MarginDiag::INVALID`
(`is_certified` below `Def`, `interval.rs:869`). The denominator reaches
zero through the cylinder arm of `geom_brep::implicit_gradient`
(`implicit.rs:213`, `w / radius`), whose `w` is `axial_radial`'s
`q = p − anchor` (`implicit.rs:104`): two independently enclosed points
subtracted, each `±4.078e-4` across the chain, so `q.y` is `r ± 8.16e-4`
against `r = 8e-4`.

**Where it is: in `geom-brep`'s dihedral predicate, on enclosures the
certification lane hands it, not in the tier.** `Sym<T>`'s value channel
is `T`'s verbatim (`impl Div for Sym<T>`, `geom-core/src/sym.rs:4954`,
computes `self.value / rhs.value`); the tier reads no enclosure and
narrows none. The two decorrelated points come from `topo`'s
`transform_rigid` (`crates/topo/src/transform.rs`): it maps the
surfaces (`map_surface`, `:645`) and the edge's carrier (`map_carrier`,
`:679`) separately through one widened rigid map, then re-certifies
(`EdgeCurve::certify_via`, `:729`), whose `run_checks` evaluates
`p = spec.carrier.eval(t)` (`certify.rs:2340`) and calls `wedge_decided`
(`certify.rs:2358`). The true `‖w‖` is `r` at every point of the box (a
rigid map preserves `p₀ − o₀`); its enclosure loses that because the
images are enclosed apart.

The same shape at three links (node 21) and four (node 30), 1.02×,
default ε: tip pin, `q.y` `[-1.577e-5, 1.6157e-3]` and
`[-1.527e-5, 1.6153e-3]`, `‖n_cyl‖` `[0, 2.0198]` and `[0, 2.0191]`,
`sin θ` `Entire`/`Trv`.

**This is also why the tip sits at half the pin radius.** The point and
the axis each carry the tip's lateral half-width `δ`; their difference
carries `2δ`; the radial enclosure reaches zero at `2δ = r`. Sample 4 is
the circle's point whose radial direction lies across the chain.

**2. The ε dependence.** The poison point does not move with ε: at 1.02×
of the default fraction, two links, the probe's `p`, `origin` and `q`
are bit-identical at ε = `1e-9`, `1e-6` and `1e-5`. What moves is the
WALL, because the wall is not the poison. As the box grows, the wedge
margin's lower end falls continuously to zero —
`margin.lo ≈ r·(1 − x)/(1 + x)`, `x = f/f*` and `f*` the box at which
`2δ = r` — and the box stops certifying where that lower end drops under
`K·ε`. At the default ε, `K·ε = 1e-8` is 25 ppm short of `f*`, inside the
bisection's 0.27 % step, so the next box up is already past the poison.
At larger ε the wall is a STRADDLE of the same margin. Re-measured
(`certifiable_fraction` at each ε; the first refusal at 1.02× of each
ε's own fraction):

| ε | links | measured | model `x·f*` | at 1.02×: `dihedral_wedge`, `EdgeKey(1v1)`, sample 4 |
|---|---|---|---|---|
| 1e-9 | 2 / 3 / 4 | `3.70208e-1` / `1.85104e-1` / `1.10961e-1` | `3.7036e-1` / `1.8518e-1` / `1.1111e-1` | margin INVALID (poison) |
| 1e-6 | 2 / 3 / 4 | `3.60318e-1` / `1.80159e-1` / `1.08290e-1` | `3.6123e-1` / `1.8061e-1` / `1.0837e-1` | straddle: lo `2.95e-6` / `3.01e-6` / `2.32e-6` < `1e-5` |
| 1e-5 | 2 / 3 / 4 | `2.87797e-1` / `1.43899e-1` / `8.62605e-2` | `2.8807e-1` / `1.4403e-1` / `8.6420e-2` | straddle: lo `9.24e-5` / `9.25e-5` / `9.29e-5` < `1e-4` |

So the earlier sentence, "at 1.02× and 1.10× at `1e-6` the first
refusal is the poisoned wedge", was measured at the DEFAULT ε's
fractions, which at `1e-6` lie past the poison point. At `1e-6`'s own
wall the first refusal is the same wedge straddling `K·ε`, not poisoned.

**3. The classification: a legitimate undefined enclosure at the minting
operation.** The enclosures `wedge_decided` is handed admit a point on
the cylinder's axis (`p.y` and `origin.y` overlap), where the gradient
vanishes and no tangent plane exists. No function of those enclosures
can return a definite or straddling `sin θ` without claiming a value at
a point where the quantity is undefined. What makes the wall sit where
it does is the width the transform's re-certification loses by
subtracting two separately enclosed images of one rigid map. That loss
is not a poison, and it is not the tier's.

Minimal row outside the chain:
`geom_brep::dihedral::tests::a_cylinder_gradient_reaching_zero_poisons_the_wedge`.
At `Interval`, a cap plane and a unit cylinder whose origin and point
are each enclosed `±δ` across the radial direction classify Transverse
at `δ = 0.49 r` and escalate at `δ = 0.51 r`: `"dihedral_wedge"`,
`LeverRung::Reading`, margin INVALID.

**Stop rule: applies.** The poison is minted outside the tier, and the
lever that would move the box lives in `topo::transform_rigid`'s
re-certification, which is `offset`'s and `shell`'s ground by
`work.py territory`. Transporting the source's certificate across an
isometry, or carrying `p − origin` through the map, would do it; neither
is local to the minting operation. A refusal that names the cause
("the gradient's enclosure reaches zero") needs a new rung or check in
the shared escalation taxonomy (`LeverRung`, `CertCheck`), since the
margin it renders is the wedge's. Per this row's Home, it re-homes.

Text these findings made wrong, corrected in SYM-15's PR (#3804).
Pins: none moved. Grepped for `not
established`, `unestablished`, `is a hypothesis`, `1.083e-1`, `POISONED`
and `poisoned margin` over `demos/`, `crates/`, `work/` and `docs/`; the
chain's hits:
- `demos/tour/src/chain.rs` (`CERTIFIABLE_FRACTION`'s doc): "WHY it moves
  is not established". It is now: the `K·ε` crossing of the same margin.
- `demos/tour/src/chaintol.rs`:
  - the header's "just above the WALL … default ε and `1e-6` alike …
    POISONED". At `1e-6`'s own wall it straddles.
  - "The mechanism is a hypothesis". It is measured (finding 1).
  - the narration's and `certifiable_fraction`'s "not established" /
    "nothing here establishes that a poison cannot reappear at a
    narrower width". The poison needs the radial enclosure to reach
    zero, and a sub-box's enclosure is inside the box's.
  - the fractions row's doc, "What makes them move is not established".
  - `the_wall_is_the_wedge_not_the_arm` asserts "margin is invalid" at
    1.02× of the DEFAULT fractions at every ε. That stays true, because
    those boxes are past the poison point at `1e-6` too; its doc now
    says it is scoped that way. What it asserts is unchanged.
- `demos/tour/src/mcchain.rs` (the box note; the sheet's caption says only
  that the box moves, which stays true, and is left as written) and
  `demos/README.md` (the chain's paragraph): "why it moves is not
  established".
- `work/sym/a-chain-of-three-joints-straddles-dihedral-arm.md`,
  `a-widened-rotation-angle-is-unmeasured-on-the-certified-lane.md` and
  `SYM-14.md`: "POISONED" as the wall. True at the default ε only.
- `work/sym/log.md`: history, left as written.
