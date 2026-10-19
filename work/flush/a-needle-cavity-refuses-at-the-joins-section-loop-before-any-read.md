---
id: a-needle-cavity-refuses-at-the-joins-section-loop-before-any-read
kind: issue
title: A needle cavity 10.5–15ε across refuses SectionLoopUndecided at the join before any side or role is read
status: open
opened: 2026-10-19
priority: P2
cost: M
refs: [5119]
---


Found by PR 5119's lane while measuring the needle class (2026-10-19).

**What happens.** `brick ∖ bar`, where the bar is `(0,1)×(0,w)×(0,w)`
with w 10.5–15ε, at ε 1e-9, refuses at the join with
`SectionLoopUndecided`. This happens before any result exists, so the
side reader never sees it. From 21ε up, and in R1's posed sweep, the
cavity builds and reads `[Outer, Void]`.

The pairs are definite (> Kε). So this is a thinness refusal in the
join's section-loop decision, which `[coincidence.booleans]` ("how thin
a result is gets judged by its pairs alone") does not allow.

**What is owed.**
- Find which margin the section loop holds against the band at a
  needle's narrow faces: likely an in-face walk near a loop boundary,
  the same shape the edge-meeting ray removes from the side reader.
- Say whether it decides a tie the band forbids (`[verdicts]`), or
  measures a real pair.
- Fix it at the join (`join.rs` is FLUSH's), and pin the cavity at
  10.5, 12 and 15ε at ε 1e-9 and 1e-6, both orientations.
