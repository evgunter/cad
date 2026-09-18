---
id: assembly-door-raises-only-the-head-of-each-refusal-list
kind: issue
title: assemble_gathered raises only the head of unminted and of carried_unminted; the widening it names as a follow-up is not filed
status: closed
opened: 2026-09-06
parent: PORT-DOORS-1
pr: 2635
closed: 2026-09-15
---



Found by MSOLVE-5's style review (PR 2090, Q8), outside the unit's
fence.

`assemble_gathered` (`crates/editor-core/src/assembly.rs`, the two
`into_iter().next()` sites) raises the HEAD in gather order of
`carried_unminted`, then the head of the document's own `unminted`,
and drops the rest. Its comment says: "Widening this to every carried
refusal is the same follow-up as widening the sibling — one second
refusal channel on `AssemblyError`, carried through the pncad-py
façade, would serve both — so the two heads stay one rule rather
than diverging." A grep of `work/` and `docs/` for "second refusal
channel" and "only the head is raised" finds no item: the follow-up
is recorded in a comment and nowhere the tracker reads.

What is owed is a decision, then either the file or the comment: (a)
one refusal channel carrying every `MintRefusal` (own and carried),
through `AssemblyError` and the Python façade, so an author with two
broken mates learns about both in one evaluation; or (b) a ruling
that the head is the contract, in which case the comment stops
calling it a follow-up. `Product::unminted` already holds the whole
list as data, so (a) is a door change, not a gather change.

## Decided: widen (PORT orchestrator, 2026-09-15)

Ev was asked whether this call was theirs and answered that if the
orchestrator is confident it is the orchestrator's. It is, and the
evidence is stronger than the row assumed: **this is not an open
design question, because the door has already answered it in its own
third arm.**

`AssemblyError::AtRest` (`crates/editor-core/src/assembly.rs`) takes
every finding. Its doc-comment publishes that as a guarantee —
*"Every finding travels, in the kernel's own deterministic sweep
order, each carrying what it says about the declarations"* — and
`assemble_gathered` builds the `Vec<AtRestFinding>` to match, then
splits `Uncertified` from `AtRest` by reading **all** of them. So the
enum already carries a refusal list on the arm where a list was
needed.

The two mint arms are the ones out of step. `CarriedMintRefusal` and
the `unminted` head each `into_iter().next()` and drop the rest, in
the same function, twenty lines from the arm that keeps everything.
Contracting instead would mean narrowing `AtRest` to one finding —
deleting a guarantee the type publishes, to make three arms agree at
the weaker answer. Widening makes the enum consistent with itself at
the answer it already gives.

The code's own comment proposes the same fix and names its shape:
*"one second refusal channel on `AssemblyError`, carried through the
pncad-py façade, would serve both — so the two heads stay one rule
rather than diverging."* That sentence is what this row was filed
against (*"the widening it names as a follow-up is not filed"*), and
it is now filed; per `docs/prompts/reviewer-style-lane.md` Q6 a
recorded pickup is not a schedule, and this row is the schedule.

**What the decision does not settle**, left to the spec: whether the
widened mint refusals reuse the `AtRest`/`Uncertified` shape (a
`Vec` on the arm) or take a channel of their own, and what the
pncad-py façade raises for a list where it raised a scalar. The
façade half is LIB's ground and is announced there.

**Dispatches as one unit with
`product-table-answers-a-tie-before-kind-the-operand-the-reverse`** —
same file, same door, both about what a refusal says, and answering
one alone leaves the two doors disagreeing in a new way
(`work/port/plan.md`, Order).

## Re-homed to PORT (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

PORT collects the crate-boundary doors — the Python surface, the
exchange crates and the façade refusals — where the thing a user meets
is a refusal. This row is one of them.

Its class at the cut was **M** — decide widen-vs-contract first;
widening is a public error-channel change through pncad-py. The class is
a dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.
