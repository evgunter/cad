---
id: a-slide-beside-a-ball-refuses-though-it-fixes-the-turn
kind: issue
title: A slide on a direction mate beside a ball refuses Overconstrained, though in general position it fixes the ball's turn through the copy's lever
status: open
opened: 2026-10-18
priority: P3
cost: M
---

Found by the review of INTENT stage 3 F (PR #5299, finding 4). This is a disclosed design choice of F, not a defect.

**What happens.** A bundle of a Point mate, a Direction mate and a Slide value on the Direction mate refuses `Overconstrained`, naming the Point and the slide (`mate::solve::tests::the_four_value_availability_rows`).

**Why it is not redundant in general position.** The two mates leave a turn about the direction through the ball's point, and the copy's origin is off that line. So the slide coordinate moves with the turn, and the slide would fix it. The rank is 6, with no redundancy.

**Why F refuses it.** F reads a slide as a translation equation and a spin as a rotation equation, and settles each structurally inside what the mates leave (`own_freedom`, `crates/editor-core/src/mate/solve.rs`; ASSEMBLY A11 (1)). A slide that fixes a turn would be a circle meeting a plane: two solutions or none, which the structural settle does not solve. The recourse is a spin on the direction mate.

**What would close it.**
- Keep the refusal, and give it a refusal of its own ("the bundle's other mates leave this slide no translation"), not `Overconstrained`'s "fixes an equation the rest already fix".
- Or admit it with a branch-choosing settle.

Either way it is a ruling to take first, through the designer procedure (`memories/orchestration-model.md`), and then to Ev.
