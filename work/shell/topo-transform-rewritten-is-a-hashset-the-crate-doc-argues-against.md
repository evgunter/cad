---
id: topo-transform-rewritten-is-a-hashset-the-crate-doc-argues-against
kind: issue
title: transform.rs's rewritten set is the one HashSet in topo, and the crate doc spends a paragraph concluding a SecondaryMap would be cheaper and D9-consistent
status: open
opened: 2026-09-11
refs: [S40]
---


## Carved out of `S40` (2026-09-11, by the WIRE orchestrator)

`S40` ("Residue and editing artifacts", accepted by Ev 2026-08-18)
was one file carrying four unrelated residues on three programs'
territory. WIRE inherited it whole in the cut of 2026-09-11 and owns
only one of the four; the rest are filed where the code lives, per
`work/README.md`. `S40` keeps its id and its `WitnessSlot` row; this is
the fourth of its four bullets, verbatim below.

## Finding (`S40` bullet 4, verbatim)

- **Confidence**: sure

`crate docs` devote a paragraph to defending the single `HashSet` that
violates D9's determinism rule, concluding "a `SecondaryMap` would be
both cheaper and consistent with the rule" — for a set used at three
sites (`topo/src/lib.rs:60`). **STILL OPEN** — a D9 determinism design
call, and the comment itself is W3b's pass.

## What it is, located (2026-09-11)

Re-read at `8851abb`. The set is **one** site, not three:
`crates/topo/src/transform.rs:551`, `let mut rewritten =
std::collections::HashSet::new();` — the curve keys the walk remapped,
so an orphaned entry is refused as corruption. The paragraph defending
it is `crates/topo/src/lib.rs:57-69` (`# Determinism (D9)`), and it
is nine lines to say that a membership-only, never-iterated set cannot
leak hash order, ending: *"a `SecondaryMap` would be both cheaper and
consistent with the rule, and is the preferred form for anything new."*
`crates/topo/src/body.rs:18` carries a second sentence pointing at the
same fact.

**It is not a design call.** D9 is ratified and the crate doc already
states the preferred form; converting `rewritten` to a `SecondaryMap`
*conforms* to D9 rather than revising it, and it lets the paragraph
shrink to the invariant it is defending instead of the exception it is
excusing. `S40`'s "design call" reading is from 2026-08-18 and is the
one thing this file re-judges — say so in the PR if you disagree.

## Why it is SHELL's

`crates/topo/src/transform.rs` is SHELL's by `work/shell/program.md`'s
`paths`, and it is the only code the row touches. `crates/topo/src/lib.rs`
is in no open program's `paths` — TOPO's `keep_out` enumerates its files
deliberately and does not claim it — so the crate-doc paragraph is a
sentence-level follow-on in the same PR that moves the set, announced
rather than fenced. If SHELL would rather not hold this, moving it is a
`git mv` and this section is the record of why it arrived.
