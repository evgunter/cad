---
id: directory-prefix-skips-have-no-subject-check
kind: issue
title: the directory-prefix exemptions have no subject check, and the helper the file skips got cannot give them one
status: open
opened: 2026-09-08
refs: [whole-file-skips-do-not-check-their-subject]
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
