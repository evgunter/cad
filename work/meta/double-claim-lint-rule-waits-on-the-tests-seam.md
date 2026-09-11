---
id: double-claim-lint-rule-waits-on-the-tests-seam
kind: issue
title: The double-claim check is a warning until the */tests/* seam is decided and ten pairs are recorded
status: open
opened: 2026-09-11
---


**Filed by META at the landing of `territory-cannot-see-a-path-two-programs-both-claim`
(2026-09-11), which built the check and could not flip it.**

## What is built

`scripts/work.py lint` measures every open program's `paths` globs
against `git ls-files`, pairs the programs that share a tracked path,
and names each pair whose two `keep_out`s do not both name the other,
with the count of paths it shares. `work/README.md` states the rule it
enforces. It is a **warning**.

## Why it is not the error the plan asked for

`work/meta/plan.md` decided the shape — *"a lint rule that errors on an
unrecorded double claim and passes one recorded in `keep_out`"* — and
that shape is what shipped, minus the erroring. The measurement is the
reason: **most pairs in the tree are unrecorded, and their fix is a
`keep_out` line in another program's `program.md`**, none of which is
META's to edit (one file, one item — and this program's `keep_out` says
so in as many words). An error would red `main` the day it landed for a
change only other programs can make. A check whose first act is to break
every program's CI is not a check, and softening it afterwards would be
worse than not landing it. So it names the rows and leaves the fix to
whoever owns them, which is what a warning is for.

**No count is frozen in this item, deliberately** — `python3
scripts/work.py lint` prints the current reading, and the reading moves
faster than a written figure survives. It moved *while this PR was
open*: at `6ebe47bd` (2026-09-11 04:18Z) the check named **13 pairs, 10
unrecorded, 3 recorded on both sides**; ninety minutes later at `0d90fa9a`,
after merging main, it named **22 pairs, 19 unrecorded, the same 3
recorded**. Nothing about the rule changed. S-TCOST split, S-TINT took
half its territory, and nine new pairs existed before this PR's first
CI failure had finished being diagnosed.

That is the argument for the whole shape of this check, made by
accident: a hand-written census of this population is wrong within
hours, which is why the first version of it (2026-09-05, by hand)
miscounted three of its own figures and why this one prints itself on
every lint run instead of being written down.

## The instance that arrived on day one

**`tcost` + `tint`, 1236 shared paths, recorded on one side only** —
caught by this check within hours of the split that created it, and the
cleanest possible demonstration of what it is for.

S-TCOST split into S-TCOST (cost) and S-TINT (test-suite integrity) on
2026-09-11. Both are open, both claim `crates/*/tests/*` and
`crates/test-utils/*`, and the split was done carefully: **S-TINT's
`keep_out` names S-TCOST at length** — *"cost is S-TCOST's question and
no row here is justified by a cpu-second or a wall-second — a row that
turns out to be a cost lever goes back by `git mv`"*, plus the ci-filter
and slowest-tests ownership, plus S-TCOST's keep-outs inherited
unchanged.

**S-TCOST's `keep_out` does not name S-TINT at all.** Not a defect in
either program — S-TINT opened today and did exactly what the rule asks,
and S-TCOST's clause was written before S-TINT existed. It is precisely
the asymmetry the DOCM/MSOLVE instance named on 2026-09-05: *a one-sided
record is invisible from the side that was there first.* The program
that would be surprised by a lane landing in `crates/*/tests/*` is
S-TCOST, and S-TCOST's own file says nothing about it.

Routed to S-TCOST, not fixed here.

## The one thing left to decide

**The overwhelming bulk of them are the `*/tests/*` family.** S-TCOST's territory is
every crate's `tests/` by design, and code-quality's Track W already
states the seam in prose — *"a track that owns a crate's `src/` does
not otherwise own its `tests/`"*. Either every crate-owning program writes that
sentence into its `keep_out` (and S-TCOST and S-TINT write every name
into theirs), or the check learns the seam once. The second is one
clause and no cross-program edits; the first is a PR per program and a
standing cost for every program opened after it — **and the split that
created S-TINT just doubled that bill in an afternoon**, which is the
strongest argument available for teaching the check instead.

This is a change to the tracker contract that binds every program, so
it is Ev's, not a sequencing call: it decides what a `keep_out` is for.
The recommendation is to teach the check the seam — a pair where one
program's claim on the shared paths is entirely `*/tests/*` and the
other's is the crate's `src/` is the ratified arrangement, not an
unrecorded overlap — and then flip the remaining cases to an error once
`docm`+`msolve`, `docm`+`lib`, `chrome`+`tcost` and `tcost`+`view`
have their second-side clauses.

## Routed, because they are not this program's to fix

Each pair's missing `keep_out` clause belongs to the program that owns
the file. The four non-`tests` pairs, with what is missing:

- **`docm`+`msolve`** (4 paths, `mate.rs` and `mate/*`) — MSOLVE's
  `keep_out` names DOCM at length; **DOCM's does not name MSOLVE**. The
  instance the 2026-09-05 correction turned up, still one-sided six
  days later. DOCM's clause is owed.
- **`docm`+`lib`** (1 path, `docs/RECIPE-DOORS-DESIGN.md`) — DOCM names
  LIB; LIB's clause is owed.
- **`chrome`+`tcost`**, **`tcost`+`view`** and now their `tint` twins —
  CHROME and VIEW each name S-TCOST; S-TCOST's side is owed, and neither
  names S-TINT yet. This is the family the argument above is about, and
  it is the one that grew.
- **`tcost`+`tint`** (1236) — the day-one instance, above.

## A blind spot to close with the flip

`_names` asks whether a `keep_out`'s prose contains the other program's id as
a whole word. Many ids are ordinary English — `view`, `shell`, `fix`, `trim`,
`blend`, `meta`, `curved` — so a clause that happens to use the word reads as
a record and **suppresses the warning**. Measured, not hypothetical: `bool`'s
`keep_out` contains both "ops.rs's `curved` arm" (incidental) and "are CURVED's
since 2026-09-0…" (the record). That pair passes for the right reason; a pair
with only the incidental use would pass for the wrong one.

As a warning this fails in the safe direction — quiet rather than nagging. **As
an error it does not**, because a false record would let an unrecorded overlap
through the gate silently, which is the exact failure this check exists to end.
So the flip wants a `keep_out` that names programs in a field rather than in
prose (a schema change: a `cedes_to`-shaped key, or a convention that a clause
opens with the program id), and that is part of the same decision as the
`*/tests/*` seam — both are questions about what a `keep_out` IS.
