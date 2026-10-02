# TQUERY — the log

## 2026-09-20 — opened

Cut out of TOPO, which was carrying 132 budget points in one directory
— about four and a half sittings — when Ev ratified the priority and
track-size conventions in chat the same day. The cut divided TOPO on
its PRIORITY seam rather than on another territory seam, per
`work/README.md` "Track size": five successors plus the Euler-operator
remainder TOPO keeps.

7 rows arrived, each by `git mv` with its id, body and history
unchanged. Band 6500-6599 claimed in this commit
(`docs/MODEL-AB-LOG.md`). Nothing dispatched.

## Ev ruled `rim-of-compares-point-bits-…`: a consumer (TOPO orchestrator, 2026-09-29)

On PR 3156 (2026-09-24), Ev answered "(b)": `query.rs`'s `same_bits` /
`same_point_bits` behind `rim_of`'s `same_circle` is a production
bit-identity coincidence check, which the retirement forbids. The
ruling and the repair shape (a `GeomSource` read first, allowlisted
`eq_bits` only where no recipe exists) are on the row. The gate half
is filed on GUARD.
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/split.rs`. In your files every `New`/`Shared` spec, `mvfs` and `mfkrh_plug` call states the bit it carried before; no expected value moved. (TOPO implementer)

## 2026-10-02 — first sitting: track taken

An orchestrator holds the track (`status: active`). CLEAVE (active)
shares `topo::split`'s ground (`crates/topo/src/splitting/*`); lanes
here announce that seam in their PRs.

**Review posture, answered** (the plan left it open): single FULL
review (claims + style lane) on the two P0 units — both change what a
public door decides, so believing them takes more than reading them;
orchestrator's read for the `E` rows that ride along. No dual: neither
unit is a broad architectural choice, and Ev already ruled the shape
of the `rim_of` repair (PR 3156).

**Dispatch, wave 1** (both measure first, since main has moved a long
way under these rows since they were filed):
- `tquery/split-cyl-feature` — `split-refuses-cylindrical-feature-box`:
  re-measure both variants on current main, diagnose, fix.
- `tquery/rim-of-recipe` — `rim-of-compares-point-bits-…` (Ev's
  ruling) which also decides `rim-of-refuses-extruded-multi-arc-rims`;
  riders in the same file: `rim-of-flattens-a-dangling-curve-key`,
  `tquery-refusal-prose-outgrows-the-viewer`.
- 2026-10-02 — `split-edge-cannot-carry-a-fitted-or-general-pcurve-row`
  parked on PR 3759 (PCERT, "pcurve rows are mandatory at rest"),
  which rewrites `crates/topo/src/pcurves.rs`'s carry (`split_cache`'s
  home) and `tests/split_edge_pcurve_rows.rs`; building the
  Fitted/General carry under it would race it.
- 2026-10-02 — `curve-kind-placement-…` (the open ruling) sent to the
  designer pair; blinding record on
  `analysis/design-fork/tquery-curve-kind-placement`.
- 2026-10-02 — `split-edges-key-retention-direction-is-pinned-by-no-row`
  dispatched (`tquery/split-edge-retention`), orchestrator's-read tier:
  a test-only row whose content is written in the item.
- 2026-10-02 — designers agreed on the final state (one kind mirror per enum, in `geom`; sets stay in `topo::query`), split only on authoring (hand-written vs `strum` derive); `[ev]` PR 3763 opened carrying both reports as A/B (fork-log row 41).
- 2026-10-02 — `tquery/rim-of-recipe` STOPPED at a fork, nothing built.
  Ev's preferred repair (a `GeomSource` read) has nothing to read:
  every kernel-direct body's curves are `KernelDirect`, and editor-core
  stamps one source per curve description, so a rim's arcs are always
  distinct sources. The fallback (`eq_bits`) keeps the refusal.
  Measured: all arcs of an extruded rim share the same two surface keys
  (extrude decides one cylinder per run), and with the bit compare off
  the rim suites pass bar four rows that pin the bit rule or the
  winding contract. A CLASS finding: N6's recipe provenance does not
  reach kernel-direct bodies, so any repair that leans on it is
  editor-core-only. Sent to the designer pair
  (`analysis/design-fork/tquery-rim-identity`); the dangling-key and
  prose riders wait for the answer (same arms of `RimError`).
