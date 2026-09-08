---
id: directory-prefix-skips-have-no-subject-check
kind: issue
title: the directory-prefix exemptions have no subject check, and the helper the file skips got cannot give them one
status: review
opened: 2026-09-08
refs: [whole-file-skips-do-not-check-their-subject]
branch: gates/dir-prefix-subject
---

## Finding

`whole-file-skips-do-not-check-their-subject` closed the class for
every exemption whose unit is one FILE: `gate_require_homes` reads the
same list the filter is built from and reds on a path the tree does not
have. Its sweep — `gate_grep -vE '^crates…'` over the directory —
turned up the exemptions that unit could not close, and they are all in
one file:

  * `scripts/gates/witness-not-ambient.sh:113` — `^crates/pncad/src/`
  * `scripts/gates/witness-not-ambient.sh:114` — `^crates/pncad-py/src/py/`
  * `scripts/gates/witness-not-ambient.sh:115` — `^crates/[^/]+/src/bin/`

Two directory prefixes and a path CLASS. None names one file, so none
has a path `gate_require_homes` could prove exists, and the gate's own
header says so where it holds `HOME_FILE`.

**The class is the same one, with the same live route.** A prefix whose
directory is renamed away exempts nothing while it stands, and the day
something new is written at the old path it is exempt without argument
— D103's class, which is why the file skips are a red and not an
abstention (`lib.sh`, `gate_exact_skip_subject`). `crates/pncad/src/`
is the curated door and `crates/pncad-py/src/py/` is the pyo3 boundary;
both are ordinary directories that a crate rename or a module
reorganisation moves.

## Why it is not simply the same fix

The third entry is why this needs a decision rather than a loop.
`^crates/[^/]+/src/bin/` is cargo's convention for "this file is a
program" and not a claim that any tree has one: the exemption was
written when `crates/viewer` grew the repo's first `src/bin` target,
and a tree with no bin target anywhere is not a defect. A check that
demanded a resident would red on a correct tree, so the first two and
the third do not want the same answer:

  * a DIRECTORY that must exist (`[ -d ]` over the prefix, the file
    check's shape one level up), for the two that name real places;
  * a path CLASS, which has nothing to prove and wants the argument
    written down instead — or wants converting into whatever it is
    really exempting.

Whether the second is a check at all, or a note at the exclusion, is
the ruling this row is asking for.

## Where it stops

`scripts/gates/` holds no other prefix exclusion — the sweep pattern
above returns these three and nothing else (the other `gate_grep -vE
'^…'` hits are comment strips in `gate-roster.sh` and
`probe-suite-census.sh`, which exempt no path). Accurate as of
`ce640a6e8`.

## Landed

**The two NAMED prefixes now prove their subject, and the row's premise
was wrong about what that subject is.** The row reads "None names one
file, so none has a path `gate_require_homes` could prove exists." Each
of the two DOES: a directory prefix's subject is the file the directory
is ABOUT, not the directory itself.

  * `^crates/pncad/src/` exempts a CRATE'S curated door, and what makes
    that path a door rather than a name a rename left behind is
    `crates/pncad/src/lib.rs`, cargo's own crate root.
  * `^crates/pncad-py/src/py/` exempts a MODULE'S FFI boundary, and what
    makes that path the boundary is `crates/pncad-py/src/py/mod.rs`,
    rustc's own module root.

Both are handed to `gate_require_homes` unchanged, one call per subject,
each with its own subject sentence. The helper already answers exactly
the question a prefix needs answered — the path is a file, and a file
this scan actually READS — and proving the root is strictly stronger
than proving the directory: a root that is in the scan set is a scanned
resident under the prefix, so membership comes with it.

**Why not `[ -d ]`, and why not "some scanned file lives under the
prefix".** Both are satisfied by the exempted file ITSELF. That is the
live route the reviewer of PR 2156 demonstrated: remove
`crates/pncad/src`, write `crates/pncad/src/new.rs` minting
`Tol::witness()`, and under either directory-shaped check the new file
restores the very directory whose existence is supposed to license it —
the prefix ratifies the file and is then ratified by it. D103's circle
with a check drawn around it is not a check on D103. Anchored at the
root, that scenario reds, and it is planted as its own fixture
(`plant_door_renamed_away_then_rewritten`) rather than left to the
home-gone case, because it is the COMPOSITION of the two halves that is
the route.

**Where the check lives: nowhere new.** Re-running the row's sweep at
this branch's base (`gate_grep -vE '^…'` over `scripts/gates/*.sh`)
still returns these three and the two comment strips, so a directory
mode would be a `lib.sh` generality with one caller; and once the
subject is a file there is no directory mode to write. No new refusal
text was added — both diagnoses are `gate_home_gone_refusal`'s and
`gate_require_homes`'s own.

**One `lib.sh` change was needed, in the home-check section only**
(`gate_plant_home_unscanned`, §"THE WHOLE-FILE SKIP'S OWN CASES"). The
out-of-scan case mounts a home test-only by writing `#[cfg(test)] mod
NAME;` into the home's own directory's `mod.rs`. For a home that IS a
`mod.rs`, that overwrote the home with `mod mod;`, which resolves onto
its own declarer — the resolver drops such a declaration as naming no
other file, so the home stayed in the scan and the case passed a gate it
was written to red. A `mod.rs` home is its DIRECTORY'S module, so the
declaration that mounts it names the directory from one level up. No
existing caller had a `mod.rs` home, so nothing else moves.

**The clean fixture now plants all three subjects, each minting the
witness its exemption covers**, so every skip is live in every fixture:
back out either prefix skip and the clean case reds.

**Mutation table.**

| mutation | result |
| --- | --- |
| `gate_require_homes "$DOOR_SUBJECT" "$DOOR_HOME"` removed | red: `plant_door_renamed_away_then_rewritten` PASSED on a planted violation |
| ditto, with that case also removed | red: `gate_plant_home_gone crates/pncad/src/lib.rs` PASSED |
| `gate_require_homes "$FFI_SUBJECT" "$FFI_HOME"` removed | red: `gate_plant_home_gone crates/pncad-py/src/py/mod.rs` PASSED |
| `lib.sh`'s `mod.rs` branch removed | red: `gate_plant_home_unscanned crates/pncad-py/src/py/mod.rs` PASSED |
| `gate_grep -vE '^crates/pncad/src/'` removed | red: the gate FAILED on a clean fixture |
| `gate_grep -vE '^crates/pncad-py/src/py/'` removed | red: the gate FAILED on a clean fixture |

Live output is byte-identical to the merge base, stdout and stderr
(`cmp`): `witness-not-ambient OK: no kernel library code mints a
tolerance witness (406 source files scanned)`.

## What is still open: the third exemption

`^crates/[^/]+/src/bin/` is deliberately NOT in the list, and whether it
should be is a ruling, not a lane's call. It names a cargo CONVENTION —
anything under `src/bin/` is a bin target by construction — rather than
a place, so it has no root to anchor. The two options:

  * **Keep it unchecked as a convention-class exemption**, with the
    reasoning stated at the site (this is what landed). Cost: one
    exemption in this directory stands without a subject check, so the
    class the row opened is closed for named places and open for
    convention classes — a distinction a future reader has to be told
    about, and the argument for it lives in a comment rather than in a
    check.
  * **Require at least one `src/bin/` resident across the workspace.**
    Cost: a correct tree with no bin target anywhere reds. Today
    `crates/viewer/src/bin` is the only one in the repo, so retiring
    that one binary — an ordinary change — turns the gate red until
    someone deletes the exemption too. That is the check working as
    designed and it is also a red on a correct tree.

Asked of Ev in the `[ev]` PR named in `blocked_on`. This row stays open
on this half.
