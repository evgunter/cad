---
id: the-viewer-drops-every-dm7-rename-report
kind: issue
title: The viewer keeps only cluster_rows() of an applied edit, so no Rebound or Strand row ever reaches a GUI user
status: open
opened: 2026-09-24
priority: P1
cost: D
---


`crates/viewer/src/session.rs:2647`, in `commit_run`, keeps only
`applied.cluster_rows()` from an accepted edit, for the logged entry. The
other two log sites, `:2772` and `:2848`, do the same.

The `Strand`, `StrandedAppearance`, `OrphanedDeclare` and `Rebound` rows
that DM7 requires the edit door to report (`crates/editor-core/REFERENCES.md`
DM7) are dropped:
- They are not logged. That is correct, because they are re-derived at
  replay.
- They are not surfaced either. `OpOutcome` carries no maintenance, so a
  GUI user who drags a parameter past a canonical-numbering change never
  sees:
  - that their selection was rebound (`reanchor_report`, PR #3180);
  - that it was stranded.
- The same holds for `DeleteNode` strands and `SetProgram` rebinds.

DM7's "why not as-is: a legal edit whose consequence is invisible until
evaluation is what the maintenance column exists to end" holds at the
API and fails at the one client.

The fix belongs to the viewer: carry the rows on `OpOutcome` and show
them in the chrome, as DM7's own bullet about the cascade affordance
already anticipates.
