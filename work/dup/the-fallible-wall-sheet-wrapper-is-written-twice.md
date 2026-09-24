---
id: the-fallible-wall-sheet-wrapper-is-written-twice
kind: issue
title: try_wall_sheet is token-identical in two review-probe suites and wraps the now-shared door
status: closed
opened: 2026-09-19
priority: P4
cost: E
closed: 2026-09-20
branch: dup/src-cyl-sheet
refs: [try-wall-sheet-stands-down-on-any-panic]
pr: 2925
---


## Finding

`crates/topo/tests/r1_mate5_probe.rs` and
`crates/topo/tests/r2_probes.rs` each carry a `try_wall_sheet`:
a `std::panic::catch_unwind` around the cylinder-wall sheet builder
that turns a fixture the current ε cannot MINT into a `None`, so the
row stands down instead of reporting. The two are **token-identical**
after comment and whitespace normalisation (one md5), measured
2026-09-19 at merge base `5b4979ef2`; only their doc comments differ,
and those differ in substance — `r2_probes`' cites the tilt map its
own suite carries.

Both now wrap one door
(`topo::test_support::cyl_wall_sheet`, PR for
`the-cylindrical-patch-rim-builder-is-written-nine-times`), which is
what makes the remaining wrapper a duplication rather than two
builders that happen to share a shape. **This row exists because that
unit minted it** — method item 5, X4.

## Why it was not folded with the builder

The obvious home is `topo::test_support` beside the door. Two reasons
to decide that deliberately rather than by reflex:

- Nothing in `test_support_fixtures.rs` swallows a panic. A fixture
  door that returns `None` where its sibling doors panic is a posture
  change for the module, not a move.
- The stand-down is a per-suite JUDGEMENT — *this row's constants won
  the mint lottery at the default ε and would not be evidence at
  another* — and a shared wrapper states it once for suites that may
  not mean the same thing by it.

Neither reason survives measurement on its own; a unit here should
take both and decide, not assume.

## What a unit here owes

Re-take the count (two is the count at this merge base; the wrapper is
easy to copy again), then either give it the shared home with the
posture written down, or state why two suites keep it and delete the
duplicate doc sentence that is not true of both.

## Closed

Folded into `crates/topo/tests/probe_support/mod.rs`, a helper module
of the aggregated test binary, declared once in `tests/all.rs` beside
`fixture`. Both suites now `use crate::probe_support::try_wall_sheet;`.

**Two claims in the Finding above were stale at this merge base
(`cd9fdfd6b`), both because PR #2887 landed between them and now.**

- *"token-identical after comment and whitespace normalisation (one
  md5)"* — **no longer true.** `r1`'s wrapped `cyl_wall_sheet(body,
  frame, Some(src_id), (u0, u1), (v0, v1), Tol::witness())` directly;
  `r2`'s wrapped its own flat-argument `wall_sheet` adapter, which that
  PR kept and `r1`'s did not. The two signatures stayed identical and
  the two bodies diverged by that one call.
- *"only their doc comments differ, and those differ in substance —
  `r2_probes`' cites the tilt map its own suite carries"* — **exactly
  backwards.** The two doc comments were **byte-identical**, fifteen
  lines each, and the citation is in BOTH: `r1_mate5_probe`'s copy
  cites `r2_diag_mintable_tilts`, a row that lives in `r2_probes.rs`.
  A doc sentence copied into a file where its referent is not is what
  was there, not a per-suite judgement.

**The home is the `tests/` side, and the row's first objection is why.**
`git grep catch_unwind` over every tracked file: outside
`geom-core/src/k_stats.rs`'s own `#[cfg(test)]` module, **every**
occurrence in the repo is under a `tests/` tree. A `None`-returning
fixture door in `topo::test_support` would be the first of its kind in
any crate's `src`, and the row's second objection — that the stand-down
is a per-suite judgement — is the reason it should stay where a suite
can see it. One helper module, one statement of the posture, beside
the flat `wall_sheet` adapter the stand-down wraps (the fix pass below
moved that adapter there too; `r2_probes`' ten call sites read it
flat).

**What the fold measured on its way past.** Planting a
`try_wall_sheet` that stands down unconditionally reds **zero** rows;
planting a broken builder under it reds 22 rows elsewhere and leaves
its own two green, standing down. That is a row that cannot go red, not
a duplication, and it is filed as
`work/tint/try-wall-sheet-stands-down-on-any-panic.md`.

## Fix pass, 2026-09-20 — the fold re-minted the adapter it folded

X4, caught by a reader. `probe_support`'s `try_wall_sheet` was written
wrapping `cyl_wall_sheet(body, frame, Some(src_id), (u0, u1), (v0, v1),
Tol::witness())` — **verbatim the body of `r2_probes.rs`'s
`wall_sheet`**, the same seven parameters in the same order. Before the
fold `try_wall_sheet` *called* `wall_sheet`; after it they were two
independent copies of one adapter inside one test binary, so a change
to either's tolerance or source convention would not have reached the
other.

`wall_sheet` is now `probe_support`'s too, `try_wall_sheet` wraps it,
and `r2_probes` imports both. Its **ten** call sites are unchanged —
the Finding's *"eleven"* counted the definition — and
`r1_mate5_probe`'s `sheet` is three lines over the same adapter instead
of a second copy of the door's argument list.

**The direct `cyl_wall_sheet(` sites the reviewer asked about are a
different shape and are filed, not folded.**
`r1_mate5_probe.rs`'s `sheet` and five in `mate5_cyl_eps_rung.rs`
return `(Body<f64>, FaceKey)` — they make the body rather than taking
one — and `mate5_cyl_eps_rung.rs` was outside this unit's fence. Row:
`work/dup/the-fresh-body-cylinder-sheet-wrapper-is-written-six-times`.
That file's `interval_lane` pair is in neither class: it builds
`Body<Interval>`, and both adapters are `f64`-only.
