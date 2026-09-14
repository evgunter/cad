---
id: query-rs-fence-names-a-closed-program
kind: issue
title: TOPO's keep_out sends a query.rs seam to SEAT's board, which left the tracker on 2026-09-06
status: open
opened: 2026-09-14
---


## What

`work/topo/program.md`'s `keep_out` says "topo/src/query.rs and
flush.rs are SEAT's - face-kind-read-has-two-homes reaches query.rs for
ONE door and is a seam to be announced on SEAT's board before any edit
lands there", and `work/topo/log.md` (the opening entry) says the same.
SEAT left the tracker on 2026-09-06 — `docs/DOC-LEDGER.md` sweep 8,
"SEAT leaves the tracker", its exit walk ratified and its directory
deleted — so there is no board to announce on and no owner to agree
with. `python3 scripts/work.py territory --base origin/main` confirms
it from the other side: on a branch touching `crates/topo/src/query.rs`
it names no owner for that path, because no open program's `paths`
covers it.

Found by `edge-carrier-kind-has-no-readback-door`, which edits
`query.rs` for exactly the one door the clause anticipated and had
nowhere to announce it (the PR says so and announces the seams that do
have owners, WIRE's and LIB's).

## Why it is not just stale prose

The clause is a fence instruction, so a lane that reads it either stops
and looks for a board that does not exist, or edits and cannot tell
whether it has crossed someone. Two questions for the orchestrator,
neither this unit's:

- Does `query.rs` (and `flush.rs`) join TOPO's enumerated `paths` now
  that the program that held them is closed? They are the query and
  flush seats of the crate this program otherwise owns, and the
  remaining-35-files clause in `plan.md` reads them as a separate
  category from "unowned and not finished".
- If they stay unowned, the `keep_out` sentence and `log.md`'s opening
  entry should say "unowned since SEAT closed (DOC-LEDGER sweep 8)"
  rather than naming a program, so the next lane knows the announcement
  has no addressee rather than looking for one.

Same for `face-kind-read-has-two-homes`'s closed-record prose, which is
history and correct as written — it records what was true when it
landed and needs no edit.
