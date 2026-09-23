---
id: PORT-DIMS-1
kind: unit
title: The dimension refusals at the Python boundary: keep the structure at the load door, and let the name mean what Rust means
status: closed
opened: 2026-09-15
refs: [694, 689, S107]
branch: port/dims-1-load-door-and-name
pr: 2702
closed: 2026-09-23
---


One door, two questions, one unit. `load-path-stringifies-structured-refusals`
asks that a structured kernel refusal keep its structure when it crosses
the load door; `python-dimensionerror-names-the-quantity-check-not-the-dimension-check`
asks what the class it arrives as is called. Neither is answerable alone:
the first row's fix has to name a class, and the class the second row
frees is the candidate. Specced together (`work/port/plan.md`, Order).

## The rows

- `load-path-stringifies-structured-refusals` — **H**, and the one row
  this program gives a full review (`plan.md`, Review posture, triggers
  2 and 3).
- `python-dimensionerror-names-the-quantity-check-not-the-dimension-check`
  — **M**, `S107`'s successor. Ev ruled the naming a defect on
  2026-09-15: *Python should always match Rust where it can.*

## Spec

`docs/PORT-DIMS-1-SPEC.md`, deleted at merge with its `docs/DOC-LEDGER.md`
row in the same PR.

## Territory

`crates/editor-core/src/persist/*` is **EDIT's**; `crates/pncad-py/*` is
**LIB's**. PORT claims no paths; both are announced and either may take
the unit.

## Closed (2026-09-23, #2702)

Both rows closed with the unit. The spec is deleted per
`memories/docs-ledger.md`; its note is `docs/doc-ledger/port-dims-1-spec.md`.

**Three of the spec's statements were wrong, and the item files are the
record of each.**

1. The spec listed among the things it had **confirmed** that a rebuild
   refusal *"becomes `PersistError::Parse`, whose Python tag is
   `parse`"*. It becomes `PersistError::Unreadable`, tag `unreadable`,
   and has since PR 1553 routed body refusals by
   `serde_json::error::Category` — `Parse` is the reader's own classes
   and a rebuild refusal is `Data`. The four-week-old row the spec
   inherited the claim from said `parse` too, and so did five doc
   comments, the binding census and the guide; the unit corrected all of
   them. **The correct fact was already in the tree** — `parse_err`'s
   own doc said so, and the `pncad-py` probe was named
   `…_as_an_untyped_unreadable_refusal` — while eight prose sites had
   copied the wrong one onward.
2. The spec said the row's first sweep *"names instances that are gone
   or were never instances"*. True of the `pncad-py` half only; the
   `persist/` half was exact — thirteen `Error::custom` calls, disposed
   one by one in the PR.
3. The spec sketched three directions for where the structure goes and
   called them equally open. They are not: a deserializer adapter costs
   the reader's line/column payload, and validating ahead of serde costs
   a parallel wire tree for the whole document. The side channel is the
   one that exists, and the unit priced the other two rather than
   assuming them.

**What the unit changed after review.** The refusal channel became a
guard type rather than a bare `record`/`take` pair, because
`docs/PERF-SCAN-2026-08.md` states the standard for a thread-local that
delivers a production value — an RAII guard whose `Drop` harvests,
re-entrancy that fails loud rather than discarding, thread confinement
enforced by the type, and the cross-crate coupling visible at both ends
— and `k_stats::Bracket` (PR #1969) is the worked precedent, recorded in
`work/scalar/D283.md` as *"the thread-local stays; its correctness is
now a type."* The first draft met none of the four.
