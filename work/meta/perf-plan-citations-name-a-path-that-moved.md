---
id: perf-plan-citations-name-a-path-that-moved
kind: issue
title: Five citations name docs/PERF-PLAN.md, a path that became work/perf/plan.md on 2026-09-03; twenty-seven more name the document, which is alive
status: open
opened: 2026-09-11
refs: [perf-plan-is-cited-by-twenty-nine-files-and-absent-from-tree-and-ledger]
---


**Filed by META at the landing of
`perf-plan-is-cited-by-twenty-nine-files-and-absent-from-tree-and-ledger`
(2026-09-11), whose premise the landing measurement overturned.**

## What is true

`docs/PERF-PLAN.md` was **renamed** to `work/perf/plan.md` in
`4916f90cfc5cd45c0092b9464fd1fed604f93140` (2026-09-03, *"work: migrate
m10, pcurve, verbs, lib, gui, gauth, seat, blend, perf"*, PR #1619)
with **zero content change** — `git show --stat 4916f90c --
docs/PERF-PLAN.md work/perf/plan.md` prints
`docs/PERF-PLAN.md => work/perf/plan.md | 0`. It was never deleted,
nothing was lost, and the move was `scripts/work.py lint`'s own
`docs/*-PLAN.md` rule doing its job. Sweep 4 of `docs/DOC-LEDGER.md`
already recorded the move as a class; that sweep now names this file in
its own row.

## What is left

Measured at `HEAD`, 2026-09-11:

- **Five files cite the dead PATH** `docs/PERF-PLAN.md`:
  `crates/mesh/tests/profile_overrides.rs:12` (S-TCOST's file),
  `docs/PERF-SCAN-2026-08.md:6,1279` (twice), `docs/WORK-TRACKS-2026-09.md:872`,
  `docs/MODEL-AB-LOG.md:4382`, and `work/docm/log.md:302`. Two of those
  five are records rather than pointers — the A/B row disclosing that a
  spec cited it, and DOCM's log entry recording the finding — and a
  record of what a document said is not a citation to repoint.
- **Twenty-seven more name the DOCUMENT, not the path** (`PERF-PLAN.md`
  or `PERF-PLAN` bare), across `crates/topo`, `crates/geom-brep`,
  `crates/bvh`, `crates/editor-core`, `crates/sweep`, `benches/`,
  `docs/DESIGN.md` and `docs/perf-data/*/README.md`. **These are not
  broken.** The document still carries that name — `work/perf/plan.md`
  opens `# PERF-PLAN — the performance work still owed` — so a reader
  grepping the name finds it. They are only broken if PERF renames the
  document to match its path.

## What it wants

1. The live path citations repointed at `work/perf/plan.md`. **This PR
   repoints none of them, and that is not an omission**: every one is
   either another program's file or a record of what a document said at
   the time, and META edits neither. Routed:
   - `crates/mesh/tests/profile_overrides.rs:12` → **S-MESH and
     S-TCOST** (`territory` names both; the doc comment points a reader
     at the discipline).
   - `docs/PERF-SCAN-2026-08.md:6` and `:1279` → **PERF**
     (`docs/PERF-SCAN-2026-08.md` is in its `paths`). `:1279` says the
     scan's corrections "have been annotated into `docs/PERF-PLAN.md`
     itself" — true of the file, at its new path.
   - `docs/WORK-TRACKS-2026-09.md:872` — the 2026-09 cut's record of
     routing this very item to META, restating the claim this landing
     overturned. A historical cut document; left as written, noted here
     so the next reader of it knows the paragraph is superseded.
   - `docs/MODEL-AB-LOG.md:4382` — another program's A/B row disclosing
     that its spec cited the path. META owns that file's rules and
     never edits another program's row.
2. A decision that is **PERF's, not this program's**: whether a
   document keeps a name its path no longer carries. Keeping it costs
   nothing and the 27 mentions keep resolving; changing it makes the 27
   the sweep the closed item thought it was already facing. PERF has no
   orchestrator (its charter: *"no orchestrator and no units"*), so this
   waits on whoever picks that register up, and is recorded here so the
   next reader of those 27 mentions does not re-derive the measurement.

## The lesson worth more than the repoint

The closed item's own headline — *"cited by path from 29 tracked files
and absent from tree and ledger"* — was three claims and **all three
were wrong**: the count was of name mentions, not path citations (5,
not 29); the file was moved, not absent; and the ledger did record the
move, as a class. It was filed by a lane that followed a citation, did
not find the file, and reported what that looked like. `git log
--diff-filter=D` is empty for a renamed file, which is exactly what
makes the wrong conclusion the easy one — so `docs/DOC-LEDGER.md`'s
recovery section now says to run `--all --full-history` before
concluding a document is gone, with this as the worked example.
