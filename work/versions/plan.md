# VERSIONS — plan

**STATUS: OPEN (2026-11-23), no orchestrator.** Live state is the log's
tail and the item files.

## The one row, and how to take it

`the-store-keeps-every-version-and-history-undoes-without-destroying` (P0, X)
is "work out how, then do it" (Ev, PR 7462). Take it in this order:

1. **Design first.** Put the problem to designers per
   `memories/orchestration-model.md`: what the store owes a pin, what
   non-destructive undo means across documents and sessions, and whether
   one history covers both. Start from fork-log row 301 and PR 7462's
   two reports as evidence. Their recommendation (keep every pinned
   version; the memo checks the store still holds a pin; the session
   reads the store once as a value) was made before version control was
   in view. Any change to IDENTITY DI2, ASSEMBLY AQ1, GQ3 or GUI-6 is an
   `[ev]` PR.
2. **Cut the build** into units on their owners (the store in
   `pncad::workspace`, the memo's reuse rule in editor-core, the
   session's read in the viewer, the history in `viewer::history` or
   wherever the design puts it), and file them here or on those owners.
3. **Build,** in an order where no in-between meaning ships.

## Exit criteria

The X row and every unit cut from it closed;
`document-seam-no-in-session-change-detection` closed by the build.
