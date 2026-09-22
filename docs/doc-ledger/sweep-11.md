# Sweep 11 — 2026-09-11: code-quality leaves the tracker

Sweep SHA: `8851abb6daff4822f5a55c98e940c4c061223953` — the commit immediately before the deletion (the
`main` tip this PR branched from; it is the state in which
`work/code-quality/` is complete and every row in it closed, the 110
live ones having left the same day in the cut below), so every path here
is recoverable at
`git show 8851abb6daff4822f5a55c98e940c4c061223953:work/code-quality/<FILE>`
and `git show 8851abb6daff4822f5a55c98e940c4c061223953:work/issues/<FILE>`.

**code-quality** — *"where a structural finding waits until a program
claims it"* — opened 2026-08-18 as the tracker home of the 2026-08
structural-findings register (`docs/SMELL-SCAN-2026-08.md`, sweep 4) and
its Tracks K–X schedule, and closed 2026-09-11. It is the first program
to close **empty by design rather than by finishing its board**: it was
a holding ground, its charter said a row leaves the moment a program
claims it, and on 2026-09-11 all 110 of its remaining live rows and
`work/issues/`'s were claimed at once by eleven programs opened for them
(`docs/WORK-TRACKS-2026-09.md` addendum 3, PR #2370). What was left the
next day was 32 closed rows, two rule documents, a log, and ten closed
tracks' execution records.

**No exit walk was written, on Ev's direction (in-chat, 2026-09-11:
*"can you delete all the closed items in issues, and the code-quality dir
entirely"*).** The contract's exception — a program closes on a ratified
`docs/<NAME>-EXIT-WALK.md` *or* on Ev's ruling that it needs none — is
what this sweep runs on, and it is recorded here because the absence
would otherwise read as an omission. The three criteria a walk would have
tested are answered by the cut instead: its board is empty (criterion 1),
its successors exist and are named below (criterion 2), and its rules
survive relocation (criterion 3, the section that follows).

### What survived, and where

Two documents were **first moved to `docs/` and then, on Ev's
correction the same day, deleted with everything else** — see the
amendment at the end of this entry, which is the disposition of record.
What survives of them is named there: one sentence in the reviewer
brief, and the rules each of the eleven programs actually uses, inlined
into that program's own `plan.md`.

### What was deleted

- **`work/code-quality/` whole** (46 tracked files at the sweep SHA, less
  the two moved above): `program.md`, `log.md`, the **32 closed rows**
  (`C13`, `C14`, `D106`, `D202`, `D204`, `D205`, `D207`, `D208`, `D209`,
  `D224`, `D288`, `D289`, `D320`, `D321`, `D323`, `D324`, `D402`, `D64`,
  `D68`, `S22-row-1`, `S26`, `S290`, `chart-region-lane-contract`,
  `corner-config-tag-all-concave-trihedron`,
  `demo-tour-dead-constant-breaks-compile`,
  `demo-typed-refusal-exit-convention`,
  `directory-prefix-skips-have-no-subject-check`,
  `flat-pack-gap-rationale-invented-mechanism`,
  `probe-cutaway-comment-claims-shipped-box`, `scaled-square`,
  `smell-scan-2026-08-findings-register`,
  `tour-suite-never-runs-wall-probes`), and **`logs/`** — the ten closed
  tracks' execution records (`SMELL-C`, `SMELL-E`, `SMELL-F`, `SMELL-G`,
  `SMELL-H`, `SMELL-I`, `SMELL-KPW`, `SMELL-T`, `SMELL-UV` and
  `migration-census-2026-09-03.md`), about 11,000 lines.
- **The seven closed items in `work/issues/`**
  (`actions-budget-denies-job-starts`,
  `bounds-census-roster-lists-anchor-span-twice-with-two-dispositions`,
  `fillet-specs-require-a-narrowing-ci-config`,
  `freecad-lane-reports-no-drift-on-a-cell-whose-geometry-changed`,
  `m10-5-e2e-channel-slider-reds-at-eps-1e-6`,
  `render-lanes-checkout-merge-ref-vanishes`,
  `reviewer-pair-rebuilds-two-trees-two-rules`), on the same direction
  and the same rule: `work/` tracks work still to be done.
  `work/issues/README.md` stays and the directory keeps its purpose.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `code-quality` | Code quality — where a structural finding waits until a program claims it | 2026-09-11 | this row and the amendment below; `docs/WORK-TRACKS-2026-09.md` addendum 3 for where its 110 live rows went; the merged PRs of its closed tracks, named in the logs recoverable at the SHA above |

### The eighteen `refs:` this sweep rewrote

Deleting the closed rows would have broken `scripts/work.py`'s
*references resolve* rule on sixteen live rows across ten programs. They
were rewritten first, in GATES' and METER's form (sweeps 7 and 10): the
dying id is replaced by the number of the PR that closed it — ints are PR
numbers and lint does not check them — and a
`## Refs at code-quality's sweep (2026-09-11)` section on each citing row
says what changed and why.

| dying id | now cited as | citing row(s) |
| --- | --- | --- |
| `D64` | 1643 | `work/comb/L4.md`, `work/door/viewer-grid-pitch-nonfinite-fallback.md` |
| `D205` | 1642 | `work/tint/D386.md`, `work/topo/D261.md` |
| `D204` | 1642 | `work/instr/k-lint-csv-header-unpinned-against-five-producers.md` |
| `D320`, `D321` | 1782 (one entry, not two — both closed in it) | `work/wire/profile-has-no-scalar-lift-door.md`; `D320` alone in `work/scalar/sweep-test-rebuilds-validated-net-for-v-reversal.md` |
| `D323`, `D324` | 1783 (one entry, same reason) | `work/comb/L5.md` |
| `S26` | 1366 | `work/props/purchasable-area-tightness-valve.md` |
| `C13` | `epsilon-has-no-type-of-its-own` — the live row whose §Closed IS the ruling that closed it | `work/scalar/D283.md` |

**Five references were dropped rather than re-aimed**, and the reason is
the one the METER sweep did not meet: the row closed with **no PR to be
cited by**. Three closed on a ruling with no implementation (`C13` and
`C14`, whose rulings are recorded in the two live `work/exch/` rows that
cited them — so those rows were pointing at their own record;
`fillet-specs-require-a-narrowing-ci-config`, closed BY
`work/ciw/delete-config-trailer.md`, which cited it). Two recorded no
closing PR at all (`D68`, cited by `work/guard/D212.md` and
`work/guard/G4.md`; `D289`, cited by
`work/tint/decoration-seam-header-names-no-pin-for-enclose.md` — the 1533
in `D289`'s own `refs:` was CERT-M1, where it was *filed*, not where it
landed). Each of those five citing rows carries the dropped id, its
title, and the SHA it is recoverable at, in its
`## Refs at code-quality's sweep` section; nothing was silently removed.

`work.py lint` before the sweep: 0 problems, 23 warnings. After: 0
problems, 22 warnings — the one that went was the duplicate `github: 1607`
claim, whose second claimant was a closed `work/issues/` row.

### A note on inbound references, again

**102 files cite `work/code-quality/…` in prose, 217 times**, and they
survive unrewritten, which is what *A note on inbound references* in
`docs/DOC-LEDGER.md` is for and what GATES' and METER's sweeps did with their own. Two classes
were fixed, because both are live contract text rather than a citation:

- **The eleven programs of the 2026-09-11 cut** had their `plan.md`
  charters re-aimed off `work/code-quality/plan.md`; see the amendment
  below for where each rule they cited now lives.
- **`work/README.md`** (META's file, edited here by announced seam
  because this sweep is what makes it false, in the same commit): the
  clause saying `work/code-quality/` is where a finding waits for a
  claim, the `process-observations.md` row of the layout, the
  "code-quality only" gloss on `track:` and on `blocks:`. **A finding
  now goes straight onto the slate of the program whose ground it lands
  on**, and `work/issues/` is the last resort it always was.
- `docs/DESIGN.md`'s two citations of `work/code-quality/S14.md` and
  `S65.md` were already stale from the cut and now name `work/pipe/` and
  `work/pred/`.

One consequence is named rather than left to be found: **`d321-row-number-reissued`
(CITE's) and `S176` (CITE's) both ask for edits inside `SMELL-T-LOG.md`,
`SMELL-KPW-LOG.md` and `SMELL-G-LOG.md`**, which this sweep archived.
`S176`'s live half is its convention and is unaffected; `d321` is
overtaken on both halves, which the amendment below explains. CITE's
plan recorded it and left the tree at sweep 12; the row itself is
`work/meta/d321-row-number-reissued.md`.


### Amendment (2026-09-11, same day): the two relocated documents were deleted too

This sweep first moved `plan.md` and `process-observations.md` into
`docs/` on the argument that `plan.md` was cited 60 times and was
therefore load-bearing. **Ev rejected that** (in-chat: *"where is it
cited? we don't want to mint any new rows because we're using the in
repo issue tracker now, not the one big doc that descends from"*), and
the count did not survive being read:

- **48 of the 60 are one of three provenance sentences** repeated
  verbatim in `## Claimed by` sections — 17 rows in `work/tint/` and 14
  in `work/topo/` — and each of those sentences says, in its own second
  clause, that the content is **restated in the claiming program's own
  plan**. They are history, not lookups.
- 9 were this ledger, 3 `work/README.md`, 1 `docs/WORK-TRACKS-2026-09.md`.
- **Ten were live**, all of them in the eleven new programs' `plan.md`
  charters, and every one cited a single self-contained rule.

**The block ledger was the other half of the argument and it is simply
retired.** Ids in this tracker come from an item's name; the per-track
`D<N>`/`S<N>` blocks were the numbering of the 2026-08 register that the
tracker replaced, and keeping a document alive so that a future row
could be minted from a block would have preserved the scheme this
project stopped using. **No new row is minted from a block.** The rows
that carry such an id keep it — ids are stable for life — and nothing
allocates another.

**So both documents are deleted**, recoverable at this entry's sweep
SHA, and what was live in them went to where it is used:

| what | where it went |
| --- | --- |
| *The fix mints a fresh instance of the defect it closes; naming the trap in your own PR body does not prevent it; only a reader who did not write the fix has ever caught it* | **`docs/prompts/reviewer-style-lane.md` §1**, as a bullet in the stance — the standing brief every reviewer reads, which is what the rule is for. It was the only sentence in either document with no surviving home |
| The ordering, partition, seam and fence rules the eleven charters cited (ten citations) | **inlined into the citing `plan.md`**, one to three sentences each, so each program states the rule it runs on instead of pointing at a document about a program that no longer exists |
| *Ask what a reported sweep's pattern could not match* (`C15`) | nowhere — it was **already** `docs/prompts/reviewer-style-lane.md` §Q1. `work/door/viewer-grid-pitch-nonfinite-fallback.md` now cites the brief instead of `§C` |
| The `C2`/`H17` and `C21` labels on `work/comb/S37.md` and `work/door/run-on-whitespace-in-message-literals.md` | **folded into those rows** as the populations and dispositions they stood for; the labels themselves were a process-observation number and a track row id colliding, which is `C-namespace`'s point |
| The W `test-utils` ceiling seam, and P's three sub-lanes | nowhere — already restated, the first in all 17 `work/tint/` rows that cite it and the second in `work/topo/plan.md`, both by the same sentence that cited the plan |
| `C1`–`C27` otherwise | the archive. Of 27 process observations, **three were cited by a live row** and all three are handled above; the rest are a closed program's retrospective and are recoverable at the sweep SHA |

One observation is worth naming here rather than leaving at a SHA,
because it records a decision rather than a finding: **`C21` carries
Ev's ruling of 2026-08-20** that the *skip-reading-as-a-pass* class
stays un-rolled-up — a class rule was drafted around giving skips
*floors*, which concedes the skip, and the prior question is whether the
test should be skipping at all. A future scan re-opens that question
rather than re-proposing the floors.

`d321-row-number-reissued` (CITE's) is **overtaken on both halves** by
this amendment: its retired-id rule has no number ledger to live in now
that the blocks are retired, and its citation disambiguation was inside
two `SMELL-*-LOG.md` files this sweep archived. The row itself says so
and names the one thing still worth doing — a `work.py` check that an id
is never reissued — as META's; it was re-homed to
`work/meta/d321-row-number-reissued.md` when CITE closed at sweep 12.
