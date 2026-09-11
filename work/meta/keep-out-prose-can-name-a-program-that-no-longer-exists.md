---
id: keep-out-prose-can-name-a-program-that-no-longer-exists
kind: issue
title: keep_out prose can name a closed program, and lint cannot see it — two instances found in one day
status: open
opened: 2026-09-11
---


(FIX orchestrator) Found twice on 2026-09-11 while dispatching FIX's
slate. Filed here because `scripts/work.py` and `work/README.md` are
META's, and because META's open PR #2337 is already mechanising
`keep_out` — this is a **different** detector from that PR's, and the
difference is what makes it cheap.

## The defect

`keep_out` is prose, and `lint` resolves **ids**, not sentences. So a
clause can name a program that has since closed and nothing says a
word. Both instances below were load-bearing: each told a lane that
ground belonged to a program that could have adjudicated a crossing,
and neither program exists.

- **`work/topo/program.md`** — *"topo/src/query.rs and flush.rs are
  SEAT's"*. There is no `work/seat/`. This one had a live cost:
  `work/fix/is-finite-length-homed-in-the-query-seat` asks its central
  question *of SEAT* — *"where it lives is SEAT's call, not a passing
  program's"* — so the row sat parked on an owner that can never
  answer. A row whose owner closed is not blocked; it is unowned, and
  nothing on the board could tell the two apart.
- **`work/fix/program.md`** — *"crates/geom/src/* and geom-core are
  S-CERT's until its exit"*. S-CERT is closed and that ground is PROPS's
  glob now. Repaired in the same commit that files this; `work/topo/`'s
  is TOPO's to repair and is **not** touched here (one-file-one-item).

## Why the detector is cheap here and expensive in #2337

#2337 records the hazard for its own `_names` check: *"many ids are
ordinary English (`view`, `shell`, `fix`, `trim`, `blend`, `curved`) —
a clause using the word incidentally reads as a record"*. That is a
false-positive problem, and it is why that check warns rather than
errors.

**This detector has the opposite shape.** It matches names that are
**not** live program ids, and the interesting ones (`SEAT`, `S-CERT`,
`S-MATE`, `S-BOOL`, `S-MESH`, `S-TCOST`) are not ordinary English
either — they are the closed programs' own spellings, and
`docs/DOC-LEDGER.md` is the record of exactly that set, since
`work/README.md` requires a closing program's deletion to be recorded
there. So the check is: *for each `keep_out` clause, does it name a
program in the ledger's closed set?* No English ambiguity, and the
reference list already exists and is maintained for another reason.

## What it should say when it fires

Not "delete the clause" — a clause naming a closed program is often
still true as a **record** of how ground moved, and deleting it loses
that. The useful output is *"this clause names CLOSED program X; say
where that ground went"*. Both instances above were repaired that way
rather than struck: the FIX clause now names PROPS and says why.

Warning, not error, for the same reason #2337 gives: a check whose
first act is to red `main` for prose twenty programs wrote gets
softened within the day.

## Related, and not the same row

That the predicate's owner had closed is also why
`is-finite-length-homed-in-the-query-seat` could be taken by FIX at all
— recorded in `work/fix/log.md`, 2026-09-11. This row is about the
board being unable to say so, not about that decision.
