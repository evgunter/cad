---
id: import-snaps-each-zero-relation-by-a-closed-catalogue
kind: unit
title: step-import: the snap catalogue — a relation between surfaces the file band reads Zero (coaxial, tangent, axis-normal, one locus) made exact by one catalogue construction per connected class, the earlier record staying, each move reported (D7 step 3, PR 7074)
status: closed
opened: 2026-11-18
priority: P1
cost: H
parent: import-snaps-a-relation-the-file-band-reads-zero-and-the-run-reads-apart
branch: importgate/snap
closed: 2026-11-19
---

The healing step of D7 as ratified on PR 7074 (step 3): a relation between surfaces that reads `Zero` at the file's band is made exact by one construction from a closed catalogue, each connected class of such relations solved jointly; where the construction is symmetric the record earlier in the file stays; each move is reported; a relation no catalogue construction makes exact refuses, naming the entity and the lane. It stacks on the build (PR 7128), whose `SNAP_LANE` arms (`crates/step-import/src/build.rs`: `analytic_pair`, `served_pair`, `tangent`, `spline_pair`) refuse today where the file band reads a relation the run's band reads apart.

## Scope

- **Measure first.** At main's head, census the corpus's `SNAP_LANE` refusals (file, edge, the relation, the pair's kinds, the gap at the run's band) and the relations the file band reads `Zero` that the run already reads exact. Say which relations the catalogue needs; `nist/nist_ftc_09_asme1_rd.stp` (plane × cylinder tangent at the file band, ε_in ≈ 34 µm) is one.
- **The catalogue, closed.** One construction per relation, each in the kernel's own vocabulary:
  - *coaxial* — axes of two revolved surfaces made one line;
  - *tangent* — a pair the file band reads tangent along an edge made tangent (a plane's offset or a revolved surface's radius, whichever the construction moves);
  - *axis-normal* — a plane's normal made parallel to a revolved surface's axis;
  - *one locus* — two records of one analytic surface made one record (non-planar: `two-records-of-one-non-planar-surface-stay-two-at-import`; the planar act is `[topo.one-record]`'s, PR 7091).

  What is in the catalogue is stated in one place (the code's list and the crate docs). A relation outside it refuses `NotYetSupported`, naming a filed row.
- **Joint solve.** Relations form a graph over surface records; each connected class is solved by one construction (inputs that no relation moves stay; the record earlier in the file stays where the construction is symmetric). A class the catalogue cannot solve jointly refuses, naming its lane: no sequential snapping whose order decides the result.
- **At interpretation.** Snaps are applied to the interpreted surfaces before the build, so the build constructs the exact relation. The `SNAP_LANE` arms then build instead of refusing; the lane constant and its refusals go where nothing reaches them.
- **Reported.** Every moved surface on the ledger (`NormalizationKind`), with what moved and by how much. Every move is bounded by the band that read `Zero`, which needs no new threshold.
- **Readings.** Every relation is read through `FileBand` / `[verdicts]`. An in-band reading is the file's ambiguity, as today; a definite reading is apart.

Closes: `import-snaps-a-relation-the-file-band-reads-zero-and-the-run-reads-apart`, `wild-nist-ftc-09-refuses-import-at-eps-1e-12` (re-pin `tier_gate`'s `NIST09_SNAP`; restore the `demos/wild` cell, which needs a `[render]` commit), `two-records-of-one-non-planar-surface-stay-two-at-import` (`TAIL_TURBINE` should import again; say if it does not).

Not in scope: the normalizations' move after the build; the remaining C5 section arms (`an-imported-edge-on-a-pair-the-c5-table-does-not-build-refuses`); tangent loci with no closed form.

Letter H, tier dual. Dispatched during the 2026-11-18 Fable outage on Opus (Ev, PR 7091), so no model-compare draws and no row.

- 2026-11-19 — Closed by PR 7135 (the snap catalogue, D7 step 3).
