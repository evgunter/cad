---
id: a-mate-through-a-parametric-placer-is-solved-at-the-nominal-in-box-and-seed-runs
kind: issue
title: A mate read through a parametric Transform or Pattern is solved at the nominal in a box or seed run, while the placer's own map widens
status: open
opened: 2026-09-29
priority: P2
cost: M
design: true
parent: MSOLVE-14
---

Filed by the EDIT orchestrator from the design-fork review of the
placement unit (`work/place/placement-is-spelled-three-ways-node-registry-and-rule`,
`[ev]` PR on branch `edit/ev-placement-design`). Both designers
found it independently. They disagreed only on whether the fix is a clause of
that unit or a row of its own; it is filed here because the class
predates the unit.

## The finding (read, not yet probed)

- **Every lane runs the mate solve at the nominal.**
  `evaluate` solves once, at `doc.param_env::<f64>()`
  (`crates/editor-core/src/eval/mod.rs:2971`, `:2989`,
  `crate::mate::solve_with_env(doc, &nominal_env, ..)`). That holds in
  a box (interval) run and in a seed (derivative) run too.
- **Every instance reads its pose as that f64 frame.**
  `wire.rs:338` calls `env.poses.placement(doc, id)`, and
  `wire_instantiate_part` lifts the frame into the lane's scalar.
  `mate/` has no box or seed handling.
- **So the mated part's pose never widens.** Take a mate whose placer
  offset is read through a `Transform` or `Pattern` whose slots
  reference a parameter the box or seed binds. The placer's own map
  widens in the lane, but the solved relative pose stays the nominal's.
- **So the enclosure can omit the true pose.** A box run then reports
  an enclosure that may not contain where the mated part sits at some
  parameter value in the box, and a seed run reports a zero sensitivity
  for the mated part's pose.
- **This contradicts what `refuse_param_box` promises**
  (`eval/mod.rs:3131-3133`: "an `f64` run that silently ignored its box
  would report the nominal build's answer for a question about a box").

## The fix both designers named

- **Refuse typed** when the run's box or seed binds a parameter that a
  conjugated placer reads. This is a structural check: the bound names
  intersected with the names the placer references.
  - Once the placement unit lands, the check also covers a parametric
    cluster placement.
  - The placement unit must not ship a parametric placement into the
    lanes before this refusal exists. They land together, or this row
    lands first.
- **Later, lift the refusal** by re-composing the relative pose at the
  lane's scalar over the f64 solve's tree (a "guided" re-composition).

**First step:** a red probe. Take a box run over a document whose mate
reads through a parametric `Transform`, and show the mated part's
enclosure omitting its pose at a box corner.


## Widened (2026-10-01, PR #3676, EDIT P2-core)

A checked offset is decided at the nominal too. `mate/solve.rs`'s
`check_offsets` composes each member's world pose over the group's
frame (`group_frame`: the gauge chain and the root's offset) whenever a
placer stands on either path, and reads that frame at the document's
own parameters, as every number the solve reads is. A box or seed run
that binds a parameter of a gauge or a root offset therefore checks the
member's stated offset at the nominal while the lane composes the
instance at the box. It is this row's class — one solve answer read in
a lane that moved what it was decided over — and the refusal this row
proposes covers it when it also intersects the bound names with the
names a parametric gauge or root offset reads. With no placer on either
path the frame cancels and is not read, so the check is lane-exact.

## Weighed (2026-10-01)

Plan item 19 gathers this row and its sibling
(`from-face-frame-under-an-analysis-lane-refuses-unpinned`,
`a-mate-through-a-parametric-placer-is-solved-at-the-nominal-in-box-and-seed-runs`)
into one design fork, which two designers weighed on
`msolve/ev-analysis-lane-solve`. That PR adds the sentence to
`ASSEMBLY.md` A11 rule 5 that the recommendation would make true.

## Ruled (Ev, `[ev]` PR 3679, 2026-10-01)

Approved: the mate solve runs at the evaluation's own scalar, over the
evaluation's own parameters. A pattern's count and a `Part`'s index
are read at the nominal, because no box or seed binds them.
`ASSEMBLY.md` A11 (5) states this in place. The build is an MSOLVE
unit: the solve goes generic over the scalar, and `Unpinned` loses its
producer.

