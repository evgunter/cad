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

## Order

1. `interior-iso-curve-de-boor-extractor` — the de Boor collapse
   extractor so an interior iso of a NURBS chart certifies as an exact
   `IsoLine`; widen `nurbs_iso_derive`'s wall-wall arm; retire the
   `an_interior_column_still_refuses` pin; L. THE OPENER.
2. `general-pcurve-face-props-and-tess-refuse` — lift the six
   `QuadratureUnsupported`/tessellation refusals for non-rectangular
   chart trim regions; L; measured against the whole body 1 lets
   mint. `mesh/trimmed.rs`, `mesh/chords.rs` and `topo/props.rs` are
   S-MESH's and Track M's ground: announced seams.
3. `clearance-window-tightening-needs-chart-boundary` — a pcurve-layer
   chart-boundary description (planar: the loop's 2-D extent;
   cylinder: the real angular span) that `editor-core/clearance.rs`
   intersects each carrier window with; M10-5's declared deviation D3.
   Independent of 1–2; may run in parallel.
4. `loft-seam-carrier-exact-knot-compare` (S-CERT's file today; D→H)
   — a tolerance-structural compare with a soundness story, or an
   exact skin-fit reproduction of the chart's boundary row. Its file is
   S-CERT's: dispatched by announced seam or after S-CERT's exit.
5. `unify-edge-descriptions-on-pcurves` (S-CERT's file today) — check
   its state against the ledger's PCURVE row before scheduling.
6. Riders on the four Track Q files, landing with whichever unit opens
   their file: `D36`, `S394`, `S83`, `D305`,
   `fitted-magnitude-nan-schedule-parameter`.

## Review posture

Full v6 dual with Fable specs.

## Exit shape

`docs/PCURVE-P2-SPEC.md` is either landed and deleted per the ledger
or superseded by a spec here; the walk convention applies.
