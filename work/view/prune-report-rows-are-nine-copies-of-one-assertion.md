---
id: prune-report-rows-are-nine-copies-of-one-assertion
kind: issue
title: Nine near-verbatim assertion blocks across seven test files, because the shared harness is other programs' ground
status: open
opened: 2026-09-05
refs: [session-shims-and-test-imports, 1886]
---


Found by #1886's style review. The lane's call was defensible and the
tension it exposes is the filable thing.

## What happened

#1886 retyped `OpOutcome::superseded` from `Vec<RecipeNodeId>` to a
report carrying each withdrawal's cause. Nine assertion sites moved,
and each one-line `assert_eq!(outcome.superseded, vec![x])` became a
~15-line block. The blocks are near-verbatim across
`assembly_display.rs:610`, `assembly_walk.rs:212`,
`frame_policy.rs:963`, `instance_authoring.rs:178`,
`review_gui4_r1.rs:446`, `review_gui4_r2.rs:507` and
`story_assembly.rs:491,644,717` — the message string *"and the outcome
carries WHY it went, not only which went: {}"* appears **eight
times**.

## Why the lane did that, and why it was right to

The natural home for the helper is `crates/viewer/tests/common/asm.rs`.
That is a **test mechanism**, and `crates/viewer/tests/*` is declared
territory of S-TCOST and Track W as well as VIEW — VIEW's `keep_out`
says test-mechanism changes are announced, not assumed, and the
announce has not been made. The lane's brief told it so, it kept
`asm.rs` untouched, and it said so. That is the fence working.

**So this is not a finding against the unit.** It is the record that
the fence has a price, that the price was nine copies this time, and
that nobody has decided whether to pay it again.

## What resolving it looks like

One shared assertion helper for *the outcome carries a cause*, in
`crates/viewer/tests/common/`. It needs the announce VIEW's `keep_out`
requires — to S-TCOST and Track W — before it lands, not after. That
announce is the orchestrator's and is owed regardless of whether this
item is taken, because the next retyping of an `OpOutcome` field will
meet the same wall.

The alternative is to decide that nine copies is the correct price of
not touching shared ground, and to say so where the copies are. What is
not acceptable is the current state, where the reason lives only in a
merged PR body.

## Home

VIEW's, with the announce owed to S-TCOST and Track W:
`crates/viewer/tests/common/`.
