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

## Stage (b)

Stage (b)'s first part settles what stage (a)'s review found, and builds step 2:

- **The root finders have one shape** (`geom_brep::roots`): `Roots<T, N, C>` holds decided answers only (a miss, ON, or a `Found`: one to `N` roots, read only through `Found::roots`), and every refusal is typed in the `Err` (`Unsettled`, or `RootRefusal` where a finder escalates). `TrigRoots`, `QuarticRoots` and topo's `CircleRoots` and `FirstHarmonicRoots` are its instances; `Roots::widen` is the one bridge. `circle_roots::rounding_charge` is retired, and `half_angle_roots` takes a `DegreeTwo`, so a residual past degree two is unrepresentable.
- **One span type and one point-on-span reading**: `geom_brep::curve_pair::{Span, Contact}`.

Its second part is step 3, the meeting reading.

**Stage (c) owes** the callers, and the split of `roots::Unsettled` by what its consumers act on. It holds three kinds in one enum:
- pose-caused refusals: `Tangency`, and the cone door's `AtApex` and `Degenerate`, which are cone vocabulary in a crate with no cone door;
- resolution-caused ones: `Meter`, `SignInBand`, `Exhausted`;
- self-contradictions, a kernel-bug class: `ImpossibleCount`, `Resolvent`, `CountDisagrees`.

`reduce::wall_crossing` reads `AtApex` by name, and the line × torus door and `solid_contain::torus_crossings` read `CountDisagrees`; every other consumer treats them alike. The split moves `AtApex` and `Degenerate` into topo's cone door and the contradictions into an arm consumers cannot fold into a graze. It touches every root-finder row, so it was left out of stage (b).

The callers' four readings still decide the point-on-span question themselves and move onto `Span::contact`:
- `carrier_cross::strictly_inside`;
- `census::curved::interior_at` and `clear_of_end`;
- the census line lane's `on_edge_interior`;
- `LoopEdge::contact`'s conic rows (`ConicArc::hit`), which read the distance to the conic rather than at a parameter. Its spiric row carries a refusal inside its value (`SpiricHit::Unsettled`, read as `EdgeContact::Unread`), which the move types.

**Closes on landing:**
- `two-curve-curve-meeting-finders`
- `near-concentric-arcs-run-the-census-meeting-search-past-its-budget`
- `an-unrecorded-curved-edge-meeting-is-unseen-at-tier-three-prime` (its pass 5′)
- cleave's conditioning P1, when cleave has not landed its own fix first

Coordinate with cleave on `carrier_cross` (cleave/hone ground).
