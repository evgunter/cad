---
id: a-pin-or-disabled-red-can-cite-a-closed-or-missing-row
kind: issue
title: a pinned or disabled red can cite a row that is closed or does not exist, and nothing reds
status: closed
opened: 2026-10-29
closed: 2026-10-30
priority: P3
cost: E
---


## What

GREEN's rule pins or disables a red and cites the row that owns its
fix: `#[ignore = "disabled red: work/<program>/<row>"]`,
`#[ignore = "pinned red: …"]`, `test_utils::pinned_red::row` call sites
with their row comment, and nightly.yml filters that cite a row. Nothing
checks that the cited path exists and is open, or that a closed row
leaves no pin behind.

Instance: #6027's first head routed 24,045 k-lint samples to
`work/cleave/lily-walls-curved-clearance-crowds-the-band-under-k-lint.md`,
closed on 2026-10-07 (caught in review, 2026-10-29).

There are two spellings, `pinned red:` and `disabled red:`, and nothing
reads either.

## Owed

A check (a `scripts/gates/` gate, or `work.py lint` reading the tree)
that every `pinned red:` / `disabled red:` citation names an open
`work/` row. Rows that close must therefore take their pins with them.

## Closed (2026-10-30)

`scripts/gates/pin-cites-an-open-row.sh` reads every pin citation in
`*.rs` under `crates/`, `demos/`, `tools/` and
`interval-transcendentals/`, and in `.github/workflows/*.yml`: a
`work/<program>/<id>` path in a comment block or line that says
`pinned red` or `disabled red` (in a workflow also `held out`), or in a
comment between a `pinned_red::row(` / `kernel_limit::pinned*(` call
and its enclosing `fn`, or in that fn's doc and attribute block. Each
must name a row that exists and is not closed; a row that moved is
reported at its new path. It runs plain on every PR through
`scripts/run-gates.sh` and `--selftest` nightly, in about 0.3 s.

On main it found 72 citations and flagged two, both in a pinned test's
doc: `sweep/tests/reach_split_gate_azimuth.rs` cited
`work/quad/quadrature-convergence-test-escalates-instead-of-refining`
(closed by PR 4839; the stand-down it describes is now
`work/quadtail/the-escalation-tolerating-rows-can-assert-their-answers-now`),
and `sweep/tests/encl_curved_loft_shell.rs` cited
`work/quad/a-fitted-cap-cut-by-planes-has-a-sub-range-trim-image`, moved
to `work/quadtail/`. Both re-pointed.

What the gate cannot see is in its header: a citation in a helper the
pin reaches through an argument (every such site on main cites an open
row), a `/* */` block, and a pin that cites nothing. The wider class,
`work/` citations outside any pin that name a closed, moved or missing
row, is `work/meta/code-citations-of-work-rows-go-stale-when-rows-close-or-move`.
