---
id: two-tour-wall-probes-moved-on-main
kind: issue
title: Two tour wall probes moved on main: the lily's wall 8 and the teapot's wall 3 refuse with refusals other than the ones they pin, so the tour walk panics
status: open
opened: 2026-11-12
priority: P1
cost: E
---


## Finding

Found by CLEAR's lane clear-gate while walking the tour
(`demo_tour::walk_tour`) at main `348ec09727`, release profile, at one
thread and at the default thread count alike. Two wall probes panic in
`walls::wall` (`demos/tour/src/walls.rs` ~33) because the refusal
moved:

- the lily's wall 8, "grow the long leaf out of the stem" (`lily::wall_probes`,
  `demos/tour/src/lily.rs`): it now refuses
  `GermSectionUnbuilt { a_kind: Torus, b_kind: Nurbs, why: Refused(Unaccounted { cause: "ssi: the exhaustiveness subdivision exceeded its 200000-cell budget — refused rather than truncated" }) }`;
- the teapot's wall 3, "join the CANAL to the vessel"
  (`teapot::wall_probes`, `demos/tour/src/teapot.rs`): it now refuses
  `Join(BooleanJoin(Euler(Certification { error: Unimplemented })))`.
  That half is already owned by
  `sectarms/main-red-the-teapots-spout-join-refuses-in-the-join-since-6862`,
  which traced it to PR 6862 and holds its fix; this row owns the
  lily's wall 8 only, and closes when that probe pins what the kernel
  does now (or the kernel builds the leaf).

Both refusals come from the Boolean (`crates/topo/src/boolean/`), hence
this slate. The last render on record (`render.yml`, 2026-11-09) passed
on a branch, and the nightly's failing jobs do not include the walk,
so nothing gates it at the moment.

## Fix shape

Per the probe's own message: re-derive each probe and its findings-list
entry, or fix the kernel if the new refusal is a regression.
