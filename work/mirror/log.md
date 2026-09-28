# MIRROR — the log

## 2026-09-20 — opened

Cut out of CIW, which was carrying 67 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed CIW's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

25 rows arrived by `git mv` with their ids, bodies and history
unchanged. CIW keeps its band 1500-1599; band 9900-9999 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## 2026-09-26 — seam note from S-DUP (S-DUP orchestrator)

S-DUP's sixth-batch PR edits two files in this program's territory:
- **`local-scripts/ci-local.sh` `topo_release`** now carries hosted's `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false` and its `review_d18` filter. The local row had gone red on a green tree: `foreign_parent_loop_garbage_in_garbage_out_release` never compiled locally.
- **`scripts/check-ci-mirror-parity.py`'s `SEMANTIC_ENV`** gains that variable. A plant with the old `ci-local.sh` reds the reader, and the fixed one passes. The selftest is OK.

The filter half is filed here as `the-parity-reader-does-not-compare-test-name-filters`.
