---
id: a-staging-doors-end-derives-tier1-once-and-its-orphan-sweep-is-batched
kind: unit
title: A staging door's end derives tier 1 once, not two or three times, and the attach setter's orphan sweep is one batched pass, not a whole-arena scan per removed key
status: closed
opened: 2026-10-24
priority: P1
cost: M
refs: [doors-still-read-the-whole-body-for-tier1, offset-doors-take-solids-and-a-per-chart-rule]
closed: 2026-10-24
pr: 5661
branch: offset/door-end-tier1-once
---


Cut from `doors-still-read-the-whole-body-for-tier1` after a designer
pair weighed it (2026-10-24; reports in `work/offset/log.md`). Both
designers rejected a scoped tier 1 — the item's suggested closure — as
a second validator by another name, and both found the door's
whole-body cost lies elsewhere. No ratified decision moves: D1's "one
whole-body tier-1 re-derivation per public door" is what this makes
true.

## 1. One derivation per door end

Every staging door clones the body, mutates the clone under a surgery
scope, closes with `sweep_and_close` (a whole-body `validate`, under
debug assertions) and then gates with `validate_closed`, which runs the
same `tier1` again before its tier-2 checks. That is two derivations;
`boolean::ops::structural_gate` adds a third `validate`.
`validate::closed_by_tier` already derives tier 1 once and splits the
findings by tier.

Build: a `Surgery` close for a door that gates its clone at tier 2. It
runs `closed_by_tier` once, `debug_assert`s the tier-1 half empty (D1's
postcondition, unchanged in what it checks), and returns the tier-2
half typed for the door's `ResultNotClosed` / `ResultInvalid`. With
assertions off, a non-empty tier-1 half is returned typed as today.
Every staging door moves onto it (`replace_faces_offset`, the three
simultaneous doors, merge, the boolean's and splitting's finish,
`shell`); `structural_gate` drops its extra `validate`.
`review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
learns the new spelling. Re-word, with the change, `surgery.rs`'s
paragraph calling the sweep-and-gate pair "deliberate" (agent text,
`9f947f0e95`: the two contracts, panic on a kernel bug and typed on a
tier-2 state, share one walk) and `offset_together.rs` `Scope`'s
account of its O(body) reads and their 2026-09-08 numbers. Settle the
two descriptions of `ResultNotClosed` (typed but worded
`KERNEL_DEFECT_ENDING`; `surgery.rs` calls it a refusal the caller is
entitled to).

## 2. The orphan sweep is batched

Measured by one designer (release, debug assertions on, callgrind):
offsetting every face of one n-gon prism costs 0.08 / 0.32 / 1.9 / 18
/ 237 ms at n = 4 / 16 / 64 / 256 / 1024, while a whole-body
`validate_closed` on the 1024-gon is 0.9 ms. The door is **quadratic
in its own scope**, and about 90% of it is the attach setter
(`set_face_surfaces_describing` / `_certifying`, `crates/topo/src/attach.rs`):
`Body::remove_surface_if_orphaned` and `remove_curve_if_orphaned`
(`crates/topo/src/body.rs`) scan every face, curve and edge for each key
they are asked about, so the cost is writes × body (about 40% of the
512-gon profile alone). `stranded` / `rechart_edges(self.edges.keys(),
…)` walk every edge of the body, and `moved_slot` / `written.iter().any`
are linear finds.

Build: one batched orphan pass per setter call (collect the candidate
keys, then one refcount pass), and the setter's walks driven by the
keys it wrote. Pin the result with a deterministic counter (arena
visits, not a timing). Sweep the door files for the same shape:
`offset_together.rs`'s `plane_of` / `point_at` and `replace_face.rs`'s
`group_by_point` / `split_by_solid` are linear finds over the scope;
`offset_axial.rs` likely matches; tier-1 pass 13 scans every edge per
null-scaffold curve. Record the hit list.

## Not here

- The decide-phase walks iterating `Scope`'s `SecondaryMap`s instead of
  the arenas rides SHELF's `offset-doors-take-solids-and-a-per-chart-rule`
  (a note is on that row).
- `shell_open` staging once per solid: SHELL's
  `shell-open-stages-a-door-per-solid`.
- A small move in a large multi-solid body stays linear in the body:
  the clone and the one walk are D1's per-door cost, and the many-solid
  case that actually hurts is PERF's
  `graft-door-sweep-is-quadratic-in-assembly-instances`.

Review tier: single FULL (a gate's two contracts merge into one walk;
the panic/typed split is the claim to falsify).

## Closed (2026-10-24, PR 5661)

`Surgery::close_gated` derives tier 1 once at a staging door's end (a
tier-1 finding panics naming the door, tier 2 comes back typed); the
three simultaneous offset doors and the merge close with it, and
`structural_gate` lost its extra `validate`. The attach setter's orphan
pass is one batched reference count per call, its walks are driven by
the moved faces, and `attach::tests::orphan_sweep_scales_with_the_writes`
counts it (7n+2 arena visits). The n-gon prism offset went from 2.04 /
17.2 / 224 ms to 1.15 / 6.0 / 49 ms at n = 64 / 256 / 1024.

Residue, each in its own file:
`staging-doors-gated-at-tier-3-still-derive-tier-1-twice` (the boolean,
shell, split and STEP import gate at tier 3),
`wedge-reach-folds-a-faces-whole-boundary-per-edge`, SHELF's
`offset-charts-staged-corner-solve-walks-the-edge-arena-per-group`,
TOPO's `set-edge-curve-scans-the-edge-arena-per-call`,
`an-offset-doors-trailing-join-sweeps-tier-1-per-kill`. The merge door
closes gated but still derives tier 1 twice (its re-description scope),
recorded in the tier-3 row.
