---
id: readme-ratification-amendments-need-ev
kind: ruling
title: The viewer README paragraph that replaced a retired kind overstates; the approval-rule gap it exposed is closed in CLAUDE.md
status: open
opened: 2026-09-12
needs_ev: true
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

**That is false in a live case.**
`work/door/viewport-pointer-buttons-mirror-a-toolkit-enum-by-hand`
documents a complete mirror of `egui::PointerButton` inside
`crates/viewer/src` whose declaring crate cannot publish an `ALL` — the
gate misses it only because it is an inline array rather than a `const`.
So the retired kind was vacated of *rostered* instances, not falsified,
and the replacement paragraph overstates.

Three answers, and the row does not presume one:

1. **Leave it.** The roster is what the gate reads and it is correct.
2. **Amend the paragraph** to say the kind was vacated rather than
   falsified, naming the toolkit case as the reason the reason survives.
3. **Restore the kind** and roster the pointer-button list under it —
   which needs the gate to see inline arrays, and is therefore the
   largest of the three.

The orchestrator's read is **(2)**: cheapest, and it makes the sentence
true without asking the gate to grow. But this is the call the rule now
reserves for Ev.
