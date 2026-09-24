---
id: the-half-edge-to-face-walk-is-spelled-per-test-file
kind: issue
title: The half-edge to face walk is spelled once per test file across 91 sites in five crates' suites
status: open
opened: 2026-09-19
priority: P4
cost: E
---


## Finding

- **Where**: `crates/sweep/tests` (72 sites), `crates/topo/tests` (8),
  `crates/mesh/tests` (5), `crates/editor-core/tests` (3),
  `crates/step-export/tests` and `crates/step-import/tests` (1 each).
- **Importance**: medium
- **Confidence**: sure for the buckets; the per-file counts are a
  type-directed probe's, and its blind spots are below.
- **Raised by**: PR #2857's fix pass, 2026-09-19, announced by seam from
  `work/dup/half-edge-to-face-walk-is-spelled-once-per-suite.md`.

One walk — half-edge → its `parent_loop` → that loop's `.face`, and in
`sweep/tests` often one more hop to `.surface`. PR #2857 folded
`topo/src` onto `pncad::topo::Body::face_of_half_edge`, which is `pub`
and total. The suites still spell it out, once per file, almost always
under `unwrap`/`expect` where a stale key is a test failure and no
refusal is lost by folding.

`sweep/tests` is the bulk and carries the `.surface` hop at 24 of 41
regex-visible sites; `topo::test_support_fixtures::face_surface_of_he`
is the composition (door + `get_face(f)?.surface`) that already serves
it.

**`topo/tests` is 8, and a census said 1.** The undercount was the
instrument, not the tree: `trim_3_chart_bound.rs` (~:582) is behind
`#![cfg(feature = "interval")]`, and feature-gated code never
type-checks, so it never warns. The eight are
`issue86_double_subtract.rs` (~:110), `readback_sense_kind.rs` (~:254),
`review_m9_1_r2_probes.rs` (~:234), `m3_pr2_reduce.rs` (~:338),
`m3_pr3_split.rs` (~:385), `verbs_f7_collinear_seam.rs` (~:84),
`bool4r1_probes.rs` (~:198) and `trim_3_chart_bound.rs` (~:582).

This row is `tint`'s rather than `tcost`'s per `work/dup/program.md`'s
`keep_out`: both claim `crates/*/tests/*`, and a row that is both a
duplication row and a test-integrity row goes to S-TINT first.

## What the instruments could not see

- **The probe** (`#[deprecated]` on `HalfEdge::parent_loop` and
  `Loop::face`, warning spans paired within ±3 lines over
  `cargo check --workspace --all-targets`, deduped on `(file, line)`)
  cannot see feature-gated code, and `--workspace` is not every cargo
  root — `scripts/doc-gate.sh --print-roots` names seven outside it.
- It counts field WRITES and 2-hop reads alongside 3-hop read walks, so
  the bucket totals above are an upper bound on the foldable set, not a
  fold count.

## One more instance, and it was written TWICE in one test (ATREST-2, 2026-09-21)

`crates/sweep/tests/m5_s10_face_sense.rs`, in
`every_sense_reading_gate_shuts_on_the_arc_loft` as landed by ATREST-2.
The row's third gate needed "does this half-edge's face carry a spline
chart", spelled it as an `is_spline` closure doing the full
half-edge → face → surface walk with `face_of_half_edge` — and the
SAME face → surface test was already written out inline twelve lines
above, in the row's second gate, over a face key. Two spellings of one
predicate inside one `#[test]` body, neither reaching for the other.

Folded by this unit into a local `is_spline_chart_face(body, face)`
that both gates call; the half-edge gate reaches it through
`Body::face_of_half_edge` (the `topo/src` door this class's parent row
names), so the walk itself is no longer restated.

The instance is small and the point it carries is not: the duplicate
was minted inside a single function, by one author, in one sitting. A
census that reasons about this class as a per-FILE or per-SUITE
phenomenon will not predict that, and the fold that prevents it is a
named local predicate rather than a shared home.
