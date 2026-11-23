---
id: review-curved-accepts-an-invariant-refusal-at-any-pose
kind: issue
title: review_curved's known-refusal filter accepts the srecledges ClassificationInvariant at any pose, op and eps
status: open
opened: 2026-11-22
priority: P2
cost: E
---


Found by PR 7383's review (NOTE n3). `sweep`'s
`review_arm_vs_reach_probes::review_curved` sorts its failures with a
known-failure filter, and its `srec` clause is the bare
`b.contains("surviving record with an On code after srecledges")`. That
accepts a `ClassificationInvariant` refusal (`srecledges`'
postcondition) on any pose, any op and at any ε. The module's own doc
counts an invariant refusal as a failure.

**What it accepts today.** At ε = 1e-6 (nightly 44870651337's junit,
main `f5ef8f2b`) it files these as known, each `B ∪ A`, `B ∩ A` and
`B ∖ A`:
- `r = 0.5`, `c = −0.5`, `lb = 0.4`, `θ = 1e-4` and `1e-3`;
- `r = 0.05`, `c = −0.05`, `θ = 1e-3`, at both `lb = 0.04` and
  `lb = 0.4`.

That is wider than the one pose SPLITDOOR's
`work/splitdoor/arm-readers-left-at-the-sector-arm-after-the-reach-reading.md`
names as the candidate witness: `r = 0.05`, `c = −0.05`, the boss
dipping `1e-4` into the disc. So a new invariant refusal at any pose
of this row would be read as known.

**Fix.** Bound the clause to the runs it is known for, at each ε, as
PR 7383 did for the row's `coarse` clause (`straddles_at_coarse_band`).
Read the default-ε runs off a nightly's junit first. Cite the SPLITDOOR
row beside the clause.
