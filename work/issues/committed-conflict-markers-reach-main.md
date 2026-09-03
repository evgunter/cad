---
id: committed-conflict-markers-reach-main
kind: issue
title: Committed conflict markers keep reaching main — three instances in two days; CI owes a tree-wide marker/delimiter guard
status: open
opened: 2026-08-30
github: 1287
refs: [1224]
---

## From GitHub issue 1287

Opened 2026-08-30; 0 comments.

Class finding, filed for a durable home (S-QA's track-J territory: a gate that would have caught all three).

**The class:** high-traffic append-heavy files resolved under pipeline pressure land on main carrying unresolved conflict artifacts, and nothing in CI looks:

1. `docs/KERNEL-VERBS.md` — literal `<<<<<<<`/`=======`/`>>>>>>>` block committed by the SHELLFIX 2b merge train, repaired at `efaf6b97`.
2. `crates/pncad-py/src/py/mod.rs` — a doc-string-heavy `pyo3::create_exception!` call lost its closing delimiter in a union merge, so **main did not compile**; sibling of main's own `dfd921ef` repair; fixed in [PR #1224](https://github.com/evgunter/cad/pull/1224)'s pass.
3. `docs/MODEL-AB-LOG.md` — a ~280-line committed conflict block (containing, among others, the corrected-vs-stale M10-P sample renumber), flagged by a BLEND-7 lane, repaired keep-both-dedup in its own PR.

The marker half is a one-line gate: `git grep -nE '^(<{7}|={7}|>{7})( |$)'` over the tree, red on any hit (the SMELL logs quote markers in prose mid-line, so anchor to line starts and the trailing space/EOL). It would have caught 1 and 3 at the PR. Instance 2's delimiter-loss shape is not grep-able the same way, but the compile itself catches it *when the tier compiles that crate* — which is the adjacent known gap (a docs-classified push skips the code tier; instance 2 shipped exactly that way).

Not S-BLEND's fence; filed for track J / S-QA scheduling.

## Home

`work/issues/` — the gate it asks for is track J / S-QA ground (`.github/workflows/*`, `scripts/*`), and S-QA is closed with track J empty.
