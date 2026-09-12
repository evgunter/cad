---
id: interrogate-writes-the-family-vocabulary-a-third-time
kind: issue
title: names/interrogate.rs matches ValuePayload and spells six family words itself, a third copy of kind_name's match in a file that never sees eval::family
status: review
opened: 2026-09-11
refs: [2376]
branch: wire/names-vocab
pr: 2474
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
