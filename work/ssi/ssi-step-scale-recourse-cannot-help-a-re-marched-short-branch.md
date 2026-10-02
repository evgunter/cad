---
id: ssi-step-scale-recourse-cannot-help-a-re-marched-short-branch
kind: issue
title: ssi: a re-marched short branch whose step collapses into the band ends with STEP_SCALE's 'name a feature extent near the feature' advice, which cannot lengthen its step
status: open
opened: 2026-10-01
priority: P3
cost: E
---


(SSI implementer `ssi-short`, from the §5 sweep of PR "SSI: a short
branch is re-marched at its own length, and BranchUndersampled
retires", 2026-10-01.)

## What

`march_both` (`ssi/march.rs`) re-marches a branch too short for the
cubic in steps of its length over `SHORT_BRANCH_STEPS` (five). A branch
shorter than about `5·K·ε` therefore re-marches into the band and
refuses `StepCollapsed`, or escalates `ssi_step_progress`. Measured on
the collapsed net (`m5_pr7_ssi.rs`'s
`a_tiny_net_the_plane_meets_traces_or_refuses_by_the_kind_its_size_earns`):
a 30 µm branch at ε 1e-6, 30 nm at 1e-9 and 30 pm at 1e-12 escalate.

Both end by `ssi.rs`'s `STEP_SCALE`: "bring the operands within the
model's size range, and name a domain and feature extent near the size
of the feature traced". The second clause cannot help here: the
re-march's step is `min(SSI_STEP_MAX·extent, length/5)`, so a smaller
extent only shortens it. The first clause (the feature is below the
model's resolution) is the one that applies.

## Open

Whether `StepCollapsed` should say which cap set the collapsed step
(the extent's, the branch's own length, or a curvature term), so the
ending can name the lever that moves it.

## The plane × NURBS lane (PR 3862)

The short-branch re-march no longer runs on this lane. A branch whose
certified ends are under `SSI_SHORT_CLIP·Kε` apart takes the Hermite
cubic through them, and a refused candidate ends as a sized refusal in
its length (`SsiError::ShortBranchUncertified`); the collapsed net's
spreads whose plane lies within the band of the wall's long sides now
report those sides' regions. The re-march, and this ending question
with it, survive only on the ℝ³ lane (`ssi-r3-slab-is-not-geometry`).
