---
id: the-replays-cylinder-axis-is-levered-at-an-unsourced-metre
kind: issue
title: editor-core's replay reads two cylinders' axis parallelism at an unsourced metre of lever
status: open
opened: 2026-11-10
priority: P3
cost: M
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
