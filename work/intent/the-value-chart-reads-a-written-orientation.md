---
id: the-value-chart-reads-a-written-orientation
kind: unit
title: The value chart reads a written orientation: the swing retires, slides carry by the least turn, one pole refuses
status: open
opened: 2026-10-24
priority: P1
cost: M
---


## What

Build `[placement.values]` as ratified on PR 5621 (Ev chose F, 2026-10-24; design-fork row 192).

- **The orientation.** It is a discrete kind on a bundle with a direction-fixing mate (plane, axis, direction; pivot later), and only there: one of the 24 axis correspondences, the identity unless written. It goes on the wire, in persistence and in the content key. Mateless and point-mate bundles keep their spin chain and carry none.
- **The least turn** `S` carries `n = O·d_c` (where the orientation puts the copy's mated direction) onto the mated direction `d`. The copy's rotation is `rot(d, θ)·S·O`.
  - The one spin `θ` is the residual about `d`, with no reference axis on either side. It is spelled `Spin` with no axis beside a mate, and `Spin(X|Y|Z)` for the chain.
  - Slides run along `S·x`, `S·y` and `S·z`. When `S` is the identity the bits are today's for axis-aligned mates.
  - The pole `n = −d` refuses in its band, typed, naming the orientation to write. Nothing refuses short of the pole band, and an orientation more than a quarter turn off at authoring is at most a lint.
- **Slide admission.** Where the source's mated direction is one of its axes, the mate takes the carried slide that lands on `d`. Otherwise two written slides are admitted only where they span the plane with it, decided once from the source's geometry.
- **Retires:**
  - `reference_about` and its `mate_chart_reference_antipode` decision;
  - the `copy_x` swing;
  - the half-turn at −z;
  - "a mate's `Flip` is on the copy's side". `Flip` stays as the mate's sense.
- **Absorbs:** `a-lone-slide-refuses-the-chart-it-never-reads`, and `a-value-chart-jumps-at-the-antipode-under-a-downward-face`, which closes with this unit.
- **The viewer's mate tool** (`SessionOp::AddMate`, `viewer/src/session.rs`) writes the orientation nearest where the copy stands, plus the residual spin. It also rewrites the slides it keeps so the copy moves only by the snap. Today it re-reads kept values on the swung chart, a live defect. Python's `place(...)` takes `orient=`, defaulting to the identity.
- **Tests:**
  - test 24 is re-spelled with its intent kept;
  - the antipode rows: a boss hung under a plate tilted 1e-12 to 0.1 about either axis stays within O(tilt) of x = +10, and continuous through zero;
  - the pole-band refusal names the orientation;
  - a floor swept through 90° stays continuous;
  - each row says what breaks it.
- **Docs:** `crates/viewer/README.md`'s "swung onto the mated direction" sentence; `docs/INTENT-STAGE3-SPEC.md` §5′.2's chart text.
- **Waits:** `flag-and-pivot-pose-kinds` follows this unit, and a pivot takes the residual-spin rule.

**Review tier:** dual (letter M). It is the chart every valued placement reads, hard to change later.
