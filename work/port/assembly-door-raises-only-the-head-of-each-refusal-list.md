---
id: assembly-door-raises-only-the-head-of-each-refusal-list
kind: issue
title: assemble_gathered raises only the head of unminted and of carried_unminted; the widening it names as a follow-up is not filed
status: open
opened: 2026-09-06
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
