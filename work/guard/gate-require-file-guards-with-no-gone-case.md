---
id: gate-require-file-guards-with-no-gone-case
kind: issue
title: viewer-module-kinds.sh's two gate_require_file guards can both be deleted with its selftest green
status: open
opened: 2026-09-15
priority: P3
cost: D
---


Filed by PORT (2026-09-15) from `msrv-floor-is-declared-and-never-compiled`'s
style review, which found the same hole in PORT's own new gate and asked for
the sweep. PORT fixed its own; this row is what the sweep turned up.

`lib.sh`'s `gate_require_file` exists so that a gate whose subject is a NAMED
file cannot go green forever after that file is renamed or deleted — its
message is *"the gate's subject is gone, so it cannot decide; move the gate
with the file or retire it deliberately"*. It is a guard, and `lib.sh` says
in as many words, repeatedly, that **a guard never shown to fire is not a
guard**.

`scripts/gates/viewer-module-kinds.sh` calls it twice, at `:664` and `:665`
(`$README` and `$MANIFEST`), and its `--selftest` plants **neither** file's
disappearance. Proved by mutation rather than by reading:

| mutation | `--selftest` |
| --- | --- |
| `:664` `gate_require_file "$README"` replaced by `:` | **green** |
| `:665` `gate_require_file "$MANIFEST"` replaced by `:` | **green** |

Each guard can be deleted outright with the self-test still passing, which
is this directory's own definition of an unheld claim. The gate does plant
`plant_src_tree_gone`, but that aims at the hand-written `$SRC` check three
lines below, not at either `gate_require_file`.

**The sibling is already right, which is what makes this a defect and not a
convention question.** `scripts/gates/viewer-vocab-declared-once.sh` plants
`plant_readme_gone` and `plant_src_gone` and asserts `"does not exist under"`
on both (`:1548-1549`), and `bit-identity-debug-only.sh` plants one case per
named subject with a comment saying why. The convention is one case per
named subject; `viewer-module-kinds.sh` is the one member of the directory
that does not keep it.

## The fix

Two planters and two `gate_selftest_case` lines, modelled on
`viewer-vocab-declared-once.sh`'s pair. Note that the second one is not
automatic: `gate_require_file` ends the gate at the FIRST missing subject,
so a case that removes the README never reaches the manifest guard and each
needs its own fixture.

## Scope — what this sweep could not match

The sweep was `grep -n 'gate_require_file' scripts/gates/*.sh`, which finds
CALL SITES and nothing else. Its blind spots, stated because a negative
result without them is a claim:

- It cannot see a **hand-written** subject-gone check (`viewer-module-kinds.sh`'s
  own `$SRC` arm at `:667`, `signed-zero-one-home.sh`'s discussion at
  `:204-212`); whether those have fixtures is a different sweep.
- It cannot tell a covered guard from an uncovered one at all — that took a
  mutation per site, and only the three live call sites outside PORT's own
  gate were mutated: `viewer-module-kinds.sh` `:664`, `:665`, and
  `viewer-vocab-declared-once.sh` `:829`. The last is covered.
- `bit-identity-debug-only.sh:454` calls it inside a loop over a path list;
  it was read (it has a comment claiming coverage) but **not** mutated,
  because its self-test is the longest in the directory and the mutation
  budget went to the two that read as uncovered. It is the one site this
  row does not settle.
