---
id: the-side-reader-cannot-read-a-solids-orientation-over-a-box
kind: issue
title: The side reader cannot read a solid's orientation over a macroscopic box: every aimed ray's boundary margin blows up
status: dispatched
opened: 2026-11-02
priority: P0
cost: M
refs: [6371]
branch: boxfinish/aimed-ray
---

Split from `the-sweep-doors-finish-refuses-what-the-box-studies-read`
(PR 6371). This is one of the rows that retire the box-scalar switch at
`topo::AtRestPolicy::leaf` (`crates/topo/src/props.rs`, the switch). Ev on PR 6371 (2026-11-02): "we should prioritize making it
actually work for all these cases, with the common cases as P0".

**What fails.** With the switch off, the sweep doors finish their own
result at `Interval`/`Sym`, and check 7 asks the side reader
(`crates/topo/src/side_reader.rs`) which side of the solid's faces holds
its material. Over the slab box of
`editor-core/tests/m10_3_driver_interval.rs::a_macroscopic_box_certifies_at_the_shipped_depth_budget`
(±0.05 on a 1.0 slab, one leaf) all 6 aimed rays are set aside on the same
test, whether a crossing lies inside the face's boundary:

- the aim points are box-wide enclosures (z ∈ [0.86, 1.14] on the top
  face);
- the boundary margins come back `[-0.91, 2.99]` and `[-5.22, 3.13]`
  against the (1e-9, 1e-8) band, and one ray's comes back NaN/refused
  ("6 of the 6 rays aimed at its faces read nothing",
  `side_reader.rs:192`);
- E12's symbolic zero does not discharge them, since they are not
  identically zero;
- bisection spends the 32-leaf budget, and the box certifies 0.

**Who it holds.** The slab box above; the derived-frame boss's plain and
A0-alone rungs at 5e-2
(`m10_derived_frame_interval.rs::m10_the_derived_frames_refusal_is_not_a_freeze`,
and the shipped rules in
`m10_the_derived_frame_extrude_agrees_with_its_authored_twin_below_that_width`);
and the tour's certified stop (`demos/tour/src/tolerance.rs`,
`eps_regression::certified_cells_run_green_at_default_eps`), which
reported 0.0000% certified, 99.4607% refused (budget).

**The test that says it is closed.** Delete the switch's box arm (or
make `leaf` call `finish` at `Interval`/`Sym`) and run those three rows:
each must pass with its main-era assertion (one leaf certifies; the
boss's rungs certify; the tour certifies its share).

**An untried idea.** Aim the side reader's rays at points that are exact
in the parameters (a face's own exact point, not an enclosure derived
across the box), so a crossing margin carries the face's own width and
not the box's. Unmeasured.

## Slate triage (2026-11-08, ONEHOME split)

- Moved to BOXFINISH (`work/boxfinish/`) by `git mv`, id and body unchanged. ONEHOME measured 78 budget points against 35 after PRs 6371 and 6380 landed, and split along its seams (`work/README.md` Track size): the rows that let the box scalars finish their sweep leaves at tier 3 and so retire Ev's switch (PR 6371) went to BOXFINISH.

## Measured again (2026-11-14, main `b363f5fa`, switch off), and re-scoped to the ray

BOXFINISH's M1 lane (`boxfinish/probe-switch-off`, `probe/M1-REPORT.md`).
What the text above says is corrected here:

- **The slab does not spend the 32-leaf budget and certify 0.** It
  certifies in four leaves, by the volume. The margins quoted above are
  not today's.
- **The width has two sources, and this row keeps one.** The stored
  plane normals carry the box
  (`the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box`).
  The other is the ray's own construction, below.
- **"Aim at exact points" alone is measured insufficient.**
- **The tour's stop is not the side reader's.** It is check 1 on a
  cylinder's `u_ref`, in the row just named. The derived-frame rows
  refuse at the widened cube, which is the slab again.

**The ray loses what it was built from** (`side_reader::aimed_ray`). On
the slab's bottom cap the aim, the normal and `out` are exact (width
1e-16), and still:

- `beyond` is `[1.392, 1.544]`, so `from` is (0.04, 0.04, 0.30) wide;
- `d = (aim − from).normalize()` is (0.026, 0.024, 0.204) wide, though
  it equals `−out`;
- the crossing point `p = from + d·t` is (0.23, 0.21, 1.80) wide,
  though it equals the aim;
- six `side_at_infinity_edge` gaps read `[0, 1.6..3.0]`, and the ray is
  set aside on `side_at_infinity_winding`, `MarginDiag(Invalid)`, raised
  by `turned_inside` over `loop_turn`'s atan2 sum (inferred: a box that
  holds the origin).

By hand substitution on the slab, normals as stored: the ray as built
certifies in four leaves by the volume; with `d = −out`, four leaves by
the rays (205 to 220 ms a leaf, against 465 to 540); with a ray
constant across the box, two leaves by the rays (55 to 61 ms a leaf).
With the normals substituted too, one leaf.

**The unit.** An aimed ray keeps, across the box, what it is by
construction: its direction is `−out`, and it meets its own face at its
aim. **The test that says it is closed:** over the slab at the stored
normals, the rays read at a width where every one is set aside today,
and no ray reads a side it should not: a body whose side differs
across the box still refuses. Branch `boxfinish/aimed-ray`.

**Letter M; review tier: dual (sequential).** The side reader decides a
solid's orientation at every at-rest gate, so an unsound ray is a live
wrong answer, and its rays have been wrong before in ways a read of the
diff did not show.
