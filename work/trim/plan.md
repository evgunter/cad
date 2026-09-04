# TRIM — the NURBS trim frontier (plan)

**STATUS: OPEN (2026-09-03).** Opened 2026-09-03 from `docs/WORK-TRACKS-2026-09.md` (TRIM section), which is this
program's charter until this plan supersedes it. Live state is
`work/trim/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`trim/`** — unit branches
`trim/<unit>-<slug>`, orchestrator branch `trim/orchestrator`.
Away-channel tag `(TRIM orchestrator)`. A/B ordinal band
**TRIM = 2500–2599**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit, per that entry's rule.

**Opens when CURVED lands the rim arms.** See §Opening condition.

## Charter

Every face carrying a General pcurve certifies, measures and
tessellates. PCURVE's exit walk (`docs/PCURVE-EXIT-WALK.md`, in the
ledger) closed edge-description unification and left P-2 — the
interior iso-curve and trimmed-region frontier — unmerged at
`docs/PCURVE-P2-SPEC.md`, which this program reads as its spec input.

## Opening condition

The two largest units need a whole body at rest carrying an interior
column, which CURVED's rim arms produce; until then the openers are
the loft seam compare and the chart-boundary description.

## Order

1. `loft-seam-carrier-exact-knot-compare` (S-CERT's file today; D→H)
   — a tolerance-structural compare with a soundness story, or an
   exact skin-fit reproduction of the chart's boundary row.
2. `clearance-window-tightening-needs-chart-boundary` — a pcurve-layer
   chart-boundary description (planar: the loop's 2-D extent;
   cylinder: the real angular span) that `editor-core/clearance.rs`
   intersects each carrier window with; M10-5's declared deviation D3.
3. `interior-iso-curve-de-boor-extractor` — the de Boor collapse
   extractor so an interior iso of a NURBS chart certifies as an exact
   `IsoLine`; widen `nurbs_iso_derive`'s wall-wall arm; retire the
   `an_interior_column_still_refuses` pin; L.
4. `general-pcurve-face-props-and-tess-refuse` — lift the six
   `QuadratureUnsupported`/tessellation refusals for non-rectangular
   chart trim regions; L; rides with 3.
5. `unify-edge-descriptions-on-pcurves` (S-CERT's file today) — check
   its state against the ledger's PCURVE row before scheduling.

## Review posture

Full v6 dual with Fable specs.

## Exit shape

`docs/PCURVE-P2-SPEC.md` is either landed and deleted per the ledger
or superseded by a spec here; the walk convention applies.
