---
id: ssi-short-branch-step-reads-only-the-callers-extent
kind: issue
title: ssi/march: the longest step is SSI_STEP_MAX of the caller's extent alone, so a feature shorter than a few steps is refused (BranchUndersampled), never traced
status: open
opened: 2026-10-01
---


Residue of `ssi-tiny-net-meeting-plane-refuses-as-too-few-fit-points`,
closed by the `ssi/chart-floor` lane with a named refusal. This row is
the design option that unit did not take.

`ssi/march.rs`'s realized stepper caps its step at
`SSI_STEP_MAX · ctx.extent / speed`, and `ctx.extent` is the caller's
`SsiDomain::extent`. A branch shorter than about three such steps
leaves the domain in one step each way and yields 3 samples, below the
cubic fit's 4. Measured on the collapsed net at spreads `1e-2` through
`1e-8` against a plane that meets it (extent 1.5 m, longest step
4.69e-2 m): each branch has 3 samples. Each now refuses
`SsiError::BranchUndersampled`, naming the branch's length, the
longest step and the extent, with the recourse "name a feature extent
no larger than this feature". Following that recourse traces and
certifies spreads `1e-2` through `1e-6`; at `1e-8` (a 3e-8 m branch)
the step collapses into the band, which is honest.

Open: whether the stepper should trace such a branch itself, for
example by re-marching a trace shorter than the fit's need with the
step scaled to the trace's own length. That makes the step rule read
something other than the caller's extent, which today is the only
length it reads (`SSI_STEP_MAX`'s doc). It is a stepper design choice,
not a fix.
