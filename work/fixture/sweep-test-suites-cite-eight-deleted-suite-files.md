---
id: sweep-test-suites-cite-eight-deleted-suite-files
kind: issue
title: sweep tests: sixteen tests→tests citations across thirteen suites, eight naming suite files that no longer exist
status: open
opened: 2026-09-08
priority: P4
cost: E
---


## Finding (BLEND unit 5's style review and fix pass, PR 2155; filed by the BLEND orchestrator)

`crates/sweep/tests/**` carries sixteen `tests/<file>.rs` citations of
OTHER suites across thirteen suite files (e.g. `cert_m2r1_passes.rs:218`,
`common/oracles.rs:21`), eight of which name suite files that no
longer exist. BLEND-5 normalised `crates/sweep/src`'s citations to the
`<module>::<row>` spelling and added a resolver row whose corpus is
`crates/sweep/src` only; the tests→tests class is outside that unit's
fence and this glob's. The fix shape is the same: the module spelling,
and the resolver row's corpus widened to `crates/sweep/tests/**`
(`review_blend5_r5_probes.rs`'s reader, one path added).

## Home

`work/tcost/` — `crates/sweep/tests/*` is S-TCOST's glob.

## Moved to S-TINT (2026-09-11)

Moved by `git mv` from `work/tcost/` at S-TINT's opening. Id, title and
body are unchanged; the directory is the claim (`work/README.md`).

**Why it moved.** S-TCOST's board was re-sorted on 2026-09-11 against the
repository going public on 2026-09-03 (`work/tcost/log.md`, the
2026-09-11 seam). That sort found this row is not a cost lever in either
currency — it neither shortens the gate's critical path nor saves a
billed minute, and it was never argued on one. It reached S-TCOST by the
tracker-wide re-home of 2026-09-04, which routed rows by PATH GLOB
(`crates/*/tests/*`, `crates/test-utils/*`) rather than by question.
This program is the question it was always about: whether the suite
asserts what it claims to assert.

## Re-derived (2026-09-15, lane D)

**VERDICT: PARTIAL** — the dangling half is gone (0 of the class dangles
today, down from 8); the resolver half is untouched and live.

**Command.** The whole `tests/…→tests/` class, re-derived by name:

```
grep -rnoE '[A-Za-z0-9_./-]*tests/[A-Za-z0-9_./-]+\.rs' crates/sweep/tests/
```

**25 citations across 19 citing files.** Dropping the 11 that name a
module mount rather than a suite (`tests/all.rs` x6 from
`fillet_h6_cap_rim.rs` x2, `k_report.rs`, `review_blend5_r5_probes.rs`,
`revolve_common/mod.rs`, `verbs_shell_r2b.rs`; and the five `…/mod.rs`
targets) leaves **14 suite-naming citations across 13 citing files** —
the same shape the row counted as "sixteen across thirteen suites".

**Every one of the 14 resolves.** Targets checked by name for existence:
`editor-core/tests/{blend5_rim_support,lib_g16_chamfer_node,m5_pr12_fillet_node}.rs`,
`editor-core/tests/corpus/loft_prism.rs` (x2), `mesh/tests/m5_s11_concave_sense.rs`,
`step-export/tests/m5_s11_same_sense.rs`,
`geom/tests/curves/m8_14_long_turn_meter.rs`,
`geom-core/tests/knot_queries_differential.rs`, `topo/tests/rim_of.rs`,
`demos/tour/tests/verbs_teapot.rs`, `crates/sweep/tests/m9_3_zip.rs`,
`crates/sweep/tests/shell7_seam_corner.rs`. The one apparent miss,
`m5_pr6_pcurves.rs`'s `tests/pcurve_parameter_finding.rs`, is not a miss:
that header's own sentence says the rows "live in `geom-brep`", and
`crates/geom-brep/tests/pcurve_parameter_finding.rs` exists.

**Wider census, because a `tests/`-prefixed pattern is not the only
spelling.** Every `*.rs` basename mentioned anywhere under
`crates/sweep/tests/**` — **344 distinct names** — checked against the
whole tree: exactly **one** resolves nowhere,
`zz_mb_probes.rs`, and it is not a suite citation but a history note
(`m5_s13_review_probes.rs`, *"History notes (merge-base evidence,
reviewer's `zz_mb_probes.rs`)"*) naming a reviewer's expired merge-base
probe file.

**Blind spots.** (a) The row never enumerated its eight names, so "did
any come back" cannot be answered by name — only that nothing in the
class dangles now. (b) A citation that names a `#[test]` row that no
longer exists inside a suite file that does still exist would not show
up here; settling that for the `tests/` corpus is exactly the resolver
work below. (c) The shallow clone rules out `git log -S` on the eight.

**The fix shape the row asks for is NOT done.**
`review_blend5_r5_probes.rs`'s resolver row,
`every_test_citation_in_the_sweep_docs_resolves_to_a_test_row`, still
declares *"The corpus is `crates/sweep/src/**` citing
`crates/sweep/tests/**`"* and builds its corpus from
`crate_dir(env!("CARGO_MANIFEST_DIR")).join("src")` in `sweep_prose()`.
`crates/sweep/tests/**`'s own 14 citations remain unguarded, which is
why they could rot again. **Recommend: keep open, rewrite the body** —
the census is stale (0, not 8) and the live ask is the one-path corpus
widening.
