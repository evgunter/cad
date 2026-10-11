---
id: a-wedge-flush-on-a-block-top-refuses-every-op-undeclared-though-its-face-reads-zero
kind: issue
title: A wedge flush on a block's top, its face tilted 1.2 to 2 eps, refuses every op undeclared though the face stands off the block's plane by a Zero margin; the declared spelling builds, and INTENT F retires it
status: open
opened: 2026-10-11
priority: P1
cost: M
---


Found by `flush/readers-closes` re-measuring
`a-declared-flush-wedge-sunk-in-a-block-refuses-its-intersect-join-desync`
without its declaration (main `19b704d92f`, release build).

## Repro

The fixture of `crates/topo/tests/door_backstop_settled_residue.rs`:
a `3 × 4.5 × 1` block, and a parallelepiped cornered on its top at
`(0.5, 0.2, 1)` by a 5° wedge angle. The parallelepiped's face in
the block's top plane is tilted about the wedge's edge by
`θ = k · band.zero()`, `k ∈ {±1.2, ±2}`. It either stands on the block
(height 1) or is sunk 0.5 into it. The test declares the pair
(`Rest` standing, `Continuation` sunk). Run the same poses through the
plain `topo::union` / `intersect` / `subtract` instead.

## Measured

- **Main, undeclared**: every op refuses, in both operand orders, at
  every pose and at ε 1e-9, 1e-6 and 1e-12 alike (36 runs per ε row).
  Each refusal is `Escalated { decision: Coincidence(Sectors,
  Settles(Settling { class, .. })) }`, offering `Rest` (standing) or
  `Continuation` (sunk), from `boolean/recl.rs` `require_same` →
  `unglued_tilt`. The predicate is `bool_sector_coplanar`, and its
  margin is 1.046e-10 (±1.2ε) or 1.743e-10 (±2ε) against the zero band
  ±1e-9, at ε 1e-9.
- **Main, declared** (the test as written): it builds every op within
  the gap of the box arithmetic. The exceptions are the crossings the
  test allows, which refuse `ResultVolumeImplausible`. Green at all
  three ε.
- **INTENT F's WIP head** (`intent/s4-f-declared-pairs-retire` at
  `68555105c`), plain booleans: all 24 runs refuse "how two corners
  of the two solids overlap where they meet is undecided: margin
  1.743e-10 lies within the zero band (±1e-9). Recourse: move the
  parts … or tighten the tolerance". F deletes
  `door_backstop_settled_residue.rs` and its declared spelling. Once F
  lands, these poses refuse every op and no test pins them.

## Why it is a finding

The face's largest standoff from the block's plane is
`θ · sin φ · |e_b| = 2e-9 · sin 5° ≈ 1.74e-10` at 2ε, which is the
reported margin: Zero. D10 ("Booleans") glues what its verdicts decide
Zero. The refusal comes from the carrier reading before it:
`carrier_eq_reading` on an undeclared pair reads the two planes over
an `ExtentBall` about the corner of radius `s1.span()` (the
`ReadOn::FirstBounds` reach), not over the faces. The tilt, levered
at that radius, decides `Distinct`, so `unglued_tilt` refuses although
every bound reads Zero. The declared spelling reads the carriers over
the door's consumed extent of both faces, which is why it builds.

So on main a declaration does move this body: it turns a refusal into
a build. INTENT F's premise that a declaration moves no body does not
hold for this fixture, and F deletes the fixture.

## Not measured

- Whether reading an undeclared pair's carriers over the two faces'
  extent (what the declared door consumes) is the fix, or whether the
  span lever is right and the bounds' Zero should be read as the
  sliver band instead.
- Which other planar near-flush poses `unglued_tilt` refuses this way.

The pre-PR-3866 history of this fixture is in the closed row
`a-declared-flush-wedge-sunk-in-a-block-refuses-its-intersect-join-desync`,
and its volume-backstop residue in
`work/reachhold/a-settled-declared-coincidence-crosses-a-tight-volume-bound.md`.
