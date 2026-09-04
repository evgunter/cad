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
