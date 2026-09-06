---
id: closure-reaches-tree-wide-guards
kind: unit
title: The change closure reaches tree-wide guards, derived from what a suite reads
status: review
opened: 2026-09-05
branch: ciw/closure-reaches-tree-wide-guards
refs: [f3-recosting-on-a-public-repo, 1829, 1859, 1871, 1884, 1889]
pr: 1909
---

Ev, 2026-09-05: *"oh yeah the closure should reach tree wide guards"*, and
*"don't hand the unit to tcost; you can take it"*.

`work/ciw/tree-wide-guards-outside-the-change-closure.md` (PR #1889) measures
the defect: `scripts/ci-filter.py`'s TIER=closure scope is the DEPENDENT
closure, so a crate is in scope only when it is touched or something it depends
on is. Several guards assert over the whole repository and live in crates
almost nothing depends on — `crates/test-utils/Cargo.toml:10` calls the leaf's
zero dependencies deliberate, which is right for the layering and puts the
tree's most tree-wide guard in scope for **1 of 18** members. It reddened
`main` twice on 2026-09-04, each time from a PR that was fully green because
the guard was never built in it.

## What this unit does

`PKGS` becomes the dependent closure **plus the read reach**: a member whose
own sources build a path outside their crate has a build edge its manifest does
not declare, and the filter now resolves those edges by reading them.

- lands at the repository root, or at `crates/` itself — the file reads every
  member, so its crate is pinned into every non-docs closure;
- lands inside another member — a read edge, keyed on that member's **SEEDS**
  (what it reads is the other crate's text, and only its own files move that);
- lands anywhere else tracked — already unscopable by the allowlist, except
  that a `.md` reached this way joins `_consumed_markdown` and leaves the docs
  tier;
- lands under `target/` — nothing tracked is there, so no diff can name it.

Nothing names a guard. The rule is derived from what a file reads, which is why
it found `crates/bvh/tests/aggregator_headers.rs` — a sixth tree-wide guard the
sweep in #1889 did not have, whose own header says *"its subject is
workspace-wide, so no crate owns it and any home is arbitrary"* — and
`crates/geom-core/tests/flagged_census.rs`'s read of
`docs/predicate-dimension-audit.md`, a seventh instance one tier over.

Derived on this tree, the reach is `bvh, editor-core, geom-core, pncad-py,
test-utils`, with read edges `pncad -> {editor-core, profile}`,
`step-import -> step-export`, `viewer -> editor-core` (all three already real
dependencies, so all three fire on nothing today).

## Two fail-closed arms and a floor

- an ascent the chain resolver cannot follow is measured **from the crate
  directory**, so an unreadable spelling lands at the root and pins rather than
  going missing;
- a reach that finds **no** tree-wide guard at all raises `Bail` — the tree has
  several, so an empty answer is a scanner that stopped reading, and the
  failure mode of a silently empty reach is this defect restored with no tell.

## What it does not change

`JOB_ROOTS` is keyed on the dependent closure with the reach subtracted again,
so pinning `editor-core` into every code-tier closure does not switch the four
named job rows permanently on. `SEEDS` is untouched, so the viewer-toolkit and
python-suite axes are unmoved. F3 and what a `main` push re-gates are untouched
— that is an `[ev]` ruling and this fix does not need it.

## Closed

The demonstration is in the PR: both real breaking diffs re-classified at their
own tree states with `test-utils` now in scope and the census red under it, ten
mutations of the new behaviour each shown to red a named selftest arm, and the
measured cost on both tiers.

### Residues, each its own row

- `work/ciw/reach-cannot-follow-every-ascent.md` — the class of ascent the
  chain resolver does not follow, and what the fail-closed sweep leaves.
