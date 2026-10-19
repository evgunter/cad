---
id: a-space-names-its-bodies
kind: issue
title: A space names its bodies to its readers: a pick is a body variable of the space, defined as one placement's copy
status: open
opened: 2026-10-18
priority: P0
cost: M
refs: [a-placement-relates-two-frames]
---

From row 162 (PR 5312). Ev, chat 2026-10-18: "a space is defined by a set of placements, and then provides a nice api to things downstream where the api doesn't say anything about how the geometry got there". The ratified sentence is "what reads it downstream reads the space's bodies, never a placement". But nothing represents a space's body today. A copy is an output of its placement, read by key (`CopyKey`, `BodyRead.key`, `crates/editor-core/src/operand.rs:502`; `copy_of`, `eval/wire.rs:2575`). Its names are re-minted under the placement node (`RoleSeg::Placed`, `names/role.rs:1579`; `name_placed*`, `names/emit.rs:919–1000`; `InPart`, `emit.rs:951`). Line numbers are stage 3 D's branch at bf45076901.

Weighed by an Opus and a Fable designer (fork-log row 163). The clause is in the `[ev]` PR that opens this item.

**Converged:**
- A body of a space is a variable of that space: a construction's or operation's output, or a *pick*. A pick is a `Body` variable defined as one copy a placement places there; it is a `Bodies` family read by index for a pattern.
- Readers hold the pick, and nothing else reads a placement's copies.
- A copy's names pass through from the source. An operation reading two picks of one source keys names by the pick (`From { read }`).
- `Placed`, `InPart`, `CopyKey` and `copy_of` retire.
- Picks are unnamed unless named (VR2).
- There is no enumeration read.

**Split, for Ev:**
- (1) A pick's `of` is either strictly a variable of the source (an unpicked body of another document refuses `NoSuchBody`, and the author picks it in the source and re-pins), or a path through the source's placements to the first variable, which is its one spelling.
- (2) A second pick of one copy is either refused, or two variables, as for a selection.

Built in stage 3 D′ (`a-placement-pins-by-written-values-without-a-mate`, filed on stage 3 D's branch), with spec §1.

Seen, not taken: `RoleSeg::From`'s doc on D's branch says "a placement adds no name segment", which `name_placed*` contradicts. It becomes true when `Placed` retires. If D′ does not retire it, fix the comment.

**Ev (PR 5315, 2026-10-19).** (1) The path to the first variable: "good, B is better/necessary then". (2) No preference, so two variables, as for a selection.
