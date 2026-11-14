---
id: limb-2s-band-decided-sheet-reading-omits-the-height-reading-beside-a-horns-cusp
kind: issue
title: Limb 2's band-decided sheet reading takes |f| over the lever beside a horn's cusp where the hull's reading takes the height reading
status: open
opened: 2026-11-11
priority: P4
cost: E
---


(SPIRICARM, residue of PR 6806, `a-horns-cusp-revolved-part-way-refuses-at-its-cap-meridians-certificate`; R1's NOTE 2 in that PR's review.)

## What

On a sheet, `geom-brep` `pcurve_cache/projected.rs` `Meters::certified` (the hull's reading, `SpanHull::meters`) is the least of `over`, the reading at the part's height (`Meters::sheet_at_height`) and the apex reach. `net_incidence`, limb 2's band-decided reading, takes `over` first, with its levers decided in the band. It falls to `Meters::apex_part` (the hull's reading) only where `over` refuses. Beside a horn's cusp, where `over`'s lever and side check both clear in band, it keeps `|f|`/lever, which is loose there (`|f|`'s rounding over about `z²`), instead of the height reading. That is sound, but looser.

No build was shown to fail on it: the horn lunes of `the_football_revolves` build at 1e-6, the default ε and 1e-12.

## Done when

`net_incidence` on a sheet reads the least of its band-decided `over` and the hull's reading. Alternatively, a row shows that no part reaches the band through `over` where the height reading is the tighter one, and this row closes as no defect.
