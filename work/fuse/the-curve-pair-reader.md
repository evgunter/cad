---
id: the-curve-pair-reader
kind: unit
title: Build geom-brep's curve-pair meeting reader: one point-on-span reading, one total (CurveKind, CurveKind) table, the census walk as oracle; carrier_cross, the census confirm and the edge-face cut become its callers
status: open
opened: 2026-10-14
priority: P1
cost: H
refs: [two-curve-curve-meeting-finders, 5027]
---


## The decision it carries out

Ev ruled on PR 5027 (2026-10-14). The reader lives in `geom-brep`, beside C5's surface-pair table, and C5 governs curve pairs too: one exhaustive `(CurveKind, CurveKind)` table, one method per pair fixed at write time, no runtime fallback. The designer pair's agreed final state is in the PR body and in `two-curve-curve-meeting-finders`.

## What to build, in order

1. **Move the shared machinery down.**
   - `topo::bisect` goes to `geom-core`.
   - `topo::carrier_speed`'s rate bounds and the half-angle ladder (`topo::boolean::circle_roots::half_angle_roots`) go down with the reader.
2. **One span type and one point-on-span reading** (`Off`, `Carrier`, `On(t, margin)`, `End`). It replaces the four readings: `carrier_cross::strictly_inside`, `census::curved::interior_at`/`clear_of_end`, the census line lane's `on_edge_interior`, and `LoopEdge::contact`'s conic rows.
3. **The table.**
   - Line, circle and ellipse pairs are closed forms in one conic's angle, measured in metres. They are well-conditioned: a double root is decided by the gap between its roots along the curve, and a `Zero` reading places its candidate with the exactly degenerate formula.
   - Spiric and spline pairs use the certified walk.
   - The contract: an in-band decision escalates and never prunes.
4. **Callers.** `carrier_cross` becomes a caller, then the census confirm (and through it `records::landings`), then the census edge-face cut (`restread/the-census-edge-face-cut-crosses-chords-and-conics-only`).
5. **Rows.**
   - The walk serves as a differential oracle over the closed forms, outside the band. It would have caught the cleave P1 `a-near-tangent-coplanar-circle-pair-places-its-touch-off-both-circles` in seconds.
   - The near-concentric census row re-baselines to a decided answer.

## Stage (b) owes

Stage (a) (PR 5093) moved the machinery down unchanged. These were found in its review and belong to stage (b)'s reading:

- **The four root enums are one class.** `geom_brep::half_angle::TrigRoots`, `topo`'s `circle_roots::CircleRoots`, `FirstHarmonicRoots` and `geom_brep::quartic::QuarticRoots`, with their two `From` bridges. Under D3's "One reading per question", stage (b) decides their one shape.
- **`certified_subdivision` hides a refusal in its value.** It folds a bisect `Exhausted` and a meter refusal into the value arm `TrigRoots::Uncertain`, an untyped refusal inside the value. Stage (b)'s reading returns a typed refusal instead (D4).
- **Two spellings of one charge.** `topo`'s `circle_roots::rounding_charge` is a one-line alias of `geom_brep::rounding_charge`. Retire the alias.
- **`LadderRefusal::PastDegreeTwo` is a runtime check a type could rule out**: a degree-two type (`TrigPoly::second`) would make it unrepresentable.

**Closes on landing:**
- `two-curve-curve-meeting-finders`
- `near-concentric-arcs-run-the-census-meeting-search-past-its-budget`
- `an-unrecorded-curved-edge-meeting-is-unseen-at-tier-three-prime` (its pass 5′)
- cleave's conditioning P1, when cleave has not landed its own fix first

Coordinate with cleave on `carrier_cross` (cleave/hone ground).
