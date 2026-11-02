---
id: restdatum
kind: program
title: RESTDATUM — checks 1 and 4: a carrier read against its datum and against its faces
status: ready
opened: 2026-10-29
group: kernel/topo/certify
prefix: restdatum/
tag: (RESTDATUM orchestrator)
paths: [crates/topo/src/validate.rs, crates/topo/src/tier3_tests.rs]
keep_out: [split from RESTFRONT on 2026-10-29 and shares validate.rs with its five siblings RESTFRONT RESTCURVE RESTSHELL RESTGATE RESTDATUM RESTWORD - the files are shared and the questions are not, so a lane announces the seam on a sibling's log only where the changes are coupled (work/README.md Territory) - ATREST owns the census (topo census.rs) and TOPO the Euler operators - geom's convention module (frame_margins) and geom-brep's EdgeCurve::certify are read by these checks and are other programs' ground]
---
**How tier 3 reads a carrier.** Check 1 reads each carrier's datum
(frames, unit conventions, D2's declared/derived chart) and check 4
reads an edge's carrier against its faces (the material arm, the
transverse/smooth classification, the carrier's ends). Every row here
is a convention certified nowhere, a band that reads the wrong
quantity, or an edge class the arm never judges.
