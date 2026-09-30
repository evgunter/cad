---
id: run-with-a-prior-evaluation-has-seven-private-copies
kind: issue
title: run(doc, prior, ...) has seven private copies across editor-core's suites and no shared home
status: open
opened: 2026-09-15
priority: P4
cost: E
---


Disclosed by SUITE's `editor-core-suites-carry-eleven-part-resolver-stubs`
migration, which hoisted the no-prior `run(doc, &EvalOptions)` onto
`fixture::run` in twelve suites and left this shape standing.

`fixture::run(doc, o)` is exactly the `prior = None` specialization of a
`run` that takes a prior, so the fixture COULD hold both — one
`run_with_prior(doc, prior, o)` with `run` delegating. It does not,
because the prior-taking spelling is a **seven-file class** and six of
the seven are outside that unit entirely (no part store, no resolver, so
the unit's own sweeps never touched them). Hoisting it there would have
served one caller and left six, which is the half-fix
`docs/prompts/reviewer-style-lane.md` names.

The seven, in `crates/editor-core/tests/`, and how each differs:

| file | signature | options |
| --- | --- | --- |
| `docm4_evaluation_identity` | `(doc, prior, opts)` | caller's |
| `m4_pr2_eval` | `(doc, prior, parallel)` | `EvalOptions { parallel, .. }` |
| `review_m4_pr2` | `(doc, prior, parallel)` | identical to `m4_pr2_eval`'s |
| `m4_pr4_banked` | `(doc, prior)` | `EvalOptions::default()` |
| `m4_pr4_edits` | `(doc, prior)` | identical to `m4_pr4_banked`'s |
| `m4_pr4_diff` | `(doc, prior)` | `EvalOptions { boolean_sweep: Idealized, .. }` |
| `m4_pr4_resolve` | `(doc, prior)` | identical to `m4_pr4_diff`'s |

Three pairs are byte-identical (`m4_pr2_eval`/`review_m4_pr2`,
`m4_pr4_banked`/`m4_pr4_edits`, `m4_pr4_diff`/`m4_pr4_resolve`), and the
seventh is the general one all six specialize. So the shape is not
"seven suites each needing something different"; it is one function and
three option presets, written out seven times.

**What this class is invisible to.** It has no store, no resolver and no
`in_part`, so every sweep the stub-migration ran — `impl .*PartResolver
for`, a `Store`/`Resolver` struct-name grep, `resolver: Some(`,
`^fn in_part` — misses all seven. The pattern that finds it is
`^fn run\(doc: &ProfileDoc, prior:`, and nothing in the tree has ever run
it. That is the reason this is a file and not a sentence in a PR body:
the next lane to touch these suites will not re-derive the census by
accident.
