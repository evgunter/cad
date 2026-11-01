---
id: prose-census-reads-a-match-guard-as-the-arms-pattern
kind: issue
title: the prose census reads a guarded match arm's guard as its pattern, so a guarded Display arm's named placeholders are undecided
status: open
opened: 2026-10-27
priority: P3
cost: E
---


Found fixing `work/cleave/main-red-prose-census-cannot-read-a-guarded-edge-in-face-arm.md`
(branch `cleave/edge-in-face-one-arm`).

## What

`crates/pncad-py/src/prose_census.rs` `arm_pattern` scans back from an
arm's `=>` and stops at the first depth-zero closing delimiter once it
has seen anything. On a guarded arm the guard comes first, so the scan
crosses the guard (and any `(...)` inside it) and stops at the
pattern's own closing `}` or `)`: the "pattern" it returns is
`if <guard>`, which binds nothing. Every named placeholder in that arm
then answers Undecided.

It fails loud, not silent: the found set gains entries and
`every_site_this_census_cannot_decide_is_named_with_its_reason` goes
red. But that test is in the slow set, which the per-PR gate does not
run, so a guarded `Display` arm with a `{name:?}` placeholder passes
its PR and turns `main` red at the next slow-set run. PR 5546 did
exactly that with `BooleanError::EdgeInFace`
(`crates/topo/src/boolean/mod.rs`), and the fix was to rewrite the
arm unguarded rather than teach the census.

Guarded arms that write output today (none trips the census at
2026-10-27; each is a place the next placeholder edit would):
`crates/topo/src/chart_region.rs` (`Escalated`),
`crates/topo/src/boolean/mod.rs` (`ContactContradicted`),
`crates/viewer/src/camera.rs` (`Unfittable`),
`crates/editor-core/src/pose.rs`, `crates/editor-core/src/names/emit.rs`
(`EntityNamed`), `crates/geom-core/src/predicate.rs` (`Placement::Undecided`, two),
`crates/geom-core/src/spline/knots.rs`, `crates/geom-core/src/tolerance.rs`.

## What is owed

`arm_pattern` strips a trailing top-level `if <guard>` (an `if` keyword
at depth zero after the pattern's closing delimiter) and returns the
pattern before it, with a fixture test of a guarded arm whose binding
is typed. `if let` guards (`crates/editor-core/src/node.rs`,
`crates/editor-core/src/persist/check.rs`) bind names of their own and
want the same thought.
