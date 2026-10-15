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

## Decision: two readings and their composition (2026-10-15)

Ev approved the designers' converged recommendation on 5154 (2026-10-15): the point-on-span reading splits into `geom`'s one-carrier readings (the point's foot, `Curve3::foot`, and whether it lies on the carrier, `Curve3::on_carrier`, decided off only on a lower bound on its distance to the whole carrier) and the span's reading of its ends (`Span::place`), composed once beside `Span` (`Span::contact`), which decides nothing of its own. The fork-log is row 146.

## What to build, in order

1. **Move the shared machinery down.**
   - `topo::bisect` goes to `geom-core`.
   - `topo::carrier_speed`'s rate bounds and the half-angle ladder (`topo::boolean::circle_roots::half_angle_roots`) go down with the reader.
2. **One span type and one point-on-span reading** (`Off`, `Carrier`, `On(t, margin)`, `End`). It replaces the four readings: `carrier_cross::strictly_inside`, `census::curved::interior_reading` (once `interior_at`) and `clear_of_end`, the census line lane's `on_edge_interior`, and `LoopEdge::contact`'s conic rows.
3. **The table.**
   - Line, circle and ellipse pairs are closed forms in one conic's angle, measured in metres. They are well-conditioned: a double root is decided by the gap between its roots along the curve, and a `Zero` reading places its candidate with the exactly degenerate formula.
   - Spiric and spline pairs use the certified walk.
   - The contract: an in-band decision escalates and never prunes.
4. **Callers.** `carrier_cross` becomes a caller, then the census confirm (and through it `records::landings`), then the census edge-face cut (`restread/the-census-edge-face-cut-crosses-chords-and-conics-only`).
5. **Rows.**
   - The walk serves as a differential oracle over the closed forms, outside the band. It would have caught the cleave P1 `a-near-tangent-coplanar-circle-pair-places-its-touch-off-both-circles` in seconds.
   - The walk check is vacuous where the walk refuses. It runs past its budget on two circles that stay within a few bands of each other round a long arc: on most grazing and tilted draws. There the closed forms are checked by draws built with their least distance in the zero band, by an independent least distance against every decided "no", and by an independent distance from every decided meeting to both spans.
   - The near-concentric census row re-baselines to a decided answer.

## Stage (b)

Stage (b)'s first part settles what stage (a)'s review found, and builds step 2:

- **The root finders have one shape** (`geom_brep::roots`): `Roots<T, N, C>` holds decided answers only (a miss, ON, or a `Found`: one to `N` roots, read only through `Found::roots`), and every refusal is typed in the `Err` (`Unsettled`, or `RootRefusal` where a finder escalates). `TrigRoots`, `QuarticRoots` and topo's `CircleRoots` and `FirstHarmonicRoots` are its instances; `Roots::widen` is the one bridge. `circle_roots::rounding_charge` is retired, and `half_angle_roots` takes a `DegreeTwo`, so a residual past degree two is unrepresentable.
- **One span type and one point-on-span reading**: `geom_brep::curve_pair::{Span, Contact}`.

Its second part builds step 3 and the rows of step 5:

- **The table, the pose and the entry point**: `geom_brep::curve_pair::{method, pose, meet}`. Line, circle and ellipse pairs are read in closed form, and pairs with a spiric or spline side by the walk (`curve_pair/walk.rs`).
- **Across planes**, a candidate is the two carriers' closest pair, polished from where one crosses the other's plane and metered. Each span's ends are read against the other.
- **The rows**:
  - `curve_pair::meet_rows`, the poses;
  - `curve_pair::meet_oracle`: the walk as differential oracle, and draws built inside the zero band, a decided "no" held to an independent least distance;
  - the cleave pin `a_near_tangent_coplanar_pair_is_placed_on_both_circles`.
- **Filed**:
  - `two-coplanar-ellipses-have-no-meeting-method`;
  - `curve-pair-polish-meters-refuse-shallow-meetings`;
  - `curve-pair-line-beside-a-thin-conic-is-unsupported`;
  - `curve-pair-harmonic-with-both-extremes-in-band-refuses-tangency`.

**Stage (c) owes** the callers (step 4):
- `carrier_cross::meetings` becomes a caller of `meet`;
- then the census confirm (`census::curved::reads_meet`, and through it `records::landings`);
- then the census edge-face cut (`restread/the-census-edge-face-cut-crosses-chords-and-conics-only`);
- the near-concentric census row re-baselines to the decided answer that `meet` gives (`near_concentric_arcs_are_decided` pins it in `geom-brep`).

Stage (c) also owes the split of `roots::Unsettled` by what its consumers act on. It holds three kinds in one enum:
- pose-caused refusals: `Tangency`, and the cone door's `AtApex` and `Degenerate`, which are cone vocabulary in a crate with no cone door;
- resolution-caused ones: `Meter`, `SignInBand`, `Exhausted`;
- self-contradictions, a kernel-bug class: `ImpossibleCount`, `Resolvent`, `CountDisagrees`.

`reduce::wall_crossing` reads `AtApex` by name, and the line × torus door and `solid_contain::torus_crossings` read `CountDisagrees`; every other consumer treats them alike. The split moves `AtApex` and `Degenerate` into topo's cone door and the contradictions into an arm consumers cannot fold into a graze. It touches every root-finder row, so it was left out of stage (b).

These four readings decided the point-on-span question themselves, and move onto `Span::contact`:
- `carrier_cross::strictly_inside` (moved in PR 5154);
- `census::curved::interior_reading` (once `interior_at`) and `clear_of_end`;
- the census line lane's `on_edge_interior`;
- `LoopEdge::contact`'s conic rows (`ConicArc::hit`), which read the distance to the conic rather than at a parameter. Its spiric row carries a refusal inside its value (`SpiricHit::Unsettled`, read as `EdgeContact::Unread`), which the move types.

**Follow-on, its own unit:** `two-coplanar-ellipses-have-no-meeting-method`, the coplanar ellipse pair's method. The census confirm refuses that pose typed since it reads `meet`.

**Closes on landing:**
- `two-curve-curve-meeting-finders`
- `near-concentric-arcs-run-the-census-meeting-search-past-its-budget`
- `an-unrecorded-curved-edge-meeting-is-unseen-at-tier-three-prime` (its pass 5′)
- cleave's conditioning P1, when cleave has not landed its own fix first

Coordinate with cleave on `carrier_cross` (cleave/hone ground).
