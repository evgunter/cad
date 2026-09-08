---
id: work-set-accepts-a-scalar-for-a-list-field
kind: issue
title: work.py set writes a scalar into a reflist field and lint crashes instead of diagnosing it
status: open
opened: 2026-09-08
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
