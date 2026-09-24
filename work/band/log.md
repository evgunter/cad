# BAND — the log

## 2026-09-20 — opened

Cut out of CARVE, which was carrying 73 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed CARVE's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

9 rows arrived by `git mv` with their ids, bodies and history
unchanged. CARVE keeps its band 5600-5699; band 7800-7899 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## 2026-09-24 — seam: TOPO re-worded one comment in `blend/surgery.rs`

TOPO's `half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`
(branch `topo/mint-rows-at-the-mint-site`) re-worded the comment above
the surgery's closing `mint_pcurves` in `crates/sweep/src/blend/surgery.rs`
("the input's caches are stale the moment the first strut lands"):
`mev`, `mef` and `mekr` now leave a face with complete rows complete
or rowless, never half-minted, so the pass is described as minting the
faces the surgery builds and re-deriving the rest. No code in the file
moved. The same unit's sweep run measured the fillet surgery's
"annulus mate trim mef" meeting states the closed-form lane cannot
mint mid-surgery; the operators leave those faces rowless for this
pass rather than refusing.
