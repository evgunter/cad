---
id: the-winding-verdicts-scale-reads-corner-norms-not-what-an-evaluation-rounds-at
kind: issue
title: the winding verdict's scale is the corners' norm, which is not the magnitude a point evaluated as centre plus offset rounds at
status: open
opened: 2026-11-17
priority: P2
cost: M
---



Raised by the two reviews of PR 7043 (the clearance and winding unit), filed by that unit's fix pass. Not changed there.

## What

`crates/mesh/README.md` `[mesh.winding]` promises, in its second sentence, that a normal computed from a triangle's stored corners "would not flip if each coordinate moved by its evaluation rounding". The one definition (`mesh::validate::resolved_normal`) holds the triangle's height over its longest side against `cert::rounding_allowance` at `cert::coordinate_scale`, the largest of the three corners' norms. Two things stand between that measure and the sentence.

- **The margin.** One reviewer measured a triangle at 1.05 allowances flipping under a move of ±1 allowance per coordinate: the measure leaves about half an allowance per coordinate. The other measured that resolved triangles never flip under ±4 ulps per coordinate. Both are measurements of the definition as built; the sentence says "its evaluation rounding" and no row says how many ulps that is.
- **The scale drops the anchor.** `cert::position_scale` takes a chart's anchor as well as the corners; `coordinate_scale` takes the corners alone. A point of a surface evaluated as centre plus offset rounds at the centre's magnitude, so on a surface centred far from the origin whose face passes near the origin the corners' norms understate what each coordinate rounded at. Unmeasured: no body of the suite is built that way.

A third reading is on the same line: the design fork's count on the frustum moved a metre off its axis (6,610 unresolved at δ `10⁻⁸`) matches a scale of the largest |coordinate| (6,183 to 6,810 by direction, measured by the unit); under the corners' norm the same body reads 9,330 to 9,732, and its window widens to δ `2·10⁻⁸` (4,465 to 4,734).

## Owed

Say what "its evaluation rounding" is in ulps of what magnitude, measure the far-centred case, and then either the definition carries the anchor (a second input at the join, which today reads positions alone, and nothing for `check_winding` to read from a bare mesh) or the page's sentence is re-worded to what the measure holds. A change to either is a design choice on a ratified page.
