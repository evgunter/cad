---
id: generation-get-has-no-reader
kind: issue
title: Generation::get has no reader anywhere in the repo, and it is a third of the new leaf module's API
status: closed
opened: 2026-09-06
refs: [2079]
closed: 2026-09-06
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

## Closed

**`Generation::get` deleted**, and `next`'s ceiling paragraph trimmed
to what is constructible.

**Why deletion rather than the `tool-kind-all-and-ordinal` precedent.**
That row kept a `pub` item with no PRODUCTION reader because the suites
read it, and the close was a doc saying so — the item was preserved
because it had a reader, and the doc was made honest about which one.
There is no reader here at all, in `src`, in `tests`, in `examples` or
in the demos, so the precedent's premise is absent and its conclusion
does not carry. What that row actually settled is that a reader
somewhere is enough; it did not settle that `pub` alone is.

**The display need the doc named survives the deletion.** `get`'s doc
was *"the raw counter, for a caller displaying it"*, and `Generation`
derives `Debug` — so `{:?}` in a log and a debugger both still show the
number. Deleting the accessor removes a second door onto what `Debug`
already opens; it does not make the counter unobservable, which is what
would have made the deletion a real loss.

**The "external consumer" argument has no population.** Nothing outside
this repo consumes `viewer`, so keeping a `pub` item against a
hypothetical downstream is keeping it against nobody. When a display
need does arrive it costs three lines, and the type's doc now says
exactly that, so the next reader meets the reasoning rather than
re-deriving it.

**The ceiling paragraph.** Twelve lines argued what happens at the
`u64` ceiling — *"every request shares a generation and the staleness
filter degrades to accepting everything"* — a state nothing can
construct: `FIRST` is zero, `next` is the only way to advance, and
there is no `u64` constructor, so a test cannot reach it either. The
INVARIANT in that paragraph is load-bearing and stays, because it is
the reason the code says `saturating_add` rather than `+`: a wrap makes
a stale result compare equal to the current request, which is the one
thing this type exists to prevent. What went is the description of the
unreachable state; what replaced it is the statement that it is
unreachable and why. Six lines instead of ten, and the module is 51
lines with two public items, both of which have readers.

**Where it is now**, since the citations above name the pre-fix tree:
`crates/viewer/src/generation.rs` has no `get`, and `next` with its
trimmed doc is at `generation.rs:40-50`. The type's own doc gained the
paragraph that says the counter is deliberately unreadable
(`generation.rs:27-32`).
