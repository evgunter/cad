---
id: two-tour-wall-probes-moved-on-main
kind: issue
title: Two tour wall probes moved on main: the lily's wall 8 and the teapot's wall 3 refuse with refusals other than the ones they pin, so the tour walk panics
status: closed
opened: 2026-11-12
closed: 2026-11-12
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

## Closed (2026-11-12): an honest move, re-pinned

**What moved it.** PR 6862 (`sectarms/join-reads-spline-implicit`,
merge `7d83cdff9e`). Wall 8's union alone, built at `ci-tour`: the
merge's first parent `c8c2b4f70b` refuses `GermSectionSpiric { a_kind:
Torus, b_kind: Plane, loops: AroundTube, section: RoutesToGeneralRung {
pair: "plane×torus", .. } }`, the old pin, in about 4.5 s at ε 1e-9 and
1e-6; the merge refuses `GermSectionUnbuilt { a_kind: Torus, b_kind:
Nurbs, why: Refused(Unaccounted { cause: "…200000-cell budget…" }) }`
in 54 s and 33 s, as main `c51fc90f58` does.

**Why it is honest.** PR 6862 made the join's germ mint carry a held
spline × implicit pair before the join walks any segment. Wall 8's
torus wall × the leaf's ridge wall is such a pair, so the join now
reads a section it never read before, ahead of the plane × torus frame
that refused first; that frame is unchanged and stands behind it. No
refusal got worse on one pair: the kernel meets a different pair first.
The new pair's whole reading meets a real limit, measured: its
accounting spends the whole cell budget clearing a region of the wall a
centimetre from the thin torus (R/r ≈ 83), because the exclusion bounds
the torus's quartic over an ℝ³ box and that bound is about `2R/r` too
wide there. Filed on SSIARITH's slate:
`ssiarith/the-chart-accounting-clears-a-thin-torus-only-in-sub-millimetre-cells`
(P0, blocking `sectarms/a-nurbs-face-against-a-torus-has-no-section-arm`,
which now names it in its `blocked_on`).

**The pin.** `lily::wall_probes`' wall 8 now pins `GermSectionUnbuilt`
with `a_face` the lower stem's torus, `b_face` the leaf's ridge wall
(by its control net's side of the root's chord), and `why:
Refused(SectionError::Unaccounted { cause })` with `cause` naming the
cell budget. It holds at ε 1e-9 and 1e-6; at 1e-12 the lower stem's
declared frontier stands the lily walls down before it. The findings
entry above the probe says the same. The teapot's wall 3 is SECTARMS'
`main-red-the-teapots-spout-join-refuses-in-the-join-since-6862`.

