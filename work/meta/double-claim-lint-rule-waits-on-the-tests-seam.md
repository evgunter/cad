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
reason. At `HEAD` on 2026-09-11 the check names **thirteen pairs, of
which ten are unrecorded**:

| pair | paths | record |
| --- | --- | --- |
| `exch`+`tcost` | 179 | neither |
| `chrome`+`tcost` | 54 | one-sided |
| `tcost`+`view` | 54 | one-sided |
| `mesh`+`tcost` | 56 | neither |
| `bool`+`tcost` | 50 | neither |
| `m10`+`tcost` | 42 | neither |
| `lib`+`tcost` | 38 | neither |
| `shell`+`tcost` | 3 | neither |
| `docm`+`msolve` | 4 | one-sided |
| `docm`+`lib` | 1 | one-sided |

(The three that pass: `chrome`+`view`, `bool`+`curved`, `m10`+`props`.)

An error would red `main` the day it landed, on ten pairs whose fix is
a `keep_out` line in **twenty programs' `program.md` files**, none of
which is META's to edit (one file, one item — and this program's
`keep_out` says so in as many words). A check whose first act is to
break every program's CI for a change only other programs can make is
not a check, and softening it afterwards would be worse than not
landing it. So it names the rows and leaves the fix to whoever owns
them, which is what a warning is for.

## The one thing left to decide

**Nine of the ten are the `*/tests/*` family.** S-TCOST's territory is
every crate's `tests/` by design, and code-quality's Track W already
states the seam in prose — *"a track that owns a crate's `src/` does
not otherwise own its `tests/`"*. Either nine programs write that
sentence into their `keep_out` (and S-TCOST writes nine names into
its), or the check learns the seam once. The second is one clause and
no cross-program edits; the first is nine PRs and a standing cost for
every program opened after it.

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
- **`chrome`+`tcost`** (54) and **`tcost`+`view`** (54) — CHROME and
  VIEW each name S-TCOST; S-TCOST's side is owed, and is the same
  sentence nine times, which is the argument above.

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
