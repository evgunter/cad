---
id: ssi-short-branch-step-reads-only-the-callers-extent
kind: issue
title: ssi/march: the longest step is SSI_STEP_MAX of the caller's extent alone, so a feature shorter than a few steps is refused (BranchUndersampled), never traced
status: closed
opened: 2026-10-01
priority: P2
cost: M
pr: 3730
branch: ssi/short-branch
closed: 2026-10-02
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
step scaled to the trace's own length. Today the step's lengths are
the caller's extent (`SSI_STEP_MAX`'s cap), the curvature terms, and
the march domain's diagonal, which caps a step at the whole domain and
so never shortens one below a branch that crosses it. None of them is
the branch's own length, so re-marching at it would add a scale the
step rule does not read. It is a stepper design choice, not a fix.

## Design (2026-10-01): the tracer owns a short branch

Two designers weighed this independently, and both reached the same
answer in round 1. No ratified text changes, so it does not go to Ev.
README C3's `BranchUndersampled` sentence is agent text from PR 3694
that describes the code. This section is the spec.

**Premise correction.**
- This case is not exotic. On an ordinary 1 m flat NURBS face, a plane
  clipping a corner refuses `BranchUndersampled` for branches of 1.4 cm
  and 4.2 cm. Booleans meet that routinely.
- The extent is one knob per operation, while branches are many per
  operation. A call holding a body-length branch and a centimetre
  clip has no extent that serves both (estimated beyond a ~500× length
  ratio).
- Following today's recourse also shrinks the extent's other roles: the
  transversality lever arm, the seeding floor, the tube ladder's widest
  rung, and the idealized step. That weakens the certificate for no
  geometric reason.

**The design.**
1. `march` reads one step ceiling in metres (`step_cap`), separate from
   `ctx.extent`. The extent keeps its other roles: it is the lever arm
   of last resort (the arm clamp stays on the extent), the seeding
   floor, the tube ladder's widest rung, and a trust radius on how far
   a step extrapolates the local jet.
2. `march_both`, the one place a whole branch is known, marches with
   `step_cap = SSI_STEP_MAX · extent`. If the spliced open trace has
   fewer samples than the cubic fit needs (`SSI_FIT_DEGREE + 1`), it
   marches once more with
   `step_cap = min(SSI_STEP_MAX · extent, SSI_STEP_MAX · length)`,
   where `length` is the first trace's polyline length. That is the
   module's own step density, read off the branch, with no new
   constant. The cap only shrinks. It fires at most once, by a fixed
   rule (D9).
3. The idealized stepper reads the same `step_cap`, because its
   `SSI_IDEALIZED_STEP · extent` has the same defect.
4. `SsiError::BranchUndersampled` and `fit_branch`'s `TooFewPoints`
   mapping are retired, along with its arm in `pcurve_cache.rs`. A trace
   still short after the re-march cannot happen by construction. If it
   does, it is a kernel defect, and the fit's own `TooFewPoints` stands
   with D4's last-resort wording.
5. Re-pin `a_tiny_net_the_plane_meets_refuses_by_the_kind_its_size_earns`
   (spreads 1e-2..1e-6 or 1e-7 certify; the smallest refuse or escalate
   in the band). Turn `a_branch_shorter_than_the_march_step_names_the_extent_that_set_it`
   into "a short branch traces at the caller's extent". Add the
   corner-clip fixture as a non-degenerate row. Re-word `SSI_STEP_MAX`'s
   doc and README C3 to describe the landed rule.

Probes (measured, uncommitted): every corner clip from 1.4 cm to 28 cm
certifies at extent 1 m and 1.5 m. The collapsed net certifies down to
a ~3e-7 m branch. No other row of the SSI suite enters the re-march.

## Closed (2026-10-02, PR 3730)

The tracer owns a short branch. `march` reads a step cap in metres,
separate from the extent. `march_both` re-marches a trace that has
length but too few samples for the cubic. `BranchUndersampled` is
retired.

The landed re-march rule is **not** Design item 2's
`SSI_STEP_MAX·length`. That rule walked a state from a mid-branch seed
onto each end, a few ε inside the boundary, and escalated the open end
at ε 1e-12. The cap is `length / SHORT_BRANCH_STEPS`, with
`SHORT_BRANCH_STEPS = (SSI_FIT_DEGREE + 1) | 1 = 5`; the orchestrator's
ruling is in PR 3730's body.

A trace with no length (a point contact, or a branch below the
boundary search's resolution) refuses `SsiError::TraceUnresolved`, not
the fit's count.

Residue, filed:
- `ssi-a-plane-through-a-faces-vertex-is-a-point-contact-not-a-refusal`
- `ssi-final-chord-far-shorter-than-the-step-fails-the-certificate`
- `ssi-step-scale-recourse-cannot-help-a-re-marched-short-branch`
