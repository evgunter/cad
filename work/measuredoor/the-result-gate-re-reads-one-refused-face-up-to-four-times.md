---
id: the-result-gate-re-reads-one-refused-face-up-to-four-times
kind: issue
title: The result gate's volume reads one refused face up to four times per boolean
status: open
opened: 2026-11-03
priority: P4
cost: M
---


Found by SPIRICARM's chord-volume decide (`spiricarm/chord-volume-decide`, 2026-11-03), with the `SPIRICARM_PROBE=1` dump that branch adds in `topo::props::probe_dump`.

On a boolean whose result the volume lane refuses, the result gate reads the same refused face several times. The probe log shows faces 9v1, 10v1, 5v3 and 4v3 each repeated, up to four times per boolean: `cylinder_sphere_frame` pose 0 and the donut ∩ rod cases, on main `1c738072cd`. Each repeat redoes the lane's quadrature setup before it refuses, and on the slow cylinder pairs (`spiricarm/a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses`) that cost multiplies.

Which call sites repeat the read was not traced. Start from the result gate's measure path and `topo::props::face_flux_about`'s callers. Either one read should be shared, or the repeats should be shown to ask different questions. (Filed by the SPIRICARM orchestrator.)
