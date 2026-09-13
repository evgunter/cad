---
id: interrogate-writes-the-family-vocabulary-a-third-time
kind: issue
title: names/interrogate.rs matches ValuePayload and spells six family words itself, a third copy of kind_name's match in a file that never sees eval::family
status: closed
opened: 2026-09-11
refs: [2376]
pr: 2474
closed: 2026-09-13
---


## Finding

Found by WIRE's `family-consts` lane (PR 2376) in its sweep, outside the
unit's fence; filed here by the WIRE orchestrator. Accurate at `af8bbca`.

`crates/editor-core/src/names/interrogate.rs:438-446` matches over
`ValuePayload` and produces `none("datum")`, `none("profile")`,
`none("declarations")`, `none("mate")`, `none("measure")`,
`none("assertion")` — **`ValuePayload::kind_name`'s match written a
third time**, in a file that never imports `eval::family`. EVAL-11
(PR 2195) gave this vocabulary one home and wired `kind_name`,
`node_value_kind` and `wire::body_operand`'s `found:` to it; PR 2376
wired `wire.rs`'s seven `expected:` sites. This is the reader neither
pass could see, and the lane calls it *"the largest remaining instance
of this class"*.

Two routes, and choosing between them is the unit: call `kind_name()` on
the payload the match already has in hand, or import the consts and keep
the match. The first is smaller and removes the match; the second keeps
whatever reason the match exists for, if there is one — read it before
deciding, because a match that exists only to re-spell `kind_name` is the
finding and a match that narrows is not.

`none("empty boolean")` at `:423` is a composed sibling and stays prose,
on PR 2376's own argument for the eight composed phrases in `wire.rs`:
a phrase that is not a family word gains nothing from a const, and the
tree has no compile-time string concatenation to build one with.

## Fence

**`crates/editor-core/src/names/interrogate.rs` is in no open program's
`paths`** — checked against every `program.md` at `dc251ce`. WIRE takes
the row because the VOCABULARY is WIRE's (`eval::family` lives in
`crates/editor-core/src/eval/mod.rs`, this program's file) even though
the reader is not. The unit that lands it **draws the fence in the PR
that mints it**, which is the convention `work/topo/program.md`'s
`keep_out` states for the unowned `topo/src` files and the same
situation one crate over.


## Closed 2026-09-13 (PR 2474)

**Both routes, and the reading is what separated them.** The match
**narrows** — five arms gate on `index == 0`, map through
`SplitHalf::of_output_body`, index a vector, or answer the composed
`none("empty boolean")` — so it stays; its **six terminal arms** did
not, and became one or-pattern answering `none(payload.kind_name())`.
`eval::family` is deliberately not imported: `kind_name` is the door.
Exhaustiveness is preserved with no wildcard, confirmed by the review
with a scratch `ValuePayload` variant producing exactly three `E0004`s,
one of them this match.

**No fourth reader exists**, and that is a measured negative rather than
a clean grep. The lane swept the vocabulary and **stated its blind
spots**; the review then went after the two that could have hidden a
reader — a word bound to a differently-named const, and a word built at
runtime — by shape rather than by word, reading each of the nineteen
files with a `ValuePayload::…` arm. A stated blind spot is one someone
else can close, which is the whole reason the rule asks for it.

**The unit's own "no test row is possible" argument was wrong, and the
correction is the useful part.** The premise — that `output_body` is
reached only through `entity_of` after a name resolves — was falsified
by the reviewer with a fixture: `clearance::clearance` is public, takes
a caller-authored `Selection`, and reaches the arm with no name in the
picture. The fix pass did not re-assert the conclusion under a new
premise; it enumerated all four callers and found the word is computed
on a reachable path and observable on **none**, **because the two
reachable callers destroy it** (`map_err(|_| …)` and `.ok()?`). That is
contingent, not structural: the moment SHELL repairs `clearance.rs:1979`
the word becomes observable and the fixture becomes the row that pins
it. Evidence and fixture appended to
`work/shell/clearance-reports-a-no-bodies-payload-as-a-bad-body-index.md`,
authorship kept, carried as a fenced block because `work/` is
markdown-only.
