---
id: tools-assert-sweep-blind-spots-unscheduled
kind: issue
title: Two blind spots in the tools-wide cannot-fail assert sweep, and nothing schedules them
status: open
opened: 2026-09-16
priority: P3
cost: D
---



## Finding

INSTR unit 4 re-swept every `assert!` / `assert_eq!` / `assert_ne!` /
`debug_assert*` in `tools/*/src` and `tools/*/tests` for the class
*a predicate no bug could break*, in three scripted passes: operands
that are `const` items; operands that are locals derived from consts
alone, by a fixpoint over `let` bindings in the enclosing `fn`; and
predicates forced by how their own operands were built.

**Two blind spots remain open, and nothing schedules a pass that
closes them.**

1. **A closure in the operands drops the assert from pass 2.** The
   fixpoint reads a closure parameter as a lowercase local it cannot
   resolve, so any assert containing one is discarded. That is why
   `tools/tess-lint/src/lib.rs`'s
   `SIZED_CHART_TAGS.iter().all(|t| CHART_TAGS.contains(t))` surfaced
   only under the third pass, and it would have been missed entirely
   had the third pass not existed. Closures are the common spelling
   for a predicate over a const table, so this is where the class
   lives, not an edge.
2. **An assert behind a shadowing `let`, or built by a macro, is
   matched by none of the three passes.** A macro-built assert is the
   worst case: the operands are not in the source text the sweep
   reads at all, so the pattern cannot even record a miss.

Unit 4's third stated blind spot — that the pattern the original row
claimed could not match its own instance — is discharged, in the body
of `work/instr/baseline-census-partition-assert-cannot-fail.md`.

## What would discharge this

A pass that resolves closure parameters (binding them from the
receiver rather than treating them as free), plus a scan for
assert-generating macros under `tools/`, run at the current tree. Or a
written reason the two are acceptable to leave open, which is the
other half of Q6's remedy.

## Was

Disclosed by INSTR unit 4's style review (`instr/u4-census-partition-assert`).
