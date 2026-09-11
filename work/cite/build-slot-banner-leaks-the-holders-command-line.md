---
id: build-slot-banner-leaks-the-holders-command-line
kind: issue
title: with-build-slot.sh's waiting banner prints the holder's command line, a leak channel between blinded review lanes
status: open
opened: 2026-09-08
---


`local-scripts/with-build-slot.sh` prints, while a caller waits for a
slot, a banner naming the HOLDER's command line (the holder file's
"shared: cargo test …" text). During the LIB-TEAPOT v6 dual review
(2026-09-08, ordinal 304) that banner landed in reviewer R1's own build
logs three times, each time carrying R2's command line — a `tess-budget`
sweep with R2's scratch path, `cargo test … lib_teapot_r2 …`, and the
tour's test binary. R1 disclosed it (no finding of R1's came from those
lines; the pair is flagged under v6 item 5 conservatively). The holder
file is REPORTING, not correctness (the script's own header), so the
fix is a spelling one: the banner should name the holder by pid and
lane-agnostic slot only (`slot 1 held since 15:29:24`), never by
command line or path — and the holder file should record the same.
`memories/orchestration-model.md`'s item-5 clause tells lanes not to
list processes; this is the one process listing the box does on their
behalf.

Also seen the same day (orchestrator's own lesson, not the script's):
a reviewer's `ls` of the shared session scratchpad root showed the
other lane's brief FILENAME and directory name. Reviewer briefs now go
in each lane's own directory and reviewer scratch should be pointed
there too (`<lane>/scratch`), never at the shared root.

## Re-homed to CITE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

CITE collects the rows about the project's own text and harness rather
than its kernel: citations that rot, numbers that were reissued, and the
paperwork a lane runs on. This row is one of them.

Its class at the cut was **E** — fix stated: banner prints pid and slot
only, one script. The class is a dispatch estimate made by reading the
row against the tree on 2026-09-11, not a verdict on the finding, and a
lane that finds it wrong says so in its PR. The id, the `track:` letter
where the row carries one, and the body above are unchanged by the move.
