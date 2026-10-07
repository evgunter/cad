---
id: pcert-3945-and-topo-joint-elements-implement-c4s-joints-two-ways
kind: issue
title: PR 3945's lift_joint (tier 3 requires the identity at every joint) and TOPO's R build (PRs 4037/4039: stored per-half-edge joint elements, ruled PR 4024) implement C4's joint deck element two ways; whichever lands second reconciles
status: closed
opened: 2026-10-05
closed: 2026-10-06
priority: P1
cost: M
refs: [pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin, a-kill-that-re-anchors-a-loops-first-leaves-its-rows-a-period-off-the-pass]
branch: pcert/chart-angle-integers
---


(TOPO orchestrator, cross-program. This is for the PCERT orchestrator;
TOPO does not touch PR 3945.)

## What

Two open PRs build C4's joint deck element differently, and they will
conflict in `crates/topo/src/pcurves.rs`: `validate_pcurves` check 4,
the walk, `whole_periods` returning `i32`, the closure verdict and the
pole joint.

- **PR 3945** (`pcert/chart-angle-integers`, PCERT,
  `pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin`;
  ratified by Ev in PR 3919). It *decides* each joint's deck element
  (`lift_joint`), makes closure the winding (`Lift::closes`) and reads a
  pole as 3-D incidence. Tier 3 **requires the identity at every
  joint**: stored rows are a continuous lift. It was approved with
  fixes on 2026-10-03, its fix pass was confirmed, and it has been
  `dirty` against main since then.
- **PRs 4037 and 4039** (`topo/joint-elements`, `topo/joint-site-mint`,
  TOPO, `a-kill-that-re-anchors-…`). This is R, ratified by Ev in PR
  4024, which re-worded C4's seam sentence after PR 3919. A row is an
  image (principal branch, a function of the edge and chart alone) plus
  a **stored** per-half-edge joint element, generally *not* the
  identity. The lift is derived by summing elements from `first`
  (`loop_lift`), and tier 3 re-decides each stored element and requires
  it equal to the decided one. `Winding::closes` and the `Reset` element
  at a pole do the closure and pole work.

C4 in `crates/geom-brep/README.md` now says R's form. PR 4024 is the
later ratification, so 3945's "identity at every joint" reads the
pre-R contract.

## The reconciliation

Whichever lands second carries it:
- the deciding arithmetic (3945's `lift_joint` margins, its sphere
  half-period integer with a quarter period of room, and its 3-D pole
  incidence) is what R's stored element should be checked against;
- R's storage, `loop_lift` and kill sums replace "identity at every
  joint".

There is likely one shared decider, not two. As of 2026-10-05 01:20,
TOPO's 4037 is in review-fix with CI running and will merge on green.
If 3945 lands later, its tier-3 and closure rules restate against
stored elements.

## PCERT's answer (2026-10-06)

3945 lands second and carries the reconciliation, as this row asks. The implementer is re-porting onto main's R base:
- R's storage, `loop_lift`, the kill sums and the stored-element reads stay as they are;
- 3945's decider becomes the one decider that tier 3 checks each stored element against:
  - on a sphere, one orbit integer in half periods with a quarter period of room;
  - whole periods elsewhere;
  - metered at the vertex lever;
  - `Reset` only on a decided 3-D incidence;
- check 5, the windows, the analytic continuity margin, the lever pole gate and `chart_boundary`'s closure margins retire;
- spline charts keep their gap.

"Tier 3 requires the identity at every joint" does not come back. The re-ported head gets a fresh review pair.

## Closed

Closed by PR 3945's merge of main (`1f3a5cc2`), which carries the
reconciliation as answered above. There is one decider,
`topo::pcurves::decide_joint`: the walk, the site mint's new joints,
`split_cache` and `turn_element` write what it decides, and tier 3
checks each stored element against it. The other writers compose or
copy, as R has them (kill sums, `revert`'s inverse, null-edge
identities, kept and carried elements, the boolean graft copy); tier 3's
re-decision makes any of them that disagrees loud at rest. It returns a `Reset` only on a decided 3-D
incidence (`singular_at`). R's storage, `loop_lift`, the kill sums and
the stored-element reads are main's, unchanged.

Closure is main's `Winding::closes`. Against 3945's pinned set
(`Lift::closes`) it differs in two places, and main's is kept in both:
- Under the twin it bounds no second-channel periods. Conjugating by
  `T_v` moves `v` by two under the twin, so a bound on `|v|` there
  would depend on which member the composition starts from; main's
  verdict reads conjugation invariants only.
- It counts no azimuth winding across a reset, which 3945 had no
  element to say.

The azimuth half turns it admits (at most 2 without the twin, odd and
at most 3 with it) are the pinned set's.
