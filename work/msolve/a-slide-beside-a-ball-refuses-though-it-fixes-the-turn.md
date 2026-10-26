---
id: a-slide-beside-a-ball-refuses-though-it-fixes-the-turn
kind: issue
title: A slide on a direction mate beside a ball refuses NoChart (SlideTurnsABall), though in general position it fixes the ball's turn through the copy's lever: a branch-choosing settle would admit it
status: open
opened: 2026-10-18
priority: P3
cost: M
---

Found by the review of INTENT stage 3 F (PR #5299, finding 4). This is a disclosed design choice of F, not a defect.

**What happens.** A bundle of a Point mate, a Direction mate and a Slide value on the Direction mate refuses `MateFault::NoChart { why: NoChartWhy::SlideTurnsABall }` at the door and in the fold. The orchestrator ruled on the PR that it must not say `Overconstrained`, because that reads as a repeat and points the author the wrong way. The deciding function is `slide_beside_a_ball` (`crates/editor-core/src/mate/solve.rs`). The rows are `mate::solve::tests::the_four_value_availability_rows` and `intent_s3_b_placements::a_slide_beside_a_ball_refuses_as_having_no_chart`.

**Why it is not redundant in general position.** The two mates leave a turn about the direction through the ball's point, and the copy's origin is off that line. So the slide coordinate moves with the turn, and the slide would fix it. The rank is 6, with no redundancy.

**Why F refuses it.** F reads a slide as a translation equation and a spin as a rotation equation, and settles each structurally inside what the mates leave (`own_freedom`, `crates/editor-core/src/mate/solve.rs`; ASSEMBLY A11 (1)). A slide that fixes a turn would be a circle meeting a plane: two solutions or none, which the structural settle does not solve. The recourse is a spin on the direction mate.

**What would close it.** A settle that chooses a branch, so the slide is admitted. It would need a rule for which of the two meetings to take (the nearer to the copy's prior pose, or one the author names). Until then the refusal's recourse is the writable alternative: set the turn as the direction mate's spin, or pin the copy by a frame mate and its values. This is a ruling to take first, through the designer procedure (`memories/orchestration-model.md`), and then to Ev.

## Scoped (INTENT orchestrator, 2026-10-18)

The honest refusal is already built. F refuses with
`MateFault::NoChart { why: SlideTurnsABall }` and names both recourses: a spin
on the direction mate, or a frame mate and its values (de67cea75d). So this
item is only the lift, a branch-choosing settle that admits the slide. It stays
at P3 with no work scheduled. If an author ever needs the slide admitted, it
goes to the designer pair first, then to Ev.

The forks the lift would have to choose between:

- **(b1) The branch nearer a reference.**
  - *By the copy's prior or drawn pose:* ruled out by DI5
    (`crates/editor-core/IDENTITY.md`). Display state is read by no logic, and
    a copy's prior pose is not a document value.
  - *By the turn's zero:* arbitrary where that zero lies on the far side of
    both meetings, and it flips between the branches as the slide's value
    crosses the zero's projection.
- **(b2) A value that names the branch:** a sign, or a second coordinate,
  written beside the slide. This is the likely shape. The author states which
  meeting, the solve picks nothing, and the document stays the record.
- **(b3) No meeting**, where the slide's plane misses the turn's circle:
  refuse, by a measure the settle would then take (the circle's distance from
  the plane), unlike today's structural settle.

## D′2 re-read (2026-10-23)

D′2 left this refusal as F built it: a slide beside a point mate and a
direction mate still refuses `NoChart { why: SlideTurnsABall }`
(`slide_beside_a_ball`, `crates/editor-core/src/mate/solve.rs`; the
row `intent_s3_b_placements::a_slide_beside_a_ball_refuses_as_having_no_chart`). A lone point mate now leaves a
spin chain about the target space's axes, but a direction mate fixes
a direction, so this bundle is charted as before. The lift stays a
designer-pair question.
