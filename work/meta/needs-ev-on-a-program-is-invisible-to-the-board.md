---
id: needs-ev-on-a-program-is-invisible-to-the-board
kind: issue
title: needs_ev is schema-valid on a program and the Waiting on Ev queue can never show one, so a program's exit ask is invisible on the board
status: open
opened: 2026-09-15
priority: P3
cost: E
---


## Finding

Raised by the SUITE orchestrator, 2026-09-15, on setting `needs_ev: true`
on `work/suite/program.md` to say that the program's exit walk waits on
Ev — and finding the board unchanged.

**The schema permits it and the renderer cannot show it.**

- `scripts/work.py:83` — `"needs_ev": ("flag", ("unit", "issue", "ruling", "program"))`.
  `program` is in the tuple, so `lint` accepts the key on a program header
  and reports 0 problems.
- `scripts/work.py:587` — `live = [it for it in items if it.kind != "program" and it.status != "closed"]`.
- `scripts/work.py:602` — the "Waiting on Ev" queue is built from `live`.

So a `needs_ev` on a program is silently dropped from the one view that
exists to carry it. Measured: with `needs_ev: true` on `work/suite/program.md`
and the program `open`, `work.py status` lists seven rows under
**Waiting on Ev** and `suite` is not among them. The per-program summary's
`on Ev` column is empty too — `:622` counts over the program's own items,
which never include the program.

## Why it matters, rather than being a cosmetic gap

`work/README.md` states the guarantee this breaks, in as many words:

> `STATUS.md` lists every open `needs_ev` oldest first, so the two views
> (the PR list filtered on `[ev]`, and the tracker) always name the same
> set.

That is **false for a program-level ask**, and a program-level ask is not
an edge case: **a program closing is exactly when one happens.** The exit
walk is the one question every program eventually puts to Ev, and it is
the one question the board cannot display. An orchestrator who sets the
flag, sees `lint` pass, and trusts the README's guarantee has recorded
its ask somewhere nothing reads — which is the defect shape SUITE's own
units spent a day cataloguing.

## What would settle it

Three shapes, and the choice is a tracker-contract question rather than a
code one:

1. **Include programs in the queue** — drop `it.kind != "program"` from
   `:587`, or build the queue from `items` rather than `live`. The queue
   already prints a `program` column, which would read as the program's
   own id. Smallest change; makes the README's guarantee true.
2. **Refuse the key on a program** — remove `"program"` from `:83`'s
   tuple, so `lint` rejects it and an orchestrator is told at once. Then
   a program's exit ask needs a carrier item, and the contract should say
   which (a `kind: ruling` row, per `work/README.md`'s "a question only
   Ev answers"?).
3. **Leave both and say so** — document at `:83` and in `work/README.md`
   that a program-level `needs_ev` is a header fact only, never a board
   row, and that the `[ev]` PR is its sole channel.

(1) is what this row's author would take: the flag already means the
right thing, the column already exists, and it is the option that makes
the README's stated guarantee true rather than narrowing it.

**SUITE is the live instance.** Its `program.md` carries `needs_ev: true`
for its exit walk and will keep carrying it until the walk is ratified;
until this is settled, that ask is visible only on the `[ev]` PR and in
the header itself.
