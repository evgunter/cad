---
id: directory-prefix-skips-have-no-subject-check
kind: issue
title: the directory-prefix exemptions have no subject check, and the helper the file skips got cannot give them one
status: review
opened: 2026-09-08
refs: [whole-file-skips-do-not-check-their-subject]
branch: gates/dir-prefix-subject
pr: 2170
blocked_on: [2171]
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
this scan actually READS.

**Anchoring is stronger than the directory test only while the root
lies UNDER the prefix it licenses**, and that condition is not free.
Under it a root in the scan set is also a scanned resident of the
prefix, so membership comes with it. `py/mod.rs` satisfies it; rustc's
other spelling of the same module, `py.rs`, does not — it sits in
`src/`, one level above the module it names. So the gate holds the
DIRECTORY once and builds the root from it (`FFI_HOME=$FFI_DIR/mod.rs`),
which makes the condition structural. Written the other way round — the
prefix derived from the root — a re-anchoring at `py.rs` would widen
the exemption from `src/py/` to `src/` in silence; measured, and it is
why the pair is written directory-first. A reorganisation to `py.rs`
moves the prefix too, and the pair is re-argued rather than retyped.

**Why not `[ -d ]`, and why not "some scanned file lives under the
prefix".** Both are satisfied by the exempted file ITSELF. That is the
VACATED-PATH route the reviewer of PR 2156 demonstrated: remove
`crates/pncad/src`, write `crates/pncad/src/new.rs` minting
`Tol::witness()`, and under either directory-shaped check the new file
restores the very directory whose existence is supposed to license it —
the exemption conjures the resident that licenses it. Anchored at the
root, that scenario reds. It is planted as a fixture
(`plant_door_renamed_away_then_rewritten`) as a record of the review's
scenario; the refusal is terminal before the scan, so it reds for the
same reason the home-gone case does and what it adds is the minting
file the directory test would have been satisfied by.

**This closes the vacated-path route only.** The other D103 route — a
directory-granular exemption whose argument is not directory-wide — is
untouched, and the door prefix is where it would bite: measured,
`^crates/pncad/src/` covers eleven scanned files and exempts one live
minting site (`crates/pncad/src/tolerance.rs:92`). It is NOT narrowed
here, because the exemption is crate-granular by its own argument — the
façade crate is where a user's tolerance becomes a witness, so any file
in it can be the place that happens. The pyo3 prefix's reachability
argument really is directory-wide. Both are said at the site: this check
says the place is still there, not that the place is the right size.

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
declaration that mounts it names the directory from one level up.

**One existing caller has a `mod.rs` home**: `evalscalar-allowlist.sh:42`
(`crates/editor-core/src/eval/mod.rs`), in a gate that does NOT narrow,
so its case is `gate_selftest_passes` and it passed before for the wrong
reason — the home was overwritten and stayed in the scan, so nothing was
mounted. It now mounts for real (`gate_test_only_mounts` resolves the
planted declarer to `crates/editor-core/src/eval/`) and the gate passes
because it scans every source, which is what the case says.

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
| the `$door_prefix` skip removed from the filter | red: the gate FAILED on a clean fixture |
| the `$ffi_prefix` skip removed from the filter | red: the gate FAILED on a clean fixture |
| `DOOR_SUBJECT` and `FFI_SUBJECT` swapped at their `gate_require_homes` call sites | red: fired on `gate_plant_home_gone crates/pncad/src/lib.rs` with an unexpected message |
| `DOOR_DIR` and `FFI_DIR` swapped | red live and in the self-test: `crates/pncad-py/src/py/lib.rs` is not a file |

Live output is byte-identical to the merge base, stdout and stderr
(`cmp`): `witness-not-ambient OK: no kernel library code mints a
tolerance witness (406 source files scanned)`.

## What is still open: the third exemption

`^crates/[^/]+/src/bin/` is not in the list, and whether it should be is
a ruling, not a lane's call. It names a cargo CONVENTION — anything
under `src/bin/` is a bin target by construction — so unlike the two
named prefixes it does not name one place with one root. Three shapes
the check could take, one cost each:

  * **No check; the reasoning stated at the site.** Cost: one exemption
    in this directory stands with nothing behind it, and the class this
    row opened closes for named places while staying open for convention
    classes — a distinction a future reader is told about in a comment
    rather than shown by a check.
  * **At least one `src/bin/` resident across the workspace.** Cost: a
    correct tree with no bin target anywhere reds. `crates/viewer/src/bin`
    is the repo's only one, so retiring that binary — an ordinary change
    — reds the gate until the exemption is deleted too.
  * **Cargo's own evidence: the `[[bin]]` targets and `src/bin/*.rs`
    files the workspace declares, or the hitting file's own crate root.**
    Cost: the gate grows a manifest reader (or a per-hit root lookup) and
    a second source of truth to keep in step with the scan, where the
    other two are one line each.

**The question:** does a convention-class exemption — one matching by a
construction rather than naming a place — owe the subject check the rest
of this class now carries, and if so which of the three shapes above is
its subject?

Asked of Ev in the `[ev]` PR named in `blocked_on`. This row stays open
on this half.

## The fix pass

The style review of PR 2170 asked for eight further changes; all are in.
The two that changed behaviour rather than prose:

  * **The prefix skips are BUILT from the held directories** rather than
    spelled a second time as raw EREs (`gate_licensed_prefix`, over
    `DOOR_DIR` / `FFI_DIR`), so the filter, the subject sentence, the
    root and the fixtures all read one spelling of each directory.
    Swapping the two directories now reds live and in the self-test,
    where before the swap was caught only by a fixture's want string.
  * **`gate_selftest_homes` takes `--subject`**, and the home-gone case
    then requires the diagnosis to carry that sentence as well as the
    path (`gate_selftest_case --also`). Swapping the two subjects at
    their `gate_require_homes` call sites was green before this and reds
    now. The flag is optional, so the six other callers are unchanged
    and can adopt it; nothing outside this gate's fence was edited.

Also stated rather than checked, at the sites: for a crate-root home the
out-of-scan case exercises the resolver's basename rule and not a shape
a tree could take, since no crate root can be mounted test-only at all
(what it proves there is that the gate reds whenever a home leaves the
scan, which for a crate root is reachable by a symlink or a narrowing);
and the unchecked third prefix's site now points at this row and at the
`[ev]` PR.
