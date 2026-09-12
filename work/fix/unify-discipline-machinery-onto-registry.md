---
id: unify-discipline-machinery-onto-registry
kind: issue
title: checks/disciplines - unify the shared machinery onto the registry (planned)
status: closed
opened: 2026-08-25
closed: 2026-09-11
github: 981
refs: [978, 979, 984, debug-in-prose-residue-after-finding-sink]
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

## Reading pass, 2026-09-11 — the answer is neither arm: step 1 already landed

The one question was *is there now a second real consumer of the
finding/menu sink, in the DS8 sense?* Read at the merge base, in the
code rather than in this row's summary, the answer is **yes — and the
sink itself is already built**. Step 1 is not spec-able because it is
done. It landed as **PR #984**, before this row was ever homed here.

**The receipt is the module's own header.**
`crates/editor-core/src/finding.rs:1-6` opens:

> **The document layer's finding sink** (DISCIPLINES-DESIGN DS8;
> #981 part 1): one composition and one list rendering for the
> layer's finding surfaces — the checks report and refusal, the
> undeclared-contact refusal, and the assembly at-rest gate […]

`#981` is this row's own `github:` number. The file names itself as
part 1 of this plan.

**Three consumers, not two, and all three are wired — not merely
declared.** The trait is `finding::Finding`
(`crates/editor-core/src/finding.rs:41`), with one composition
(`compose`, :57) and one list rendering (`render_list`, :73):

- `impl crate::finding::Finding for CheckFinding` —
  `crates/editor-core/src/checks.rs:381`; called at `checks.rs:467`
  (`compose`), `:492` and `:619` (`render_list`). This is step 1's
  named `checks::CheckFinding` arm.
- `impl crate::finding::Finding for UndeclaredContactFinding<'_>` —
  `crates/editor-core/src/eval/mod.rs:1376`; called at
  `eval/mod.rs:1676`, inside `Display for NodeErrorKind`'s
  `UndeclaredContact` arm. That arm's payload is exactly what
  `refusal_menu` builds at
  `crates/editor-core/src/eval/wire.rs:3606-3657` — the boxed
  `names::FlushFinding` and the ladder's `diag`. This is step 1's
  named `refusal_menu` / `FlushFinding` arm.
- `impl crate::finding::Finding for AtRestFinding` —
  `crates/editor-core/src/assembly.rs:450`; called at
  `assembly.rs:466`, `:781`, `:793`. The third consumer, which this
  row never named.

**DS8 read directly** (`docs/DISCIPLINES-DESIGN.md:485-508`), not
through this row's summary: *"they adopt shared machinery only where a
second consumer makes the sharing real (the finding/menu/severity sink
at the document layer is the first such seam — `refusal_menu` in
`editor-core/src/eval/wire.rs` already renders discipline refusals
through one door)"* (:492-496). `refusal_menu` was consumer one at the
time of writing; `CheckFinding` and `AtRestFinding` are two and three
today. The DS8 condition is met and was met by the unit that landed
the sink.

One deliberate narrowing, recorded here because DS8's phrase is
*finding/menu/severity* and what landed is finding/menu only:
`finding.rs:12-22` fences severity and check identity out of the sink
by name — *"no severity and no check identity — those are report
plumbing (`crate::checks`), not part of what a finding says"* — along
with a recourse enum and refusal-to-run types. This row's own step-1
text says "the finding/menu sink" and matches what landed; the
narrowing is argued at the site and is not re-opened here.

**Step 2 is unchanged and is DOCM's.** `work/docm/plan.md:94-96`
carries it verbatim as a rider — verified at the merge base, the line
exists and reads as the orchestrator disposition quotes it. No
parameter-coincidence item exists anywhere under `work/` (grepped
`parameter.coincidence` over the whole tracker: three hits, all
prose — this file, `work/fix/plan.md:85-91`, and that DOCM rider), so
there is no id and no PR number a `parked` header could name and lint
could see fire. Per `work/README.md`'s `parked` rule and this unit's
brief, that makes `parked` the wrong status rather than one to invent
a trigger for.

**Residue is already filed and is not this row's to carry.** The
finding sink's prose-debt residue is
`work/docm/debug-in-prose-residue-after-finding-sink.md` (issue 985,
open, on DOCM's slate), whose body records PR #984 as "#981 part 1"
and lists what it left. Nothing further is owed here.

So the row closes: step 1 shipped at PR #984 with three consumers on
the sink, step 2 rides DOCM's plan, and the DS1 predicate move stays
explicitly not planned.
