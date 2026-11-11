---
id: the-rotational-lane-reads-the-torus-chart
kind: unit
title: The rotational-chart Green lane, stage 3: the torus chart (the spiric wall onto the lane, projected chords and rims on any trimmed torus wall, rings)
status: dispatched
opened: 2026-11-10
priority: P0
cost: H
branch: spiricarm/rotational-lane-torus
refs: [a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane, a-torus-wall-holding-a-spiric-beside-a-spline-chord-has-no-volume-lane, the-spiric-wall-lane-omits-its-junctions-connectors-from-its-region-term, the-rotational-lane-reads-the-sphere-chart]
---


Stage 3 of 3 of the chord-volume decide.
- Stage 1, the cylinder: PR 6478, review row `docs/dual-review-log/rows/PR-6478.md`.
- Stage 2, the sphere: PR 6509, review row `PR-6509.md`.

Read both rows. Every class they record recurred from stage 1 to stage 2:
- an uncontained chart term;
- circular swap rows;
- unwitnessed pads;
- pins that assert a face kind in prose.

## Closes

- `a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane` (P0). This is teapot wall 2's stop, a donut ∩ rod row and germ cells.
- `a-torus-wall-holding-a-spiric-beside-a-spline-chord-has-no-volume-lane` (P2).
- `the-spiric-wall-lane-omits-its-junctions-connectors-from-its-region-term` (P2).

Take these too if the migration makes them natural, and say which you take:
- `a-spiric-bounded-torus-band-has-no-volume-lane` (P3)
- `the-spiric-wall-quadrature-lane-reads-only-the-own-torus` (P3)

## Phase 0: decide the torus's `∮v du` first

On the torus, `F′ = rρ(R cos v + r)` has a constant term, so `F` is linear in `v`, and along a chord `v` is an `atan2`. C9 bars transcendentals from certification arithmetic. The decide's candidate route was never verified. It writes `v(t) = v(t_a) + ∫v′`, which makes `∫v u′` an iterated integral of algebraic functions, with `v(t_a)` read at the vertex through `image.eval` at the lane scalar, as `spiric_wall`'s `ends` do.

Verify it, or find the right route. If no transcendental-free enclosure exists, STOP and report it as a problem: "the torus chart-area term over a spline chord has no algebraic primitive, and C9 bars `atan` from certification arithmetic". That is an Ev fork.

## What it builds

- **The torus as the lane's third chart.**
- **The spiric wall** (`props/spiric.rs` `spiric_wall`) moves onto the lane's edge-term enum as an arm. There is no second edge loop, closure or pad. This is `[simple.uniform]`, and both earlier stages minted fresh duplication their reviews caught.
- **Routing.** Every trimmed torus wall goes to the lane, not only a wall carrying a spiric. `reads_rings` opens for a trimmed torus.
- **Junction connectors** cover the spiric arm, and the stage 2 corner-pad derivation is checked for the torus's metric.

## Oracles (containment, never narrowness)

- **Carrier-swap fixtures on the torus.** Compute the expected values independently of the function under test. Include:
  - a rim, a meridian and a spiric rewritten as exact rationals;
  - a chord that winds;
  - both the outer and inner equator;
  - the body translated by 1e3.
- **The donut ∩ rod rows and teapot wall 2.** Check each body against an independent volume (a slicing oracle) or the closed form. A pin that names a kind reads `refused_face_kind`.
- **Inclusion–exclusion is not admissible.**

## Rules carried

- Run a mutant pass over your own pads and guards BEFORE opening the PR, and report it. Every pad or guard needs a row that goes red without it; a pad that is sound by slack is argued at its site.
- An equivalence claim for a mutant needs the whole subject's rows.
- A disclosed gap becomes an issue file.

**Review tier:** dual.
