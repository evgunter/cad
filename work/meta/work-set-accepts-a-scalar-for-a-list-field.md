---
id: work-set-accepts-a-scalar-for-a-list-field
kind: issue
title: work.py set writes a scalar into a reflist field and lint crashes instead of diagnosing it
status: closed
opened: 2026-09-08
closed: 2026-09-11
---


## Finding

Reported by the GATES lane for PR 2170 (2026-09-08).
`python3 scripts/work.py set <id> blocked_on=2171` writes
`blocked_on: 2171` — a scalar into a field the schema treats as a list
of refs — without complaint, and the next `python3 scripts/work.py
lint` dies with an uncaught `TypeError: 'int' object is not iterable`
at `scripts/work.py:389` (`for v in it.get(key) or []`) instead of
reporting a malformed field on that item. The working spelling is
`blocked_on=[2171]`, which nothing tells the caller.

Two fixes, both small: `set` coerces a scalar into a one-element list
for the list-typed fields (`refs`, `blocked_on`, `rides_with`), or
`lint` reports "`<id>`: `blocked_on` is not a list" and keeps going.
The second makes the first unnecessary to remember; the first is what
a caller wants. Filed on META's slate because `scripts/work.py` is its
path.

## Closed (2026-09-11)

**Both fixes taken, because the item is right that they answer
different halves.** `set` no longer produces the shape (what a caller
wants), and `lint` no longer dies on it (what a hand-written header
still needs).

- `_parse_assignment` coerces a bare scalar into a one-element list for
  every list-typed field, so `work.py set <id> blocked_on=2171` writes
  `blocked_on: [2171]`. It reaches `new --set` by the same path.
- Every reader of a list-typed field in `lint` and `render` now goes
  through `_listed()`, which yields nothing when the header does not
  hold a list. `_check_type` already reported `must be a list` and the
  run died four checks later before printing it; now it prints it.

**One correction to the item.** It names the list-typed fields as
`refs`, `blocked_on` and `rides_with`. `rides_with` is a `ref` — a
single id, by schema — so it is not coerced and must not be; the
coerced set is `refs`, `blocked_on`, and the three `strlist` program
fields (`paths`, `keep_out`, `blocks`), which had the same trap.

**Proof it works, from this PR's own use of it**: the two residue items
filed here were created with `--set refs=<id>` and carry
`refs: [<id>]`, which is the spelling the item says nothing told the
caller about.
