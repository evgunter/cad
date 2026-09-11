---
id: unify-discipline-machinery-onto-registry
kind: issue
title: checks/disciplines - unify the shared machinery onto the registry (planned)
status: open
opened: 2026-08-25
github: 981
refs: [978, 979]
---

## From GitHub issue 981

Opened 2026-08-25; 0 comments.

Recording Ev's ask (2026-08-25, the checks-registry conversation, on reading PR #978's diff): the registry unit deliberately unified none of the existing discipline machinery (DISCIPLINES-DESIGN DS8's no-speculative-refactor rule — sharing lands where a second consumer makes it real), and the unification is now **planned**, not merely possible. Not scheduled by this issue.

The plan, in order:

1. **The finding/menu sink** — the document layer's one rendering door: `refusal_menu`'s discipline refusals (`FlushFinding` / `NodeErrorKind::UndeclaredContact`, `crates/editor-core/src/eval/wire.rs`) and `checks::CheckFinding` rendering through shared machinery. The seam DS8 already names as first.
2. **The DS1 discipline scaffolding** — the shared five-part-pattern plumbing (ladder walk, verify-table shape, detector-as-verifier, refusal menu) that today is hand-built three times (profile tangency #101, carrier equality, declared contact C4). It materializes **with the parameter-coincidence unit** (the first new discipline — a template with no second instance is speculation; the second instance is what proves the template), after which the flush/Rest detect-declare pieces (`names/flush.rs`) migrate onto it.

Explicitly NOT planned, by DS1's own rule: moving the three disciplines' predicates and verify tables out of their geometry homes. The registry unifies the machinery *around* the predicates; each stratum's mathematics stays where its geometry lives (the C4 tables in `topo`, the joint classifier in `profile`).

Pointers: `docs/DISCIPLINES-DESIGN.md` DS1/DS8/DS9, PR #978, #979 (the void-birth-marking plan, independent).

## Home

The registry and the finding sink live in `crates/editor-core/src/checks.rs` and `eval/wire.rs`, which no open program's territory covers, so it lands unowned under `work/issues/`.

## Orchestrator disposition, 2026-09-11

`work/fix/plan.md` has held this row undispatched since the program
opened, as the one item on the slate whose fix is **not written in its
body** — it names a seam and an order, not a diff. That hold stands,
and here is what it now resolves to, step by step.

**Step 2 is settled and owes this program nothing.** `work/docm/plan.md:94-96`
already carries it as a named rider: *"`unify-discipline-machinery-onto-registry`
step 2 once the parameter-coincidence unit exists."* Both sides
therefore agree in writing, which is what a cross-program handoff
needs and what this row previously lacked. No file is owed on DOCM's
slate and none should be written there. (The same line records half
(2) of `mate-clocking-has-no-gui-path` as DOCM's, with FIX holding
half (1) — that pairing is confirmed too.)

**Step 1 is FIX's ground but is not yet FIX-shaped.**
`crates/editor-core/src/checks.rs` is in this program's `paths` glob
and `eval/wire.rs` is in no open program's, so there is no fence
question. The charter question is real though: FIX is *"one-PR items
whose body already contains the fix"*, and a refactor of two rendering
doors is not that.

**What has to be established before it can be cut, and it is one
question, not a design exercise.** DS8's rule is that sharing lands
where a **second consumer makes it real**. This row was filed
2026-08-25 asserting the second consumer was not yet there. Since
then this program has landed several units into exactly that
vocabulary — `checks-product-refusal-degrades-to-string` (PR 2344),
`census-flattens-the-typed-chart-region-declines` (PR 2354) — so the
premise may simply have expired. Someone has to read `refusal_menu`'s
discipline refusals and `checks::CheckFinding` **as they stand today**
and answer:

- Is there now a second real consumer of the finding/menu sink, in the
  DS8 sense?
- If yes, the seam is real and this is a spec-able unit; write the spec.
- If no, the row is **parked on a trigger that has not fired**, not
  merely held — and it should say so with the trigger named, which is
  what `work/README.md` requires of a `parked` row and what this one
  has never had.

That question is the next action on this row. It is a reading pass,
not a refactor, and its output is either a spec or a `parked` header.
