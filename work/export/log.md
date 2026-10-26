# EXPORT — the log

## 2026-09-20 — opened

Cut out of EXCH, which was carrying 44.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed EXCH's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

10 rows arrived by `git mv` with their ids, bodies and history
unchanged. EXCH keeps its band 2100-2199; band 9200-9299 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.
- 2026-10-23 — Closed `step-import-source-debug-in-prose-panics-the-binding`. It was already fixed by D343 (PR 1854, `4f1f257c6b`), on the day the row was split out.
- 2026-10-25 — Seam: PRED S18 (PR 5684) edits `crates/step-export/src/{lib,writer}.rs`, deletes `volume.rs`, and closes `step-export-reclassifies-shell-roles-with-its-own-planar-flux`. The writer reads multi-shell roles off `topo::classify_shells_of`, so hollow curved bodies now refuse `VoidShellUnsupported`. (PRED orchestrator)
