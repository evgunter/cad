---
id: two-depth-limited-atom-walks-have-no-memo-and-cost-the-product-of-the-fan-out
kind: issue
title: Two depth-limited walks through atom arguments have no memo: a node's test costs the product of the fan-out over the levels, not the sub-tree's size
status: open
opened: 2026-11-24
---


Found by the sweep of BOXFINISH's report-bound unit (2026-11-24), which
bounded the shape report's renderer
(`crates/geom-core/src/sym/report.rs`, `Pen`): that renderer had a depth
limit (`DEPTH = 4`) and no bound on breadth, each atom spelling its own
argument at every site, and a 241-node form rendered to 16.7 MB (fan-out
16, five levels: `16^5` atom sites). Two other walks in `sym/` have the
same shape. They build no text, so what they cost is time, not memory.

- `crates/geom-core/src/sym/signed.rs`, `enclose_indet` /
  `enclose_deep` / `enclose_form_deep` (the decision read's deep
  enclosure): recurses through every atom's argument to
  `ENCLOSE_DEPTH = 8` with no memo, so an atom reached at several
  sites is enclosed at each. Its cost is bounded by the product of the
  fan-out over eight levels, not by the number of distinct atoms.
- `crates/geom-core/src/sym/manifest.rs`, `positive_indet` /
  `positive_mono` / `positive_poly` / `positive_at` (rule F's
  positivity test): the same recursion to `ATOM_DEPTH = 4`. The
  constant's doc says so in part ("there is no memo, so an atom
  appearing twice is tested twice") and then calls the predicate
  "linear in the sub-tree it reaches", which holds of the unrolled
  tree and not of the DAG. Its `all` and `any` short-circuit, so the
  product is the worst case and not every call's cost.

**Not measured:** neither walk's cost on a real document. The tower
that reproduces the renderer's growth
(`crates/geom-core/tests/shape_report_render_budget.rs`, `tower`) is
the fixture that would measure them: its atoms are `sqrt` over sums of
`sqrt`, which both walks descend. `work/decide/decision-read-triples-the-plate-pin-suites-wall-time.md`
(closed) measured the deep enclosure's cost per node on the plate and
did not attribute it to re-walked atoms.

**What would settle it:** a memo per (atom, remaining depth) inside
one call, as the renderer's measuring pass keeps, or a count of the
atoms entered per call set against the distinct atoms reached.
(BOXFINISH report-bound lane)
