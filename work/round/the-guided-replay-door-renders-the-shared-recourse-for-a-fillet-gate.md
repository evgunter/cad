---
id: the-guided-replay-door-renders-the-shared-recourse-for-a-fillet-gate
kind: issue
title: replay_guided wraps an in-band fillet verdict so the caller reads the shared coincidence recourse, not the gate's sentence
status: open
opened: 2026-09-13
priority: P1
cost: E
---


## Finding

BLEND-12 gave `PathError::Escalated`'s `Display` a fillet arm, so an
in-band `fillet_*` verdict reaching a caller through the ordinary path
door renders the gate's own sentence and no coincidence tail. **There is
a second mouth of the public door that it does not cover.**

Under a guided replay (`profile::replay_guided`) the arc-carrier
construction wraps the same `Indeterminate` in a structure refusal
instead of a `PathError::Escalated`
(`crates/profile/src/path/arc_fillet.rs`, the three
`StructureRefusal::indeterminate(Decision::…, source)` sites: the corner
gate, the reach gate and the `ArcFilletOutcome` arm). Its `Display`
(`crates/profile/src/structure.rs`, `StructureRefusalKind::Indeterminate`)
prints the `Indeterminate` WHOLE — and `Indeterminate`'s own `Display`
ends in `geom_core::COINCIDENCE_RECOURSE`. So through that door the
caller reads "declare the coincidence, move the geometry, or lower the
tolerance" at a fillet they asked for, exactly the defect the fillet arm
was written to repair, and the gate's tailored sentence appears nowhere.

Driven, not inferred:
`crates/profile/tests/review_fillet_recourse_arm_r1_probes.rs`'s
`a_guided_replay_still_renders_the_coincidence_recourse_for_a_fillet_gate`
replays a recorded program with the fillet radius moved into the band,
and asserts the rendered text names `'fillet_offset_line_circle'`, DOES
contain the shared recourse and does NOT contain
`FILLET_NO_CORNER_RECOURSE`. It is a characterization: it goes red the
day this is repaired, and says so in its own message.

## Why BLEND-12 did not repair it

Two reasons, both worth stating rather than one:

1. **Fence.** BLEND-12's seam is `crates/profile/src/{validate,path,lib}.rs`
   plus `crates/profile/tests/**`. `structure.rs` is neither, and it is
   the guided lattice's own file.
2. **It is not obviously a routing bug.** The two situations differ. The
   ordinary door's is "you asked for this fillet and the gate could not
   classify it", whose levers are the geometry's. The guided door's is
   "a decision this recorded structure already took cannot be
   re-verified at this scalar", whose lever is the parameter box — and
   the wrapper says exactly that ("narrow the parameter box and try
   again"). Following the fillet sentence there would change the
   geometry being replayed, which is not what a guided pass is for.

So the question owed is whether the guided refusal should carry BOTH —
its own lever and the gate's, with the shared coincidence tail dropped
either way — or whether the shared tail is simply wrong at every
structure refusal and the fix belongs to `Indeterminate`'s `Display`
seam rather than here. That is the same question
`work/blend/escalation-recourse-dispatch-has-three-homes.md` and
`work/blend/every-escalation-carries-the-coincidence-recourse-first.md`
raise from the routing side, and it wants deciding with them.

## What a repair costs

`StructureRefusal`'s `Display` would compose from `source.payload()`
(the payload view without the shared tail) and append the routed
sentence when `validate::fillet_recourse_for` knows the name — the same
shape `PathError::Escalated` already uses, and the one map is already
the single home for the mapping. The characterization row above is the
re-baseline target; nothing else in the tree asserts that text.
