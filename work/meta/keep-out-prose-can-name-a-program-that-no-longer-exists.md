---
id: keep-out-prose-can-name-a-program-that-no-longer-exists
kind: issue
title: keep_out prose can name a closed program, and lint cannot see it — two instances found in one day
status: open
opened: 2026-09-11
priority: P3
cost: E
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

## The class measured, and where its boundary actually falls (2026-09-16)

INSTR unit 0's sweep (PR 2735) measured the wider class this row is the
load-bearing corner of. Across the tree, **28 distinct `work/<program>/`
prefixes no longer resolve** — `ab`, `cert`, `code-quality`, `fillet`,
`gui`, `m10`, `meter`, `verbs` and twenty more — and **almost all of
them sit in `Moved from …` provenance lines**, past tense, inside dated
disclosure blocks.

That measurement sharpens this row rather than widening it, and the
distinction is worth stating because the next reader will meet the 28
before they meet the two:

- **A dangling prefix in a dated provenance line is the tracker working
  as designed.** `work/README.md` requires a closing program's directory
  to be deleted and the deletion recorded in `docs/DOC-LEDGER.md`, and
  ids are stable — so `work/meter/C15.md` names a row that still exists
  as `C15`, reachable by id. Repairing those 28 is churn against
  records, and `baseline_census.rs`'s own doctrine (*"a dated record may
  keep the FIGURES it reported"*) is the argument for leaving the
  sentence alone.
- **A `keep_out` clause naming a closed program is not that**, and the
  two instances above are why: each told a lane that ground belonged to
  a program that could have adjudicated a crossing, and neither program
  existed. `work/fix/is-finite-length-homed-in-the-query-seat` sat
  parked on an owner that could never answer. The clause is read
  FORWARD, as a live statement about who owns what, so a dangling name
  in one is a false claim rather than a stale record.

- **And a third bucket exists that is neither**, which the first
  version of this section missed and INSTR unit 0's style review
  caught. **Live prose in code doc** that instructs a reader to go
  somewhere is read forward exactly as a `keep_out` clause is, and is
  not a record of anything. Measured in `tools/` alone on 2026-09-16:
  **13 dangling `work/meter/…` pointers across four files** —
  `tools/tess-lint/tests/baseline_census.rs` (6),
  `tools/k-lint/tests/predicate_roster.rs` (5, one of them **inside an
  assertion string**, so a failing test prints a path to a reader and
  sends them nowhere), `tools/k-lint/src/lib.rs` (1) and
  `tools/tess-meter/src/lib.rs` (1). Those are INSTR's fence and INSTR
  is filing them; they are named here because they are what makes the
  rule below a rule rather than a rule about `keep_out`.

**So the dividing line is not whether the name resolves, and not
whether the sentence is a `keep_out` clause.** It is whether the
sentence is read as a **record of what happened** or as a **claim about
what is true now** — and live code doc is the second even though it is
neither of the two shapes this row was opened from. `keep_out` stays
the cheapest place to detect it, for the reason given above (the
ledger's closed set is a maintained reference list and no English
ambiguity arises), but the detector's scope is a matter of cost, not of
where the defect lives. A general "no tracker prose may name a closed
program" check would fire 28 times on correct text and be softened
within the day, which is the failure mode this row already names for
#2337's `_names` check — the narrowing to avoid that should be to
FORWARD-READ prose, not to `keep_out` syntax.

**Nobody has swept `crates/*/src` for this.** The 28 figure is
tracker-and-tools-wide; the kernel's own source has not been measured
against `ls work/`.

**One residue the measurement turned up and this row does not cover.**
`work/instr/tess-budget-doc-identity-column-list.md` cites `D201`, which
exists nowhere as a row — not a moved id but an id with no referent at
all. It is past tense in a dated block, so it is a record by the test
above and INSTR left it; noting it here because "the id resolves
elsewhere" is doing work in the argument above, and this is the case
where it does not.
