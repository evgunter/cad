---
id: pcurve-posture-guard-is-blind-to-body-producing-doors
kind: issue
title: The pcurve posture guard walks only &mut Body doors, so every &self -> Body producer's row posture is prose checked by nothing
status: open
opened: 2026-09-14
refs: [revert-does-not-mirror-plane-chart-images, revert-leaves-a-periodic-charts-loop-wrap-mid-chain]
---

Filed by the fix pass of `revert-does-not-mirror-plane-chart-images`
(TOPO, PR 2542), on this slate because the guard is `pcurves.rs`'s.

`crates/topo/src/pcurves.rs`'s posture guard,
`staleness_posture::every_mutation_door_declares_its_pcurve_posture`,
walks `crate::source_walk::mutation_doors` — the `pub fn`s taking
`&mut self` and the free functions taking `&mut Body<T>` — and
requires each to re-mint in its own body or carry a `DECLARED` entry.
The module docs say so ("It reads `topo/src` only", the guard's own
"what it cannot see" list), and the guard's `DECLARED` table refuses
an entry naming a door outside that population. So every door that
takes a body and RETURNS one is invisible to it, and its row posture
is a prose survey: `Body::revert` (`crates/topo/src/revert.rs`, the
producer position the posture docs describe — carries every row key
for key, plane-face rows mirrored with their frames),
`transform_rigid` / `transform_rigid_via`
(`crates/topo/src/transform.rs`, re-derives when the operand carried
caches), `split` (`crates/topo/src/splitting/mod.rs`, the splitting
lane, `Maintains` by the survey), `plane_section`
(`crates/topo/src/splitting/section.rs`), `shell` / `shell_open`
(`crates/topo/src/shell.rs`, the closing mint), and `union` /
`intersect` / `union_with` (`crates/topo/src/boolean/ops.rs`, the
boolean pipeline's finished-body mint). Each is a `Maintains`-or-
carries claim the module docs make in prose; nothing goes red when
one of them stops re-minting, or starts.

Why it matters now: `revert`'s posture was the one that was wrong
(its plane rows travelled unmirrored — the closed TOPO item), and it
was found by a SHELL diagnosis lane's probe rows, not by the guard.
The two-arc sphere's loop wrap
(`revert-leaves-a-periodic-charts-loop-wrap-mid-chain`) is the same
door's next residue and is likewise covered only by prose and by
`sweep`'s SHELL-9 probe rows.

Closing shape, this program's call: extend `source_walk` with a
second population — `pub fn`s and free functions whose return type
names `Body<T>` (or `Result<Body<T>, _>`, or a struct carrying one,
as `shell`'s `Shelled` does) — and require each to call the pass in
its own body or carry a declared posture, the same three mechanical
checks the existing walk makes; or record in the guard's docs that
the producer population is deliberately a survey, with the list
above as the census so a new producer is at least a doc diff. The
first is the one that would have caught `revert`.
