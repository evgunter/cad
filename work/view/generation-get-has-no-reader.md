---
id: generation-get-has-no-reader
kind: issue
title: Generation::get has no reader anywhere in the repo, and it is a third of the new leaf module's API
status: open
opened: 2026-09-06
refs: [2079]
---



Found by the style review of #2079.

## What

`crates/viewer/src/generation.rs:48-50`:

    /// The raw counter, for a caller displaying it.
    pub fn get(self) -> u64 {

There is no such caller. `rg 'generation[a-z_]*\(\)\.get\(\)|generation\.get\(\)|Generation::get'`
over the whole tree returns nothing, and `Generation` has no other
door out to `u64` — no `Display`, no `From`, no public field. So the
counter it wraps is unobservable from outside the type, and the one
accessor written to make it observable is dead.

Two consequences the split makes newly visible:

- The new module is 51 lines and its public surface is `FIRST`,
  `next` and `get`. One of the three has no reader, which is a third
  of the API of a module whose whole argument is that it is small
  enough to be a leaf.
- `next`'s doc comment (`generation.rs:33-44`, twelve lines) argues at
  length about what happens at the `u64` ceiling — *"at the ceiling
  every request shares a generation and the staleness filter degrades
  to accepting everything"*. Nothing can reach that state and no test
  can construct one, because there is no `Generation` constructor
  taking a `u64`. The justification is longer than the function and
  describes an unobservable.

Neither is caused by #2079 — both travelled verbatim from
`evalseam.rs` — but the move is what put them alone in a file where
they are the majority of it.

## Class

Same shape as `work/view/tool-kind-all-and-ordinal-have-no-production-reader.md`
and `work/view/opoutcome-superseded-has-no-production-reader.md`. If a
sweep runs, the other read-back doors on the moved types
(`PickIndex::generation`, `PickIndex::delta`, `IdMap::len`,
`IdMap::is_empty`, `EdgeOverlay::segments`) are where I would look
next.

## Confidence

`sure` that `get` has no reader. `likely` that the ceiling paragraph is
worth trimming rather than kept.
