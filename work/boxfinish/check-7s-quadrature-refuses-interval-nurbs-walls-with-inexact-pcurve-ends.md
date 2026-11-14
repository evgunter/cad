---
id: check-7s-quadrature-refuses-interval-nurbs-walls-with-inexact-pcurve-ends
kind: issue
title: Check 7's quadrature refuses an Interval NURBS wall whose pcurve ends are not exact 0/1, so a boxed loft cannot finish
status: dispatched
opened: 2026-11-02
priority: P1
cost: M
refs: [6371]
branch: boxfinish/chart-ends
---

Split from `the-sweep-doors-finish-refuses-what-the-box-studies-read`
(PR 6371). This is one of the rows that retire the box-scalar switch at
`topo::AtRestPolicy::leaf` (`crates/topo/src/props.rs`, the switch). Ev on PR 6371: "nurbs might be P1".

**What fails.** With the switch off, an `Interval` loft or sweep over
NURBS walls finishes at the door. Check 7's quadrature refuses a NURBS
wall whose pcurve endpoints are not exact structure
(`QuadratureUnsupported`: "the rectangle-trim certificate needs the
minted exact 0/1 chart values"). The door then refuses with
`LoftError::NotFinished { VolumeUncomputable }`. Every `Interval` loft
in the sweep suites hit it. Those suites read the door's construction
state through
`sweep::test_support::{loft_body,loft_profiles,sweep_body}_construction`,
and the document row
`editor-core/tests/eval10_section_reads_the_nominal.rs::a_box_over_a_section_width_lofts_the_family_it_spans`
evaluated no boxed loft at `Interval`.

**The test that says it is closed.** With the switch's box arm deleted,
that eval10 row lofts the family it spans (its main-era assertion), and
the sweep suites' `Interval` lofts read the door.

## Slate triage (2026-11-08, ONEHOME split)

- Moved to BOXFINISH (`work/boxfinish/`) by `git mv`, id and body unchanged. ONEHOME measured 78 budget points against 35 after PRs 6371 and 6380 landed, and split along its seams (`work/README.md` Track size): the rows that let the box scalars finish their sweep leaves at tier 3 and so retire Ev's switch (PR 6371) went to BOXFINISH.

## Measured again (2026-11-14, main `b363f5fa`, switch off): it needs no box

BOXFINISH's M1 lane (`boxfinish/probe-switch-off`, `probe/M1-REPORT.md`;
two probe tests added there to `a_loft_computes_its_walls_in_t.rs`).

- **The refusal is not a boxed loft's.** The same loft with every
  section a point interval, no box at all, refuses the same way:
  `Loft(NotFinished { [VolumeUncomputable { face 3v1, QuadratureUnsupported
  { "… minted exact 0/1 …" } }] })`, raised by the `exact` closure in
  `topo::props::quad_lane::nurbs_face`.
- **The end is an exact 0 or 1 that went through arithmetic.** The
  stored image is `IsoLine { p0, pl }` over the edge's own parameter,
  read as `image.eval(t1) = p0 + pl·t1`. It is minted by
  `topo::pcurves::side_image_of` (`slope = (b − a)/span; p0 = a − slope·t0`)
  and `sweep::loft::describe_section_rim` (`slope = 1/(t1 − t0)`), and
  outward rounding makes the end 2.2e-15 wide on the point loft
  (`pl.x = 1/2.2`), 5.6e-16 on a unit rim, and 1.56e-11 on eval10's
  boxed rim. Ten of thirteen printed half-edges read an exact 0 or 1.
- **Why check 7 reaches the quadrature:** the side reader casts no ray
  at a selection holding a NURBS face
  (`set_aside = [Limit(KindUnsupported { face 3v1, kind: Nurbs })]`,
  `cast_aims`' `uncrossable`), so a loft always goes to the volume.
- **Not measured:** what refuses next once the ends are exact. The unit
  measures that before it builds.

**The unit.** A chart end that is an exact 0 or 1 by construction is
stored so that it reads as one. Branch `boxfinish/chart-ends`.

**Letter M; review tier: single, FULL.** What reads as exact feeds the
rectangle-trim certificate, so an end wrongly read as exact certifies a
volume over the wrong region.
