---
id: the-replays-cylinder-axis-is-levered-at-an-unsourced-metre
kind: issue
title: editor-core's replay reads two cylinders' axis parallelism at an unsourced metre of lever
status: closed
opened: 2026-11-10
priority: P3
cost: M
closed: 2026-11-10
---


Found by ONECOPY's roster-reach unit, routing `editor-core/src/coincide/replay.rs::one_cylinder`'s raw `sign_within` through `k_stats` (audit F40).

`one_cylinder` decides two cylinders one axis on the three components of `da × db`, the axes' cross product. Those are sines of two unit directions, so they are dimensionless. The raw read held them against the band in metres as if they were lengths. Routing them, the unit kept the behaviour bit-identical by levering them at `Lever::unsourced(1 m)` under their own name, `replay_one_cylinder_axis`. That keeps the verdicts the same, and the code now says the lever has no source.

**The real lever.** A tilt `θ` between the axes displaces the walls by about `θ · arm` over the extent the coincidence is claimed on. Candidates:
- **the radius** (`ra`, or `ra + rb` as rim_wedge's `same_circle` uses for its axis datum): the tilt's displacement at the wall, over one radius of axial run;
- **the faces' reach**, as `topo::boolean::carrier_eq`'s cylinder datum does (`Margin::levered(a1.cross(a2).norm(), arm)`, `arm` from the pair's consumed extent): the tilt's displacement over the stretch both faces cover. This is what the kernel path beside it (`carrier_pair_decided`) reads, so the replay and the kernel would judge one tilt alike.

Also: the replay reads each cross component apart, where carrier_eq reads `|da × db|`. With a lever, the norm is the honest datum: one tilt, not three projections of it.

**Measurement plan.**
1. Under each candidate, run editor-core's `ci` suite at default ε and at 1e-12, and record which `coincide` rows change from discharged to deferred or back.
2. Run the replay's corpus tests (`intent_*`, `docm*`, `name_invariance::*`) and diff each document's deferred list before and after.
3. Count `replay_one_cylinder_axis`'s K samples on the probe corpus, and compare their spread under each lever against the band.
4. Prefer the candidate whose verdicts agree with `carrier_pair_decided` on every row both read. Any disagreement is the finding.

## Resolution (`onecopy/replay-axis-lever`)

**The lever is the faces' reach**, the kernel's own. A new public door,
`topo::boolean::carrier_pair_arm`, returns the arm
`carrier_pair_decided` levers a pair's angular data at: the reach of
the ball enclosing both faces from the pivot the ladder reads the
position datum at (for two cylinders, the foot of the ball's centre on
the second axis). `one_cylinder` reads its tilt there, so the replay
and the kernel judge one tilt alike (plan item 4). The radius was the
other candidate. It is a curvature arm, not the axial run a tilt
displaces the walls over, so it is not the datum's arm.

**One datum, not three.** The tilt is read as `|da × db|`, as
`carrier_eq`'s `carrier_cyl_axis_parallel` reads it, instead of each
component of the cross product apart. `Lever::unsourced` leaves
`replay.rs`, and `scripts/gates/lever-sources.counts` drops its line.
Audit F40 says so.

**What moved: nothing.** A probe (on the branch's history, removed
before the PR) recorded every `one_cylinder` call over the 799 editor-core
tests that reach the door and the replay's corpus (`coincidence_door`,
`intent_*`, `docm*`, `name_invariance`, `revalue_*`, the mates and the
bores, run with `--ignore-default-filter`). It saw 65 calls at each ε,
19 deciding the axis (`stated = false`). Every component of `da × db`
read exactly `0`, so every lever reads the same Zero. The arm read `Ok`
at every call, between 0.75 and 2.58 m. The set's verdicts are the same
before and after, at both ε: its three reds fail identically on main
and are filed (`main-red-the-composed-die-door-proves-two-joints-where-one-is-pinned`,
`coaxial-unions-with-a-torus-or-a-sphere-and-a-tangent-cone-refuse`,
`main-reds-editor-core-slow-set-and-the-k-probe-sweep`). The axis's K
samples go from three per deciding call to one, all at margin 0. No
ledger pins `replay_one_cylinder_axis`.

**Also here (the orchestrator's note on 6741).** `cell_region.rs`
`place`, the other held door 6669 named, read a curved `OffCarrier` as
`Out` through `curved_face_containment`. Its station is `constructed`
(a point of the face's own stored chart), so it now reads
`curved_face_placement`, and a curved `OffCarrier` panics as the
kernel's defect, as at every other held door. Its `names-boolean` pin
rises 11 → 14, cited to
`work/boxes/the-face-covers-sphere-arm-reads-the-booleans-chart-trim-and-face-placement.md`,
which records it.
