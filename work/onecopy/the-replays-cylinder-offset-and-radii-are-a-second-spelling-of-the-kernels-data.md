---
id: the-replays-cylinder-offset-and-radii-are-a-second-spelling-of-the-kernels-data
kind: issue
title: editor-core's replay decides two cylinders' axis offset and radii on its own spelling, beside the kernel's cylinder data door
status: closed
opened: 2026-11-10
priority: P3
cost: M
closed: 2026-11-16
refs: [7030, the-cylinder-data-doors-leave-a-test-only-arm-two-paths-and-a-wide-surface]
---


Found by PR 6773's review (`work/onecopy/the-replays-cylinder-axis-is-levered-at-an-unsourced-metre.md`). That PR read the replay's axis **tilt** as the kernel does: one datum, `|da × db|`, levered at the pair's reach (`topo::boolean::carrier_pair_arm`). The rest of `one_cylinder`'s data are still the replay's own spelling of what the kernel's cylinder data door (`crates/topo/src/boolean/carrier_eq.rs` `cylinder_data`, ~line 658) already reads:

| datum | replay (`crates/editor-core/src/coincide/replay.rs` `one_cylinder`) | kernel (`carrier_eq.rs` `cylinder_data`, after `at_consumed_extent`) |
|---|---|---|
| axis offset | three components of `(ob − oa) × db`, each `Margin::of` (`replay.rs:527`) | one `Margin::norm3(perpendicular(p2 − p1, a1))`, with `p2` the foot of the extent's centre on the second axis (`carrier_eq.rs:665`) |
| radius | `Margin::of(ra − rb)` (`replay.rs:544`) | `Margin::of(r1 − r2)` (`carrier_eq.rs:668`) |
| rims | each rim's radius less its cylinder's, `Margin::of(rim − r)` (`replay.rs:541`) | none: the kernel reads the cylinder's stated radius alone |

The offset is the one that differs in kind: three projections at the axes' shared origin, where the kernel reads one distance at the pivot nearest the faces. The radius difference is the same reading spelled twice. The rim reading is the replay's own, and rightly so (a revolve's wall read from its meridian's two ends, `rim_radii`'s doc), so it stays.

**Owed.**
1. Read the offset and the radius through the kernel's data door: a public reading of a cylinder pair's data as the ladder reads them (beside `carrier_pair_arm`), which the replay decides under its own names and discharges. Then `one_cylinder` keeps only what is the replay's own: the stated-axis branch, the rims and the sense.
2. Measure as 6773 did: the probe over the door's slow corpus, and whether any verdict moves.
3. `one_cylinder` is the replay's only per-kind reading today (`replay.rs` has no `one_plane` or `one_sphere`; the other kinds go through `carrier_pair_decided` whole). A twin added for another kind should read the kernel's data door from the start.

## Resolution (`onecopy/replay-cylinder-data`)

**One computation, two readers.** `geom_brep::carrier_eq`'s
`cylinder_datum` makes the three data once (the tilt `|a1 × a2|`
levered at the arm, `Margin::norm3` of the second axis's offset at the
extent's foot, and `Margin::of(r1 − r2)`), its offset and radius through
`cylinder_offset` and `cylinder_radius`. The pair's displacement over its
extent (`reading`, the over-extent sum) reads those two as well, and
keeps its own chord tilt at its own widened arm. `cylinder_data` names
the data for the ladder. `cylinder_data_at` reads them over a pair's
extent, and topo's public `topo::boolean::cylinder_pair_data` hands them
to the replay. Where the axis is decided, `one_cylinder` decides the
tilt, the offset and the radius difference on those margins under its
own names; an extent that does not read leaves the axis unestablished,
with the pair door's reason. Where the mates state the axis, it reads
the radius difference as it stands, with no extent. The rims and the
sense stay the replay's. Audit F40 says so.

**What pins it.**
- editor-core `coincide::replay::tests::the_axis_offset_is_one_distance_not_three_projections`:
  parallel axes slid `1.2·zero` along a diagonal. Each component of the
  old `(ob − oa) × db` is `0.85·zero` (Zero). The kernel's distance is
  `1.2·zero` (in band), and the replay escalates on exactly that margin.
  The old three-component reading put back records a Zero and fails it.
- topo `carrier_pair::lever_rows::the_cylinder_pair_data_are_the_ladders`:
  a twin differing in every datum (tilted `0.3·zero / arm`, slid
  `1.2·zero`, radius `0.4·zero` larger). `carrier_pair_decided` escalates
  on `carrier_cyl_axis_offset` with exactly the door's `offset`. The
  door's tilt is `|a1 × a2|` at the door's arm and its radius the radii's
  difference, both nonzero, so another arm or extent reads other numbers.

**The 6773 review notes, and 7030's review.**
- The kinds no premise check reads are `geom_brep::is_analytic`'s
  (`edge_nurbs.rs`, already exported, with six callers), which
  `Held::check` and `cell_region`'s held door now read. The inline copies
  in `injective.rs`, `replace_face.rs` and step-import's `chart.rs` read
  it too. `implicit.rs`'s exhaustive match arms are filed.
- `cell_region`'s spline `OffCarrier → None` arm stays silent (7030's
  review ruled the earlier debug assertion out): a spline's off-carrier
  answer is a search the door could not settle, and a debug panic would
  crash a debug consumer on a marginal body.
  `spline_region::rows::a_spline_faces_stations_read_on_its_own_chart`
  is the evidence it is not reached on a chart's own stations.

**Left:** `work/onecopy/the-cylinder-data-doors-leave-a-test-only-arm-two-paths-and-a-wide-surface.md`.

**Measurement.** A probe recorded each `one_cylinder` outcome over the
799 editor-core door and corpus slow tests (`--ignore-default-filter`):
once on the code before the port (63cecf86), and once on the port with
6773's notes (8bdc704d), before 7030's review. The probe was reverted
in 8d9a7790.

| ε | before | after |
|---|---|---|
| default | 65 calls: `stated = false` 1 `Ok`, 6 `Err(Axis)`, 12 `Err(Radius)`; `stated = true` 10 `Ok`, 36 `Err(Radius)`; every `Err` an undischarged Zero | identical, call for call, in order |
| 1e-12 | the same 65 | the same 65 (order differs by test scheduling only) |
| 1e-6 | the same 65 | 51 calls recorded before a pause stopped the run; each is one of the 65 |

No outcome moved where the probe reached. The 1e-6 run's last 14 calls
were not measured.
