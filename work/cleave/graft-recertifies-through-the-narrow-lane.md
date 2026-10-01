---
id: graft-recertifies-through-the-narrow-lane
kind: issue
title: boolean graft re-certifies through the plain certify door, the second instance of the transform split
status: dispatched
opened: 2026-09-12
priority: P1
cost: M
branch: cleave/nurbs-lane
---


Disclosed by `transform-recertifies-through-the-narrow-lane`, which
named it and left its reachability unestablished. That unit gave
`transform_rigid` a lane-injecting twin; this is the same shape at the
other door, and it is filed rather than taken because its class was
NOT established as reachable.

## The site

`boolean::combine::graft_solids_with` re-certifies every grafted
carrier against the destination body's points and surfaces through the
plain `geom_brep::EdgeCurve::certify`
(`crates/topo/src/boolean/combine.rs:440`, inside the curve pass), so a
grafted edge of the M7-8 class — an `Intersection` between a plane and
a DESCRIBED NURBS wall — would refuse `CertifyError::Unimplemented`,
surfacing as `BooleanError::GraftRecertify`. The split is stated at
neither door.

## Why it is not that unit's fix

**The bound cannot simply be raised.** `graft_solids_with` is
`T: geom_core::Decide` and sits under `boolean_op_with`, which
`verbs::Verb`'s `impl<T: Decide + Bounds + topo::AtRestPolicy>`
block runs and the dual corpus instantiates at `Dual64`
(`crates/editor-core/tests/r1_dual_probes.rs`). No `Dual` implements
`geom_core::CertifiedEnclosure`, so tightening this chain to
`CertifiedBounds` does not compile — the hazard `crates/geom-core/src/real.rs`
already records for that same `verbs` block. The remedy is the one
`transform_rigid_via` took: `EdgeCurve::certify_via` with the lane as
an argument, and a `_via` door on the graft for a caller that can name
it.

## What was NOT established

**Whether a body of that class ever reaches the graft.** The boolean
pipeline has a NURBS re-gate ahead of it, so the refusal may be
unreachable in practice and this may be a latent split rather than a
live defect. Establishing it needs a boolean over a described-NURBS
operand that survives to the graft, which was not cheap at the time of
filing. A unit taking this row should settle reachability FIRST: if
the class cannot reach the graft, what is owed is the sentence at the
door and not a new door.

## Re-homed to REACH, 2026-09-20

(FIX orchestrator) Ev, in chat, 2026-09-20: *"can you kick all the design decisions back to
the track they actually belong to, leaving fix design-free?"* FIX is the
program for rows whose fix is already written; a row whose blocking
question is a DESIGN decision belongs to the track that owns the surface
the decision is about.

**The decision this row is blocked on:** whether a body of the M7-8 class ever reaches the graft at all (reachability
was explicitly NOT established when the row was filed), and only then whether
`graft_solids_with` gets a `_via` door.

**Why REACH.** `crates/topo/src/boolean/combine.rs` is **owned by reach**
(`crates/topo/src/boolean/*` in its `paths`), and REACH's slate is where the
boolean pipeline's reachability questions already live.

**The bound cannot simply be raised** and the row says why:
`graft_solids_with` is `T: geom_core::Decide` under `boolean_op_with`, which
`verbs::Verb`'s blanket impl instantiates at `Dual64`, and no `Dual` implements
`CertifiedEnclosure`. Its sibling
`plain-transform-rigid-still-refuses-the-m7-8-class` moves to SHELL in this
sweep with the same structural constraint and the same ratified discriminator
(`crates/geom-core/src/real.rs`, Ev 2026-08-29: *"the discriminator is that
nothing generic calls this door"*). If either program rules, the other should
read that ruling before re-deriving it.

Nothing about the finding is changed by the move: same id, same
evidence, still `open`, and no part of its question is answered for
you except where this note says Ev answered it.

## Reachability, measured (CLEAVE, 2026-10-01)

**No boolean reaches the graft's plain certify.** An M7-8 edge
certifies only on a `Curve3::Nurbs` carrier, and the boolean refuses
those everywhere ahead of the graft:
- the operand gate (`boolean::reduce::gate_operand_edges`) refuses them
  as `CurvedEdgeUnsupported`;
- the join has no plane × NURBS arm;
- the crossing layer refuses NURBS faces;
- the containment fallback refuses a NURBS face as
  `NurbsExtentUnsupported`.

Against `m4_pr2_transform`'s `m7_8_cube`, `subtract(big, cube)` refuses
`CurvedPairUnsupported`, and `union(cutter, cube)` and
`subtract(cube, cutter)` refuse `CurvedEdgeUnsupported`.

**The public void door reaches it.** `topo::insert_void` (and
`insert_voids`) calls `cavity.revert()` and then
`graft_solids_with(.., Bridge::Recertify, ..)`, with no NURBS gate on
the way. Take the M7-8 cube as the cavity inside a [−1, 2]³ brick,
with true `VoidContainment::Carried { sign: Positive }` evidence. It
returns `Err(VoidInsertError::Recertify(Unimplemented))`. That is the
plain door refusing a body the lane-injecting door would certify.
Whether shell or offset can feed that door an M7-8 cavity has not been
established. The question is therefore live, and it is the same one as
`work/shell/plain-transform-rigid-still-refuses-the-m7-8-class`: which
door the capability is reached through, given that the bound cannot be
raised on a chain `verbs::Verb` instantiates at `Dual64`.

## Decided (CLEAVE designer pair, 2026-10-01)

The two designers converged after one reconciliation round.
- **The certification right belongs to the scalar.** The plane × NURBS
  lane becomes a sealed value (`NurbsLane::certified()`, the shape
  `FittedLane` has) held per scalar by `AtRestPolicy::nurbs_lane()`.
- **The transform re-derives.** `transform_rigid` reads its lane from
  that policy, and `transform_rigid_via` goes.
- **The graft carries certificates.** The graft copies geometry bit for
  bit, so `insert_void[s]` uses `Bridge::RemapKeys` and does not
  re-certify.
- **A scalar without the lane gets its own refusal.**
  `CertifyError::NurbsLaneUnsupported` replaces `Unimplemented`, and
  `needs_nurbs_lane` retires.

This applies H5 ruling 3 (PR 2701) to the one lane that predates it. No
ratified text changes. The `real.rs` M7-8 paragraph is agent-written,
and it is re-worded to match the new code. `AtRestPolicy`'s rustdoc counts
its doors ("three doors, one policy"); that moves to four in the same
change. Collapsing the mint doors (`set_edge_curve` reading the
policy) is a second unit, sized separately: the bound raise reaches 46
call sites.
