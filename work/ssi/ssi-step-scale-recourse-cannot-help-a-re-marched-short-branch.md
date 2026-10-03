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
retires", 2026-10-01. Re-measured on main after PR 3862, 2026-10-03.)

## What

The ℝ³ lane's `march_both` (`ssi/march.rs`) re-marches a branch too
short for the cubic, with steps of its length over
`SHORT_BRANCH_STEPS` (five). A branch shorter than about `5·K·ε`
therefore re-marches into the band. It refuses `StepCollapsed`, or it
escalates `ssi_step_progress`.

Both endings come from `ssi.rs`'s `STEP_SCALE`: "bring the operands
within the model's size range, and name a domain and feature extent
near the size of the feature traced". The second clause cannot help.
The re-march's step is `min(SSI_STEP_MAX·extent, length/5)`, so a
smaller extent only shortens it. The first clause, that the feature is
below the model's resolution, is the one that applies.

Measured on main. The fixture is the threaded cylinder × unit sphere
(`m5_pr7_ssi.rs`'s `threaded_cylinder`, `sphere`), with a cube domain
of half-extent `L/2` centred on the north loop's point
`(0.11, 0, √(1 − 0.11²))`. The arc inside the cube runs about `L`
through the centre, which is the one seed. Calls go through
`cylinder_sphere_ssi`:

| ε | L | extent 0.2 | extent L |
|---|---|---|---|
| 1e-6 | 30 µm | escalates `StepProgress`, margin 6.0e-6 (`L/5`) | `StepCollapsed`, step 9.4e-7 (`L/32`, the extent's cap) |
| 1e-6 | 300 nm to 3 pm | `StepCollapsed`, step `√3·L` (the domain diagonal's cap) | escalates `TransversalityArm` |
| 1e-9 | 30 nm | escalates `StepProgress`, margin 6.0e-9 | `StepCollapsed`, step 9.4e-10 |
| 1e-12 | 30 pm | escalates `StepProgress`, margin 6.0e-12 | `StepCollapsed`, step 9.4e-13 |

Each `StepProgress` and `StepCollapsed` row ends with `STEP_SCALE`.
A smaller extent moves the 30 µm branch from the escalation to a
collapse at about a sixth of the step. A larger extent lifts the cap
only as far as `L/5`, which is still in the band. No extent clears the
band, so the ending's second clause names a lever that does nothing
here.

## Open

Whether `StepCollapsed` should say which cap set the collapsed step
(the extent's, the branch's own length, or a curvature term), so the
ending can name the lever that moves it.

## The plane × NURBS lane

PR 3862 retired the short-branch re-march on this lane. Where the
march cannot progress, with its step in the band, the branch takes the
Hermite cubic through its two certified ends. A refused candidate on a
branch too short for a fifth of it to clear the band ends as a refusal
sized in its length (`SsiError::ShortBranchUncertified`).

The row's original fixture is the collapsed net
(`a_tiny_net_the_plane_meets_traces_or_refuses_by_the_kind_its_size_earns`).
It no longer reproduces at ε 1e-6, 1e-9 or 1e-12: every spread whose
plane lies more than ε off the wall traces its one branch, and the
rest report the long sides' regions. What remains is the ℝ³ lane,
whose slab is itself in question (`ssi-r3-slab-is-not-geometry`).
