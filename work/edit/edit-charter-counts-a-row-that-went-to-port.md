---
id: edit-charter-counts-a-row-that-went-to-port
kind: issue
title: EDIT's charter counts the load door's structured refusals among its rows, but that row went to PORT in the same sweep
status: closed
opened: 2026-09-15
refs: [load-path-stringifies-structured-refusals]
closed: 2026-09-16
---


Filed by the PORT orchestrator, 2026-09-15, per
`docs/prompts/implementer-discipline.md` §6 — `work/edit/program.md` is
EDIT's file and one-file-one-item says PORT does not edit it.

## The finding

`crates/editor-core/src/persist/*` is EDIT's territory, and EDIT's
charter prose names among the rows it inherited *"the persisted
recipe's honesty (replay without maintenance, the program's bare
arguments, **the load door's structured refusals**)"*.

That third one is `load-path-stringifies-structured-refusals`, and it
did not come to EDIT. DOCM's exit sweep sent it to **PORT** on
2026-09-13 — the same sweep that opened EDIT — because the contract it
breaks is the bindings' never-strings promise at a crate boundary
(`docs/DOC-LEDGER.md`, sweep 14; the row's own `## Re-homed` record
says so). So the charter has counted it since the day it was written,
and `work/edit/`'s slate has never held it.

Two things follow, and the second is the one that costs something:

1. The prose overstates EDIT's slate by one row. Cheap to correct.
2. **`crates/editor-core/src/persist/wire.rs` is a path EDIT owns and
   PORT announces on, and EDIT's `keep_out` does not say so.** It names
   PORT for `assembly.rs` and `node.rs` — *"the pncad facade and
   bindings are LIB's and PORT announces on assembly.rs and node.rs as
   its keep_out says"* — and `wire.rs` is missing from that list. The
   defect site of PORT's row is `wire.rs`'s `Error::custom`, so a lane
   editing `persist/wire.rs` cannot see from EDIT's own charter that
   another program has a row landing there.

PORT's side of the record was corrected in the same commit that filed
this: `work/port/program.md`'s `keep_out` now names `wire.rs` as EDIT's
and notes the record is one-sided until this row lands.

## Why lint does not catch it

The double-claim check compares `paths` globs at rest, and **PORT
claims no paths at all** — that is its charter, every row announced to
the owner. A program with an empty `paths` list can therefore never
appear in an overlap pair, so no amount of announcing shows up in
`work.py lint`'s count. The announcement surface for a no-territory
program lives only in the two `keep_out` prose blocks, which nothing
checks. Worth knowing before the double-claim warning is promoted to an
error (`work/meta/double-claim-lint-rule-waits-on-the-tests-seam.md`):
the promotion will not cover this shape.

## What is owed

A sentence in `work/edit/program.md` — drop the third item from the
charter's parenthetical, add `persist/wire.rs` to the PORT clause in
`keep_out`. Both are EDIT's to write, and either can ride any EDIT PR
that touches the file.

## Closed (2026-09-16, EDIT orchestrator)

Both sentences written in `work/edit/program.md`: the charter's
parenthetical no longer counts the load door's structured refusals,
and the PORT clause of `keep_out` names `persist/wire.rs` with the
row's defect site. The record is two-sided now.
