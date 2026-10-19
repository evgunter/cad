---
id: predicate-dimension-audit-doc-forces-the-whole-suite
kind: issue
title: an edit to docs/predicate-dimension-audit.md forces TIER=all (22 of 257 merges in a week)
status: closed
closed: 2026-10-16
opened: 2026-10-16
priority: P3
cost: E
---


## Measured (2026-10-16, CIW orchestrator)

In the same 7-day replay as `a-slow-set-edit-to-nextest-toml-runs-the-whole-suite`, 22 merges reached TIER=all only because they edited `docs/predicate-dimension-audit.md`. A single `docs/guide/north-star-audit.md` edit did the same.

## Wanted

Find what reads the audit doc: ci-filter's markdown-read-by-tests logic classifies it workspace-level. Scope it to the crates or suites that actually read it. If one census reads it, the doc's edits seed that census's crate.

## Closed

One suite reads the page: `crates/geom-core/tests/flagged_census.rs` opens it
at run time. The `.rs` files in topo, editor-core, geom-brep and geom-core that
also name it do so only in comments. ci-filter found that read through the read reach
(`_suite_read_markdown`). That took the page out of the docs tier, and then
`classify` bailed on it as a path outside every member. Now each half of
`_consumed_markdown` keeps its readers (`_markdown_readers`), and `classify`
seeds the members that read a consumed page. The page goes unscopable only
when a reader sits outside every member: `docs/TESS-BUDGET.md` and
`tools/README.md`, which `tools/` reads, stay TIER=all. The rule is derived and
applies to every consumed page. `docs/predicate-dimension-audit.md` now seeds
geom-core. That is TIER=closure, but geom-core's dependent closure is 16 of
the 18 members. `docs/guide/north-star-audit.md` and the other guide pages seed
pncad and pncad-py: the `include_str!` in pncad, and the python tests, which
also switches the python suite on.

Replay, the same runs as `a-slow-set-edit-to-nextest-toml-runs-the-whole-suite`:
in the 268-merge window, 21 of the 22 audit-doc merges and the 1 north-star
merge moved from all to closure. The 22nd also touched
`scripts/payload-rung-sweep.py`. Over 805 merges, 85 nextest.toml, 40 audit-doc, 4 `docs/GUIDE.md`,
1 north-star and 1 `docs/guide/assembly.md` moved from all to closure.
Those counts are by first unscopable path.
