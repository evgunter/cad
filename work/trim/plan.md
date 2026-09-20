# TRIM — the NURBS trim frontier (plan)

**STATUS: OPEN, DISPATCHING (since 2026-09-04).** Opened 2026-09-03
from `docs/WORK-TRACKS-2026-09.md` (TRIM section); this plan supersedes
that section as the charter. Live state is
`work/trim/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`trim/`** — unit branches
`trim/<unit>-<slug>`, orchestrator branch `trim/orchestrator`.
Away-channel tag `(TRIM orchestrator)`. A/B ordinal band
**TRIM = 2500–2599**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit, per that entry's rule.

**Opened for dispatch 2026-09-04.** See §Opening condition.

## Charter

Every face carrying a General pcurve certifies, measures and
tessellates. PCURVE's exit walk (`docs/PCURVE-EXIT-WALK.md`, in the
ledger) closed edge-description unification and left P-2 — the
interior iso-curve and trimmed-region frontier — unmerged at
`docs/PCURVE-P2-SPEC.md`, which this program reads as its spec input.

## Opening condition (revised 2026-09-04, Ev's nod in-chat)

The tracks doc gated this program on "CURVED's rim arms". Traced: the
rim arms that block a whole body at rest carrying an interior column
are `nurbs_iso_derive`'s own arms in `topo/src/pcurves.rs` — PCURVE
P-2 (PR #1177) widened the cap-rim arm and deliberately REVERTED the
wall-seam arm, whose remaining blocker is the de Boor collapse
extractor, this program's own item. VERBS' rim capability (RIMCAP, the
partial-revolve circle-profile rim) is a different rim and nothing in
CURVED feeds this program. So the extractor is the opener, the
props/tess lane measures behind it, and the loft-seam compare and the
clearance-window description run independently.

## Order (re-cut 2026-09-20)

TRIM-1, TRIM-2 (both PRs) and TRIM-3 (both PRs) are DELIVERED and
their specs ledgered; the former order and the residue lists are
recoverable in this file's history at the cut commit. On Ev's in-chat
direction the chart-side residue (31 items) moved to **CHART**
(`work/chart/`), opened in the same commit, and the chord-count
arithmetic class to TESS's slate. What remains here:

1. `boundary-iso-doors-panic-before-they-can-refuse` — E: the two
   doors refuse typed on a corrupt net and their `# Errors` contract
   says what they do; block TRIM-B2 slot 2 (FABLE by the draw) if the
   orchestrator triages it INTO the protocol, else opus/opus outside
   it per v7 — the call at dispatch (its logic is not tricky; the
   default is outside).
2. `pcurve-p2-spec-says-edge-nurbs-throws-the-image-away` — closes
   with the walk: `docs/PCURVE-P2-SPEC.md` is superseded by the three
   delivered specs and deleted per the ledger, the item's finding
   recorded in the ledger entry.

## Review posture

Full v6 dual with Fable specs.

## Exit shape

The P-2 body mints, validates, measures and tessellates at rest
(TRIM-1, TRIM-2); the clearance window reads the chart boundary
(TRIM-3); `boundary_iso_u/_v` refuse instead of panic;
`docs/PCURVE-P2-SPEC.md` is deleted per the ledger as superseded; the
walk convention applies.
