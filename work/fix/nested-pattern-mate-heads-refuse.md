---
id: nested-pattern-mate-heads-refuse
kind: issue
title: Nested patterns and pattern-of-transform mate heads refuse DanglingHead — narrower than the A11 rider's literal text
status: open
opened: 2026-08-31
github: 1411
refs: [1400]
---

## From GitHub issue 1411

Opened 2026-08-31; 0 comments.

Filed from the MATE-1 dual review (PR #1400, both arms; R1 MINOR-5(a)). The landed member vocabulary accepts `Pattern` + `Instance(i)` over a live `InstantiatePart` input only. A nested pattern's copy (pattern-of-pattern) and a pattern over a transform both refuse `DanglingHead` at the (outer) pattern node — typed and honest, and disclosed in the PR body, but NARROWER than the rider's literal text, which fences the head spelling (`Pattern` node + `Instance(i)` qualifier) and says nothing about the pattern's *input*.

Two dispositions possible, and the choice is a small design call: extend `head_of`'s member vocabulary through nested inputs (the derived offset composes through the chain — the rule-1 conjugation is associative over it), or ratify the single-level fence as intended v1 scope with a sentence in the rider. S-MATE's backlog either way; not scheduled as a unit yet.

Signed: (S-MATE orchestrator)

## Home

`work/mate/` — the issue names S-MATE's backlog explicitly, and mates × patterns (the A11 member vocabulary) is the program's charter ground.
