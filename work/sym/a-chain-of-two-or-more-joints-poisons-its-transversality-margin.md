---
id: a-chain-of-two-or-more-joints-poisons-its-transversality-margin
kind: issue
title: the wedge's transversality margin POISONS at two joints and at three, and it is what bounds the chain's certifiable box: just above the wall it is the first refusal at every link count
status: closed
opened: 2026-09-22
priority: P1
cost: D
refs: [SYM-14]
closed: 2026-10-02
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

**This is what bounds `chain::CERTIFIABLE_FRACTION`.** Just above each
link count's wall the first refusal is this wedge, at two links (node
14), three (node 21) and four (node 30) alike: `EdgeKey(1v1)`, sample 4.
What it reads there depends on ε:
- at the default ε its margin is undefined, the poison;
- at `1e-6` and `1e-5` it straddles `K·ε`.

`chaintol.rs`'s `the_wall_is_the_wedge_not_the_arm` pins the default
ε's.

**The certifiable box moves with ε** (`1.110e-1` at the default,
`1.083e-1` at `1e-6`), while the arm's straddling bracket does not. Why
is "What Phase 1 found", 2, below.

**It is not exclusive to two joints.** The three-link leaf carries the
SAME poison, on the same edge and the same sample (`EdgeKey(1v1)`,
sample 4, nodes 18 and 20) — it is simply not the FIRST refusal
reported there, because `dihedral_arm`'s straddle at node 12 comes
earlier in evaluation order. An earlier reading of this said "the
shorter chain is the poisoned one"; that was evaluation order read as
a fact about the chain, and it is wrong.

## What answers it

The upstream quantity that goes to NaN, named:
`memories/refusal-text-is-not-cause.md` applies, so the wedge
predicate's message is where the branch was refused and not where the
poison was minted. Answered by execution in "What Phase 1 found" below.
The pin's cylinder gradient (`geom_brep::implicit_gradient`, `w / radius`,
`crates/geom-brep/src/implicit.rs:220`) reaches zero once the point and
the cylinder's axis together are enclosed wider than the pin radius.
That is also why the certified tip half-width sits at half
`chain::PIN_RADIUS` (`chain::CERTIFIED_TIP_OVER_PIN_RADIUS`). The poison
is legitimate at the operation that mints it, and SYM-15's refusal now
names it.

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
`geom_brep::dihedral::wedge_decided` (`crates/geom-brep/src/dihedral.rs:232` at Phase 1's head, `:263` after SYM-15):
an interval quotient by a denominator containing zero is `Entire` at
decoration `Trv` (`impl Div for Interval`, `crates/geom-core/src/interval.rs:406`),
which `Interval::sign_within` reads as `MarginDiag::INVALID`
(`is_certified` below `Def`, `interval.rs:869`). The denominator reaches
zero through the cylinder arm of `geom_brep::implicit_gradient`
(`implicit.rs:220`, `w / radius`), whose `w` is `axial_radial`'s
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

**2. The ε dependence.**
- **The poison point does not move with ε.**
  - At 1.02× of the default fraction, two links, the probe's `p`,
    `origin` and `q` are bit-identical at ε = `1e-9`, `1e-6` and
    `1e-5`.
  - SYM-15's review bisected the onset of the poisoned first refusal
    (`sym/15-review` @ `e282874edc`, env `SYM15_POISON_BISECT`): at
    `3.702434e-1`–`3.702470e-1` of the study at two links and
    `1.110986e-1`–`1.110997e-1` at four, at all three ε.
- **The model's `f*`.** `f*` is the box at which the point's and the
  axis's lateral half-widths add up to the radius, `2δ = r`. With
  `δ = L·3σ·f·n(n+1)/2`, it is computed from the document's constants,
  not measured: `r / (2·L·3σ·n(n+1)/2)` = `3.70370e-1` at two links,
  `1.85185e-1` at three, `1.11111e-1` at four. The bisected onsets sit
  0.03 % and 0.01 % below it, because `p` carries a little more width
  than `δ`.
- **What moves is the WALL**, and the wall is not the poison.
  - As the box grows, the wedge margin's lower end falls toward zero as
    `margin.lo ≈ r·(1 − x)/(1 + x)`, with `x = f/f*`.
  - The box stops certifying where that lower end drops under `K·ε`.
  - At the default ε, `K·ε = 1e-8` is crossed at `1 − x ≈ 2.5e-5` by
    that model, inside the bisection's 0.27 % step. So the next box up
    is already past the poison.
  - At larger ε the wall is a STRADDLE of the same margin.
- **The lower end's fall is the FORMULA's, not the geometry's.**
  - `sin θ = ‖n1 × n2‖/(‖n1‖·‖n2‖)` carries `‖w‖` in its numerator and
    its denominator. At sample 4 the plane's normal and the cylinder's
    axis are exactly `±z` and `w ⊥ axis`, so the true image of `sin θ`
    over these boxes is `{1}`.
  - SYM-15's review measured the cosine spelling, `sqrt(1 − cos²)`
    (`sym/15-review` @ `e282874edc`, env `SYM15_COSFORM`). With it the
    box stops moving with ε: four links `1.10961e-1` and two links
    `3.69207e-1` at `1e-9`, `1e-6` and `1e-5` alike.
  - **Two levers, then, and each wall has its own:**
    - (i) the formula's dependency, local to `dihedral.rs`, sets the
      wall at every ε above the default. Filed:
      `work/props/interval-sin-theta-as-cross-over-norms-loses-the-shared-magnitude`.
    - (ii) the transform's decorrelation sets the poison point, and so
      the wall at the default ε. Filed:
      `work/shell/transform-rigid-recertifies-images-enclosed-apart`.

Measured by SYM-15 on 2026-10-02 with the Phase 1 probe (the patch is in
PR #3804's body):
- the fractions by
  `SYM15_FRACTION=1 CAD_TOLERANCE_EPS=<ε> SYM15_LINKS=<n> demo_tour sym15_probe --ignored`
  (`certifiable_fraction`);
- the first refusal at 1.02× of each ε's own fraction by the same test
  with `SYM15_OVER`.

| ε | links | measured | model `x·f*` | at 1.02×: `dihedral_wedge`, `EdgeKey(1v1)`, sample 4 |
|---|---|---|---|---|
| 1e-9 | 2 / 3 / 4 | `3.70208e-1` / `1.85104e-1` / `1.10961e-1` | `3.7036e-1` / `1.8518e-1` / `1.1111e-1` | margin INVALID (poison) |
| 1e-6 | 2 / 3 / 4 | `3.60318e-1` / `1.80159e-1` / `1.08290e-1` | `3.6123e-1` / `1.8061e-1` / `1.0837e-1` | straddle: lo `2.95e-6` / `3.01e-6` / `2.32e-6` < `1e-5` |
| 1e-5 | 2 / 3 / 4 | `2.87797e-1` / `1.43899e-1` / `8.62605e-2` | `2.8807e-1` / `1.4403e-1` / `8.6420e-2` | straddle: lo `9.24e-5` / `9.25e-5` / `9.29e-5` < `1e-4` |

At 1.02× and 1.10× of the DEFAULT ε's fractions, the first refusal is
the poisoned wedge at `1e-6` too, because those boxes lie past the
poison point. At `1e-6`'s own wall the first refusal is the same wedge
straddling `K·ε`.

**3. The classification.**
- **The poison is a legitimate undefined enclosure at the operation that
  mints it.** The enclosures `wedge_decided` is handed admit a point on
  the cylinder's axis (`p.y` and `origin.y` overlap), where the gradient
  vanishes and no tangent plane exists. No function of those enclosures
  can return a definite `sin θ` there.
- **Where the poison point sits is lever (ii)'s**: the transform
  re-certifies two separately enclosed images of one rigid map.
- **Where the wall sits at larger ε is lever (i)'s**: the formula's
  dependency, which is not the geometry's. Neither is the tier's.

**The minimal row outside the chain** is
`geom_brep::dihedral::tests::a_cylinder_gradient_reaching_zero_leaves_no_tangent_plane`.
At `Interval`, it takes a cap plane and a unit cylinder whose origin and
point are each enclosed `±δ` across the radial direction:
- at `δ = 0.49 r` they classify Transverse;
- at `δ = 0.51 r` `wedge_decided` escalates with no tangent plane, and
  `classify_dihedral` still reports the reading's invalid margin.

**What SYM-15 did with it.**
- **Both levers are filed and neither is fixed here**, per the stop
  rule: (i) on PROPS, (ii) on SHELL.
- **The refusal says its cause**, the spec's legitimate-poison Phase 2:
  - `wedge_decided` names an invalid margin whose gradient magnitudes
    are readable as `WedgeEscalation::NoTangentPlane`.
  - Certification reports it as
    `CertCheck::TangentPlanes`: "the surfaces' tangent planes at sample
    4 are undefined: a surface's gradient is zero or undefined somewhere
    over the enclosure of the point there, so no tangent plane, and no
    angle between the surfaces, is defined over it".
  - Its recourse is "keep the edge clear of each surface's axis or centre;
    over a parameter box, a narrower box encloses the edge and its
    surfaces closer to where they are", in place of the transversality
    margin's "may indicate a kernel bug".
- **No verdict moved.** The cause is read only after the wedge decision
  has already refused with an invalid margin. `classify_dihedral` and
  every caller other than certification are unchanged.

**Text these findings made wrong, corrected in SYM-15's PR (#3804).**
- **How it was found.** Grepped for `not established`,
  `unestablished`, `is a hypothesis`, `1.083e-1`, `POISONED`,
  `poisoned margin`, `margin is invalid`, `open question` and
  `not yet confirmed` over `demos/`, `crates/`, `work/` and `docs/`.
  This item's own body was included.
- **The mechanism now has one home:** `demos/tour/src/chaintol.rs`'s
  module header, "What sets the wall", with both levers attributed. Each
  of these says what it needs in a line and points there:
  - `demos/tour/src/chain.rs`, the `CERTIFIABLE_FRACTION` and
    `CERTIFIABLE_FRACTION_BY_LINKS` docs;
  - `chaintol.rs`'s narration comment, `certifiable_fraction`'s
    monotonicity comment (the half and quarter rows are its guard), and
    the fractions and wall rows' docs;
  - `demos/tour/src/mcchain.rs`'s box note;
  - `demos/README.md`'s chain paragraph.
- **Measured numbers live in the table above**, not in that prose.
- **`work/sym`:**
  - `a-chain-of-three-joints-straddles-dihedral-arm.md`,
    `a-widened-rotation-angle-is-unmeasured-on-the-certified-lane.md`
    and `SYM-14.md` say the wall is the poison at the default ε and a
    straddle of the same margin at `1e-6` and `1e-5`.
  - `log.md` is history, left as written.
- **Left as written:**
  - `mcchain.rs`'s rendered caption, which says only that the box moves
    with ε;
  - the narration's `println!` quoting `1.083e-1`.

## Closed (SYM-15, PR #3804, 2026-10-02)

Answered: what goes to NaN is named, it is legitimate, and the
certify path's refusal says so. What would move the chain's
certifiable box is not the tier's and is carried by two successors:
`interval-sin-theta-as-cross-over-norms-loses-the-shared-magnitude`
(PROPS, the wall at larger ε) and
`transform-rigid-recertifies-images-enclosed-apart` (SHELL, the poison
point, with the tier's own proof of `‖w‖ = r` as a candidate). The
other `classify_dihedral` callers' text is a seam on PROPS'
`invalid-margin-recourse-cannot-tell-an-unimplemented-kind-from-bad-inputs`.
