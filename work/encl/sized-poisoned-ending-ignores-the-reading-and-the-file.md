---
id: sized-poisoned-ending-ignores-the-reading-and-the-file
kind: issue
title: geom-core: a sized decision's poisoned-margin ending says 'kernel bug' at every door and keeps a lever a NaN cannot follow
status: closed
closed: 2026-10-10
branch: encl/poisoned-sized-ending
opened: 2026-10-09
priority: P3
cost: M
pr: 4475
---



(Filed by the ENCL orchestrator from the review of PR 4461. Pre-existing.)

## What

A sized decision on a poisoned margin (`MarginDiag::INVALID`, NaN) ends through `MarginDiag::sized_recourse`'s unreadable arm (`crates/geom-core/src/predicate.rs` ~1187). That ending is the decision's lever plus `UNREADABLE_MARGIN_NOTE`, "an unreadable or collapsed margin may indicate a kernel bug worth reporting". Two problems:

1. **The ending ignores the reading.** At rest or at the import door, a body may have come from a file, and the unread margin may then mean a damaged file. The note still says only "kernel bug". `Unsized` ends a poisoned arm in `defect_ending(reading)` instead, since PR 4453, which names "a kernel defect or a damaged file" at rest.
2. **The lever may not be followable on a NaN.** For example, `SHELL_ROLE`'s "thicken or remove the degenerate geometry": "remove" can be followed, "thicken" cannot. PR 4461's sort now carries this ending for a poisoned shell role.

## Repair shape

Make the unreadable arm of the sized ending reading-aware, so it names the file at rest and at the import door as `defect_ending` does. Decide whether a poisoned margin keeps the lever at all. Give the poison→note rule one home: today it is decided at `not_yet`, `LeverOnly::recourse`, validate `unnamed` and `MarginDiag::sized_recourse` (see `too-close-to-call-remainder`'s bullet (c)). That likely folds both rows' (c) into one unit. Texts move only on poisoned sized arms.

## Closed

2026-10-10. PR 4475 merged at `61977302ae` after a full review (fix pass), the fix pass and a delta review (verdict: merge); hosted CI was green.

**Rule.** A poisoned margin keeps its decision's own ending, the lever or not-yet, and drops only the tolerance arm. D4 ¶1 (i) carries this rule alone. The note depends on the door that reads the margin:
- at a build: "an unreadable margin may indicate a kernel bug worth reporting";
- at rest or at the import door: "an unreadable margin may indicate a kernel or file defect worth reporting".

"Or collapsed" is dropped everywhere: no collapsed case reaches the note since PR 3431.

**Homes.**
- `Reading` stays in geom-brep, per `docs/DESIGN-FORK-LOG.md` row 9 (Ev, 2026-09-29: "the at-rest text stays with its reader").
- `recourse::unreadable_margin_note(reading)`, beside `defect_ending`, is the one place a reading picks a note.
- geom-core carries only the sentences, passed as `&str` (`SizedWords::unreadable`, `Indeterminate::ending_noted`).
- The at-rest readers write their own text: check 10 and the checks window for the shell role, check 11 and the import door for the undecided join.

**Moved texts:** every poisoned note. The poison→note rule has one home, so `too-close-to-call-remainder`'s (c) is dropped.

**Follow-ups:**
- `work/exch/import-door-pcurve-escalation-ends-in-the-build-note.md`.
- The import door's poisoned undecided-join text has no pin; this is noted on that EXCH row.
- `"{JOIN_SUBJECT} is undecided ({payload}). {ending}"` is spelled at three readers on purpose: each reader composes its own text, per fork-log row 9.
