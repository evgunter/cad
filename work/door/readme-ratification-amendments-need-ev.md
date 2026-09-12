---
id: readme-ratification-amendments-need-ev
kind: ruling
title: The viewer README paragraph that replaced a retired kind overstates; the approval-rule gap it exposed is closed in CLAUDE.md
status: closed
opened: 2026-09-12
closed: 2026-09-12
---


## It was a gap, and CLAUDE.md is where it is closed

Ev's instruction (2026-09-12) was to find where the approval rule lives,
**decide whether this is genuinely a gap**, and only then clarify it
there. The answer is yes, narrowly, and the diagnosis is this program's
own subject.

**The spirit was already unambiguous.** `CLAUDE.md` calls
`crates/<crate>/README.md` pages *"design docs for finished work … with
their clause ids kept"*; it says of settled decisions *"do not
re-litigate … discussed with Ev first"*; and
`memories/orchestration-model.md` closes with *"when unsure which kind a
decision is, treat it as a fork."* That catch-all covers retiring a
ratified clause and should have stopped the orchestrator. It did not,
which is a reading failure and not the document's.

**The letter was a hand-written enumeration missing a member.** The
Git-workflow exception named *"PRs that ratify **open** design
questions"* and `memories/`. Retiring a **settled** clause is neither —
it is close to the opposite of ratifying an open one. And
`orchestration-model.md` names *"changes to ratified **DESIGN.md**
decisions"*: `DESIGN.md` specifically, not the README pages the same
document defines as design docs. So the rule was a list of two homes
where the principle has four, and `docs/prompts/` — read by every lane
by path, binding the orchestrator's own judgement — was the other one
missing.

That is the defect class DOOR spent the day closing, in the rule that
governs DOOR.

**Closed in `CLAUDE.md`**, in the same PR as this row, by stating the
test instead of the list: *"the exception is text that binds future work
rather than describing this change"*, with the four homes as what it
covers today and a sentence saying a new home is covered the day it
exists rather than the day the line is updated. `docs/prompts/` is named
per Ev's suggestion.

## What is left for Ev, which is not a rule question

**PR #2387 broke no rule that existed** — the letter did not reach it —
so nothing here is a violation to remedy, and nothing proposes
reverting. What is left is a **substantive** defect the review of #2391
found, and under the rule as now written its fix needs Ev's sign-off
because it amends a ratified README clause.

The paragraph #2387 put in place of the retired kind claims generally:

> a mirror claiming completeness has an answer one crate over, where the
> owner publishes its own `ALL` beside the declaration and this crate
> maps over it

**The orchestrator's counterexample was misclassified, and checking it
on Ev's question reversed the recommendation.**

The claimed counterexample was
`work/door/viewport-pointer-buttons-mirror-a-toolkit-enum-by-hand`: a
complete mirror of `egui::PointerButton` inside `crates/viewer/src`
whose declaring crate cannot publish an `ALL`. Read properly, it is not
an instance of the retired kind at all. `crates/viewer/src/input.rs` is
toolkit-free by design — its module doc says *"Module kind:
**vocabulary** — it names no driver type and no `app`-only crate"* — so
the viewer's own enum is an anti-corruption boundary and
`pane/viewport.rs` is its adapter. Completeness there is a PRODUCT
decision about which buttons the viewport binds, which is nearer the
surviving *"deliberately partial"* kind than the retired one.

(Checking it also turned up a real defect that has nothing to do with
this ruling: `egui::PointerButton` has **five** variants and the adapter
handles three, so side-button input is dropped silently. That is
recorded on the row itself.)

So **no live instance of the retired kind is known inside
`crates/viewer/src`**, and the paragraph #2387 wrote may be sound as
merged. Three answers stay live, with the weight moved:

1. **Leave it as merged.** The kind had no rostered instances, the gate
   forced the edit, and the counterexample against the replacement
   paragraph has been withdrawn. *(Now the orchestrator's read.)*
2. **Amend the paragraph** anyway, if its general claim — *"a mirror
   claiming completeness has an answer one crate over"* — still reads
   as broader than the tree can support. No counterexample supports
   this today; it would rest on taste about how absolute the sentence
   is.
3. **Restore the kind.** Nothing now argues for it.

The orchestrator's read is now **(1)**, reversed from (2) after Ev
asked why the mirror existed and the answer falsified the premise. This
is still the call the rule reserves for Ev — the row's value is that it
now presents the question without a wrong argument attached.

## Closed — (2), and the diagnosis is Ev's

The recommendation moved three times, which is worth keeping because
the last move is the one that found the actual defect.

1. **(2) amend** — the paragraph overstates, citing the viewport's
   pointer-button mirror as a live counterexample.
2. **(1) leave** — Ev asked why that mirror existed. The answer
   (`input.rs` is toolkit-free by design; the mirror is an
   anti-corruption boundary) showed the counterexample was
   misclassified, and no live instance of the retired kind was then
   known in `crates/viewer/src`.
3. **(2) amend** — PR #2450, closing that very row, built one:
   `egui_buttons()` is a mirror claiming completeness, and **egui
   publishes no `ALL`** (only `NUM_POINTER_BUTTONS`), so the answer the
   paragraph names as *the* answer does not exist at that site.

**Ev's diagnosis, which is better than the orchestrator's.** The claim
was never false — it was **over-specified**. The principle is that a
mirror claiming completeness is held by something the owner declares
rather than by hand; the paragraph wrote down one mechanism for that
(`ALL`, mapped over) as though it were the mechanism. The pointer
button case satisfies the principle by another route — length pinned to
the owner's count, contents pairwise distinct, conversion through an
exhaustive match, which together admit exactly one permutation of the
enum.

So the amendment is smaller than the orchestrator proposed. Not "counts
are allowed too", which is a second special case waiting for a third,
but the property stated once with both mechanisms as instances of it.

**This is the same defect as the approval rule fixed the same morning**
(`CLAUDE.md`, PR #2410): a principle written down as its most familiar
instance, so that the first case outside the instance reads as an
exception. Twice in one day, in the two documents that govern how the
others are written. Worth remembering as a class rather than two
incidents: **when a document names a mechanism, check whether it meant
a property.**
