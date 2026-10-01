# TRIM exit walk

**Program:** TRIM — the NURBS trim frontier (`work/trim/`). Opened
2026-09-03, open for dispatch 2026-09-04; cut on 2026-09-20 (Ev, in-chat)
to what it would finish itself, with its chart-side residue moved to
CHART. **Charter, criteria and order:** `work/trim/plan.md`.
**Narrative:** `work/trim/log.md`.

The criteria are the plan's `## Exit shape`, quoted verbatim and taken
one clause at a time. They were written at the 2026-09-20 cut, after the
first three units had merged and before the last one ran.

## The slate, as it stands

Nine rows, nine closed, zero open (`work.py status --program trim`).

| row | status | closed by |
| --- | --- | --- |
| `interior-iso-curve-de-boor-extractor` (TRIM-1) | closed | PR 2095, merged 2026-09-07 at 416ccdfc6 |
| `clearance-window-tightening-needs-chart-boundary` (TRIM-3) | closed | PRs 1911 and 2554, merged 2026-09-07 and 2026-09-15 |
| `general-pcurve-face-props-and-tess-refuse` (TRIM-2) | closed | PRs 2564 and 2863, merged 2026-09-19 and 2026-09-20 |
| `boundary-iso-doors-panic-before-they-can-refuse` | closed | PR 3525, merged 2026-10-01 at c60bfd61f |
| `trim-escalations-offer-a-declaration-the-door-cannot-take` | closed (rider) | PR 3525 |
| `S394` | closed (rider) | TRIM-1, PR 2095 |
| `fitted-magnitude-nan-schedule-parameter` | closed (rider) | TRIM-1, PR 2095 |
| `loop-continuity-meters-u-through-levered-and-v-through-metered` | closed | outside TRIM (a TOPO lane on `pcurves.rs`), recorded in its item |
| `pcurve-p2-spec-says-edge-nurbs-throws-the-image-away` | closed | the spec's deletion, PR 3619 |

## The criteria

> The P-2 body mints, validates, measures and tessellates at rest
> (TRIM-1, TRIM-2);

**Met.** TRIM-1 (PR 2095) built the de Boor collapse extractor for
interior iso-curves; its closing measurement had the P-2 body's whole
cache set mint and `validate_pcurves` come back empty — the first
whole-body mint of a trimmed chart at rest. TRIM-2 PR-1 (PR 2564) made
volume and area answer on a face carrying a `General` pcurve, through
Green's theorem on the chord polygon with certified lune pads. The
degree-2 widening's `mass_properties` brackets the oracle prism's at
all three ε, and after the fix pass the area bracket equals the oracle's.
TRIM-2 PR-2 (PR 2863) made `mesh::tessellate` answer the same body. The
`General`-faced wall's patch is bit-identical to the oracle wall's
(71 600 triangles, 36 736 ids). Its row asserts that per patch.

One limit, recorded rather than claimed: the P-2 body's `General` image
spans `u ∈ [2 − 2.2e-16, 2]`, so no end-to-end body exercises the trimmed
lane on a chart image with real curvature. The quadrature's curved
behaviour is pinned by hand-built rows only. That gap waits on a
producer and is filed on CHART as
`curved-trim-e2e-fixture-waits-for-a-producer`.

> the clearance window reads the chart boundary (TRIM-3);

**Met.** TRIM-3 PR-1 (PR 1911) built the chart-boundary description
(`topo::chart_boundary`, `chart_bound.rs`: SAT plus ray parity on a
metred hull). TRIM-3 PR-2 (PR 2554) made `editor-core/clearance.rs`'s
sweep read it. The root window is cut to the described hull, and cells
certified off the face are dropped. The three measured shapes flip: the
L-cap notch, the U-channel slider, and the whole-body self-intersection.
`min_separation` stays untightened as the identity, with its mechanism
stated correctly. The residue (cone, sphere and torus windows, round
holes, exact-region cells, revolved bands) is on CHART.

> `boundary_iso_u/_v` refuse instead of panic;

**Met.** PR 3525. Both doors, and `interior_iso_u`, which had the same
indexing, check the net's lengths against its knots before forming an
index, and refuse typed (`ControlCountMismatch`, `WeightCountMismatch`)
through one count rule in `geom`. The row builds a short net and covers
both slices, both directions and both ends; removing the check reds it.
`StepImportError::WallColumnStructure`'s doc is back to the wording
#2406 first meant.

> `docs/PCURVE-P2-SPEC.md` is deleted per the ledger as superseded;

**Met.** PR 3619, with `docs/doc-ledger/pcurve-p2-spec.md` naming the
SHA it is recoverable at and the false sentence it carried.

> the walk convention applies.

**Met** by this file. Every row is closed; nothing is parked or
deferred.

## Residue, re-homed by file before the sweep

- **CHART** (`work/chart/`, opened at TRIM's 2026-09-20 cut): 31 rows on
  `pcurves.rs`, `pcurve_cache.rs`, `chart_bound.rs`, `chart_region.rs`
  and `ssi/` — TRIM-3's and TRIM-2's residue among them. Two more
  `pcurves.rs` doc rows followed on 2026-09-30.
- **TESS** (`work/tess/`): `chord-count-arithmetic-is-plain-f64-across-every-speed-arm`,
  which names TESS as its owner.
- **PCERT** (`work/pcert/`): `line-seam-boundary-row-refusal-discarded-as-iso-unsupported`,
  filed by the iso-door unit's sweep.

Nothing is left in `work/trim/` that is not closed.

## The A/B record

TRIM ran blocks TRIM-B1 (concluded and folded 2026-09-15) and TRIM-B2
under the model A/B protocol. TRIM-B2's slots 0 and 1 concluded
(TRIM-2's two PRs). Its slot 2 is unspent: the protocol was suspended on
2026-09-23 before the last unit dispatched, and that unit ran outside it
on a single Opus style review. The block's record folds to main closed
by the suspension.

## On ratification

`work/trim/` (`program.md`, `plan.md`, `log.md` and the nine closed rows)
and this walk are deleted in the sweep, with a note under
`docs/doc-ledger/` naming the SHA they are recoverable at. TRIM's A/B
band 2500–2599 stays claimed in the A/B log, with 2504 its last ordinal.
