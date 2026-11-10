---
id: a-coaxial-blend-arm-mints-a-negative-major-torus-predicate-3-passes
kind: issue
title: The coaxial blend arm stores a signed spine radius, and a negative one passes predicate 3: the band surface is a torus of negative major radius, outside the ring convention
status: open
opened: 2026-11-10
priority: P2
---

Found while closing `the-torus-floor-is-read-by-doors-no-ring-gate-guards` (PR 6777's review, item 4).

## What

`sweep::blend::arms`'s coaxial arm (`crates/sweep/src/blend/arms.rs:957-1010`, `blend`) stores the spine radius signed. Its doc says so: "a centre that has crossed the axis yields a negative major radius, which is a torus the surface inventory reports rather than one this arm quietly repairs". It sets `major_radius: s` and `spine_curvature: 1/s`.

Predicate 3 (`crates/sweep/src/blend/battery.rs:674-692`, `spine_regularity`) reads the margin `r − r²·κ`. With `s < 0`, `κ` is negative, the margin is above `r`, and it passes. So the blend's band surface is a `Surface::Torus` of negative major radius, which `geom::RingTorus::of` decides outside the convention (`R − r < 0`). Predicate 3's doc says it refuses "the torus constructor's degenerate case ... before the surface exists", and that holds only for `0 < s ≤ r`.

What reads such a surface before the at-rest inventory sees it:
- The curve × torus root doors refuse it with the convention's refusal (`DoorRefusal::Torus`, PR 6777), so they give no wrong answer.
- Other readers are unchecked.

## The question

Should predicate 3 read `|κ|`, refusing a spine on either side of the axis that the tube swallows? Or should the coaxial arm refuse a ball centre past the axis outright? Or should it mint the torus with `|s|` and the seam turned half a turn? The arm's doc calls the signed store deliberate, so this is the blend lane's call.
