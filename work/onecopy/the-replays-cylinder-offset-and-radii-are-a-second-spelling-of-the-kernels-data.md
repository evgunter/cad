---
id: the-replays-cylinder-offset-and-radii-are-a-second-spelling-of-the-kernels-data
kind: issue
title: editor-core's replay decides two cylinders' axis offset and radii on its own spelling, beside the kernel's cylinder data door
status: open
opened: 2026-11-10
priority: P3
cost: M
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
