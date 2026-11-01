---
id: axis-channel-serves-only-the-line-reading
kind: issue
title: the per-surface axis token names a LINE, so the ratified concentricity and parallelism readings need component tables of their own
status: open
opened: 2026-09-29
priority: P3
cost: M
design: true
refs: [axis-per-component-source-beside-geom-source]
---


Found by the review of PR 3419 (NOTE 3); filed at adjudication.

`docs/AXIS-DECLARATION-DESIGN.md` describes one channel with several
readings: coaxiality (same line), concentricity (same point),
parallelism (same direction). PR 3419's token is one per surface and
names the axis LINE, which serves coaxiality only. Two spheres sharing
a line token are not thereby concentric, and a direction-only reading
cannot be answered from a line. The other readings would be further
component tables beside the line one (additive, not a redesign). No
consumer asks for them yet: step 4's consumer (`cs_pair_frame`) reads
coaxiality. Decide when a consumer for either reading appears; say in
the design doc, if the narrowing should be stated there, that the
per-surface token is the LINE reading.
