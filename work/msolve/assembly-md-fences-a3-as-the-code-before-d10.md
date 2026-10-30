---
id: assembly-md-fences-a3-as-the-code-before-d10
kind: issue
title: ASSEMBLY.md still fences A3 under 'Until the intent refactor lands' and misspells InstantiatePart's fields, though stage 3 built D10's placement
status: closed
opened: 2026-10-29
closed: 2026-10-30
priority: P4
cost: E
refs: [d10-exit-walk]
---


Found by the D10 exit walk (`work/intent/d10-exit-walk-report.md`, ASSEMBLY A3).

`crates/editor-core/ASSEMBLY.md:11-14` says A3 is gathered under "Until
the intent refactor lands" and describes "the code as it stands, in the
model DESIGN `[placement]` … replace[s]" (`:567-571`). Stage 3 closed
today (B #5232, C #5267, D #5506, F #5299, G #5954). A3's body (`:573`
on) now states D10's placement: a bundle of mates and values, picks,
and index variables in place of patterns. So the fence no longer
separates a superseded model from D10's.

A3 also spells `Node::InstantiatePart { doc_ref, interface, constraints }`
(`:574`). The node has a `target` (`crates/editor-core/src/node.rs:1855`,
"The space the copy is placed into"). And "the world … read only by a
placement's constraints and by export" (`:582`) predates the space
kinds: the world is a placement's target (DESIGN `[placement.spaces]`).

DESIGN's companion table row for ASSEMBLY says "A3 and A5 fenced until
INTENT retires them". A5 is no longer fenced (`## At rest`, `:433`).
That row is in DESIGN.md, so its rewording goes to Ev with the page's.

Fix: move A3 out of the fence into "Nodes and mates", fix the two
spellings, and drop the header paragraph. Check each sentence against
`[placement]` while doing it.

## Closed

2026-10-30 (`intent/stale-companion-pages`). The header paragraph and
the "Until the intent refactor lands" section are gone, and A3 sits
under "Nodes and mates" ahead of A12. It spells
`Node::InstantiatePart { doc_ref, interface, target, constraints }`
(`src/node.rs:1839`). The world sentence now reads the code: the world
defines the world's space (`VarId::WORLD`), and a placement reads a
space only as its target. DESIGN's companion-table row for ASSEMBLY
was reworded in the same PR.
