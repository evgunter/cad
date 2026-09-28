---
id: loft-between-opposite-turning-joints-reverses-a-seam-between-stations
kind: issue
title: A loft whose sections turn one joint opposite ways reverses that seam between stations, minting an undeclared wedge end, and tier 3 says Ok
status: open
opened: 2026-09-28
priority: P2
cost: H
design: true
refs: [every-op-that-can-mint-a-wedge-end-refuses-an-undeclared-one, self-overlapping-spines-build-and-validate]
---


Found by the GATHER wedge-end door audit
(`work/gather/every-op-that-can-mint-a-wedge-end-refuses-an-undeclared-one.md`).
D1 tier 3 makes an op that could mint a wedge-0/2π edge its inputs
never declared own that refusal. The loft can mint one, and it has no
door.

## The witness

Two valid sections, same vertex count, both CCW, lofted at
`v_degree = 1` onto `z = 0` and `z = 1` (`sweep::loft_body`):

- section 0: `(0,0) (2,0) (0.03,0.347) (-2,1) (-2,-1)`. Joint 1 at
  `(2,0)` turns +170°, a spike with interior 10°.
- section 1: `(0,0) (2,0) (0.03,-0.347) (3,-3) (3,3)`. Joint 1 turns
  −170°, a notch with interior 350°.

Between them the joint's outgoing direction is the linear blend
`(-0.985, 0.174·(1 − 2v))`. At `v = 1/2` it is exactly the reverse of
the incoming `+x`. The seam between walls 0 and 1 passes through
wedge 0 there, and the mid-station section is not simple either (its
edge `(0.5,-1)–(0.5,1)` crosses `(0,0)–(2,0)`). `loft_body` returns
`Ok`, and `topo::validate_geometric` on the body returns `Ok(())`.
Executed on the audit's branch.

## Why nothing catches it

- Each section passes `Profile::validate` on its own. The cusp door
  (`ProfileError::UndeclaredTangency`, `PathError::JunctionCusp`)
  judges a section's joints, and neither section has a cusp.
- Tier 3's material arm exempts an edge with a NURBS face by kind,
  so the seam is never judged. `crates/editor-core/tests/m10_2_measure.rs`'s
  `a_cusp_loft_document_gathers_with_its_nurbs_seam_unjudged_by_kind`
  states the same exemption for a declared cusp.
- `sweep_body` cannot reach this, because every station is the same
  profile. The loft's cross-section correspondence is the author's
  (`loft_body`'s "Correspondence" docs), so nothing compares joint
  turns across sections.

## What a door needs (design)

- **A turn-sign rule on the sections.** Refuse a joint whose turn is
  convex in one section and reflex in another unless every section
  declares it a cusp. This is cheap and exact at `v_degree = 1` if the
  blend passes through the reversal and not through the straight
  (wedge-π) continuation. That depends on the blended directions,
  and at `v_degree ≥ 2` the blend runs through control points, not
  through the sections. So the rule is either conservative (refuse
  any sign change) or it needs a certified bound on the blended
  turn.
- **A per-station verdict on the NURBS seam.** Tier 3's material arm
  lifted to NURBS edges. That is the general answer, and it is the
  exemption-by-kind's own open question.
- The declared-cusp case is the other half. A joint declared a cusp
  in one section and a transverse corner in another has a seam whose
  wedge varies along it. The retired `Lofted::declared_contacts`
  docs (PR 3362) named that case too.

Related: `self-overlapping-spines-build-and-validate` is the same
family (a loft that builds a self-overlapping body and validates).
This row is the wedge-end instance, which D1 assigns to the op.
