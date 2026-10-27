---
id: the-value-chart-reads-a-written-orientation
kind: unit
title: The value chart reads a written orientation: the swing retires, slides carry by the least turn, one pole refuses
status: closed
opened: 2026-10-24
priority: P1
cost: M
branch: intent/value-chart-orientation
pr: 5742
closed: 2026-10-27
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

## Handoff

(2026-10-26, implementer.) PR 5742, branch `intent/value-chart-orientation`, merged over main through ATREST stage 4 I (#5637), so the mate is `{ on, to, orient }`.

- **Built as ruled.**
  - The orientation is `Mate.orient` (`mate/orient.rs`).
  - The chart is the least turn, with carried slides, the residual spin `Coord::Spin(None)`, one pole band (`MateFault::Backwards`, naming the orientation to write), and the span check (`SlidesDoNotSpan`).
  - The swing and `reference_about` are gone.
  - The viewer's snap writes the nearest orientation, the residual spin and the snapped slides (`viewer/src/session/snap.rs`).
  - Python: `Mate(orient=)` and `Orientation`.
  - An older document refuses at load naming `orient`.
- **Deviations** (the PR body says why):
  - the orientation sits on the mate, not on `place(orient=)`;
  - `sketch_on_face` takes the orientation and writes the two slides it leaves;
  - the panel offers every slide and refuses by count.
- **The spin's sense** (Ev, PR 5742, option (b)): the one spin turns about the target's own direction (`mate::flipped` undoes the target side's flips), so a rest spins the same way whichever side carries its flip, as before this unit.
- **Review 1's fix pass:** sketch-on-face's slides follow the orientation (six-face rows in Rust and Python); the least turn has one home (`mate/turn.rs`); `toward` keeps the author's x; the orientation's words are named tags; the review's probes are ordinary rows; the panel's count refusal is filed on CHROME.
- **What moved:** ids after every mate (the mint chain), content key v21, the m10/sym decision counts (two antipode theorems out and one pole decision in per mated placement), and the tour die's ±x pip seams (a quarter turn, rim vertices within an ulp). The PR body has the full list.
- **Open for the reviewer:**
  - the slow-set reds that are main's own are attributed in the PR body's test status;
  - `review_pick_r2` (red on main) moves by `kitchen_sink`'s first edit, 29 fewer aimed-vertex answers;
  - `pncad-py`'s `test_validate` ×4 is red on main's nightly.
