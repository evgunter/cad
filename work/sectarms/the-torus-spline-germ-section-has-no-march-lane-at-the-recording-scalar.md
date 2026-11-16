---
id: the-torus-spline-germ-section-has-no-march-lane-at-the-recording-scalar
kind: issue
title: At the recording scalar the torus × spline germ section refuses NoMarch before its accounting, so the K sweep reads the lily's wall 8 one refusal short of the tour
status: open
opened: 2026-11-15
priority: P3
cost: M
---


Filed by GREEN's `green/nightly-1115` lane. It takes over what is still
open of
`work/green/the-tours-k-probe-reads-lily-wall-8-at-the-probe-scalar-where-it-refuses-nomarch.md`
(closed there with the pin), whose measurement was made by BOXFINISH's
chart-ends lane.

**What main does.** The lily's wall 8 (`demos/tour/src/lily.rs`,
`wall_probes`, `try_union(stem, leaf_a)`) carries the germ pair of the
lower stem's torus wall against the long leaf's ridge wall, a torus ×
spline section:

- at `f64` (the tour) it refuses `GermSectionUnbuilt { a_kind: Torus,
  b_kind: Nurbs, why: Refused(Unaccounted { cause ∋ "cell budget" }) }`,
  the refusal CLEAVETAIL pinned on 2026-11-12 after PR 6862
  (`work/cleavetail/two-tour-wall-probes-moved-on-main.md`);
- at `geom_core::Probe` (the K sweep, `demo-tour k-probe`) it refuses the
  same pair with `why: Refused(NoMarch { scalar: "telemetry probe" })`:
  `pair_section`'s spline arms take `lane.ok_or(SectionError::NoMarch)`
  and the recording scalar is handed no march door.

So one wall has two refusals by scalar, and the sweep that exists to
record the tour's margins stops one step before the accounting the tour
runs. GREEN pinned the wall per scalar (`<S as Real>::NAME`), so the two
`k_probe_brackets` rows and the nightly's `k-lint (dev-probe)` sweep run
through, and the pin reds if either scalar's refusal moves.

**Not bisected.** PR 6862 (`sectarms/join-reads-spline-implicit`, merge
`7d83cdff9e`) is the candidate: before it the wall refused
`GermSectionSpiric` on the plane × torus pair at `f64`, a refusal that
needs no march. Whether the Probe reading moved at that merge was not
measured; the rows were red on `bdb498df` already (BOXFINISH's
measurement).

**Owed.** Decide whether the recording scalar should march (so the K
sweep records the margins of the section the tour actually reads) or
whether a wall the sweep cannot follow should be skipped by it with a
stated reason, and re-state wall 8's probe to match.

Reproduce: `cd demos/tour && CAD_TOLERANCE_EPS=1e-9 cargo run --features probe -- k-probe /tmp/k.csv`
(with the pin reverted it panics at `walls::wall`, wall 8).
