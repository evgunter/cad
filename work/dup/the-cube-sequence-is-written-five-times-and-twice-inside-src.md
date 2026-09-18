---
id: the-cube-sequence-is-written-five-times-and-twice-inside-src
kind: issue
title: The §9.4.2 cube sequence is written five times in topo; two of the copies are in src/ and fold onto each other
status: parked
opened: 2026-09-18
blocked_on: [brick-has-two-constructions-and-two-homes]
---


## Finding

- **Where**: `crates/topo/src/fixtures.rs` (`ops_cube`, `:765`),
  `crates/topo/src/review_m1_pr3.rs` (`build_box`),
  `crates/topo/src/cert_m3r1_probes.rs` (its verbatim copy of
  `geometric_cube`), `crates/topo/tests/common/mod.rs`
  (`geometric_cube`/`cube_into`, and `prism_ops` which subsumes them),
  against each other.
- **Importance**: medium
- **Confidence**: sure for the `ops_cube`/`build_box`/`geometric_cube`
  triple, which was measured by dumping; the fifth copy
  (`cert_m3r1_probes.rs`) is carried from an earlier row and was NOT
  dumped here
- **Raised by**: the S-DUP orchestrator, 2026-09-18, out of the link-3
  home measurement on
  `work/dup/brick-has-two-constructions-and-two-homes.md` — the lane
  measured it while settling whether `review_m1_pr3::build_box` was a
  member of the family, and disclosed it in that row's prose. Prose is
  not scheduling (`work/README.md`), so it gets a file.

## The measurement

**`fixtures::ops_cube` is `geometric_cube` with the face geometry
declined.** The two derived `Body` dumps are byte-identical in
`points`, **`curves` (all 1199 lines)**, `half_edges`, `loops`,
`edges`, `vertices`, all seven provenance maps, `curve_origins` and
`surgery`. They differ in exactly three sections — `faces` (which
surface key each face carries), `surfaces` (one shared NURBS
placeholder against six Newell planes) and `surface_origins`. Reading
confirms the dumps: operator for operator, including the
`f_bottom.he_plus` strut anchor and the `f_front.he_plus` close.

**`review_m1_pr3::build_box` is `ops_cube` at a uniform 2× scale.**
Every section of both dumps is identical except `points` (`1.0` →
`2.0`) and the `curves` that carry those coordinates.

So the §9.4.2 sequence stands written in `topo` at least five times:
`prism_ops` (the shared builder link 1b landed), `cube_ops` (folded
into it by link 1b), `cert_m3r1_probes.rs`'s copy, `fixtures::ops_cube`
and `review_m1_pr3::build_box`. **Two of those five are in `src/`, are
the same function as each other, and are the same function as the
`tests/` builder up to one axis (face geometry) and one scale.**

`crates/topo/src/review_m1_pr2/cube_independent.rs` is a sixth
spelling and is **exempt**, not a defect: its header states *"independent
derivations — do not 'simplify' them to match shipped fixtures …
Promoted per Ev's request (PR #17 thread)"*, and it genuinely differs in
addressing (seeds at the top, closes the TOP face first, struts
downward, the seed face ends as the bottom). Anyone working this row
reads that header before touching it. `review_m1_pr2/atomicity.rs`
builds a digon pillow and is not a member either — the census flagged it
on `mvfs`/`mev_line` call sites, which is the predicted
undercount-by-shape running in the other direction.

## What this row is NOT

It is **not** "delete four of the five". Two of the three measured
copies decline something on purpose, and what they decline is the point
of the suite they serve:

- `ops_cube` declines the face surfaces because its consumers are
  operator-count and atomicity tests that must not depend on geometry.
- `build_box`'s 2× scale may be load-bearing for whatever
  `review_m1_pr3` asserts, or may be arbitrary — **unmeasured**.

The honest shape is a builder with the declined axes as parameters,
which is what `prism_ops` already is. Whether the `src/` copies can
reach it is a **consequence of link 3**, not independent of it: today
nothing in `src/` can name `tests/common`, which is the same namability
wall that holds `cert_m3r1_probes.rs`'s copy in place.

## Sequencing

**Parked behind link 3.** With the family in `src/` at
`topo::test_support`, all three in-`src` copies become foldable in one
unit; before that they are three separate arguments with a wall in
front of each. Do not dispatch this before
`brick-has-two-constructions-and-two-homes` closes.

## What is unmeasured

- `cert_m3r1_probes.rs`'s copy was not dumped against the other four;
  its membership is carried from
  `work/dup/topo-src-cert-m3r1-probes-holds-an-in-src-copy-of-the-cube-family.md`,
  which established it by reading.
- Whether `build_box`'s 2× scale is read by any assertion in
  `review_m1_pr3`.
- `f64` only, default features, one eps row.
- No consumer of `ops_cube` was checked for a dependency on the
  placeholder surfaces *being* placeholders (as opposed to merely not
  being read).
