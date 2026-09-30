---
id: kfmrh-removes-a-face-or-shell-another-record-names
kind: issue
title: kfmrh removes f2, and in its fusion form f2's shell, proving no other record names them: DanglingTopology through Ok
status: review
pr: 3592
branch: topo/kill-proves-half-edges
opened: 2026-09-30
priority: P3
cost: E
refs: [kef-kvfs-and-mekr-remove-a-face-shell-solid-or-edge-another-record-names]
---

## What

Found by the receipt of the kill-proofs unit (PR 3570), which re-derived
every production `.remove(` on a topology arena in `crates/topo/src`
and mapped each to the proof that covers it.

`Body::kfmrh` (`crates/topo/src/euler_ring.rs`, `kfmrh_execute`, the
`self.faces.remove(f2)` and, in the cross-shell fusion form,
`solid.shells.retain` then `self.shells.remove(f2_shell)`) removes
`f2` and its shell on the strength of what it reads from `f2` and
`f2`'s shell: `f2.outer` (the loop it demotes) and `s2.faces` (the
faces it re-homes). `kfmrh_plan` proves neither converse:

- a loop other than the demoted ring whose `face` is torn to `f2`, or a
  shell other than the one that drops `f2` listing it, is left naming
  a dead face;
- in the fusion form, a face outside `s2.faces` whose `shell` is torn
  to `f2`'s shell, or a solid other than `s2.solid` listing that shell,
  is left naming a dead shell. In the same-shell form, `f1`'s shell is
  the one that drops `f2`; in the fusion form it is `f2`'s own, and a
  torn `f1`-shell list naming `f2` is left dangling too.

Not measured: `review_d18::kill_anchors_on_torn_bodies` does not drive
`kfmrh`.

## The shape to give

PR 3570's spine proofs apply as they stand: `Body::require_face_unnamed`
for `f2` (clearing the ring, which the kill re-homes, and the shell
that drops `f2`) and, in the fusion form, `Body::require_shell_unnamed`
for `f2`'s shell (clearing `f2` and the re-homed faces, and
`s2.solid`), both refusing `EulerOpError::KillLeavesDangling`, after
the ring's walk in `kfmrh_plan`. Add `kfmrh` to the probe's operators
and pin one row per record.
