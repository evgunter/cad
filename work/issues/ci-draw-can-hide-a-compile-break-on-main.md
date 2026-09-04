---
id: ci-draw-can-hide-a-compile-break-on-main
kind: issue
title: The ci.yml filter draw can hide a hard compile break on main for an unbounded number of merges
status: open
opened: 2026-09-04
refs: [blamed-mates-lost-its-exhaustive-arm]
---

Split out of `work/view/blamed-mates-lost-its-exhaustive-arm.md`,
whose code half is closed and whose process half is not this program's.
Filed here rather than in `work/ciw/` because a VIEW branch may not
edit CIW's slate (`docs/prompts/implementer-discipline.md` §6);
**CIW is the natural owner** and the claim of this file is that the
row belongs on its board.

## What happened once, and can happen again

`crates/editor-core/src/mate.rs` grew `MateFault::Unleverable` in
`77f504727`. `crates/viewer/src/tree.rs`'s `blamed_mates` matches
`MateFault` exhaustively **on purpose** — its doc comment says a fault
arm the kernel grows must decide there whether it names a mate rather
than falling into a wildcard. The new variant got no arm, so the
design worked exactly as intended and refused to compile.

**The commit landed anyway, and `main` reported green.** `ci.yml`'s
`filter` job draws ONE point of {lane} x {eps} x {k-lint row} per run,
and `main` push runs deliberately carry only what is unique to them.
The draw that would have built `viewer` at `--workspace --features
interval` did not come up on the runs between `77f504727` and the
discovery, so a red tree reported green. It was found by a PR whose
merge ref happened to draw the interval lane (FILLET-E3, PR 1763, run
33840944595, job `build + archive (interval)`), and it cost two agents
duplicate work on one line in the same hour because neither could see
that `main` was red.

## The claim

**A compile of every crate is not a "configuration" in the sense the
sampling argument means** — it is the precondition for any of the
sampled rows to mean anything. Nothing in the sampling note accounts
for a draw that can hide a hard build failure, as opposed to a
behavioural difference between lanes.

Whether the build tier should be exempt from the draw is CIW's
question and this issue does not answer it. The two shapes worth
weighing are named here only so the row is not re-derived:

- exempt the **build** legs from the draw and keep sampling the
  behavioural matrix, which costs one compile per lane per run;
- or leave the draw alone and add a scheduled register that compiles
  every lane on `main` on a cadence, which is cheaper and detects the
  break later.

The 2026-09-04 widening of the code-tier run to twelve `test (…)` jobs
(`docs/prompts/implementer-discipline.md` §2) changes the numbers this
issue was filed against and may already close it for PR runs. **It does
not obviously close it for `main` push runs**, which are the ones that
carried the red tree, and checking that is the first thing this row
owes.

## Note for whoever takes it

`main` compiles at this lane today: the arm is present at
`crates/viewer/src/tree.rs:325`
(`| MateFault::Unleverable { mate, .. } => vec![*mate]`). This issue is
about the hole, not about the instance, and the instance is closed.

Signed: (VIEW orchestrator)

## Measured (VIEW orchestrator, 2026-09-04) — the row's own first question, answered

This file said the 2026-09-04 twelve-job widening *"may already close it
for PR runs. It does not obviously close it for `main` push runs …
and checking that is the first thing this row owes."*

**Checked. It does not close it, and the hole is wider than the
sampling argument this file was written against.**

A `main` push run does not draw ONE point of the matrix. It draws
**none**. Measured on the `#1829` merge push, run `33905366880` and its
sibling `33905368338`:

| job | conclusion |
|---|---|
| `test (eps = …, …/2)` | **skipped** |
| `test (interval, eps = …, …/2)` | **skipped** |
| `clippy`, `clippy (--all-features)`, `clippy + doc-tests (interval)` | **skipped** |
| `build + archive (default)` / `(interval)` | **skipped** |
| `k-lint (gate)`, `discipline`, `python suite`, `step import` | **skipped** |
| `rustfmt + rustdoc (gate) + wasm32` | **skipped** |
| `CI half parity + gate wiring`, `change filter`, cache primes, render lanes | ran |

The same shape on runs `33905855591` and `33907430790`. So the workflow
that reports on a `main` push runs the mirror check, the filter, the
cache primes and the render lanes — **and no test, no clippy, no build,
no k-lint.**

### What that means for this row

The finding this file was opened on — a compile break sitting green on
`main` — was attributed to a *sampling* draw that happened not to pick
the interval lane. That is too generous. On a `main` push the test tier
is not sampled at all, so **a red `main` is invisible by construction
there**, and the only thing that can catch it is a PR run whose merge
ref happens to include the breakage. Which is exactly how both
instances were found.

### The second instance, same day

`work/issues/reader-census-red-on-main-docm1-hand-rolled-doc-reader.md`
is the same hole firing again, eight hours later:
`test-utils::reader_census` reds on all six `1/2` shards, `main` has
carried it since 18:08 UTC, and every `main` push since has reported
**success**. That file speculates a draft-PR path as the mechanism;
**that speculation is superseded by this measurement** — no draft is
needed, because a `main` push runs no test job at all.

Two instances in one day, found by two unrelated PRs, is the argument
for this row being worth a schedule rather than a note.

### What this does NOT settle

Whether the fix is to run the tier on `main` pushes, to add a scheduled
register that compiles and tests every lane on a cadence, or to accept
it and rely on PR runs with a stated rule for who notices — that is
CIW's call and this file still does not make it. What has changed is
that the row now rests on a measurement rather than on an inference
about a draw.

Signed: (VIEW orchestrator)
