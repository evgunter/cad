---
id: validation-pair-pass-has-no-pruning
kind: issue
title: Validation's pair pass reads every pair of segments with no pruning
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Filed by the cut of `work/repeat/a-loop-cannot-repeat-inside-a-profile.md`
(both designers of fork log row 296 asked for it).

## What

`crates/profile/src/validate.rs`, step "3: simplicity" in `validate`
(the `for li … for si … for sj` nest near line 1685), calls
`judge_pair` on every unordered pair of segments in the profile, across
all loops. Nothing prunes the pairs first: no bounding-box sweep and no
grid. Each pair is judged per evaluation and per scalar tier.

## Why now

Before repetition inside a profile, segment counts were what an author
wrote out by hand. With `each k`, one written run of 8 pieces at
`N = 60` is 480 segments, about 115,000 pairs per evaluation and tier.
That is unmeasured. `work/repeat/a-ring-closes-each-k-from-its-start-point.md`
records the time its 60-tooth test takes.

## Shape of a fix

Prune pairs whose boxes are apart by more than the band before
`judge_pair`, keeping the deterministic first-error order the comment
names. The pruning must be certified at `Interval`, not only at `f64`:
a box apart at `f64` must be apart at every lane. Not designed here.
