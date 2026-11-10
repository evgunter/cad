---
id: check-7s-quadrature-refuses-interval-nurbs-walls-with-inexact-pcurve-ends
kind: issue
title: Check 7's quadrature refuses an Interval NURBS wall whose pcurve ends are not exact 0/1, so a boxed loft cannot finish
status: open
opened: 2026-11-02
priority: P1
cost: H
refs: [6371]
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
