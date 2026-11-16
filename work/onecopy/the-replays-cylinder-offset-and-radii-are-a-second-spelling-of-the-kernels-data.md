---
id: the-replays-cylinder-offset-and-radii-are-a-second-spelling-of-the-kernels-data
kind: issue
title: editor-core's replay decides two cylinders' axis offset and radii on its own spelling, beside the kernel's cylinder data door
status: closed
opened: 2026-11-10
priority: P3
cost: M
closed: 2026-11-16
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
`cylinder_datum` now makes the three data once (the tilt `|a1 × a2|`
levered at the arm, `Margin::norm3` of the second axis point's offset at
the extent's foot, and `Margin::of(r1 − r2)`). `cylinder_data` names them
for the ladder. `cylinder_data_at` reads them over a pair's extent, and
topo's public `topo::boolean::cylinder_pair_data` hands them to the
replay. `one_cylinder` decides the tilt, the offset and the radius
difference on those margins under its own names. The rims and the sense
stay the replay's. An extent that does not read leaves the axis
unestablished (the radius, where the axis is stated), with the pair
door's reason. Audit F40 says so. `carrier_pair_arm` stays the public
reading of the arm, and its pin holds.

**What pins it.**
- editor-core `coincide::replay::tests::the_axis_offset_is_one_distance_not_three_projections`:
  parallel axes slid `1.2·zero` along a diagonal. Each component of the
  old `(ob − oa) × db` is `0.85·zero` (Zero). The kernel's distance is
  `1.2·zero` (in band), and the replay escalates on exactly that margin.
  The old three-component reading put back records a Zero and fails it.
- topo `carrier_pair::lever_rows::the_cylinder_pair_data_are_the_ladders`:
  on the same kind of pose, `carrier_pair_decided` escalates on
  `carrier_cyl_axis_offset` with exactly the door's `offset`, and the
  door's `tilt` and `radius` are exactly 0.

**The 6773 review notes, folded in.**
- N3: the kinds no premise check reads are one predicate,
  `geom_brep::has_implicit_form`, read through `held::premise_reads` by
  `Held::check` and by `cell_region`'s held door.
- N1: `cell_region`'s spline `OffCarrier → None` arm is a
  `debug_assert!` in debug builds, so a regression of the spline search
  goes red in the suites while release keeps the cell.

**Measurement.** A probe recorded each `one_cylinder` outcome over the
799 editor-core door and corpus slow tests (`--ignore-default-filter`):
once on the code before the port (63cecf86), and once on the port with
N1 and N3 (8bdc704d). The probe was reverted in 8d9a7790.

| ε | before | after |
|---|---|---|
| default | 65 calls: `stated = false` 1 `Ok`, 6 `Err(Axis)`, 12 `Err(Radius)`; `stated = true` 10 `Ok`, 36 `Err(Radius)`; every `Err` an undischarged Zero | identical, call for call, in order |
| 1e-12 | the same 65 | the same 65 (order differs by test scheduling only) |
| 1e-6 | the same 65 | 51 calls recorded before a pause stopped the run; each is one of the 65 |

No outcome moved where the probe reached. The 1e-6 run's last 14 calls
were not measured.
